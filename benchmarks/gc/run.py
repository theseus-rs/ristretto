#!/usr/bin/env python3
"""Linux runner for paired, provenance-preserving Java GC workload screens."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import random
import re
import shutil
import signal
import subprocess
import sys
import tarfile
import time
import uuid

from acceptance import VERSION, validate_options
from report import write_report, validate
from cgroups import Budget
from pauses import observation as pause_observation, validate_trace, workload_digest

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
RUNTIMES = ["ristretto-baseline", "ristretto-candidate", "g1", "zgc", "g1-int", "zgc-int", "wasm-baseline", "wasm-candidate", "ristretto-baseline-jit", "ristretto-candidate-jit"]
RUNTIMES += [
    "ristretto-baseline-before", "ristretto-candidate-before",
    "ristretto-baseline-jit-before", "ristretto-candidate-jit-before",
    "wasm-baseline-before", "wasm-candidate-before",
]


def digest(path):
    with Path(path).open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def capture(command, environment=None):
    result = subprocess.run(command, cwd=ROOT, env=environment, capture_output=True, text=True, timeout=120, check=True)
    return (result.stdout + result.stderr).strip()


def optional_text(path):
    try:
        return Path(path).read_text().strip()
    except OSError:
        return None


def java_major(version):
    match = re.search(r'version "(\d+)', version)
    if not match:
        raise ValueError("could not determine JDK version")
    return int(match[1])


def command_for(runtime, options, classes, run_dir, binary, wasm, binary_before=None, wasm_before=None):
    workload = [options.workload, str(options.seconds * 1000), str(options.warmup * 1000), str(options.rate), str(options.seed)]
    if runtime.endswith("-before"):
        runtime = runtime.removesuffix("-before")
        if runtime.startswith("ristretto-"):
            if binary_before is None:
                raise ValueError("prior native build is required")
            binary = binary_before
        else:
            if wasm_before is None:
                raise ValueError("prior Wasm build is required")
            wasm = wasm_before
    if runtime.startswith("ristretto-"):
        return [str(binary), str(options.jdk), str(classes), str(options.heap_mib), str(options.workers), runtime.removeprefix("ristretto-"), *workload]
    if runtime.startswith("wasm-"):
        return [str(options.wasmtime), "run", "-W", "max-wasm-stack=8388608", "-S", "http",
                "--dir", f"{options.jdk}::/jdk", "--dir", f"{classes}::/classes",
                str(wasm), "/jdk", "/classes", str(options.heap_mib), "0", runtime.split("-")[1], *workload]
    collector = "-XX:+UseZGC" if runtime.startswith("zgc") else "-XX:+UseG1GC"
    return [str(options.jdk / "bin/java"), collector,
            f"-Xms{options.heap_mib}m", f"-Xmx{options.heap_mib}m",
            f"-XX:ActiveProcessorCount={len(options.cpus)}",
            f"-Xlog:gc*,safepoint:file={run_dir / 'gc.log'}:uptime,level,tags",
            *(["-Xint"] if runtime.endswith("-int") else []),
            "-cp", str(classes), "GcBench", *workload]


def freeze_build(source, destination):
    """Keep a revision's binary and verified build/source evidence together."""
    provenance = source.with_name(source.name + ".provenance.json")
    archive = source.parent / "sources.tar.gz"
    build = json.loads(provenance.read_text())
    if build["binary_sha256"] != digest(source) or build["source_archive_sha256"] != digest(archive):
        raise ValueError("binary/source build provenance does not match")
    paths = [destination, destination.with_name(destination.name + ".provenance.json"),
             destination.with_name(destination.name + ".sources.tar.gz")]
    for original, copied in zip([source, provenance, archive], paths):
        shutil.copy2(original, copied)
    return paths


def freeze_acceptance_policy(source, output, options):
    """Capture the declared limits before any child runs; no implicit defaults."""
    raw = source.read_bytes()
    validate_options(json.loads(raw), options)
    destination = output / "acceptance-policy.json"
    with destination.open("xb") as stream:
        stream.write(raw)
    return {"version": VERSION, "path": destination.name, "sha256": digest(destination)}


def qualification_observation(directory):
    """Missing telemetry stays missing; lifetime GC/profile counters cannot replace it."""
    lines = [line[len("GC_QUALIFICATION "):]
             for line in (optional_text(directory / "stdout.txt") or "").splitlines()
             if line.startswith("GC_QUALIFICATION ")]
    if not lines:
        return None
    if len(lines) != 1:
        raise ValueError("expected exactly one GC_QUALIFICATION packet")
    return json.loads(lines[0])


def execute(command, directory, timeout, cpus, environment, budget=None, record_identity=False):
    """Keep output even on failure, kill/reap the complete group on timeout."""
    resource_file = directory / "resource.txt"
    if record_identity:
        command = [sys.executable, str(HERE / "process_identity.py"),
                   str(directory / "process_identity.json"), *command]
    wrapped = ["/usr/bin/time", "-f", 'RESOURCE {"user_s":%U,"system_s":%S,"peak_rss_kib":%M}',
               "-o", str(resource_file), "--", *command]
    started = time.monotonic()
    resource_samples = []
    interval = max(1, timeout / 99999)
    def initialize_child():
        os.sched_setaffinity(0, cpus)
        if budget is not None:
            budget.join()
    with (directory / "stdout.txt").open("w") as stdout, (directory / "stderr.txt").open("w") as stderr:
        child = subprocess.Popen(wrapped, cwd=ROOT, env=environment, stdout=stdout, stderr=stderr,
                                 start_new_session=True, preexec_fn=initialize_child)
        try:
            while True:
                remaining = max(0, started + timeout - time.monotonic())
                try:
                    code = child.wait(timeout=min(remaining, interval) if budget else remaining)
                    break
                except subprocess.TimeoutExpired:
                    if budget:
                        resource_samples.append({"seconds": time.monotonic() - started,
                            "memory_bytes": int((budget.path / "memory.current").read_text()),
                            "cpu": budget.snapshot()["cpu_stat"]})
                    if time.monotonic() >= started + timeout:
                        raise
            status = "ok" if code == 0 else "failed"
        except subprocess.TimeoutExpired:
            os.killpg(child.pid, signal.SIGKILL)
            child.wait()
            code, status = child.returncode, "timeout"
        except BaseException:
            os.killpg(child.pid, signal.SIGKILL)
            child.wait()
            raise
    resources = None
    for line in (optional_text(resource_file) or "").splitlines():
        if line.startswith("RESOURCE "):
            resources = json.loads(line[len("RESOURCE "):])
    return {"status": status, "exit_code": code, "process_wall_s": time.monotonic() - started, "resources": resources, "resource_samples": resource_samples}


def gc_observation(directory):
    """Process-wide diagnostic only: includes bootstrap, warmup and output."""
    stdout = optional_text(directory / "stdout.txt") or ""
    instrumented = any(line.startswith("RISTRETTO_EXECUTION ") for line in stdout.splitlines())
    for line in stdout.splitlines():
        if line.startswith("RISTRETTO_GC "):
            data = json.loads(line[len("RISTRETTO_GC "):])
            return {"scope": "whole process", "activity_observed": data["collections_completed"] > 0,
                    "ristretto": data, "global_pause_ms": None,
                    "execution_profile": instrumented}
    log = optional_text(directory / "gc.log")
    if log is None:
        return {"scope": "whole process", "activity_observed": None, "global_pause_ms": None}
    pauses = [float(x) for x in re.findall(r"GC\(\d+\).*Pause[^\n]*? ([\d.]+)ms\s*$", log, flags=re.M)]
    return {"scope": "whole process", "activity_observed": bool(re.search(r"GC\(\d+\)", log)),
            # GC phase durations omit rendezvous and are not complete global-stop evidence.
            "global_pause_ms": None,
            "gc_phase_pause_ms": {"count": len(pauses), "max": max(pauses, default=0), "total": sum(pauses)}}


def validate_execution_mode(runtime, observation):
    if observation.get("execution_profile"):
        raise ValueError("execution-profile build is diagnostic; use coverage.py, not timing comparisons")
    if runtime.startswith("ristretto-"):
        expected = runtime.removesuffix("-before").endswith("-jit")
        actual = observation.get("ristretto", {}).get("jit_enabled", False)
        if actual is not expected:
            raise ValueError("runtime label disagrees with the VM's execution mode")


def freeze_sources(output):
    names = subprocess.check_output(["git", "ls-files", "-co", "--exclude-standard", "-z"], cwd=ROOT).decode().split("\0")
    hashes = {}
    with tarfile.open(output / "sources.tar.gz", "w:gz") as archive:
        for name in sorted(set(names)):
            path = ROOT / name
            if path.is_file() and (path.suffix in {".rs", ".java", ".py", ".toml", ".md", ".yml", ".patch"} or path.name == "Cargo.lock"):
                # Never recursively archive generated results placed in the tree.
                if output == path or output in path.parents:
                    continue
                hashes[name] = digest(path)
                archive.add(path, arcname=name, recursive=False)
    return hashes


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--jdk", type=Path, required=True, help="JDK 24+; same libraries for all runtimes")
    parser.add_argument("--ristretto", type=Path, default=ROOT / "target/release/examples/gc_compare")
    parser.add_argument("--ristretto-before", type=Path, help="frozen prior build for native '-before' rows; both builds need provenance")
    parser.add_argument("--wasm", type=Path, default=ROOT / "target/wasm32-wasip2/release/examples/gc_compare.wasm")
    parser.add_argument("--wasm-before", type=Path, help="frozen prior build for Wasm '-before' rows; both builds need provenance")
    parser.add_argument("--wasmtime", type=Path)
    parser.add_argument("--runtimes", nargs="+", choices=RUNTIMES, default=RUNTIMES[:4])
    parser.add_argument("--workloads", nargs="+", choices=["control", "allocation", "roots", "mixed", "reference-array"], default=["allocation", "roots", "mixed"])
    parser.add_argument("--rates", nargs="+", type=int, default=[0, 1000], help="0: saturation; positive: fixed offered operations/s")
    parser.add_argument("--seconds", type=int, default=30)
    parser.add_argument("--warmup", type=int, default=10)
    parser.add_argument("--repeats", type=int, default=3)
    parser.add_argument("--heap-mib", type=int, default=256)
    parser.add_argument("--workers", type=int, default=2)
    parser.add_argument("--cpus", type=int, nargs="+", required=True, help="identical Linux CPU affinity for every child")
    parser.add_argument("--cgroup-parent", type=Path, help="delegated cgroup v2 parent with cpu+memory enabled")
    parser.add_argument("--rss-mib", type=int, help="enforced per-child physical memory limit, including all runtime overhead")
    parser.add_argument("--cpu-quota", type=float, help="total CPU cores worth of quota, including all runtime threads")
    parser.add_argument("--timeout", type=int, default=180, help="whole process deadline including startup/output")
    parser.add_argument("--seed", type=int, default=42)
    parser.add_argument("--output", type=Path, required=True, help="new directory; existing evidence is never overwritten")
    parser.add_argument("--pause-recording", action="store_true",
                        help="diagnostic publication capture; requires experimental-pause-recorder build")
    parser.add_argument("--pause-capacity", type=int, default=65536,
                        help="fixed pause slots per diagnostic process; overflow stays incomplete")
    parser.add_argument("--acceptance-policy", type=Path,
                        help="predeclared v2 native workload/resource limits and coordination audit")
    options = parser.parse_args()
    if not 1 <= options.pause_capacity <= 2**32 - 1:
        parser.error("invalid pause recorder capacity")
    if sys.platform != "linux":
        parser.error("resource capture currently requires Linux")
    if not 1 <= options.seconds <= 86400 or not 0 <= options.warmup <= 86400 or options.repeats < 1:
        parser.error("invalid duration, warmup or repetitions")
    if options.heap_mib < 32 or options.workers < 0 or any(r < 0 or r > 10**9 for r in options.rates):
        parser.error("invalid heap, workers or offered rate")
    if not -(2**31) <= options.seed < 2**31 or len(set(options.cpus)) != len(options.cpus):
        parser.error("invalid seed or duplicate CPUs")
    if not set(options.cpus) <= os.sched_getaffinity(0):
        parser.error("requested CPUs are outside available affinity")
    if options.timeout <= options.seconds + options.warmup:
        parser.error("timeout must exceed warmup plus measured duration")
    supplied = [options.cgroup_parent is not None, options.rss_mib is not None, options.cpu_quota is not None]
    if any(supplied) and not all(supplied):
        parser.error("--cgroup-parent, --rss-mib and --cpu-quota must be supplied together")
    if all(supplied) and (options.rss_mib < options.heap_mib or not 0 < options.cpu_quota <= len(options.cpus)):
        parser.error("physical budget must cover the heap; CPU quota must fit the affinity")
    if options.acceptance_policy and not all(supplied):
        parser.error("--acceptance-policy requires enforced cgroup CPU/memory limits")
    if options.cgroup_parent:
        options.cgroup_parent = options.cgroup_parent.resolve()
        probe = Budget(options.cgroup_parent, options.rss_mib * 1024**2, round(options.cpu_quota * 100000))
        try:
            probe.snapshot()
        finally:
            probe.close()
    options.output = options.output.resolve()
    options.jdk = options.jdk.resolve()
    environment = dict(os.environ)
    removed = {}
    for key in ["JAVA_TOOL_OPTIONS", "_JAVA_OPTIONS", "JDK_JAVA_OPTIONS", "CLASSPATH", "JAVA_HOME", "RISTRETTO_JDK_HOME",
                "RISTRETTO_QUALIFICATION_RUN_ID", "RISTRETTO_QUALIFICATION_POLICY_SHA256"]:
        # Record names only; inherited values may contain secrets.
        removed[key] = environment.pop(key, None) is not None
    environment.update({"LC_ALL": "C", "JAVA_HOME": str(options.jdk), "RISTRETTO_JDK_HOME": str(options.jdk)})
    version = capture([str(options.jdk / "bin/java"), "-version"], environment)
    if java_major(version) < 24:
        parser.error("JDK 24+ required so ZGC consistently means generational ZGC")
    output = options.output
    output.mkdir(parents=True, exist_ok=False)
    acceptance_policy = None
    if options.acceptance_policy:
        try:
            acceptance_policy = freeze_acceptance_policy(options.acceptance_policy, output, vars(options))
        except (OSError, ValueError, KeyError, TypeError) as error:
            parser.error(f"invalid predeclared acceptance policy: {error}")
    classes = output / "classes"
    classes.mkdir()
    subprocess.run([str(options.jdk / "bin/javac"), "--release", "8", "-Xlint:-options", "-d", str(classes), str(HERE / "GcBench.java"), str(HERE / "GcTelemetry.java")], env=environment, check=True)
    binary, wasm = output / "gc_compare", output / "gc_compare.wasm"
    if any(r.startswith("ristretto-") for r in options.runtimes):
        shutil.copy2(options.ristretto, binary)
    if any(r.startswith("wasm-") for r in options.runtimes):
        if options.wasmtime is None:
            parser.error("--wasmtime required for Wasm comparisons")
        options.wasmtime = options.wasmtime.resolve()
        shutil.copy2(options.wasm, wasm)
    binary_before, wasm_before = None, None
    build_artifacts = []
    for prefix, source, prior, destination in [
            ("ristretto-", options.ristretto, options.ristretto_before, binary),
            ("wasm-", options.wasm, options.wasm_before, wasm)]:
        needs_prior = any(r.startswith(prefix) and r.endswith("-before") for r in options.runtimes)
        needs_current = acceptance_policy is not None and prefix == "ristretto-"
        if needs_prior or needs_current:
            if needs_prior and prior is None:
                parser.error(f"prior build required for {prefix}before rows")
            before = destination.with_name(destination.name + "-before")
            try:
                build_artifacts.extend(freeze_build(source, destination))
                if needs_prior:
                    build_artifacts.extend(freeze_build(prior, before))
            except (OSError, ValueError, KeyError) as error:
                parser.error(f"invalid build provenance: {error}")
            if needs_prior and prefix == "ristretto-":
                binary_before = before
            elif needs_prior:
                wasm_before = before
    source_hashes = freeze_sources(output)
    artifacts = {str(p.relative_to(output)): digest(p) for p in classes.glob("*.class")}
    artifacts.update({p.name: digest(p) for p in [binary, wasm] if p.exists()})
    artifacts.update({p.name: digest(p) for p in build_artifacts})
    if acceptance_policy:
        artifacts[acceptance_policy["path"]] = acceptance_policy["sha256"]
    metadata = {
        "schema": 1, "created_utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
        "qualification": False, "scope": "same-Java end-to-end runtime comparison; not isolated collector speed",
        "acceptance_policy": acceptance_policy,
        "options": {k: str(v) if isinstance(v, Path) else v for k, v in vars(options).items()},
        "git_head": capture(["git", "rev-parse", "HEAD"]), "git_status": capture(["git", "status", "--short"]),
        "rustc": capture(["rustc", "--version"]), "java": version,
        "wasmtime": capture([str(options.wasmtime), "--version"]) if wasm.exists() else None,
        "host": platform.platform(), "cpuinfo": optional_text("/proc/cpuinfo"),
        "cpu_quota": optional_text("/sys/fs/cgroup/cpu.max"), "memory_limit": optional_text("/sys/fs/cgroup/memory.max"),
        "transparent_hugepages": optional_text("/sys/kernel/mm/transparent_hugepage/enabled"),
        "shmem_hugepages": optional_text("/sys/kernel/mm/transparent_hugepage/shmem_enabled"),
        "removed_environment_keys": removed, "artifacts_sha256": artifacts,
        "source_sha256": source_hashes, "source_archive_sha256": digest(output / "sources.tar.gz"),
        "java_sha256": digest(options.jdk / "bin/java"), "jdk_modules_sha256": digest(options.jdk / "lib/modules"),
        "wasmtime_sha256": digest(options.wasmtime) if wasm.exists() else None,
    }
    (output / "metadata.json").write_text(json.dumps(metadata, indent=2) + "\n")
    rng = random.Random(options.seed)
    jobs = []
    for repeat in range(options.repeats):
        pairs = [(work, rate) for work in options.workloads for rate in options.rates]
        rng.shuffle(pairs)
        for work, rate in pairs:
            order = list(options.runtimes)
            rng.shuffle(order)
            jobs.extend((repeat, work, rate, runtime) for runtime in order)
    (output / "schedule.json").write_text(json.dumps(jobs, indent=2) + "\n")
    failures = 0
    with (output / "raw.jsonl").open("w") as raw:
        for index, (repeat, work, rate, runtime) in enumerate(jobs):
            options.workload, options.rate = work, rate
            directory = output / f"run-{index:04d}-{runtime}-{work}-{rate}"
            directory.mkdir()
            command = command_for(runtime, options, classes, directory, binary, wasm, binary_before, wasm_before)
            print(f"[{index+1}/{len(jobs)}] {runtime} {work} rate={rate} repeat={repeat+1}", flush=True)
            row = {"runtime": runtime, "workload": work, "rate": rate, "repeat": repeat,
                   "target": "wasm" if runtime.startswith("wasm-") else "native", "directory": directory.name,
                   "command": command, "run_id": uuid.uuid4().hex}
            (directory / "command.json").write_text(json.dumps(command, indent=2) + "\n")
            budget = Budget(options.cgroup_parent, options.rss_mib * 1024**2, round(options.cpu_quota * 100000)) if options.cgroup_parent else None
            try:
                child_environment = dict(environment)
                child_environment.update(RISTRETTO_QUALIFICATION_RUN_ID=row["run_id"])
                child_environment.pop("RISTRETTO_QUALIFICATION_POLICY_SHA256", None)
                child_environment.pop("RISTRETTO_SOURCE_ARCHIVE_SHA256", None)
                if acceptance_policy:
                    child_environment["RISTRETTO_QUALIFICATION_POLICY_SHA256"] = acceptance_policy["sha256"]
                record_pauses = options.pause_recording and runtime.startswith("ristretto-")
                if record_pauses:
                    child_environment["RISTRETTO_PAUSE_CAPACITY"] = str(options.pause_capacity)
                    frozen = Path(command[0])
                    build = json.loads(frozen.with_name(frozen.name + ".provenance.json").read_text())
                    child_environment["RISTRETTO_SOURCE_ARCHIVE_SHA256"] = build["source_archive_sha256"]
                row.update(execute(command, directory, options.timeout, options.cpus, child_environment, budget,
                                   record_identity=record_pauses))
                row["cgroup"] = budget.snapshot() if budget else None
                if budget and row["cgroup"]["memory_events"].get("oom", 0):
                    row["status"] = "memory_limit"
            finally:
                if budget:
                    budget.close()
            row["gc_observation"] = gc_observation(directory)
            if row["status"] == "ok":
                try:
                    lines = [s[len("GC_RESULT "):] for s in (directory / "stdout.txt").read_text().splitlines() if s.startswith("GC_RESULT ")]
                    if len(lines) != 1:
                        raise ValueError("expected exactly one GC_RESULT")
                    row["measurement"] = json.loads(lines[0])
                    validate(row["measurement"], work, rate, options.seconds, options.seed)
                    validate_execution_mode(runtime, row["gc_observation"])
                    row["qualification_evidence"] = qualification_observation(directory)
                    trace = pause_observation(directory)
                    if trace is not None:
                        row["pause_evidence"] = trace
                    if record_pauses:
                        receipt = json.loads((directory / "process_identity.json").read_text())
                        if receipt.get("run_id") != row["run_id"]:
                            raise ValueError("pause launcher run identity mismatch")
                        expected = {"run_id": row["run_id"], "process_id": receipt["process_id"],
                                    "recorder_capacity": options.pause_capacity,
                                    "policy_sha256": acceptance_policy["sha256"] if acceptance_policy else None,
                                    "binary_sha256": build["binary_sha256"],
                                    "source_archive_sha256": build["source_archive_sha256"],
                                    "workload_sha256": workload_digest(classes),
                                    "runtime": runtime.removesuffix("-before"), "workload": work,
                                    "duration_ms": options.seconds * 1000, "warmup_ms": options.warmup * 1000,
                                    "rate": rate, "seed": options.seed}
                        row["pause_observation"] = validate_trace(trace, expected, row["measurement"]["elapsed_ns"])
                        row["diagnostic_only"] = True
                    elif trace is not None:
                        raise ValueError("pause-recorder build is diagnostic; use --pause-recording")
                except (ValueError, KeyError, TypeError) as error:
                    row.update(status="invalid", error=str(error))
            if row["status"] != "ok":
                failures += 1
                print(f"  {row['status']}: see {directory.name}/stderr.txt", flush=True)
            row["output_sha256"] = {name: digest(directory / name)
                                     for name in ["stdout.txt", "stderr.txt", "resource.txt", "gc.log", "process_identity.json"]
                                     if (directory / name).is_file()}
            raw.write(json.dumps(row) + "\n")
            raw.flush()
    # Detect changing inputs rather than combining them in one comparison.
    unchanged = all(digest(ROOT / name) == value for name, value in source_hashes.items())
    unchanged = unchanged and digest(options.jdk / "lib/modules") == metadata["jdk_modules_sha256"]
    unchanged = unchanged and digest(options.jdk / "bin/java") == metadata["java_sha256"]
    unchanged = unchanged and all(digest(output / name) == expected for name, expected in artifacts.items())
    if wasm.exists():
        unchanged = unchanged and digest(options.wasmtime) == metadata["wasmtime_sha256"]
    (output / "integrity.json").write_text(json.dumps({"inputs_unchanged": unchanged}) + "\n")
    write_report(output)
    return 1 if failures or not unchanged else 0


if __name__ == "__main__":
    sys.exit(main())
