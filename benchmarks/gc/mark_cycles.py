#!/usr/bin/env python3
"""Frozen paired full-graph collection screen; not pause or throughput qualification."""
import argparse
import json
import os
from pathlib import Path
import platform
import random
import shutil
import statistics
import subprocess

from paired_measurement import paired_interval, sha256

INVOCATION_NAME = 'mark_cycles'


def invocation(binary, arguments, runner=None):
    """Keep argv[0] independent of the frozen artifact's location and label."""
    if runner:
        return [str(runner), 'run', '-W', 'max-wasm-stack=8388608',
                '--argv0', INVOCATION_NAME, str(binary), *arguments], None
    return [INVOCATION_NAME, *arguments], str(binary)


def validate(data, shape, policy, nodes, cycles, workers, diagnostics):
    expected = dict(schema=1, shape=shape, policy=policy, nodes=nodes, cycles=cycles,
                    workers=workers, diagnostics=diagnostics, marked=nodes * cycles,
                    completed=cycles, checksum=nodes * (nodes + 1) // 2,
                    teardown_live=0, teardown_bytes=0)
    if shape == 'mixed':
        swept = max(1, nodes // 4) * cycles
        expected.update(swept=swept, freed=swept * 8)
    for key, value in expected.items():
        if type(data.get(key)) is not type(value) or data[key] != value:
            raise ValueError(f'{key}: missing or inconsistent evidence')
    if type(data.get('elapsed_ns')) is not int or data['elapsed_ns'] <= 0:
        raise ValueError('invalid collection duration')
    tasks = data.get('trace_tasks')
    if type(tasks) is not int or (tasks < nodes * cycles if diagnostics else tasks != 0):
        raise ValueError('inconsistent tracing evidence')
    if shape == 'barrier':
        expected_records = (nodes - 1) * cycles if diagnostics else 0
        for field, value in [('barrier_buffered', expected_records),
                             ('barrier_flushed', expected_records), ('barrier_overflows', 0)]:
            if type(data.get(field)) is not int or data[field] != value:
                raise ValueError(f'{field}: missing or inconsistent barrier work')
        if workers != 0:
            raise ValueError('barrier fixture requires cooperative collection')


def validate_builds(builds):
    if set(builds) != {'before', 'after'}:
        raise ValueError('need both builds')
    a, b = builds['before'], builds['after']
    for field in ['fixture_sha256', 'toolchain', 'target', 'profile_sha256', 'dependencies_sha256']:
        if not a.get(field) or a[field] != b.get(field):
            raise ValueError(f'build {field} differs or is missing')
    if a['target'] not in ['native', 'wasm']:
        raise ValueError('unknown build target')
    if a.get('source_sha256') != b.get('source_sha256') and a.get('binary_sha256') == b.get('binary_sha256'):
        raise ValueError('different sources produced identical timed binaries; verify independent target directories')
    for build in builds.values():
        if not build.get('source_sha256') or sha256(Path(build['binary'])) != build['binary_sha256']:
            raise ValueError('unrecorded source or changed binary')


def summarize(rows, conditions, repeats, seed):
    expected = {(repeat, *condition, label) for repeat in range(repeats)
                for condition in conditions for label in ['before', 'after']}
    selected = {}
    for row in rows:
        key = (row['repeat'], row['shape'], row['policy'], row['workers'], row['diagnostics'], row['binary'])
        if key not in expected or key in selected or row['status'] != 'ok':
            raise ValueError('failed, duplicate or unexpected process; no survivor-only comparison')
        selected[key] = row['measurement']
    if set(selected) != expected:
        raise ValueError('incomplete paired matrix')
    results = []
    for condition in conditions:
        pairs = []
        for repeat in range(repeats):
            values = [selected[repeat, *condition, label] for label in ['before', 'after']]
            pairs.append(tuple(v['elapsed_ns'] / v['marked'] for v in values))
        shape, policy, workers, diagnostics = condition
        results.append(dict(shape=shape, policy=policy, workers=workers, diagnostics=diagnostics,
                            before_ns=statistics.median(a for a, _ in pairs),
                            after_ns=statistics.median(b for _, b in pairs),
                            after_before=paired_interval(pairs, seed)))
    return results


def plot(results, output, repeats, target):
    import matplotlib.pyplot as plt
    groups = sorted({(s['workers'], s['diagnostics']) for s in results})
    fig, axes = plt.subplots(1, len(groups), figsize=(5 * len(groups), 5), layout='constrained', squeeze=False)
    for axis, (workers, diagnostics) in zip(axes[0], groups):
        values = [s for s in results if (s['workers'], s['diagnostics']) == (workers, diagnostics)]
        for x, value in enumerate(values):
            ci = value['after_before']
            axis.errorbar(x, ci['median'], yerr=[[ci['median'] - ci['low']], [ci['high'] - ci['median']]], fmt='o', capsize=4)
        axis.axhline(1, color='grey', linestyle='--', label='Baseline')
        axis.axhline(1 / 3, color='#ad4c27', linestyle=':', label='3× faster')
        axis.legend(fontsize='small')
        axis.set_xticks(range(len(values)), [f"{s['shape']}\n{s['policy']}" for s in values])
        axis.set_ylim(bottom=0)
        axis.set_ylabel('After / before collection time (lower is better)')
        axis.set_title(f'{workers} workers; diagnostics {diagnostics}')
    fig.suptitle(f'{target}: complete graph collections; {repeats} process pairs, exploratory 95% intervals')
    for extension in ['svg', 'png']:
        fig.savefig(output / f'mark-cycles.{extension}', dpi=160)
    plt.close(fig)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--before-build', type=Path, required=True)
    parser.add_argument('--after-build', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--cpus', type=int, nargs='+', required=True)
    parser.add_argument('--workers', type=int, nargs='+', default=[0, 4])
    parser.add_argument('--shapes', nargs='+', choices=['tree', 'wide', 'barrier', 'mixed'], default=['tree', 'wide'])
    parser.add_argument('--repeats', type=int, default=7)
    parser.add_argument('--nodes', type=int, default=8191)
    parser.add_argument('--cycles', type=int, default=32)
    parser.add_argument('--seed', type=int, default=42)
    parser.add_argument('--timeout', type=int, default=120)
    parser.add_argument('--wasmtime', type=Path)
    parser.add_argument('--plot', action='store_true')
    args = parser.parse_args()
    if args.repeats < 2 or not 2 <= args.nodes <= 65535 or not 1 <= args.cycles <= 10000 or args.timeout <= 0:
        parser.error('invalid run size or timeout')
    if not set(args.cpus) <= os.sched_getaffinity(0) or len(set(args.cpus)) != len(args.cpus):
        parser.error('CPUs must be distinct and available')
    if len(set(args.workers)) != len(args.workers) or any(w < 0 or w > 16 or (w and w + 1 > len(args.cpus)) for w in args.workers):
        parser.error('each native worker and caller needs an available CPU; worker counts must be distinct')
    if len(set(args.shapes)) != len(args.shapes) or ('barrier' in args.shapes and args.workers != [0]):
        parser.error('shapes must be distinct; barrier requires --workers 0')
    builds = {label: json.loads(getattr(args, label + '_build').read_text()) for label in ['before', 'after']}
    validate_builds(builds)
    target = builds['before']['target']
    if (target == 'wasm') != bool(args.wasmtime) or (target == 'wasm' and args.workers != [0]):
        parser.error('Wasm requires Wasmtime and zero workers; native must omit Wasmtime')
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    binaries = {}
    for label, build in builds.items():
        path = output / label
        shutil.copy2(build['binary'], path)
        binaries[label] = dict(path=str(path), sha256=sha256(path), build=build)
    runner = None
    if args.wasmtime:
        path = output / 'wasmtime'
        shutil.copy2(args.wasmtime, path)
        version = subprocess.run([str(path), '--version'], capture_output=True, text=True, check=True).stdout.strip()
        runner = dict(path=str(path), sha256=sha256(path), version=version)
    conditions = [(s, p, w, d) for s in args.shapes for p in ['gated', 'counted']
                  for w in args.workers for d in [False, True]]
    rng, jobs = random.Random(args.seed), []
    for repeat in range(args.repeats):
        order = list(conditions)
        rng.shuffle(order)
        for condition in order:
            labels = ['before', 'after']
            rng.shuffle(labels)
            jobs.extend((repeat, *condition, label) for label in labels)
    metadata = dict(binaries=binaries, runner=runner, target=target, host=platform.platform(),
                    cpus=args.cpus, shapes=args.shapes, nodes=args.nodes, cycles=args.cycles, repeats=args.repeats,
                    seed=args.seed, qualification=False, argv0=INVOCATION_NAME,
                    harness_sha256=sha256(Path(__file__)),
                    scope='All full-collection phases and waits, plus buffered edge publication for barrier shape; mixed prepares unreachable scalars outside the clock; no construction/warmup/teardown; no application workload or per-operation latency')
    (output / 'metadata.json').write_text(json.dumps(metadata, indent=2) + '\n')
    (output / 'schedule.json').write_text(json.dumps(jobs) + '\n')
    rows = []
    with (output / 'raw.jsonl').open('w') as raw:
        for repeat, shape, policy, workers, diagnostics, label in jobs:
            cpus = args.cpus[:1] if workers == 0 else args.cpus[:workers + 1]
            command, executable = invocation(
                binaries[label]['path'],
                [shape, policy, str(args.nodes), str(args.cycles), str(workers), str(diagnostics).lower()],
                runner['path'] if runner else None)
            row = dict(repeat=repeat, shape=shape, policy=policy, workers=workers,
                       diagnostics=diagnostics, binary=label, command=command, cpus=cpus)
            row['executable'] = executable
            try:
                child = subprocess.run(command, executable=executable,
                                       capture_output=True, text=True, timeout=args.timeout,
                                       preexec_fn=lambda: os.sched_setaffinity(0, cpus))
                row.update(stdout=child.stdout, stderr=child.stderr, exit_code=child.returncode)
                child.check_returncode()
                measurement = json.loads(child.stdout)
                validate(measurement, shape, policy, args.nodes, args.cycles, workers, diagnostics)
                row.update(status='ok', measurement=measurement)
            except (subprocess.SubprocessError, ValueError, KeyError) as error:
                row.update(status='failed', error=str(error))
                if isinstance(error, subprocess.TimeoutExpired):
                    row.update(stdout=(error.stdout or b'').decode(errors='replace'), stderr=(error.stderr or b'').decode(errors='replace'))
            rows.append(row)
            raw.write(json.dumps(row) + '\n')
            raw.flush()
    unchanged = all(sha256(Path(v['path'])) == v['sha256'] for v in binaries.values())
    unchanged &= runner is None or sha256(Path(runner['path'])) == runner['sha256']
    try:
        if not unchanged:
            raise ValueError('frozen inputs changed')
        results = summarize(rows, conditions, args.repeats, args.seed)
    except ValueError as error:
        (output / 'summary.json').write_text(json.dumps(dict(results=[], error=str(error), inputs_unchanged=unchanged, qualification=False), indent=2) + '\n')
        return 1
    (output / 'summary.json').write_text(json.dumps(dict(results=results, inputs_unchanged=unchanged, qualification=False), indent=2) + '\n')
    lines = ['# Full-graph collection comparison', '',
             f'Target: {target}. {args.repeats} independent process pairs per condition; {args.nodes} nodes and {args.cycles} measured full collections per process.',
             f'Both programs receive argv[0]={INVOCATION_NAME!r}, independently of artifact paths. This controls invocation-name allocation; it does not establish insensitivity to other heap layouts.',
             'Includes every collection phase and its waits; barrier shape also publishes one required buffered edge per leaf per cycle (32 records/context, no overflow, zero workers). Excludes construction, three warmup cycles, checks and teardown. No application workload, allocation rate, STW pause or scheduled-arrival latency is measured.',
             'Ratios below one favor the candidate. Exploratory 95% intervals resample process pairs and are not adjusted for multiple comparisons. This is not release qualification.', '',
             '| Graph | Policy | Workers | Diagnostics | Before ns/node/cycle | After ns/node/cycle | Ratio [95% CI] |',
             '|---|---|---:|---|---:|---:|---:|']
    for s in results:
        ci = s['after_before']
        lines.append(f"| {s['shape']} | {s['policy']} | {s['workers']} | {s['diagnostics']} | {s['before_ns']:.1f} | {s['after_ns']:.1f} | {ci['median']:.3f} [{ci['low']:.3f}, {ci['high']:.3f}] |")
    lines += ['', f'All {len(rows)} processes validated; frozen binaries/runner unchanged: {unchanged}.',
              'Native multiworker contention is scheduler-dependent. This driver does not count fallback-queue publications and cannot attribute time specifically to the fallback path.']
    (output / 'REPORT.md').write_text('\n'.join(lines) + '\n')
    if args.plot:
        plot(results, output, args.repeats, target)
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
