"""Revalidate a completed fixed-arrival pilot and render descriptive reports.

Usage: python plot_scheduled_ring.py COMPLETED_PLAN --output NEW_DIRECTORY
The capture tree is read-only. This command never executes a workload.
"""

import argparse
import csv
import json
from pathlib import Path
import statistics
import sys
import time

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
import numpy as np

from canonical_service import digest
from report import quantile
from ring_artifacts import check_canonical, check_peer, packet

LABELS = {
    "g1": "HotSpot G1",
    "zgc": "HotSpot ZGC",
    "native-compiled": "Native compiled",
    "native-interpreted": "Native interpreted",
    "wasi-interpreted": "WASI interpreted",
}
COLORS = {
    "g1": "#3976a8",
    "zgc": "#8b66a8",
    "native-compiled": "#bd6432",
    "native-interpreted": "#20836b",
    "wasi-interpreted": "#c9445a",
}
MARKERS = {
    "g1": "o",
    "zgc": "s",
    "native-compiled": "^",
    "native-interpreted": "D",
    "wasi-interpreted": "v",
}
GROUP_METRICS = (
    "by_deadline_ops_s",
    "deadline_completed",
    "deadline_backlog",
    "late_inflight",
    "unstarted",
    "peak_waiting_backlog",
    "service_p50_upper_ms",
    "service_p99_upper_ms",
    "response_p50_upper_ms",
    "response_p99_upper_ms",
    "response_p999_upper_ms",
    "response_max_ms",
    "lifetime_peak_rss_mib",
    "observed_waiting_growth_per_s",
)


def require(condition, message):
    if not condition:
        raise ValueError(message)


def read_json(path):
    try:
        return json.loads(path.read_text())
    except (OSError, json.JSONDecodeError) as error:
        raise ValueError(f"Cannot read JSON receipt {path}: {error}") from error


def write_json(path, value):
    path.write_text(json.dumps(value, indent=2) + "\n")


def parse_args(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("completed_plan", type=Path)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args(argv)
    plan_directory = args.completed_plan.resolve()
    if plan_directory.is_file():
        require(plan_directory.name == "plan.json", "The completed plan file must be named plan.json")
        plan_directory = plan_directory.parent
    output = args.output.resolve()
    require(
        not output.is_relative_to(plan_directory) and not plan_directory.is_relative_to(output),
        "Report output must be separate from the immutable completed plan directory",
    )
    require(not output.exists(), f"Report output already exists; choose a new directory: {output}")
    return plan_directory, output


def validate_inventory(plan_directory):
    """Check the completed fixed-pilot inventory before examining individual cases."""
    plan = read_json(plan_directory / "plan.json")
    execution = read_json(plan_directory / "execution/results.json")
    terminal_path = plan_directory.parent / "pilot-execution.json"
    terminal = read_json(terminal_path) if terminal_path.exists() else None
    require(
        plan["policy"] == "fixed-pilot" and len(plan["workloads"]) == 1,
        "This reporter requires a completed single-workload fixed-rate pilot",
    )
    if terminal is not None:
        require(terminal["exit_code"] == 0, "The optional outer pilot terminal receipt reports a failed process")
    require(execution["status"] == "completed", "The scheduled execution receipt does not report completion")
    require(execution["plan_unchanged"] is True, "The execution receipt reports that the plan changed")
    expected_processes = len(plan["runtimes"]) * len(plan["rates"]) * plan["forks"]
    require(
        len(execution["processes"]) == expected_processes,
        f"Expected {expected_processes} completed processes, found {len(execution['processes'])}",
    )
    require(
        digest(plan_directory / "plan.json") == execution["plan_sha256"],
        "The completed plan SHA-256 differs from the execution receipt",
    )
    return plan, execution, terminal_path, terminal


def validate_capture(plan_directory, plan, resources, process):
    """Revalidate bound raw evidence and return one fork's derived observations."""
    case_id = process["id"]
    require(process["valid"] is True, f"{case_id}: execution marked this capture invalid")
    require(process["exit_code"] == 0, f"{case_id}: execution reports a nonzero exit code")
    directory = plan_directory / "execution/runs" / case_id
    peer = process["runtime"] in ("g1", "zgc")
    result = check_peer(directory) if peer else check_canonical(directory)
    mode = "peer" if peer else "wasi" if process["runtime"].startswith("wasi") else "native"
    run = read_json(directory / "run.json")
    sample_plan = read_json(directory / "sample-plan.json")
    launch = read_json(directory / "launch-plan.json")
    tier = "compiled" if process["runtime"] == "native-compiled" else "interpreted"
    admission_name = f"{process['workload']}-{tier}.json"
    require(
        run["admission_sha256"] == plan["admission_sha256"][admission_name],
        f"{case_id}: admission hash differs from the declared workload/tier",
    )
    require(run["workload_sha256"] == plan["workload_sha256"], f"{case_id}: workload bytecode hash differs")
    for field, expected in (
        ("arrival_rate", process["rate"]),
        ("duration_seconds", plan["seconds"]),
        ("warmup_invocations", plan["warmup"]),
    ):
        require(sample_plan[field] == expected, f"{case_id}: {field} differs from the declared plan ({expected})")
    require(launch["watchdog_seconds"] == plan["watchdog_seconds"], f"{case_id}: requested watchdog differs")
    require(launch["resources"] == resources, f"{case_id}: launch resource envelope differs")
    build = read_json(directory / ("peer-build.json" if peer else "build.json"))
    require(
        build["source_manifest_sha256"] == plan["builds"][mode]["source_manifest_sha256"],
        f"{case_id}: build source manifest differs from the declared {mode} artifact",
    )
    prefix = "RISTRETTO_RING_SCHEDULED_PEER " if peer else "RISTRETTO_RING_SCHEDULED "
    data = packet(directory / "stdout.txt", prefix)
    schedule = data["schedule"] if peer else data["data"]["schedule"]
    floor = plan["screen_limits"]["minimum_completions"]
    row = {
        "id": case_id,
        "runtime": process["runtime"],
        "rate": process["rate"],
        "fork": process["fork"],
        "offered": schedule["offered"],
        "deadline_completed": schedule["deadline_completed"],
        "deadline_backlog": schedule["deadline_backlog"],
        "completed": schedule["completed"],
        "late_inflight": schedule["after_deadline_completions"],
        "unstarted": schedule["unfinished"],
        "peak_waiting_backlog": schedule["peak_backlog"],
        "by_deadline_ops_s": schedule["deadline_completed"] / plan["seconds"],
        "all_completed_ops_s": result["schedule"]["completed_ops_s"],
        "gc_activity_observed": result["gc_activity_observed"],
        "sample_floor_satisfied": schedule["completed"] >= floor,
        "historical_response_limit_satisfied": quantile(schedule["response"], 9990) <= plan["screen_limits"]["p999_us"],
        "historical_backlog_limit_satisfied": schedule["peak_backlog"] * 1e6 / process["rate"] <= plan["screen_limits"]["backlog_us"],
        "screen_status": "invalid_below_sample_floor" if schedule["completed"] < floor else "see_individual_limits",
    }
    histograms = []
    for kind in ("service", "response"):
        for name, rank in (("p50", 5000), ("p99", 9900), ("p999", 9990)):
            row[f"{kind}_{name}_upper_ms"] = quantile(schedule[kind], rank) / 1000
        row[f"{kind}_max_ms"] = schedule[kind]["max_ns"] / 1e6
        histograms.append({"id": case_id, "kind": kind, "histogram": schedule[kind]})
    usage = result["process_resources"]
    row["lifetime_peak_rss_mib"] = usage["peak_rss_bytes"] / 2**20
    row["process_resources"] = usage
    row["lifetime_cpu_seconds"] = usage["user_cpu_seconds"] + usage["system_cpu_seconds"]
    timeline = [
        {
            "id": case_id, "runtime": row["runtime"], "rate": row["rate"], "fork": row["fork"],
            "elapsed_seconds": elapsed / 1e9, "completed": completed, "waiting_backlog": backlog,
        }
        for elapsed, completed, backlog in schedule["timeline"]
    ]
    if schedule["timeline"]:
        first, last = schedule["timeline"][0], schedule["timeline"][-1]
        row["observed_waiting_growth_per_s"] = (last[2] - first[2]) / ((last[0] - first[0]) / 1e9) if last[0] > first[0] else None
    after = read_json(directory / "resource-after.json")
    row["memory_events"] = {key: int(value) for key, value in (line.split() for line in after["memory.events"].splitlines())}
    cpu = dict(line.split() for line in after["cpu.stat"].splitlines())
    row["cpu_throttled_periods"] = int(cpu.get("nr_throttled", "0"))
    if peer:
        row["gc_collection_counts"] = data["gc_collection_counts"]
    else:
        row["primary_cycles"] = result["primary_cycle_batch_bounds"]
        for field in ("workload_counts", "heap", "preparation", "heap_detach_cleanup_and_drop_ns", "program_teardown_ns"):
            row[field] = data[field]
    return row, timeline, histograms, {"id": case_id, "result": result, "packet": data}


def group_forks(plan, rows):
    groups = []
    for runtime in plan["runtimes"]:
        for rate in plan["rates"]:
            selected = [row for row in rows if row["runtime"] == runtime and row["rate"] == rate]
            metrics = {}
            for metric in GROUP_METRICS:
                values = [row[metric] for row in selected]
                require(all(value is not None for value in values), f"{runtime} at {rate}/s: {metric} is unavailable in a fork")
                metrics[metric] = {
                    "forks": values, "median": statistics.median(values), "min": min(values), "max": max(values),
                }
            groups.append({
                "runtime": runtime, "rate": rate, "fork_ids": [row["id"] for row in selected],
                "metrics": metrics,
                "sample_adequate_forks": sum(row["sample_floor_satisfied"] for row in selected),
                "gc_activity_forks": sum(row["gc_activity_observed"] for row in selected),
            })
    return groups


def revalidate(plan_directory):
    plan, execution, terminal_path, terminal = validate_inventory(plan_directory)
    resources = read_json(plan_directory / "resources.json")
    seen = set()
    rows, timeline, histograms, packets = [], [], [], []
    for process in execution["processes"]:
        key = (process["runtime"], process["rate"], process["fork"])
        require(key not in seen, f"Duplicate runtime/rate/fork in the execution inventory: {key}")
        seen.add(key)
        try:
            row, progress, histograms_for_case, validated = validate_capture(plan_directory, plan, resources, process)
        except (KeyError, ValueError, OSError) as error:
            raise ValueError(f"Capture {process['id']} failed independent validation: {error}") from error
        rows.append(row)
        timeline.extend(progress)
        histograms.extend(histograms_for_case)
        packets.append(validated)
        print("VALIDATED", len(rows), process["id"], flush=True)
    expected = {(runtime, rate, fork) for runtime in plan["runtimes"] for rate in plan["rates"] for fork in range(plan["forks"])}
    require(seen == expected, f"Runtime/rate/fork inventory differs: missing={sorted(expected-seen)}, unexpected={sorted(seen-expected)}")
    groups = group_forks(plan, rows)
    summary = {
        "schema_version": 1,
        "qualification": False,
        "global_capture_complete": False,
        "source_manifest_sha256": plan["builds"]["native"]["source_manifest_sha256"],
        "plan_sha256": digest(plan_directory / "plan.json"),
        "execution_sha256": digest(plan_directory / "execution/results.json"),
        "terminal_sha256": digest(terminal_path) if terminal is not None else None,
        "quiet_window_id": terminal["quiet_window_id"] if terminal is not None else execution["quiet_window_id"],
        "processes": len(rows),
        "total_offered": sum(row["offered"] for row in rows),
        "total_completed": sum(row["completed"] for row in rows),
        "total_deadline_completed": sum(row["deadline_completed"] for row in rows),
        "sample_adequate_forks": sum(row["sample_floor_satisfied"] for row in rows),
        "groups": groups, "forks": rows, "no_workload_reruns": True,
        "scope": plan["scope"], "workload": plan["workloads"][0], "plan": plan,
    }
    return summary, timeline, histograms, packets


def write_csv(path, rows):
    fields = list(dict.fromkeys(key for row in rows for key in row))
    with path.open("w") as stream:
        writer = csv.DictWriter(stream, fieldnames=fields)
        writer.writeheader()
        writer.writerows(rows)


def write_data(output, summary, timeline, histograms, packets):
    write_json(output / "pilot-summary.json", summary)
    write_json(output / "pilot-validated-packets.json", packets)
    write_json(output / "pilot-histograms.json", histograms)
    write_csv(output / "pilot-forks.csv", [
        {key: value for key, value in row.items() if not isinstance(value, (dict, list))}
        for row in summary["forks"]
    ])
    write_csv(output / "pilot-progress.csv", timeline)
    lines = [
        "| Runtime | Rate /s | By-deadline /s | End backlog | Service p50 upper ms | Response p50 upper ms | Response p99 upper ms |",
        "|---|---:|---:|---:|---:|---:|---:|",
    ]
    for group in summary["groups"]:
        metrics = group["metrics"]
        lines.append(
            f"| {group['runtime']} | {group['rate']} | {metrics['by_deadline_ops_s']['median']:.1f} "
            f"| {metrics['deadline_backlog']['median']:g} | {metrics['service_p50_upper_ms']['median']:.3f} "
            f"| {metrics['response_p50_upper_ms']['median']:.3f} | {metrics['response_p99_upper_ms']['median']:.3f} |"
        )
    (output / "PILOT_TABLE.md").write_text("\n".join(lines) + "\n")
    return lines


def plot_metric(axis, metric, title, ylabel, logarithmic, plan, groups):
    for runtime in plan['runtimes']:
        selected = [g for g in groups if g['runtime'] == runtime]
        x = [g['rate'] for g in selected]
        metrics = [g['metrics'][metric] for g in selected]
        y = [metric_value['median'] for metric_value in metrics]
        axis.plot(x, y, marker=MARKERS[runtime], color=COLORS[runtime], label=LABELS[runtime], linewidth=1.8)
        for rate, value in zip(x, metrics):
            axis.vlines(rate, value['min'], value['max'], color=COLORS[runtime], linewidth=1.2)
            axis.scatter([rate * 0.985, rate, rate * 1.015], value['forks'], color=COLORS[runtime], s=14, alpha=0.55)
    axis.set_xscale('log', base=2)
    axis.set_xticks(plan['rates'], plan['rates'])
    axis.set_xlabel('Offered calls per second')
    axis.set_ylabel(ylabel)
    axis.set_title(title)
    axis.grid(True, alpha=0.2)
    if logarithmic:
        axis.set_yscale('log')


def save_figure(figure, name, output):
    for extension in ("svg", "png"):
        figure.savefig(output / f"{name}.{extension}", dpi=170, bbox_inches="tight")
    plt.close(figure)


def render_charts(output, plan, groups, timeline):
    plt.rcParams.update({'font.size': 11, 'axes.titlesize': 13, 'figure.facecolor': 'white', 'axes.spines.top': False, 'axes.spines.right': False})
    figure, axes = plt.subplots(2, 2, figsize=(13.5, 9.2))
    figure.subplots_adjust(top=0.83, bottom=0.13, hspace=0.38, wspace=0.25)
    plot_metric(axes[0, 0], 'by_deadline_ops_s', 'Completed by the offer deadline', 'Completed calls per second', False, plan, groups)
    axes[0, 0].plot(plan['rates'], plan['rates'], color='#777777', linestyle='--', linewidth=1, label='All offered work')
    plot_metric(axes[0, 1], 'deadline_backlog', 'Incomplete at the offer deadline', 'Requests, including any in-flight call', False, plan, groups)
    plot_metric(axes[1, 0], 'service_p50_upper_ms', 'Service: empirical p50 upper bucket', 'Milliseconds · logarithmic', True, plan, groups)
    plot_metric(axes[1, 1], 'response_p50_upper_ms', 'Scheduled response: empirical p50 upper bucket', 'Milliseconds · logarithmic', True, plan, groups)
    figure.suptitle(f"Persistent {plan['workloads'][0]} ring · fixed-arrival diagnostic\n{plan['seconds']} seconds · {plan['warmup']} warmups · {plan['forks']} independent forks per point", fontsize=17, y=0.985)
    handles, legend = axes[0, 0].get_legend_handles_labels()
    figure.legend(handles, legend, loc='upper center', bbox_to_anchor=(0.5, 0.91), ncol=3, frameon=False)
    figure.text(0.5, 0.025, 'Dots: each fork; lines: fork medians; bars: observed ranges. Same two-CPU / 512 MiB envelope and 64 MiB payload or Java-heap ceilings.\nAll points miss the predeclared sample floor. No capacity bracket, extreme-tail confidence, pause or endurance qualification.', ha='center', fontsize=10)
    save_figure(figure, 'scheduled-pilot-overview', output)
    figure, axes = plt.subplots(1, 2, figsize=(13.5, 5.2))
    figure.subplots_adjust(top=0.75, bottom=0.23, wspace=0.25)
    plot_metric(axes[0], 'response_p99_upper_ms', 'Scheduled response: empirical p99 upper bucket', 'Milliseconds · logarithmic', True, plan, groups)
    plot_metric(axes[1], 'response_max_ms', 'Largest observed scheduled response', 'Milliseconds · logarithmic', True, plan, groups)
    figure.suptitle('Finite scheduled-response observations', fontsize=17, y=0.99)
    handles, legend = axes[0].get_legend_handles_labels()
    figure.legend(handles, legend, loc='upper center', bbox_to_anchor=(0.5, 0.9), ncol=5, frameon=False)
    figure.text(0.5, 0.025, 'All three fork values and ranges retained; histograms are not pooled. Unstarted requests have no completed-response sample.\nThese short-run order statistics do not establish p99.9/p99.99 behavior; deadline backlog remains visible in the main graph.', ha='center', fontsize=10)
    save_figure(figure, 'scheduled-response-observations', output)
    figure, axes = plt.subplots(1, 5, figsize=(17, 4.6), sharey=True)
    figure.subplots_adjust(top=0.75, bottom=0.25, wspace=0.12)
    rate_colors = plt.cm.viridis(np.linspace(0.08, 0.88, len(plan['rates'])))
    for axis, runtime in zip(axes, plan['runtimes']):
        for rate, color in zip(plan['rates'], rate_colors):
            for fork in range(plan['forks']):
                progress = [row for row in timeline if row['runtime'] == runtime and row['rate'] == rate and (row['fork'] == fork)]
                axis.plot([row['elapsed_seconds'] for row in progress], [row['waiting_backlog'] for row in progress], color=color, alpha=0.65, linewidth=1.2, label=f'{rate}/s' if fork == 0 else None)
        axis.set_title(LABELS[runtime])
        axis.set_xlabel('Seconds from offer start')
        axis.grid(True, alpha=0.2)
        axis.set_xlim(0, plan['seconds'] * 1.02)
    axes[0].set_ylabel('Observed waiting backlog')
    figure.suptitle('Bounded progress observations: every fork retained', fontsize=17, y=0.99)
    handles, legend = axes[0].get_legend_handles_labels()
    figure.legend(handles, legend, loc='upper center', bbox_to_anchor=(0.5, 0.9), ncol=len(plan['rates']), frameon=False)
    figure.text(0.5, 0.025, 'Up to 128 completion-relative progress rows per fork. Waiting backlog excludes the service call; deadline backlog includes incomplete in-flight work.\nA finite rising or flat trace is descriptive, not a sustained stability proof or complete queue-event trace.', ha='center', fontsize=10)
    save_figure(figure, 'scheduled-backlog-progress', output)


def write_report(output, summary, rows, groups, table_lines, plan):
    report = [
        '# Fixed-arrival persistent-ring pilot',
        '',
        f"All {len(rows)} predeclared processes passed independent source, executable, bytecode, admission, PID, watchdog, cgroup, checksum and result checks. No capture was rerun. The fixed policy offered {summary['total_offered']:,} calls; {summary['total_deadline_completed']:,} completed by the deadline and {summary['total_completed']:,} completed including {summary['total_completed'] - summary['total_deadline_completed']} late in-flight calls.",
        '',
        '![Deadline throughput, backlog and latency](scheduled-pilot-overview.png)',
        '',
        (
            'The table gives medians of three process observations. Latencies are empirical histogram upper '
            'buckets; CSV/JSON retains all fork values and raw bounded histograms. The response clock starts at '
            'the scheduled release, while service covers the invocation itself. Unstarted work has no response '
            'sample and remains in deadline/unstarted counts. These are finite workload observations, not '
            'throughput capacity bounds.'
        ),
        '',
        *table_lines,
        '',
        '## Sample sufficiency and the unchanged screen',
        '',
        f"{summary['sample_adequate_forks']}/{len(rows)} processes meet the predeclared {plan['screen_limits']['minimum_completions']:,}-completion floor. The historical screen remains {plan['screen_limits']['p999_us']:,} us response p99.9 upper bucket and {plan['screen_limits']['backlog_us']:,} us waiting-backlog time. Every point is sample-inadequate, regardless of individual latency/backlog comparisons. No capacity bracket or extreme-tail confidence is established. Deadline completion behavior is reported independently of those latency criteria.",
        '',
        '![Response observations](scheduled-response-observations.png)',
        '',
        '## Observed backlog behavior',
        '',
        '![Progress observations](scheduled-backlog-progress.png)',
        '',
        (
            'The logical FIFO counts arrivals independently of consumer iterations, so continuously busy service '
            'does not hide offered work. No new call starts at/after the deadline. At most one in-flight call '
            'completes afterward; remaining requests are unstarted. Final heap drain is separate. Progress '
            'samples and peak waiting backlog cover selected completion/dispatch/horizon boundaries and exclude '
            'the active service call. They do not constitute a complete continuous queue-event trace.'
        ),
        '',
        (
            '| Runtime | Rate /s | Deadline-completed counts, all forks | Deadline backlog, all forks | Late '
            'in-flight, all forks | Observed waiting growth /s, all forks |'
        ),
        '|---|---:|---|---|---|---|',
    ]
    for group in groups:
        metrics = group['metrics']
        report.append(f"| {group['runtime']} | {group['rate']} | {metrics['deadline_completed']['forks']} | {metrics['deadline_backlog']['forks']} | {metrics['late_inflight']['forks']} | {[round(value, 3) for value in metrics['observed_waiting_growth_per_s']['forks']]} |")
    report += [
        '',
        '## Collection exposure, lifetime accounting and resources',
        '',
        (
            'Canonical primary-cycle bounds use four passive observations around the schedule, including driver '
            'gaps and observer work; cycles need not have begun inside the interval. Each capture verifies '
            'whole-workload logical allocations/releases plus final cleanup and all compiled method '
            'identities/counters where applicable. Peers retain each outer GC-bean count separately: observed '
            'activity across that bracket is not a conservative in-clock completion lower bound and bean '
            'categories are not summed into fabricated cycles.'
        ),
        '',
        (
            '| Runtime | Rate /s | Collection observations, all forks | Lifetime peak RSS MiB median [range] | '
            'Lifetime CPU s median [range] |'
        ),
        '|---|---:|---|---|---|',
    ]
    for group in groups:
        selected = [row for row in rows if row['runtime'] == group['runtime'] and row['rate'] == group['rate']]
        exposure = [row['primary_cycles'] if 'primary_cycles' in row else {bean['name']: None if bean['before'] is None else bean['after'] - bean['before'] for bean in row['gc_collection_counts']} for row in selected]
        rss = [row['lifetime_peak_rss_mib'] for row in selected]
        cpu = [row['lifetime_cpu_seconds'] for row in selected]
        report.append(f"| {group['runtime']} | {group['rate']} | {exposure} | {statistics.median(rss):.3f} [{min(rss):.3f}, {max(rss):.3f}] | {statistics.median(cpu):.6f} [{min(cpu):.6f}, {max(cpu):.6f}] |")
    report += [
        '',
        (
            'RSS and CPU cover the complete Linux child lifetime, including reaped descendants; RSS is a maximum,'
            ' not simultaneous tree total. WASI describes the Wasmtime host. All runtimes retain the original '
            'two-CPU / 512 MiB cgroup envelope. Canonical payload and peer Java heaps have equal 64 MiB numeric '
            'ceilings but different representation and collection policies. Reserved catalog charge, metadata, '
            'program/JIT, native allocators, stacks and host costs remain distinct from managed payload and '
            'inside process/cgroup scope. The raw JSON retains exact reserved charge, preparation, final '
            'detach/drain/drop, program teardown and payload state for every canonical fork.'
        ),
        '',
        f"Processes with nonzero cgroup memory events: {sum((any(row['memory_events'].values()) for row in rows))}. Processes with CPU throttled periods: {sum((row['cpu_throttled_periods'] > 0 for row in rows))}. These are lifetime observations, not an attribution to an individual call.",
        '',
        (
            'The workload retains one heap and attached context across warmup and the schedule. HotSpot '
            'admits/compiles under its own policy; canonical admission/compilation, setup and final heap drain '
            'are separately observed. This is a different timer and arrival scope from the earlier closed-loop '
            'invocation comparison. Scheduler sleep/yield, histogram updates, counters and observers contribute '
            'to later response delay; overhead is not assumed zero.'
        ),
        '',
        '## Reproduction and limits',
        '',
        (
            'Run `python benchmarks/gc/plot_scheduled_ring.py COMPLETED_PLAN --output NEW_DIRECTORY` with '
            'Matplotlib and NumPy installed. The command reads the supplied completed fixed-rate plan, '
            'revalidates every bound raw capture using the existing independent checker, writes tables/CSV/JSON '
            'and recreates these SVG/PNG graphs. It executes no workload and writes only to the new output '
            'directory. Captures retain recorded absolute launch paths, which the underlying identity checker '
            'validates.'
        ),
        '',
        (
            'No result here qualifies global/equivalent pauses, sustainable capacity, p99.9/p99.99, general '
            'multi-mutator fairness, three paired hours or 24-hour endurance. Global capture completeness remains'
            ' false. A later throughput exploration must independently predeclare completion/backlog/sample '
            'requirements and retain the historical latency screen as a separate metric.'
        ),
    ]
    (output / 'REPORT.md').write_text('\n'.join(report) + '\n')


def write_provenance(output, plan_directory, summary):
    provenance = {
        "schema_version": 1,
        "qualification": False,
        "command": sys.argv,
        "python": sys.version,
        "matplotlib": matplotlib.__version__,
        "numpy": np.__version__,
        "renderer_sha256": digest(Path(__file__)),
        "plan_sha256": digest(plan_directory / "plan.json"),
        "execution_sha256": digest(plan_directory / "execution/results.json"),
        "source_manifest_sha256": summary["source_manifest_sha256"],
        "revalidated_processes": summary["processes"],
        "workload_runs": 0,
        "histogram_percentiles_pooled": False,
        "generated_unix": time.time(),
    }
    write_json(output / "provenance.json", provenance)
    files = {
        path.name: {"sha256": digest(path), "bytes": path.stat().st_size}
        for path in sorted(output.iterdir()) if path.is_file()
    }
    (output / "manifest.json").write_text(json.dumps(files, indent=2, sort_keys=True) + "\n")


def main(argv=None):
    plan_directory, output = parse_args(argv)
    summary, timeline, histograms, packets = revalidate(plan_directory)
    output.mkdir(parents=True, exist_ok=False)
    table_lines = write_data(output, summary, timeline, histograms, packets)
    render_charts(output, summary["plan"], summary["groups"], timeline)
    write_report(output, summary, summary["forks"], summary["groups"], table_lines, summary["plan"])
    write_provenance(output, plan_directory, summary)
    print(json.dumps({
        "report": str(output / "REPORT.md"),
        "manifest_sha256": digest(output / "manifest.json"),
        "revalidated_processes": summary["processes"],
        "workload_runs": 0,
    }, indent=2))


if __name__ == "__main__":
    try:
        main()
    except (KeyError, ValueError, OSError) as error:
        print(f"Scheduled report failed: {error}", file=sys.stderr)
        sys.exit(1)
