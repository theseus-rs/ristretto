#!/usr/bin/env python3
"""Report per-process workload-call timings in a finite ring capture."""
import argparse
import csv
import json
import math
import shutil
import tempfile
import xml.etree.ElementTree as ET
from contextlib import ExitStack
from pathlib import Path
from statistics import quantiles
from typing import NamedTuple


WORKLOADS = {"small": (8, 64, 63), "medium": (32, 1024, 255), "large": (128, 4096, 1023)}
# Okabe-Ito colors. Cycle shapes separately so colors can repeat with a new marker.
COLORS = ("#0072b2", "#d55e00", "#009e73", "#cc79a7", "#56b4e9", "#e69f00", "#000000", "#f0e442")
MARKERS = (
    "M-4 0a4 4 0 1 0 8 0a4 4 0 1 0 -8 0",  # circle
    "M-4 -4h8v8h-8z",                       # square
    "M0 -5l5 9h-10z",                       # triangle
    "M0 -5l5 5-5 5-5-5z",                   # diamond
    "M-5 -3h10L0 5z",                       # inverted triangle
)


class ForkRow(NamedTuple):
    fork: int
    collector: str
    median_call_us: float
    p25_call_us: float
    p75_call_us: float


def ring_generations(retained, steps):
    """Visit each removed generation, then the surviving ring from its head."""
    ring = list(range(retained))
    slot = 1
    for step in range(steps):
        yield ring[slot]
        ring[slot] = retained + step
        slot = (slot + 2) % retained
    yield from ring


def expected(workload, steps):
    if not isinstance(steps, int) or isinstance(steps, bool) or not 0 <= steps <= 100_000:
        raise ValueError("invalid capture step count")
    if workload not in WORKLOADS:
        raise ValueError(f"unknown ring workload: {workload}")
    retained, base, mask = WORKLOADS[workload]
    checksum = 1
    for generation in ring_generations(retained, steps):
        length = base + (generation & mask)
        # The first and last payload bytes are signed in Java.
        first = (generation + 128) % 256 - 128
        last = (generation * 17 + 3 + 128) % 256 - 128
        checksum = (31 * (checksum + generation) + length + 7 * first + last) & 0xffffffff
    return checksum if checksum < 2**31 else checksum - 2**32


def fork_rows(capture):
    if capture["schema"] != 1:
        raise ValueError("unsupported capture format")
    checksum = expected(capture["workload"], capture["steps"])
    if capture["expected"] != checksum:
        raise ValueError("capture checksum differs from the integer model")
    forks = {}
    count = None
    for process in capture["processes"]:
        fork, name = process["fork"], process["collector"]
        if not isinstance(fork, int) or isinstance(fork, bool) or fork < 0:
            raise ValueError("invalid fork index")
        if not isinstance(name, str) or not name.strip():
            raise ValueError("missing collector name")
        by_collector = forks.setdefault(fork, {})
        if name in by_collector:
            raise ValueError(f"duplicate process for fork {fork}, collector {name}")
        samples = process["samples"]
        if not samples or (count is not None and len(samples) != count):
            raise ValueError("inconsistent sample count")
        count = len(samples)
        times = [s["elapsed_ns"] for s in samples]
        if (any(s["checksum"] != checksum for s in samples)
                or any(not math.isfinite(t) or t <= 0 for t in times)):
            raise ValueError("invalid sample or checksum")
        if count == 1:
            low = middle = high = times[0]
        else:
            low, middle, high = quantiles(times, n=4, method="inclusive")
        by_collector[name] = ForkRow(fork, name, middle / 1000, low / 1000, high / 1000)

    if not forks:
        raise ValueError("empty capture")
    if sorted(forks) != list(range(len(forks))):
        raise ValueError("fork indices must be consecutive starting at zero")
    collectors = sorted(forks[0])
    for fork, processes in forks.items():
        if processes.keys() != forks[0].keys():
            raise ValueError(f"collector set differs at fork {fork}")
    return [forks[fork][name] for fork in range(len(forks)) for name in collectors]


def render_svg(capture, rows):
    collectors = sorted({row.collector for row in rows})
    indices = {name: index for index, name in enumerate(collectors)}
    fork_count = max(row.fork for row in rows) + 1
    left, top = 80, 65
    width = max(460, fork_count * max(55, 16 * len(collectors)))
    height = max(245, 22 * len(collectors))
    right, bottom = left + width, top + height
    svg_width = right + 55 + 10 * max(map(len, collectors))
    svg_height = bottom + 55
    svg = ET.Element("svg", xmlns="http://www.w3.org/2000/svg", attrib={
        "width": str(svg_width), "height": str(svg_height), "viewBox": f"0 0 {svg_width} {svg_height}",
        "font-family": "sans-serif", "font-size": "12", "fill": "#222",
    })

    def add(tag, parent=svg, **attrs):
        return ET.SubElement(parent, tag, {k.replace("_", "-"): str(v) for k, v in attrs.items()})

    def marker(parent, index, x, y):
        add("path", parent, d=MARKERS[index % len(MARKERS)], fill=COLORS[index % len(COLORS)],
            transform=f"translate({x:.3f} {y:.3f})", stroke="white", stroke_width=0.8)

    title = f"{capture['workload']} ring · {capture['steps']} steps per call"
    add("title").text = title
    add("rect", width="100%", height="100%", fill="white")
    add("text", x=left, y=24, font_size=18).text = title
    add("text", x=left, y=45).text = "Markers show medians. Bars span the 25th to 75th percentiles."

    peak = max(row.p75_call_us for row in rows)
    magnitude = 10 ** math.floor(math.log10(peak / 4))
    step = next(mult * magnitude for mult in (1, 2, 5, 10) if mult * magnitude >= peak / 4)
    ticks = math.ceil(peak / step)

    def y(value):
        return round(bottom - height * value / (ticks * step), 3)

    for tick in range(ticks + 1):
        value = tick * step
        add("line", x1=left, x2=right, y1=y(value), y2=y(value), stroke="#ddd")
        add("text", x=left - 9, y=y(value) + 4, text_anchor="end").text = f"{value:g}"
    add("text", transform=f"translate(18 {top + height / 2}) rotate(-90)",
        text_anchor="middle").text = "Workload-call time (µs)"
    spacing = width / fork_count
    for fork in range(fork_count):
        add("text", x=left + (fork + 0.5) * spacing, y=bottom + 18,
            text_anchor="middle").text = str(fork + 1)
    add("text", x=left + width / 2, y=bottom + 41, text_anchor="middle").text = "Fork"

    # Two series sit at −0.08 and +0.08 forks. Larger sets stay within one fork.
    dodge = min(0.16, 0.7 / max(1, len(collectors) - 1))
    for row in rows:
        index = indices[row.collector]
        offset = (index - (len(collectors) - 1) / 2) * dodge
        x = round(left + (row.fork + 0.5 + offset) * spacing, 3)
        group = add("g")
        add("title", group).text = (f"Fork {row.fork + 1}, {row.collector}: median {row.median_call_us:g} µs, "
                                    f"IQR {row.p25_call_us:g}–{row.p75_call_us:g} µs")
        add("line", group, x1=x, x2=x, y1=y(row.p25_call_us), y2=y(row.p75_call_us),
            stroke=COLORS[index % len(COLORS)], stroke_opacity=0.55, stroke_width=3, stroke_linecap="round")
        for bound in (row.p25_call_us, row.p75_call_us):
            add("line", group, x1=x - 5, x2=x + 5, y1=y(bound), y2=y(bound),
                stroke=COLORS[index % len(COLORS)], stroke_opacity=0.55)
        marker(group, index, x, y(row.median_call_us))
    for name, index in indices.items():
        legend_y = top + 8 + 22 * index
        marker(svg, index, right + 16, legend_y)
        add("text", x=right + 28, y=legend_y + 4).text = name
    return ET.tostring(svg, encoding="unicode") + "\n"


def write_outputs(csv_path, figure_path, rows, svg, overwrite):
    # Finish both files before publishing either. Reserve new names exclusively.
    with ExitStack() as temps, ExitStack() as rollback:
        staged = {}
        for path in (csv_path, figure_path):
            directory = temps.enter_context(tempfile.TemporaryDirectory(dir=path.parent))
            staged[path] = Path(directory) / "new"
        with staged[csv_path].open("w", newline="", encoding="utf-8") as stream:
            writer = csv.writer(stream)
            writer.writerow(ForkRow._fields)
            writer.writerows(rows)
        with staged[figure_path].open("w", newline="\n", encoding="utf-8") as stream:
            stream.write(svg)
        for path, temp in staged.items():
            if overwrite and path.exists():
                backup = temp.with_name("previous")
                shutil.copy2(path, backup, follow_symlinks=False)
                temp.replace(path)
                rollback.callback(backup.replace, path)
            else:
                with path.open("xb"):
                    pass
                rollback.callback(path.unlink, missing_ok=True)
                temp.replace(path)
        rollback.pop_all()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("capture", type=Path)
    parser.add_argument("csv", type=Path)
    parser.add_argument("figure", type=Path, help="output SVG")
    parser.add_argument("-f", "--overwrite", action="store_true", help="replace existing output files")
    args = parser.parse_args()
    outputs = (args.csv, args.figure)
    if len({path.resolve() for path in (args.capture, *outputs)}) != 3:
        parser.error("capture, CSV and SVG paths must be distinct")
    if args.figure.suffix.lower() != ".svg":
        parser.error("figure path must end in .svg")
    for path in outputs:
        if path.exists() and not args.overwrite:
            parser.error(f"output already exists: {path} (use --overwrite to replace)")
        if path.exists() and not path.is_file():
            parser.error(f"expected an output file: {path}")
    try:
        capture = json.loads(args.capture.read_text(encoding="utf-8"))
        rows = fork_rows(capture)
        write_outputs(args.csv, args.figure, rows, render_svg(capture, rows), args.overwrite)
    except (OSError, ValueError) as error:
        parser.error(str(error))


if __name__ == "__main__":
    main()
