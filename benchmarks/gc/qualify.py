#!/usr/bin/env python3
"""Evaluate versioned native short-run evidence; never certify full qualification."""
import argparse
from collections import defaultdict
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path

from acceptance import (MIN_PAIRS, MIN_TAIL_COMPLETIONS, PAUSE_LIMIT_NS, VERSION,
                        canonical_hash, integer, resource_limits, sha256, validate_options,
                        validate_packet)
from mechanisms import paired_interval
from report import quantile, validate

REMAINING = [
    "validated warmup, compilation stability and recorder overhead",
    "calibrated sustainable capacity and declared operating envelope",
    "bounded backlog and live/retired/RSS memory time series",
    "forced stalled-publisher/callback progress and coordination qualification",
    "three paired one-hour trials and a separate 24-hour candidate trial",
    "Wasm host-turn and browser qualification",
]


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def frozen_policy(directory, metadata, options):
    declaration = metadata.get("acceptance_policy")
    if (not isinstance(declaration, dict) or declaration.get("version") != VERSION
            or type(declaration.get("version")) is not int):
        raise ValueError("no predeclared acceptance policy v2; historical data is not reclassified")
    if declaration.get("path") != "acceptance-policy.json":
        raise ValueError("unexpected frozen policy path")
    expected = sha256(declaration.get("sha256"), "frozen policy")
    if (metadata.get("artifacts_sha256", {}).get("acceptance-policy.json") != expected
            or digest(directory / "acceptance-policy.json") != expected):
        raise ValueError("frozen acceptance policy hash mismatch")
    policy = json.loads((directory / "acceptance-policy.json").read_text())
    validate_options(policy, options)
    return policy, expected


def frozen_build(directory, metadata):
    artifacts = metadata.get("artifacts_sha256", {})
    for name, expected in artifacts.items():
        path = Path(name)
        if path.is_absolute() or ".." in path.parts:
            raise ValueError("invalid frozen artifact path")
        if digest(directory / path) != sha256(expected, "artifact hash"):
            raise ValueError(f"frozen artifact changed: {name}")
    if digest(directory / "sources.tar.gz") != metadata.get("source_archive_sha256"):
        raise ValueError("runner source archive changed or missing")
    names = ["gc_compare", "gc_compare.provenance.json", "gc_compare.sources.tar.gz"]
    for name in names:
        if digest(directory / name) != artifacts.get(name):
            raise ValueError(f"unverified frozen build artifact: {name}")
    build = json.loads((directory / names[1]).read_text())
    if (build.get("binary_sha256") != artifacts[names[0]]
            or build.get("source_archive_sha256") != artifacts[names[2]]):
        raise ValueError("binary/source provenance mismatch")
    return build


def process_output(directory, row):
    relative = Path(row["directory"])
    if relative.is_absolute() or ".." in relative.parts:
        raise ValueError("invalid process output path")
    output = directory / relative
    hashes = row.get("output_sha256", {})
    if not {"stdout.txt", "stderr.txt"} <= set(hashes):
        raise ValueError("captured process output hashes missing")
    for name, expected in hashes.items():
        if name not in {"stdout.txt", "stderr.txt", "resource.txt", "gc.log", "process_identity.json"}:
            raise ValueError("unexpected captured output path")
        if digest(output / name) != sha256(expected, "process output hash"):
            raise ValueError("captured process output changed")
    lines = (output / "stdout.txt").read_text().splitlines()
    if any(line.startswith("RISTRETTO_PAUSES ") for line in lines):
        raise ValueError("partial diagnostic pause recordings cannot qualify timing")
    for marker, field in [("GC_RESULT ", "measurement"),
                          ("GC_QUALIFICATION ", "qualification_evidence")]:
        packets = [json.loads(line[len(marker):]) for line in lines if line.startswith(marker)]
        if len(packets) != 1 or canonical_hash(packets[0]) != canonical_hash(row.get(field)):
            raise ValueError(f"{field} differs from captured process output or is missing")
    if row["runtime"].startswith("ristretto-"):
        observation = row.get("gc_observation")
        if not isinstance(observation, dict):
            raise ValueError("runtime mode observation missing")
        modes = [json.loads(line[len("RISTRETTO_GC "):])
                 for line in lines if line.startswith("RISTRETTO_GC ")]
        profiled = any(line.startswith("RISTRETTO_EXECUTION ") for line in lines)
        if (len(modes) != 1 or canonical_hash(modes[0]) != canonical_hash(observation.get("ristretto"))
                or profiled != observation.get("execution_profile", False)):
            raise ValueError("runtime mode differs from captured process output")


def _evaluate_v2(directory, candidate):
    metadata = json.loads((directory / "metadata.json").read_text())
    options = metadata["options"]
    rows = [json.loads(line) for line in (directory / "raw.jsonl").read_text().splitlines()]
    failures, process_results = [], []
    policy, policy_sha, build = None, None, None
    try:
        policy, policy_sha = frozen_policy(directory, metadata, options)
        if candidate is not None and candidate != policy["candidate"]:
            raise ValueError("candidate differs from the predeclared policy")
    except (OSError, ValueError, KeyError, TypeError) as error:
        failures.append(str(error))
    candidate = candidate or (policy["candidate"] if policy else "ristretto-candidate")
    if policy:
        try:
            build = frozen_build(directory, metadata)
        except (OSError, ValueError, KeyError, TypeError) as error:
            failures.append(str(error))
    if json.loads((directory / "integrity.json").read_text()).get("inputs_unchanged") is not True:
        failures.append("inputs changed or integrity evidence missing")
    repeats = integer(options["repeats"], "process repetitions", 1)
    expected = set(range(repeats))
    rates = options["rates"]
    if (not isinstance(rates, list) or not rates or len(set(rates)) != len(rates)
            or any(type(r) is not int or r < 0 for r in rates)):
        raise ValueError("invalid offered-rate matrix")
    workloads = sorted(set(options["workloads"]) - {"control"})
    if not workloads or 0 not in rates or not any(r > 0 for r in rates):
        failures.append("need primary workloads, saturation and common fixed offered rates")
    selected = {candidate, "g1", "zgc"}
    expected_schedule = {(i, work, rate, runtime) for i in expected
                         for work in workloads for rate in rates for runtime in selected}
    observed_schedule, seen_ids, duplicates = set(), set(), set()
    groups, valid_rows, limits = defaultdict(dict), set(), set()
    for row in rows:
        if row["runtime"] not in selected or row["workload"] == "control":
            continue
        identity = (row["repeat"], row["workload"], row["rate"], row["runtime"])
        key = (row["workload"], row["rate"], row["runtime"])
        if identity not in expected_schedule:
            failures.append(f"unexpected process: {identity}")
        if identity in observed_schedule:
            failures.append(f"duplicate process pair: {identity}")
            duplicates.add(identity)
        observed_schedule.add(identity)
        groups[key][row["repeat"]] = row
        process = {"directory": row["directory"], "runtime": row["runtime"], "pass": False}
        try:
            if row["status"] != "ok":
                raise ValueError(row["status"])
            if policy and (type(row.get("exit_code")) is not int or row["exit_code"] != 0):
                raise ValueError("successful process exit required")
            data = row["measurement"]
            validate(data, row["workload"], row["rate"], options["seconds"], options["seed"])
            if integer(data["schema"], "measurement schema", 1) != 1:
                raise ValueError("unknown measurement schema")
            integer(data["completed"], "completed work", 1)
            integer(data["elapsed_ns"], "elapsed time", 1)
            integer(data["duration_ns"], "declared duration", 1)
            integer(data["unfinished"], "unfinished requests")
            for metric in ["service", "response"]:
                integer(data[metric]["max_ns"], "maximum duration")
                for upper, count in data[metric]["buckets_us"]:
                    integer(upper, "histogram bound")
                    integer(count, "histogram count", 1)
            if row["target"] != "native":
                raise ValueError("external qualification requires native peers")
            if data["unfinished"]:
                raise ValueError("unfinished requests censor latency")
            if row["rate"] and data["completed"] < MIN_TAIL_COMPLETIONS:
                raise ValueError("fewer than one million completions for extreme-tail claims")
            budget = row.get("cgroup")
            if not isinstance(budget, dict):
                raise ValueError("no enforced CPU/RSS ceiling")
            for event in ["oom", "oom_kill"]:
                if integer(budget["memory_events"][event], event):
                    raise ValueError("memory budget exhausted")
            integer(budget["memory_peak"], "charged memory peak", 1)
            limits.add(json.dumps(budget["limits"], sort_keys=True))
            if policy:
                process_output(directory, row)
                if budget["limits"] != resource_limits(policy):
                    raise ValueError("resource ceiling differs from predeclared policy")
                if budget["memory_peak"] > policy["resources"]["memory_bytes"]:
                    raise ValueError("charged memory exceeds declared ceiling")
                run_id = row.get("run_id")
                if not isinstance(run_id, str) or not run_id or run_id in seen_ids:
                    raise ValueError("missing or reused process identity")
                seen_ids.add(run_id)
            valid_rows.add(identity)
            if row["runtime"] == candidate:
                if policy is None:
                    raise ValueError("candidate lacks a frozen current acceptance policy")
                bound = policy["workloads"][row["workload"]]
                if row["rate"]:
                    for metric, rank in [("response_p999_ns", 9990), ("response_p9999_ns", 9999)]:
                        if quantile(data["response"], rank) * 1000 > bound[metric]:
                            raise ValueError(f"{metric}: predeclared workload latency limit exceeded")
                    if integer(data["peak_backlog"], "peak backlog") > bound["peak_backlog"]:
                        raise ValueError("predeclared backlog limit exceeded")
                process.update(validate_packet(row.get("qualification_evidence"), row,
                                               policy, policy_sha, build))
            elif policy:
                process.update(validate_packet(row.get("qualification_evidence"), row,
                                               policy, policy_sha, build, candidate=False))
            process["pass"] = True
        except (OSError, ValueError, KeyError, TypeError, ZeroDivisionError) as error:
            process["error"] = str(error)
            failures.append(f"{row['directory']}: {error}")
        process_results.append(process)
    if len(limits) != 1:
        failures.append("a single equal enforced resource policy is required")
    if observed_schedule != expected_schedule:
        failures.append("process schedule incomplete or unexpected")
    try:
        scheduled = [tuple(job) for job in json.loads((directory / "schedule.json").read_text())
                     if job[3] in selected and job[1] != "control"]
        if len(scheduled) != len(set(scheduled)) or set(scheduled) != expected_schedule:
            raise ValueError("frozen schedule differs from declared matrix")
    except (OSError, ValueError, KeyError, TypeError, IndexError) as error:
        failures.append(f"schedule: {error}")

    valid_rows.difference_update(duplicates)
    # Ratios remain diagnostics, including when the candidate is much slower.
    # A missing/censored pair is never discarded to compute a favorable interval.
    results = []
    for work in workloads:
        for rate in rates:
            for peer in ["g1", "zgc"]:
                a, b = groups[work, rate, peer], groups[work, rate, candidate]
                if set(a) != expected or set(b) != expected or repeats < MIN_PAIRS:
                    failures.append(f"{work}/{rate}/{peer}: need at least five complete independent process pairs")
                metrics = ["throughput"] if rate == 0 else ["p999", "p9999"]
                for metric in metrics:
                    result = {"workload": work, "rate": rate, "peer": peer, "metric": metric,
                              "role": "diagnostic", "candidate_peer_ratio": None}
                    try:
                        if set(a) != expected or set(b) != expected or repeats < MIN_PAIRS:
                            raise ValueError("incomplete process pairs")
                        def value(row):
                            identity = (row["repeat"], row["workload"], row["rate"], row["runtime"])
                            if identity not in valid_rows:
                                raise ValueError("invalid/censored process pair")
                            data = row["measurement"]
                            if metric == "throughput":
                                return data["completed"] * 1e9 / data["elapsed_ns"]
                            return quantile(data["response"], 9990 if metric == "p999" else 9999)
                        pairs = [(value(a[i]), value(b[i])) for i in sorted(expected)]
                        if any(x <= 0 or y <= 0 for x, y in pairs):
                            result["unavailable_reason"] = "undefined or resolution-limited ratio"
                        else:
                            result["candidate_peer_ratio"] = paired_interval(pairs)
                    except (ValueError, KeyError, TypeError, ZeroDivisionError) as error:
                        result["unavailable_reason"] = str(error)
                    results.append(result)
    return {"policy_version": VERSION, "policy_id": "native-stw-v2",
            "historical_only": False, "policy_sha256": policy_sha,
            "short_comparison_gate": bool(results) and not failures,
            "qualification": False, "candidate": candidate,
            "pause_limit_ns_exclusive": PAUSE_LIMIT_NS,
            "results": results, "processes": process_results,
            "failures": failures, "remaining_gates": REMAINING}


def evaluate(directory, candidate=None, policy_version=VERSION):
    directory = Path(directory)
    if policy_version == 1:
        from qualify_legacy import evaluate as legacy
        result = legacy(directory, candidate or "ristretto-candidate")
        result.update(policy_version=1, policy_id="legacy-peer-v1", historical_only=True)
    elif policy_version == VERSION:
        try:
            result = _evaluate_v2(directory, candidate)
        except (OSError, ValueError, KeyError, TypeError, IndexError, ZeroDivisionError) as error:
            result = {"policy_version": VERSION, "policy_id": "native-stw-v2",
                      "historical_only": False, "short_comparison_gate": False,
                      "qualification": False, "results": [], "processes": [],
                      "failures": [f"invalid evidence: {error}"], "remaining_gates": REMAINING}
    else:
        raise ValueError("unknown acceptance policy version")
    result["evaluated_utc"] = datetime.now(timezone.utc).isoformat()
    result["evaluator_sha256"] = {name: digest(Path(__file__).with_name(name))
                                 for name in ["qualify.py", "qualify_legacy.py", "acceptance.py",
                                              "report.py", "mechanisms.py"]}
    result["input_sha256"] = {name: digest(directory / name)
                              for name in ["metadata.json", "raw.jsonl", "integrity.json",
                                           "schedule.json", "acceptance-policy.json"]
                              if (directory / name).is_file()}
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("directory", type=Path)
    parser.add_argument("--candidate")
    parser.add_argument("--policy-version", type=int, choices=[1, VERSION], default=VERSION,
                        help="v1 is historical replay only; v2 is the current plan")
    parser.add_argument("--output", type=Path, help="new output path; existing reports are never overwritten")
    args = parser.parse_args()
    result = evaluate(args.directory, args.candidate, args.policy_version)
    output = args.output or args.directory / f"gates-v{args.policy_version}.json"
    try:
        with output.open("x") as stream:
            stream.write(json.dumps(result, indent=2, allow_nan=False) + "\n")
    except FileExistsError:
        parser.error(f"report already exists: {output}; choose a new --output path")
    print(json.dumps({"policy_version": result["policy_version"],
                      "historical_only": result["historical_only"],
                      "short_comparison_gate": result["short_comparison_gate"],
                      "failures": len(result["failures"]), "qualification": False}))
    return int(not result["short_comparison_gate"])


if __name__ == "__main__":
    raise SystemExit(main())
