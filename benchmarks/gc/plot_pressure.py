#!/usr/bin/env python3
"""Export measured GC/cgroup time series from the collector endurance driver."""
import argparse
import hashlib
import json
from pathlib import Path


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("results", type=Path)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    import matplotlib
    matplotlib.use("Agg")
    import matplotlib.pyplot as plt
    data = json.loads(args.results.read_text())
    args.output.mkdir(parents=True, exist_ok=True)
    for index, run in enumerate(data["runs"]):
        measurement = run.get("measurement", {})
        samples = measurement.get("capacity_samples", [])
        if not samples:
            continue
        fig, axes = plt.subplots(3, 1, figsize=(11, 9), layout="constrained")
        seconds = [s["seconds"] for s in samples]
        axes[0].plot(seconds, [s["backlog_operations"] for s in samples], color="darkorange")
        axes[0].set(ylabel="Pending scheduled operations", title="Workload backlog (sampled)")
        for key, label in [("logical_bytes", "logical"), ("shared_committed_bytes", "arena committed"),
                           ("retired_bytes", "retired")]:
            if all(s.get(key) is not None for s in samples):
                axes[1].plot(seconds, [s[key] / 2**20 for s in samples], label=label)
        axes[1].set(ylabel="MiB", title="Collector accounting (sampled)")
        axes[1].legend()
        resources = run.get("resource_samples", [])
        axes[2].plot([s["seconds"] for s in resources], [s["memory_bytes"] / 2**20 for s in resources], label="cgroup charged memory")
        limit = int(run["cgroup"]["limits"]["memory.max"]) / 2**20
        peak = run["cgroup"]["memory_peak"] / 2**20
        axes[2].axhline(peak, linestyle="--", color="grey", label=f"whole-process peak: {peak:.1f} MiB")
        axes[2].set(xlabel="Elapsed seconds", ylabel="MiB",
                    title=f"Whole-process cgroup memory; enforced ceiling {limit:g} MiB")
        axes[2].legend()
        for axis in axes:
            axis.grid(alpha=.2)
            axis.set_ylim(bottom=0)
        fig.suptitle(f"Collector pressure screen • {measurement['operations']:,} completed operations • status: {run['status']}\n"
                     "Instrumented run; samples can miss short peaks. Not endurance qualification.")
        for extension in ["svg", "png"]:
            fig.savefig(args.output / f"pressure-{index}.{extension}", dpi=160)
        plt.close(fig)
    (args.output / "provenance.json").write_text(json.dumps({
        "matplotlib": matplotlib.__version__,
        "input_sha256": hashlib.sha256(args.results.read_bytes()).hexdigest(),
        "renderer_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
    }, indent=2) + "\n")


if __name__ == "__main__":
    main()
