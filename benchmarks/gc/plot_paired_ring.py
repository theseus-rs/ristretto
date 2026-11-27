"""Report paired persistent Java ring invocations from the original process captures."""
import argparse
import csv
from itertools import product
import json
from pathlib import Path
from statistics import median

from canonical_service import digest
from canonical_service_artifacts import read_json
from ring_artifacts import check_canonical, packet

TIERS = {'native-interpreted': 'Native interpreted', 'native-compiled': 'Native compiled',
         'wasi-interpreted': 'WASI interpreted'}


def require(condition, message):
    if not condition:
        raise ValueError(message)


def read_pairs(root):
    plan = read_json(root / 'plan.json')
    ready = read_json(root / 'ready.json')
    execution = read_json(root / 'execution.json')
    terminal = read_json(root / 'timing-execution.json')
    require(terminal['exit_code'] == 0 and execution['complete'] and execution['all_children_ended'],
            'paired execution did not finish')
    require(plan['qualification'] is False and plan['invocation_scope'] == 'shared_heap_invocation_v1',
            'unsupported timing scope')
    require(execution['plan_sha256'] == digest(root / 'plan.json')
            and execution['ready_sha256'] == digest(root / 'ready.json'), 'plan or readiness receipt changed')
    require(set(plan['variants']) == {'baseline', 'candidate'}
            and set(plan['runtimes']) <= TIERS.keys(), 'unexpected variant or execution tier')
    pairs = {}
    sources = set()
    blocks = {}
    captures = []
    for case, receipt in zip(plan['cases'], execution['cases'], strict=True):
        name = case['id']
        observed = root / 'runs' / name
        require(name == receipt['id'] and receipt['exit_code'] == 0 and not receipt['timeout'],
                f'{name} failed or differs from the plan')
        require(digest(observed / 'stdout.txt') == receipt['raw_packet_sha256'], f'{name} raw packet changed')
        checked = check_canonical(observed)
        sample_plan = read_json(observed / 'sample-plan.json')
        launch = read_json(observed / 'launch-plan.json')
        build = read_json(observed / 'build.json')
        source = ready['builds'][case['variant'] + '-' + case['family']]['source_manifest_sha256']
        require(source == plan['source_' + case['variant']]
                and checked['valid'] and checked['scope'] == plan['invocation_scope']
                and build['source_manifest_sha256'] == source
                and read_json(observed / 'run.json')['admission_sha256'] == case['admission_sha256']
                and sample_plan['mode'] == 'measure' and sample_plan['samples'] == plan['samples_per_child']
                and sample_plan['warmup_invocations'] == plan['warmup_invocations']
                and launch['resources'] == plan['resource_envelope']
                and launch['watchdog_seconds'] == plan['child_watchdog_seconds'], f'{name} capture scope differs')
        measured = packet(observed / 'stdout.txt', 'RISTRETTO_CANONICAL_RING ')
        times = [sample['elapsed_ns'] / 1e6 for sample in measured['samples']]
        require(len(times) == plan['samples_per_child'] and min(times) > 0, f'{name} lacks positive call timings')
        key = (case['runtime'], case['size'], case['fork'])
        require(blocks.setdefault(case['block'], key) == key, f'{name} shares a block with a different fork')
        row = pairs.setdefault(key, {})
        require(case['variant'] not in row, f'{name} duplicates a fork')
        row[case['variant']] = median(times)
        sources.add((case['variant'], source))
        usage = checked['process_resources']
        captures.append(dict(id=name, source=source, variant=case['variant'], fork=case['fork'],
                             runtime=case['runtime'], size=case['size'], median_ms=median(times),
                             calls=len(times), min_ms=min(times), max_ms=max(times),
                             cycle_lower=checked['primary_cycle_batch_bounds']['lower'],
                             cycle_upper=checked['primary_cycle_batch_bounds']['upper'],
                             peak_rss_mib=usage['peak_rss_bytes'] / 2**20,
                             cpu_seconds=usage['user_cpu_seconds'] + usage['system_cpu_seconds'],
                             final_heap_cleanup_ms=measured['heap_detach_cleanup_and_drop_ns'] / 1e6,
                             raw_sha256=receipt['raw_packet_sha256']))
    expected = set(product(plan['runtimes'], plan['sizes'], range(plan['forks'])))
    require(len(blocks) == len(expected) and set(pairs) == expected
            and all(set(row) == {'baseline', 'candidate'} for row in pairs.values()),
            'missing or unexpected matched forks')
    require(len(sources) == 2 and len(captures) == plan['children']
            and len({row['id'] for row in captures}) == len(captures), 'inconsistent sources or process identities')
    return plan, pairs, captures


def render(root, output):
    plan, pairs, captures = read_pairs(root)
    output.mkdir(parents=True, exist_ok=False)
    with (output / 'forks.csv').open('w') as stream:
        writer = csv.DictWriter(stream, fieldnames=list(captures[0]))
        writer.writeheader()
        writer.writerows(captures)
    import matplotlib
    matplotlib.use('Agg')
    import matplotlib.pyplot as plt

    groups = list(product(plan['runtimes'], plan['sizes']))
    figure, (time_axis, change_axis) = plt.subplots(1, 2, figsize=(11, max(4, len(groups)*.57)),
                                                   sharey=True, layout='constrained')
    lines = ['# Paired persistent-heap Java ring measurements', '', plan['purpose'].rstrip('.') + '.', '',
             f"Each process ran {plan['warmup_invocations']} warmup calls and {plan['samples_per_child']} measured calls. "
             f"Each size and tier has {plan['forks']} independent matched pairs. The time columns give the median of "
             'the per-process medians. The change column gives the median of the within-pair percentage changes.', '',
             '| Runtime | Size | Baseline (ms) | Candidate (ms) | Median change | Range of changes | Faster pairs |',
             '|---|---|---:|---:|---:|---:|---:|']
    group_values = []
    for y, (tier, size) in enumerate(groups):
        rows = [pairs[tier, size, fork] for fork in range(plan['forks'])]
        before = [row['baseline'] for row in rows]
        after = [row['candidate'] for row in rows]
        changes = [(b / a - 1)*100 for a, b in zip(before, after)]
        for index, (a, b) in enumerate(zip(before, after)):
            offset = (index - (len(rows)-1)/2) * .075
            time_axis.plot([a, b], [y+offset, y+offset], color='#a3acb0', linewidth=1)
            time_axis.scatter(a, y+offset, color='#156180', s=25, zorder=3,
                              label='Baseline' if y == index == 0 else None)
            time_axis.scatter(b, y+offset, color='#b04b22', marker='D', s=20, zorder=3,
                              label='Candidate' if y == index == 0 else None)
        change_axis.plot([min(changes), max(changes)], [y, y], color='#6a7479')
        change_axis.scatter(changes, [y]*len(rows), facecolors='white', edgecolors='#243e53', zorder=3)
        change_axis.scatter(median(changes), y, color='#b04b22', marker='D', zorder=4)
        faster = sum(change < 0 for change in changes)
        lines.append(f'| {TIERS[tier]} | {size} | {median(before):.3f} | {median(after):.3f} | '
                     f'{median(changes):+.2f}% | {min(changes):+.2f}% to {max(changes):+.2f}% | {faster}/{len(rows)} |')
        group_values.append(dict(tier=tier, size=size, median_change_percent=median(changes),
                                 changes_percent=changes, faster_pairs=faster))
    time_axis.set_yticks(range(len(groups)), [TIERS[tier]+' · '+size for tier, size in groups])
    time_axis.invert_yaxis()
    time_axis.legend(frameon=False)
    time_axis.set_xlabel('Per-process median invocation time (ms)')
    change_axis.axvline(0, color='#34444c', linewidth=1)
    change_axis.set_xlabel('Change in paired time (%) · negative is faster')
    for axis in (time_axis, change_axis):
        axis.grid(axis='x', alpha=.2)
    figure.suptitle('Persistent Java ring · paired process measurements')
    for ext in ('svg', 'png'):
        figure.savefig(output / ('paired-ring.' + ext), dpi=160, bbox_inches='tight')
    plt.close(figure)
    lines += ['', '![Matched process observations](paired-ring.png)', '',
              f"The {len(captures)} processes completed {sum(row['calls'] for row in captures):,} measured calls. "
              'Each process checked the Java result and the final heap drain. Setup and final drain are outside the '
              'invocation clocks. The per-process CSV includes batch collection bounds, final drain time, and whole-process CPU and peak RSS.',
              '', 'These closed-loop measurements cover the restricted Java ring. Sustainable throughput and scheduled '
              'tails require fixed-arrival trials. [Fork data](forks.csv) · [Machine-readable summary](summary.json).']
    (output / 'REPORT.md').write_text('\n'.join(lines)+'\n')
    summary = dict(qualification=False, plan_sha256=digest(root/'plan.json'),
                   execution_sha256=digest(root/'execution.json'), processes=len(captures),
                   samples=sum(row['calls'] for row in captures), groups=group_values,
                   renderer_sha256=digest(Path(__file__)), matplotlib=matplotlib.__version__)
    (output / 'summary.json').write_text(json.dumps(summary, indent=2)+'\n')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('directory', type=Path)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    render(args.directory.resolve(), args.output.resolve())
