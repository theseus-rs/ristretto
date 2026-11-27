"""Render completed adaptive rate points from one or more fixed-arrival campaigns."""
import argparse
import csv
import hashlib
import json
from pathlib import Path
import statistics

from plot_scheduled_ring import COLORS, LABELS, MARKERS, plt, require, validate_capture
from run_scheduled_capacity import point_window
from canonical_service import workload_digest


def digest(path):
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def read(path):
    return json.loads(path.read_text())


def replay(directory):
    plan = read(directory / "plan.json")
    require(plan["schema_version"] == 2 and plan["scope"] == "shared_heap_scheduled_v1"
            and plan["policy"] == "delivered-capacity" and plan["qualification"] is False,
            "expected an exploratory delivered-capacity plan")
    require(workload_digest(directory / "classes") == plan["workload_sha256"], "workload bytecode changed")
    require(all(digest(directory / "admissions" / n) == v for n, v in plan["admission_sha256"].items()),
            "admission changed")
    receipt = (directory / "execution/results.json").read_bytes()
    execution = json.loads(receipt)
    plan_hash = digest(directory / "plan.json")
    require(execution["plan_sha256"] == plan_hash, "execution belongs to another plan")
    resources = read(directory / "resources.json")
    require(digest(directory / "resources.json") == plan["resources_sha256"], "resource declaration changed")
    processes = {p["id"]: p for p in execution["processes"]}
    require(len(processes) == len(execution["processes"]), "duplicate process in execution")
    rows = []
    for search in execution["searches"]:
        workload, runtime = search["workload"], search["runtime"]
        require(workload in plan["workloads"] and runtime in plan["runtimes"], "undeclared search")
        for index, point in enumerate(search["points"]):
            rate = point["rate"]
            point_id = f"{workload}-{runtime}-point{index:02d}-rate{rate}"
            window = read(directory.parent / "windows" / (point_id + ".json"))
            require(window["plan_sha256"] == plan_hash and window["point"] == point_id, "window belongs to another point")
            seconds, watchdog = point_window(plan, rate)
            adjusted = dict(plan, seconds=seconds, watchdog_seconds=watchdog)
            require(len(point["forks"]) == plan["forks"], "completed point has missing forks")
            forks = []
            for fork in range(plan["forks"]):
                case = point_id + f"-fork{fork}"
                process = processes[case]
                require((process["rate"], process["runtime"], process["fork"], process["workload"]) ==
                        (rate, runtime, fork, workload), "capture identity differs from declared point")
                require(process["quiet_window_id"] == window["quiet_window_id"], "point window mismatch")
                row, _, _, packet = validate_capture(directory, adjusted, resources, process)
                recorded = point["forks"][fork]
                require(recorded["fork"] == fork and recorded["schedule"] == packet["result"]["schedule"]
                        and recorded["gc_activity_observed"] == row["gc_activity_observed"], "recorded fork differs from raw")
                receipt = "peer-build.json" if runtime in ("g1", "zgc") else "build.json"
                family = "peer" if runtime in ("g1", "zgc") else "wasi" if runtime.startswith("wasi") else "native"
                capture_dir = directory / "execution/runs" / case
                require(digest(capture_dir / receipt) == plan["builds"][family]["receipt_sha256"],
                        "capture build receipt differs from planned build")
                require(row["offered"] == seconds * rate, "arrival count mismatch")
                require(not row["memory_events"]["oom"] and not row["memory_events"]["oom_kill"], "out of memory")
                forks.append(dict(row, workload=workload, seconds=seconds, point=point_id,
                                  confirmation=point["confirmation"], plan_sha256=plan_hash,
                                  stdout_sha256=digest(directory / "execution/runs" / case / "stdout.txt")))
            enough = all(f["sample_floor_satisfied"] for f in forks)
            delivered = all(f["offered"] == f["deadline_completed"] and
                            not (f["deadline_backlog"] or f["late_inflight"] or f["unstarted"]) for f in forks)
            status = ("pass" if delivered else "fail") if enough else "invalid"
            require(len(point["forks"]) == len(forks) and status == point["status"], "delivery classification mismatch")
            if enough:
                latency = delivered and all(f["historical_response_limit_satisfied"] and
                                            f["historical_backlog_limit_satisfied"] for f in forks)
                require(point["latency_screen_status"] == ("pass" if latency else "fail"), "latency screen mismatch")
            require(point["gc_activity_in_every_fork"] == all(f["gc_activity_observed"] for f in forks),
                    "collection exposure mismatch")
            for row in forks:
                row["delivery_status"] = status
            rows.extend(forks)
    return plan, rows, {"path": str(directory), "plan_sha256": plan_hash,
                        "execution_sha256": hashlib.sha256(receipt).hexdigest(),
                        "execution_status": execution["status"], "completed_forks": len(rows)}


def report(directories, output):
    captures = [replay(directory) for directory in directories]
    fields = ("workload_sha256", "resources_sha256", "admission_sha256", "offer_policy",
              "request_drain_policy", "screen_limits", "warmup")
    first = captures[0][0]
    require(all(all(plan[k] == first[k] for k in fields) for plan, _, _ in captures),
            "campaigns differ in fixtures, admissions, resources, screening or scheduling")
    rows = [r for _, group, _ in captures for r in group]
    require(rows and len({(r["plan_sha256"], r["id"]) for r in rows}) == len(rows), "missing or duplicate captures")
    groups = {}
    for r in rows:
        groups.setdefault((r["workload"], r["runtime"], r["plan_sha256"], r["point"]), []).append(r)
    ordered = sorted(groups.values(), key=lambda g: (g[0]["workload"], LABELS[g[0]["runtime"]], g[0]["rate"], g[0]["point"]))
    output.mkdir(parents=True, exist_ok=False)
    columns = ("workload", "runtime", "rate", "seconds", "point", "confirmation", "fork", "delivery_status", "offered",
               "deadline_completed", "by_deadline_ops_s", "deadline_backlog", "late_inflight", "unstarted",
               "peak_waiting_backlog", "service_p50_upper_ms", "response_p999_upper_ms", "gc_activity_observed",
               "sample_floor_satisfied", "lifetime_peak_rss_mib", "lifetime_cpu_seconds", "plan_sha256", "stdout_sha256")
    with (output / "forks.csv").open("w") as stream:
        writer = csv.DictWriter(stream, columns, extrasaction="ignore")
        writer.writeheader()
        writer.writerows(rows)
    figure, axes = plt.subplots(1, 3, figsize=(13, 2.3 + 0.65 * len(ordered)), layout="constrained",
                                sharey=True, gridspec_kw={"width_ratios": [1.4, 1, 1]})
    labels = []
    table = []
    for pos, forks in enumerate(ordered):
        row = forks[0]
        runtime = row["runtime"]
        label = LABELS[runtime] + (" (confirmation)" if row["confirmation"] else "")
        labels.append(f"{label} · {row['rate']:,}/s\n{row['workload']} · {row['seconds']} s")
        for i, fork in enumerate(forks):
            y = pos + (i - (len(forks) - 1) / 2) * min(.12, .6 / len(forks))
            vals = (fork["by_deadline_ops_s"], 100 * fork["deadline_backlog"] / fork["offered"], fork["response_p999_upper_ms"])
            for axis, val in zip(axes, vals):
                axis.scatter(val, y, c=COLORS[runtime], s=30, marker=MARKERS[runtime])
        axes[0].scatter(row["rate"], pos, marker="|", c="black", s=160)
        table.append(f"| {row['workload']} | {label} | {row['rate']:,} | {row['seconds']} | "
                     f"{len(forks)} | {statistics.median(f['by_deadline_ops_s'] for f in forks):,.1f} | "
                     f"{min(f['deadline_backlog'] for f in forks):,}–{max(f['deadline_backlog'] for f in forks):,} | "
                     f"{row['delivery_status']} |")
    for axis, title in zip(axes, ("By-deadline calls/s", "Outstanding at deadline (%)", "Response p99.9 upper bucket (ms)")):
        axis.set_title(title, fontsize=11)
        axis.grid(axis="x", alpha=.25)
        axis.set_axisbelow(True)
    axes[0].set_xscale("log")
    axes[0].set_yticks(range(len(labels)), labels)
    axes[0].invert_yaxis()
    axes[1].set_xlim(left=-1)
    axes[2].set_xscale("log")
    figure.suptitle("Completed fixed-arrival rate points · one marker per process")
    for ext in ("svg", "png"):
        figure.savefig(output / ("capacity-points." + ext), dpi=160)
    plt.close(figure)
    validators = ("run_scheduled_capacity.py", "plot_scheduled_ring.py", "ring_artifacts.py",
                  "scheduled_ring_comparison.py", "canonical_service.py", "report.py")
    summary = {"qualification": False, "source_sha256": digest(Path(__file__)),
               "validator_sha256": {n: digest(Path(__file__).with_name(n)) for n in validators}, "campaigns": [c[2] for c in captures],
               "forks": len(rows), "completed_points": len(ordered)}
    (output / "summary.json").write_text(json.dumps(summary, indent=2) + "\n")
    content = ["# Fixed-arrival ring measurements", "",
               "Each marker represents one independently validated process. The black tick shows its offered rate. "
               "Response times start at the scheduled release. Unstarted calls remain in the backlog and have no latency sample.",
               "", "![Offered rates, deadline delivery and scheduled response](capacity-points.png)", "",
               "| Workload | Runtime | Offered/s | Seconds/fork | Forks | Median delivered/s | Deadline backlog range | Delivery screen |",
               "|---|---|---:|---:|---:|---:|---:|---|", *table, "",
               "Rates and observation durations are shown for each point. The figure includes completed points while "
               "the adaptive searches continue. Sustained capacity, matched-rate latency superiority and extreme tails "
               "require additional measurements. [Individual processes](forks.csv) · [Source receipts](summary.json)."]
    (output / "REPORT.md").write_text("\n".join(content) + "\n")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("campaigns", type=Path, nargs="+", help="plan directories with point windows")
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    report([path.resolve() for path in args.campaigns], args.output.resolve())
