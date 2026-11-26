#!/usr/bin/env python3
"""Explore fixed-arrival capacity; preserve every trial and never certify endurance."""
import argparse
import json
import math
from pathlib import Path
import shutil
import subprocess
import sys

from report import quantile, validate
from run import HERE, ROOT, RUNTIMES, digest, freeze_sources


def classify(directory, latency_us, backlog_us, minimum):
    """All independent forks must satisfy the declared limits; never pool tails."""
    metadata = json.loads((directory / "metadata.json").read_text())
    options = metadata["options"]
    rows = [json.loads(line) for line in (directory / "raw.jsonl").read_text().splitlines()]
    expected = set(range(options["repeats"]))
    if (len(rows) != len(expected) or {r["repeat"] for r in rows} != expected
            or not json.loads((directory / "integrity.json").read_text())["inputs_unchanged"]):
        return {"status": "invalid", "reason": "missing/duplicate trials or changed inputs", "forks": []}
    forks = []
    limits = set()
    for row in rows:
        try:
            if row["status"] != "ok":
                raise ValueError(f"process {row['status']}; not a valid overload observation")
            if ([row["runtime"]] != options["runtimes"] or [row["workload"]] != options["workloads"]
                    or [row["rate"]] != options["rates"] or row["rate"] <= 0):
                raise ValueError("trial identity mismatch")
            data = row["measurement"]
            validate(data, row["workload"], row["rate"], options["seconds"], options["seed"])
            if row.get("cgroup") is None or row["cgroup"]["memory_events"].get("oom", 0):
                raise ValueError("missing resource enforcement or memory failure")
            limits.add(json.dumps(row["cgroup"]["limits"], sort_keys=True))
            if data["completed"] < minimum:
                raise ValueError("insufficient completions for the declared screening sample floor")
            tail = quantile(data["response"], 9990)
            backlog = data["peak_backlog"] * 1e6 / row["rate"]
            if not math.isfinite(backlog) or backlog < 0:
                raise ValueError("invalid backlog")
            reasons = []
            if data["unfinished"]:
                reasons.append("unfinished requests")
            if tail > latency_us:
                reasons.append("response p99.9 limit")
            if backlog > backlog_us:
                reasons.append("peak backlog time limit")
            forks.append({"repeat": row["repeat"], "completed": data["completed"],
                          "unfinished": data["unfinished"], "p999_us": tail,
                          "peak_backlog_us": backlog, "reasons": reasons,
                          "gc_observed": row.get("gc_observation", {}).get("activity_observed"),
                          "million_samples": data["completed"] >= 1_000_000})
        except (KeyError, ValueError, TypeError) as error:
            return {"status": "invalid", "reason": str(error), "forks": forks}
    if len(limits) != 1:
        return {"status": "invalid", "reason": "resource limits differ between forks", "forks": forks}
    return {"status": "fail" if any(f["reasons"] for f in forks) else "pass", "forks": forks}


def bounds(points):
    if any(p["status"] == "invalid" for p in points):
        return {"status": "invalid", "lower": None, "upper": None}
    passing = [p["rate"] for p in points if p["status"] == "pass"]
    failing = [p["rate"] for p in points if p["status"] == "fail"]
    lower, upper = max(passing, default=None), min(failing, default=None)
    # A lower-rate failure cannot be hidden by selecting a later higher success.
    if lower is not None and upper is not None and lower >= upper:
        return {"status": "inconsistent", "lower": lower, "upper": upper}
    return {"status": "bounded" if lower is not None and upper is not None else "unbracketed",
            "lower": lower, "upper": upper}


def next_rate(points, initial, maximum, tolerance):
    b = bounds(points)
    if b["status"] in ["invalid", "inconsistent"]:
        return None
    lo, hi = b["lower"], b["upper"]
    if lo is None:
        return initial if hi is None else (max(1, hi // 2) if hi > 1 else None)
    if hi is None:
        return min(maximum, lo * 2) if lo < maximum else None
    if hi - lo <= max(1, lo * tolerance):
        return None
    return (lo + hi) // 2


def save(output, report, plot):
    (output / "summary.json").write_text(json.dumps(report, indent=2) + "\n")
    lines = ["# Fixed-arrival capacity screen", "",
             "Every point contains fresh process forks. All forks must meet response p99.9, peak backlog, completion, and sample-floor limits.",
             "Intervals below are observed passing/failing rate brackets, not confidence intervals or sustained-load guarantees. All points, including overload and failed confirmation, remain in the evidence.", "",
             "| Workload | Runtime | Status | Passing lower ops/s | Failing upper ops/s | Confirmed endpoints |",
             "|---|---|---|---:|---:|---|"]
    for search in report["searches"]:
        b = bounds(search["points"])
        lines.append(f"| {search['workload']} | {search['runtime']} | {b['status']} | {b['lower']} | {b['upper']} | {search['confirmed']} |")
    lines += ["", f"Inputs unchanged: {report.get('inputs_unchanged', 'pending')}. Qualification: **false**. Use fresh, longer fixed-rate comparisons below the lowest peer's validated capacity; do not reuse adaptive search points as independent performance confirmation."]
    (output / "REPORT.md").write_text("\n".join(lines) + "\n")
    if not plot:
        return
    import matplotlib.pyplot as plt
    for work in report["options"]["workloads"]:
        searches = [s for s in report["searches"] if s["workload"] == work]
        fig, axis = plt.subplots(figsize=(9, 4), layout="constrained")
        for i, search in enumerate(searches):
            for status, marker, color in [("pass", "o", "#19724b"), ("fail", "x", "#c44430"), ("invalid", "s", "#8c649d")]:
                rates = [p["rate"] for p in search["points"] if p["status"] == status]
                axis.scatter(rates, [i] * len(rates), marker=marker, color=color,
                             label=status if i == 0 else None)
        axis.set_yticks(range(len(searches)), [s["runtime"] for s in searches])
        axis.set_xscale("log")
        axis.set_xlabel("Offered operations/s (each point has fresh forks)")
        axis.set_title(f"{work}: fixed-arrival screen, not endurance qualification")
        axis.legend()
        for extension in ["svg", "png"]:
            fig.savefig(output / f"capacity-{work}.{extension}", dpi=160)
        plt.close(fig)


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--output", type=Path, required=True)
    p.add_argument("--jdk", type=Path, required=True)
    p.add_argument("--ristretto", type=Path, default=ROOT / "target/release/examples/gc_compare")
    p.add_argument("--wasm", type=Path, default=ROOT / "target/wasm32-wasip2/release/examples/gc_compare.wasm")
    p.add_argument("--wasmtime", type=Path)
    p.add_argument("--runtimes", nargs="+", choices=RUNTIMES, default=["ristretto-candidate", "g1", "zgc"])
    p.add_argument("--workloads", nargs="+", choices=["allocation", "roots", "mixed"], default=["allocation", "roots"])
    p.add_argument("--initial-rate", type=int, default=10000)
    p.add_argument("--max-rate", type=int, default=100000000)
    p.add_argument("--max-points", type=int, default=16, help="search points per runtime/workload, plus two endpoint confirmations")
    p.add_argument("--tolerance", type=float, default=.05)
    p.add_argument("--p999-us", type=float, default=1000)
    p.add_argument("--backlog-us", type=float, default=10000)
    p.add_argument("--minimum-completions", type=int, default=10000)
    p.add_argument("--seconds", type=int, default=10)
    p.add_argument("--warmup", type=int, default=5)
    p.add_argument("--repeats", type=int, default=5)
    p.add_argument("--heap-mib", type=int, default=64)
    p.add_argument("--workers", type=int, default=2)
    p.add_argument("--cpus", nargs="+", type=int, required=True)
    p.add_argument("--cgroup-parent", type=Path, required=True)
    p.add_argument("--rss-mib", type=int, required=True)
    p.add_argument("--cpu-quota", type=float, required=True)
    p.add_argument("--seed", type=int, default=42)
    p.add_argument("--plot", action="store_true")
    o = p.parse_args()
    if (not 0 < o.initial_rate <= o.max_rate <= 10**9 or o.max_points < 1 or o.repeats < 2
            or not 0 < o.tolerance < 1 or not 0 < o.p999_us < float("inf")
            or not 0 < o.backlog_us < float("inf") or o.minimum_completions < 1
            or len(set(o.runtimes)) != len(o.runtimes) or len(set(o.workloads)) != len(o.workloads)):
        p.error("invalid search limits, screening limits or duplicate conditions")
    o.output.mkdir(parents=True, exist_ok=False)
    sources = freeze_sources(o.output.resolve())
    report = {"qualification": False, "options": {k: str(v) if isinstance(v, Path) else v for k, v in vars(o).items()},
              "source_sha256": sources, "searches": []}
    artifacts = [o.jdk / "bin/java", o.jdk / "lib/modules"]
    # Every adaptive point must execute the same binaries, even if callers build
    # a replacement in the original target directory while this screen runs.
    for key, prefix, name in [("ristretto", "ristretto-", "gc_compare"), ("wasm", "wasm-", "gc_compare.wasm")]:
        if any(r.startswith(prefix) for r in o.runtimes):
            destination = o.output.resolve() / name
            shutil.copy2(getattr(o, key), destination)
            setattr(o, key, destination)
            artifacts.append(destination)
    if any(r.startswith("wasm-") for r in o.runtimes):
        if o.wasmtime is None:
            p.error("--wasmtime required for Wasm runtimes")
        artifacts.append(o.wasmtime)
    hashes = {str(path.resolve()): digest(path) for path in artifacts}
    report["artifacts_sha256"] = hashes
    common = [sys.executable, str(HERE / "run.py")]
    for key in ["jdk", "ristretto", "wasm", "heap_mib", "workers", "seconds", "warmup", "repeats", "seed", "cgroup_parent", "rss_mib", "cpu_quota"]:
        common += ["--" + key.replace("_", "-"), str(getattr(o, key))]
    common += ["--cpus", *map(str, o.cpus), "--timeout", str(o.seconds + o.warmup + 180)]
    if o.wasmtime:
        common += ["--wasmtime", str(o.wasmtime)]
    for work in o.workloads:
        for runtime in o.runtimes:
            search = {"workload": work, "runtime": runtime, "points": [], "confirmed": False}
            report["searches"].append(search)
            def trial(rate, confirmation=False):
                directory = o.output / f"{work}-{runtime}-{len(search['points']):02d}-{rate}"
                command = common + ["--output", str(directory), "--workloads", work, "--runtimes", runtime, "--rates", str(rate)]
                with (o.output / "commands.jsonl").open("a") as log:
                    log.write(json.dumps(command) + "\n")
                child = subprocess.run(command)
                try:
                    point = classify(directory, o.p999_us, o.backlog_us, o.minimum_completions)
                    if child.returncode:
                        point.update(status="invalid", reason=f"runner exit {child.returncode}")
                except (OSError, ValueError, KeyError, TypeError) as error:
                    point = {"status": "invalid", "reason": str(error), "forks": []}
                point.update(rate=rate, directory=directory.name, confirmation=confirmation)
                search["points"].append(point)
                save(o.output, report, False)
            for _ in range(o.max_points):
                rate = next_rate(search["points"], o.initial_rate, o.max_rate, o.tolerance)
                if rate is None:
                    break
                trial(rate)
            b = bounds(search["points"])
            if b["status"] == "bounded":
                trial(b["lower"], True)
                trial(b["upper"], True)
                search["confirmed"] = (search["points"][-2]["status"] == "pass"
                                       and search["points"][-1]["status"] == "fail")
    report["inputs_unchanged"] = (all((ROOT / name).is_file() and digest(ROOT / name) == value for name, value in sources.items())
                                  and all(Path(name).is_file() and digest(name) == value for name, value in hashes.items()))
    save(o.output, report, o.plot)
    return int(not report["inputs_unchanged"] or any(bounds(s["points"])["status"] in ["invalid", "inconsistent"] for s in report["searches"]))


if __name__ == "__main__":
    raise SystemExit(main())
