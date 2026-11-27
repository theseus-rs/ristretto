"""Validate partial runtime publication-gate captures without qualifying them.

This contract deliberately cannot prove zero STW, whole-process coverage or
measured-window GC exposure. Raw failing/overflowed traces remain reviewable.
"""
import hashlib
import json
from pathlib import Path

from acceptance import PAUSE_LIMIT_NS, fields, integer, sha256

RECORDER = "ristretto-publication-gate-v2"


def workload_digest(root):
    """Sorted UTF-8 relative path + NUL + lowercase class SHA-256 + LF."""
    root = Path(root)
    paths = sorted(root.rglob("*.class"))
    if not paths:
        raise ValueError("workload class manifest is empty")
    digest = hashlib.sha256()
    for path in paths:
        digest.update(path.relative_to(root).as_posix().encode("utf-8"))
        digest.update(b"\0")
        digest.update(hashlib.sha256(path.read_bytes()).hexdigest().encode("ascii"))
        digest.update(b"\n")
    return digest.hexdigest()


def observation(directory):
    lines = [line[len("RISTRETTO_PAUSES "):]
             for line in (Path(directory) / "stdout.txt").read_text().splitlines()
             if line.startswith("RISTRETTO_PAUSES ")]
    if not lines:
        return None
    if len(lines) != 1:
        raise ValueError("expected exactly one RISTRETTO_PAUSES packet")
    return json.loads(lines[0])


def validate_trace(trace, expected_identity, measurement_elapsed_ns=None):
    if measurement_elapsed_ns is not None:
        integer(measurement_elapsed_ns, "measured Java duration", 1)
    if not isinstance(trace, dict):
        raise ValueError("native publication trace missing")
    if (type(trace.get("schema_version")) is not int or trace["schema_version"] != 2
            or trace.get("diagnostic_only") is not True or trace.get("qualification") is not False
            or trace.get("recorder") != RECORDER
            or trace.get("scope") != "instrumented_collector_lifetime"
            or trace.get("clock") != "clock_monotonic_ns"):
        raise ValueError("unsupported partial pause recorder")
    identity = trace.get("identity")
    fields(identity, "run_id policy_sha256 source_archive_sha256 binary_sha256 workload_sha256 "
           "process_id recorder_capacity runtime workload duration_ms warmup_ms rate seed", "pause identity")
    if identity != expected_identity:
        raise ValueError("pause trace differs from launched process/workload/build identity")
    if not isinstance(identity["run_id"], str) or not identity["run_id"]:
        raise ValueError("pause run identity missing")
    integer(identity["process_id"], "pause process id", 1)
    capacity = integer(identity["recorder_capacity"], "pause recorder capacity", 1)
    for key in ["source_archive_sha256", "binary_sha256", "workload_sha256"]:
        sha256(identity[key], key)
    if identity["policy_sha256"] is not None:
        sha256(identity["policy_sha256"], "pause policy")
    if (trace.get("global_source_coverage_complete") is not False
            or trace.get("global_stop_sources") != ["publication"]
            or trace.get("uncovered_global_stop_sources") != ["allocation_recovery", "gc_safepoint"]):
        raise ValueError("publication recorder cannot claim complete global source coverage")
    start = integer(trace.get("capture_start_ns"), "pause capture start")
    end = integer(trace.get("capture_end_ns"), "pause capture end", start + 1)
    resolution = integer(trace.get("clock_resolution_ns"), "pause clock resolution", 1)
    dropped = integer(trace.get("dropped_events"), "dropped pause events")
    opened = integer(trace.get("open_events"), "open pause events")
    errors = integer(trace.get("clock_errors"), "pause clock errors")
    if trace.get("capture_complete") is not False:
        raise ValueError("partial publication telemetry cannot claim complete global capture")
    complete = trace.get("instrumented_capture_complete")
    if type(complete) is not bool or (complete and (dropped or opened or errors)):
        raise ValueError("pause recorder claimed complete capture after evidence loss")
    events = trace.get("events")
    if not isinstance(events, list):
        raise ValueError("pause event stream missing")
    attempts = integer(trace.get("publication_attempts"), "publication attempts")
    acquisitions = integer(trace.get("publication_acquisitions"), "publication acquisitions")
    failed = integer(trace.get("publication_failed_attempts"), "publication failed attempts")
    if (attempts != acquisitions + failed or acquisitions != len(events) + dropped
            or len(events) != min(acquisitions, capacity)):
        raise ValueError("publication attempts/acquisitions differ from retained evidence")
    if trace.get("mutator_admission_waits_recorded") is not False:
        raise ValueError("publication writer recorder does not capture local admission waits")
    intervals, unfinished = [], 0
    for sequence, event in enumerate(events):
        fields(event, "sequence source kind requested_ns acquired_ns released_ns", "pause event")
        if (integer(event["sequence"], "pause sequence") != sequence
                or event["source"] != "publication" or event["kind"] != "equivalent_global_stall"):
            raise ValueError("pause sequence/source mismatch")
        requested = integer(event["requested_ns"], "pause request")
        acquired = integer(event["acquired_ns"], "publication acquired", requested)
        if not start <= requested <= acquired <= end:
            raise ValueError("pause request/acquisition outside capture")
        released = event["released_ns"]
        if released is None:
            unfinished += 1
            continue
        released = integer(released, "publication released", acquired)
        if released > end or released <= requested:
            raise ValueError("pause release outside capture or invalid duration")
        intervals.append((requested, released))
    if opened != unfinished:
        raise ValueError("unfinished pause count differs from event stream")
    merged = []
    for begin, finish in sorted(intervals):
        if merged and begin <= merged[-1][1]:
            merged[-1][1] = max(finish, merged[-1][1])
        else:
            merged.append([begin, finish])
    maximum = max((finish - begin for begin, finish in merged), default=None)
    measurement = trace.get("measurement_window")
    boundaries = trace.get("measurement_boundaries")
    fields(boundaries, "state errors begin end", "measurement boundaries")
    boundary_errors = integer(boundaries["errors"], "measurement protocol errors")
    if boundaries["state"] not in {"missing", "begin_incomplete", "open", "end_incomplete", "closed"}:
        raise ValueError("invalid measurement protocol state")
    def boundary(value):
        fields(value, "timestamp_ns observer_start_ns observer_end_ns collections_before collections_after",
               "measurement boundary")
        first = integer(value["observer_start_ns"], "observer start", start)
        timestamp = integer(value["timestamp_ns"], "boundary timestamp", first)
        last = integer(value["observer_end_ns"], "observer end", timestamp)
        before = integer(value["collections_before"], "GC count before boundary")
        after = integer(value["collections_after"], "GC count after boundary", before)
        if last > end:
            raise ValueError("measurement observer outside capture")
        return value
    begin_boundary = boundary(boundaries["begin"]) if boundaries["begin"] is not None else None
    end_boundary = boundary(boundaries["end"]) if boundaries["end"] is not None else None
    expected_presence = {"missing": (False, False), "begin_incomplete": (False, False),
                         "open": (True, False), "end_incomplete": (True, False), "closed": (True, True)}
    if (begin_boundary is not None, end_boundary is not None) != expected_presence[boundaries["state"]]:
        raise ValueError("measurement state does not match boundaries")
    if measurement is None:
        if boundaries["state"] == "closed" or trace.get("gc_exposure") is not None:
            raise ValueError("measurement window missing for closed boundaries")
        if measurement_elapsed_ns is not None:
            raise ValueError("real measured-window boundaries missing")
    else:
        if boundary_errors or boundaries["state"] != "closed" or errors:
            raise ValueError("measurement boundary protocol or clock incomplete")
        fields(measurement, "start_ns end_ns", "pause measurement window")
        begin = integer(measurement["start_ns"], "measurement start", start)
        finish = integer(measurement["end_ns"], "measurement end", begin + 1)
        if (finish > end or begin != begin_boundary["timestamp_ns"]
                or finish != end_boundary["timestamp_ns"]
                or end_boundary["observer_start_ns"] < begin_boundary["observer_end_ns"]):
            raise ValueError("pause measurement does not match monotonic boundaries")
        if measurement_elapsed_ns is not None and finish - begin != measurement_elapsed_ns:
            raise ValueError("pause/GC window differs from measured Java duration")
        gc = trace.get("gc_exposure")
        fields(gc, "scope collections_completed_lower_bound collections_completed_upper_bound", "GC exposure")
        lower = integer(gc["collections_completed_lower_bound"], "GC lower bound")
        upper = integer(gc["collections_completed_upper_bound"], "GC upper bound", lower)
        if (gc["scope"] != "measurement"
                or lower != end_boundary["collections_before"] - begin_boundary["collections_after"]
                or upper != end_boundary["collections_after"] - begin_boundary["collections_before"]):
            raise ValueError("GC exposure differs from boundary counter samples")
        # Never clip away the part of a global event outside a measurement edge.
        touching = [(a, b) for a, b in intervals if a < finish and b > begin]
        measurement = {"elapsed_ns": finish - begin, "overlapping_events": len(touching),
                       "boundary_crossings": sum(a < begin or b > finish for a, b in touching),
                       "max_full_event_ns": max((b - a for a, b in touching), default=None),
                       "begin_observer_ns": begin_boundary["observer_end_ns"] - begin_boundary["observer_start_ns"],
                       "end_observer_ns": end_boundary["observer_end_ns"] - end_boundary["observer_start_ns"],
                       "gc_exposure": gc, "qualification": False}
    return {"diagnostic_only": True, "qualification": False,
            "capture_complete": False, "instrumented_capture_complete": complete,
            "source_coverage_complete": False,
            "event_count": len(events), "open_events": opened,
            "dropped_events": dropped, "clock_errors": errors,
            "max_observed_continuous_stall_ns": maximum,
            "observed_budget_violation": maximum is not None and maximum >= PAUSE_LIMIT_NS,
            "clock_resolves_budget": resolution < PAUSE_LIMIT_NS,
            "measurement": measurement}
