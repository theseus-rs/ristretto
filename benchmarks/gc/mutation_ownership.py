#!/usr/bin/env python3
"""Frozen, paired mutable-borrow costs; this is not a GC qualification run."""
import argparse
from collections import defaultdict
import json
import os
from pathlib import Path
import platform
import random
import shutil
import statistics
import subprocess

from mechanisms import paired_interval, sha256, validate_target


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--before", type=Path, required=True)
    parser.add_argument("--after", type=Path, required=True)
    parser.add_argument("--wasmtime", type=Path)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--cpu", type=int, required=True)
    parser.add_argument("--repeats", type=int, default=7)
    parser.add_argument("--operations", type=int, default=100000)
    parser.add_argument("--seed", type=int, default=20261109)
    parser.add_argument("--baseline-limitation", default="before lacks safe extracted-value ownership; its benchmark source stays rooted, so this cost control does not validate escaping values")
    parser.add_argument("--workloads", nargs=4,
                        choices=["field-store", "element-borrow", "slice-borrow", "load",
                                 "primitive-borrow", "primitive-bytes", "primitive-whole", "primitive-load"],
                        default=["field-store", "element-borrow", "slice-borrow", "load"])
    parser.add_argument("--plot", action="store_true")
    args = parser.parse_args()
    if args.repeats < 5 or args.operations < 32 or args.cpu not in os.sched_getaffinity(0):
        parser.error("need five pairs, at least 32 operations and an available CPU")
    target = validate_target(args.before, args.after, args.wasmtime)
    if sha256(args.before) == sha256(args.after):
        parser.error("before and after are identical; verify source-to-binary binding")
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    inputs = {}
    for label, source in [("before", args.before), ("after", args.after),
                          ("mutation_ownership.py", Path(__file__)),
                          ("mechanisms.py", Path(__file__).with_name("mechanisms.py"))]:
        dest = output / label
        shutil.copy2(source, dest)
        inputs[label] = {"path": str(dest), "sha256": sha256(dest)}
    runner = None
    if args.wasmtime:
        dest = output / "wasmtime"
        shutil.copy2(args.wasmtime, dest)
        inputs["wasmtime"] = {"path": str(dest), "sha256": sha256(dest)}
        runner = str(dest)
    policies = {"gated-host": (False, False, False),
                "counted-host": (True, False, False),
                "counted-compact-contexts": (True, True, True)}
    workloads = args.workloads
    if len(set(workloads)) != len(workloads):
        parser.error("workloads must be distinct")
    rng = random.Random(args.seed)
    jobs = []
    for repeat in range(args.repeats):
        conditions = [(mode, length, policy) for mode in workloads
                      for length in [1, 32] for policy in policies]
        rng.shuffle(conditions)
        for mode, length, policy in conditions:
            labels = ["before", "after"]
            rng.shuffle(labels)
            operations = args.operations // length if mode == "slice-borrow" else args.operations
            jobs.extend((repeat, mode, length, policy, operations, label) for label in labels)
    metadata = {"target": target, "inputs": inputs, "cpu": args.cpu,
                "argv0": "mutation_ownership",
                "host": platform.platform(), "repeats": args.repeats, "seed": args.seed,
                "policies": policies, "workloads": workloads, "operations": args.operations, "qualification": False,
                "scope": "fixed live graph, one mutator, no workers; in-guest aggregate operation time; no GC during timed region; no latency/capacity/resource gate",
                "baseline_limitation": args.baseline_limitation}
    if runner:
        metadata["wasmtime_version"] = subprocess.check_output([runner, "--version"], text=True).strip()
    (output / "metadata.json").write_text(json.dumps(metadata, indent=2) + "\n")
    (output / "schedule.json").write_text(json.dumps(jobs) + "\n")
    groups = defaultdict(lambda: defaultdict(dict))
    failures = []
    with (output / "raw.jsonl").open("w") as raw:
        for index, (repeat, mode, length, policy, operations, label) in enumerate(jobs):
            counted, compact, contexts = policies[policy]
            arguments = [mode, str(operations), str(length),
                         *[str(value).lower() for value in (counted, compact, contexts)]]
            executable = inputs[label]["path"]
            command = [metadata["argv0"], *arguments]
            if runner:
                command = [runner, "run", "--argv0", metadata["argv0"], executable, *arguments]
                executable = None
            row = {"repeat": repeat, "mode": mode, "length": length, "policy": policy,
                   "operations": operations, "binary": label, "command": command,
                   "executable": executable}
            try:
                child = subprocess.run(command, executable=executable,
                                       capture_output=True, text=True, timeout=60,
                                       preexec_fn=lambda: os.sched_setaffinity(0, [args.cpu]))
                row.update(stdout=child.stdout, stderr=child.stderr, exit_code=child.returncode)
                child.check_returncode()
                value = json.loads(child.stdout)
                expected = dict(mode=mode, operations=operations, length=length, counted=counted,
                                compact=compact, contexts=contexts, live_objects=0, bytes_allocated=0)
                if any(value.get(key) != expected[key] for key in expected) or value["elapsed_ns"] <= 0:
                    raise ValueError("measurement identity or integrity failure")
                row.update(status="ok", measurement=value)
                groups[mode, length, policy][repeat][label] = value["elapsed_ns"] / operations
            except (subprocess.SubprocessError, ValueError, KeyError) as error:
                row.update(status="failed", error=str(error))
                if isinstance(error, subprocess.TimeoutExpired):
                    row["stdout"] = (error.stdout or b"").decode(errors="replace")
                    row["stderr"] = (error.stderr or b"").decode(errors="replace")
                failures.append(index)
            raw.write(json.dumps(row) + "\n")
            raw.flush()
            if (index + 1) % 48 == 0:
                print(f"{target}: {index + 1}/{len(jobs)} processes; failures={len(failures)}", flush=True)
    summaries = []
    for (mode, length, policy), repeats in sorted(groups.items()):
        pairs = [(values["before"], values["after"]) for values in repeats.values()
                 if set(values) == {"before", "after"}]
        if len(pairs) == args.repeats:
            summaries.append(dict(mode=mode, length=length, policy=policy,
                                  before_ns=statistics.median(a for a, _ in pairs),
                                  after_ns=statistics.median(b for _, b in pairs),
                                  ratio=paired_interval(pairs, args.seed)))
    unchanged = all(sha256(Path(item["path"])) == item["sha256"] for item in inputs.values())
    complete = len(summaries) == len(workloads) * 2 * len(policies)
    (output / "summary.json").write_text(json.dumps(dict(results=summaries, failures=failures,
        inputs_unchanged=unchanged, complete=complete, qualification=False), indent=2) + "\n")
    lines = ["# Mutable-value ownership costs", "", f"Target: {target}; {args.repeats} independent process pairs per condition.",
             "Both binaries receive argv[0]='mutation_ownership'; executable paths are recorded separately.",
             "Aggregate in-guest ns/op excludes startup, runtime compilation, setup, validation and collection. This is not operation latency or sustainable capacity.",
             f"Baseline limitation: {args.baseline_limitation}. Ratios above one mean added cost. Exploratory 95% paired-bootstrap intervals are unadjusted for multiple comparisons.", "",
             "| Operation | Length | Policy | Before ns/op | After ns/op | Ratio [95% CI] |",
             "|---|---:|---|---:|---:|---:|"]
    for row in summaries:
        ci = row["ratio"]
        lines.append(f"| {row['mode']} | {row['length']} | {row['policy']} | {row['before_ns']:.1f} | {row['after_ns']:.1f} | {ci['median']:.3f} [{ci['low']:.3f}, {ci['high']:.3f}] |")
    lines.extend(["", f"Failed processes: {len(failures)}. Complete: {complete}. Frozen inputs unchanged: {unchanged}. Qualification: false."])
    (output / "REPORT.md").write_text("\n".join(lines) + "\n")
    if args.plot:
        import matplotlib.pyplot as plt
        fig, axes = plt.subplots(2, 2, figsize=(13, 10), layout="constrained")
        for axis, mode in zip(axes.flat, workloads):
            rows = [row for row in summaries if row["mode"] == mode]
            for index, row in enumerate(rows):
                ci = row["ratio"]
                axis.errorbar(index, ci["median"], yerr=[[ci["median"] - ci["low"]], [ci["high"] - ci["median"]]], fmt="o", capsize=3)
            axis.axhline(1, color="grey", linestyle="--")
            axis.set_xticks(range(len(rows)), [f"n={row['length']}\n{row['policy']}" for row in rows], rotation=25, ha="right")
            axis.set_title(mode)
            axis.set_ylabel("After / before aggregate ns/op")
            axis.set_ylim(bottom=0)
        fig.suptitle(f"{target}: mutable-value ownership cost — {args.repeats} pairs; no qualification claim")
        for extension in ["svg", "png"]:
            fig.savefig(output / f"mutation-ownership.{extension}", dpi=160)
        plt.close(fig)
    return int(bool(failures) or not unchanged or not complete)


if __name__ == "__main__":
    raise SystemExit(main())
