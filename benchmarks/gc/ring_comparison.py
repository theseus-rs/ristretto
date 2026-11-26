"""Strict finite ring comparison checks. None of these packets qualifies GC."""
import importlib.util
from pathlib import Path
from canonical_service import fields, integer

_model_path = Path(__file__).with_name("canonical-ring-oracle") / "verify_oracle.py"
_spec = importlib.util.spec_from_file_location("ring_reference_model", _model_path)
MODEL = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(MODEL)
SIZES = MODEL.CONFIGURATIONS
METHODS = sorted([
    ("CanonicalRingWorkload", "<init>", "()V"),
    ("CanonicalRingWorkload", "small", "(I)I"),
    ("CanonicalRingWorkload", "medium", "(I)I"),
    ("CanonicalRingWorkload", "large", "(I)I"),
    ("CanonicalRingWorkload", "inspect", "([BI)I"),
    ("CanonicalRingWorkload", "execute", "(IIII)I"),
    ("CanonicalRingWorkload$Node", "<init>", "(III)V"),
])


def minimum_payload(workload, steps):
    retained, base, mask = SIZES[workload]
    return sum(base + (generation & mask) for generation in range(retained + steps))


def validate_plan(plan, peer=False):
    if plan.get("schema_version") == 2:
        from scheduled_ring_comparison import validate_plan as scheduled_plan
        return scheduled_plan(plan)
    fields(plan, "schema_version run_id mode warmup_invocations samples", "sample plan")
    allowed = {"check", "warmup-check", "measure", "allocation"} if peer else {"check", "warmup-check", "measure"}
    if type(plan["schema_version"]) is not int or plan["schema_version"] != 1 or plan["mode"] not in allowed:
        raise ValueError("unsupported sample plan")
    if not isinstance(plan["run_id"], str) or not plan["run_id"]:
        raise ValueError("missing run identity")
    warm = integer(plan["warmup_invocations"], "warmup invocations")
    samples = integer(plan["samples"], "samples", 1)
    if warm > 128 or samples > 128 or (plan["mode"] == "check" and (warm != 0 or samples != 1)):
        raise ValueError("sample bounds; check mode requires one untimed invocation")
    return plan


def validate_warmup_check(value, identity, expected, plan, canonical):
    fields(value, "schema_version qualification identity scope completed_warmups completed_checks expected_result "
           "checksum_verified cleanup_verified timings allocation_counters execution_counters gc_counters method_profiles", "warmup check")
    import json
    if (plan["mode"] != "warmup-check" or type(value["schema_version"]) is not int or value["schema_version"] != 1
            or value["qualification"] is not False or value["scope"] != "warmup_correctness_only"
            or json.dumps(value["identity"], sort_keys=True) != json.dumps(identity, sort_keys=True)
            or value["completed_warmups"] != plan["warmup_invocations"] or type(value["completed_warmups"]) is not int
            or value["completed_checks"] != plan["samples"] or type(value["completed_checks"]) is not int
            or value["expected_result"] != expected or type(value["expected_result"]) is not int
            or value["checksum_verified"] is not True or value["cleanup_verified"] is not (True if canonical else None)
            or any(value[key] is not None for key in ("timings", "allocation_counters", "execution_counters", "gc_counters", "method_profiles"))):
        raise ValueError("warmup check failed or emitted measurement data")
    return {"valid": True, "scope": "warmup_correctness_only", "qualification": False,
            "completed_warmups": plan["warmup_invocations"], "completed_checks": plan["samples"],
            "timing_samples_available": False, "allocator_counts_available": False}


def validate_admission(admission):
    shared = type(admission.get("schema_version")) is int and admission["schema_version"] in (2, 3)
    fields(admission, "schema_version tier class method argument configuration limits recorder_capacity "
           "expected_result require_compiled_backedges window_kind" +
           (" max_payload_bytes heap_cleanup_polls minimum_primary_cycles" if shared else ""), "ring admission")
    if (type(admission["schema_version"]) is not int or admission["schema_version"] not in (1, 2, 3)
            or admission["tier"] not in {"interpreted", "compiled"}
            or admission["class"] != "CanonicalRingWorkload" or admission["method"] not in SIZES
            or admission["window_kind"] != ("shared_heap_scheduled_v1" if admission["schema_version"] == 3 else "shared_heap_invocation_v1" if shared else "full_invocation_v1")
            or type(admission["recorder_capacity"]) is not int or admission["recorder_capacity"] != 0
            or admission["require_compiled_backedges"] is not True):
        raise ValueError("unsupported ring admission")
    if shared:
        if integer(admission["max_payload_bytes"], "payload ceiling", 1) != 64 * 1024 * 1024:
            raise ValueError("shared protocol requires a 64 MiB managed payload ceiling")
        integer(admission["heap_cleanup_polls"], "whole-heap cleanup polls", 1)
        if integer(admission["minimum_primary_cycles"], "primary-cycle lower bound", 1) != 1:
            raise ValueError("shared protocol requires observed sample-batch primary-cycle exposure")
    steps = integer(admission["argument"], "steps")
    if steps > 1_000_000:
        raise ValueError("finite ring step cap exceeded")
    if type(admission["expected_result"]) is not int or admission["expected_result"] != MODEL.expected(admission["method"], steps):
        raise ValueError("expected result differs from independent index-ring model")
    fields(admission["configuration"], "objects roots mutators helpers edges_per_object", "configuration")
    for key, value in admission["configuration"].items():
        integer(value, key, 1)
    if admission["configuration"]["mutators"] != 1 or admission["configuration"]["helpers"] != 1:
        raise ValueError("only the admitted single-mutator helper configuration is supported")
    fields(admission["limits"], "instructions request_polls cleanup_polls frames native_slices array_bytes", "limits")
    for key, value in admission["limits"].items():
        integer(value, key, 1)
    return admission


def _profiles(rows, workload, steps, invocations):
    if not isinstance(rows, list) or len(rows) != len(METHODS):
        raise ValueError("incomplete compiled method census")
    retained = SIZES[workload][0]
    for row, identity in zip(rows, METHODS):
        fields(row, "class name descriptor entries resumes backedges", "method profile")
        if tuple(row[key] for key in ("class", "name", "descriptor")) != identity:
            raise ValueError("compiled method identity missing, duplicated or reordered")
        for key in ("entries", "resumes", "backedges"):
            integer(row[key], "method " + key)
        cls, method, _ = identity
        entries = (1 if method in {workload, "execute"} else
                   retained + steps if method == "inspect" or cls.endswith("$Node") else 0)
        backedges = 2 * retained + steps - 1 if method == "execute" else 0
        if row["entries"] != entries * invocations or row["backedges"] != backedges * invocations:
            raise ValueError("per-method entries/backedges differ from ring execution model")
        if invocations == 0 and row["resumes"] != 0:
            raise ValueError("initial compiled census is not empty")


def validate_canonical(packet, identity, admission, plan):
    validate_admission(admission)
    if admission["schema_version"] != 1:
        raise ValueError("dedicated packet requires dedicated admission")
    validate_plan(plan)
    fields(packet, "schema_version qualification diagnostic_only sample_mode identity scope collector_lifecycle "
           "service_recording_enabled warmup_invocations samples clock clock_resolution_ns preparation "
           "program_teardown_ns method_profiles_before_warmup method_profiles_before_samples method_profiles_after_samples "
           "gc_cycles_completed global_capture_complete scheduled_latency_available sustainable_throughput_available", "canonical ring")
    if (type(packet["schema_version"]) is not int or packet["schema_version"] != 1
            or packet["qualification"] is not False or packet["diagnostic_only"] is not True
            or packet["scope"] != "full_invocation_v1"
            or packet["collector_lifecycle"] != "dedicated_heap_setup_body_cleanup_per_invocation"
            or packet["service_recording_enabled"] is not False
            or packet["global_capture_complete"] is not False or packet["gc_cycles_completed"] is not None
            or packet["scheduled_latency_available"] is not False or packet["sustainable_throughput_available"] is not False):
        raise ValueError("unsupported scope or qualification claim")
    import json
    if json.dumps(packet["identity"], sort_keys=True) != json.dumps(identity, sort_keys=True):
        raise ValueError("canonical identity differs from launched artifacts")
    if (packet["sample_mode"] != plan["mode"] or packet["warmup_invocations"] != plan["warmup_invocations"]
            or type(packet["warmup_invocations"]) is not int):
        raise ValueError("sample mode/warmup differs from frozen plan")
    measured = plan["mode"] == "measure"
    if packet["clock"] not in {"clock_monotonic_ns", "process_instant_ns"}:
        raise ValueError("unsupported monotonic clock")
    if packet["clock_resolution_ns"] is not None:
        integer(packet["clock_resolution_ns"], "clock resolution", 1)
    elif packet["clock"] == "clock_monotonic_ns":
        raise ValueError("native Linux clock resolution is missing")
    fields(packet["preparation"], "admission_ns compilation_ns", "preparation")
    for key, value in {**packet["preparation"], "program_teardown_ns": packet["program_teardown_ns"]}.items():
        available = measured and (key != "compilation_ns" or admission["tier"] == "compiled")
        if available:
            integer(value, key)
        elif value is not None:
            raise ValueError("untimed/unavailable preparation must remain null")
    samples = packet["samples"]
    if not isinstance(samples, list) or len(samples) != plan["samples"]:
        raise ValueError("missing or extra invocation samples")
    arrays = SIZES[admission["method"]][0] + admission["argument"]
    for row in samples:
        fields(row, "elapsed_ns result allocations array_allocations released_objects payload_bytes_after_cleanup "
               "catalog_bytes_after_drop java_calls compiled_entries compiled_resumes compiled_backedges native_slices helper_polls", "sample")
        if measured:
            integer(row["elapsed_ns"], "elapsed invocation", 1)
        elif row["elapsed_ns"] is not None:
            raise ValueError("check mode must not produce timing samples")
        for key, value in row.items():
            if key not in {"elapsed_ns", "result"}:
                integer(value, key)
        if (type(row["result"]) is not int or row["result"] != admission["expected_result"]
                or row["allocations"] != 2 * arrays or row["array_allocations"] != arrays
                or row["released_objects"] != 2 * arrays or row["payload_bytes_after_cleanup"] != 0
                or row["catalog_bytes_after_drop"] != 0 or row["java_calls"] != 2 + 2 * arrays):
            raise ValueError("ring result/allocation/call/cleanup model mismatch")
        if admission["tier"] == "interpreted":
            if any(row[key] for key in ("compiled_entries", "compiled_resumes", "compiled_backedges", "native_slices")):
                raise ValueError("interpreted sample claims compiled execution")
        elif (row["compiled_entries"] != 2 + 2 * arrays
              or row["compiled_backedges"] != 2 * SIZES[admission["method"]][0] + admission["argument"] - 1
              or row["native_slices"] != row["compiled_entries"] + row["compiled_resumes"]):
            raise ValueError("compiled invocation counters differ")
    profile_keys = ("method_profiles_before_warmup", "method_profiles_before_samples", "method_profiles_after_samples")
    if admission["tier"] == "compiled":
        for key, invocations in zip(profile_keys, (0, plan["warmup_invocations"], plan["warmup_invocations"] + plan["samples"])):
            _profiles(packet[key], admission["method"], admission["argument"], invocations)
        before, after = packet[profile_keys[1]], packet[profile_keys[2]]
        for field, report in (("entries", "compiled_entries"), ("resumes", "compiled_resumes"), ("backedges", "compiled_backedges")):
            deltas = [b[field] - a[field] for a, b in zip(before, after)]
            if any(value < 0 for value in deltas) or sum(deltas) != sum(row[report] for row in samples):
                raise ValueError("method census delta differs from invocation counters")
    elif any(packet[key] is not None for key in profile_keys):
        raise ValueError("interpreted method census must remain unavailable")
    return {"valid": True, "scope": packet["scope"], "samples": len(samples), "qualification": False,
            "gc_cycles_completed": None, "global_capture_complete": False,
            "compiled_method_census_valid": admission["tier"] == "compiled"}


def validate_peer(packet, workload, steps, plan):
    validate_plan(plan, peer=True)
    fields(packet, "schema_version qualification diagnostic_only run_id sample_mode workload steps warmup_invocations scope "
           "collector_lifecycle clock samples actual_thread_allocated_bytes gc_observation_scope gc_collection_counts "
           "in_invocation_gc_exposure_proven global_capture_complete scheduled_latency_available sustainable_throughput_available", "peer ring")
    if (type(packet["schema_version"]) is not int or packet["schema_version"] != 1
            or packet["qualification"] is not False or packet["diagnostic_only"] is not True
            or packet["run_id"] != plan["run_id"] or packet["sample_mode"] != plan["mode"]
            or packet["workload"] != workload or packet["steps"] != steps or type(packet["steps"]) is not int
            or packet["warmup_invocations"] != plan["warmup_invocations"] or type(packet["warmup_invocations"]) is not int
            or packet["scope"] != "workload_call_deferred_gc_v1"
            or packet["collector_lifecycle"] != "shared_hotspot_heap_deferred_gc"
            or packet["clock"] != "java_system_nano_time"
            or packet["gc_observation_scope"] != "whole_sample_batch_including_driver_gaps"
            or any(packet[key] is not False for key in ("in_invocation_gc_exposure_proven", "global_capture_complete", "scheduled_latency_available", "sustainable_throughput_available"))):
        raise ValueError("peer scope/identity/qualification claim mismatch")
    if not isinstance(packet["samples"], list) or len(packet["samples"]) != plan["samples"]:
        raise ValueError("peer sample count differs")
    for row in packet["samples"]:
        fields(row, "result elapsed_ns", "peer sample")
        if type(row["result"]) is not int or row["result"] != MODEL.expected(workload, steps):
            raise ValueError("peer checksum differs from independent ring model")
        if plan["mode"] == "measure":
            integer(row["elapsed_ns"], "peer invocation", 1)
        elif row["elapsed_ns"] is not None:
            raise ValueError("non-measurement mode produced timing samples")
    if plan["mode"] == "allocation":
        integer(packet["actual_thread_allocated_bytes"], "actual allocation", minimum_payload(workload, steps) * plan["samples"])
    elif packet["actual_thread_allocated_bytes"] is not None:
        raise ValueError("allocator observation must run separately from timing")
    if not isinstance(packet["gc_collection_counts"], list) or not packet["gc_collection_counts"]:
        raise ValueError("collector bean inventory missing")
    names = set()
    for row in packet["gc_collection_counts"]:
        fields(row, "name before after", "collector observation")
        if not isinstance(row["name"], str) or not row["name"] or row["name"] in names:
            raise ValueError("invalid/duplicate collector bean identity")
        names.add(row["name"])
        if plan["mode"] == "measure":
            if (row["before"] is None) != (row["after"] is None):
                raise ValueError("partial collector count pair")
            if row["before"] is not None:
                integer(row["before"], "collector count")
                integer(row["after"], "collector count", row["before"])
        elif row["before"] is not None or row["after"] is not None:
            raise ValueError("check/allocation mode unexpectedly observed GC counts")
    return {"valid": True, "scope": packet["scope"], "samples": plan["samples"], "qualification": False,
            "payload_allocation_lower_bound_verified": plan["mode"] == "allocation",
            "in_invocation_gc_exposure_proven": False, "global_capture_complete": False}


def validate_matrix(plan):
    """Keep dedicated and shared heap plans distinct before launching any child."""
    version = plan.get("schema_version")
    if type(version) is not int or version not in (1, 2) or plan.get("qualification") is not False:
        raise ValueError("unsupported matrix schema")
    if version == 1:
        # Older dedicated plans may omit the later optional watchdog/warmup keys.
        # Their existing launch and result validators retain their own contracts.
        if (plan.get("scopes", {}).get("canonical", "full_invocation_v1") != "full_invocation_v1"
                or plan.get("canonical_gc_cycles_available", False) is not False
                or "canonical_primary_cycle_scope" in plan or "canonical_primary_cycle_required" in plan):
            raise ValueError("dedicated matrix cannot claim shared-heap scope")
        return plan
    shared = True
    fields(plan, "schema_version qualification check_only warmup_check builds steps warmup_invocations samples "
           "allocation_samples forks workloads heap_bytes watchdog_seconds resources_sha256 admission_sha256 cases "
           "scopes canonical_gc_cycles_available scheduled_latency_available sustainable_throughput_available "
           "body_diagnostics_in_this_matrix" + (" canonical_primary_cycle_scope canonical_primary_cycle_required" if shared else ""), "ring matrix")
    version = plan["schema_version"]
    if type(version) is not int or version not in (1, 2) or plan["qualification"] is not False:
        raise ValueError("unsupported matrix schema")
    shared = version == 2
    if (plan["scopes"] != {"canonical": "shared_heap_invocation_v1" if shared else "full_invocation_v1",
                          "hotspot": "workload_call_deferred_gc_v1"}
            or plan["canonical_gc_cycles_available"] is not False
            or any(plan[k] is not False for k in ("scheduled_latency_available", "sustainable_throughput_available", "body_diagnostics_in_this_matrix"))):
        raise ValueError("matrix scope/coverage differs from heap protocol")
    if shared and (plan["canonical_primary_cycle_scope"] != "sample_batch_including_driver_gaps_and_observer"
                   or plan["canonical_primary_cycle_required"] is not (not plan["warmup_check"])):
        raise ValueError("shared primary-cycle scope/requirement differs")
    if shared and (type(plan["heap_bytes"]) is not int or plan["heap_bytes"] != 64*1024*1024):
        raise ValueError("shared matrix requires the declared 64 MiB payload/peer heap limit")
    return plan


def validate_matrix_admission(plan, admission):
    validate_admission(admission)
    if admission["schema_version"] != plan["schema_version"]:
        raise ValueError("matrix/admission heap lifetimes differ")
    if plan["schema_version"] == 2 and admission["max_payload_bytes"] != plan["heap_bytes"]:
        raise ValueError("matrix/shared payload ceilings differ")
    if admission["argument"] != plan["steps"] or admission["method"] not in plan["workloads"]:
        raise ValueError("matrix/admission workload differs")
    return admission
