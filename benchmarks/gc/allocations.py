#!/usr/bin/env python3
"""Paired allocator counts and process memory; counter-instrumented runs are not timings."""
import argparse
from collections import defaultdict
import json
import os
from pathlib import Path
import random
import shutil
import statistics

from cgroups import Budget
from mechanisms import paired_interval
from run import digest, execute


def validate(data, policy, operations, mode="ephemeral"):
    if (data["policy"] != policy or data["operations"] != operations
            or data["checksum"] != 42 * operations or data["teardown_live"] != 0):
        raise ValueError("allocation-count identity or teardown failure")
    if data.get("mode", "ephemeral") != mode:
        raise ValueError("allocation-count mode mismatch")
    for name in ["allocations", "allocated_bytes", "deallocations", "released_bytes",
                 "collection_allocations", "collection_allocated_bytes",
                 "collection_deallocations", "collection_released_bytes"]:
        if type(data[name]) is not int or data[name] < 0:
            raise ValueError(f"invalid allocator counter: {name}")
    if mode == "weak_retention":
        for name in ["retained_after_collection_bytes", "released_on_weak_drop_bytes"]:
            if type(data.get(name)) is not int or data[name] < 0:
                raise ValueError(f"invalid weak-retention counter: {name}")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--before", type=Path, required=True)
    parser.add_argument("--after", type=Path, required=True)
    parser.add_argument("--wasmtime", type=Path, help="run both frozen Wasm modules with this Wasmtime binary")
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--cpu", type=int, required=True)
    parser.add_argument("--cgroup-parent", type=Path, required=True)
    parser.add_argument("--rss-mib", type=int, default=512)
    parser.add_argument("--operations", type=int, default=100000)
    parser.add_argument("--mode", choices=["ephemeral", "weak_retention", "construction_batches"], default="ephemeral")
    parser.add_argument("--repeats", type=int, default=5)
    parser.add_argument("--seed", type=int, default=42)
    parser.add_argument("--plot", action="store_true")
    args = parser.parse_args()
    if args.cpu not in os.sched_getaffinity(0) or args.repeats < 2 or not 1 <= args.operations <= 100000:
        parser.error("need an available CPU, at least two pairs, and 1..100000 operations")
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    analysis = output / "analysis_sources"
    analysis.mkdir()
    for name in ["allocations.py", "run.py", "mechanisms.py", "cgroups.py"]:
        shutil.copy2(Path(__file__).with_name(name), analysis / name)
    binaries = {}
    for name in ["before", "after"]:
        binary = output / name
        shutil.copy2(getattr(args, name), binary)
        binaries[name] = {"path": str(binary), "sha256": digest(binary)}
    runtime = None
    if args.wasmtime:
        for binary in binaries.values():
            with Path(binary["path"]).open("rb") as module:
                if module.read(4) != b"\0asm":
                    parser.error("--wasmtime requires two Wasm modules")
        path = output / "wasmtime"
        shutil.copy2(args.wasmtime, path)
        runtime = {"path": str(path), "sha256": digest(path)}
    target = "Wasm" if runtime else "native"
    (output / "metadata.json").write_text(json.dumps({
        "qualification": False, "binaries": binaries, "runtime": runtime, "target": target,
        "analysis_sha256": {p.name: digest(p) for p in analysis.iterdir()},
        "options": {k: str(v) if isinstance(v, Path) else v for k, v in vars(args).items()},
        "scope": "successful system-allocator requests; realloc counts one release and one allocation; requested bytes exclude allocator headers and fragmentation; process cgroup peak includes setup/collection/teardown; no timing claims"
    }, indent=2) + "\n")
    rng = random.Random(args.seed)
    jobs = []
    for repeat in range(args.repeats):
        policies = ["baseline", "candidate"]
        rng.shuffle(policies)
        for policy in policies:
            order = ["before", "after"]
            rng.shuffle(order)
            jobs.extend((repeat, policy, label) for label in order)
    (output / "schedule.json").write_text(json.dumps(jobs) + "\n")
    groups = defaultdict(dict)
    failures = []
    with (output / "raw.jsonl").open("w") as raw:
        for index, (repeat, policy, label) in enumerate(jobs):
            directory = output / f"run-{index:03d}-{policy}-{label}"
            directory.mkdir()
            command = [binaries[label]["path"], policy, str(args.operations), args.mode]
            if runtime:
                command = [runtime["path"], "run", *command]
            row = {"repeat": repeat, "policy": policy, "binary": label, "command": command, "directory": directory.name}
            with_budget = Budget(args.cgroup_parent, args.rss_mib * 1024**2, 100000)
            try:
                row.update(execute(command, directory, 120, [args.cpu], dict(os.environ), with_budget))
                row["cgroup"] = with_budget.snapshot()
            finally:
                with_budget.close()
            try:
                if row["status"] != "ok" or row["cgroup"]["memory_events"].get("oom", 0):
                    raise ValueError("failed or memory-limited process")
                data = json.loads((directory / "stdout.txt").read_text())
                validate(data, policy, args.operations, args.mode)
                row["measurement"] = data
                groups[policy].setdefault(repeat, {})[label] = {
                    "allocations_per_op": data["allocations"] / args.operations,
                    "requested_bytes_per_op": data["allocated_bytes"] / args.operations,
                    "peak_memory_mib": row["cgroup"]["memory_peak"] / 1024**2,
                }
                if args.mode == "weak_retention":
                    groups[policy][repeat][label].update({
                        "retained_after_collection_bytes_per_op": data["retained_after_collection_bytes"] / args.operations,
                        "released_on_weak_drop_bytes_per_op": data["released_on_weak_drop_bytes"] / args.operations,
                    })
            except (ValueError, KeyError, TypeError) as error:
                row.update(status="invalid", error=str(error))
                failures.append(directory.name)
            raw.write(json.dumps(row) + "\n")
            raw.flush()
    summary = []
    metrics = ["allocations_per_op", "requested_bytes_per_op", "peak_memory_mib"]
    if args.mode == "weak_retention":
        metrics += ["retained_after_collection_bytes_per_op", "released_on_weak_drop_bytes_per_op"]
    for policy, repeats in sorted(groups.items()):
        if len(repeats) != args.repeats or any(set(pair) != {"before", "after"} for pair in repeats.values()):
            continue
        for metric in metrics:
            pairs = [(pair["before"][metric], pair["after"][metric]) for pair in repeats.values()]
            summary.append({"policy": policy, "metric": metric,
                            "before": statistics.median(a for a, _ in pairs),
                            "after": statistics.median(b for _, b in pairs),
                            "after_before": paired_interval(pairs, args.seed)})
    unchanged = all(digest(Path(b["path"])) == b["sha256"] for b in binaries.values())
    unchanged = unchanged and (runtime is None or digest(Path(runtime["path"])) == runtime["sha256"])
    (output / "summary.json").write_text(json.dumps({"results": summary, "failures": failures,
        "inputs_unchanged": unchanged, "qualification": False}, indent=2) + "\n")
    lines = ["# System allocation and memory comparison", "", "Instrumented counts, not a timing benchmark. Requested bytes exclude allocator headers and fragmentation. Wasm counters cover the guest allocator; charged process peak also includes Wasmtime. Charged process peak includes setup, collection and teardown. Retained bytes cover all counted allocations while cleared weak handles remain alive; they are not managed live bytes.", f"Target: {target}. Mode: {args.mode}.", "",
             "| Policy | Metric | Before | After | Ratio [95% CI] |", "|---|---|---:|---:|---:|"]
    for row in summary:
        ci = row["after_before"]
        lines.append(f"| {row['policy']} | {row['metric']} | {row['before']:.3f} | {row['after']:.3f} | {ci['median']:.3f} [{ci['low']:.3f}, {ci['high']:.3f}] |")
    lines += ["", f"Failed processes: {len(failures)}. Frozen inputs unchanged: {unchanged}. Qualification: false."]
    (output / "REPORT.md").write_text("\n".join(lines) + "\n")
    if args.plot:
        import matplotlib.pyplot as plt
        fig, axes = plt.subplots(1, len(metrics), figsize=(4 * len(metrics), 4), layout="constrained")
        for axis, metric in zip(axes, metrics):
            rows = [r for r in summary if r["metric"] == metric]
            for i, row in enumerate(rows):
                axis.bar(i - .18, row["before"], width=.36, color="#64748b", label="Before" if i == 0 else None)
                axis.bar(i + .18, row["after"], width=.36, color="#0284c7", label="After" if i == 0 else None)
            axis.set_xticks(range(len(rows)), [r["policy"] for r in rows])
            axis.set_title(metric.replace("_", " "))
            axis.legend()
        fig.suptitle(f"{target} allocator counts and process memory — {args.repeats} independent pairs; medians")
        for suffix in ["svg", "png"]:
            fig.savefig(output / f"allocations.{suffix}", dpi=160)
        plt.close(fig)
    return int(bool(failures) or not unchanged)


if __name__ == "__main__":
    raise SystemExit(main())
