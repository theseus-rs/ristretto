import copy
import json
import unittest
from canonical_service import MISSING, PROTOCOL, unique_object, validate_trace


def fixture():
    identity = {"run_id": "finite-run", "mode": "native", "tier": "interpreted",
                "host_process_id": 1234, "guest_process_id": 1234,
                **{key: "a" * 64 for key in ["host_executable_sha256", "binary_sha256",
                   "source_archive_sha256", "source_manifest_sha256", "workload_sha256",
                   "admission_sha256", "build_receipt_sha256", "launch_receipt_sha256"]}}
    admission = {"tier": "interpreted", "recorder_capacity": 16, "expected_result": 42,
                 "require_compiled_backedges": True}
    events = []
    def event(kind, op, start, end, outcome="complete", parent=None):
        events.append({"sequence": len(events), "parent": parent, "kind": kind,
                       "operation": op, "start_ns": start, "end_ns": end, "outcome": outcome})
    event("measurement", "measurement", 100, 900)
    event("request", "allocation", 200, 700)
    event("preparation", "allocation", 210, 300, parent=1)
    event("request_poll", "allocation", 310, 400, "pending", 1)
    event("helper_poll", "allocation", 420, 450, parent=1)
    event("request_poll", "allocation", 600, 650, parent=1)
    event("helper_poll", "maintenance", 920, 930)
    gaps = [{"request": 1, "start_ns": a, "end_ns": b, "after_pending": pending}
            for a, b, pending in [(200, 210, False), (300, 310, False), (400, 420, True),
                                  (450, 600, True), (650, 700, False)]]
    report = {"result": 42, "instructions": 10, "instruction_count_available": True,
              "allocations": 1, "array_allocations": 0, "reference_reads": 0, "reference_writes": 0, "calls": 1,
              "helper_polls": 2, "released_objects": 1, "payload_bytes_after_cleanup": 0,
              "catalog_bytes_after_drop": 0, "compiled_entries": 0, "compiled_resumes": 0,
              "compiled_backedges": 0, "native_slices": 0}
    trace = {"schema_version": 1, "protocol": PROTOCOL, "diagnostic_only": True,
             "qualification": False, "scope": "canonical_single_actor_services",
             "window_kind": "entry_body_v1", "identity": copy.deepcopy(identity),
             "binding": {"process_id": 1234, "run_id": "finite-run", "source_sha256": "a" * 64,
                         "binary_sha256": "a" * 64, "workload_sha256": "a" * 64,
                         "configuration_sha256": "a" * 64},
             "clock": "clock_monotonic_ns", "clock_resolution_ns": 1,
             "capture_start_ns": 50, "capture_end_ns": 1000, "capacity": 16,
             "overflow": False, "open_events": 0, "producer_issues": [],
             "instrumented_capture_complete": True, "global_capture_complete": False,
             "missing_sources": list(MISSING), "measurement_sequence": 0, "recorder_id": 1,
             "events": events, "gaps": gaps,
             "execution": {"status": "ok", "error": None, "report": report}}
    return trace, identity, admission


class CanonicalServiceTests(unittest.TestCase):
    def test_cycle_boundary_intervals_are_not_reclaimed_objects(self):
        trace, identity, admission = fixture()
        trace.update(schema_version=2, cycle_observation={
            "status": "complete", "scope": "canonical_primary_full_epochs", "measurement_sequence": 0,
            "observations": [{"lower": a, "upper": b} for a, b in [(2, 3), (3, 5), (8, 9), (9, 10)]],
            "bounds": {"lower": 3, "upper": 8}})
        result = validate_trace(trace, identity, admission)
        self.assertEqual(result["gc_cycle_bounds"], {"lower": 3, "upper": 8})
        self.assertIsNone(result["gc_cycles_completed"])
        self.assertFalse(result["qualification"])
        trace["cycle_observation"].update(
            observations=[{"lower": n, "upper": n} for n in [2, 2, 9, 9]],
            bounds={"lower": 7, "upper": 7})
        self.assertEqual(validate_trace(trace, identity, admission)["gc_cycles_completed"], 7)
        self.assertEqual(trace["execution"]["report"]["released_objects"], 1)

    def test_unavailable_cycle_sample_never_becomes_zero(self):
        trace, identity, admission = fixture()
        trace.update(schema_version=2, cycle_observation={
            "status": "complete", "scope": "canonical_primary_full_epochs", "measurement_sequence": 0,
            "observations": [None, *[{"lower": 2, "upper": 2}] * 3], "bounds": None})
        result = validate_trace(trace, identity, admission)
        self.assertIsNone(result["gc_cycles_completed"])
        self.assertFalse(result["gc_cycle_observation_valid"])
        trace["cycle_observation"]["bounds"] = {"lower": 0, "upper": 0}
        with self.assertRaises(ValueError):
            validate_trace(trace, identity, admission)

    def test_forged_or_nonmonotone_cycle_observations_reject(self):
        mutations = [
            lambda c: c.update(status="incomplete"),
            lambda c: c.update(scope="all_gc_cycles"),
            lambda c: c.update(measurement_sequence=1),
            lambda c: c.update(measurement_sequence=False),
            lambda c: c.update(observations=[]),
            lambda c: c.update(bounds={"lower": 0, "upper": 0}),
            lambda c: c["observations"][2].update(lower=1, upper=1),
            lambda c: c["observations"][1].update(lower=4, upper=3),
            lambda c: c["observations"][0].update(lower=True),
            lambda c: c["observations"][3].update(upper=1 << 64),
        ]
        for mutate in mutations:
            trace, identity, admission = fixture()
            trace.update(schema_version=2, cycle_observation={
                "status": "complete", "scope": "canonical_primary_full_epochs", "measurement_sequence": 0,
                "observations": [{"lower": n, "upper": n} for n in [2, 2, 9, 9]],
                "bounds": {"lower": 7, "upper": 7}})
            mutate(trace["cycle_observation"])
            with self.assertRaises(ValueError):
                validate_trace(trace, identity, admission)

    def test_pending_gaps_and_no_qualification(self):
        trace, identity, admission = fixture()
        result = validate_trace(trace, identity, admission)
        self.assertEqual(result["pending_gap_elapsed_ns"], 170)
        self.assertEqual(result["service_elapsed_ns"], 260)
        self.assertIsNone(result["gc_cycles_completed"])
        self.assertFalse(result["global_capture_complete"])
        self.assertFalse(result["zero_global_pause_proven"])

    def test_wasi_host_pid_is_distinct_from_unavailable_guest_pid(self):
        trace, identity, admission = fixture()
        identity.update(mode="wasi", guest_process_id=None, host_executable_sha256="b" * 64)
        trace["identity"] = copy.deepcopy(identity)
        trace["binding"]["process_id"] = None
        trace.update(clock="process_instant_ns", clock_resolution_ns=None)
        self.assertTrue(validate_trace(trace, identity, admission)["instrumented_capture_valid"])
        trace["identity"]["guest_process_id"] = identity["guest_process_id"] = 1234
        trace["binding"]["process_id"] = 1234
        with self.assertRaises(ValueError):
            validate_trace(trace, identity, admission)

    def test_compiled_requires_real_counters(self):
        trace, identity, admission = fixture()
        identity["tier"] = trace["identity"]["tier"] = admission["tier"] = "compiled"
        with self.assertRaises(ValueError):
            validate_trace(trace, identity, admission)
        trace["execution"]["report"].update(instructions=0, instruction_count_available=False,
                                           compiled_entries=1, compiled_resumes=2,
                                           compiled_backedges=1, native_slices=3)
        validate_trace(trace, identity, admission)

    def test_identity_substitution(self):
        for field in ["run_id", "source_archive_sha256", "binary_sha256", "workload_sha256",
                      "admission_sha256", "host_process_id", "launch_receipt_sha256"]:
            with self.subTest(field=field):
                trace, identity, admission = fixture()
                trace["identity"][field] = 9999 if field == "host_process_id" else "b" * 64
                with self.assertRaises(ValueError):
                    validate_trace(trace, identity, admission)

    def test_incomplete_capture_and_global_claims(self):
        for field, value in [("qualification", True), ("global_capture_complete", True),
                             ("missing_sources", []), ("overflow", True), ("open_events", 1),
                             ("producer_issues", ["abandoned_event"]),
                             ("instrumented_capture_complete", False), ("measurement_sequence", False),
                             ("capacity", 17), ("clock", "wall_clock")]:
            with self.subTest(field=field):
                trace, identity, admission = fixture()
                trace[field] = value
                with self.assertRaises(ValueError):
                    validate_trace(trace, identity, admission)

    def test_span_tampering(self):
        mutations = [(2, "parent", None), (2, "parent", 4), (2, "parent", 0),
                     (2, "start_ns", 99), (2, "end_ns", 710), (2, "outcome", "abandoned"),
                     (2, "end_ns", None), (2, "kind", "native"), (2, "operation", "maintenance"),
                     (3, "operation", "child_read"), (3, "outcome", "complete"),
                     (4, "operation", "maintenance"), (6, "operation", "allocation"), (4, "start_ns", 390), (5, "outcome", "pending"), (0, "end_ns", 620)]
        for index, field, value in mutations:
            with self.subTest(index=index, field=field):
                trace, identity, admission = fixture()
                trace["events"][index][field] = value
                with self.assertRaises(ValueError):
                    validate_trace(trace, identity, admission)

    def test_forged_gaps_or_progress(self):
        mutations = [lambda t: t["gaps"].clear(), lambda t: t["gaps"][0].update(after_pending=0),
                     lambda t: t["gaps"][2].update(after_pending=False),
                     lambda t: t["execution"]["report"].update(helper_polls=1),
                     lambda t: t["execution"]["report"].update(allocations=2),
                     lambda t: t["execution"]["report"].update(array_allocations=1),
                     lambda t: t["execution"]["report"].update(result=41),
                     lambda t: t["execution"]["report"].update(catalog_bytes_after_drop=1)]
        for mutate in mutations:
            trace, identity, admission = fixture()
            mutate(trace)
            with self.assertRaises(ValueError):
                validate_trace(trace, identity, admission)

    def test_duplicate_json_keys_reject(self):
        with self.assertRaises(ValueError):
            json.loads('{"qualification":true,"qualification":false}', object_pairs_hook=unique_object)


if __name__ == "__main__":
    unittest.main()
