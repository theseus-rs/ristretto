#!/usr/bin/env python3
"""Verify that shipping entry points resolve to the ordinary collector on each host."""
from pathlib import Path
import subprocess


ROOT = Path(__file__).resolve().parents[3]


def verify(package, target):
    args = ["cargo", "tree", "--locked", "--package", package, "--all-features",
            "--edges", "normal,build", "--prefix", "none", "--format", "{p}|{f}"]
    if target:
        args.extend(["--target", target])
    output = subprocess.check_output(args, cwd=ROOT, text=True)
    collectors = set()
    for line in output.splitlines():
        package_id, features = line.removesuffix(" (*)").split("|")
        name = package_id.split()[0]
        if name in {"ristretto_vm_research", "ristretto_gc_catalog"}:
            raise ValueError(f"{package} includes research dependency {name} on {target}")
        if name in {"ristretto_gc", "ristretto_vm"}:
            if any(feature.startswith("experimental-canonical") for feature in features.split(",")):
                raise ValueError(f"{package} enables research features in {name} on {target}")
            if name == "ristretto_gc":
                collectors.add(package_id)
    if len(collectors) != 1:
        raise ValueError(f"{package} resolves {len(collectors)} Ristretto collectors on {target}")
    print(f"{package} ({target or 'native'}): {collectors.pop().split(' (')[0]}")


if __name__ == "__main__":
    for entry in ["ristretto_vm", "ristretto_java"]:
        for platform in [None, "wasm32-wasip2"]:
            verify(entry, platform)
    verify("ristretto_playground", "wasm32-unknown-unknown")
