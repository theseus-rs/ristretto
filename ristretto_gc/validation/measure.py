#!/usr/bin/env python3
"""Run the existing collector endurance/recovery drivers, preserving failed trials.

Every process receives the requested physical memory and total CPU ceilings.
Results are evidence, not automatic no-STW or long-duration certification.
"""
import argparse
import json
import os
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "benchmarks/gc"))
from cgroups import Budget
from run import digest, execute

FLAGS = ["predictive", "root-slots", "independent-marking", "work-stealing", "context-roots",
         "local-allocation", "adaptive-workers", "independent-relocation", "cost-aware-relocation",
         "concurrent-publication", "overlapping-generations", "independent-phases", "diagnostics"]


def command_for(o, rate):
    diagnostics = o.workload == "recovery" if o.diagnostics is None else o.diagnostics
    common = [o.predictive, o.root_slots, o.mark_batch_size, o.independent_marking,
              o.metadata_shards, o.work_stealing, o.context_roots, o.barrier_records, o.local_allocation]
    tail = [o.adaptive_workers, o.independent_relocation, o.cost_aware_relocation, o.relocation_reserve_bytes]
    if o.workload == "endurance":
        args = [o.seconds, o.workers, rate, o.heap_mib, o.compact, o.enforce, diagnostics,
                *common, *tail, o.concurrent_publication, o.overlapping_generations,
                o.region_shards, o.independent_phases, o.telemetry_ms, o.endurance_mode, o.old_collection_interval]
        if o.endurance_array_write != "generic":
            args.append(o.endurance_array_write)
    else:
        args = [o.seconds, o.workers, o.heap_mib, o.compact, o.recovery_mode, 8,
                *common, o.payload_bytes, *tail, diagnostics, o.concurrent_publication,
                o.overlapping_generations, o.region_shards, o.independent_phases, o.old_collection_interval]
    return [str(o.binary), *[str(a).lower() if isinstance(a, bool) else str(a) for a in args]]


def validate(data, workload, array_write="generic"):
    if data["operations"] <= 0 or data["seconds"] <= 0:
        raise ValueError("no completed work")
    if data["final_live_objects"] != 0 or data["final_occupied_bytes"] != 0:
        raise ValueError("heap teardown leak")
    for key, value in data["roots"].items():
        if key.startswith("final_") and (any(value) if isinstance(value, list) else value != 0):
            raise ValueError(f"unfinished teardown: {key}")
    if workload == "endurance":
        # Earlier drivers always used unrestricted mutable access.
        if data.get("array_write", "generic") != array_write:
            raise ValueError("endurance driver used a different array write")
        if data["backlog_operations"] or data["operations"] != data["offered_operations"]:
            raise ValueError("offered work was not completed")
        histograms = [data["operation_histogram_us"], data["arrival_histogram_us"]]
    else:
        histograms = [data["operation"]["histogram_us"]]
    for histogram in histograms:
        if not isinstance(histogram, dict) or not histogram:
            raise ValueError("expected a nonempty upper-bound/count histogram map")
        if any(not upper.isdecimal() or not isinstance(count, int) or count <= 0
               for upper, count in histogram.items()):
            raise ValueError("invalid histogram bucket")
        if sum(histogram.values()) != data["operations"]:
            raise ValueError("histogram lost operation samples")


def record_measurement(row, directory, workload, array_write):
    # The driver's latency assertion runs after it prints the final histogram.
    # Keep that evidence alongside the original nonzero process status.
    if row["status"] not in ("ok", "failed"):
        return
    try:
        data = json.loads((directory / "stdout.txt").read_text())
        row["measurement"] = data
        validate(data, workload, array_write)
        if row["cgroup"]["memory_events"].get("oom", 0):
            raise ValueError("memory budget exhausted")
    except (OSError, ValueError, KeyError, TypeError) as error:
        if row["status"] == "ok":
            row.update(status="invalid", error=str(error))
        else:
            row["measurement_error"] = str(error)


def parser():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--binary", type=Path, required=True)
    p.add_argument("--build-provenance", type=Path, required=True)
    p.add_argument("--output", type=Path, required=True)
    p.add_argument("--workload", choices=["endurance", "recovery"], default="endurance")
    p.add_argument("--endurance-array-write", choices=["generic", "element"], default="generic",
                   help="use unrestricted mutable access or the JVM reference-array element setter")
    p.add_argument("--endurance-mode", choices=["sync", "async"], default="async")
    p.add_argument("--recovery-mode", choices=["sync", "async"], default="async")
    p.add_argument("--compact", choices=["true", "false"], default="true")
    p.add_argument("--seconds", type=int, default=60)
    p.add_argument("--workers", type=int, default=2)
    p.add_argument("--rates", nargs="+", type=int, default=[10000])
    p.add_argument("--repeats", type=int, default=1)
    p.add_argument("--heap-mib", type=int, default=64)
    p.add_argument("--process-memory-mib", type=int, required=True)
    p.add_argument("--cgroup-parent", type=Path, default=os.environ.get("GC_CGROUP_PARENT"))
    p.add_argument("--cpus", type=int, nargs="+", default=sorted(os.sched_getaffinity(0))[:2])
    p.add_argument("--cpu-quota", type=float, default=2)
    p.add_argument("--enforce", action="store_true", help="also enforce the driver's operation latency gates")
    for flag in FLAGS:
        p.add_argument(f"--{flag}", action="store_true", default=None if flag == "diagnostics" else False)
    p.add_argument("--no-diagnostics", dest="diagnostics", action="store_false")
    for name, default in [("mark-batch-size", 1), ("metadata-shards", 1), ("region-shards", 1),
                          ("barrier-records", 0), ("old-collection-interval", 0),
                          ("relocation-reserve-bytes", 0), ("telemetry-ms", 1000), ("payload-bytes", 4096)]:
        p.add_argument(f"--{name}", type=int, default=default)
    return p


def main():
    p = parser()
    o = p.parse_args()
    if o.workload != "endurance" and o.endurance_array_write != "generic":
        p.error("--endurance-array-write applies to the endurance workload")
    if o.cgroup_parent is None:
        p.error("set --cgroup-parent or GC_CGROUP_PARENT to an existing delegated cpu+memory parent")
    if (o.seconds < 1 or o.seconds > 86400 or o.workers < 0 or o.repeats < 1
            or any(rate < 1 for rate in o.rates) or not 0 < o.cpu_quota <= len(o.cpus)
            or o.process_memory_mib < o.heap_mib or not set(o.cpus) <= os.sched_getaffinity(0)):
        p.error("invalid duration, repetitions, rate or resource budget")
    o.binary = o.binary.resolve()
    provenance = json.loads(o.build_provenance.read_text())
    if digest(o.binary) != provenance["binary_sha256"]:
        p.error("binary differs from build provenance")
    o.output = o.output.resolve()
    evidence = o.output.with_suffix(".runs")
    if o.output.exists():
        p.error("output already exists")
    evidence.mkdir(parents=True, exist_ok=False)
    report = {"qualification": False, "provenance": provenance, "runs": []}
    failed = False
    for repeat in range(o.repeats):
        for rate in o.rates if o.workload == "endurance" else [0]:
            directory = evidence / f"run-{repeat}-{rate}"
            directory.mkdir()
            command = command_for(o, rate)
            row = {"repeat": repeat, "rate": rate, "command": command, "directory": str(directory)}
            budget = Budget(o.cgroup_parent, o.process_memory_mib * 1024**2, round(o.cpu_quota * 100000))
            try:
                row.update(execute(command, directory, o.seconds + 180, o.cpus,
                                   {**os.environ, "LC_ALL": "C"}, budget))
                row["cgroup"] = budget.snapshot()
            finally:
                budget.close()
            record_measurement(row, directory, o.workload, o.endurance_array_write)
            failed |= row["status"] != "ok"
            report["runs"].append(row)
            report["binary_unchanged"] = digest(o.binary) == provenance["binary_sha256"]
            o.output.write_text(json.dumps(report, indent=2) + "\n")
    return int(failed or not report["binary_unchanged"])


if __name__ == "__main__":
    raise SystemExit(main())
