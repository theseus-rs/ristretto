"""Rebuild summaries from raw results without averaging or pooling percentiles."""
import argparse
from collections import defaultdict
import json
from pathlib import Path
import statistics


def expected_checksum(workload, count):
    cycles, tail = divmod(count, 128)
    marks = cycles * 8128 + tail * (tail - 1) // 2
    if workload == "control":
        return marks
    if workload == "roots":
        return 16 * marks + 120 * count
    return 2 * marks


def validate(data, workload, rate, seconds, seed):
    if data["schema"] != 1 or data["workload"] != workload or data["rate"] != rate or data["seed"] != seed:
        raise ValueError("workload identity mismatch")
    count = data["completed"]
    if not isinstance(count, int) or count <= 0 or data["elapsed_ns"] <= 0:
        raise ValueError("no completed work")
    if data["duration_ns"] != seconds * 10**9:
        raise ValueError("duration mismatch")
    if data["checksum"] != expected_checksum(workload, count):
        raise ValueError("workload checksum mismatch")
    offered = rate * seconds
    if data["offered"] != offered or (rate and count > offered):
        raise ValueError("offered count mismatch")
    if data["unfinished"] != (max(0, offered - count) if rate else 0):
        raise ValueError("unfinished work omitted")
    for metric in ["service", "response"]:
        bins = data[metric]["buckets_us"]
        if not bins or any(b[0] < 0 or b[1] <= 0 for b in bins):
            raise ValueError("invalid histogram")
        if any(a[0] >= b[0] for a, b in zip(bins, bins[1:])) or sum(b[1] for b in bins) != count:
            raise ValueError("histogram order/count mismatch")
        if data[metric]["max_ns"] < 0 or bins[-1][0] * 1000 < data[metric]["max_ns"]:
            raise ValueError("histogram maximum mismatch")
    if data["response"]["max_ns"] < data["service"]["max_ns"]:
        raise ValueError("response cannot be shorter than service maximum")


def quantile(histogram, numerator, denominator=10000):
    bins = histogram["buckets_us"]
    rank = (sum(c for _, c in bins) * numerator + denominator - 1) // denominator
    seen = 0
    for upper, count in bins:
        seen += count
        if seen >= rank:
            return upper
    raise ValueError("empty histogram")


def summarize(rows):
    groups = defaultdict(list)
    failures, diagnostics = [], []
    for row in rows:
        if row["status"] != "ok":
            failures.append({k: row[k] for k in ["runtime", "workload", "rate", "status", "directory"]})
            continue
        if row.get("diagnostic_only") or row.get("pause_evidence") is not None:
            diagnostics.append({k: row[k] for k in ["runtime", "workload", "rate", "directory"]})
            continue
        groups[(row["runtime"], row["workload"], row["rate"], row["target"])].append(row)
    results = []
    for (runtime, workload, rate, target), runs in sorted(groups.items()):
        samples = defaultdict(list)
        for row in runs:
            data = row["measurement"]
            window = max(data["duration_ns"], data["elapsed_ns"]) if rate else data["elapsed_ns"]
            samples["throughput_ops_s"].append(data["completed"] * 1e9 / window)
            for kind in ["service", "response"]:
                for label, rank in [("p50", 5000), ("p99", 9900), ("p999", 9990), ("p9999", 9999)]:
                    samples[f"{kind}_{label}_us"].append(quantile(data[kind], rank))
                samples[f"{kind}_max_us"].append(data[kind]["max_ns"] / 1000)
            samples["unfinished"].append(data["unfinished"])
            samples["peak_backlog"].append(data["peak_backlog"])
            samples["completed"].append(data["completed"])
            if row.get("resources"):
                resource = row["resources"]
                samples["process_peak_rss_mib"].append(resource["peak_rss_kib"] / 1024)
                samples["process_cpu_s"].append(resource["user_s"] + resource["system_s"])
        results.append({"runtime": runtime, "workload": workload, "rate": rate, "target": target, "runs": len(runs),
                        "runs_with_process_gc": sum(row.get("gc_observation", {}).get("activity_observed") is True for row in runs),
                        "tail_samples_sufficient": min(samples["completed"]) >= 1_000_000,
                        "metrics": {k: {"median": statistics.median(v), "min": min(v), "max": max(v)} for k, v in samples.items()}})
    return {"schema": 1, "qualification": False, "aggregation": "median and range of per-process metrics; quantiles are not pooled",
            "groups": results, "failures": failures, "diagnostics": diagnostics}


def write_report(directory):
    directory = Path(directory)
    rows = [json.loads(line) for line in (directory / "raw.jsonl").read_text().splitlines() if line]
    summary = summarize(rows)
    if (directory / "schedule.json").exists():
        scheduled = json.loads((directory / "schedule.json").read_text())
        observed = [(r["repeat"], r["workload"], r["rate"], r["runtime"]) for r in rows]
        summary["schedule_complete"] = sorted(map(tuple, scheduled)) == sorted(observed)
    else:
        summary["schedule_complete"] = False
    summary["integrity"] = json.loads((directory / "integrity.json").read_text()) if (directory / "integrity.json").exists() else {"inputs_unchanged": None}
    (directory / "summary.json").write_text(json.dumps(summary, indent=2) + "\n")
    lines = ["# GC workload comparison", "", "Exploratory end-to-end runtime measurements, not isolated collector rankings or completed qualification.",
             "HotSpot uses its default JIT unless the runtime name ends in `-int`; native Ristretto names with `-jit` before the optional `-before` suffix enable its JIT, other Ristretto rows interpret. Wasm is a separate target.",
             "Heap limits constrain different layouts; process peak RSS and CPU include startup, warmup and result output.",
             "Rate 0 is a saturation screen, not proof of sustainable capacity. Positive rates include scheduled-arrival delay; unfinished requests remain visible.",
             "Values below are medians across processes; raw histograms and per-run ranges are retained. Low sample counts do not establish extreme-tail behavior.", "",
             "| Runtime | Workload | Offered ops/s | Runs | Completed ops/s | Service p99.9 µs | Arrival p99.9 µs | Arrival p99.99 µs | Unfinished | RSS MiB | Tail N ≥ 1M/run | Runs observing GC* |",
             "|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---|---:|"]
    for group in summary["groups"]:
        m = group["metrics"]
        def value(key):
            return f"{m[key]['median']:,.1f}" if key in m else "n/a"
        lines.append(f"| {group['runtime']} | {group['workload']} | {group['rate']} | {group['runs']} | "
                     + " | ".join(value(k) for k in ["throughput_ops_s", "service_p999_us", "response_p999_us", "response_p9999_us", "unfinished", "process_peak_rss_mib"])
                     + f" | {'yes' if group['tail_samples_sufficient'] else 'no'} | {group['runs_with_process_gc']} |")
    lines += ["", "*GC observation covers the whole process, including bootstrap/warmup/output; it does not establish measured-window collection exposure. Missing activity cannot support collector-performance claims.",
              f"Failed/invalid/timed-out runs: {len(summary['failures'])}. Input integrity: {summary['integrity']['inputs_unchanged']}. Schedule complete: {summary['schedule_complete']}.",
              "No winner or release gate is inferred from this report. Inspect failures, backlog, CPU/RSS limits, repetitions and longer runs before making a claim.", ""]
    if summary["diagnostics"]:
        lines.append(f"Diagnostic captures excluded from timing groups: {len(summary['diagnostics'])}. "
                     "Publication-only traces do not establish complete GC pause coverage; "
                     "recorder overhead remains unmeasured.")
    for failure in summary["failures"]:
        lines.append(f"- {failure['directory']}: {failure['status']}")
    (directory / "REPORT.md").write_text("\n".join(lines) + "\n")
    return summary


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("results", type=Path)
    write_report(parser.parse_args().results)
