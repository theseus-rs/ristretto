#!/usr/bin/env python3
"""Historical peer-ratio policy v1, retained unchanged for old evidence only."""
import argparse
from collections import defaultdict
import json
from pathlib import Path

from mechanisms import paired_interval
from report import quantile, validate


def evaluate(directory, candidate="ristretto-candidate"):
    metadata = json.loads((directory / "metadata.json").read_text())
    options = metadata["options"]
    rows = [json.loads(line) for line in (directory / "raw.jsonl").read_text().splitlines()]
    failures = []
    if not json.loads((directory / "integrity.json").read_text())["inputs_unchanged"]:
        failures.append("inputs changed during measurement")
    groups = defaultdict(dict)
    limits = set()
    for row in rows:
        if row["runtime"] not in [candidate, "g1", "zgc"]:
            continue
        key = (row["workload"], row["rate"], row["runtime"])
        if row["repeat"] in groups[key]:
            failures.append(f"duplicate process pair: {key}, repeat {row['repeat']}")
        groups[key][row["repeat"]] = row
        try:
            if row["status"] != "ok":
                raise ValueError(row["status"])
            data = row["measurement"]
            validate(data, row["workload"], row["rate"], options["seconds"], options["seed"])
            if row["target"] != "native":
                raise ValueError("external qualification requires native peers")
            if data["unfinished"]:
                raise ValueError("unfinished requests censor latency")
            if data["completed"] < 1_000_000:
                raise ValueError("fewer than one million completions for extreme-tail claims")
            budget = row.get("cgroup")
            if budget is None:
                raise ValueError("no enforced CPU/RSS ceiling")
            if budget["memory_events"].get("oom", 0):
                raise ValueError("memory budget exhausted")
            limits.add(json.dumps(budget["limits"], sort_keys=True))
        except (ValueError, KeyError, TypeError) as error:
            failures.append(f"{row['directory']}: {error}")
    if len(limits) != 1:
        failures.append("a single equal enforced resource policy is required")
    results = []
    workloads = sorted(set(options["workloads"]) - {"control"})
    if not workloads or 0 not in options["rates"] or not any(r > 0 for r in options["rates"]):
        failures.append("need primary workloads, saturation and common fixed offered rates")
    for work in workloads:
        for rate in options["rates"]:
            for peer in ["g1", "zgc"]:
                a = groups[work, rate, peer]
                b = groups[work, rate, candidate]
                expected = set(range(options["repeats"]))
                if set(a) != expected or set(b) != expected or len(expected) < 5:
                    failures.append(f"{work}/{rate}/{peer}: need at least five complete independent process pairs")
                    continue
                metrics = [("throughput", 1.1, "low")] if rate == 0 else [("p999", .8, "high"), ("p9999", 1.05, "high")]
                for metric, threshold, bound in metrics:
                    def value(row):
                        if row["status"] != "ok":
                            raise ValueError("failed process")
                        data = row["measurement"]
                        if metric == "throughput":
                            return data["completed"] * 1e9 / data["elapsed_ns"]
                        return quantile(data["response"], 9990 if metric == "p999" else 9999)
                    try:
                        interval = paired_interval([(value(a[i]), value(b[i])) for i in sorted(expected)])
                        passed = interval[bound] >= threshold if bound == "low" else interval[bound] <= threshold
                    except (ValueError, KeyError, ZeroDivisionError) as error:
                        failures.append(f"{work}/{rate}/{peer}/{metric}: {error}")
                        continue
                    result = {"workload": work, "rate": rate, "peer": peer, "metric": metric,
                              "candidate_peer_ratio": interval, "threshold": threshold, "pass": passed}
                    results.append(result)
                    if not passed:
                        failures.append(f"{work}/{rate}/{peer}/{metric}: performance target not met")
    return {"short_comparison_gate": bool(results) and not failures, "qualification": False,
            "results": results, "failures": failures,
            "remaining_gates": ["validated warmup and calibrated sustainable capacity",
                                "three paired one-hour trials and separate 24-hour trial",
                                "bounded backlog and retired/RSS memory time series",
                                "stalled publisher/trace progress and no-global-blocking proof",
                                "Wasm host-turn and browser qualification"]}
