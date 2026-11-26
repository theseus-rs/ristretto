#!/usr/bin/env python3
"""Functional checks only: every workload command includes --check (no samples)."""
import argparse
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import tempfile


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('binary', type=Path)
    parser.add_argument('--wasmtime', type=Path)
    parser.add_argument('--compiled', action='store_true')
    parser.add_argument('--evidence', type=Path, required=True)
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[3]
    fixture = root / 'ristretto_vm/tests/fixtures/canonical'
    oracle = {}
    for row in (fixture / 'ring-oracle.tsv').read_text().splitlines():
        method, steps, result = row.split('\t')
        oracle[method, int(steps)] = int(result)
    args.evidence.mkdir(parents=True, exist_ok=False)
    commands = []
    with tempfile.TemporaryDirectory(prefix='canonical-alloc-check-') as name:
        inputs = Path(name)
        classes = inputs / 'classes'
        classes.mkdir()
        for filename in ['CanonicalRingWorkload.class', 'CanonicalRingWorkload$Node.class']:
            shutil.copy2(fixture / filename, classes / filename)
        manifest = b''
        for path in sorted(classes.iterdir()):
            manifest += path.name.encode() + b'\0' + hashlib.sha256(path.read_bytes()).hexdigest().encode() + b'\n'
        class_digest = hashlib.sha256(manifest).hexdigest()
        tiers = ['interpreted', 'compiled'] if args.compiled else ['interpreted']
        cases = [('small', -1, 8), ('small', 1, 8), ('medium', 7, 32), ('large', 0, 128)]

        def run(admission, label, success=True):
            encoded = (json.dumps(admission, indent=2) + '\n').encode()
            (inputs / 'admission.json').write_bytes(encoded)
            if args.wasmtime:
                command = [str(args.wasmtime.resolve()), 'run', '-W', 'max-wasm-stack=8388608',
                           '-S', 'http=y', '--dir', str(inputs), str(args.binary.resolve())]
            else:
                command = [str(args.binary.resolve())]
            command += [str(inputs), '--check', '--samples', '2']
            assert '--check' in command
            result = subprocess.run(command, text=True, capture_output=True, timeout=120)
            (args.evidence / (label + '.stdout')).write_text(result.stdout)
            (args.evidence / (label + '.stderr')).write_text(result.stderr)
            (args.evidence / (label + '.admission.json')).write_bytes(encoded)
            commands.append(dict(command=command, returncode=result.returncode, label=label))
            if not success:
                assert result.returncode != 0 and 'RISTRETTO_CANONICAL_ALLOCATIONS' not in result.stdout, label
                return
            assert result.returncode == 0, (label, result.stderr)
            lines = result.stdout.splitlines()
            assert len(lines) == 1 and lines[0].startswith('RISTRETTO_CANONICAL_ALLOCATIONS '), label
            packet = json.loads(lines[0].split(' ', 1)[1])
            assert packet['check_only'] and not packet['measurement'] and not packet['qualification']
            assert not packet['timing_comparable'] and packet['warmup_invocations'] == 0
            assert packet['admission_sha256'] == hashlib.sha256(encoded).hexdigest()
            assert packet['class_manifest_sha256'] == class_digest
            phases = [row['phase'] for row in packet['stages']]
            wanted = ['admission'] + (['compilation'] if admission['tier'] == 'compiled' else [])
            assert phases == wanted + ['full_invocation'] * 2 + ['program_teardown']
            assert all(row['counts'] is None for row in packet['stages'])
            for row in packet['stages']:
                if row['phase'] == 'full_invocation':
                    arrays = 0 if admission['argument'] < 0 else retained + admission['argument']
                    assert row['result'] == admission['expected_result']
                    assert row['array_allocations'] == arrays
                    assert row['payload_allocations'] == row['released_objects'] == arrays * 2
                    assert row['payload_bytes_after_cleanup'] == row['catalog_bytes_after_drop'] == 0

        for tier in tiers:
            for method, steps, retained in cases:
                admission = dict(schema_version=1, tier=tier, **{'class': 'CanonicalRingWorkload'},
                                 method=method, argument=steps, expected_result=oracle[method, steps],
                                 require_compiled_backedges=steps >= 0,
                                 configuration=dict(objects=retained * 4 + 8, roots=128, mutators=1,
                                                    helpers=1, edges_per_object=3),
                                 limits=dict(instructions=4000000, request_polls=1000000,
                                             cleanup_polls=4000000, frames=64,
                                             native_slices=4000000, array_bytes=8388608))
                run(admission, f'{tier}-{method}-{steps}')
        admission['expected_result'] ^= 1
        run(admission, 'wrong-checksum', success=False)
        admission['unexpected'] = 1
        run(admission, 'unknown-field', success=False)
        if args.wasmtime:
            del admission['unexpected']
            admission['tier'] = 'compiled'
            run(admission, 'unsupported-compiled', success=False)
    (args.evidence / 'commands.json').write_text(json.dumps(commands, indent=2) + '\n')
    print(f'{len(commands)} functional cases passed; accounting disabled; no allocation samples')


if __name__ == '__main__':
    main()
