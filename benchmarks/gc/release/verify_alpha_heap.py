#!/usr/bin/env python3
"""Verify that shipping entry points resolve to the ordinary collector on each host."""
import json
from pathlib import Path
import subprocess


ROOT = Path(__file__).resolve().parents[3]


RETIRED = {"ristretto_vm_research", "ristretto_gc_catalog"}


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
    if any((ROOT / "ristretto_gc/validation/catalog").rglob("Cargo.toml")):
        raise ValueError("retired standalone catalog implementation is present")
    if collector is None:
        raise ValueError(f"missing workspace collector at {collector_manifest}")
    return collector


def check_tree(output, expected):
    collectors = set()
    for line in output.splitlines():
        package_id, features = line.removesuffix(" (*)").rsplit("|", 1)
        name = package_id.split()[0]
        if name in RETIRED:
            raise ValueError(f"retired research dependency {name}")
        if name in {"ristretto_gc", "ristretto_vm"}:
            if any(feature.startswith("experimental-canonical") for feature in features.split(",")):
                raise ValueError(f"retired research feature in {name}")
            if name == "ristretto_gc":
                collectors.add(package_id)
    if collectors != {expected}:
        raise ValueError(f"expected {expected}, found {sorted(collectors)}")


def verify(package, target, expected, all_features):
    edges = "normal,build" if all_features else "normal"
    args = ["cargo", "tree", "--locked", "--package", package,
            "--edges", edges, "--prefix", "none", "--format", "{p}|{f}"]
    if all_features:
        args.append("--all-features")
    if target:
        args.extend(["--target", target])
    mode = "all features and build dependencies" if all_features else "default runtime"
    context = f"{package} ({target or 'native'}, {mode})"
    try:
        check_tree(subprocess.check_output(args, cwd=ROOT, text=True), expected)
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
