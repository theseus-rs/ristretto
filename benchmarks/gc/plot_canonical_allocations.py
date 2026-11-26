"""Plot receipt-bound Rust allocator counts; these are not timing or RSS results."""
import argparse
import csv
import hashlib
import json
from pathlib import Path
import platform

import matplotlib
import matplotlib.pyplot as plt


def digest(path):
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("directory", type=Path, help="Completed after-count matrix directory")
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    path = args.directory / "comparison.json"
    comparison = json.loads(path.read_text())
    receipt = json.loads((args.directory / "final-receipt.json").read_text())
    if (receipt["comparison_sha256"] != digest(path)
            or receipt["summary_sha256"] != digest(args.directory / "summary.json")
            or receipt["summary_sha256"] != comparison["after_summary_sha256"]
            or not receipt["exact_baseline_inputs"]
            or not comparison["exact_inputs"]
            or not comparison["semantic_results_and_managed_exposure_equal"]
            or comparison["qualification"] or comparison["timing_conclusions"]):
        raise ValueError("count comparison binding or diagnostic scope invalid")
    rows = {}
    for row in comparison["comparisons"]:
        if row["phase"] != "full_invocation" or not row["label"].endswith("-257"):
            continue
        if row["metric"] not in {"requests", "requested_bytes"}:
            continue
        key = row["label"], row["metric"]
        if key in rows or row["samples"] != 3:
            raise ValueError("unexpected count sample shape")
        for prefix in ["before", "after"]:
            if not 0 < row[f"{prefix}_min"] <= row[f"{prefix}_median"] <= row[f"{prefix}_max"]:
                raise ValueError("invalid count interval")
        rows[key] = row
    tiers = [("native-interpreted", "Native interpreter"),
             ("native-compiled", "Native compiled"), ("wasi-interpreted", "WASI interpreter")]
    sizes = ["small", "medium", "large"]
    labels = [(f"{tier}-{size}-257", f"{name} · {size}") for tier, name in tiers for size in sizes]
    if len(rows) != 18:
        raise ValueError("expected all nine configurations and both metrics")
    args.output.mkdir(parents=True, exist_ok=False)
    plt.rcParams.update({"font.size": 10, "svg.fonttype": "none"})
    figure, axes = plt.subplots(1, 2, figsize=(13, 7), sharey=True)
    for axis, metric, divisor, title in [
        (axes[0], "requests", 1, "Rust allocator requests per invocation"),
        (axes[1], "requested_bytes", 1024 * 1024, "Requested bytes per invocation (MiB)"),
    ]:
        for offset, prefix, color, legend in [
            (-0.18, "before", "#64748b", "Before metadata links"),
            (0.18, "after", "#0f766e", "After metadata links"),
        ]:
            selected = [rows[label, metric] for label, _ in labels]
            medians = [row[f"{prefix}_median"] / divisor for row in selected]
            errors = [[(row[f"{prefix}_median"] - row[f"{prefix}_min"]) / divisor for row in selected],
                      [(row[f"{prefix}_max"] - row[f"{prefix}_median"]) / divisor for row in selected]]
            axis.barh([index + offset for index in range(len(labels))], medians,
                      height=0.33, color=color, label=legend, xerr=errors, capsize=2)
        axis.set_title(title, loc="left", fontsize=11)
        axis.set_xlim(left=0)
        axis.grid(axis="x", alpha=0.2)
        axis.set_axisbelow(True)
        axis.spines[["top", "right"]].set_visible(False)
    axes[0].set_yticks(range(len(labels)), [label for _, label in labels])
    axes[0].invert_yaxis()
    handles, names = axes[0].get_legend_handles_labels()
    figure.legend(handles, names, loc="upper right", bbox_to_anchor=(0.99, 0.99), fontsize=9)
    figure.suptitle("Canonical metadata lookup allocation costs", x=0.25, ha="left", fontsize=16)
    figure.text(0.25, 0.025, "257 replacements; medians and ranges of 3 sequential invocations per configuration.\n"
                "Full invocation includes heap/frame setup and cleanup. Rust allocator counts exclude WASI host allocation.\n"
                "Same inputs, checksums and managed-object exposure. No timing, RSS or performance-qualification claim.", fontsize=9)
    figure.subplots_adjust(left=0.25, right=0.98, top=0.90, bottom=0.14, wspace=0.12)
    for suffix in ["svg", "png"]:
        figure.savefig(args.output / f"allocations.{suffix}", dpi=160)
    plt.close(figure)
    with (args.output / "counts.csv").open("w", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=list(next(iter(rows.values()))))
        writer.writeheader()
        writer.writerows(rows.values())
    result = {"comparison_sha256": digest(path), "receipt_sha256": digest(args.directory / "final-receipt.json"),
              "plotter_sha256": digest(Path(__file__)), "python": platform.python_version(),
              "matplotlib": matplotlib.__version__, "qualification": False, "timing_conclusions": False,
              "artifacts": {path.name: digest(path) for path in args.output.iterdir() if path.is_file()}}
    (args.output / "manifest.json").write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
