#!/usr/bin/env python3
"""Paired frame-publication helper costs from frozen release VM test binaries.

This is a single-mutator mechanism comparison. It excludes VM setup, warmup,
spill stores, compiled instruction execution and collection; it cannot qualify
whole-JVM performance, contention, latency tails or G1/ZGC superiority.
"""
import argparse
from collections import defaultdict
import hashlib
import json
import os
from pathlib import Path
import platform
import random
import statistics
import subprocess
import tarfile

from mechanisms import paired_interval
from run import digest, execute, freeze_build, optional_text

TEST = "jit_runtime_helpers::publication_bench::measure_frame_publication"
FIXTURE = "ristretto_vm/src/jit_runtime_helpers/publication_bench.rs"
WORKLOADS = ["empty", "stable1", "stable8", "duplicates", "changing1", "changing8", "stable64"]
POLICIES = ["gated", "counted"]


def validate(data, workload, policy, operations, warmup):
    target_count = {"empty": 0, "stable1": 1, "stable8": 8, "duplicates": 1,
                    "changing1": 2, "changing8": 16, "stable64": 64}[workload]
    expected = {"schema_version": 2, "workload": workload, "policy": policy,
                "operations": operations, "warmup": warmup, "checksum": operations,
                "target_count": target_count, "payload_checksum": target_count * 42,
                "survivors": 0, "collections_during": 0,
                "registered_workers_before": 0, "registered_workers_after": 0,
                "execution_profile": False, "qualification": False}
    if any(type(data.get(key)) is not type(value) or data[key] != value for key, value in expected.items()):
        raise ValueError("fixture identity, ownership, collection or execution-mode mismatch")
    if type(data.get("elapsed_ns")) is not int or data["elapsed_ns"] <= 0:
        raise ValueError("missing positive measured duration")
    return data


def parse_measurement(stdout):
    rows = [json.loads(line.partition("FRAME_PUBLICATION ")[2])
            for line in stdout.splitlines() if "FRAME_PUBLICATION " in line]
    if len(rows) != 1:
        raise ValueError("expected exactly one helper measurement")
    return rows[0]


def fixture_digest(archive):
    with tarfile.open(archive) as sources:
        with sources.extractfile(FIXTURE) as stream:
            return hashlib.file_digest(stream, "sha256").hexdigest()


def summarize(rows, repeats, workloads, seed):
    groups = defaultdict(dict)
    failures = []
    seen = set()
    for index, row in enumerate(rows):
        key = row["workload"], row["policy"], row["repeat"], row["revision"]
        if (row["workload"] not in workloads or row["policy"] not in POLICIES
                or row["revision"] not in {"before", "after"}
                or type(row["repeat"]) is not int or not 0 <= row["repeat"] < repeats):
            raise ValueError("process outside requested paired grid")
        if key in seen:
            raise ValueError("duplicate process in paired grid")
        seen.add(key)
        if row["status"] != "ok":
            failures.append(index)
            continue
        groups[row["workload"], row["policy"]].setdefault(row["repeat"], {})[row["revision"]] = (
            row["measurement"]["elapsed_ns"] / row["measurement"]["operations"])
    results, incomplete = [], []
    for work in workloads:
        for policy in POLICIES:
            cells = groups[work, policy]
            if (set(cells) != set(range(repeats))
                    or any(set(cell) != {"before", "after"} for cell in cells.values())):
                incomplete.append({"workload": work, "policy": policy})
                continue
            pairs = [(cells[repeat]["before"], cells[repeat]["after"]) for repeat in range(repeats)]
            results.append({"workload": work, "policy": policy,
                            "before_ns": statistics.median(a for a, _ in pairs),
                            "after_ns": statistics.median(b for _, b in pairs),
                            "ratio": paired_interval(pairs, seed)})
    return {"results": results, "failed_processes": failures, "incomplete_cells": incomplete,
            "qualification": False}


def report(output, summary, repeats, plot):
    lines = ["# Frame-publication helper cost", "", __doc__.strip(), "",
             "Ratios below one favor the new binary. Intervals are exploratory 95% paired-process bootstrap intervals; they do not represent per-operation latency tails.", "",
             "| Workload | Policy | Before ns/call | After ns/call | After/before [95% CI] |",
             "|---|---|---:|---:|---:|"]
    for result in summary["results"]:
        ci = result["ratio"]
        lines.append(f"| {result['workload']} | {result['policy']} | {result['before_ns']:.2f} | {result['after_ns']:.2f} | {ci['median']:.3f} [{ci['low']:.3f}, {ci['high']:.3f}] |")
    lines.extend(["", f"Failed processes: {len(summary['failed_processes'])}; incomplete cells: {len(summary['incomplete_cells'])}; inputs unchanged: {summary['inputs_unchanged']}."])
    (output / "REPORT.md").write_text("\n".join(lines) + "\n")
    if plot:
        import matplotlib.pyplot as plt
        fig, axes = plt.subplots(1, 2, figsize=(13, 5), layout="constrained")
        for axis, policy in zip(axes, POLICIES):
            values = [r for r in summary["results"] if r["policy"] == policy]
            for x, value in enumerate(values):
                ci = value["ratio"]
                axis.errorbar(x, ci["median"], yerr=[[ci["median"] - ci["low"]], [ci["high"] - ci["median"]]], fmt="o", capsize=4)
            axis.set_xticks(range(len(values)), [r["workload"] for r in values], rotation=30, ha="right")
            axis.axhline(1, linestyle="--", color="grey")
            axis.set_ylim(bottom=0)
            axis.set_title(policy)
            axis.set_ylabel("After / before ns per helper call (lower is better)")
        fig.suptitle(f"Frame publication — {repeats} process pairs; no concurrent collection")
        for extension in ["svg", "png"]:
            fig.savefig(output / f"frame-roots.{extension}", dpi=160)
        plt.close(fig)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for label in ["before", "after", "jdk", "output"]:
        parser.add_argument(f"--{label}", type=Path, required=True)
    parser.add_argument("--cpu", type=int, required=True)
    parser.add_argument("--repeats", type=int, default=7)
    parser.add_argument("--operations", type=int, default=100000)
    parser.add_argument("--warmup", type=int, default=10000)
    parser.add_argument("--seed", type=int, default=42)
    parser.add_argument("--timeout", type=int, default=120)
    parser.add_argument("--workloads", nargs="+", choices=WORKLOADS, default=WORKLOADS)
    parser.add_argument("--plot", action="store_true")
    args = parser.parse_args()
    if (args.cpu not in os.sched_getaffinity(0) or args.repeats < 2 or args.timeout <= 0
            or not 1 <= args.operations <= 10000000 or not 1 <= args.warmup <= 10000000
            or len(set(args.workloads)) != len(args.workloads)):
        parser.error("invalid affinity, count, timeout or repeated workload")
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    artifacts = {}
    binaries = {}
    fixture_hashes = {}
    for label in ["before", "after"]:
        destination = output / label
        paths = freeze_build(getattr(args, label).resolve(), destination)
        artifacts.update({str(path): digest(path) for path in paths})
        fixture_hashes[label] = fixture_digest(paths[2])
        binaries[label] = str(destination)
    if len(set(fixture_hashes.values())) != 1:
        raise ValueError("paired binaries must contain the identical benchmark fixture")
    args.jdk = args.jdk.resolve()
    jdk_inputs = {str(args.jdk / relative): digest(args.jdk / relative)
                  for relative in ["release", "lib/modules"]}
    artifacts.update(jdk_inputs)
    for name in ["frame_roots.py", "mechanisms.py", "run.py", "report.py", "cgroups.py"]:
        path = Path(__file__).with_name(name)
        destination = output / name
        destination.write_bytes(path.read_bytes())
        artifacts[str(destination)] = digest(destination)
    metadata = {"host": platform.platform(), "cpu": args.cpu, "repeats": args.repeats,
                "operations": args.operations, "warmup": args.warmup, "seed": args.seed,
                "workloads": args.workloads, "policies": POLICIES, "heap_mib": 64,
                "collector_workers": 0, "root_storage": "Contexts",
                "binaries": binaries, "artifacts": artifacts, "fixture_sha256": fixture_hashes,
                "jdk_version": subprocess.check_output([str(args.jdk / "bin/java"), "-version"], stderr=subprocess.STDOUT, text=True),
                "scope": __doc__, "qualification": False}
    (output / "metadata.json").write_text(json.dumps(metadata, indent=2) + "\n")
    rng = random.Random(args.seed)
    schedule = []
    for repeat in range(args.repeats):
        conditions = [(work, policy) for work in args.workloads for policy in POLICIES]
        rng.shuffle(conditions)
        for work, policy in conditions:
            revisions = ["before", "after"]
            rng.shuffle(revisions)
            schedule.extend((repeat, work, policy, revision) for revision in revisions)
    (output / "schedule.json").write_text(json.dumps(schedule) + "\n")
    rows = []
    with (output / "raw.jsonl").open("w") as raw:
        for index, (repeat, work, policy, revision) in enumerate(schedule):
            directory = output / f"run-{index:04d}"
            directory.mkdir()
            command = [binaries[revision], TEST, "--exact", "--ignored", "--nocapture", "--test-threads=1"]
            environment = dict(os.environ, RISTRETTO_FRAME_WORKLOAD=work, RISTRETTO_FRAME_POLICY=policy,
                               RISTRETTO_FRAME_OPERATIONS=str(args.operations), RISTRETTO_FRAME_WARMUP=str(args.warmup),
                               RISTRETTO_FRAME_JDK=str(args.jdk))
            row = {"repeat": repeat, "workload": work, "policy": policy, "revision": revision,
                   "command": command, "directory": str(directory)}
            row.update(execute(command, directory, args.timeout, [args.cpu], environment))
            if row["status"] == "ok":
                try:
                    row["measurement"] = validate(parse_measurement(optional_text(directory / "stdout.txt") or ""), work, policy, args.operations, args.warmup)
                except (ValueError, KeyError, TypeError) as error:
                    row.update(status="invalid", error=str(error))
            rows.append(row)
            raw.write(json.dumps(row) + "\n")
            raw.flush()
    summary = summarize(rows, args.repeats, args.workloads, args.seed)
    summary["inputs_unchanged"] = all(digest(Path(path)) == sha for path, sha in artifacts.items())
    (output / "summary.json").write_text(json.dumps(summary, indent=2) + "\n")
    report(output, summary, args.repeats, args.plot)
    return int(bool(summary["failed_processes"] or summary["incomplete_cells"]) or not summary["inputs_unchanged"])


if __name__ == "__main__":
    raise SystemExit(main())
