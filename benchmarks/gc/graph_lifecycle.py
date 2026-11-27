#!/usr/bin/env python3
"""Frozen cyclic-graph cost screen. Not a JVM, pause, or endurance qualification."""
import argparse
import csv
import json
import os
from pathlib import Path
import platform
import random
import shutil
import signal
import statistics
import subprocess
import time

from leaf_lifecycle import digest, packet

ARGV0 = 'graph_lifecycle'
TIMES = ['setup_ns', 'allocation_ns', 'mutation_ns', 'read_ns', 'cleanup_ns', 'lifecycle_ns', 'teardown_ns']
FIELDS = {'version', 'mode', 'rounds', 'capacity', 'degree', 'objects', 'payload_bytes',
          'logical_object_bytes', 'catalog_bytes', 'poll_limit', *TIMES, 'allocation_polls',
          'write_polls', 'read_polls', 'helper_polls', 'released_objects', 'released_roots',
          'released_edges', 'drops', 'checksum', 'teardown_bytes', 'teardown_catalog_bytes'}
COUNT_FIELDS = {'version', 'mode', 'rounds', 'capacity', 'sample', 'allocations', 'allocated_bytes',
                'releases', 'released_bytes', 'live_before', 'live_after', 'peak_above_baseline'}


def identity(row, fields, rounds, capacity):
    if not isinstance(row, dict) or set(row) != fields:
        raise ValueError('unexpected graph schema')
    if any(type(v) is not int or v < 0 for k, v in row.items() if k != 'mode'):
        raise ValueError('graph measurements must be nonnegative integers')
    if (row['version'], row['mode'], row['rounds'], row['capacity']) != (1, 'canonical-graph', rounds, capacity):
        raise ValueError('graph workload identity mismatch')


def validate(row, rounds, capacity):
    identity(row, FIELDS, rounds, capacity)
    expected = dict(degree=2, objects=2*rounds, payload_bytes=24, poll_limit=max(1000000, capacity*2048),
                    released_objects=2*rounds, released_roots=3*rounds, released_edges=4*rounds,
                    drops=2*rounds, checksum=rounds*rounds, teardown_bytes=0, teardown_catalog_bytes=0)
    if any(row[k] != value for k, value in expected.items()):
        raise ValueError('graph integrity, release, watchdog or teardown mismatch')
    if row['logical_object_bytes'] < 24 or not row['catalog_bytes']:
        raise ValueError('missing graph storage reservation')
    if any(row[k] <= 0 for k in TIMES) or row['lifecycle_ns'] < sum(row[k] for k in ['allocation_ns', 'mutation_ns', 'read_ns', 'cleanup_ns']):
        raise ValueError('invalid graph timing boundary')
    if (row['allocation_polls'] < 2*rounds or row['write_polls'] < 2*rounds
            or row['read_polls'] < rounds or row['helper_polls'] < 9*rounds):
        raise ValueError('missing graph operation or maintenance work')
    return row


def validate_counts(rows, rounds, capacity):
    if len(rows) != 3:
        raise ValueError('expected three complete allocator samples')
    for index, row in enumerate(rows):
        identity(row, COUNT_FIELDS, rounds, capacity)
        if row['sample'] != index or not row['allocations'] or row['allocated_bytes'] < 48:
            raise ValueError('missing allocator sample or setup')
        if row['allocations'] != row['releases'] or row['allocated_bytes'] != row['released_bytes'] or row['live_before'] != row['live_after']:
            raise ValueError('graph allocator lifetime did not balance')
        if row['peak_above_baseline'] < 48 or (index and row['live_before'] != rows[index-1]['live_after']):
            raise ValueError('inconsistent allocator sample boundaries')
    return rows


def invocation(binary, rounds, capacity, runner=None):
    args = [str(rounds), str(capacity)]
    if runner:
        return [str(runner), 'run', '-W', 'max-wasm-stack=8388608', '--argv0', ARGV0, str(binary), *args], None
    return [ARGV0, *args], str(binary)


def make_schedule(capacities, repeats, kind, seed):
    rng = random.Random(seed); schedule = []
    for repeat in range(repeats if kind == 'timing' else 1):
        order = capacities.copy(); rng.shuffle(order)
        for capacity in order:
            schedule.append(dict(index=len(schedule), repeat=repeat, capacity=capacity))
    return schedule


def run(args):
    if args.cpu not in os.sched_getaffinity(0) or args.repeats < 2 or args.timeout <= 0:
        raise ValueError('need an available CPU, positive timeout and at least two repeats')
    if not 1 <= args.rounds <= 1000000 or not args.capacities or len(set(args.capacities)) != len(args.capacities) or any(not 2 <= n <= 16384 for n in args.capacities):
        raise ValueError('invalid graph workload bounds')
    with args.binary.open('rb') as stream:
        magic = stream.read(4)
    if magic not in (b'\x7fELF', b'\0asm') or (magic == b'\0asm') != bool(args.wasmtime):
        raise ValueError('binary target and runner mismatch')
    provenance = json.loads(args.provenance.read_text())
    if provenance['binary_sha256'] != digest(args.binary) or provenance['source_archive_sha256'] != digest(args.sources):
        raise ValueError('binary/source build provenance mismatch')
    out = args.output.resolve(); out.mkdir(parents=True, exist_ok=False)
    inputs = {}
    for name, source in [('binary', args.binary), ('build.json', args.provenance), ('sources.tar.gz', args.sources),
                         ('graph_lifecycle.py', Path(__file__)), ('leaf_lifecycle.py', Path(__file__).with_name('leaf_lifecycle.py')), ('wasmtime', args.wasmtime)]:
        if source:
            destination = out/name; shutil.copy2(source, destination)
            inputs[name] = dict(path=str(destination), sha256=digest(destination))
    runner = out/'wasmtime' if args.wasmtime else None
    metadata = dict(version=1, target='WASI' if runner else 'native', kind=args.kind, qualification=False,
                    rounds=args.rounds, capacities=args.capacities, repeats=args.repeats, cpu=args.cpu,
                    seed=args.seed, label=args.label, inputs=inputs, argv0=ARGV0, host=platform.platform(),
                    clock='in-process/guest monotonic wall time', warmup='fresh process for timings; one full warmup before three allocator samples',
                    scope='two real cyclic nodes per round, two writes, one independently rooted child read, full reclamation; one mutator/helper; no host-turn or pause claim',
                    counting_scope='whole warmed invocation including setup/teardown/report String; no stdout; requested allocator bytes, not RSS')
    for name in ('scaling_governor', 'scaling_min_freq', 'scaling_max_freq'):
        p = Path(f'/sys/devices/system/cpu/cpu{args.cpu}/cpufreq/{name}')
        metadata[name] = p.read_text().strip() if p.exists() else None
    if runner:
        metadata['wasmtime_version'] = subprocess.check_output([str(runner), '--version'], text=True).strip()
    schedule = make_schedule(args.capacities, args.repeats, args.kind, args.seed)
    (out/'metadata.json').write_text(json.dumps(metadata, indent=2)+'\n')
    (out/'schedule.json').write_text(json.dumps(schedule, indent=2)+'\n')
    failures = []
    with (out/'raw.jsonl').open('w') as stream:
        for case in schedule:
            command, executable = invocation(out/'binary', args.rounds, case['capacity'], runner)
            start = time.monotonic()
            proc = subprocess.Popen(command, executable=executable, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                                    text=True, start_new_session=True, preexec_fn=lambda: os.sched_setaffinity(0, {args.cpu}))
            try:
                stdout, stderr = proc.communicate(timeout=args.timeout); timed_out = False
            except subprocess.TimeoutExpired:
                os.killpg(proc.pid, signal.SIGKILL); stdout, stderr = proc.communicate(); timed_out = True
            row = dict(**case, command=command, executable=executable, exit_code=proc.returncode, timeout=timed_out,
                       elapsed_seconds=time.monotonic()-start, stdout=stdout, stderr=stderr, valid=False)
            try:
                if timed_out or proc.returncode != 0:
                    raise ValueError('failed or timed-out process')
                row['measurement'] = (validate(packet(stdout), args.rounds, case['capacity']) if args.kind == 'timing'
                                      else validate_counts([packet(line) for line in stdout.splitlines()], args.rounds, case['capacity']))
                row['valid'] = True
            except (ValueError, TypeError, KeyError) as error:
                row['error'] = str(error); failures.append(case['index'])
            stream.write(json.dumps(row)+'\n'); stream.flush()
            print(case['index'], case['capacity'], row['valid'], flush=True)
    unchanged = all(digest(Path(r['path'])) == r['sha256'] for r in inputs.values())
    result = dict(processes=len(schedule), failures=failures, inputs_unchanged=unchanged, qualification=False,
                  files={name:digest(out/name) for name in ['metadata.json', 'schedule.json', 'raw.jsonl']})
    (out/'result.json').write_text(json.dumps(result, indent=2)+'\n')
    if failures or not unchanged:
        raise ValueError('graph screen failed; all process records retained')


def checked(directory):
    result = packet((directory/'result.json').read_text())
    if result['failures'] or result['inputs_unchanged'] is not True or result['qualification'] is not False:
        raise ValueError('failed graph screen result')
    if set(result['files']) != {'metadata.json', 'schedule.json', 'raw.jsonl'} or any(digest(directory/n) != h for n,h in result['files'].items()):
        raise ValueError('graph run records changed')
    metadata = packet((directory/'metadata.json').read_text())
    if metadata['version'] != 1 or metadata['qualification'] is not False or metadata['kind'] not in ('timing','counts'):
        raise ValueError('invalid graph screen metadata')
    if not all(digest(Path(r['path'])) == r['sha256'] for r in metadata['inputs'].values()):
        raise ValueError('frozen graph input changed')
    required = {'binary','build.json','sources.tar.gz','graph_lifecycle.py','leaf_lifecycle.py'}
    if set(metadata['inputs']) not in (required, required|{'wasmtime'}):
        raise ValueError('missing or substituted graph input')
    with Path(metadata['inputs']['binary']['path']).open('rb') as stream:
        magic = stream.read(4)
    expected_target = 'WASI' if 'wasmtime' in metadata['inputs'] else 'native'
    if metadata['target'] != expected_target or magic != (b'\0asm' if expected_target=='WASI' else b'\x7fELF'):
        raise ValueError('graph target differs from frozen binary')
    if (type(metadata['rounds']) is not int or not 1 <= metadata['rounds'] <= 1000000
            or type(metadata['repeats']) is not int or metadata['repeats'] < 2
            or not isinstance(metadata['capacities'], list) or not metadata['capacities']
            or any(type(n) is not int or not 2 <= n <= 16384 for n in metadata['capacities'])
            or len(set(metadata['capacities'])) != len(metadata['capacities'])):
        raise ValueError('invalid frozen graph workload bounds')
    schedule = packet((directory/'schedule.json').read_text())
    if schedule != make_schedule(metadata['capacities'], metadata['repeats'], metadata['kind'], metadata['seed']):
        raise ValueError('graph schedule differs from frozen seed')
    expected_count = len(metadata['capacities']) * (metadata['repeats'] if metadata['kind'] == 'timing' else 1)
    rows = [packet(line) for line in (directory/'raw.jsonl').read_text().splitlines()]
    if len(schedule) != expected_count or len(rows) != expected_count or result['processes'] != expected_count:
        raise ValueError('incomplete graph schedule')
    build = json.loads(Path(metadata['inputs']['build.json']['path']).read_text())
    if build['binary_sha256'] != metadata['inputs']['binary']['sha256'] or build['source_archive_sha256'] != metadata['inputs']['sources.tar.gz']['sha256']:
        raise ValueError('frozen graph build provenance differs')
    expected_pairs = {(repeat, cap) for repeat in range(metadata['repeats'] if metadata['kind'] == 'timing' else 1) for cap in metadata['capacities']}
    if {(r['repeat'], r['capacity']) for r in schedule} != expected_pairs or any(r['index'] != i for i, r in enumerate(schedule)):
        raise ValueError('duplicate or substituted graph schedule')
    for case, row in zip(schedule, rows, strict=True):
        if any(row[k] != v for k, v in case.items()) or type(row['exit_code']) is not int or row['exit_code'] != 0 or row['timeout'] is not False or row['valid'] is not True:
            raise ValueError('failed or substituted graph process')
        runner = Path(metadata['inputs']['wasmtime']['path']) if 'wasmtime' in metadata['inputs'] else None
        command, executable = invocation(Path(metadata['inputs']['binary']['path']), metadata['rounds'], case['capacity'], runner)
        if row['command'] != command or row['executable'] != executable:
            raise ValueError('graph process invocation differs')
        measurement = (validate(packet(row['stdout']), metadata['rounds'], case['capacity']) if metadata['kind'] == 'timing'
                       else validate_counts([packet(line) for line in row['stdout'].splitlines()], metadata['rounds'], case['capacity']))
        if measurement != row['measurement']:
            raise ValueError('graph parsed measurement differs from raw output')
    return metadata, rows


def report(args):
    inputs = [checked(p.resolve()) for p in args.inputs]
    out = args.output.resolve(); out.mkdir(parents=True, exist_ok=False)
    summaries = []
    for metadata, rows in inputs:
        for capacity in metadata['capacities']:
            selected = [r['measurement'] for r in rows if r['capacity'] == capacity]
            summary = dict(label=metadata['label'], target=metadata['target'], kind=metadata['kind'], capacity=capacity, rounds=metadata['rounds'], processes=len(selected))
            if metadata['kind'] == 'timing':
                summary.update({k:statistics.median(r[k] for r in selected) for k in TIMES})
                summary['ns_per_object'] = summary['lifecycle_ns']/(2*metadata['rounds'])
                summary['catalog_bytes'] = selected[0]['catalog_bytes']
            else:
                samples = [r for group in selected for r in group]
                summary.update({k:statistics.median(r[k] for r in samples) for k in ['allocations', 'allocated_bytes', 'peak_above_baseline']})
            summaries.append(summary)
    (out/'summary.json').write_text(json.dumps(dict(qualification=False, rows=summaries), indent=2)+'\n')
    fields = list(dict.fromkeys(k for r in summaries for k in r))
    with (out/'summary.csv').open('w') as stream:
        writer = csv.DictWriter(stream, fieldnames=fields); writer.writeheader(); writer.writerows(summaries)
    if args.plot:
        import matplotlib
        matplotlib.use('Agg')
        import matplotlib.pyplot as plt
        timing = [r for r in summaries if r['kind'] == 'timing']
        if not timing:
            raise ValueError('timing inputs required for plots')
        fig, ax = plt.subplots(figsize=(8,5), layout='constrained')
        for label, target in sorted({(r['label'], r['target']) for r in timing}):
            rows = sorted((r for r in timing if (r['label'],r['target']) == (label,target)), key=lambda r:r['capacity'])
            ax.plot([r['capacity'] for r in rows], [r['ns_per_object']/1000 for r in rows], marker='o', label=f'{label} · {target}')
        ax.set(xscale='log', yscale='log', xlabel='Object-slot capacity (two live nodes)', ylabel='Median lifecycle µs / object', title='Controlled cyclic graph cost · not pause or JVM qualification')
        ax.legend(); ax.grid(alpha=.2)
        for ext in ['svg','png']: fig.savefig(out/('lifecycle.'+ext), dpi=160)
        plt.close(fig)


def main():
    parser = argparse.ArgumentParser(description=__doc__); sub = parser.add_subparsers(dest='action', required=True)
    p = sub.add_parser('run')
    for flag in ['binary','provenance','sources','output']: p.add_argument('--'+flag, type=Path, required=True)
    p.add_argument('--wasmtime', type=Path); p.add_argument('--kind', choices=['timing','counts'], default='timing')
    p.add_argument('--label', default='canonical graph'); p.add_argument('--rounds', type=int, default=1000)
    p.add_argument('--capacities', nargs='+', type=int, default=[2,8,64,256]); p.add_argument('--repeats', type=int, default=7)
    p.add_argument('--cpu', type=int, default=0); p.add_argument('--seed', type=int, default=20261115); p.add_argument('--timeout', type=int, default=600)
    p = sub.add_parser('report'); p.add_argument('--inputs', nargs='+', type=Path, required=True); p.add_argument('--output', type=Path, required=True); p.add_argument('--plot', action='store_true')
    args = parser.parse_args(); run(args) if args.action == 'run' else report(args)


if __name__ == '__main__':
    main()
