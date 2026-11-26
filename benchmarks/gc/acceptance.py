"""Version 2 native short-run evidence contract; never a release certificate.

Recorders and reviewed coordination audits must supply these data. This module
does not synthesize telemetry, audit collector code, or qualify lifetime profiles.
"""
import hashlib
import json
import math

VERSION = 2
PAUSE_LIMIT_NS = 1_000_000
MIN_PAIRS = 5
MIN_TAIL_COMPLETIONS = 1_000_000
GLOBAL_SOURCES = {"gc_safepoint", "publication", "allocation_recovery"}


def integer(value, name, minimum=0):
    if type(value) is not int or value < minimum:
        raise ValueError(f"{name}: expected integer >= {minimum}")
    return value


def text(value, name):
    if not isinstance(value, str) or not value.strip():
        raise ValueError(f"{name}: expected nonempty text")
    return value


def fields(value, names, name):
    if not isinstance(value, dict) or set(value) != set(names.split()):
        raise ValueError(f"{name}: missing or unknown fields")


def sha256(value, name):
    if (not isinstance(value, str) or len(value) != 64
            or any(c not in "0123456789abcdef" for c in value)):
        raise ValueError(f"{name}: expected SHA-256")
    return value


def canonical_hash(value):
    return hashlib.sha256(json.dumps(value, sort_keys=True, separators=(",", ":"),
                                     allow_nan=False).encode()).hexdigest()


def method_key(method):
    if not isinstance(method, dict):
        raise ValueError("method coverage must be an object")
    return tuple(text(method[k], k) for k in ["class", "name", "descriptor"])


def validate_policy(policy):
    fields(policy, "version candidate resources workloads coordination_audit", "policy")
    if type(policy["version"]) is not int or policy["version"] != VERSION:
        raise ValueError("expected acceptance policy version 2")
    candidate = text(policy["candidate"], "candidate")
    if not candidate.startswith("ristretto-") or not candidate.endswith("-jit"):
        raise ValueError("native qualification requires a production JIT candidate")
    resources = policy["resources"]
    fields(resources, "heap_mib memory_bytes cpu_quota_us cpu_period_us cpus", "resources")
    heap = integer(resources["heap_mib"], "heap_mib", 32)
    integer(resources["memory_bytes"], "memory_bytes", heap * 1024**2)
    quota = integer(resources["cpu_quota_us"], "cpu_quota_us", 1)
    period = integer(resources["cpu_period_us"], "cpu_period_us", 1)
    cpus = resources["cpus"]
    if not isinstance(cpus, list) or not cpus:
        raise ValueError("resources: nonempty CPU affinity required")
    for cpu in cpus:
        integer(cpu, "CPU")
    if len(set(cpus)) != len(cpus) or period != 100000 or quota > period * len(cpus):
        raise ValueError("resources: invalid quota, period or duplicate CPU")
    if not isinstance(policy["workloads"], dict) or not policy["workloads"]:
        raise ValueError("primary workload limits required")
    for workload, limits in policy["workloads"].items():
        if workload not in {"allocation", "roots", "mixed"}:
            raise ValueError("unknown primary workload")
        fields(limits, "response_p999_ns response_p9999_ns peak_backlog required_methods", workload)
        p999 = integer(limits["response_p999_ns"], "response_p999_ns", 1)
        integer(limits["response_p9999_ns"], "response_p9999_ns", p999)
        integer(limits["peak_backlog"], "peak_backlog")
        methods = limits["required_methods"]
        if not isinstance(methods, list) or not methods:
            raise ValueError("required hot-method manifest is empty")
        identities = set()
        for method in methods:
            fields(method, "class name descriptor min_entries entries_per_operation backedges_per_operation",
                   "required method")
            identity = method_key(method)
            if identity in identities:
                raise ValueError("duplicate required method")
            identities.add(identity)
            integer(method["min_entries"], "min_entries", 1)
            integer(method["entries_per_operation"], "entries_per_operation")
            integer(method["backedges_per_operation"], "backedges_per_operation")
    audit = policy["coordination_audit"]
    if audit is not None:
        fields(audit, "recorder binary_sha256 source_archive_sha256 coverage_complete "
               "unbounded_global_blocking review_reference global_stop_sources", "coordination audit")
        text(audit["recorder"], "recorder")
        text(audit["review_reference"], "review_reference")
        for key in ["binary_sha256", "source_archive_sha256"]:
            sha256(audit[key], key)
        for key in ["coverage_complete", "unbounded_global_blocking"]:
            if type(audit[key]) is not bool:
                raise ValueError(f"audit {key}: expected boolean")
        sources = audit["global_stop_sources"]
        if (not isinstance(sources, list) or any(not isinstance(s, str) for s in sources)
                or len(sources) != len(set(sources)) or set(sources) != GLOBAL_SOURCES):
            raise ValueError("audit must cover safepoints, publication and allocation recovery")
    return policy


def resource_limits(policy):
    r = policy["resources"]
    return {"memory.max": str(r["memory_bytes"]), "memory.swap.max": "0",
            "cpu.max": f"{r['cpu_quota_us']} {r['cpu_period_us']}"}


def validate_options(policy, options):
    validate_policy(policy)
    if set(policy["workloads"]) != set(options["workloads"]) - {"control"}:
        raise ValueError("declared limits must cover every primary workload")
    if not {policy["candidate"], "g1", "zgc"} <= set(options["runtimes"]):
        raise ValueError("candidate and both default-JIT peers required")
    r = policy["resources"]
    if options["heap_mib"] != r["heap_mib"] or options["cpus"] != r["cpus"]:
        raise ValueError("declared heap/affinity differs from run options")
    if (type(options.get("rss_mib")) is not int
            or options["rss_mib"] * 1024**2 != r["memory_bytes"]):
        raise ValueError("declared physical memory differs from run options")
    quota = options.get("cpu_quota")
    if (type(quota) not in [int, float] or not math.isfinite(quota)
            or round(quota * 100000) != r["cpu_quota_us"]):
        raise ValueError("declared CPU quota differs from run options")


def validate_pauses(pauses, audit, process_window):
    """Validate every event, then merge overlap before checking continuous stops."""
    if audit is None or audit["coverage_complete"] is not True:
        raise ValueError("complete reviewed coordination audit required")
    if audit["unbounded_global_blocking"] is not False:
        raise ValueError("unbounded global blocking cannot pass")
    if not isinstance(pauses, dict):
        raise ValueError("missing native pause telemetry")
    if (pauses.get("diagnostic_only") is True or pauses.get("qualification") is False
            or pauses.get("schema_version") != 1 or type(pauses.get("schema_version")) is not int
            or pauses.get("scope") != "whole_process"
            or pauses.get("recorder") != audit["recorder"]
            or pauses.get("capture_complete") is not True):
        raise ValueError("incomplete or incompatible native pause recorder")
    if (pauses.get("global_stop_sources") != sorted(GLOBAL_SOURCES)
            or integer(pauses.get("dropped_events"), "dropped pause events") != 0
            or integer(pauses.get("open_events"), "unfinished pause events") != 0):
        raise ValueError("pause events or source coverage missing")
    resolution = integer(pauses.get("clock_resolution_ns"), "pause clock resolution", 1)
    if resolution >= PAUSE_LIMIT_NS:
        raise ValueError("pause clock cannot resolve the acceptance budget")
    start, end = process_window
    if (integer(pauses.get("capture_start_ns"), "capture start") != start
            or integer(pauses.get("capture_end_ns"), "capture end", start + 1) != end):
        raise ValueError("pause capture does not cover the whole process window")
    events = pauses.get("events")
    if not isinstance(events, list):
        raise ValueError("pause event stream missing")
    intervals, rendezvous = [], 0
    for event in events:
        if (not isinstance(event, dict) or not isinstance(event.get("source"), str)
                or event["source"] not in GLOBAL_SOURCES):
            raise ValueError("unknown global pause source")
        requested = integer(event.get("requested_ns"), "pause request")
        stopped = integer(event.get("mutators_stopped_ns"), "rendezvous end", requested)
        released = integer(event.get("released_ns"), "pause release", stopped)
        if not start <= requested <= stopped <= released <= end or released <= requested:
            raise ValueError("pause timestamps outside capture or invalid duration")
        intervals.append((requested, released))
        rendezvous += stopped - requested
    merged = []
    for begin, finish in sorted(intervals):
        if merged and begin <= merged[-1][1]:
            merged[-1][1] = max(finish, merged[-1][1])
        else:
            merged.append([begin, finish])
    durations = sorted(finish - begin for begin, finish in merged)
    maximum = max(durations, default=0)
    if maximum >= PAUSE_LIMIT_NS:
        raise ValueError(f"native STW {maximum} ns is not below {PAUSE_LIMIT_NS} ns")
    total = sum(durations)
    return {"pass": True, "event_count": len(events), "stop_count": len(durations),
            "max_ns": maximum, "total_stopped_ns": total,
            "stopped_fraction": total / (end - start),
            "event_rendezvous_ns": rendezvous, "clock_resolution_ns": resolution,
            "stop_durations_ns": durations,
            "percentiles_descriptive_only": True,
            "percentiles_ns": ({name: durations[(len(durations) * rank + 9999) // 10000 - 1]
                                for name, rank in [("p50", 5000), ("p99", 9900),
                                                   ("p999", 9990), ("p9999", 9999)]}
                               if durations else None),
            "zero_observed_stops": not durations,
            "limit_ns": PAUSE_LIMIT_NS}


def validate_coverage(execution, required, completed):
    if (not isinstance(execution, dict) or execution.get("schema_version") != 1
            or type(execution.get("schema_version")) is not int
            or execution.get("scope") != "measurement" or execution.get("mode") != "jit"
            or execution.get("ready_before_measurement") is not True
            or execution.get("manifest_sha256") != canonical_hash(required)):
        raise ValueError("measured-window compiled coverage/readiness required")
    if integer(execution.get("unexpected_fallbacks"), "unexpected fallbacks") != 0:
        raise ValueError("unexpected execution fallback")
    integer(execution.get("generated_code_bytes"), "generated code bytes", 1)
    integer(execution.get("compilation_ns"), "compilation time")
    methods = execution.get("methods")
    if not isinstance(methods, list):
        raise ValueError("measured method counters missing")
    observed, instances = {}, set()
    for method in methods:
        identity = method_key(method)
        if identity in observed:
            raise ValueError("ambiguous method/loader coverage")
        instance = integer(method.get("instance"), "method instance", 1)
        if instance in instances:
            raise ValueError("duplicate method instance")
        instances.add(instance)
        for key in ["compiled_entries", "compiled_backedges", "interpreter_entries", "intrinsic_entries"]:
            integer(method.get(key), key)
        observed[identity] = method
    for method in required:
        actual = observed.get(method_key(method))
        if actual is None:
            raise ValueError("required hot method missing")
        minimum = max(method["min_entries"], completed * method["entries_per_operation"])
        if (actual["compiled_entries"] < minimum
                or actual["compiled_backedges"] < completed * method["backedges_per_operation"]
                or actual["interpreter_entries"] or actual["intrinsic_entries"]):
            raise ValueError("required hot method lacks compiled entries/backedges or fell back")
    return {"pass": True, "required_methods": len(required),
            "generated_code_bytes": execution["generated_code_bytes"],
            "compilation_ns": execution["compilation_ns"]}


def validate_packet(packet, row, policy, policy_sha, build, candidate=True):
    if (not isinstance(packet, dict) or packet.get("schema_version") != 1
            or type(packet.get("schema_version")) is not int
            or not row.get("run_id") or packet.get("run_id") != row["run_id"]
            or packet.get("policy_sha256") != policy_sha):
        raise ValueError("missing or unbound per-process qualification evidence")
    if row.get("pause_evidence") is not None or row.get("pause_observation") is not None:
        raise ValueError("partial diagnostic pause recordings cannot qualify timing")
    window = packet.get("window", {})
    if not isinstance(window, dict) or window.get("clock") != "process_monotonic_ns":
        raise ValueError("qualification window clock missing")
    start = integer(window.get("process_start_ns"), "process start")
    begin = integer(window.get("measurement_start_ns"), "measurement start", start)
    finish = integer(window.get("measurement_end_ns"), "measurement end", begin + 1)
    end = integer(window.get("process_end_ns"), "process end", finish)
    if finish - begin != row["measurement"]["elapsed_ns"]:
        raise ValueError("coverage/GC window differs from measured duration")
    gc = packet.get("gc_exposure", {})
    if (not isinstance(gc, dict) or gc.get("scope") != "measurement"
            or integer(gc.get("collections_completed"), "measured collections", 1) == 0):
        raise ValueError("measured-window GC activity required")
    if not candidate:
        return {"gc_exposure": gc}
    audit = policy["coordination_audit"]
    if (audit is None or build is None or audit["binary_sha256"] != build["binary_sha256"]
            or audit["source_archive_sha256"] != build["source_archive_sha256"]):
        raise ValueError("coordination audit does not match frozen binary/source build")
    pauses = validate_pauses(packet.get("pauses"), audit, (start, end))
    execution = validate_coverage(packet.get("execution"),
                                  policy["workloads"][row["workload"]]["required_methods"],
                                  row["measurement"]["completed"])
    if row.get("gc_observation", {}).get("ristretto", {}).get("jit_enabled") is not True:
        raise ValueError("VM did not confirm JIT execution mode")
    if row.get("gc_observation", {}).get("execution_profile") is True:
        raise ValueError("lifetime diagnostic profiles cannot qualify timing")
    return {"pauses": pauses, "execution": execution, "gc_exposure": gc}
