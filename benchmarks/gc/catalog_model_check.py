#!/usr/bin/env python3
"""Check catalog/pool models and negative controls with the recorded Loom RMW fix.

This does not alter workspace dependencies or production atomics. The upstream
patch is the same unmerged PR recorded by trace_certificate_check.py. Model proofs
exclude the owning-pointer adapter and real payload/root/credit integration.
"""
import argparse
import json
import os
from pathlib import Path
import shutil
import signal
import subprocess
import time

from trace_certificate_check import ROOT, UPSTREAM, REVISION, digest


def run(command, output, environment, timeout=240):
    start = time.monotonic()
    with output.open("wb") as stream:
        process = subprocess.Popen(command, stdout=stream, stderr=subprocess.STDOUT,
                                   env=environment, start_new_session=True)
        try:
            code = process.wait(timeout=timeout)
            timed_out = False
        except subprocess.TimeoutExpired:
            os.killpg(process.pid, signal.SIGKILL)
            process.wait()
            code, timed_out = process.returncode, True
    return {"command": command, "exit_code": code, "timed_out": timed_out,
            "elapsed_seconds": time.monotonic() - start, "log": str(output),
            "log_sha256": digest(output)}


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
    patch = out / "loom-rmw-atomicity.patch"
    shutil.copy2(ROOT / "benchmarks/gc/loom-rmw-atomicity.patch", patch)
    original_atomic = digest(checker / "src/rt/atomic.rs")
    with (out / "patch.log").open("wb") as stream:
        subprocess.run(["patch", "-p1", "--batch", "--fuzz=0", "-i", str(patch)],
                       cwd=checker, stdout=stream, stderr=subprocess.STDOUT, check=True)
    packages = ["catalog-pool-core-source", "catalog-pooled-prototype"]
    inputs, preparation = {}, []
    environment = dict(os.environ, CARGO_TARGET_DIR=str(out / "build"))
    for name in packages:
        source = ROOT / "ristretto_gc/validation/catalog" / name
        destination = out / name
        files = [source / "Cargo.toml", source / "Cargo.lock",
                 *sorted((source / "src").glob("*.rs")), *sorted((source / "tests").glob("*.rs"))]
        for file in files:
            inputs[str(file.relative_to(ROOT))] = digest(file)
            copied = destination / file.relative_to(source)
            copied.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(file, copied)
        manifest = destination / "Cargo.toml"
        manifest.write_text(manifest.read_text() + '\n[patch.crates-io]\nloom = { path = '
                            + json.dumps(str(checker)) + ' }\n')
        result = run(["cargo", "generate-lockfile", "--offline", "--manifest-path", str(manifest)],
                     out / f"{name}-resolve.log", environment, 60)
        preparation.append(result)
        if result["exit_code"]:
            (out / "preparation.json").write_text(json.dumps(preparation, indent=2) + "\n")
            raise SystemExit("model dependency resolution failed; see preparation.json")
    variants = [
        ("early-reuse", "        fence(Ordering::Acquire);",
         "        fence(Ordering::Acquire);\n        self.occupied.store(false, Ordering::Release);",
         "zero_references_do_not_publish_reuse_before_destruction", "assertion failed: !state.claim()"),
        ("missing-acquire", "        fence(Ordering::Acquire);", "",
         "the_final_owner_acquires_prior_owner_writes", "left: 0"),
    ]
    for name, before, after, _, _ in variants:
        shutil.copytree(out / packages[0], out / name)
        state = out / name / "src/slot_state.rs"
        text = state.read_text()
        if text.count(before) != 1:
            raise SystemExit("negative control no longer matches the slot transition")
        state.write_text(text.replace(before, after, 1))
    frozen = {str(file.relative_to(out)): digest(file)
              for name in packages + [variant[0] for variant in variants]
              for file in (out / name).rglob("*") if file.is_file()}
    (out / "manifest.json").write_text(json.dumps(frozen, indent=2) + "\n")
    results = []
    for name, test, witness in [(name, None, None) for name in packages] + [
            (name, test, witness) for name, _, _, test, witness in variants]:
        command = ["cargo", "test", "--offline", "--locked", "--manifest-path",
                   str(out / name / "Cargo.toml"), "--test", "model"]
        if test:
            command += [test, "--", "--exact"]
        # Mutations retain distinct build directories even when their package
        # names match; later checks cannot overwrite an earlier test executable.
        run_environment = dict(environment, CARGO_TARGET_DIR=str(out / f"{name}-build"))
        result = run(command, out / f"{name}.log", run_environment)
        result["environment"] = {"CARGO_TARGET_DIR": run_environment["CARGO_TARGET_DIR"]}
        result["negative"] = bool(witness)
        result["valid"] = (result["exit_code"] == 101 and witness in Path(result["log"]).read_text()) \
            if witness else result["exit_code"] == 0
        results.append(result)
        unchanged = all(digest(ROOT / file) == sha for file, sha in inputs.items()) \
            and all(digest(out / file) == sha for file, sha in frozen.items())
        evidence = {"upstream": UPSTREAM, "upstream_commit": REVISION, "upstream_merged": False,
                    "original_atomic_sha256": original_atomic,
                    "patched_atomic_sha256": digest(checker / "src/rt/atomic.rs"),
                    "overlay_sha256": digest(patch), "inputs": inputs, "inputs_unchanged": unchanged,
                    "checker_sources": {str(p.relative_to(checker)): digest(p)
                                        for p in sorted(checker.rglob("*")) if p.is_file()},
                    "preparation": preparation, "runs": results, "qualification": False}
        (out / "result.json").write_text(json.dumps(evidence, indent=2) + "\n")
        print(name, result["exit_code"], result["valid"], flush=True)
        if not result["valid"] or not unchanged:
            raise SystemExit("model check failed; see result.json")


if __name__ == "__main__":
    main()
