#!/usr/bin/env python3
"""Actual Chromium host-turn trials with fixed arrivals and cooperative bare Wasm."""
import argparse
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import json
import math
import platform
import os
from pathlib import Path
import random
import secrets
import shutil
import signal
import statistics
import subprocess
import sys
import tempfile
import threading
import time
from urllib.parse import parse_qs, urlencode, urlparse

from cgroups import Budget
from run import digest, capture

ASSETS = Path(__file__).resolve().parent / 'browser'


def validate(result, options):
    if result.get('schema') != 1 or result.get('options') != options:
        raise ValueError('browser result identity mismatch')
    if not result.get('cross_origin_isolated'):
        raise ValueError('isolated browser clock context was not established')
    for name in ['warmup', 'measurement']:
        phase = result[name]
        expected = int(options['rate'] * options['warmup' if name == 'warmup' else 'seconds'])
        if (phase['offered'] != expected or not 0 <= phase['completed'] <= expected
                or phase['unfinished'] != expected - phase['completed']
                or phase['peak_backlog'] < phase['unfinished']):
            raise ValueError(f'inconsistent {name} completions/backlog')
        required = {'arrival', 'service', 'poll', 'turn_work', 'request_turn_work',
                    'turn_gap', 'heartbeat_gap', 'frame_gap', 'long_task'}
        if set(phase['histograms']) != required:
            raise ValueError('missing browser observation channel')
        for key, histogram in phase['histograms'].items():
            buckets = histogram['buckets_us']
            if (any(type(n) is not int or n <= 0 or type(us) is not int or us < 0 for us, n in buckets)
                    or any(a[0] >= b[0] for a, b in zip(buckets, buckets[1:]))
                    or sum(n for _, n in buckets) != histogram['count']
                    or type(histogram['count']) is not int or type(histogram['max_ns']) is not int
                    or histogram['max_ns'] < 0
                    or (buckets and histogram['max_ns'] > buckets[-1][0] * 1000)
                    or (buckets and bool(buckets[-1][0]) != bool(histogram['max_ns']))):
                raise ValueError(f'invalid {name}/{key} histogram')
            if key in ['arrival', 'service'] and histogram['count'] != phase['completed']:
                raise ValueError('completed-request histogram lost samples')
        if not phase['turns'] or phase['histograms']['turn_work']['count'] != phase['turns']:
            raise ValueError('host-turn samples missing')
        polls = phase['histograms']['poll']['count']
        request_turns = phase['histograms']['request_turn_work']['count']
        if (polls < phase['completed'] or not 0 <= request_turns <= min(polls, phase['turns'])
                or bool(polls) != bool(request_turns)
                or polls > request_turns * options['poll_budget']):
            raise ValueError('inconsistent request-bearing turn samples')
        for point in phase['series']:
            if point['backlog'] != point['offered'] - point['completed'] or point['backlog'] < 0:
                raise ValueError('inconsistent browser time series')
    if result['warmup']['unfinished']:
        raise ValueError('warmup did not drain')
    final = result['after_teardown']
    completed = result['warmup']['completed'] + result['measurement']['completed']
    if (final['completed'] != completed or final['checksum'] != 42 * completed
            or final['finalizations'] != int(options['policy'] != 'control')
            or final['finished'] != 1
            or any(final[key] for key in ['errors', 'live_objects', 'managed_bytes', 'retired_bytes',
                                         'root_pages', 'pending_barriers', 'pending_request'])):
        raise ValueError('content, finalization or cooperative teardown failed')
    if result['final_memory_bytes'] < result['initial_memory_bytes']:
        raise ValueError('invalid Wasm linear-memory observation')


def quantile(histogram, fraction):
    if histogram['count'] == 0:
        return None
    rank = max(1, math.ceil(fraction * histogram['count']))
    for upper, count in histogram['buckets_us']:
        rank -= count
        if rank <= 0:
            return upper
    raise ValueError('histogram count mismatch')


def trial(args, options, directory, frozen):
    token = secrets.token_hex(24)
    received = threading.Event()
    response = {}
    files = {'/': (frozen / 'index.html', 'text/html'),
             '/browser.js': (frozen / 'browser.js', 'text/javascript'),
             '/workload.wasm': (frozen / 'workload.wasm', 'application/wasm')}

    class Handler(BaseHTTPRequestHandler):
        def log_message(self, *_):
            pass

        def do_GET(self):
            item = files.get(urlparse(self.path).path)
            if item is None:
                self.send_error(404)
                return
            data = item[0].read_bytes()
            self.send_response(200)
            for key, value in [('Content-Type', item[1]), ('Content-Length', str(len(data))),
                    ('Cross-Origin-Opener-Policy', 'same-origin'),
                    ('Cross-Origin-Embedder-Policy', 'require-corp'), ('Cache-Control', 'no-store'),
                    ('Content-Security-Policy', "default-src 'self'; script-src 'self' 'wasm-unsafe-eval'")]:
                self.send_header(key, value)
            self.end_headers()
            self.wfile.write(data)

        def do_POST(self):
            parsed = urlparse(self.path)
            size = int(self.headers.get('Content-Length', 0))
            if parsed.path != '/result' or parse_qs(parsed.query).get('token') != [token] or not 0 < size <= 16 * 1024**2:
                self.send_error(400)
                return
            try:
                body = json.loads(self.rfile.read(size))
            except (ValueError, UnicodeDecodeError):
                self.send_error(400)
                return
            response.update(body)
            self.send_response(200)
            self.end_headers()
            received.set()

    server = ThreadingHTTPServer(('127.0.0.1', 0), Handler)
    serving = threading.Thread(target=server.serve_forever, daemon=True)
    serving.start()
    budget = Budget(args.cgroup_parent, args.rss_mib * 1024**2, int(args.cpu_quota * 100000))
    samples = []
    result = {'options': options, 'status': 'failed', 'directory': directory.name}
    process = None
    started = time.monotonic()
    profile = tempfile.TemporaryDirectory(prefix='ristretto-browser-')
    try:
        url = f'http://127.0.0.1:{server.server_port}/?' + urlencode({'token': token, 'options': json.dumps(options)})
        command = [str(args.chrome), '--headless', '--disable-gpu', '--no-first-run', '--no-default-browser-check',
            '--disable-background-networking', '--disable-default-apps', '--disable-sync', '--disable-component-update',
            '--disable-background-timer-throttling', '--disable-renderer-backgrounding',
            '--disable-features=Translate,OptimizationHints,MediaRouter', f'--user-data-dir={profile.name}', url]
        if args.no_sandbox:
            command.insert(1, '--no-sandbox')
        # A fresh launcher joins the budget before exec. Avoid preexec_fn in
        # this multithreaded HTTP host; all Chromium descendants inherit caps.
        launcher = ('import os,sys; from pathlib import Path; '
                    'Path(sys.argv[1]).write_text("0"); '
                    'os.sched_setaffinity(0,{int(x) for x in sys.argv[2].split(",")}); '
                    'os.execv(sys.argv[3],sys.argv[3:])')
        invocation = [sys.executable, '-c', launcher, str(budget.path / 'cgroup.procs'), args.cpus, *command]
        (directory / 'command.json').write_text(json.dumps(invocation, indent=2) + '\n')
        with (directory / 'chrome.log').open('w') as log:
            process = subprocess.Popen(invocation, stdout=log, stderr=subprocess.STDOUT, start_new_session=True)
            deadline = started + options['seconds'] + options['warmup'] + 60
            while not received.wait(.05):
                samples.append({'elapsed_s': time.monotonic() - started,
                                'memory_bytes': int((budget.path / 'memory.current').read_text())})
                if process.poll() is not None or time.monotonic() >= deadline:
                    break
            if received.is_set():
                (directory / 'browser.json').write_text(json.dumps(response, indent=2) + '\n')
                if response.get('status') == 'ok':
                    validate(response['result'], options)
                    result.update(status='ok', measurement=response['result'])
                else:
                    result['error'] = response.get('error', 'browser failed without explanation')
            else:
                result['error'] = 'browser exited without a result' if process.poll() is not None else 'browser timeout'
            result['cgroup'] = budget.snapshot()
            if result['cgroup']['memory_events'].get('oom', 0):
                result.update(status='failed', error='browser cgroup OOM')
    except Exception as error:
        result.update(status='failed', error=f'{type(error).__name__}: {error}')
    finally:
        if process is not None and process.poll() is None:
            os.killpg(process.pid, signal.SIGTERM)
            try:
                process.wait(timeout=5)
            except subprocess.TimeoutExpired:
                os.killpg(process.pid, signal.SIGKILL)
                process.wait()
        if 'cgroup' not in result:
            result['cgroup'] = budget.snapshot()
        budget.close()
        profile.cleanup()
        server.shutdown()
        server.server_close()
        serving.join()
    result['elapsed_s'] = time.monotonic() - started
    (directory / 'process-memory.json').write_text(json.dumps(samples) + '\n')
    (directory / 'result.json').write_text(json.dumps(result, indent=2) + '\n')
    return result


def report(rows, output, plot):
    lines = ['# Browser collector screen', '',
        'Fixed scheduled arrivals; completed-request tails are censored when requests remain unfinished. '
        'These are actual Chromium main-thread measurements of the Rust collector, not a browser JVM or a ZGC/G1 comparison. '
        'Wasm linear memory does not shrink when objects are reclaimed. Qualification remains false. '
        'Request-turn work includes all polls and the GC step in turns with at least one request poll; '
        'maintenance-only and idle turns remain in the separate raw all-turn histogram. '
        'Clock resolution is recorded per fork and limits interpretation of small durations.', '',
        '| Policy | Fork | Completed/offered | Arrival p99.9 µs | Request-turn work p99.9 µs | Heartbeat max µs | Peak cgroup memory MiB | Unfinished |',
        '|---|---:|---:|---:|---:|---:|---:|---:|']
    successful = [row for row in rows if row['status'] == 'ok']
    for row in rows:
        if row['status'] != 'ok':
            lines.append(f"| {row['options']['policy']} | {row['repeat']} | FAILED: {row.get('error')} | — | — | — | — | — |")
            continue
        m = row['measurement']['measurement']
        h = m['histograms']
        lines.append(f"| {row['options']['policy']} | {row['repeat']} | {m['completed']}/{m['offered']} | "
            f"{quantile(h['arrival'], .999)} | {quantile(h['request_turn_work'], .999)} | "
            f"{h['heartbeat_gap']['max_ns'] / 1000:.1f} | {row['cgroup']['memory_peak'] / 1024**2:.2f} | {m['unfinished']} |")
    (output / 'REPORT.md').write_text('\n'.join(lines) + '\n')
    if not plot or not successful:
        return
    import matplotlib.pyplot as plt
    metrics = [('Arrival p99.9 (µs)', lambda r: quantile(r['measurement']['measurement']['histograms']['arrival'], .999)),
        ('Request-turn work p99.9 (µs)', lambda r: quantile(r['measurement']['measurement']['histograms']['request_turn_work'], .999)),
        ('Maximum heartbeat gap (µs)', lambda r: r['measurement']['measurement']['histograms']['heartbeat_gap']['max_ns'] / 1000),
        ('Peak cgroup memory (MiB)', lambda r: r['cgroup']['memory_peak'] / 1024**2),
        ('Final Wasm linear memory (MiB)', lambda r: r['measurement']['final_memory_bytes'] / 1024**2),
        ('Unfinished requests (censored tails)', lambda r: r['measurement']['measurement']['unfinished'])]
    fig, axes = plt.subplots(2, 3, figsize=(12, 8), layout='constrained')
    policies = ['control', 'baseline', 'candidate']
    for axis, (label, metric) in zip(axes.flat, metrics):
        for index, policy in enumerate(policies):
            values = [metric(r) for r in successful if r['options']['policy'] == policy]
            values = [v for v in values if v is not None]
            if values:
                axis.scatter([index] * len(values), values, alpha=.65)
                axis.plot([index - .2, index + .2], [statistics.median(values)] * 2, color='black')
        axis.set_xticks(range(3), policies)
        axis.set_title(label)
        observed = [metric(row) for row in successful]
        observed = [value for value in observed if value is not None]
        axis.set_ylim(0, max(observed, default=0) * 1.1 or 1)
        if label.startswith('Unfinished') and all(metric(row) == 0 for row in successful):
            axis.set_ylim(0, 1)
            axis.set_yticks([0, 1])
    fig.suptitle('Actual browser collector screen — individual forks and medians; qualification incomplete')
    for extension in ['svg', 'png']:
        fig.savefig(output / f'browser.{extension}', dpi=160)
    plt.close(fig)
    (output / 'graph-provenance.json').write_text(json.dumps({
        'matplotlib': plt.matplotlib.__version__, 'raw_sha256': digest(output / 'raw.jsonl'),
        'renderer_sha256': digest(Path(__file__)),
        'graphs': {name: digest(output / name) for name in ['browser.svg', 'browser.png']}
    }, indent=2) + '\n')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--wasm', type=Path, required=True)
    parser.add_argument('--chrome', type=Path, default=Path('/usr/bin/google-chrome'))
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--cgroup-parent', type=Path, required=True)
    parser.add_argument('--cpus', default='0,2')
    parser.add_argument('--cpu-quota', type=float, default=2)
    parser.add_argument('--rss-mib', type=int, default=512)
    parser.add_argument('--heap-mib', type=int, default=16)
    parser.add_argument('--seconds', type=float, default=10)
    parser.add_argument('--warmup', type=float, default=2)
    parser.add_argument('--rate', type=int, default=10000)
    parser.add_argument('--poll-budget', type=int, default=64)
    parser.add_argument('--repeats', type=int, default=5)
    parser.add_argument('--seed', type=int, default=42)
    parser.add_argument('--no-sandbox', action='store_true', help='explicitly disable Chrome sandbox for an isolated test environment')
    parser.add_argument('--plot', action='store_true')
    args = parser.parse_args()
    cpus = {int(cpu) for cpu in args.cpus.split(',')}
    if (not cpus or not cpus <= os.sched_getaffinity(0) or not 0 < args.cpu_quota <= len(cpus)
            or not 1 <= args.heap_mib <= 1024 or args.rss_mib <= 0 or args.repeats < 1
            or not .25 <= args.seconds <= 3600 or not .25 <= args.warmup <= 600
            or not 1 <= args.poll_budget <= 1024 or args.rate <= 0
            or (args.seconds + args.warmup) * args.rate >= 2**32):
        parser.error('invalid resource, duration, arrival or polling bounds')
    args.chrome = args.chrome.resolve(strict=True)
    args.wasm = args.wasm.resolve(strict=True)
    provenance = args.wasm.with_name(args.wasm.name + '.provenance.json')
    archive = args.wasm.parent / 'sources.tar.gz'
    if not provenance.exists() or not archive.exists():
        parser.error('freeze the built Wasm and source together with validation/provenance.py first')
    build = json.loads(provenance.read_text())
    if build['binary_sha256'] != digest(args.wasm) or build['source_archive_sha256'] != digest(archive):
        parser.error('Wasm build/source provenance does not match')
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    frozen = output / 'frozen'; frozen.mkdir()
    for source, name in [(args.wasm, 'workload.wasm'), (ASSETS / 'index.html', 'index.html'),
                         (ASSETS / 'browser.js', 'browser.js'), (provenance, 'workload.provenance.json'),
                         (archive, 'sources.tar.gz')]:
        shutil.copy2(source, frozen / name)
    hashes = {str(path): digest(path) for path in frozen.iterdir()}
    chrome_engine = args.chrome.parent / 'chrome'
    for source in [Path(__file__).resolve(), Path(__file__).with_name('cgroups.py'), Path(__file__).with_name('run.py')]:
        hashes[str(source)] = digest(source)
    hashes[str(args.chrome)] = digest(args.chrome)
    if chrome_engine.is_file(): hashes[str(chrome_engine)] = digest(chrome_engine)
    metadata = {'schema': 1, 'options': {key: str(value) if isinstance(value, Path) else value for key, value in vars(args).items()},
        'python': sys.version, 'host': platform.platform(),
        'hashes': hashes, 'chrome_version': capture([str(args.chrome), '--version']), 'qualification': False,
        'scope': 'foreground-equivalent headless Chromium; one main-thread collector mutator; no concurrent build/test/profiler'}
    (output / 'metadata.json').write_text(json.dumps(metadata, indent=2) + '\n')
    rng = random.Random(args.seed)
    jobs = []
    for repeat in range(args.repeats):
        policies = ['control', 'baseline', 'candidate']; rng.shuffle(policies)
        jobs.extend((repeat, policy) for policy in policies)
    (output / 'schedule.json').write_text(json.dumps(jobs) + '\n')
    rows = []
    with (output / 'raw.jsonl').open('w') as raw:
        for number, (repeat, policy) in enumerate(jobs):
            directory = output / f'run-{number:03d}-{policy}'; directory.mkdir()
            options = {key: getattr(args, key) for key in ['seconds', 'warmup', 'rate', 'heap_mib', 'poll_budget']}
            options['policy'] = policy
            row = trial(args, options, directory, frozen)
            row['repeat'] = repeat; rows.append(row)
            raw.write(json.dumps(row) + '\n'); raw.flush()
            print(f"{repeat} {policy}: {row['status']} {row.get('error', '')}", flush=True)
    unchanged = all(digest(Path(path)) == value for path, value in hashes.items())
    (output / 'summary.json').write_text(json.dumps({'qualification': False, 'inputs_unchanged': unchanged,
        'failed': sum(row['status'] != 'ok' for row in rows), 'trials': len(rows)}, indent=2) + '\n')
    report(rows, output, args.plot)
    return int(not unchanged or any(row['status'] != 'ok' for row in rows))


if __name__ == '__main__':
    raise SystemExit(main())
