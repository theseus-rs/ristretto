"""Check explicit Java execution windows. Valid counter evidence is not qualification."""
import json
from canonical_service import fields, integer, unique_object
from coverage import interval_profile


def packet(stdout, prefix):
    lines = [line[len(prefix):] for line in stdout.splitlines() if line.startswith(prefix)]
    if len(lines) != 1:
        raise ValueError(f"expected exactly one {prefix.strip()} packet")
    return json.loads(lines[0], object_pairs_hook=unique_object)


def validate(capture, elapsed_ns, fixture, completed, require_compiled=False):
    fields(capture, "schema_version scope diagnostic_only qualification atomic_global_cut "
           "all_actor_coverage state boundary_thread_id limits evidence_losses protocol_errors "
           "retained_code_bytes begin end interval", "execution measurement")
    if (type(capture["schema_version"]) is not int or capture["schema_version"] != 1
            or capture["scope"] != "explicit_java_measurement"
            or capture["diagnostic_only"] is not True or capture["qualification"] is not False
            or capture["atomic_global_cut"] is not False or capture["all_actor_coverage"] is not False
            or capture["state"] != "closed"):
        raise ValueError("missing, incomplete or overclaimed execution window")
    integer(capture["boundary_thread_id"], "boundary thread", 1)
    for key in ("evidence_losses", "protocol_errors"):
        if integer(capture[key], key) != 0:
            raise ValueError(f"execution recorder lost evidence: {key}")
    limits = capture["limits"]
    fields(limits, "methods functions_per_method identity_bytes_per_method retained_code_bytes", "recorder limits")
    for key, upper in (("methods", 65536), ("functions_per_method", 16),
                       ("identity_bytes_per_method", 65536), ("retained_code_bytes", 1024**3)):
        if integer(limits[key], key, 1) > upper:
            raise ValueError("unsupported recorder limit")
    if integer(capture["retained_code_bytes"], "retained code") > limits["retained_code_bytes"]:
        raise ValueError("retained code exceeds recorder limit")
    boundaries = []
    for name in ("begin", "end"):
        value = capture[name]
        fields(value, "timestamp_ns observer_start_ns snapshot_start_ns snapshot_end_ns observer_end_ns", name)
        for key, item in value.items():
            integer(item, f"{name}.{key}")
        order = [value[key] for key in (
            ("observer_start_ns", "snapshot_start_ns", "snapshot_end_ns", "timestamp_ns", "observer_end_ns")
            if name == "begin" else
            ("observer_start_ns", "timestamp_ns", "snapshot_start_ns", "snapshot_end_ns", "observer_end_ns"))]
        if order != sorted(order):
            raise ValueError("execution snapshot overlaps the wrong side of its timestamp")
        boundaries.append(value)
    begin, end = boundaries
    if (begin["observer_end_ns"] > end["observer_start_ns"]
            or end["timestamp_ns"] - begin["timestamp_ns"] != integer(elapsed_ns, "Java elapsed", 1)):
        raise ValueError("execution window does not match Java's actual elapsed interval")
    interval = interval_profile(capture["interval"])
    if interval["label"] != "GcTelemetry explicit measurement":
        raise ValueError("execution interval is not the explicit Java boundary")
    for name in ("boundary_span_ns", "start_snapshot_ns", "end_snapshot_ns"):
        integer(interval[name], name)
    if (interval["start_snapshot_ns"] > begin["snapshot_end_ns"] - begin["snapshot_start_ns"]
            or interval["end_snapshot_ns"] > end["snapshot_end_ns"] - end["snapshot_start_ns"]
            or interval["boundary_span_ns"] > end["snapshot_end_ns"] - begin["snapshot_start_ns"]):
        raise ValueError("counter snapshot timings exceed their actual boundary observers")
    methods = interval["methods"]
    if len(methods) > limits["methods"]:
        raise ValueError("method inventory exceeds recorder limit")
    if sum(method["code_bytes_at_end"] for method in methods) > capture["retained_code_bytes"]:
        raise ValueError("window code inventory exceeds retained recorder bytes")
    identities = {}
    for method in methods:
        identity = (method["class"], method["name"], method["descriptor"])
        if not all(isinstance(part, str) for part in identity):
            raise ValueError("invalid method identity")
        if sum(len(part.encode("utf-8")) for part in identity) > limits["identity_bytes_per_method"]:
            raise ValueError("identity exceeds recorder limit")
        identities.setdefault(identity, []).append(method)
    if fixture == "probe":
        if integer(completed, "completed probe calls", 1) != 7:
            raise ValueError("probe completion ground truth changed")
        required = [(('GcBench', 'kernel', '(I)I'), 7, 5),
                    (('GcBench', 'outside', '(I)I'), 0, 0)]
    elif fixture in {"control", "allocation", "roots", "mixed"}:
        integer(completed, "completed operations", 1)
        # Each roots operation constructs 16 nodes, then visits all 16. Every
        # completed iteration of either loop takes one Java backward branch.
        # The control and array paths in this method have only forward branches.
        required = [(('GcBench', 'operation', '(IJI)I'), completed, 32 if fixture == "roots" else 0),
                    (('GcBench$Histogram', 'record', '(J)V'), 2 * completed, None)]
        if fixture == "roots":
            required.append((('GcBench$Node', '<init>', '(ILGcBench$Node;)V'), 16 * completed, 0))
    else:
        raise ValueError("unsupported fixture ground truth")
    rows = []
    compiled_complete = True
    for identity, entries, backedges_per_call in required:
        found = identities.get(identity, [])
        if len(found) != 1:
            raise ValueError(f"missing/ambiguous required method: {identity}")
        method = found[0]
        rejection = method.get("rejected_bytecode")
        if rejection is not None and not (
                isinstance(rejection, list) and len(rejection) == 2
                and type(rejection[0]) is int and 0 <= rejection[0] < 65535
                and type(rejection[1]) is int and 0 <= rejection[1] <= 255):
            raise ValueError(f"invalid rejected bytecode for {identity}")
        total = sum(method[key] for key in ("compiled_entries", "interpreter_entries", "intrinsic_entries"))
        if total != entries:
            raise ValueError(f"measurement entries disagree with completed Java work: {identity}")
        expected_backedges = None
        if backedges_per_call is not None:
            expected_backedges = backedges_per_call * method["compiled_entries"]
            if method["compiled_backedges"] != expected_backedges:
                raise ValueError(f"compiled backedges for {identity}: expected {expected_backedges}, "
                                 f"observed {method['compiled_backedges']}")
        compiled = (method["compiled_entries"] == entries and not method["interpreter_entries"]
                    and not method["intrinsic_entries"] and method["compiled_before_start"]
                    and not method["compilation_attempts"] and not method["compilation_failures"]
                    and not method["compilation_successes"] and not method["queued_requests"]
                    and not method["generated_code_bytes"])
        if entries == 0:
            if method["compiled_backedges"] or method.get("compiled_resumes", 0):
                raise ValueError("outside-only method executed inside measurement")
        else:
            compiled_complete &= compiled
        rows.append({"method": list(identity), "expected_entries": entries,
                     "compiled_observed": method["compiled_entries"],
                     "interpreter_observed": method["interpreter_entries"],
                     "intrinsic_observed": method["intrinsic_entries"],
                     "expected_compiled_backedges": expected_backedges,
                     "compiled_backedges": method["compiled_backedges"],
                     "rejection_reason": method.get("rejection_reason"),
                     "rejected_bytecode": rejection,
                     "ready_compiled_for_required_calls": compiled})
    if require_compiled and not compiled_complete:
        raise ValueError("required measured calls were not ready and wholly compiled")
    return {"measurement_counter_evidence_valid": True, "required_call_coverage_compiled": compiled_complete,
            "qualification": False, "coverage_gate_passed": False,
            "required_methods": rows,
            "boundary_observer_ns": sum(b["observer_end_ns"] - b["observer_start_ns"] for b in boundaries),
            "elapsed_ns": elapsed_ns,
            "unresolved": ["complete dependency and actor census", "CPU attribution and observer overhead qualification",
                           "all-source GC global/equivalent stall coverage", "sustained throughput and scheduled tails"]}
