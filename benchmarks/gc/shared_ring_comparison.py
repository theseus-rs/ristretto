"""Strict persistent-heap packet checks, independent of the Rust producer."""
import json
from canonical_service import fields, integer
from ring_comparison import SIZES, _profiles, validate_admission, validate_plan

SCOPE = "shared_heap_invocation_v1"
BATCH_SCOPE = "sample_batch_including_driver_gaps_and_observer"


def cycle_bounds(batch, measured):
    fields(batch, "scope before_start after_start before_end after_end start_clock_ns end_clock_ns "
           "elapsed_ns completed_primary_cycles", "primary-cycle batch")
    if batch["scope"] != BATCH_SCOPE:
        raise ValueError("primary cycles have no matching batch scope")
    values = []
    feasible = 0
    for key in ("before_start", "after_start", "before_end", "after_end"):
        row = batch[key]
        fields(row, "lower upper", "cycle observation")
        lo = integer(row["lower"], "primary lower")
        hi = integer(row["upper"], "primary upper", lo)
        if hi > 2**64-1:
            raise ValueError("cycle counter overflow")
        feasible = max(feasible, lo)
        if feasible > hi:
            raise ValueError("infeasible monotone primary-cycle observations")
        values.append((lo, hi))
    bounds = {"lower": max(0, values[2][0] - values[1][1]), "upper": values[3][1] - values[0][0]}
    fields(batch["completed_primary_cycles"], "lower upper", "cycle bounds")
    for key in bounds:
        integer(batch["completed_primary_cycles"][key], key)
    if batch["completed_primary_cycles"] != bounds:
        raise ValueError("primary-cycle bound arithmetic differs")
    if measured:
        start = integer(batch["start_clock_ns"], "batch start")
        end = integer(batch["end_clock_ns"], "batch end", start+1)
        if integer(batch["elapsed_ns"], "batch elapsed", 1) != end-start:
            raise ValueError("batch duration differs from actual boundary clocks")
    elif any(batch[key] is not None for key in ("start_clock_ns", "end_clock_ns", "elapsed_ns")):
        raise ValueError("untimed check emitted batch clocks")
    return bounds


def validate_shared(packet, identity, admission, plan):
    validate_admission(admission)
    validate_plan(plan)
    if admission["schema_version"] != 2:
        raise ValueError("shared packet requires distinct shared admission")
    fields(packet, "schema_version qualification diagnostic_only sample_mode identity scope collector_lifecycle "
           "service_recording_enabled warmup_invocations completed_samples expected_result checksum_verified heap_cleanup_verified "
           "samples clock clock_resolution_ns preparation heap_detach_cleanup_and_drop_ns program_teardown_ns "
           "method_profiles_before_warmup method_profiles_before_samples method_profiles_after_samples primary_cycle_batch "
           "heap workload_counts in_invocation_gc_exposure_proven global_capture_complete scheduled_latency_available "
           "sustainable_throughput_available", "shared canonical ring")
    if (type(packet["schema_version"]) is not int or packet["schema_version"] != 2
            or packet["qualification"] is not False or packet["diagnostic_only"] is not True
            or packet["scope"] != SCOPE or packet["collector_lifecycle"] != "one_heap_and_context_across_warmup_and_samples"
            or packet["service_recording_enabled"] is not False or packet["checksum_verified"] is not True
            or packet["heap_cleanup_verified"] is not True
            or any(packet[key] is not False for key in ("in_invocation_gc_exposure_proven", "global_capture_complete",
                                                        "scheduled_latency_available", "sustainable_throughput_available"))):
        raise ValueError("shared scope/cleanup/qualification claim mismatch")
    if json.dumps(packet["identity"], sort_keys=True) != json.dumps(identity, sort_keys=True):
        raise ValueError("shared identity differs from launched artifacts")
    if packet["sample_mode"] != plan["mode"]:
        raise ValueError("shared sample mode mismatch")
    for key, expected in (("warmup_invocations", plan["warmup_invocations"]), ("completed_samples", plan["samples"]),
                          ("expected_result", admission["expected_result"])):
        if type(packet[key]) is not int or packet[key] != expected:
            raise ValueError("shared count/checksum differs")
    measured = plan["mode"] == "measure"
    warmup_check = plan["mode"] == "warmup-check"
    if packet["clock"] not in {"clock_monotonic_ns", "process_instant_ns"}:
        raise ValueError("unsupported shared clock")
    if packet["clock_resolution_ns"] is not None:
        integer(packet["clock_resolution_ns"], "resolution", 1)
    elif packet["clock"] == "clock_monotonic_ns":
        raise ValueError("native Linux clock resolution missing")
    fields(packet["preparation"], "admission_ns compilation_ns heap_setup_and_attach_ns", "shared preparation")
    for key, value in {**packet["preparation"], "heap_detach_cleanup_and_drop_ns": packet["heap_detach_cleanup_and_drop_ns"],
                       "program_teardown_ns": packet["program_teardown_ns"]}.items():
        if measured and (key != "compilation_ns" or admission["tier"] == "compiled"):
            integer(value, key)
        elif value is not None:
            raise ValueError("untimed/unavailable preparation must remain null")
    heap = packet["heap"]
    fields(heap, "max_payload_bytes reserved_catalog_bytes cleanup_limit payload_bytes_before_cleanup", "shared heap")
    integer(heap["max_payload_bytes"], "heap payload", 1)
    integer(heap["cleanup_limit"], "heap cleanup limit", 1)
    integer(heap["reserved_catalog_bytes"], "reserved catalog charge", 1)
    if heap["max_payload_bytes"] != admission["max_payload_bytes"] or heap["cleanup_limit"] != admission["heap_cleanup_polls"]:
        raise ValueError("heap budget differs from admission")
    profiles = ("method_profiles_before_warmup", "method_profiles_before_samples", "method_profiles_after_samples")
    if warmup_check:
        if any(packet[key] is not None for key in ("samples", "workload_counts", "primary_cycle_batch", *profiles)):
            raise ValueError("warmup shape check emitted measurements/counters")
        if heap["payload_bytes_before_cleanup"] is not None:
            raise ValueError("warmup shape check emitted payload accounting")
        return {"valid": True, "scope": SCOPE, "qualification": False, "completed_warmups": plan["warmup_invocations"],
                "completed_checks": plan["samples"], "timing_samples_available": False, "allocator_counts_available": False}
    if integer(heap["payload_bytes_before_cleanup"], "payload before final cleanup") > heap["max_payload_bytes"]:
        raise ValueError("payload exceeded ceiling")
    rows = packet["samples"]
    if not isinstance(rows, list) or len(rows) != plan["samples"]:
        raise ValueError("shared sample inventory differs")
    expected_allocations = 2 * (SIZES[admission["method"]][0] + admission["argument"])
    bounds = cycle_bounds(packet["primary_cycle_batch"], measured)
    last = packet["primary_cycle_batch"]["start_clock_ns"] if measured else None
    for row in rows:
        fields(row, "elapsed_ns start_clock_ns end_clock_ns result allocations released_objects helper_polls native_slices", "shared sample")
        if type(row["result"]) is not int or row["result"] != admission["expected_result"]:
            raise ValueError("shared checksum differs")
        for key in ("allocations", "released_objects", "helper_polls", "native_slices"):
            integer(row[key], key)
        if row["allocations"] != expected_allocations:
            raise ValueError("shared allocation model differs")
        if measured:
            start = integer(row["start_clock_ns"], "call start", last)
            end = integer(row["end_clock_ns"], "call end", start+1)
            if integer(row["elapsed_ns"], "call elapsed", 1) != end-start:
                raise ValueError("call duration differs")
            last = end
        elif any(row[key] is not None for key in ("start_clock_ns", "end_clock_ns", "elapsed_ns")):
            raise ValueError("check mode emitted call clocks")
        if admission["tier"] == "interpreted" and row["native_slices"] != 0:
            raise ValueError("interpreted shared call claims native slices")
        if admission["tier"] == "compiled" and row["native_slices"] == 0:
            raise ValueError("compiled shared call has no native slices")
    if bounds["lower"] < admission["minimum_primary_cycles"]:
        raise ValueError("sample batch has no proven primary-cycle exposure")
    if measured and last > packet["primary_cycle_batch"]["end_clock_ns"]:
        raise ValueError("call escaped primary-cycle batch boundary")
    counts = packet["workload_counts"]
    fields(counts, "warmup_allocations warmup_released_objects warmup_helper_polls warmup_native_slices "
           "sample_allocations sample_released_objects cleanup_polls released_during_cleanup", "whole-workload counts")
    for key, value in counts.items():
        integer(value, key)
    if (counts["warmup_allocations"] != expected_allocations*plan["warmup_invocations"]
            or counts["sample_allocations"] != sum(r["allocations"] for r in rows)
            or counts["sample_released_objects"] != sum(r["released_objects"] for r in rows)
            or counts["warmup_allocations"]+counts["sample_allocations"] != counts["warmup_released_objects"]+counts["sample_released_objects"]+counts["released_during_cleanup"]
            or counts["cleanup_polls"] > heap["cleanup_limit"] or counts["released_during_cleanup"] > counts["cleanup_polls"]):
        raise ValueError("whole-workload allocations/releases/cleanup do not balance")
    if plan["warmup_invocations"] == 0 and any(counts[k] for k in ("warmup_released_objects", "warmup_helper_polls", "warmup_native_slices")):
        raise ValueError("counts claimed absent warmup")
    if admission["tier"] == "compiled":
        for key, invocations in zip(profiles, (0, plan["warmup_invocations"], plan["warmup_invocations"]+plan["samples"])):
            _profiles(packet[key], admission["method"], admission["argument"], invocations)
        before, after = packet[profiles[1]], packet[profiles[2]]
        for a,b in zip(before,after):
            if any(b[k]<a[k] for k in ("entries", "resumes", "backedges")):
                raise ValueError("compiled census reversed")
        sample_slices = sum(b["entries"]+b["resumes"]-a["entries"]-a["resumes"] for a,b in zip(before,after))
        if sample_slices != sum(row["native_slices"] for row in rows) or counts["warmup_native_slices"] != sum(a["entries"]+a["resumes"] for a in before):
            raise ValueError("compiled census/native slices differ")
    elif any(packet[key] is not None for key in profiles) or counts["warmup_native_slices"] != 0:
        raise ValueError("interpreted shared capture claims compiled census")
    return {"valid": True, "scope": SCOPE, "samples": len(rows), "qualification": False,
            "primary_cycle_batch_bounds": bounds, "heap_cleanup_verified": True,
            "compiled_method_census_valid": admission["tier"] == "compiled", "global_capture_complete": False}
