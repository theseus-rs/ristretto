#!/usr/bin/env python3
"""Compare complete process pairs from a frozen runtime screen; never qualify a release."""
import argparse
import hashlib
import json
from pathlib import Path
import shutil
import statistics

from mechanisms import paired_interval
from report import quantile, validate


def compare(metadata, rows, integrity, before, after):
    options = metadata["options"]
    if before == after or not {before, after} <= set(options["runtimes"]):
        raise ValueError("select two distinct measured runtimes")
    if not integrity.get("inputs_unchanged"):
        raise ValueError("measurement inputs did not remain unchanged")
    if options["repeats"] < 2:
        raise ValueError("at least two independent process pairs are required")
    expected = {(work, rate, repeat, runtime)
                for work in options["workloads"] for rate in options["rates"]
                for repeat in range(options["repeats"]) for runtime in [before, after]}
    selected = {}
    targets, budgets = set(), set()
    for row in rows:
        if row["runtime"] not in [before, after]:
            continue
        key = row["workload"], row["rate"], row["repeat"], row["runtime"]
        if key not in expected or key in selected:
            raise ValueError(f"unexpected or duplicate pair member: {key}")
        if row["status"] != "ok":
            raise ValueError(f"failed pair member: {key}: {row['status']}")
        validate(row["measurement"], row["workload"], row["rate"], options["seconds"], options["seed"])
        budget = row.get("cgroup")
        if budget and budget["memory_events"].get("oom", 0):
            raise ValueError(f"memory exhaustion in pair: {key}")
        budgets.add(json.dumps(budget["limits"] if budget else None, sort_keys=True))
        targets.add(row["target"])
        selected[key] = row
    if set(selected) != expected:
        raise ValueError("incomplete process pairs; refusing survivor-only comparison")
    if len(targets) != 1 or not targets <= {"native", "wasm"} or len(budgets) != 1:
        raise ValueError("paired targets and enforced resource limits must match")
    groups = []
    for work in options["workloads"]:
        for rate in options["rates"]:
            pairs = [(selected[work, rate, repeat, before], selected[work, rate, repeat, after])
                     for repeat in range(options["repeats"])]
            complete = all(not row["measurement"]["unfinished"] for pair in pairs for row in pair)
            enough_tail_samples = all(row["measurement"]["completed"] >= 1_000_000
                                      for pair in pairs for row in pair)
            def throughput(row):
                data = row["measurement"]
                window = max(data["elapsed_ns"], data["duration_ns"]) if rate else data["elapsed_ns"]
                return data["completed"] * 1e9 / window
            metrics = [
                ("throughput_ops_s", throughput, "higher", None),
                ("response_p999_us", lambda r: quantile(r["measurement"]["response"], 9990),
                 "lower", None if complete else "unfinished requests censor tails"),
                ("response_p9999_us", lambda r: quantile(r["measurement"]["response"], 9999),
                 "lower", None if complete else "unfinished requests censor tails"),
            ]
            if all((row.get("cgroup") or {}).get("memory_peak", 0) > 0 for pair in pairs for row in pair):
                metrics.append(("charged_peak_mib", lambda r: r["cgroup"]["memory_peak"] / 1024**2,
                                "lower", None))
            observations = {}
            for name, metric, direction, unavailable in metrics:
                samples = [(metric(a), metric(b)) for a, b in pairs]
                if unavailable is None and any(a <= 0 or b <= 0 for a, b in samples):
                    unavailable = "zero-valued samples have no finite multiplicative comparison"
                observations[name] = {
                    "before_median": statistics.median(a for a, _ in samples),
                    "after_median": statistics.median(b for _, b in samples),
                    "after_before": None if unavailable else paired_interval(samples, options["seed"]),
                    "better": direction, "unavailable": unavailable,
                }
            groups.append({"workload": work, "rate": rate, "complete": complete,
                           "tail_samples_sufficient": enough_tail_samples,
                           "unfinished_before": [a["measurement"]["unfinished"] for a, _ in pairs],
                           "unfinished_after": [b["measurement"]["unfinished"] for _, b in pairs],
                           "metrics": observations})
    return {"qualification": False, "before": before, "after": after,
            "target": next(iter(targets)), "pairs": options["repeats"],
            "resource_caps_enforced": next(iter(budgets)) != "null",
            "interval": "exploratory unadjusted 95% bootstrap interval of paired ratios",
            "groups": groups}


def write_report(result, output, plot):
    lines = ["# Runtime revision comparison", "",
             f"Before: {result['before']}. After: {result['after']}. Target: {result['target']}.",
             "Independent process pairs; exploratory, unadjusted 95% intervals. "
             "Rate zero is a saturation screen, not sustainable capacity. "
             "Low sample counts do not establish extreme-tail behavior. Qualification is false.", "",
             "| Workload | Rate | Metric | Before median | After median | After/before [95% CI] | Better | Tail N ≥ 1M/run |",
             "|---|---:|---|---:|---:|---|---|---|"]
    for group in result["groups"]:
        for metric, value in group["metrics"].items():
            ci = value["after_before"]
            ratio = (f"{ci['median']:.3f} [{ci['low']:.3f}, {ci['high']:.3f}]" if ci
                     else value["unavailable"])
            lines.append(f"| {group['workload']} | {group['rate']} | {metric} | "
                         f"{value['before_median']:.3f} | {value['after_median']:.3f} | "
                         f"{ratio} | {value['better']} | {group['tail_samples_sufficient']} |")
    (output / "REPORT.md").write_text("\n".join(lines) + "\n")
    if not plot:
        return
    import matplotlib.pyplot as plt
    metrics = [("throughput_ops_s", "Throughput (higher is better)"),
               ("response_p999_us", "Response p99.9 (lower is better)"),
               ("charged_peak_mib", "Charged process peak (lower is better)")]
    fig, axes = plt.subplots(1, 3, figsize=(14, 5), layout="constrained")
    labels = [f"{g['workload']} @ {g['rate'] or 'sat'}" for g in result["groups"]]
    for axis, (metric, title) in zip(axes, metrics):
        for index, group in enumerate(result["groups"]):
            value = group["metrics"].get(metric)
            ci = value["after_before"] if value else None
            if ci:
                axis.errorbar(index, ci["median"],
                              yerr=[[ci["median"] - ci["low"]], [ci["high"] - ci["median"]]],
                              fmt="o", capsize=3)
            else:
                axis.text(index, 1, "N/A", ha="center", va="bottom", fontsize=8)
        axis.axhline(1, color="grey", linestyle="--")
        axis.set_xticks(range(len(labels)), labels, rotation=45, ha="right")
        axis.set_title(title)
        axis.set_ylabel("After / before")
    fig.suptitle(f"Runtime revision screen — {result['pairs']} process pairs; qualification incomplete")
    for extension in ["svg", "png"]:
        fig.savefig(output / f"revision.{extension}", dpi=160)
    plt.close(fig)


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("directory", type=Path)
    parser.add_argument("--before", required=True)
    parser.add_argument("--after", required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--plot", action="store_true")
    args = parser.parse_args()
    directory = args.directory.resolve()
    inputs = [directory / name for name in ["metadata.json", "raw.jsonl", "integrity.json"]]
    hashes = {str(path): digest(path) for path in inputs}
    result = compare(json.loads(inputs[0].read_text()),
                     [json.loads(line) for line in inputs[1].read_text().splitlines() if line],
                     json.loads(inputs[2].read_text()), args.before, args.after)
    if any(digest(Path(path)) != value for path, value in hashes.items()):
        raise ValueError("raw inputs changed during analysis")
    args.output.mkdir(parents=True, exist_ok=False)
    source_dir = args.output / "analysis_sources"
    source_dir.mkdir()
    for name in ["revisions.py", "mechanisms.py", "report.py"]:
        source = Path(__file__).with_name(name)
        shutil.copy2(source, source_dir / name)
        hashes[str(source.resolve())] = digest(source)
    (args.output / "provenance.json").write_text(json.dumps(hashes, indent=2) + "\n")
    (args.output / "comparison.json").write_text(json.dumps(result, indent=2) + "\n")
    write_report(result, args.output, args.plot)


if __name__ == "__main__":
    main()
