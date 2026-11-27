"""Partial-stream contract tests; no fixtures here are qualification evidence."""
import copy
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

from acceptance import validate_pauses
from pauses import RECORDER, observation, validate_trace, workload_digest
from report import summarize
from test_qualification import packet, policy


def fixture():
    identity = {"run_id": "unique-launch", "policy_sha256": "a" * 64,
                "source_archive_sha256": "b" * 64, "binary_sha256": "c" * 64,
                "workload_sha256": "d" * 64, "process_id": 123, "recorder_capacity": 8,
                "runtime": "ristretto-baseline", "workload": "allocation",
                "duration_ms": 1000, "warmup_ms": 1000, "rate": 0, "seed": 42}
    return {"schema_version": 2, "diagnostic_only": True, "qualification": False,
            "identity": identity, "recorder": RECORDER,
            "scope": "instrumented_collector_lifetime", "clock": "clock_monotonic_ns",
            "capture_start_ns": 1, "capture_end_ns": 10_000_000, "clock_resolution_ns": 1,
            "capture_complete": False, "instrumented_capture_complete": True,
            "global_source_coverage_complete": False, "global_stop_sources": ["publication"],
            "uncovered_global_stop_sources": ["allocation_recovery", "gc_safepoint"],
            "measurement_window": None,
            "measurement_boundaries": {"state": "missing", "errors": 0, "begin": None, "end": None},
            "gc_exposure": None, "mutator_admission_waits_recorded": False,
            "publication_attempts": 2, "publication_acquisitions": 1,
            "publication_failed_attempts": 1, "dropped_events": 0,
            "open_events": 0, "clock_errors": 0,
            "events": [{"sequence": 0, "source": "publication", "kind": "equivalent_global_stall",
                        "requested_ns": 100, "acquired_ns": 200, "released_ns": 300}]}


def window(trace, start, end, counts=(10, 10, 12, 12)):
    trace["measurement_window"] = {"start_ns": start, "end_ns": end}
    trace["measurement_boundaries"] = {"state": "closed", "errors": 0,
        "begin": {"timestamp_ns": start, "observer_start_ns": start, "observer_end_ns": start,
                  "collections_before": counts[0], "collections_after": counts[1]},
        "end": {"timestamp_ns": end, "observer_start_ns": end, "observer_end_ns": end,
                "collections_before": counts[2], "collections_after": counts[3]}}
    trace["gc_exposure"] = {"scope": "measurement", "collections_completed_lower_bound": counts[2]-counts[1],
                            "collections_completed_upper_bound": counts[3]-counts[0]}
    return trace


class PauseTraceTests(unittest.TestCase):
    def test_real_exec_preserves_recorded_process_identity(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "identity.json"
            env = dict(os.environ, RISTRETTO_QUALIFICATION_RUN_ID="receipt-test")
            result = subprocess.run([sys.executable, str(Path(__file__).with_name("process_identity.py")),
                str(path), sys.executable, "-c", "import os; print(os.getpid())"],
                env=env, check=True, text=True, capture_output=True)
            self.assertEqual(json.loads(path.read_text()),
                             {"process_id": int(result.stdout), "run_id": "receipt-test"})

    def test_class_manifest_binds_nested_classes_and_paths(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root / "GcBench.class").write_bytes(b"main")
            (root / "nested").mkdir()
            (root / "nested/A.class").write_bytes(b"nested")
            raw = b"GcBench.class\0" + hashlib.sha256(b"main").hexdigest().encode() + b"\n"
            raw += b"nested/A.class\0" + hashlib.sha256(b"nested").hexdigest().encode() + b"\n"
            before = workload_digest(root)
            self.assertEqual(before, hashlib.sha256(raw).hexdigest())
            (root / "nested/A.class").write_bytes(b"changed")
            self.assertNotEqual(before, workload_digest(root))

    def test_empty_manifest_is_not_a_workload_binding(self):
        with tempfile.TemporaryDirectory() as tmp, self.assertRaises(ValueError):
            workload_digest(tmp)

    def test_partial_trace_remains_ineligible_and_window_missing(self):
        trace = fixture()
        result = validate_trace(trace, trace["identity"])
        self.assertFalse(result["qualification"])
        self.assertFalse(result["capture_complete"])
        self.assertFalse(result["source_coverage_complete"])
        self.assertIsNone(result["measurement"])
        self.assertEqual(result["max_observed_continuous_stall_ns"], 200)

    def test_identity_substitution_is_rejected(self):
        original = fixture()
        for key in original["identity"]:
            with self.subTest(field=key), self.assertRaisesRegex(ValueError, "identity"):
                trace = copy.deepcopy(original)
                trace["identity"][key] = "different"
                validate_trace(trace, original["identity"])

    def test_no_events_is_unavailable_global_maximum_not_zero_stw(self):
        trace = fixture()
        trace.update(events=[], publication_acquisitions=0, publication_attempts=1)
        result = validate_trace(trace, trace["identity"])
        self.assertIsNone(result["max_observed_continuous_stall_ns"])
        self.assertFalse(result["capture_complete"])
        with self.assertRaises(ValueError):
            validate_pauses(trace, policy()["coordination_audit"], (1, 10_000_000))

    def test_explicit_missing_and_duplicate_stream(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp)
            (path / "stdout.txt").write_text("no telemetry\n")
            self.assertIsNone(observation(path))
            text = "RISTRETTO_PAUSES " + json.dumps(fixture()) + "\n"
            (path / "stdout.txt").write_text(text * 2)
            with self.assertRaises(ValueError):
                observation(path)

    def test_overflow_and_open_events_retained_as_incomplete(self):
        trace = fixture()
        trace["identity"]["recorder_capacity"] = 1
        trace.update(publication_attempts=3, publication_acquisitions=2,
                     dropped_events=1, instrumented_capture_complete=False)
        self.assertEqual(validate_trace(trace, trace["identity"])["dropped_events"], 1)
        trace["events"][0]["released_ns"] = None
        trace["open_events"] = 1
        self.assertEqual(validate_trace(trace, trace["identity"])["open_events"], 1)
        trace["instrumented_capture_complete"] = True
        with self.assertRaises(ValueError):
            validate_trace(trace, trace["identity"])

    def test_request_to_release_budget_is_exclusive(self):
        for duration, violated in [(999_999, False), (1_000_000, True), (1_000_001, True)]:
            trace = fixture()
            trace["events"][0]["acquired_ns"] = 100 + duration - 1
            trace["events"][0]["released_ns"] = 100 + duration
            self.assertEqual(validate_trace(trace, trace["identity"])["observed_budget_violation"], violated)

    def test_measurement_boundary_crossing_preserves_full_duration(self):
        trace = fixture()
        trace["events"][0]["released_ns"] = 2_000_100
        window(trace, 1_999_999, 2_000_000)
        observed = validate_trace(trace, trace["identity"])
        self.assertTrue(observed["observed_budget_violation"])
        self.assertEqual(observed["measurement"]["boundary_crossings"], 1)
        self.assertEqual(observed["measurement"]["max_full_event_ns"], 2_000_000)

    def test_measured_gc_excludes_warmup_and_bounds_boundary_races(self):
        trace = window(fixture(), 500, 1000, (10, 11, 12, 13))
        checked = validate_trace(trace, trace["identity"], 500)
        self.assertEqual(checked["measurement"]["gc_exposure"], {
            "scope": "measurement", "collections_completed_lower_bound": 1,
            "collections_completed_upper_bound": 3})
        with self.assertRaisesRegex(ValueError, "Java duration"):
            validate_trace(trace, trace["identity"], 499)
        trace["gc_exposure"]["collections_completed_lower_bound"] = 12
        with self.assertRaisesRegex(ValueError, "GC"):
            validate_trace(trace, trace["identity"], 500)

    def test_missing_duplicate_and_out_of_order_boundaries_fail_measured_binding(self):
        trace = fixture()
        with self.assertRaisesRegex(ValueError, "boundaries missing"):
            validate_trace(trace, trace["identity"], 500)
        for change in [lambda t: t["measurement_boundaries"].update(errors=1),
                       lambda t: t["measurement_boundaries"].update(state="open"),
                       lambda t: t["measurement_boundaries"]["end"].update(observer_start_ns=1)]:
            changed = window(fixture(), 500, 1000)
            change(changed)
            with self.assertRaises(ValueError):
                validate_trace(changed, changed["identity"], 500)

    def test_corrupt_counters_order_scope_and_false_complete_claims(self):
        for field, value in [("publication_attempts", 0), ("publication_acquisitions", 2),
                             ("open_events", 1), ("scope", "whole_process"),
                             ("capture_complete", True), ("global_source_coverage_complete", True),
                             ("mutator_admission_waits_recorded", True), ("capture_end_ns", 200)]:
            trace = fixture(); trace[field] = value
            with self.subTest(field=field), self.assertRaises(ValueError):
                validate_trace(trace, trace["identity"])

    def test_diagnostic_markers_cannot_be_relabelled_into_pause_acceptance(self):
        valid = packet("id", "a" * 64, [], 1, 10**9)["pauses"]
        for change in [{"diagnostic_only": True}, {"qualification": False}]:
            with self.assertRaises(ValueError):
                validate_pauses(dict(valid, **change), policy()["coordination_audit"], (0, 3 * 10**9))

    def test_diagnostic_rows_do_not_enter_timing_groups(self):
        row = {"status": "ok", "runtime": "ristretto-baseline", "workload": "allocation",
               "rate": 0, "directory": "diagnostic", "pause_evidence": fixture()}
        summary = summarize([row])
        self.assertEqual(summary["groups"], [])
        self.assertEqual(len(summary["diagnostics"]), 1)


if __name__ == "__main__":
    unittest.main()
