"""Rebuild the exact-address benchmark figures from the adjacent CSV files."""
import csv
import statistics
from pathlib import Path

import matplotlib.pyplot as plt
from matplotlib.ticker import FuncFormatter

HERE = Path(__file__).resolve().parent
LANES = {
    'ristretto-candidate': 'Native interpreter',
    'ristretto-candidate-jit': 'Native, JIT enabled',
    'wasm-candidate': 'WASI interpreter',
    'g1': 'HotSpot G1',
    'zgc': 'HotSpot ZGC',
}
COLORS = ['#254f75', '#4e8098', '#56a3a6', '#94521e', '#b47c27']


def read(name):
    with (HERE / name).open(newline='') as stream:
        return list(csv.DictReader(stream))


def save(fig, name):
    path = HERE / name
    fig.savefig(path, metadata={'Date': None})
    # Matplotlib emits trailing spaces in some SVG paths. Keep the export diffable.
    path.write_text('\n'.join(line.rstrip() for line in path.read_text().splitlines()) + '\n')
    plt.close(fig)


def whole_vm(rows):
    for column, rate, title, unit, name in [
        ('completed_ops_s', '0', 'Saturation throughput', 'Operations per second', 'throughput.svg'),
        ('arrival_p999_us', '1000', 'Scheduled-arrival p99.9 at 1,000 operations/s', 'Microseconds', 'latency.svg'),
    ]:
        fig, axes = plt.subplots(1, 2, figsize=(11, 4.2), sharey=True, layout='constrained')
        for ax, workload, label in zip(axes, ['allocation', 'roots'], ['Byte arrays', 'Rooted 16-node graphs']):
            for i, (runtime, color) in enumerate(zip(LANES, COLORS)):
                values = [float(row[column]) for row in rows if row['workload'] == workload
                          and row['runtime'] == runtime and row['rate'] == rate and row['status'] == 'ok']
                if len(values) != 3:
                    raise ValueError(f'Expected three completed forks for {workload} {runtime} {rate}')
                ax.scatter(values, [i] * len(values), color=color, alpha=.4, s=28)
                ax.scatter(statistics.median(values), i, color=color, marker='|', s=200)
            ax.set_xscale('log')
            ax.grid(axis='x', alpha=.25)
            ax.set_yticks(range(len(LANES)), LANES.values())
            ax.set_xlabel(unit + ' · log scale')
            ax.set_title(label)
        axes[0].invert_yaxis()
        fig.suptitle(title + '\nThree process measurements per lane · vertical tick marks the median')
        save(fig, name)


def collector(rows):
    fig, axes = plt.subplots(1, 2, figsize=(10.4, 4.6), sharey=True, layout='constrained')
    for ax, target, label in zip(axes, ['native', 'wasm'], ['Native', 'WASI']):
        for index, (kind, shape) in enumerate((c, s) for c in ['full', 'young'] for s in ['scalar', 'graph']):
            for variant, offset, color in [('narrow', -.10, COLORS[3]), ('wide', .10, COLORS[0])]:
                row, = [r for r in rows if (r['target'], r['collection'], r['shape'], r['page_layout'])
                        == (target, kind, shape, variant)]
                mid, low, high = (float(row[k]) for k in ['median', 'low', 'high'])
                ax.errorbar(mid, index + offset, xerr=[[mid - low], [high - mid]], marker='o',
                            capsize=3, color=color, label=variant if index == 0 else None)
        ax.axvline(1, color='gray', linestyle='--')
        ax.set_yticks(range(4), ['Full · scalar', 'Full · graph', 'Young · scalar', 'Young · graph'])
        ax.set_title(label)
        ax.xaxis.set_major_formatter(FuncFormatter(lambda x, _: f'{x:g}×'))
        ax.set_xlabel('Elapsed time / shared baseline')
        ax.legend()
    axes[0].invert_yaxis()
    fig.suptitle('Allocation during marking plus remaining collection\n16 process pairs per point · paired 95% bootstrap intervals')
    save(fig, 'collector.svg')


if __name__ == '__main__':
    plt.rcParams.update({'svg.fonttype': 'none', 'svg.hashsalt': 'registered-pages-7acdabb5'})
    whole_vm(read('processes.csv'))
    collector(read('marking-ratios.csv'))
