import copy
import unittest
from ring_comparison import METHODS, MODEL, minimum_payload, validate_canonical, validate_peer, validate_warmup_check


def canonical():
    admission = {"schema_version": 1, "tier": "compiled", "class": "CanonicalRingWorkload",
                 "method": "small", "argument": 7, "expected_result": MODEL.expected("small", 7),
                 "configuration": {"objects": 40, "roots": 128, "mutators": 1, "helpers": 1, "edges_per_object": 3},
                 "limits": {key: 100000 for key in ("instructions", "request_polls", "cleanup_polls", "frames", "native_slices", "array_bytes")},
                 "recorder_capacity": 0, "require_compiled_backedges": True, "window_kind": "full_invocation_v1"}
    plan = {"schema_version": 1, "run_id": "test", "mode": "check", "warmup_invocations": 0, "samples": 1}
    identity = {"mode": "native", "tier": "compiled"}
    initial, after = [], []
    for cls, name, descriptor in METHODS:
        row = {"class": cls, "name": name, "descriptor": descriptor, "entries": 0, "resumes": 0, "backedges": 0}
        initial.append(row)
        result = dict(row)
        if name in {"small", "execute"}: result["entries"] = 1
        elif name == "inspect" or cls.endswith("$Node"): result["entries"] = 15
        if name == "execute": result.update(resumes=40, backedges=22)
        after.append(result)
    sample = {"elapsed_ns": None, "result": admission["expected_result"], "allocations": 30, "array_allocations": 15,
              "released_objects": 30, "payload_bytes_after_cleanup": 0, "catalog_bytes_after_drop": 0, "java_calls": 32,
              "compiled_entries": 32, "compiled_resumes": 40, "compiled_backedges": 22, "native_slices": 72, "helper_polls": 100}
    packet = {"schema_version": 1, "qualification": False, "diagnostic_only": True, "sample_mode": "check",
              "identity": copy.deepcopy(identity), "scope": "full_invocation_v1", "collector_lifecycle": "dedicated_heap_setup_body_cleanup_per_invocation",
              "service_recording_enabled": False, "warmup_invocations": 0, "samples": [sample], "clock": "clock_monotonic_ns",
              "clock_resolution_ns": 1, "preparation": {"admission_ns": None, "compilation_ns": None}, "program_teardown_ns": None,
              "method_profiles_before_warmup": copy.deepcopy(initial), "method_profiles_before_samples": initial,
              "method_profiles_after_samples": after, "gc_cycles_completed": None, "global_capture_complete": False,
              "scheduled_latency_available": False, "sustainable_throughput_available": False}
    return packet, identity, admission, plan


def peer():
    _, _, admission, plan = canonical()
    packet = {"schema_version": 1, "qualification": False, "diagnostic_only": True, "run_id": "test", "sample_mode": "check",
              "workload": "small", "steps": 7, "warmup_invocations": 0, "scope": "workload_call_deferred_gc_v1",
              "collector_lifecycle": "shared_hotspot_heap_deferred_gc", "clock": "java_system_nano_time",
              "samples": [{"result": admission["expected_result"], "elapsed_ns": None}], "actual_thread_allocated_bytes": None,
              "gc_observation_scope": "whole_sample_batch_including_driver_gaps",
              "gc_collection_counts": [{"name": "G1 Young Generation", "before": None, "after": None}],
              "in_invocation_gc_exposure_proven": False, "global_capture_complete": False,
              "scheduled_latency_available": False, "sustainable_throughput_available": False}
    return packet, plan


class RingComparisonTests(unittest.TestCase):
    def test_warmup_check_emits_only_correctness_receipt(self):
        plan = {"mode": "warmup-check", "warmup_invocations": 100, "samples": 30}
        identity = {"run_id": "warmup-test"}
        value = {"schema_version": 1, "qualification": False, "identity": identity,
                 "scope": "warmup_correctness_only", "completed_warmups": 100, "completed_checks": 30,
                 "expected_result": 42, "checksum_verified": True, "cleanup_verified": True,
                 "timings": None, "allocation_counters": None, "execution_counters": None,
                 "gc_counters": None, "method_profiles": None}
        validate_warmup_check(value, identity, 42, plan, True)
        for key in ("timings", "allocation_counters", "execution_counters", "gc_counters", "method_profiles"):
            bad = copy.deepcopy(value); bad[key] = []
            with self.assertRaises(ValueError): validate_warmup_check(bad, identity, 42, plan, True)

    def test_complete_census_and_untimed_check(self):
        args = canonical()
        result = validate_canonical(*args)
        self.assertTrue(result["compiled_method_census_valid"])
        self.assertFalse(result["qualification"])
        self.assertIsNone(result["gc_cycles_completed"])

    def test_missing_unexecuted_method_and_counter_forgery_reject(self):
        mutations = [lambda p: p["method_profiles_after_samples"].pop(0),
                     lambda p: p["method_profiles_after_samples"][0].update(entries=1),
                     lambda p: p["method_profiles_after_samples"][1].update(backedges=0),
                     lambda p: p["samples"][0].update(compiled_entries=31),
                     lambda p: p["samples"][0].update(compiled_resumes=39),
                     lambda p: p["samples"][0].update(released_objects=29),
                     lambda p: p["samples"][0].update(array_allocations=14),
                     lambda p: p["samples"][0].update(result=0)]
        for mutation in mutations:
            args = canonical(); mutation(args[0])
            with self.assertRaises(ValueError): validate_canonical(*args)

    def test_false_scope_and_performance_claims_reject(self):
        for key, value in (("scope", "entry_body_v1"), ("service_recording_enabled", True),
                           ("gc_cycles_completed", 0), ("qualification", True), ("global_capture_complete", True),
                           ("sustainable_throughput_available", True), ("scheduled_latency_available", True)):
            args = canonical(); args[0][key] = value
            with self.assertRaises(ValueError): validate_canonical(*args)
        args = canonical(); args[0]["samples"][0]["elapsed_ns"] = 1
        with self.assertRaises(ValueError): validate_canonical(*args)

    def test_peer_check_has_no_time_allocation_or_gc_counts(self):
        p, plan = peer(); validate_peer(p, "small", 7, plan)
        for mutate in (lambda p: p["samples"][0].update(elapsed_ns=10),
                       lambda p: p.update(actual_thread_allocated_bytes=10000),
                       lambda p: p["gc_collection_counts"][0].update(before=0, after=1)):
            p, plan = peer(); mutate(p)
            with self.assertRaises(ValueError): validate_peer(p, "small", 7, plan)

    def test_allocation_proof_is_separate_and_meets_payload_lower_bound(self):
        p, plan = peer(); plan["mode"] = p["sample_mode"] = "allocation"
        p["actual_thread_allocated_bytes"] = minimum_payload("small", 7)
        self.assertTrue(validate_peer(p, "small", 7, plan)["payload_allocation_lower_bound_verified"])
        p["actual_thread_allocated_bytes"] -= 1
        with self.assertRaises(ValueError): validate_peer(p, "small", 7, plan)

    def test_peer_cannot_invent_exact_gc_exposure(self):
        p, plan = peer(); p["in_invocation_gc_exposure_proven"] = True
        with self.assertRaises(ValueError): validate_peer(p, "small", 7, plan)


if __name__ == "__main__":
    unittest.main()
