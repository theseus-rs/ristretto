#!/usr/bin/env python3
"""Validate catalog byte charges and refund order with separate allocator audits."""
from pathlib import Path
import argparse
import csv
import io
import json
import os
import shutil
from catalog_model_check import run
from trace_certificate_check import ROOT, digest

HEADER = 'registry,mutators,helpers,backing,participants,total,requests,refund_after_last_free'


def valid_audit_result(code, log, target, witness=None):
    if 'Running `' not in log or 'could not compile' in log:
        return False
    if witness is not None:
        return code == (101 if target == 'native' else 134) and witness in log
    if code != 0 or HEADER not in log:
        return False
    lines = [line for line in log.splitlines() if line and line[0].isdigit() and ',' in line]
    try:
        rows = list(csv.DictReader(io.StringIO(HEADER + '\n' + '\n'.join(lines))))
        observed = {(int(r['registry']), int(r['mutators']), int(r['helpers'])) for r in rows}
        expected = {(n, m, 1) for n in [1, 64, 1024, 16384] for m in [0, 1, 7]}
        return len(rows) == 12 and observed == expected and all(
            int(r['backing']) > 0 and int(r['participants']) > 0
            and int(r['total']) == int(r['backing']) + int(r['participants'])
            and int(r['requests']) == 9 and r['refund_after_last_free'] == 'true'
            for r in rows)
    except (ValueError, TypeError, KeyError):
        return False


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--wasmtime', default='wasmtime')
    parser.add_argument('--catalog-source', type=Path, default=ROOT / 'ristretto_gc/validation/catalog/catalog-tagged-prototype')
    parser.add_argument('--pool-source', type=Path, default=ROOT / 'ristretto_gc/validation/catalog/catalog-tagged-pool-prototype')
    parser.add_argument('--timeout', type=int, default=180)
    args = parser.parse_args()
    out = args.output.resolve()
    out.mkdir(parents=True, exist_ok=False)
    source = out / 'inputs'
    originals = {}
    for component, package in [('catalog', args.catalog_source.resolve()), ('pool', args.pool_source.resolve())]:
        shutil.copytree(package, source / component, ignore=shutil.ignore_patterns('target'))
        originals.update({str(p): digest(p) for p in package.rglob('*')
                          if p.is_file() and 'target' not in p.relative_to(package).parts})
    manifest = source / 'catalog/Cargo.toml'
    manifest.write_text(manifest.read_text().replace('path = "../catalog-tagged-pool-prototype"', 'path = "../pool"'))
    variants = [
        ('positive', None, None, None, None),
        ('early-refund', 'catalog/src/bounded.rs',
         '        if let Some(catalog) = Arc::into_inner(self.inner.take().expect("live handle")) {\n            drop(catalog);\n        }',
         '        drop(self.inner.take());', 'credit returned before final backing free'),
        ('undercharge-arc', 'pool/src/lib.rs',
         '.pad_to_align()\n            .size(),', '.pad_to_align()\n            .size() - 1,',
         'computed backing differs from allocator requests'),
    ]
    results, sources = [], {}
    for name, path, old, new, witness in variants:
        dst = out / name
        shutil.copytree(source, dst)
        if path:
            file = dst / path
            content = file.read_text()
            assert content.count(old) == 1, name
            file.write_text(content.replace(old, new, 1))
        sources[name] = {str(p.relative_to(dst)): digest(p) for p in dst.rglob('*') if p.is_file()}
        for target in ['native', 'wasi']:
            runner = args.wasmtime + ' run -W max-wasm-stack=8388608'
            environment = dict(os.environ, CARGO_TARGET_DIR=str(out / (name + '-build')),
                               CARGO_BUILD_JOBS='4', CARGO_TARGET_WASM32_WASIP2_RUNNER=runner)
            command = ['cargo', 'run', '--offline', '--locked', '--manifest-path', str(dst / 'catalog/Cargo.toml'),
                       '--example', 'accounting', '--features', 'allocation-audit']
            if target == 'wasi':
                command += ['--target', 'wasm32-wasip2']
            print('start', name, target, flush=True)
            result = run(command, out / (name + '-' + target + '.log'), environment, args.timeout)
            result.update(name=name, target=target, negative=witness is not None,
                          valid=valid_audit_result(result['exit_code'], Path(result['log']).read_text(), target, witness),
                          environment={k: environment[k] for k in ['CARGO_TARGET_DIR', 'CARGO_BUILD_JOBS', 'CARGO_TARGET_WASM32_WASIP2_RUNNER']})
            results.append(result)
            unchanged = all(digest(Path(p)) == sha for p, sha in originals.items()) and all(
                digest(out / variant / p) == sha for variant, files in sources.items() for p, sha in files.items())
            (out / 'result.json').write_text(json.dumps(dict(runs=results, sources=sources, originals=originals,
                inputs_unchanged=unchanged, qualification=False), indent=2) + '\n')
            print('finish', name, target, result['exit_code'], result['valid'], flush=True)
            assert result['valid'] and unchanged, (name, target)


if __name__ == '__main__':
    main()
