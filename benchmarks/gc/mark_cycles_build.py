#!/usr/bin/env python3
"""Record an already built GC example and its sources for a paired run."""
import argparse
import json
from pathlib import Path
import subprocess
from paired_measurement import sha256


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--source', type=Path, required=True)
    parser.add_argument('--binary', type=Path, required=True)
    parser.add_argument('--target', choices=['native', 'wasm'], required=True)
    parser.add_argument('--fixture', choices=['mark_cycles', 'marking_births'], default='mark_cycles')
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument("--features", nargs="*", default=[])
    args = parser.parse_args()
    source, binary = args.source.resolve(), args.binary.resolve()
    fixture = source / f'ristretto_gc/examples/{args.fixture}.rs'
    paths = [source / 'Cargo.toml', source / 'Cargo.lock', source / 'rust-toolchain.toml',
             source / 'ristretto_gc/Cargo.toml', fixture]
    paths.extend(sorted((source / 'ristretto_gc/src').rglob('*.rs')))
    hashes = {str(p.relative_to(source)): sha256(p) for p in paths}
    toolchain = subprocess.run(['rustc', '--version', '--verbose'], cwd=source,
                               capture_output=True, text=True, check=True).stdout.strip()
    command = ['cargo', 'build', '--locked', '--release', '-p', 'ristretto_gc', '--example', args.fixture]
    if args.features:
        command += ['--features', ','.join(args.features)]
    if args.target == 'wasm':
        command += ['--target', 'wasm32-wasip2']
    data = dict(fixture=args.fixture, binary=str(binary), binary_sha256=sha256(binary), source=str(source),
                source_sha256=hashes, fixture_sha256=sha256(fixture), target=args.target,
                profile_sha256=sha256(source / 'Cargo.toml'),
                dependencies_sha256=sha256(source / 'Cargo.lock'), toolchain=toolchain,
                build_command=command, features=args.features,
                provenance_limit='Records supplied source and binary after build; does not attest compiler execution')
    with args.output.open('x') as stream:
        stream.write(json.dumps(data, indent=2) + '\n')


if __name__ == '__main__':
    main()
