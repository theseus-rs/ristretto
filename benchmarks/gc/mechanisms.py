#!/usr/bin/env python3
"""Paired collector-only ablations with raw evidence and optional graphs."""
import argparse
from collections import defaultdict
import hashlib
import json
import os
from pathlib import Path
import platform
import random
import shutil
import statistics
import subprocess


def sha256(path):
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def paired_interval(pairs, seed=42, draws=10000):
    """Median of paired ratios; resample independent process pairs, not operations."""
    if len(pairs) < 2 or any(a <= 0 or b <= 0 for a, b in pairs):
        raise ValueError("need at least two positive independent pairs")
    ratios = [b / a for a, b in pairs]
    rng = random.Random(seed)
    samples = sorted(statistics.median(rng.choices(ratios, k=len(ratios))) for _ in range(draws))
    return {"median": statistics.median(ratios), "low": samples[int(draws * .025)],
            "high": samples[min(draws - 1, int(draws * .975))], "pairs": len(pairs)}


def validate_target(before, after, wasmtime):
    modules = []
    for path in [before, after]:
        with path.open("rb") as stream:
            modules.append(stream.read(4) == b"\0asm")
    if modules[0] != modules[1] or modules[0] != bool(wasmtime):
        raise ValueError("compare two native executables without --wasmtime, or two Wasm modules with --wasmtime")
    return "WASI" if wasmtime else "native"


INVOCATION_NAME = "mechanisms"


def invocation(binary, arguments, runner=None):
    """Keep argv[0] independent of the frozen binary path."""
    if runner:
        return [str(runner), "run", "-W", "max-wasm-stack=8388608", "--argv0",
                INVOCATION_NAME, str(binary), *arguments], None
    return [INVOCATION_NAME, *arguments], str(binary)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--before", type=Path, required=True)
    parser.add_argument("--after", type=Path, required=True)
    parser.add_argument("--wasmtime", type=Path, help="execute both frozen WASI modules with this Wasmtime binary")
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--cpu", type=int, required=True)
    parser.add_argument("--repeats", type=int, default=7)
    parser.add_argument("--operations", type=int, default=100000)
    parser.add_argument("--seed", type=int, default=42)
    parser.add_argument("--plot", action="store_true")
    parser.add_argument("--workloads", nargs="+", choices=["control", "allocation", "leaf_allocation", "owning_allocation", "cell_allocation", "construction_batches", "nested_write", "roots", "clones", "guard_clones", "edge_clones", "unroot", "handles", "weak", "pending_poll", "pending_transfer", "barrier", "barrier4", "barrier16", "barrier64", "overwrite", "leaf_write"],
                        default=["control", "allocation", "roots", "barrier", "overwrite"])
    parser.add_argument("--policies", nargs="+", choices=["baseline", "counted", "contexts", "local", "candidate"],
                        default=["baseline", "counted", "contexts", "local", "candidate"])
    args = parser.parse_args()
    if len(set(args.policies)) != len(args.policies) or len(set(args.workloads)) != len(args.workloads):
        parser.error("policies and workloads must be distinct")
    if args.repeats < 2 or args.cpu not in os.sched_getaffinity(0):
        parser.error("need at least two pairs and an available CPU")
    try:
        target = validate_target(args.before, args.after, args.wasmtime)
    except ValueError as error:
        parser.error(str(error))
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    binaries = {}
    for label in ["before", "after"]:
        path = output / label
        shutil.copy2(getattr(args, label), path)
        binaries[label] = {"path": str(path), "sha256": sha256(path)}
    runner = None
    if args.wasmtime:
        path = output / "wasmtime"
        shutil.copy2(args.wasmtime, path)
        version = subprocess.run([str(path), "--version"], capture_output=True, text=True, check=True).stdout.strip()
        runner = {"path": str(path), "sha256": sha256(path), "version": version}
    harness = output / "mechanisms.py"
    shutil.copy2(Path(__file__), harness)
    metadata = {"binaries": binaries, "runner": runner, "target": target,
                "harness_sha256": sha256(harness), "host": platform.platform(), "cpu": args.cpu,
                "repeats": args.repeats, "seed": args.seed, "operations": args.operations,
                "workloads": args.workloads, "policies": args.policies, "argv0": INVOCATION_NAME,
                "qualification": False, "scope": "single mutator, operation time excludes between-batch collection"}
    (output / "metadata.json").write_text(json.dumps(metadata, indent=2) + "\n")
    groups = defaultdict(dict)
    jobs = []
    rng = random.Random(args.seed)
    for repeat in range(args.repeats):
        conditions = [(w, p, d) for w in args.workloads
                      for p in args.policies for d in [False, True]]
        rng.shuffle(conditions)
        for work, policy, diagnostics in conditions:
            labels = list(binaries)
            rng.shuffle(labels)
            jobs.extend((repeat, work, policy, diagnostics, label) for label in labels)
    (output / "schedule.json").write_text(json.dumps(jobs) + "\n")
    failures = []
    with (output / "raw.jsonl").open("w") as raw:
        for index, (repeat, work, policy, diagnostics, label) in enumerate(jobs):
            command, executable = invocation(binaries[label]["path"],
                [work, policy, str(args.operations), str(diagnostics).lower()],
                runner["path"] if runner else None)
            row = {"repeat": repeat, "workload": work, "policy": policy, "diagnostics": diagnostics,
                   "binary": label, "command": command, "executable": executable}
            try:
                child = subprocess.run(command, executable=executable, capture_output=True, text=True, timeout=120,
                                       preexec_fn=lambda: os.sched_setaffinity(0, [args.cpu]))
                row.update(stdout=child.stdout, stderr=child.stderr, exit_code=child.returncode)
                child.check_returncode()
                measurement = json.loads(child.stdout)
                if (measurement["checksum"] != 42 * args.operations or measurement["operations"] != args.operations
                        or measurement["teardown_live"] != 0 or measurement["operation_ns"] <= 0
                        or measurement["wall_ns"] < measurement["operation_ns"]
                        or (measurement["workload"], measurement["policy"], measurement["diagnostics"]) != (work, policy, diagnostics)):
                    raise ValueError("measurement identity or integrity failure")
                row.update(status="ok", measurement=measurement)
                groups[work, policy, diagnostics].setdefault(repeat, {})[label] = (
                    measurement["operation_ns"] / args.operations, measurement["wall_ns"] / args.operations)
            except (subprocess.SubprocessError, ValueError, KeyError) as error:
                row.update(status="failed", error=str(error))
                failures.append(index)
            raw.write(json.dumps(row) + "\n")
            raw.flush()
    summary = []
    for (work, policy, diagnostics), repeats in sorted(groups.items()):
        complete = [r for r in repeats.values() if set(r) == {"before", "after"}]
        pairs = [(r["before"][0], r["after"][0]) for r in complete]
        wall = [(r["before"][1], r["after"][1]) for r in complete]
        if len(pairs) != args.repeats:
            continue
        summary.append({"workload": work, "policy": policy, "diagnostics": diagnostics,
                        "before_ns": statistics.median(a for a, _ in pairs),
                        "after_ns": statistics.median(b for _, b in pairs),
                        "after_before": paired_interval(pairs, args.seed),
                        "wall_before_ns": statistics.median(a for a, _ in wall),
                        "wall_after_ns": statistics.median(b for _, b in wall),
                        "wall_after_before": paired_interval(wall, args.seed)})
    unchanged = all(sha256(Path(v["path"])) == v["sha256"] for v in binaries.values())
    unchanged = unchanged and (runner is None or sha256(Path(runner["path"])) == runner["sha256"])
    unchanged = unchanged and sha256(harness) == metadata["harness_sha256"] == sha256(Path(__file__))
    (output / "summary.json").write_text(json.dumps({"results": summary, "failures": failures,
                "inputs_unchanged": unchanged, "qualification": False}, indent=2) + "\n")
    lines = ["# Collector mechanism comparison", "", f"Both programs receive argv[0]={INVOCATION_NAME!r}; executable paths are recorded separately. This controls invocation-name allocation, not all layout sensitivity.", "", f"Target: {target}. Timings are inside the executable/guest; process startup and Wasmtime compilation are excluded.", "", "Operation time excludes between-batch collection. Wall time includes collection and is normalized by operation count; it is not per-operation latency.",
             "Ratios below one favor the new binary. Intervals resample process pairs; these are exploratory, unadjusted 95% intervals.", "",
             "| Workload | Policy | Diagnostics | Before ns/op | After ns/op | Operation ratio [95% CI] | Wall ratio [95% CI] |", "|---|---|---|---:|---:|---:|---:|"]
    for s in summary:
        ci = s["after_before"]
        wall = s["wall_after_before"]
        lines.append(f"| {s['workload']} | {s['policy']} | {s['diagnostics']} | {s['before_ns']:.1f} | {s['after_ns']:.1f} | {ci['median']:.3f} [{ci['low']:.3f}, {ci['high']:.3f}] | {wall['median']:.3f} [{wall['low']:.3f}, {wall['high']:.3f}] |")
    lines.extend(["", f"Failed processes: {len(failures)}. Frozen binaries unchanged: {unchanged}."])
    (output / "REPORT.md").write_text("\n".join(lines) + "\n")
    if args.plot:
        import matplotlib.pyplot as plt
        for diagnostics, metric in [(d, m) for d in [False, True] for m in ["operation", "wall"]]:
            fig, axes = plt.subplots(1, len(args.workloads), figsize=(3.4 * len(args.workloads), 5), layout="constrained", squeeze=False)
            for axis, work in zip(axes[0], args.workloads):
                values = [s for s in summary if s["diagnostics"] == diagnostics and s["workload"] == work]
                for x, s in enumerate(values):
                    ci = s["after_before" if metric == "operation" else "wall_after_before"]
                    axis.errorbar(x, ci["median"], yerr=[[ci["median"] - ci["low"]], [ci["high"] - ci["median"]]], fmt="o", capsize=3)
                axis.axhline(1, color="grey", linestyle="--")
                axis.set_xticks(range(len(values)), [s["policy"] for s in values], rotation=45, ha="right")
                axis.set_title(work)
                axis.set_ylabel(f"After / before {metric} time (lower is better)")
                axis.set_ylim(bottom=0)
            fig.suptitle(f"{target} collector mechanisms, {metric} time, diagnostics {'on' if diagnostics else 'off'} — {args.repeats} process pairs")
            for extension in ["svg", "png"]:
                infix = "" if metric == "operation" else "-wall"
                fig.savefig(output / f"mechanisms{infix}-diagnostics-{diagnostics}.{extension}", dpi=160)
            plt.close(fig)
    return int(bool(failures) or not unchanged)


if __name__ == "__main__":
    raise SystemExit(main())
