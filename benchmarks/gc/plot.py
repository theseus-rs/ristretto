#!/usr/bin/env python3
"""Export comparison figures as SVG and PNG from recorded measurements."""
import argparse
import hashlib
import json
from pathlib import Path
import platform


def plot(directory, output):
    import matplotlib
    matplotlib.use("Agg")
    import matplotlib.pyplot as plt
    summary = json.loads((directory / "summary.json").read_text())
    rows = [json.loads(s) for s in (directory / "raw.jsonl").read_text().splitlines() if s]
    output.mkdir(parents=True, exist_ok=True)
    runtimes = sorted({g["runtime"] for g in summary["groups"]})
    colors = dict(zip(runtimes, plt.get_cmap("tab10").colors))
    plt.rcParams.update({"font.size": 10, "axes.spines.top": False, "axes.spines.right": False})
    failures = len(summary["failures"])

    def save(fig, name):
        fig.text(.01, .01, f"Exploratory runtime screen • medians and min–max across processes • {failures} failed runs\n"
                 "HotSpot: JIT unless '-int'; Ristretto JIT modes end in '-jit' or '-jit-before'. Wasm separate. No qualification.", fontsize=8)
        fig.tight_layout(rect=(0, .065, 1, .96))
        for extension in ["svg", "png"]:
            fig.savefig(output / f"{name}.{extension}", dpi=160)
        plt.close(fig)

    # Separate figures by target; never imply native HotSpot is a Wasm collector.
    for target in sorted({g["target"] for g in summary["groups"]}):
        groups = [g for g in summary["groups"] if g["target"] == target]
        panels = sorted({(g["workload"], g["rate"]) for g in groups})
        if not panels:
            continue
        for metric, label, name in [("throughput_ops_s", "Completed operations / second", "throughput"),
                                    ("response_p999_us", "Arrival-to-completion p99.9 (µs)", "latency"),
                                    ("response_p9999_us", "Arrival-to-completion p99.99 (µs)", "latency-p9999"),
                                    ("process_peak_rss_mib", "Whole-process peak RSS (MiB)", "memory")]:
            fig, axes = plt.subplots(len(panels), 1, figsize=(10, max(3.5, 2.6 * len(panels))), squeeze=False)
            for ax, (workload, rate) in zip(axes[:, 0], panels):
                selected = [g for g in groups if (g["workload"], g["rate"]) == (workload, rate) and metric in g["metrics"]]
                for i, group in enumerate(selected):
                    m = group["metrics"][metric]
                    ax.errorbar(m["median"], i, xerr=[[m["median"] - m["min"]], [m["max"] - m["median"]]],
                                fmt="o", color=colors[group["runtime"]], capsize=3, markersize=6)
                    ax.annotate(f"{m['median']:,.1f}", (m["median"], i), xytext=(6, 5), textcoords="offset points", fontsize=7)
                ax.set_yticks(range(len(selected)), [g["runtime"] for g in selected])
                ax.invert_yaxis()
                ax.set_xscale("log")
                ax.margins(x=.2, y=.2)
                ax.set_xlabel(label + " • logarithmic scale")
                ax.set_title(f"{workload} | {'saturation' if rate == 0 else str(rate) + ' offered ops/s'} | {target}", loc="left")
                ax.grid(axis="x", alpha=.2)
            fig.suptitle(f"GC workload comparison: {label}", fontweight="bold")
            save(fig, f"{target}-{name}")
        workloads = sorted({g["workload"] for g in groups if g["rate"] > 0})
        if workloads:
            fig, axes = plt.subplots(len(workloads), 2, figsize=(13, 3.8 * len(workloads)), squeeze=False)
            for (left, right), workload in zip(axes, workloads):
                for row in rows:
                    if row["status"] != "ok" or row["target"] != target or row["rate"] == 0 or row["workload"] != workload:
                        continue
                    data = row["measurement"]
                    timeline = data["timeline"]
                    label = f"{row['runtime']} / {row['rate']}"
                    # Include final unfinished work even if no telemetry sample saw it.
                    x = [s[0] / 1e9 for s in timeline] + [data["elapsed_ns"] / 1e9]
                    y = [s[2] for s in timeline] + [data["unfinished"]]
                    left.plot(x, y, label=label, color=colors[row["runtime"]], alpha=.65,
                              linestyle="-" if row["repeat"] % 2 == 0 else "--")
                for group in groups:
                    if group["rate"] == 0 or group["workload"] != workload:
                        continue
                    right.scatter(group["metrics"]["throughput_ops_s"]["median"], group["metrics"]["response_p999_us"]["median"],
                                  color=colors[group["runtime"]], label=f"{group['runtime']} / {group['rate']}")
                left.set(xlabel="Measured elapsed seconds", ylabel="Pending scheduled operations", title=f"{workload}: backlog (individual runs)")
                right.set(xlabel="Completed operations / second", ylabel="Arrival p99.9 (µs)", title=f"{workload}: throughput–latency tradeoff", xscale="log", yscale="log")
                for ax in [left, right]:
                    ax.grid(alpha=.2)
                    handles, labels = ax.get_legend_handles_labels()
                    unique = dict(zip(labels, handles))
                    if unique:
                        ax.legend(unique.values(), unique.keys(), fontsize=6, ncol=2)
            save(fig, f"{target}-load")
    (output / "provenance.json").write_text(json.dumps({
        "matplotlib": matplotlib.__version__, "python": platform.python_version(),
        "sha256": {str(p): hashlib.sha256(p.read_bytes()).hexdigest()
                   for p in [Path(__file__), directory / "raw.jsonl", directory / "summary.json"]}
    }, indent=2) + "\n")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("results", type=Path)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    plot(args.results, args.output or args.results / "graphs")
