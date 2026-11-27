#!/usr/bin/env python3
"""Compare ordinary allocations made during a strong mark on two release builds."""
import argparse
import csv
import json
import os
from pathlib import Path
import platform
import random
import subprocess

from mark_cycles import validate_builds
from mechanisms import paired_interval, sha256, validate_target


SHAPES = ('scalar', 'graph')
METRICS = {'allocation_ns': 'Rooted allocation', 'finish_ns': 'Remaining collection',
           'total_ns': 'Allocation and collection'}


def measure(binary, shape, runner, collection):
    argv = ['marking_births', shape]
    if runner:
        argv = [str(runner), 'run', '-W', 'max-wasm-stack=8388608',
                '--argv0', 'marking_births', str(binary), shape]
    if collection == 'young':
        argv.append(collection)
    raw = subprocess.check_output(argv, executable=None if runner else str(binary),
                                  text=True, timeout=60)
    data = json.loads(raw)
    # Earlier collectors may enqueue scalar work. Each recorded scalar count
    # must still be either all queued or all inline under this fixed policy.
    inline = (0, 65536) if shape == 'scalar' else (0,)
    expected = dict(operations=65536, batch=512, checksum=16809984, teardown_bytes=0)
    if data.get('collection', 'full') != collection:
        raise ValueError(f'unexpected collection kind: {data}')
    if collection == 'young':
        expected['promoted'] = 65536
    if any(type(data.get(k)) is not int or data[k] != v for k, v in expected.items()):
        raise ValueError(f'{shape} workload or teardown differs: {data}')
    if type(data.get('inline_marks')) is not int or data['inline_marks'] not in inline:
        raise ValueError(f'{shape} inline mark count differs: {data}')
    if any(type(data.get(k)) is not int or data[k] <= 0 for k in ('allocation_ns', 'finish_ns')):
        raise ValueError(f'missing positive timers: {data}')
    data['total_ns'] = data['allocation_ns'] + data['finish_ns']
    return data


def plot(rows, output, target, collection):
    import matplotlib
    matplotlib.use('Agg')
    import matplotlib.pyplot as plt
    from statistics import median

    fig, axes = plt.subplots(1, len(METRICS), figsize=(10, 4), layout='constrained')
    for axis, (metric, title) in zip(axes, METRICS.items(), strict=True):
        for index, shape in enumerate(SHAPES):
            values = [[row[metric] / row['operations'] for row in rows
                       if row['shape'] == shape and row['variant'] == variant]
                      for variant in ('before', 'after')]
            x = [index - .15, index + .15]
            for pair in zip(*values, strict=True):
                axis.plot(x, pair, color='#9da3af', alpha=.45, linewidth=.8)
            for pos, series, label, color in zip(x, values, ('Before', 'After'), ('#505d76', '#167c67'),
                                                 strict=True):
                axis.scatter([pos] * len(series), series, color=color, s=12, alpha=.65,
                             label=label if index == 0 else None)
                axis.plot([pos - .06, pos + .06], [median(series)] * 2, color=color, linewidth=3)
        axis.set_xticks(range(len(SHAPES)), ['Scalar', 'Single edge'])
        axis.set_title(title)
        axis.grid(axis='y', alpha=.2)
    axes[0].set_ylabel('Nanoseconds per allocation')
    axes[0].legend(frameon=False)
    fig.suptitle(f'Allocation during a {collection} collection on {target}')
    fig.savefig(output)
    plt.close(fig)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for flag in ('before', 'after', 'output'):
        parser.add_argument('--' + flag, type=Path, required=True)
    parser.add_argument('--wasmtime', type=Path)
    parser.add_argument('--collection', choices=('full', 'young'), default='full')
    parser.add_argument('--pairs', type=int, default=16)
    parser.add_argument('--seed', type=int, default=52189)
    args = parser.parse_args()
    if args.pairs < 4 or args.pairs % 2:
        parser.error('--pairs must be even and at least four')
    builds = {k: json.loads(getattr(args, k).read_text()) for k in ('before', 'after')}
    validate_builds(builds)
    if any(b.get('fixture') != 'marking_births' or b.get('features') != [] for b in builds.values()):
        parser.error('expected default-feature marking_births build records')
    bins = {name: Path(b['binary']) for name, b in builds.items()}
    target = validate_target(bins['before'], bins['after'], args.wasmtime)
    if builds['before']['target'] != ('wasm' if args.wasmtime else 'native'):
        parser.error('recorded target differs from the binaries')
    args.output.mkdir(parents=True, exist_ok=False)
    rng = random.Random(args.seed)
    rows, results = [], {}
    for shape in SHAPES:
        for binary in bins.values():
            measure(binary, shape, args.wasmtime, args.collection)
        orders = [('before', 'after'), ('after', 'before')] * (args.pairs // 2)
        rng.shuffle(orders)
        pairs = {k: [] for k in METRICS}
        for n, order in enumerate(orders, 1):
            values = {}
            for pos, variant in enumerate(order, 1):
                sample = measure(bins[variant], shape, args.wasmtime, args.collection)
                rows.append(dict(shape=shape, pair=n, position=pos, variant=variant, **sample))
                values[variant] = sample
            for metric in METRICS:
                pairs[metric].append((values['before'][metric], values['after'][metric]))
        results[shape] = {k: paired_interval(values, args.seed) for k, values in pairs.items()}
    for name, binary in bins.items():
        if sha256(binary) != builds[name]['binary_sha256']:
            raise ValueError(f'{name} binary changed during measurement')
    with (args.output / 'samples.csv').open('x') as stream:
        writer = csv.DictWriter(stream, fieldnames=rows[0])
        writer.writeheader()
        writer.writerows(rows)
    report = dict(target=target, collection=args.collection, host=platform.platform(), pairs=args.pairs, seed=args.seed,
                  cpu_affinity=sorted(os.sched_getaffinity(0)) if hasattr(os, 'sched_getaffinity') else None,
                  wasmtime=subprocess.check_output([args.wasmtime, '--version'], text=True).strip()
                  if args.wasmtime else None, builds=builds, after_before=results, qualification=False)
    (args.output / 'report.json').write_text(json.dumps(report, indent=2) + '\n')
    plot(rows, args.output / 'marking-births.svg', target, args.collection)


if __name__ == '__main__':
    main()
