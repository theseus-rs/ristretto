"""Independent validation of partial canonical service captures; never qualifies GC."""
import hashlib
import json
from pathlib import Path

PROTOCOL = "ristretto-canonical-service-v1"
MISSING = ["owner_release", "native_execution_and_frame_publication",
           "cancellation_and_teardown", "global_actor_state"]
KINDS = {"request", "preparation", "request_poll", "helper_poll", "measurement"}
OPERATIONS = {"allocation", "child_read", "edge_write", "maintenance", "measurement",
              "primitive_array_preparation"}
OUTCOMES = {"complete", "pending", "failed", "abandoned"}


def fields(value, names, label):
    if not isinstance(value, dict) or set(value) != set(names.split()):
        raise ValueError(f"{label}: unexpected/missing fields")


def integer(value, label, minimum=0):
    if type(value) is not int or value < minimum:
        raise ValueError(f"{label}: expected integer >= {minimum}")
    return value


def digest(path):
    with Path(path).open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def sha(value, label):
    if not isinstance(value, str) or len(value) != 64 or any(c not in "0123456789abcdef" for c in value):
        raise ValueError(f"{label}: invalid lowercase SHA-256")


def workload_digest(directory):
    root = Path(directory)
    paths = sorted(root.rglob("*.class"))
    if not paths:
        raise ValueError("no classfiles")
    result = hashlib.sha256()
    for path in paths:
        if path.is_symlink():
            raise ValueError("symlink classfile")
        result.update(path.relative_to(root).as_posix().encode("utf-8") + b"\0")
        result.update(digest(path).encode("ascii") + b"\n")
    return result.hexdigest()


def observation(path):
    packets = [line.removeprefix("RISTRETTO_CANONICAL_SERVICE ")
               for line in Path(path).read_text().splitlines()
               if line.startswith("RISTRETTO_CANONICAL_SERVICE ")]
    if len(packets) != 1:
        raise ValueError("expected exactly one canonical service packet")
    return json.loads(packets[0], object_pairs_hook=unique_object)


def unique_object(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise ValueError(f"duplicate JSON key: {key}")
        result[key] = value
    return result


def cycle_bounds(value, sequence):
    """Reconstruct primary-epoch bounds from four ordered passive observations."""
    fields(value, "status scope measurement_sequence observations bounds", "cycle observation")
    if (value["status"] != "complete" or value["scope"] != "canonical_primary_full_epochs"
            or integer(value["measurement_sequence"], "cycle measurement sequence") != sequence):
        raise ValueError("cycle observations do not belong to the completed body window")
    samples = value["observations"]
    if not isinstance(samples, list) or len(samples) != 4:
        raise ValueError("cycle observation requires four boundary samples")
    minimum = 0
    for sample in samples:
        if sample is None:
            continue
        fields(sample, "lower upper", "cycle sample")
        lower = integer(sample["lower"], "cycle lower")
        upper = integer(sample["upper"], "cycle upper", lower)
        if upper > (1 << 64) - 1:
            raise ValueError("cycle count exceeds u64")
        minimum = max(minimum, lower)
        if minimum > upper:
            raise ValueError("cycle observations cannot describe monotone progress")
    expected = None if any(sample is None for sample in samples) else {
        "lower": max(0, samples[2]["lower"] - samples[1]["upper"]),
        "upper": samples[3]["upper"] - samples[0]["lower"],
    }
    if json.dumps(value["bounds"], sort_keys=True) != json.dumps(expected, sort_keys=True):
        raise ValueError("reported cycle bounds differ from boundary observations")
    return expected


def validate_trace(trace, expected_identity, admission):
    version = trace.get("schema_version") if isinstance(trace, dict) else None
    if type(version) is not int or version not in {1, 2}:
        raise ValueError("unsupported canonical service schema")
    fields(trace, "schema_version protocol diagnostic_only qualification scope window_kind identity "
           "binding clock clock_resolution_ns capture_start_ns capture_end_ns capacity overflow "
           "open_events producer_issues instrumented_capture_complete global_capture_complete "
           "missing_sources measurement_sequence recorder_id events gaps execution"
           + (" cycle_observation" if version == 2 else ""), "capture")
    if (trace["protocol"] != PROTOCOL or trace["diagnostic_only"] is not True
            or trace["qualification"] is not False
            or trace["scope"] != "canonical_single_actor_services"
            or trace["window_kind"] != "entry_body_v1"):
        raise ValueError("unsupported canonical service schema/scope")
    if (trace["global_capture_complete"] is not False or trace["missing_sources"] != MISSING):
        raise ValueError("partial canonical producer cannot claim complete global coverage")
    identity = trace["identity"]
    fields(identity, "run_id mode tier host_process_id guest_process_id host_executable_sha256 "
           "binary_sha256 source_archive_sha256 source_manifest_sha256 workload_sha256 "
           "admission_sha256 build_receipt_sha256 launch_receipt_sha256", "identity")
    if json.dumps(identity, sort_keys=True) != json.dumps(expected_identity, sort_keys=True):
        raise ValueError("capture identity differs from independently launched artifacts")
    if not isinstance(identity["run_id"], str) or not identity["run_id"]:
        raise ValueError("missing run identity")
    for key, value in identity.items():
        if key.endswith("sha256"):
            sha(value, key)
    integer(identity["host_process_id"], "host PID", 1)
    if identity["mode"] == "native":
        if (type(identity["guest_process_id"]) is not int
                or identity["guest_process_id"] != identity["host_process_id"]
                or identity["binary_sha256"] != identity["host_executable_sha256"]):
            raise ValueError("native executable/PID does not match exec receipt")
    elif identity["mode"] == "wasi":
        if identity["guest_process_id"] is not None:
            raise ValueError("WASI guest PID must remain unavailable")
        if identity["tier"] != "interpreted":
            raise ValueError("compiled canonical WASI is unsupported")
    else:
        raise ValueError("unsupported host mode")
    if identity["tier"] != admission["tier"]:
        raise ValueError("execution tier differs from frozen admission")
    expected_binding = {"process_id": identity["guest_process_id"], "run_id": identity["run_id"],
                        "source_sha256": identity["source_archive_sha256"],
                        "binary_sha256": identity["binary_sha256"],
                        "workload_sha256": identity["workload_sha256"],
                        "configuration_sha256": identity["admission_sha256"]}
    if json.dumps(trace["binding"], sort_keys=True) != json.dumps(expected_binding, sort_keys=True):
        raise ValueError("recorder binding differs from independently verified identity")
    if not isinstance(trace["clock"], str) or trace["clock"] not in {"clock_monotonic_ns", "process_instant_ns"}:
        raise ValueError("unavailable or unsupported monotonic clock")
    if trace["clock_resolution_ns"] is not None:
        integer(trace["clock_resolution_ns"], "clock resolution", 1)
    if trace["clock"] == "clock_monotonic_ns" and trace["clock_resolution_ns"] is None:
        raise ValueError("Linux clock resolution missing")
    start = integer(trace["capture_start_ns"], "capture start")
    end = integer(trace["capture_end_ns"], "capture end", start + 1)
    capacity = integer(trace["capacity"], "capacity", 1)
    if capacity != admission["recorder_capacity"]:
        raise ValueError("capacity differs from frozen admission")
    if (trace["overflow"] is not False or integer(trace["open_events"], "open events") != 0
            or trace["producer_issues"] != [] or trace["instrumented_capture_complete"] is not True):
        raise ValueError("producer reports lost, incomplete or invalid service evidence")
    integer(trace["recorder_id"], "recorder id", 1)
    events = trace["events"]
    if not isinstance(events, list) or not 0 < len(events) <= capacity:
        raise ValueError("missing/excess service events")
    windows, services, grouped = [], [], {}
    previous_start = start
    for sequence, event in enumerate(events):
        fields(event, "sequence parent kind operation start_ns end_ns outcome", "event")
        if integer(event["sequence"], "sequence") != sequence:
            raise ValueError("non-contiguous event sequence")
        if (not isinstance(event["kind"], str) or event["kind"] not in KINDS
                or not isinstance(event["operation"], str) or event["operation"] not in OPERATIONS
                or not isinstance(event["outcome"], str) or event["outcome"] not in OUTCOMES):
            raise ValueError("unknown event kind/operation/outcome")
        begin = integer(event["start_ns"], "event start", start)
        finish = integer(event["end_ns"], "event end", begin)
        if begin < previous_start:
            raise ValueError("single-actor event starts are out of sequence")
        previous_start = begin
        if finish > end or event["outcome"] in {"abandoned", "failed"}:
            raise ValueError("incomplete/failed event or capture boundary violation")
        kind, parent = event["kind"], event["parent"]
        if event["outcome"] == "pending" and kind not in {"request_poll", "helper_poll"}:
            raise ValueError("Pending is only a synchronous poll outcome")
        if parent is not None:
            parent = integer(parent, "parent")
            if parent >= sequence or kind in {"request", "measurement"}:
                raise ValueError("invalid forward/envelope parent")
            request = events[parent]
            if (request["kind"] != "request" or request["start_ns"] > begin
                    or request["end_ns"] < finish):
                raise ValueError("request does not contain child")
            if kind in {"request_poll", "helper_poll"} and event["operation"] != request["operation"]:
                raise ValueError("poll operation differs from parent")
            if kind == "preparation" and not (event["operation"] == request["operation"]
                    or (event["operation"] == "primitive_array_preparation" and request["operation"] == "allocation")):
                raise ValueError("preparation operation differs from parent")
            grouped.setdefault(parent, []).append(event)
        elif kind in {"preparation", "request_poll"}:
            raise ValueError("synchronous request work has no parent")
        if kind == "request" and event["operation"] not in {"allocation", "child_read", "edge_write"}:
            raise ValueError("invalid request operation")
        if kind == "helper_poll" and parent is None and event["operation"] != "maintenance":
            raise ValueError("invalid helper operation")
        if kind == "measurement":
            if event["operation"] != "measurement" or finish == begin:
                raise ValueError("invalid measurement event")
            windows.append(event)
        elif kind != "request":
            services.append(event)
    integer(trace["measurement_sequence"], "measurement sequence")
    if len(windows) != 1 or trace["measurement_sequence"] != windows[0]["sequence"]:
        raise ValueError("missing/multiple/mismatched measurement windows")
    window = windows[0]
    cycles = cycle_bounds(trace["cycle_observation"], window["sequence"]) if version == 2 else None
    for event in events:
        if event["kind"] == "measurement":
            continue
        a, b = event["start_ns"], event["end_ns"]
        if (a < window["end_ns"] and b > window["start_ns"]
                and (a < window["start_ns"] or b > window["end_ns"])):
            raise ValueError("event crosses measurement boundary; no clipping allowed")
        if event["kind"] == "request" and not window["start_ns"] <= a <= b <= window["end_ns"]:
            raise ValueError("entry-body request outside explicit measurement")
    finish = start
    for event in sorted(services, key=lambda item: (item["start_ns"], item["sequence"])):
        if event["start_ns"] < finish:
            raise ValueError("single-actor synchronous services overlap")
        finish = event["end_ns"]
    expected_gaps = []
    requests = [event for event in events if event["kind"] == "request"]
    request_end = start
    for request in requests:
        if request["start_ns"] < request_end:
            raise ValueError("single-actor requests overlap")
        request_end = request["end_ns"]
        children = sorted(grouped.get(request["sequence"], []), key=lambda item: item["start_ns"])
        polls = [child for child in children if child["kind"] == "request_poll"]
        if (not polls or polls[-1]["outcome"] != "complete"
                or any(child["outcome"] != "pending" for child in polls[:-1])):
            raise ValueError("request must end in exactly one completed poll")
        if sum(
                child["kind"] == "preparation" and child["operation"] == request["operation"] for child in children) != 1:
            raise ValueError("request preparation evidence missing/duplicated")
        cursor, pending = request["start_ns"], False
        for child in children:
            if child["start_ns"] > cursor:
                expected_gaps.append({"request": request["sequence"], "start_ns": cursor,
                                      "end_ns": child["start_ns"], "after_pending": pending})
            if child["kind"] == "request_poll":
                pending = child["outcome"] == "pending"
            cursor = child["end_ns"]
        if cursor < request["end_ns"]:
            expected_gaps.append({"request": request["sequence"], "start_ns": cursor,
                                  "end_ns": request["end_ns"], "after_pending": pending})
    if not isinstance(trace["gaps"], list):
        raise ValueError("missing gap list")
    for gap in trace["gaps"]:
        fields(gap, "request start_ns end_ns after_pending", "gap")
        integer(gap["request"], "gap request")
        integer(gap["start_ns"], "gap start", start)
        integer(gap["end_ns"], "gap end", gap["start_ns"] + 1)
        if type(gap["after_pending"]) is not bool:
            raise ValueError("gap Pending marker must be boolean")
    if trace["gaps"] != expected_gaps:
        raise ValueError("gap evidence differs from independently reconstructed request gaps")
    execution = trace["execution"]
    fields(execution, "status error report", "execution")
    if execution["status"] != "ok" or execution["error"] is not None:
        raise ValueError("finite execution failed")
    report = execution["report"]
    fields(report, "result instructions instruction_count_available allocations array_allocations reference_reads "
           "reference_writes calls helper_polls released_objects payload_bytes_after_cleanup "
           "catalog_bytes_after_drop compiled_entries compiled_resumes compiled_backedges native_slices", "report")
    for key, value in report.items():
        if key not in {"result", "instruction_count_available"}:
            integer(value, key)
    if type(report["result"]) is not int or report["result"] != admission["expected_result"]:
        raise ValueError("finite Java result differs from admission oracle")
    if report["payload_bytes_after_cleanup"] or report["catalog_bytes_after_drop"]:
        raise ValueError("finite cleanup did not release charged storage")
    if sum(event["kind"] == "helper_poll" for event in events) != report["helper_polls"]:
        raise ValueError("helper event count differs from runtime progress counter")
    for operation, key in [("allocation", "allocations"), ("child_read", "reference_reads"), ("edge_write", "reference_writes")]:
        if sum(event["operation"] == operation for event in requests) != report[key]:
            raise ValueError(f"{operation} request count differs from runtime counter")
    array_preparations = [event for event in events if event["kind"] == "preparation"
                          and event["operation"] == "primitive_array_preparation"]
    if (len(array_preparations) != report["array_allocations"]
            or report["array_allocations"] > report["allocations"]
            or len({event["parent"] for event in array_preparations}) != len(array_preparations)):
        raise ValueError("primitive array preparation differs from allocation counters")
    if identity["tier"] == "interpreted":
        if report["instruction_count_available"] is not True or report["instructions"] == 0 or any(report[key] for key in ["compiled_entries", "compiled_resumes", "compiled_backedges", "native_slices"]):
            raise ValueError("interpreter counter evidence invalid")
    elif identity["tier"] == "compiled":
        if (report["instruction_count_available"] is not False or report["instructions"] != 0
                or report["compiled_entries"] == 0 or report["native_slices"] == 0
                or (admission["require_compiled_backedges"] and report["compiled_backedges"] == 0)):
            raise ValueError("compiled entry/backedge evidence missing")
    else:
        raise ValueError("unknown execution tier")
    body_services = [event for event in services if window["start_ns"] <= event["start_ns"] <= event["end_ns"] <= window["end_ns"]]
    return {"diagnostic_only": True, "qualification": False, "global_capture_complete": False,
            "instrumented_capture_valid": True, "events": len(events), "requests": len(requests),
            "entry_body_elapsed_ns": window["end_ns"] - window["start_ns"],
            "service_elapsed_ns": sum(event["end_ns"] - event["start_ns"] for event in body_services),
            "max_observed_service_ns": max((event["end_ns"] - event["start_ns"] for event in body_services), default=None),
            "pending_gap_elapsed_ns": sum(gap["end_ns"] - gap["start_ns"] for gap in expected_gaps if gap["after_pending"]),
            "gc_cycles_completed": cycles["lower"] if cycles is not None and cycles["lower"] == cycles["upper"] else None,
            "gc_cycle_scope": "canonical_primary_full_epochs" if version == 2 else None,
            "gc_cycle_bounds": cycles, "gc_cycle_observation_valid": cycles is not None,
            "zero_global_pause_proven": False, "missing_sources": MISSING}
