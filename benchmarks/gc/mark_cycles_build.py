#!/usr/bin/env python3
"""Build a GC example in isolation or record an existing binary for a paired run."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import tempfile
import tomllib
from paired_measurement import sha256


def compiled_example(messages, source, fixture, target_dir):
    library = (source / 'ristretto_gc/src/lib.rs').resolve()
    producers = {}
    for line in messages.splitlines():
        item = json.loads(line)
        if item.get('reason') != 'compiler-artifact':
            continue
        src = Path(item['target']['src_path']).resolve()
        if src not in (library, fixture):
            continue
        if item['fresh'] or src in producers:
            raise ValueError(f'expected one fresh compiler artifact for {src}')
        producers[src] = item
    if library not in producers or fixture not in producers:
        raise ValueError('missing fresh collector or example compiler artifact')
    example = producers[fixture]
    if (example['package_id'] != producers[library]['package_id']
            or 'lib' not in producers[library]['target']['kind']
            or 'example' not in example['target']['kind']):
        raise ValueError('unexpected collector or example compiler target')
    artifact = Path(example['executable']).resolve(strict=True)
    if not artifact.is_relative_to(target_dir):
        raise ValueError('Cargo emitted the executable outside the isolated target')
    return artifact, example['package_id']


def build_controls(messages, source, collector_id):
    """Compare the actual compiled closure when the workspace membership changes."""
    manifest = tomllib.loads((source / 'Cargo.toml').read_text())
    workspace = manifest['workspace']
    for field in ('members', 'default-members', 'exclude'):
        workspace.pop(field, None)
    lock = tomllib.loads((source / 'Cargo.lock').read_text())
    registry_packages = [pkg for pkg in lock['package'] if 'source' in pkg]
    artifacts = []
    for line in messages.splitlines():
        item = json.loads(line)
        if item.get('reason') != 'compiler-artifact':
            continue
        package = item['package_id']
        if package == collector_id:
            package = 'ristretto_gc'
        elif not package.startswith('registry+'):
            # The receipt does not hash source for another local or Git dependency.
            return None
        artifacts.append([package, item['target']['name'], item['target']['kind'],
                          sorted(item['features']), item['profile']])
    artifacts.sort(key=lambda row: json.dumps(row, sort_keys=True))
    settings = {key: value for key, value in os.environ.items()
                if key in ('RUSTFLAGS', 'CARGO_ENCODED_RUSTFLAGS', 'RUSTC_WRAPPER')
                or key.startswith(('CARGO_PROFILE_', 'CARGO_TARGET_')) and key != 'CARGO_TARGET_DIR'}
    config_files = {str(path.relative_to(source)): sha256(path)
                    for path in [source / '.cargo/config.toml', source / '.cargo/config',
                                 source / 'rust-toolchain.toml'] if path.is_file()}
    inputs = dict(workspace=manifest, registry_packages=registry_packages,
                  artifacts=artifacts, settings=settings, config_files=config_files)
    return hashlib.sha256(json.dumps(inputs, sort_keys=True).encode()).hexdigest()


def compile_binary(source, fixture, command, binary, hashes, input_hashes):
    # A target shared by different checkouts can carry a fresh-looking executable
    # from an earlier build. Hash the artifact named by this compiler invocation.
    with tempfile.TemporaryDirectory(prefix='gc-build-', dir=binary.parent) as temp:
        target_dir = Path(temp).resolve()
        env = dict(os.environ, CARGO_TARGET_DIR=str(target_dir),
                   CARGO_BUILD_BUILD_DIR=str(target_dir / 'build'))
        result = subprocess.run([*command, '--message-format=json'], cwd=source, env=env,
                                stdout=subprocess.PIPE, text=True, check=True)
        artifact, package = compiled_example(result.stdout, source, fixture, target_dir)
        if hashes != input_hashes():
            raise ValueError('collector inputs changed during compilation')
        # The hard link publishes the completed artifact exclusively before cleanup.
        os.link(artifact, binary)
        return dict(package_id=package, library='ristretto_gc/src/lib.rs',
                    example=str(fixture.relative_to(source)), fresh=True,
                    method='isolated-cargo-json-v1',
                    controls_sha256=build_controls(result.stdout, source, package))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--source', type=Path, required=True)
    parser.add_argument('--binary', type=Path, required=True)
    parser.add_argument('--target', choices=['native', 'wasm'], required=True)
    parser.add_argument('--fixture', choices=['mark_cycles', 'marking_births'], default='mark_cycles')
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument("--features", nargs="*", default=[])
    parser.add_argument('--build', action='store_true',
                        help='compile in a fresh target and create the binary at --binary')
    args = parser.parse_args()
    source, binary = args.source.resolve(), args.binary.resolve()
    fixture = source / f'ristretto_gc/examples/{args.fixture}.rs'
    def input_hashes():
        paths = [source / 'Cargo.toml', source / 'Cargo.lock', source / 'rust-toolchain.toml',
                 source / 'ristretto_gc/Cargo.toml', fixture]
        paths.extend(sorted((source / 'ristretto_gc/src').rglob('*.rs')))
        return {str(path.relative_to(source)): sha256(path) for path in paths}

    hashes = input_hashes()
    toolchain = subprocess.run(['rustc', '--version', '--verbose'], cwd=source,
                               capture_output=True, text=True, check=True).stdout.strip()
    command = ['cargo', 'build', '--locked', '--release', '-p', 'ristretto_gc', '--example', args.fixture]
    if args.features:
        command += ['--features', ','.join(args.features)]
    if args.target == 'wasm':
        command += ['--target', 'wasm32-wasip2']
    receipt = None
    if args.build:
        if args.output.exists() or binary.exists():
            parser.error('isolated build needs unused binary and manifest paths')
        receipt = compile_binary(source, fixture, command, binary, hashes, input_hashes)
    data = dict(fixture=args.fixture, binary=str(binary), binary_sha256=sha256(binary), source=str(source),
                source_sha256=hashes, fixture_sha256=sha256(fixture), target=args.target,
                profile_sha256=sha256(source / 'Cargo.toml'),
                dependencies_sha256=sha256(source / 'Cargo.lock'), toolchain=toolchain,
                build_command=command, features=args.features,
                provenance_limit='Records supplied source and binary after build; does not attest compiler execution')
    if receipt:
        data['compilation'] = receipt
        data['build_command'] = [*command, '--message-format=json']
        data['provenance_limit'] = 'Local compiler receipt for the recorded inputs and artifact'
    with args.output.open('x') as stream:
        stream.write(json.dumps(data, indent=2) + '\n')


if __name__ == '__main__':
    main()
