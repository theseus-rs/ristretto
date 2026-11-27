#!/usr/bin/env python3
"""Concrete leaf lifecycle cost screen. Never a pause or JVM qualification."""
import argparse
import csv
import hashlib
import json
import os
from pathlib import Path
import platform
import random
import shutil
import statistics
import subprocess

MODES = ["legacy", "canonical-explicit", "canonical-auto"]
ARGV0 = "leaf_lifecycle"
TIMES = ["operation_ns", "cleanup_ns", "lifecycle_ns", "setup_ns", "teardown_ns"]
FIELDS = {"version", "mode", "operations", "batch", "payload_bytes", "logical_object_bytes",
          "heap_limit", *TIMES, "checksum", "allocation_polls", "helper_polls",
          "released_objects", "released_roots", "catalog_bytes", "payload_peak",
          "teardown_bytes", "teardown_catalog_bytes"}
COUNT_FIELDS = {"version", "mode", "operations", "batch", "round", "allocations",
                "allocated_bytes", "releases", "released_bytes", "live_before",
                "live_after", "peak_above_baseline"}


def digest(path):
    with Path(path).open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def unique_object(items):
    result = {}
    for key, value in items:
        if key in result:
            raise ValueError(f"duplicate JSON key: {key}")
        result[key] = value
    return result


def packet(text):
    return json.loads(text, object_pairs_hook=unique_object)


def identity(row, fields, mode, operations, batch):
    if not isinstance(row, dict) or set(row) != fields:
        raise ValueError("unexpected result schema")
    if any(type(value) is not int or value < 0 for key, value in row.items() if key != "mode"):
        raise ValueError("expected nonnegative integer measurements")
    if (row["version"], row["mode"], row["operations"], row["batch"]) != (1, mode, operations, batch):
        raise ValueError("measurement identity mismatch")


def validate(row, mode, operations, batch):
    identity(row, FIELDS, mode, operations, batch)
    remainder = operations % 1024
    checksum = operations // 1024 * 1024 * 1025 // 2 + remainder * (remainder + 1) // 2
    if row["checksum"] != checksum or row["payload_bytes"] != 32 or row["heap_limit"] != 67108864:
        raise ValueError("payload/checksum/configuration mismatch")
    if row["logical_object_bytes"] < 32 or row["payload_peak"] != min(operations, batch) * row["logical_object_bytes"]:
        raise ValueError("wrong logical payload peak")
    if any(row[key] <= 0 for key in TIMES) or row["lifecycle_ns"] < row["operation_ns"] + row["cleanup_ns"]:
        raise ValueError("invalid lifecycle timing boundaries")
    if row["teardown_bytes"] or row["teardown_catalog_bytes"]:
        raise ValueError("incomplete teardown")
    if mode == "legacy":
        if any(row[key] for key in ["allocation_polls", "helper_polls", "released_objects", "released_roots", "catalog_bytes"]):
            raise ValueError("legacy reports canonical activity")
    elif (row["released_objects"] != operations or row["released_roots"] != operations
          or row["allocation_polls"] < operations or row["helper_polls"] < operations
          or not row["catalog_bytes"]):
        raise ValueError("incomplete canonical allocation/reclamation")
    return row


def validate_counts(rows, mode, operations, batch):
    if len(rows) != 3:
        raise ValueError("expected three complete allocation-count rounds")
    for index, row in enumerate(rows):
        identity(row, COUNT_FIELDS, mode, operations, batch)
        if row["round"] != index or row["allocations"] < operations or row["allocated_bytes"] < operations * 32:
            raise ValueError("incomplete allocator counts")
        if row["allocated_bytes"] - row["released_bytes"] != row["live_after"] - row["live_before"]:
            raise ValueError("inconsistent requested-byte accounting")
        if row["peak_above_baseline"] < max(0, row["live_after"] - row["live_before"]):
            raise ValueError("inconsistent requested-byte peak")
        if index and row["live_before"] != rows[index - 1]["live_after"]:
            raise ValueError("unexplained inter-round allocation")
    return rows


def interval(pairs, seed):
    ratios = [candidate / legacy for legacy, candidate in pairs]
    rng = random.Random(seed)
    medians = sorted(statistics.median(rng.choices(ratios, k=len(ratios))) for _ in range(10000))
    return dict(median=statistics.median(ratios), low=medians[250], high=medians[9750], pairs=len(pairs))


def invocation(binary, arguments, runner):
    if runner:
        return [str(runner), "run", "-W", "max-wasm-stack=8388608", "--argv0", ARGV0, str(binary), *arguments], None
    return [ARGV0, *arguments], str(binary)


def run(args):
    if args.cpu not in os.sched_getaffinity(0) or args.repeats < 2:
        raise ValueError("need an available CPU and at least two independent process pairs")
    if not 1 <= args.operations <= 10000000 or len(set(args.batches)) != len(args.batches) or any(not 1 <= b <= 16384 for b in args.batches):
        raise ValueError("invalid workload bounds")
    with args.binary.open("rb") as stream:
        wasm = stream.read(4) == b"\0asm"
    if wasm != bool(args.wasmtime):
        raise ValueError("WASI needs --wasmtime; native must omit it")
    provenance = json.loads(args.provenance.read_text())
    if provenance["binary_sha256"] != digest(args.binary) or provenance["source_archive_sha256"] != digest(args.sources):
        raise ValueError("build provenance does not match frozen inputs")
    out = args.output.resolve()
    out.mkdir(parents=True, exist_ok=False)
    inputs = {}
    for name, source in [("binary", args.binary), ("build.json", args.provenance), ("sources.tar.gz", args.sources),
                         ("leaf_lifecycle.py", Path(__file__)), ("wasmtime", args.wasmtime)]:
        if source:
            path = out / name
            shutil.copy2(source, path)
            inputs[name] = dict(path=str(path), sha256=digest(path))
    runner = out / "wasmtime" if args.wasmtime else None
    runner_version = subprocess.check_output([str(runner), "--version"], text=True).strip() if runner else None
    metadata = dict(version=1, target="WASI" if wasm else "native", kind=args.kind, qualification=False,
                    operations=args.operations, batches=args.batches, repeats=args.repeats, cpu=args.cpu,
                    seed=args.seed, inputs=inputs, argv0=ARGV0, host=platform.platform(), wasmtime_version=runner_version,
                    clock="in-process/guest monotonic wall time", warmup="none; fresh process per timed invocation",
                    scope="32-byte reference-free values; one mutator; synchronous batch cleanup; setup/teardown reported separately",
                    counting_scope="whole warmed invocation including setup, teardown and report String; no stdout; requested bytes, not RSS")
    for name in ["scaling_governor", "scaling_min_freq", "scaling_max_freq"]:
        path = Path(f"/sys/devices/system/cpu/cpu{args.cpu}/cpufreq/{name}")
        metadata[name] = path.read_text().strip() if path.exists() else None
    rng = random.Random(args.seed)
    schedule = []
    for repeat in range(args.repeats if args.kind == "timing" else 1):
        batches = args.batches.copy()
        rng.shuffle(batches)
        for batch in batches:
            modes = MODES.copy()
            rng.shuffle(modes)
            schedule.extend((repeat, batch, mode) for mode in modes)
    (out / "metadata.json").write_text(json.dumps(metadata, indent=2) + "\n")
    (out / "schedule.json").write_text(json.dumps(schedule, indent=2) + "\n")
    failures = []
    with (out / "raw.jsonl").open("w") as stream:
        for index, (repeat, batch, mode) in enumerate(schedule):
            command, executable = invocation(out / "binary", [mode, str(args.operations), str(batch)], runner)
            row = dict(index=index, repeat=repeat, batch=batch, mode=mode, command=command, executable=executable)
            try:
                child = subprocess.run(command, executable=executable, capture_output=True, text=True, timeout=600,
                                       preexec_fn=lambda: os.sched_setaffinity(0, [args.cpu]))
                row.update(stdout=child.stdout, stderr=child.stderr, exit_code=child.returncode)
                child.check_returncode()
                if args.kind == "timing":
                    row["measurement"] = validate(packet(child.stdout), mode, args.operations, batch)
                else:
                    row["counts"] = validate_counts([packet(line) for line in child.stdout.splitlines()], mode, args.operations, batch)
                row["status"] = "ok"
            except (subprocess.SubprocessError, ValueError, KeyError, TypeError) as error:
                row.update(status="failed", error=str(error))
                if isinstance(error, subprocess.TimeoutExpired):
                    row.update(stdout=str(error.stdout), stderr=str(error.stderr))
                failures.append(index)
            stream.write(json.dumps(row) + "\n")
            stream.flush()
            print(f"{args.kind} {index + 1}/{len(schedule)} {mode} batch={batch} {row['status']}", flush=True)
    unchanged = all(digest(value["path"]) == value["sha256"] for value in inputs.values())
    (out / "completion.json").write_text(json.dumps(dict(failures=failures, inputs_unchanged=unchanged, qualification=False), indent=2) + "\n")
    return int(bool(failures) or not unchanged)


def report(args):
    out = args.output
    metadata = json.loads((out / "metadata.json").read_text())
    completion = json.loads((out / "completion.json").read_text())
    if metadata["kind"] != "timing" or completion["failures"] or not completion["inputs_unchanged"]:
        raise ValueError("need a complete unchanged timing run")
    if not all(digest(value["path"]) == value["sha256"] for value in metadata["inputs"].values()):
        raise ValueError("frozen input changed")
    schedule = json.loads((out / "schedule.json").read_text())
    rows = [packet(line) for line in (out / "raw.jsonl").read_text().splitlines()]
    if len(rows) != len(schedule) or len(rows) != metadata["repeats"] * len(metadata["batches"]) * len(MODES):
        raise ValueError("missing process rows")
    groups = {}
    for index, (row, expected) in enumerate(zip(rows, schedule)):
        identity_key = (row["repeat"], row["batch"], row["mode"])
        if list(identity_key) != expected or row["index"] != index or row["status"] != "ok" or identity_key in groups:
            raise ValueError("invalid process schedule")
        measured = validate(packet(row["stdout"]), row["mode"], metadata["operations"], row["batch"])
        if measured != row["measurement"]:
            raise ValueError("raw/stored measurement mismatch")
        groups[identity_key] = measured
    results = []
    for batch in metadata["batches"]:
        for mode in MODES:
            measured = [groups[repeat, batch, mode] for repeat in range(metadata["repeats"])]
            result = dict(batch=batch, mode=mode)
            for metric in TIMES:
                result[metric] = statistics.median(row[metric] for row in measured)
            result["operations_per_second"] = metadata["operations"] * 1e9 / result["lifecycle_ns"]
            result["catalog_bytes"] = measured[0]["catalog_bytes"]
            result["min_lifecycle_ms"] = min(row["lifecycle_ns"] for row in measured) / 1e6
            pairs = [(groups[r, batch, "legacy"]["lifecycle_ns"], groups[r, batch, mode]["lifecycle_ns"])
                     for r in range(metadata["repeats"])]
            result["lifecycle_ratio"] = interval(pairs, metadata["seed"])
            results.append(result)
    (out / "summary.json").write_text(json.dumps(dict(results=results, qualification=False), indent=2) + "\n")
    with (out / "results.csv").open("w", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=["target", "repeat", "batch", "mode", *TIMES, "operations_per_second", "catalog_bytes"])
        writer.writeheader()
        for (repeat, batch, mode), measured in sorted(groups.items()):
            writer.writerow(dict(target=metadata["target"], repeat=repeat, batch=batch, mode=mode,
                                 **{key: measured[key] for key in TIMES}, catalog_bytes=measured["catalog_bytes"],
                                 operations_per_second=metadata["operations"] * 1e9 / measured["lifecycle_ns"]))
    lines = ["# Concrete leaf lifecycle cost", "", f"Target: {metadata['target']}; {metadata['repeats']} independent randomized process blocks; {metadata['operations']} operations per process.",
             "", "Lifecycle includes allocation, initial rooting, typed read, root drop, synchronous reclamation, and validation. Setup and teardown are separate. No warmup is discarded. Guest/process startup is excluded. These are throughput costs, not operation latency, pauses, contention, or JVM/G1/ZGC qualification.",
             "", "Ratios above one mean canonical is slower. Intervals are exploratory unadjusted 95% bootstrap intervals over paired process blocks; operations are not independent samples.",
             "", "| Batch | Mode | Lifecycle ops/s | Operation ns/op | Cleanup ns/op | Time / legacy [95% CI] | Setup ms | Teardown ms |", "|---:|---|---:|---:|---:|---:|---:|---:|"]
    for result in results:
        ci = result["lifecycle_ratio"]
        lines.append(f"| {result['batch']} | {result['mode']} | {result['operations_per_second']:.0f} | {result['operation_ns']/metadata['operations']:.1f} | {result['cleanup_ns']/metadata['operations']:.1f} | {ci['median']:.2f} [{ci['low']:.2f}, {ci['high']:.2f}] | {result['setup_ns']/1e6:.2f} | {result['teardown_ns']/1e6:.2f} |")
    if any(result["min_lifecycle_ms"] < 100 for result in results):
        lines.extend(["", "Some process lifecycles were shorter than 100 ms; treat their variability cautiously."])
    (out / "REPORT.md").write_text("\n".join(lines) + "\n")
    if args.plot:
        import matplotlib.pyplot as plt
        fig, axes = plt.subplots(1, 2, figsize=(13, 5), layout="constrained")
        for index, mode in enumerate(MODES):
            values = [row for row in results if row["mode"] == mode]
            x = [metadata["batches"].index(row["batch"]) + (index - 1) * .22 for row in values]
            axes[0].bar(x, [row["operations_per_second"] for row in values], width=.22, label=mode)
            ci = [row["lifecycle_ratio"] for row in values]
            axes[1].errorbar(x, [c["median"] for c in ci], yerr=[[c["median"]-c["low"] for c in ci], [c["high"]-c["median"] for c in ci]], fmt="o", capsize=3, label=mode)
        axes[0].set_ylabel("Lifecycle operations/s (higher is better)")
        axes[1].set_ylabel("Lifecycle time / legacy (lower is better)")
        axes[1].axhline(1, color="grey", linestyle="--")
        for axis in axes:
            axis.set_xticks(range(len(metadata["batches"])), metadata["batches"])
            axis.set_xlabel("Batch size")
            axis.set_ylim(bottom=0)
            axis.legend()
        fig.suptitle(f"{metadata['target']}: concrete leaves, cleanup included; {metadata['repeats']} paired process blocks")
        for extension in ["svg", "png"]:
            fig.savefig(out / f"lifecycle.{extension}", dpi=160)
        plt.close(fig)
    return 0


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="command", required=True)
    execute = commands.add_parser("run")
    execute.add_argument("--binary", type=Path, required=True)
    execute.add_argument("--provenance", type=Path, required=True)
    execute.add_argument("--sources", type=Path, required=True)
    execute.add_argument("--wasmtime", type=Path)
    execute.add_argument("--output", type=Path, required=True)
    execute.add_argument("--kind", choices=["timing", "counts"], required=True)
    execute.add_argument("--cpu", type=int, required=True)
    execute.add_argument("--operations", type=int, default=100000)
    execute.add_argument("--batches", type=int, nargs="+", default=[64, 1024, 16384])
    execute.add_argument("--repeats", type=int, default=7)
    execute.add_argument("--seed", type=int, default=20261115)
    summarize = commands.add_parser("report")
    summarize.add_argument("--output", type=Path, required=True)
    summarize.add_argument("--plot", action="store_true")
    args = parser.parse_args()
    return run(args) if args.command == "run" else report(args)


if __name__ == "__main__":
    raise SystemExit(main())
