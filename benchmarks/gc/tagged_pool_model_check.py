#!/usr/bin/env python3
"""Check generation-tagged slot models and faults with the recorded Loom RMW fix.

This isolated checker does not change workspace dependencies. State-only models
exclude pointer provenance, complete link operations and collector progress.
"""
import argparse
import json
import os
from pathlib import Path
import shutil
import subprocess

from catalog_model_check import run
from trace_certificate_check import ROOT, UPSTREAM, REVISION, digest

VARIANTS = [
    ("positive", None, None, None, None),
    ("ignore-generation-limit", "Self(AtomicU64::new((MAX_GENERATION - limit) << BITS))", "Self(AtomicU64::new(0))",
     "competing_claims_cannot_cross_a_configured_final_generation", "left: 1"),
    ("stale-generation", "            && old >> BITS == generation\n", "",
     "exact_generation_acquisition_excludes_reused_storage",
     "assertion failed: !state.acquire(generation)"),
    ("early-reuse", "            self.0.load(Ordering::Acquire);",
     "            self.0.load(Ordering::Acquire);\n"
     "            self.0.store((old >> BITS) << BITS, Ordering::Release);",
     "busy_state_excludes_acquisition_and_reuse_through_destruction",
     "state.claim().is_none()"),
    ("missing-final-acquire", "            self.0.load(Ordering::Acquire);", "",
     "final_release_acquires_prior_owner_writes", "left: 0"),
    ("missing-initial-acquire",
     ".compare_exchange(old, old + 1, Ordering::Acquire, Ordering::Relaxed)",
     ".compare_exchange(old, old + 1, Ordering::Relaxed, Ordering::Relaxed)",
     "acquisition_observes_initialized_value", "left: 0"),
]


def valid_model_result(code, log, test=None, witness=None):
    """Require an executed invariant failure, not a compile error with exit 101."""
    if test is None:
        return code == 0 and "test result: ok. 6 passed; 0 failed;" in log
    return (code == 101
            and f"test {test} ... FAILED" in log
            and "test result: FAILED. 0 passed; 1 failed;" in log
            and witness in log)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, required=True)
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
    original = digest(checker / "src/rt/atomic.rs")
    with (out / "patch.log").open("wb") as stream:
        subprocess.run(["patch", "-p1", "--batch", "--fuzz=0", "-i", str(patch)],
                       cwd=checker, stdout=stream, stderr=subprocess.STDOUT, check=True)
    source = ROOT / "ristretto_gc/validation/catalog/catalog-tagged-pool-prototype"
    inputs = {str(p.relative_to(ROOT)): digest(p) for p in sorted(source.rglob("*"))
              if p.is_file() and "target" not in p.relative_to(source).parts}
    sources, preparation, results = {}, [], []
    for name, old, new, test, witness in VARIANTS:
        destination = out / name
        destination.mkdir()
        for path in inputs:
            file = ROOT / path
            copied = destination / file.relative_to(source)
            copied.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(file, copied)
        manifest = destination / "Cargo.toml"
        manifest.write_text(manifest.read_text() + '\n[patch.crates-io]\nloom = { path = '
                            + json.dumps(str(checker)) + ' }\n')
        if old:
            state = destination / "src/state.rs"
            text = state.read_text()
            if text.count(old) != 1:
                raise SystemExit(f"{name}: mutation no longer matches the transition source")
            state.write_text(text.replace(old, new, 1))
        # Identically named packages must never share a target directory.
        environment = dict(os.environ, CARGO_TARGET_DIR=str(out / f"{name}-build"))
        result = run(["cargo", "generate-lockfile", "--offline", "--manifest-path", str(manifest)],
                     out / f"{name}-resolve.log", environment, 60)
        preparation.append(result)
        if result["exit_code"]:
            (out / "preparation.json").write_text(json.dumps(preparation, indent=2) + "\n")
            raise SystemExit("model dependency resolution failed")
        sources[name] = {str(p.relative_to(destination)): digest(p)
                         for p in sorted(destination.rglob("*")) if p.is_file()}
        command = ["cargo", "test", "--offline", "--locked", "--manifest-path",
                   str(manifest), "--test", "model"]
        if test:
            command += [test, "--", "--exact"]
        result = run(command, out / f"{name}.log", environment)
        result.update(name=name, negative=bool(test),
                      environment={"CARGO_TARGET_DIR": environment["CARGO_TARGET_DIR"]})
        result["valid"] = valid_model_result(result["exit_code"],
                                            Path(result["log"]).read_text(), test, witness)
        results.append(result)
        unchanged = (all(digest(ROOT / path) == sha for path, sha in inputs.items())
                     and all(digest(out / variant / path) == sha
                             for variant, files in sources.items() for path, sha in files.items()))
        evidence = {"upstream": UPSTREAM, "upstream_commit": REVISION, "upstream_merged": False,
                    "original_atomic_sha256": original,
                    "patched_atomic_sha256": digest(checker / "src/rt/atomic.rs"),
                    "checker_sources": {str(p.relative_to(checker)): digest(p)
                                        for p in sorted(checker.rglob("*")) if p.is_file()},
                    "inputs": inputs, "sources": sources, "inputs_unchanged": unchanged,
                    "preparation": preparation, "runs": results, "qualification": False}
        (out / "result.json").write_text(json.dumps(evidence, indent=2) + "\n")
        print(name, result["exit_code"], result["valid"], flush=True)
        if not result["valid"] or not unchanged:
            raise SystemExit("model check failed; see result.json")


if __name__ == "__main__":
    main()
