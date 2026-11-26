#!/usr/bin/env python3
"""Build selected features from the workspace's actual Cargo release archives."""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import tarfile
import tempfile


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--target", help="Rust target triple for the external consumer build")
    parser.add_argument("features", nargs="+", metavar="PACKAGE[/FEATURE]")
    args = parser.parse_args()
    metadata = json.loads(subprocess.check_output([
        "cargo", "metadata", "--locked", "--no-deps", "--format-version=1"
    ]))
    packages = {p["name"]: p for p in metadata["packages"] if p["publish"] != []}
    selected = {}
    for item in args.features:
        package, sep, feature = item.partition("/")
        if package not in packages or (sep and feature not in packages[package]["features"]):
            parser.error(f"unknown publishable workspace package or feature: {item}")
        enabled = selected.setdefault(package, set())
        if sep:
            enabled.add(feature)
    required = set(selected)
    pending = list(selected)
    while pending:
        current = packages[pending.pop()]
        for dep in current["dependencies"]:
            name = dep["name"]
            if dep["kind"] != "dev" and name in packages and name not in required:
                required.add(name)
                pending.append(name)
    # Stage the same publishable set as cargo publish --workspace. cargo package
    # --workspace also tries to normalize path-only, workspace-private crates.
    stage = ["cargo", "package", "--locked", "--no-verify"]
    for name in sorted(packages):
        stage.extend(["-p", name])
    subprocess.run(stage, check=True)
    archives = Path(metadata["target_directory"]) / "package"
    with tempfile.TemporaryDirectory() as temporary:
        root = Path(temporary)
        manifest = ['[package]', 'name = "packaged-feature-check"', 'version = "0.0.0"',
                    'edition = "2024"', 'publish = false', '[workspace]', '[dependencies]']
        for name, features in sorted(selected.items()):
            version = packages[name]["version"]
            manifest.append(f'{name} = {{ version = "={version}", default-features = false, '
                            f'features = {json.dumps(sorted(features))} }}')
        # Resolve all in-flight crates at the same versions a coordinated publish would provide.
        manifest.append("[patch.crates-io]")
        package_set = hashlib.sha256()
        for name in sorted(required):
            package = packages[name]
            directory = f"{name}-{package['version']}"
            path = archives / (directory + ".crate")
            with path.open("rb") as source:
                package_set.update(hashlib.file_digest(source, "sha256").digest())
            with tarfile.open(path) as archive:
                archive.extractall(root, filter="data")
            manifest.append(f'{name} = {{ path = "{directory}" }}')
        (root / "Cargo.toml").write_text("\n".join(manifest) + "\n", encoding="utf-8")
        (root / "src").mkdir()
        libraries = [target["name"] for name in sorted(selected) for target in packages[name]["targets"]
                     if target["kind"] == ["lib"] or target["kind"] == ["proc-macro"]]
        (root / "src/lib.rs").write_text("".join(f"use {library} as _;\n" for library in libraries),
                                          encoding="utf-8")
        # The consumer is new and its dependency manifests are the normalized package versions.
        # Keep the overall workspace's pinned versions wherever their constraints allow them.
        (root / "Cargo.lock").write_bytes((Path(metadata["workspace_root"]) / "Cargo.lock").read_bytes())
        subprocess.run(["cargo", "update", "--workspace", "--manifest-path", root / "Cargo.toml", "--offline"], check=True)
        # Cargo archives normalize file timestamps. Two revisions can have the same
        # version, filenames and mtimes; only reuse artifacts for identical archives.
        suffix = "packaged-features" if args.target is None else "packaged-features-" + args.target
        build = ["cargo", "build", "--manifest-path", root / "Cargo.toml", "--locked",
                 "--target-dir", (Path(metadata["target_directory"]) / suffix / package_set.hexdigest()).resolve()]
        if args.target is not None:
            build.extend(["--target", args.target])
        subprocess.run(build, check=True)


if __name__ == "__main__":
    main()
