"""Anti-false-pass checks for actual-window counters and optional real capture replay."""
import json
import os
from pathlib import Path
import unittest
from unittest.mock import patch
from canonical_service import digest
from execution_measurement import packet, validate
from execution_measurement_artifacts import check


def fixture():
    methods = []
    for instance, name, entries, backedges in ((1, "kernel", 7, 35), (2, "outside", 0, 0)):
        methods.append({"instance": instance, "class": "GcBench", "name": name, "descriptor": "(I)I",
            "compiled_entries": entries, "compiled_backedges": backedges, "compiled_resumes": 0,
            "interpreter_entries": 0, "intrinsic_entries": 0, "queued_requests": 0,
            "compilation_attempts": 0, "compilation_successes": 0, "compilation_failures": 0,
            "compilation_ns": 0, "generated_code_bytes": 0, "present_at_start": True,
            "compiled_before_start": True, "code_bytes_at_start": 100, "code_bytes_at_end": 100})
    return {"schema_version": 1, "scope": "explicit_java_measurement", "diagnostic_only": True,
        "qualification": False, "atomic_global_cut": False, "all_actor_coverage": False, "state": "closed",
        "boundary_thread_id": 1, "limits": {"methods": 10, "functions_per_method": 2,
            "identity_bytes_per_method": 100, "retained_code_bytes": 1000},
        "evidence_losses": 0, "protocol_errors": 0, "retained_code_bytes": 200,
        "begin": {"observer_start_ns": 1, "snapshot_start_ns": 2, "snapshot_end_ns": 4, "timestamp_ns": 5, "observer_end_ns": 6},
        "end": {"observer_start_ns": 20, "timestamp_ns": 21, "snapshot_start_ns": 22, "snapshot_end_ns": 24, "observer_end_ns": 26},
        "interval": {"schema_version": 1, "diagnostic_only": True, "qualification": False,
            "scope": "checkpoint_interval", "label": "GcTelemetry explicit measurement",
            "boundary_span_ns": 22, "start_snapshot_ns": 2, "end_snapshot_ns": 2, "methods": methods}}


class ExecutionWindowTests(unittest.TestCase):
    def test_exact_compiled_ground_truth_remains_nonqualifying(self):
        result = validate(fixture(), 16, "probe", 7, True)
        self.assertTrue(result["required_call_coverage_compiled"])
        self.assertFalse(result["coverage_gate_passed"])
        self.assertFalse(result["qualification"])
        self.assertEqual(result["boundary_observer_ns"], 11)

    def test_open_missing_foreign_window_and_overclaims_reject(self):
        for mutate in [lambda c: c.update(state="open"), lambda c: c.update(begin=None),
                       lambda c: c.update(all_actor_coverage=True), lambda c: c.update(atomic_global_cut=True),
                       lambda c: c.update(qualification=True), lambda c: c.update(boundary_thread_id=0),
                       lambda c: c["begin"].update(timestamp_ns=7),
                       lambda c: c["begin"].update(snapshot_end_ns=6),
                       lambda c: c["end"].update(snapshot_start_ns=20),
                       lambda c: c["interval"].update(label="main invocation"),
                       lambda c: c["interval"].update(start_snapshot_ns=None),
                       lambda c: c["interval"].update(start_snapshot_ns=3)]:
            value = fixture()
            mutate(value)
            with self.subTest(mutate=mutate), self.assertRaises(ValueError):
                validate(value, 16, "probe", 7)

    def test_overflow_protocol_errors_and_bounds_reject(self):
        for mutate in [lambda c: c.update(evidence_losses=1), lambda c: c.update(protocol_errors=1),
                       lambda c: c.update(retained_code_bytes=1001), lambda c: c.update(retained_code_bytes=199), lambda c: c["limits"].update(methods=1),
                       lambda c: c["limits"].update(functions_per_method=17),
                       lambda c: c["limits"].update(identity_bytes_per_method=1)]:
            value = fixture()
            mutate(value)
            with self.subTest(mutate=mutate), self.assertRaises(ValueError):
                validate(value, 16, "probe", 7)

    def test_warmup_reporting_counter_and_instance_contamination_reject(self):
        for mutate in [lambda m: m[0].update(compiled_entries=16),
                       lambda m: m[0].update(compiled_entries=18),
                       lambda m: m[0].update(compiled_entries=27),
                       lambda m: m[1].update(compiled_entries=1),
                       lambda m: m[1].update(compiled_resumes=1),
                       lambda m: m[0].update(compiled_backedges=34),
                       lambda m: m[0].update(compiled_entries=True),
                       lambda m: m.append({**m[0], "instance": 3})]:
            value = fixture()
            mutate(value["interval"]["methods"])
            with self.subTest(mutate=mutate), self.assertRaises(ValueError):
                validate(value, 16, "probe", 7, True)

    def test_fallback_and_late_compilation_cannot_claim_ready_compiled_calls(self):
        for change in ({"interpreter_entries": 7, "compiled_entries": 0, "compiled_backedges": 0},
                       {"compiled_before_start": False}, {"compilation_attempts": 1}, {"compilation_failures": 1},
                       {"compilation_successes": 1}, {"queued_requests": 1}):
            value = fixture()
            value["interval"]["methods"][0].update(change)
            self.assertFalse(validate(value, 16, "probe", 7)["required_call_coverage_compiled"])
            with self.assertRaises(ValueError):
                validate(value, 16, "probe", 7, True)

    def test_roots_backedges_follow_actual_compiled_operation_entries(self):
        for compiled in (0, 1, 2):
            value = fixture()
            methods = value["interval"]["methods"]
            methods[0].update(name="operation", descriptor="(IJI)I", compiled_entries=compiled,
                              compiled_backedges=32 * compiled, interpreter_entries=2 - compiled)
            methods[1].update({"class": "GcBench$Histogram", "name": "record", "descriptor": "(J)V",
                               "compiled_entries": 4})
            methods.append({**methods[1], "class": "GcBench$Node", "name": "<init>",
                            "descriptor": "(ILGcBench$Node;)V", "instance": 3, "compiled_entries": 32})
            value["retained_code_bytes"] = 300
            if compiled == 0:
                methods[0].update(rejection_reason="unqualified_instruction", rejected_bytecode=[38, 1],
                                  compiled_before_start=False, code_bytes_at_start=0, code_bytes_at_end=0)
            with self.subTest(compiled=compiled):
                result = validate(value, 16, "roots", 2)
                self.assertEqual(compiled == 2, result["required_call_coverage_compiled"])
                row = result["required_methods"][0]
                self.assertEqual(32 * compiled, row["expected_compiled_backedges"])
                self.assertEqual(methods[0].get("rejected_bytecode"), row["rejected_bytecode"])
                methods[0]["compiled_backedges"] += 1
                with self.assertRaises(ValueError):
                    validate(value, 16, "roots", 2)

    def test_unusable_rejection_locations_fail_validation(self):
        for rejection in ([True, 1], [38, -1], [38, 256], [65535, 1], [1], "38:1"):
            value = fixture()
            value["interval"]["methods"][0]["rejected_bytecode"] = rejection
            with self.subTest(rejection=rejection), self.assertRaisesRegex(ValueError, "rejected bytecode"):
                validate(value, 16, "probe", 7)

    def test_duplicate_raw_keys_packets_and_missing_packet_reject(self):
        for stdout in ('CAP {"x":1,"x":2}', 'CAP {}\nCAP {}', ''):
            with self.assertRaises(ValueError):
                packet(stdout, "CAP ")


@unittest.skipUnless(os.environ.get("EXECUTION_WINDOW_CAPTURE"), "set EXECUTION_WINDOW_CAPTURE to replay real evidence")
class ActualCaptureMutations(unittest.TestCase):
    """Rehash mutated outputs/receipts: rejection must come from semantics, not stale file hashes."""
    def test_independent_real_capture_and_rehashed_negative_controls(self):
        root = Path(os.environ["EXECUTION_WINDOW_CAPTURE"]).resolve()
        self.assertTrue(check(root)["execution"]["measurement_counter_evidence_valid"])
        original_read = Path.read_text
        stdout = (root / "stdout.txt").read_text()
        original_run = json.loads((root / "run.json").read_text())
        def shifted_window(packet):
            for name in ("begin", "end"):
                for key in packet["capture"][name]:
                    packet["capture"][name][key] += 1
        mutations = [
            ("RISTRETTO_EXECUTION_MEASUREMENT ", shifted_window),
            ("RISTRETTO_EXECUTION_MEASUREMENT ", lambda p: p["identity"].update(process_id=p["identity"]["process_id"] + 1)),
            ("RISTRETTO_EXECUTION_MEASUREMENT ", lambda p: p["identity"].update(run_id="wrong-run")),
            ("RISTRETTO_EXECUTION_MEASUREMENT ", lambda p: p["identity"].update(source_archive_sha256="0" * 64)),
            ("RISTRETTO_EXECUTION_MEASUREMENT ", lambda p: p["capture"].update(state="open")),
            ("RISTRETTO_EXECUTION_MEASUREMENT ", lambda p: p["capture"].update(evidence_losses=1)),
            ("RISTRETTO_EXECUTION_MEASUREMENT ", lambda p: p["capture"]["begin"].update(timestamp_ns=p["capture"]["begin"]["timestamp_ns"] + 1)),
            ("GC_WINDOW_PROBE ", lambda p: p.update(checksum=71)),
            ("RISTRETTO_PAUSES ", lambda p: p.update(global_source_coverage_complete=True)),
        ]
        for prefix, mutate in mutations:
            changed = packet(stdout, prefix)
            mutate(changed)
            altered = "\n".join(prefix + json.dumps(changed) if line.startswith(prefix) else line for line in stdout.splitlines()) + "\n"
            import hashlib
            run = {**original_run, "stdout_sha256": hashlib.sha256(altered.encode()).hexdigest()}
            def read(path, *args, **kwargs):
                if path == root / "stdout.txt":
                    return altered
                if path == root / "run.json":
                    return json.dumps(run)
                return original_read(path, *args, **kwargs)
            real_digest = digest
            def hashed(path):
                return run["stdout_sha256"] if Path(path) == root / "stdout.txt" else real_digest(path)
            with self.subTest(prefix=prefix), patch.object(Path, "read_text", read), patch("execution_measurement_artifacts.digest", hashed):
                with self.assertRaises(ValueError):
                    check(root)


if __name__ == "__main__":
    unittest.main()
