"""Plot validated finite invocation costs, preserving different GC lifecycles."""
import argparse
import json
from pathlib import Path
import statistics
from canonical_service import digest
from canonical_service_artifacts import read_json
from ring_artifacts import check_canonical, check_peer, packet
from ring_comparison import validate_matrix, validate_matrix_admission


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("directory", type=Path)
    p.add_argument("--output", type=Path, required=True)
    args = p.parse_args()
    root = args.directory.resolve()
    plan, results = validate_matrix(read_json(root / "plan.json")), read_json(root / "results.json")
    shared = plan["schema_version"] == 2
    if (plan["check_only"] or results["complete"] is not True or results["qualification"] is not False
            or results["plan_sha256"] != digest(root / "plan.json")):
        raise ValueError("only a complete measured matrix can be plotted")
    if [row["id"] for row in results["results"]] != [row["id"] for row in plan["cases"]]:
        raise ValueError("matrix is missing, duplicating or reordering forks")
    observations, exposure = {}, set()
    for case, receipt in zip(plan["cases"], results["results"]):
        run = root / "runs" / case["id"]
        peer = case["runtime"] in {"g1", "zgc"}
        if receipt["exit_code"] != 0 or digest(run / "checked.json") != receipt["checked_sha256"]:
            raise ValueError("fork validation changed or failed")
        validate_matrix_admission(plan, read_json(run / "admission.json"))
        checked = check_peer(run) if peer else check_canonical(run)
        usage = checked.get("process_resources")
        if usage is not None and usage["timeout_seconds"] != plan.get("watchdog_seconds"):
            raise ValueError("resource watchdog differs from frozen comparison matrix")
        value = packet(run / "stdout.txt", "RISTRETTO_RING_PEER " if peer else "RISTRETTO_CANONICAL_RING ")
        if case["mode"] == "allocation":
            if checked["payload_allocation_lower_bound_verified"] is not True:
                raise ValueError("fresh peer allocation exposure missing")
            exposure.add((case["workload"], case["fork"], case["runtime"]))
            continue
        if case["mode"] != "measure": raise ValueError("mixed untimed/measurement matrix")
        if peer and (case["workload"], case["fork"], case["runtime"]) not in exposure:
            raise ValueError("peer timing lacks its preceding independent allocation fork")
        values = [row["elapsed_ns"] / 1_000_000 for row in value["samples"]]
        observations.setdefault((case["workload"], case["runtime"]), []).append(statistics.median(values))
    runtimes = ["g1", "zgc", "native-compiled", "native-interpreted", "wasi-interpreted"]
    for workload in plan["workloads"]:
        for runtime in runtimes:
            if len(observations.get((workload, runtime), [])) != plan["forks"]:
                raise ValueError("missing equal runtime/fork coverage")
    import matplotlib
    matplotlib.use("Agg")
    import matplotlib.pyplot as plt
    figure, axes = plt.subplots(1, len(plan["workloads"]), figsize=(6 * len(plan["workloads"]), 5.4), squeeze=False)
    canonical_label = "shared heap" if shared else "setup + cleanup"
    labels = ["HotSpot G1\ndeferred GC", "HotSpot ZGC\ndeferred GC", f"Native compiled\n{canonical_label}",
              f"Native interpreted\n{canonical_label}", f"WASI interpreted\n{canonical_label}"]
    for axis, workload in zip(axes[0], plan["workloads"]):
        groups = [observations[(workload, runtime)] for runtime in runtimes]
        axis.bar(range(5), [statistics.median(group) for group in groups], color=["#9baec8", "#9baec8", "#46968d", "#46968d", "#46968d"], alpha=.65)
        for index, group in enumerate(groups):
            axis.scatter([index] * len(group), group, color="#172b3a", zorder=3, s=24)
        axis.set_xticks(range(5), labels, rotation=25, ha="right")
        axis.set_ylabel("Invocation milliseconds (dots: fork medians)")
        axis.set_title(f'{workload}: {plan["steps"]} replacements / invocation')
        axis.grid(axis="y", alpha=.2)
    figure.suptitle(("Finite shared-heap invocation costs" if shared else "Finite invocation costs with different collection lifecycles") + "\nNo scheduled-tail, sustainable-throughput or global-pause qualification")
    caption = ("Canonical shared_heap_invocation_v1 keeps heap/context across warmup + samples; setup and final cleanup reported separately.\nPrimary-cycle observations cover the full sample batch including driver gaps, not individual calls; HotSpot retains its heap."
               if shared else "Canonical full_invocation_v1 includes dedicated heap setup + cleanup; HotSpot uses its shared heap and defers GC.\nCanonical in-body cycle exposure is unavailable. Recorded entry_body_v1 is a separate diagnostic.")
    figure.text(.5, .015, caption, ha="center", fontsize=9)
    figure.tight_layout(rect=(0, .09, 1, .9))
    args.output.parent.mkdir(parents=True, exist_ok=True)
    figure.savefig(args.output.with_suffix(".svg"))
    figure.savefig(args.output.with_suffix(".png"), dpi=160)
    summary = {"qualification": False, "plan_sha256": digest(root / "plan.json"),
               "fork_median_ms": [{"workload": workload, "runtime": runtime, "values": values}
                                  for (workload, runtime), values in observations.items()],
               "scopes": plan["scopes"], "canonical_gc_cycles_completed": None, "canonical_primary_cycle_scope": "sample_batch_including_driver_gaps_and_observer" if shared else None,
               "scheduled_latency_available": False, "sustainable_throughput_available": False}
    args.output.with_suffix(".json").write_text(json.dumps(summary, indent=2) + "\n")


if __name__ == "__main__":
    main()
