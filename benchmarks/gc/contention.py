#!/usr/bin/env python3
"""Paired reclamation under barrier/root contention; unfinished cycles stay censored."""
import argparse
from collections import defaultdict
import json
import os
from pathlib import Path
import platform
import random
import shutil
import statistics
import subprocess
from cgroups import Budget
from mechanisms import paired_interval, sha256


def producer_operations(result):
    return result.get('producer_operations', result['barrier_operations'])


def validate_resources(resources):
    if any(resources['memory_events'].get(name, 0) for name in ['max', 'oom', 'oom_kill', 'oom_group_kill']):
        raise ValueError('process hit its memory ceiling')


def validate_fixture_hashes(binaries):
    hashes = [binary.get('fixture_sha256') for binary in binaries.values()]
    if len(hashes) != 2 or not hashes[0] or hashes[0] != hashes[1]:
        raise ValueError('both revisions must use the same contention fixture')


def validate(result, policy, kind, producers, cycles, targets=1, operation='barriers'):
    if result.get('operation', 'barriers') != operation:
        raise ValueError('producer operation differs')
    operations = producer_operations(result)
    if operation == 'roots' and 'producer_operations' not in result:
        raise ValueError('root workload needs its own operation counter')
    if result['barrier_operations'] != (operations if operation == 'barriers' else 0):
        raise ValueError('producer and barrier operation counts disagree')
    if result.get('targets', 1) != targets:
        raise ValueError('target working set differs')
    for name, expected in [('policy', policy), ('kind', kind), ('producers', producers),
                           ('cycles_requested', cycles), ('diagnostics', True), ('checksum_ok', True),
                           ('teardown_live', 0), ('teardown_bytes', 0)]:
        if result[name] != expected:
            raise ValueError(f'{name}: unexpected evidence')
    done = result['cycles_completed']
    if not 0 <= done <= cycles or len(result['cycle_ns']) != done:
        raise ValueError('inconsistent completion samples')
    if any(n <= 0 for n in result['cycle_ns']) or result['elapsed_ns'] <= 0:
        raise ValueError('invalid durations')
    unfinished = result['unfinished_cycle_ns']
    if (done == cycles) != (unfinished is None) or (unfinished is not None and unfinished <= 0):
        raise ValueError('unfinished cycle must remain censored')
    if sum(result['cycle_ns']) + (unfinished or 0) > result['elapsed_ns']:
        raise ValueError('cycle durations exceed measurement window')
    if result['allocated_objects'] != 16 * (done + int(unfinished is not None)):
        raise ValueError('allocation workload differs')
    if operations <= 0 or result['live_before_cleanup'] < targets:
        raise ValueError('missing producers or resident root')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--before', type=Path, required=True)
    parser.add_argument('--after', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--cpus', type=int, nargs='+', required=True)
    parser.add_argument('--cgroup-parent', type=Path, required=True)
    parser.add_argument('--memory-mib', type=int, default=256)
    parser.add_argument('--repeats', type=int, default=5)
    parser.add_argument('--producers', type=int, nargs='+', default=[1, 4])
    parser.add_argument('--cycles', type=int, default=64)
    parser.add_argument('--targets', type=int, choices=[1, 2, 4, 8, 16, 32, 64, 128, 256], default=1)
    parser.add_argument('--seconds', type=int, default=5)
    parser.add_argument('--operation', choices=['barriers', 'roots'], default='barriers')
    parser.add_argument('--seed', type=int, default=42)
    parser.add_argument('--plot', action='store_true')
    args = parser.parse_args()
    if args.repeats < 2 or not set(args.cpus) <= os.sched_getaffinity(0):
        parser.error('need independent pairs and available CPUs')
    if not all(1 <= p <= 32 for p in args.producers) or not 1 <= args.cycles <= 10000 or not 1 <= args.seconds <= 3600:
        parser.error('invalid workload bounds')
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    analysis = output / 'analysis_sources'
    analysis.mkdir()
    for name in ['contention.py', 'cgroups.py', 'mechanisms.py']:
        shutil.copy2(Path(__file__).with_name(name), analysis / name)
    binaries = {}
    for label in ['before', 'after']:
        original = getattr(args, label).resolve()
        provenance = original.with_name(original.name + '.provenance.json')
        archive = original.parent / 'sources.tar.gz'
        build = json.loads(provenance.read_text())
        if build['binary_sha256'] != sha256(original) or build['source_archive_sha256'] != sha256(archive):
            parser.error('binary/source provenance mismatch')
        directory = output / label
        directory.mkdir()
        path = directory / original.name
        for source, name in [(original, path.name), (provenance, 'build.json'), (archive, 'sources.tar.gz')]:
            shutil.copy2(source, directory / name)
        binaries[label] = {'path': str(path), 'sha256': sha256(path), 'source_sha256': sha256(directory / 'sources.tar.gz'),
                          'fixture_sha256': build['source_sha256'].get('ristretto_gc/examples/reclamation_contention.rs')}
    validate_fixture_hashes(binaries)
    jobs = []
    rng = random.Random(args.seed)
    for repeat in range(args.repeats):
        conditions = [(p, k, n) for p in ['baseline', 'counted', 'candidate'] for k in ['full', 'young'] for n in args.producers]
        rng.shuffle(conditions)
        for policy, kind, producers in conditions:
            labels = list(binaries)
            rng.shuffle(labels)
            jobs.extend((repeat, policy, kind, producers, label) for label in labels)
    metadata = {'binaries': binaries, 'host': platform.platform(), 'cpus': args.cpus,
        'memory_mib': args.memory_mib, 'cpu_quota_us': len(set(args.cpus)) * 100000,
        'repeats': args.repeats, 'cycles': args.cycles, 'targets': args.targets, 'seconds': args.seconds, 'seed': args.seed,
        'operation': args.operation, 'analysis_sha256': {p.name: sha256(p) for p in analysis.iterdir()},
        'diagnostics': True, 'qualification': False, 'scope': 'fixed cycle count under saturated producers; not a scheduled-arrival latency test'}
    (output / 'metadata.json').write_text(json.dumps(metadata, indent=2) + '\n')
    (output / 'schedule.json').write_text(json.dumps(jobs) + '\n')
    groups = defaultdict(dict)
    failures = []
    with (output / 'raw.jsonl').open('w') as raw:
        for repeat, policy, kind, producers, label in jobs:
            command = [binaries[label]['path'], policy, kind, str(producers), str(args.cycles), str(args.seconds), str(args.targets), args.operation]
            row = {'repeat': repeat, 'policy': policy, 'kind': kind, 'operation': args.operation, 'producers': producers, 'binary': label, 'command': command}
            budget = Budget(args.cgroup_parent, args.memory_mib * 1024 * 1024, metadata['cpu_quota_us'])
            def prepare():
                os.sched_setaffinity(0, args.cpus)
                budget.join()
            try:
                child = subprocess.run(command, capture_output=True, text=True, timeout=args.seconds + 30, preexec_fn=prepare)
                row.update(stdout=child.stdout, stderr=child.stderr, exit_code=child.returncode)
                if child.returncode:
                    raise ValueError('process failed')
                result = json.loads(child.stdout)
                validate(result, policy, kind, producers, args.cycles, args.targets, args.operation)
                row.update(status='ok', result=result)
            except (subprocess.TimeoutExpired, ValueError, TypeError, KeyError) as error:
                row.update(status='failed', error=str(error))
                failures.append([repeat, policy, kind, producers, label])
            finally:
                row['resources'] = budget.snapshot()
                budget.close()
            if row['status'] == 'ok':
                try:
                    validate_resources(row['resources'])
                    groups[policy, kind, producers].setdefault(repeat, {})[label] = result
                except ValueError as error:
                    row.update(status='failed', error=str(error))
                    failures.append([repeat, policy, kind, producers, label])
            raw.write(json.dumps(row) + '\n')
            raw.flush()
    unchanged = all(sha256(Path(b['path'])) == b['sha256'] and sha256(Path(b['path']).parent / 'sources.tar.gz') == b['source_sha256'] for b in binaries.values())
    summary = []
    lines = [f'# Reclamation under {args.operation} contention', '',
             f'Working set: {args.targets} resident targets shared by all producers.', '',
        'Fresh process pairs, fixed cycle count, diagnostics enabled. Incomplete runs retain the last cycle as censored; their run durations are lower bounds on completion time. No ratio excludes unfinished runs. Producers saturate the CPU, so faster operations increase the offered contention; this is not a comparison at a matched arrival rate. Root operations acquire and release a fresh registration from a heap edge protected by a separate resident root. These screens do not qualify sustained load or G1/ZGC performance.', '',
        '| Policy | Generation | Producers | Completed runs before/after | Median seconds before/after | Producer Mops/s before/after | Completion-time ratio [95% CI] |',
        '|---|---|---:|---:|---:|---:|---|']
    for (policy, kind, producers), repeats in sorted(groups.items()):
        if len(repeats) != args.repeats or any(set(pair) != {'before', 'after'} for pair in repeats.values()):
            continue
        row = {'policy': policy, 'kind': kind, 'producers': producers}
        for label in ['before', 'after']:
            values = [pair[label] for pair in repeats.values()]
            row[label] = {'completed_runs': sum(r['cycles_completed'] == args.cycles for r in values),
                          'median_seconds': statistics.median(r['elapsed_ns'] / 1e9 for r in values),
                          'median_producer_mops': statistics.median(producer_operations(r) * 1000 / r['elapsed_ns'] for r in values)}
            if args.operation == 'barriers':
                row[label]['median_barrier_mops'] = row[label]['median_producer_mops']
        all_complete = all(row[label]['completed_runs'] == args.repeats for label in ['before', 'after'])
        row['after_before'] = paired_interval([(pair['before']['elapsed_ns'], pair['after']['elapsed_ns']) for pair in repeats.values()], args.seed) if all_complete else None
        ci = row['after_before']
        interval = f"{ci['median']:.3f} [{ci['low']:.3f}, {ci['high']:.3f}]" if ci else 'censored; no ratio'
        lines.append(f"| {policy} | {kind} | {producers} | {row['before']['completed_runs']}/{row['after']['completed_runs']} of {args.repeats} | {row['before']['median_seconds']:.4f}/{row['after']['median_seconds']:.4f} | {row['before']['median_producer_mops']:.3f}/{row['after']['median_producer_mops']:.3f} | {interval} |")
        summary.append(row)
    lines += ['', f'Failed processes: {len(failures)}. Frozen inputs unchanged: {unchanged}. Qualification: false.']
    (output / 'REPORT.md').write_text('\n'.join(lines) + '\n')
    (output / 'summary.json').write_text(json.dumps({'results': summary, 'failures': failures, 'inputs_unchanged': unchanged, 'qualification': False}, indent=2) + '\n')
    if args.plot:
        import matplotlib.pyplot as plt
        fig, axes = plt.subplots(1, 2, figsize=(12, 5), layout='constrained')
        for axis, kind in zip(axes, ['full', 'young']):
            rows = [r for r in summary if r['kind'] == kind]
            for i, row in enumerate(rows):
                for label, offset, color in [('before', -.12, '#64748b'), ('after', .12, '#0284c7')]:
                    value = row[label]
                    complete = value['completed_runs'] == args.repeats
                    axis.scatter(i + offset, value['median_seconds'], color=color, marker='o' if complete else '^', label=label if i == 0 else None)
                    if not complete:
                        axis.annotate(f"{value['completed_runs']}/{args.repeats}", (i + offset, value['median_seconds']), xytext=(0, 7), textcoords='offset points', ha='center', fontsize=8)
            axis.set_xticks(range(len(rows)), [f"{r['policy']}\n{r['producers']} producers" for r in rows], rotation=30)
            axis.set_yscale('log')
            axis.set_ylabel('Median run seconds (triangles include censored runs)')
            axis.set_title(kind)
            axis.legend()
        fig.suptitle(f"{args.operation}: {args.cycles} cycles, {args.targets} shared targets; {args.repeats} process pairs; triangle labels = complete runs")
        for suffix in ['svg', 'png']:
            fig.savefig(output / f'contention.{suffix}', dpi=160)
        plt.close(fig)
        (output / 'graph-provenance.json').write_text(json.dumps({name: sha256(output / name) for name in ['raw.jsonl', 'metadata.json', 'summary.json', 'contention.svg', 'contention.png']}, indent=2) + '\n')
    return int(bool(failures) or not unchanged)


if __name__ == '__main__':
    raise SystemExit(main())
