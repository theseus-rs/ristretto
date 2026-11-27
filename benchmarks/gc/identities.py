#!/usr/bin/env python3
"""Paired production identity-allocation screen; not GC or application throughput."""
import argparse
import json
import os
from pathlib import Path
import platform
import random
import statistics

from cgroups import Budget
from mechanisms import paired_interval
from run import digest, execute, freeze_build


def validate(data, producers, count):
    if (data['producers'] != producers or data['per_worker'] != count
            or data['operations'] != producers * count or data['unique'] is not True):
        raise ValueError('identity count or uniqueness failure')
    if (type(data['wall_ns']) is not int or data['wall_ns'] <= 0
            or len(data['worker_ns']) != producers
            or any(type(ns) is not int or ns <= 0 or ns > data['wall_ns'] for ns in data['worker_ns'])):
        raise ValueError('invalid worker timings')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--before', type=Path, required=True)
    parser.add_argument('--after', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--cpus', type=int, nargs='+', required=True)
    parser.add_argument('--cpu-quota', type=float, required=True)
    parser.add_argument('--cgroup-parent', type=Path, required=True)
    parser.add_argument('--rss-mib', type=int, default=256)
    parser.add_argument('--producers', type=int, nargs='+', default=[1, 4])
    parser.add_argument('--operations', type=int, default=1000000, help='identities per producer')
    parser.add_argument('--repeats', type=int, default=7)
    parser.add_argument('--seed', type=int, default=42)
    parser.add_argument('--plot', action='store_true')
    args = parser.parse_args()
    if (not set(args.cpus) <= os.sched_getaffinity(0) or not 0 < args.cpu_quota <= len(args.cpus)
            or not 1 <= args.operations <= 10000000 or args.repeats < 2
            or any(not 1 <= p <= 32 for p in args.producers) or len(set(args.producers)) != len(args.producers)):
        parser.error('invalid CPU, producer, operation or repetition count')
    output = args.output.resolve(); output.mkdir(parents=True, exist_ok=False)
    binaries = {name: output / name for name in ['before', 'after']}
    inputs = []
    for name, binary in binaries.items():
        inputs.extend(freeze_build(getattr(args, name), binary))
    inputs.extend(Path(__file__).with_name(name).resolve() for name in ['identities.py', 'run.py', 'cgroups.py', 'mechanisms.py'])
    hashes = {str(p): digest(p) for p in inputs}
    (output / 'metadata.json').write_text(json.dumps({
        'qualification': False, 'host': platform.platform(), 'python': platform.python_version(),
        'options': {k: str(v) if isinstance(v, Path) else v for k, v in vars(args).items()},
        'hashes': hashes, 'scope': 'production identity allocator, preallocated output vectors; validation/sorting excluded; no GC or application throughput claim'
    }, indent=2) + '\n')
    jobs = []; rng = random.Random(args.seed)
    for repeat in range(args.repeats):
        producers = list(args.producers); rng.shuffle(producers)
        for producer in producers:
            order = list(binaries); rng.shuffle(order)
            jobs.extend((repeat, producer, name) for name in order)
    (output / 'schedule.json').write_text(json.dumps(jobs) + '\n')
    rows = []
    with (output / 'raw.jsonl').open('w') as raw:
        for number, (repeat, producer, name) in enumerate(jobs):
            directory = output / f'run-{number:03d}-{name}-{producer}'; directory.mkdir()
            command = [str(binaries[name]), str(producer), str(args.operations)]
            row = {'repeat': repeat, 'producers': producer, 'binary': name, 'command': command, 'directory': directory.name}
            budget = Budget(args.cgroup_parent, args.rss_mib * 1024**2, round(args.cpu_quota * 100000))
            try:
                row.update(execute(command, directory, 120, args.cpus, dict(os.environ), budget))
                row['cgroup'] = budget.snapshot()
            finally:
                budget.close()
            try:
                if row['status'] != 'ok' or row['cgroup']['memory_events'].get('oom', 0):
                    raise ValueError('failed or memory-limited process')
                data = json.loads((directory / 'stdout.txt').read_text())
                validate(data, producer, args.operations); row['measurement'] = data
            except (ValueError, KeyError, TypeError) as error:
                row.update(status='invalid', error=str(error))
            rows.append(row); raw.write(json.dumps(row) + '\n'); raw.flush()
    unchanged = all(digest(Path(p)) == expected for p, expected in hashes.items())
    failures = [r['directory'] for r in rows if r['status'] != 'ok']
    results = []
    for producer in args.producers:
        selected = [r for r in rows if r['producers'] == producer]
        if any(r['status'] != 'ok' for r in selected):
            continue  # Never select only the successful pairs after a failure.
        for metric in ['wall_ns_per_id', 'worker_ns_per_id']:
            def value(row):
                m = row['measurement']
                return (m['wall_ns'] if metric == 'wall_ns_per_id' else sum(m['worker_ns'])) / m['operations']
            pairs = []
            for repeat in range(args.repeats):
                pair = {r['binary']: value(r) for r in selected if r['repeat'] == repeat}
                pairs.append((pair['before'], pair['after']))
            results.append({'producers': producer, 'metric': metric,
                            'before': statistics.median(a for a, _ in pairs),
                            'after': statistics.median(b for _, b in pairs), 'ratio': paired_interval(pairs)})
    (output / 'summary.json').write_text(json.dumps({'qualification': False, 'inputs_unchanged': unchanged,
        'failures': failures, 'results': results}, indent=2) + '\n')
    lines = ['# Identity allocator screen', '', 'Isolated allocator mechanism, not GC/application throughput. Output buffers are preallocated; validation/sorting are excluded. Ratios resample independent process pairs. Worker elapsed time includes scheduling delay.', '',
             '| Producers | Metric | Before | After | After/before [95% interval] |', '|---:|---|---:|---:|---|']
    for row in results:
        ratio = row['ratio']
        lines.append(f"| {row['producers']} | {row['metric']} | {row['before']:.3f} | {row['after']:.3f} | {ratio['median']:.3f} [{ratio['low']:.3f}, {ratio['high']:.3f}] |")
    lines += ['', f'Failed runs: {len(failures)}. Inputs unchanged: {unchanged}. Qualification: false.']
    (output / 'REPORT.md').write_text('\n'.join(lines) + '\n')
    if args.plot:
        import matplotlib
        matplotlib.use('Agg')
        import matplotlib.pyplot as plt
        fig, axes = plt.subplots(1, 2, figsize=(11, 4))
        for axis, metric in zip(axes, ['wall_ns_per_id', 'worker_ns_per_id']):
            selected = [r for r in results if r['metric'] == metric]
            for offset, name, color in [(-.18, 'before', '#64748b'), (.18, 'after', '#0284c7')]:
                axis.bar([i + offset for i in range(len(selected))], [r[name] for r in selected], .36, label=name, color=color)
            axis.set_xticks(range(len(selected)), [r['producers'] for r in selected]); axis.set_xlabel('Producers')
            axis.set_ylabel(metric.replace('_', ' ')); axis.legend()
        fig.suptitle('Production identity allocation — isolated mechanism')
        fig.text(.01, .01, f'Medians across processes; {len(failures)} failures. Paired intervals in report. No GC throughput or release qualification.', fontsize=8)
        fig.tight_layout(rect=(0, .05, 1, .95))
        for extension in ['svg', 'png']: fig.savefig(output / f'identities.{extension}', dpi=160)
        plt.close(fig)
        (output / 'graph-provenance.json').write_text(json.dumps({'matplotlib': matplotlib.__version__,
            'sha256': {str(p): digest(p) for p in [Path(__file__), output / 'raw.jsonl', output / 'summary.json']}}, indent=2) + '\n')
    return int(bool(failures) or not unchanged)


if __name__ == '__main__':
    raise SystemExit(main())
