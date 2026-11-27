#!/usr/bin/env python3
"""Verify that shipping entry points resolve to the ordinary collector on each host."""
import json
from pathlib import Path
import subprocess
import sys


ROOT = Path(__file__).resolve().parents[3]


sys.path.insert(0, str(ROOT / ".github/scripts"))
from collector_graph import RETIRED, check_collector_source, verify_tree


def verify_workspace():
    metadata = json.loads(subprocess.check_output(
        ["cargo", "metadata", "--locked", "--no-deps", "--format-version=1"], cwd=ROOT))
    collector_manifest = ROOT / "ristretto_gc/Cargo.toml"
    collector = None
    for package in metadata["packages"]:
        if package["name"] in RETIRED or any(
            f.startswith("experimental-canonical") for f in package["features"]
        ):
            raise ValueError(f"retired research package or feature in {package['name']}")
        if (Path(package["manifest_path"]) == collector_manifest
                and package["name"] == "ristretto_gc"
                and package["id"] in metadata["workspace_members"]):
            collector = f"ristretto_gc v{package['version']} ({collector_manifest.parent})"
    check_collector_source(collector_manifest.parent)
    for name in RETIRED:
        if (ROOT / name / "Cargo.toml").is_file():
            raise ValueError(f"retired research crate in workspace checkout: {name}")
    if collector is None:
        raise ValueError(f"missing workspace collector at {collector_manifest}")
    return collector


def verify(package, target, expected, all_features):
    mode = "all features and build dependencies" if all_features else "default runtime"
    context = f"{package} ({target or 'native'}, {mode})"
    try:
        verify_tree(ROOT / "Cargo.toml", expected, package=package, target=target,
                    all_features=all_features, build_deps=all_features)
    except ValueError as error:
        raise ValueError(f"{context}: {error}") from error
    print(f"{context}: {expected}")


if __name__ == "__main__":
    expected = verify_workspace()
    for package, targets in [
        ("ristretto_vm", [None, "wasm32-wasip2"]),
        ("ristretto_java", [None, "wasm32-wasip2"]),
        ("ristretto_javac", [None]),
        ("ristretto_playground", ["wasm32-wasip2", "wasm32-unknown-unknown"]),
    ]:
        for target in targets:
            for all_features in (False, True):
                verify(package, target, expected, all_features)
