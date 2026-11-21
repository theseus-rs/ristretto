#!/usr/bin/env python3
"""Report per-JVM workload-call medians in a paired finite ring capture."""
import argparse
import csv
import json
from pathlib import Path
from statistics import median

from verify_oracle import expected


def fork_rows(capture):
    if capture["schema"] != 1:
        raise ValueError("unsupported capture format")
    checksum = expected(capture["workload"], capture["steps"])
    if capture["expected"] != checksum:
        raise ValueError("capture checksum differs from the integer model")
    result = {}
    count = None
    for process in capture["processes"]:
        key = process["fork"], process["collector"]
        if key in result or key[1] not in ("G1", "ZGC"):
            raise ValueError("duplicate or unrecognized process")
        samples = process["samples"]
        if not samples or (count is not None and len(samples) != count):
            raise ValueError("inconsistent sample count")
        count = len(samples)
        if any(s["checksum"] != checksum or s["elapsed_ns"] <= 0 for s in samples):
            raise ValueError("invalid sample or checksum")
        result[key] = median(s["elapsed_ns"] for s in samples) / 1000
    if not result:
        raise ValueError("empty capture")
    forks = sorted({fork for fork, collector in result})
    if forks != list(range(len(forks))) or len(result) != 2 * len(forks):
        raise ValueError("missing paired JVM fork")
    return [(fork, name, result[fork, name]) for fork in forks for name in ("G1", "ZGC")]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("capture", type=Path)
    parser.add_argument("csv", type=Path)
    parser.add_argument("figure", type=Path, help="output SVG")
    args = parser.parse_args()
    capture = json.loads(args.capture.read_text())
    rows = fork_rows(capture)
    if args.csv.exists() or args.figure.exists() or args.figure.suffix != ".svg":
        parser.error("specify new CSV and SVG output paths")
    import matplotlib
    matplotlib.use("Agg")
    import matplotlib.pyplot as plt
    matplotlib.rcParams["svg.hashsalt"] = "ristretto-ring"
    fig, ax = plt.subplots(figsize=(6.8, 4), layout="constrained")
    for name, color, marker in [("G1", "#0072b2", "o"), ("ZGC", "#d55e00", "s")]:
        points = [(fork + 1, us) for fork, who, us in rows if who == name]
        x, y = zip(*points)
        ax.scatter(x, y, label=name, color=color, marker=marker)
    ax.set_xticks(sorted({fork + 1 for fork, _, _ in rows}))
    ax.set_ylim(bottom=0)
    ax.set_xlabel("Paired fork")
    ax.set_ylabel("Median workload-call time (µs)")
    ax.set_title(f"{capture['workload']} ring · {capture['steps']} steps per call")
    ax.legend()
    fig.savefig(args.figure, format="svg", metadata={"Date": None})
    plt.close(fig)
    with args.csv.open("x", newline="") as stream:
        writer = csv.writer(stream)
        writer.writerow(("fork", "collector", "median_call_us"))
        writer.writerows(rows)


if __name__ == "__main__":
    main()
