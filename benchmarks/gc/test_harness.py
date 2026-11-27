"""Tests for false-positive benchmark results and process isolation."""
import os
import json
from pathlib import Path
import sys
import tempfile
import tarfile
import unittest
from unittest.mock import patch
from types import SimpleNamespace

from mechanisms import paired_interval
from qualify import evaluate
from cgroups import Budget
from allocations import validate as validate_allocations
from identities import validate as validate_identities

from report import expected_checksum, quantile, summarize, validate
from run import command_for, execute, freeze_build, freeze_sources, java_major, gc_observation, validate_execution_mode


def measurement():
    return {"schema": 1, "workload": "allocation", "rate": 10, "seed": 42,
            "completed": 3, "offered": 10, "unfinished": 7, "duration_ns": 10**9,
            "elapsed_ns": 10**9, "checksum": 6, "service": {"max_ns": 2000, "buckets_us": [[1, 2], [2, 1]]},
            "response": {"max_ns": 3000, "buckets_us": [[1, 2], [3, 1]]}}


class ProtocolTests(unittest.TestCase):
    def test_source_freeze_includes_nested_package_locks_and_excludes_outputs(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            files = ["Cargo.lock", "catalog/Cargo.lock", "catalog/Cargo.toml",
                     "catalog/src/lib.rs", "results/generated.py", "unrelated.bin"]
            for name in files:
                path = root / name
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text(name)
            with patch("run.ROOT", root), patch("run.subprocess.check_output",
                    return_value=("\0".join(files) + "\0").encode()):
                hashes = freeze_sources(root / "results")
            self.assertEqual(set(hashes), set(files[:4]))
            with tarfile.open(root / "results/sources.tar.gz") as archive:
                self.assertEqual(set(archive.getnames()), set(hashes))
                for name in hashes:
                    self.assertEqual(archive.extractfile(name).read(), name.encode())

    def test_execution_profiles_cannot_enter_timing_comparisons(self):
        with tempfile.TemporaryDirectory() as temp:
            path = Path(temp)
            (path / "stdout.txt").write_text('RISTRETTO_EXECUTION {}\nRISTRETTO_GC {"collections_completed":1,"jit_enabled":true}\n')
            observation = gc_observation(path)
            for runtime in ["ristretto-candidate-jit", "wasm-candidate"]:
                with self.assertRaisesRegex(ValueError, "diagnostic"):
                    validate_execution_mode(runtime, observation)

    def test_stock_comparisons_preserve_policy_and_verify_the_observed_admission(self):
        options = SimpleNamespace(jdk=Path("/jdk"), heap_mib=64, workers=2, wasmtime="/wt",
                                  workload="allocation", seconds=1, warmup=1, rate=0, seed=1)
        args = options, Path("/classes"), Path("/out"), Path("/native"), Path("/module")
        for runtime in ("ristretto-stock-counted-jit", "wasm-stock-counted-before"):
            command = command_for(runtime, *args, Path("/old-native"), Path("/old-module"))
            self.assertIn("stock-counted-jit" if runtime.startswith("ristretto") else "stock-counted", command)
            for actual in (None, False, 1):
                with self.assertRaisesRegex(ValueError, "stock collector policy"):
                    validate_execution_mode(runtime, {"ristretto": {"collector_policy": "stock-counted",
                                            "concurrent_publication": actual}})
        for actual in (True, False):
            validate_execution_mode("wasm-stock", {"ristretto": {"collector_policy": "stock",
                                                                 "concurrent_publication": actual}})

    def test_identity_screen_rejects_lost_ids_and_invalid_worker_timings(self):
        data = {'producers': 2, 'per_worker': 10, 'operations': 20, 'unique': True,
                'wall_ns': 100, 'worker_ns': [80, 90]}
        validate_identities(data, 2, 10)
        for change in [{'unique': False}, {'operations': 19}, {'worker_ns': [80]},
                       {'worker_ns': [80, 101]}, {'wall_ns': 0}]:
            with self.assertRaises(ValueError):
                validate_identities({**data, **change}, 2, 10)

    def test_prior_build_labels_preserve_policy_and_select_the_requested_artifact(self):
        options = SimpleNamespace(jdk=Path("/jdk"), heap_mib=64, workers=2,
                                  workload="roots", seconds=2, warmup=1, rate=0, seed=42,
                                  wasmtime=Path("/wasmtime"))
        args = [options, Path("/classes"), Path("/run"), Path("/current"), Path("/current.wasm")]
        for policy in ["baseline", "candidate"]:
            native = command_for(f"ristretto-{policy}-before", *args, Path("/prior"), Path("/prior.wasm"))
            self.assertEqual((native[0], native[5]), ("/prior", policy))
            wasm = command_for(f"wasm-{policy}-before", *args, Path("/prior"), Path("/prior.wasm"))
            self.assertIn("/prior.wasm", wasm)
            self.assertEqual(wasm[wasm.index("/prior.wasm") + 5], policy)
        with self.assertRaises(ValueError):
            command_for("ristretto-candidate-before", *args)
        with self.assertRaises(ValueError):
            command_for("wasm-candidate-before", *args)

    def test_revision_comparisons_reject_mismatched_build_sources(self):
        with tempfile.TemporaryDirectory() as temp:
            path = Path(temp)
            binary = path / "program"
            binary.write_bytes(b"binary")
            (path / "sources.tar.gz").write_bytes(b"sources")
            (path / "program.provenance.json").write_text(json.dumps({
                "binary_sha256": "wrong", "source_archive_sha256": "wrong"}))
            with self.assertRaisesRegex(ValueError, "provenance"):
                freeze_build(binary, path / "copy")
    def test_allocation_counts_reject_corruption_and_invalid_counters(self):
        data = {"policy": "candidate", "operations": 2, "checksum": 84, "teardown_live": 0,
                "allocations": 6, "allocated_bytes": 1024, "deallocations": 2, "released_bytes": 128,
                "collection_allocations": 0, "collection_allocated_bytes": 0,
                "collection_deallocations": 4, "collection_released_bytes": 896}
        validate_allocations(data, "candidate", 2)
        with self.assertRaisesRegex(ValueError, "mode"):
            validate_allocations(data, "candidate", 2, "weak_retention")
        retained = {**data, "mode": "weak_retention", "retained_after_collection_bytes": 256,
                    "released_on_weak_drop_bytes": 32}
        validate_allocations(retained, "candidate", 2, "weak_retention")
        for key in ["retained_after_collection_bytes", "released_on_weak_drop_bytes"]:
            with self.assertRaises(ValueError):
                validate_allocations({**retained, key: -1}, "candidate", 2, "weak_retention")
        for key, value in [("checksum", 42), ("teardown_live", 1), ("allocations", -1), ("allocations", True)]:
            with self.assertRaises(ValueError):
                validate_allocations({**data, key: value}, "candidate", 2)

    def test_jit_and_interpreter_labels_select_distinct_native_modes(self):
        options = SimpleNamespace(jdk=Path("/jdk"), heap_mib=64, workers=2,
                                  workload="allocation", seconds=2, warmup=1, rate=0, seed=42)
        for policy in ["baseline", "candidate", "baseline-jit", "candidate-jit"]:
            command = command_for(f"ristretto-{policy}", options, Path("/classes"),
                                  Path("/run"), Path("/gc_compare"), Path("/gc_compare.wasm"))
            self.assertEqual(command[5], policy)

    def test_jit_label_requires_explicit_vm_confirmation(self):
        validate_execution_mode("ristretto-candidate-jit", {"ristretto": {"jit_enabled": True}})
        validate_execution_mode("ristretto-candidate-jit-before", {"ristretto": {"jit_enabled": True}})
        with self.assertRaises(ValueError):
            validate_execution_mode("ristretto-candidate-jit-before", {"ristretto": {"jit_enabled": False}})
        validate_execution_mode("ristretto-candidate", {})  # older interpreter-only driver
        for observed in [{}, {"ristretto": {"jit_enabled": False}}, {"ristretto": {"jit_enabled": 1}}]:
            with self.assertRaises(ValueError):
                validate_execution_mode("ristretto-candidate-jit", observed)
        with self.assertRaises(ValueError):
            validate_execution_mode("ristretto-candidate", {"ristretto": {"jit_enabled": True}})

    def test_checksum_matches_independent_operation_sequence(self):
        for count in [1, 127, 128, 129, 1029]:
            marks = [n & 127 for n in range(count)]
            self.assertEqual(expected_checksum("roots", count), sum(sum(m + j for j in range(16)) for m in marks))
            self.assertEqual(expected_checksum("allocation", count), sum(2 * m for m in marks))

    def test_overload_is_retained_not_silently_accepted_as_completed(self):
        data = measurement()
        validate(data, "allocation", 10, 1, 42)
        data["unfinished"] = 0
        with self.assertRaisesRegex(ValueError, "unfinished"):
            validate(data, "allocation", 10, 1, 42)

    def test_reject_corruption_and_missing_samples(self):
        for mutate in [lambda d: d.update(checksum=7), lambda d: d["service"].update(buckets_us=[[1, 1]])]:
            data = measurement()
            mutate(data)
            with self.assertRaises(ValueError):
                validate(data, "allocation", 10, 1, 42)

    def test_extreme_tail_uses_ceiling_rank(self):
        self.assertEqual(quantile({"buckets_us": [[1, 9999], [5000, 1]]}, 9999), 1)
        self.assertEqual(quantile({"buckets_us": [[1, 9998], [5000, 2]]}, 9999), 5000)

    def test_failures_remain_visible(self):
        failed = {"runtime": "zgc", "workload": "mixed", "rate": 10, "status": "timeout", "directory": "run-0"}
        result = summarize([failed])
        self.assertEqual(len(result["failures"]), 1)
        self.assertEqual(result["groups"], [])
        self.assertFalse(result["qualification"])

    def test_version_requires_explicit_jdk_number(self):
        self.assertEqual(java_major('openjdk version "25.0.5"'), 25)
        with self.assertRaises(ValueError):
            java_major("unknown")

    def test_gc_phase_starts_do_not_double_count_pause_durations(self):
        with tempfile.TemporaryDirectory() as temp:
            path = Path(temp)
            (path / "gc.log").write_text("[gc,start] GC(0) Pause Young\n[gc] GC(0) Pause Young 32M->4M(64M) 1.234ms\n")
            result = gc_observation(path)
            self.assertTrue(result["activity_observed"])
            self.assertIsNone(result["global_pause_ms"])
            self.assertEqual(result["gc_phase_pause_ms"]["count"], 1)
            self.assertEqual(result["gc_phase_pause_ms"]["max"], 1.234)

    def test_paired_interval_resamples_processes_and_rejects_unpaired_claims(self):
        ci = paired_interval([(10, 5), (20, 10), (30, 15)], draws=200)
        self.assertEqual((ci["low"], ci["median"], ci["high"]), (.5, .5, .5))
        with self.assertRaises(ValueError):
            paired_interval([(10, 5)])
        with self.assertRaises(ValueError):
            paired_interval([(10, 0), (20, 10)])

    def test_qualification_rejects_incomplete_evidence(self):
        with tempfile.TemporaryDirectory() as temp:
            path = Path(temp)
            (path / "metadata.json").write_text(json.dumps({"options": {
                "workloads": ["allocation"], "rates": [0, 10], "repeats": 2,
                "seconds": 1, "seed": 42}}))
            (path / "integrity.json").write_text('{"inputs_unchanged": true}')
            row = {"runtime": "ristretto-candidate", "workload": "allocation", "rate": 10,
                   "repeat": 0, "status": "ok", "measurement": measurement(),
                   "directory": "run-0", "target": "native"}
            (path / "raw.jsonl").write_text(json.dumps(row) + "\n")
            result = evaluate(path)
            self.assertFalse(result["short_comparison_gate"])
            self.assertFalse(result["qualification"])
            self.assertTrue(any("unfinished" in reason for reason in result["failures"]))
            self.assertTrue(any("five complete" in reason for reason in result["failures"]))
            self.assertTrue(any("resource" in reason for reason in result["failures"]))

    def test_cgroup_budget_never_silently_falls_back(self):
        with tempfile.TemporaryDirectory() as temp:
            path = Path(temp)
            (path / "cgroup.subtree_control").write_text("cpu")
            with self.assertRaisesRegex(ValueError, "cpu and memory"):
                Budget(path, 1024, 100000)
            self.assertEqual(list(path.iterdir()), [path / "cgroup.subtree_control"])

    @unittest.skipUnless(sys.platform == "linux", "Linux resource capture")
    def test_timeout_reaps_process_and_preserves_output(self):
        with tempfile.TemporaryDirectory() as temp:
            path = Path(temp)
            result = execute([sys.executable, "-c", "import time; print('started',flush=True); time.sleep(30)"],
                             path, .2, {min(os.sched_getaffinity(0))}, dict(os.environ))
            self.assertEqual(result["status"], "timeout")
            self.assertIn("started", (path / "stdout.txt").read_text())


if __name__ == "__main__":
    unittest.main()
