#!/usr/bin/env python3
"""Run the exact certificate models with Loom's proposed RMW-atomicity repair.

Loom 0.7.2 can interleave a store inside an atomic RMW (#254/#389), yielding a
false post-join certificate resurrection. PR #391 is not merged as of 2026-11-14.
This isolated checker keeps the production atomics and workspace dependencies
unchanged, runs the normally ignored regression, and records the tool overlay.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import signal
import subprocess
import time

ROOT = Path(__file__).resolve().parents[2]
UPSTREAM = "https://github.com/tokio-rs/loom/pull/391"
REVISION = "c673a8d800f3a8163586e8a6177ddafd01f2e665"


def digest(path):
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--loom-source", type=Path)
    args = parser.parse_args()
    if args.loom_source is None:
        registry = Path(os.environ.get("CARGO_HOME", str(Path.home() / ".cargo"))) / "registry/src"
        matches = list(registry.glob("*/loom-0.7.2"))
        if len(matches) != 1:
            parser.error("pass --loom-source for the installed Loom 0.7.2 source")
        args.loom_source = matches[0]
    out = args.output.resolve()
    out.mkdir(parents=True, exist_ok=False)
    checker = out / "loom"
    shutil.copytree(args.loom_source, checker, ignore=shutil.ignore_patterns("target", ".git"))
    patch = ROOT / "benchmarks/gc/loom-rmw-atomicity.patch"
    copied_patch = out / patch.name
    shutil.copy2(patch, copied_patch)
    original = digest(checker / "src/rt/atomic.rs")
    command = ["patch", "-p1", "--batch", "--fuzz=0", "-i", str(copied_patch)]
    with (out / "patch.log").open("w") as stream:
        subprocess.run(command, cwd=checker, stdout=stream, stderr=subprocess.STDOUT, check=True)
    package = out / "models"
    (package / "src").mkdir(parents=True)
    (package / "tests").mkdir()
    inputs = {
        "ristretto_gc/src/trace_certificate.rs": "src/trace_certificate.rs",
        "ristretto_gc/tests/trace_certificate_model.rs": "tests/model.rs",
    }
    hashes = {}
    for source, destination in inputs.items():
        hashes[source] = digest(ROOT / source)
        shutil.copy2(ROOT / source, package / destination)
    (package / "src/lib.rs").write_text("")
    (package / "Cargo.toml").write_text(
        '[workspace]\n[package]\nname="trace-certificate-model-check"\n'
        'version="0.0.0"\nedition="2024"\n[dev-dependencies]\nloom={path="../loom"}\n'
    )
    shutil.copy2(ROOT / "Cargo.lock", package / "Cargo.lock")
    command = ["cargo", "test", "--offline", "--manifest-path", str(package / "Cargo.toml"),
               "--test", "model", "--", "--include-ignored"]
    start = time.time()
    with (out / "check.log").open("w") as stream:
        process = subprocess.Popen(command, cwd=ROOT, stdout=stream, stderr=subprocess.STDOUT,
                                   env=dict(os.environ, CARGO_TARGET_DIR=str(out / "cargo")),
                                   start_new_session=True)
        try:
            code = process.wait(timeout=600)
        except subprocess.TimeoutExpired:
            os.killpg(process.pid, signal.SIGKILL)
            process.wait()
            code = "timeout"
    unchanged = all(digest(ROOT / source) == sha for source, sha in hashes.items())
    evidence = {
        "command": command, "exit": code, "elapsed": time.time() - start,
        "environment": {"CARGO_TARGET_DIR": str(out / "cargo")},
        "upstream": UPSTREAM, "upstream_commit": REVISION, "upstream_merged": False,
        "overlay": str(copied_patch), "overlay_sha256": digest(copied_patch),
        "original_atomic_sha256": original,
        "patched_atomic_sha256": digest(checker / "src/rt/atomic.rs"),
        "production_and_model_sources": hashes, "inputs_unchanged": unchanged,
        "log": str(out / "check.log"), "log_sha256": digest(out / "check.log"),
        "checker_sources": {str(p.relative_to(checker)): digest(p)
                            for p in sorted(checker.rglob("*")) if p.is_file()},
        "qualification": False,
    }
    (out / "result.json").write_text(json.dumps(evidence, indent=2) + "\n")
    if code != 0 or not unchanged:
        raise SystemExit("certificate model check failed; see retained output")
    print(f"Certificate models passed with the recorded checker overlay: {out}")


if __name__ == "__main__":
    main()
