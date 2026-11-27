#!/usr/bin/env python3
"""Verify that shipping entry points resolve to the ordinary collector on each host."""
import argparse
import json
from pathlib import Path
import subprocess
import sys


ROOT = Path(__file__).resolve().parents[3]


sys.path.insert(0, str(ROOT / ".github/scripts"))
from collector_graph import RETIRED, check_shipping_source, verify_tree, verify_workspace_tree


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
        manifest = Path(package["manifest_path"])
        check_shipping_source(package["name"], manifest.parent)
        if (manifest == collector_manifest
                and package["name"] == "ristretto_gc"
                and package["id"] in metadata["workspace_members"]):
            collector = f"ristretto_gc v{package['version']} ({collector_manifest.parent})"
    for name in RETIRED:
        if (ROOT / name / "Cargo.toml").is_file():
            raise ValueError(f"retired research crate in workspace checkout: {name}")
    if collector is None:
        raise ValueError(f"missing workspace collector at {collector_manifest}")
    return collector


def verify(package, target, expected, feature_args=(), *, build_deps=False):
    mode = " ".join(feature_args) or "default features"
    edges = "normal and build" if build_deps else "normal"
    context = f"{package} ({target or 'native'}, {mode}, {edges})"
    try:
        verify_tree(ROOT / "Cargo.toml", expected, package=package, target=target,
                    feature_args=feature_args, build_deps=build_deps)
    except ValueError as error:
        raise ValueError(f"{context}: {error}") from error
    print(f"{context}: {expected}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--launcher-target", help="verify the java release binary for this Rust target")
    parser.add_argument("--no-default-features", action="store_true")
    parser.add_argument("--features", help="Cargo features selected by the release build")
    args = parser.parse_args()
    if not args.launcher_target and (args.no_default_features or args.features):
        parser.error("launcher feature selection requires --launcher-target")
    expected = verify_workspace()
    if args.launcher_target:
        features = ["--no-default-features"] if args.no_default_features else []
        if args.features:
            features.extend(["--features", args.features])
        verify("ristretto_java", args.launcher_target, expected, features)
        sys.exit(0)

    verify_workspace_tree(ROOT / "Cargo.toml", expected)
    print(f"All workspace targets, features and dependency kinds: {expected}")

    for package, targets in [
        ("ristretto_classloader", [None, "wasm32-wasip2"]),
        ("ristretto_vm", [None, "wasm32-wasip2"]),
        ("ristretto_java", [None, "wasm32-wasip2"]),
        ("ristretto_javac", [None]),
        ("ristretto_playground", ["wasm32-wasip2", "wasm32-unknown-unknown"]),
    ]:
        for target in targets:
            verify(package, target, expected)
            verify(package, target, expected, ["--all-features"], build_deps=True)
    for target in ("wasm32-unknown-unknown", "wasm32-wasip2", "wasm64-unknown-unknown"):
        verify("ristretto_java", target, expected, ["--no-default-features"])
