#!/usr/bin/env python3
"""Run paired finite HotSpot, native and WASI ring processes in a bounded cgroup."""
import argparse
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import platform
import subprocess
import tempfile
import time

from report import WORKLOADS, expected

HERE = Path(__file__).resolve().parent
FIXTURES = HERE.parents[2] / "ristretto_vm/tests/fixtures/canonical"
CLASSES = ("CanonicalRingWorkload", "CanonicalRingWorkload$Node")


def resource_limits():
    root = Path("/sys/fs/cgroup")
    group = next(line[3:] for line in Path("/proc/self/cgroup").read_text().splitlines()
                 if line.startswith("0::"))
    directory = root / group.lstrip("/")
    memory = []
    cpu = []
    while True:
        limit = directory / "memory.max"
        if limit.exists() and (value := limit.read_text().strip()) != "max":
            memory.append(int(value))
        quota = directory / "cpu.max"
        if quota.exists():
            value, period = quota.read_text().split()
            if value != "max":
                cpu.append(int(value) / int(period))
        if directory == root:
            break
        directory = directory.parent
    if not memory or not cpu:
        raise SystemExit("run inside a cgroup with finite memory.max and cpu.max (see README)")
    return dict(cgroup=group, memory_max_bytes=min(memory), cpu_quota=min(cpu),
                affinity=sorted(os.sched_getaffinity(0)))


def digest(path):
    with path.open("rb") as raw:
        return hashlib.file_digest(raw, "sha256").hexdigest()


def host_details(cpus):
    uname = platform.uname()
    model = uname.machine
    for line in Path("/proc/cpuinfo").read_text().splitlines():
        key, separator, value = line.partition(":")
        if separator and key.strip() in ("model name", "Hardware"):
            model = value.strip()
            break
    topology = []
    for cpu in cpus:
        root = Path(f"/sys/devices/system/cpu/cpu{cpu}/topology")
        package, core = root / "physical_package_id", root / "core_id"
        if package.is_file() and core.is_file():
            topology.append(dict(cpu=cpu, package=int(package.read_text()), core=int(core.read_text())))
    return dict(system=uname.system, kernel=uname.release, architecture=uname.machine,
                cpu_model=model, topology=topology)


def capture_rust(command, plan, checksum, classes, revision, retained):
    before = time.monotonic_ns()
    process = subprocess.run([*command, plan.workload, str(plan.steps), str(checksum),
                              str(plan.warmup), str(plan.samples)],
                             text=True, capture_output=True, timeout=300)
    elapsed = time.monotonic_ns() - before
    if process.returncode:
        raise RuntimeError(f"{command[0]} failed: {process.stderr}")
    samples = []
    identities = {}
    metadata = {}
    for line in process.stdout.splitlines():
        key, *values = line.split("\t")
        if key == "sample":
            result, duration, allocations, polls, released = map(int, values)
            if result != checksum or duration <= 0 or min(allocations, polls, released) < 0:
                raise ValueError(f"invalid sample: {line}")
            if allocations != 2 * (retained + plan.steps):
                raise ValueError(f"unexpected ring allocation count: {allocations}")
            samples.append(dict(checksum=result, elapsed_ns=duration, allocations=allocations,
                                helper_polls=polls, released_objects=released))
        elif key == "class" and values[0] not in identities:
            name, value = values
            identities[name] = value
        elif key in ("limits", "cleanup", "revision", "debug_assertions", "batch_ns") and key not in metadata:
            metadata[key] = values
        else:
            raise ValueError(f"unexpected or duplicate output: {line}")
    if identities != classes or metadata["revision"] != [revision]:
        raise ValueError("Ristretto source revision or loaded Java class hashes differ")
    if metadata["debug_assertions"] != ["false"] or len(samples) != plan.samples:
        raise ValueError("expected a release build and the full sample plan")
    limits = list(map(int, metadata["limits"]))
    if limits != [4 * retained + 8, 128, 3, 64 * 1024 * 1024]:
        raise ValueError("Ristretto heap settings differ from the comparison plan")
    before_drain, polls, released = map(int, metadata["cleanup"])
    if min(before_drain, polls, released) < 0:
        raise ValueError("invalid cleanup receipt")
    batch, = map(int, metadata["batch_ns"])
    if not sum(s["elapsed_ns"] for s in samples) <= batch <= elapsed:
        raise ValueError("invalid measured batch interval")
    return dict(process_ns=elapsed, batch_ns=batch, command=list(map(str, command)), samples=samples,
                objects=limits[0], roots=limits[1], edges_per_object=limits[2],
                payload_limit_bytes=limits[3], payload_bytes_before_drain=before_drain,
                final_drain_polls=polls, released_in_final_drain=released, cleanup_verified=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("output", type=Path)
    parser.add_argument("--native", type=Path, required=True)
    parser.add_argument("--compiled", type=Path, help="native example built with the compiled tier")
    parser.add_argument("--wasi", type=Path, required=True)
    parser.add_argument("--wasmtime", type=Path, required=True)
    parser.add_argument("--revision", required=True, help="full Git commit embedded in all binaries")
    parser.add_argument("--workload", choices=WORKLOADS, default="small")
    parser.add_argument("--steps", type=int, default=17)
    parser.add_argument("--warmup", type=int, default=2)
    parser.add_argument("--samples", type=int, default=4)
    parser.add_argument("--forks", type=int, default=3)
    args = parser.parse_args()
    if args.output.exists() or not (0 <= args.steps <= 100000 and 0 <= args.warmup <= 128
                                    and 1 <= args.samples <= 128 and 1 <= args.forks <= 32):
        parser.error("choose a new output path and a finite positive sample plan")
    if len(args.revision) != 40 or any(c not in "0123456789abcdef" for c in args.revision):
        parser.error("use the full lower-case Git commit ID")
    os.sched_setaffinity(0, sorted(os.sched_getaffinity(0))[:2])
    limits = resource_limits()
    checksum = expected(args.workload, args.steps)
    retained = WORKLOADS[args.workload][0]
    artifacts = {"native": args.native.resolve(), "wasi": args.wasi.resolve(),
                 "wasmtime": args.wasmtime.resolve()}
    lanes = {"native-interpreted": [artifacts["native"], "interpreted"],
             "wasi-interpreted": [artifacts["wasmtime"], "run", "-W", "max-wasm-stack=8388608",
                                  artifacts["wasi"], "interpreted"]}
    if args.compiled:
        artifacts["compiled"] = args.compiled.resolve()
        lanes["native-compiled"] = [artifacts["compiled"], "compiled"]
    hashes = {k: digest(path) for k, path in artifacts.items()}
    classes = {name + ".class": digest(FIXTURES / (name + ".class")) for name in CLASSES}
    host = host_details(limits["affinity"])
    started = datetime.now(timezone.utc).isoformat()
    records = []
    jdk = None
    with tempfile.TemporaryDirectory() as directory:
        for fork in range(args.forks):
            order = ["HotSpot", *lanes]
            offset = fork % len(order)
            for name in order[offset:] + order[:offset]:
                if limits != resource_limits():
                    raise RuntimeError("resource limits changed during the comparison")
                if name == "HotSpot":
                    output = Path(directory) / f"java-{fork}.json"
                    subprocess.run([HERE / "hotspot.sh", output, "--workload",
                                    args.workload, "--steps", str(args.steps), "--samples",
                                    str(args.samples), "--warmup", str(args.warmup), "--forks", "1",
                                    "--first-collector", "ZGC" if fork % 2 else "G1"], check=True)
                    capture = json.loads(output.read_text())
                    if (capture["limits"] != limits or capture["expected"] != checksum
                            or capture["class_sha256"] != classes or (jdk and jdk != capture["jdk"])):
                        raise ValueError("HotSpot source, resource or oracle mismatch")
                    jdk = capture["jdk"]
                    for row in capture["processes"]:
                        records.append(dict(row, fork=fork, tier=row["collector"]))
                else:
                    row = capture_rust(lanes[name], args, checksum, classes, args.revision, retained)
                    records.append(dict(row, fork=fork, tier=name))
    if hashes != {k: digest(path) for k, path in artifacts.items()} or limits != resource_limits():
        raise ValueError("artifact identity or resource limits changed")
    wasmtime = subprocess.check_output([artifacts["wasmtime"], "--version"], text=True).strip()
    with args.output.open("x") as out:
        json.dump(dict(schema=2, source=args.revision, workload=args.workload, steps=args.steps,
                       expected=checksum, warmup=args.warmup, samples=args.samples, forks=args.forks,
                       arrival="one call begins after the previous call returns", jdk=jdk,
                       wasmtime=wasmtime, artifact_sha256=hashes, class_sha256=classes,
                       resource_limits=limits, host=host, started_utc=started,
                       finished_utc=datetime.now(timezone.utc).isoformat(), processes=records), out, indent=2)
        out.write("\n")


if __name__ == "__main__":
    main()
