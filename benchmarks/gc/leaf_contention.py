#!/usr/bin/env python3
"""Finite canonical-leaf stress. Progress counts, not latency or JVM qualification."""
import argparse
import csv
import hashlib
import json
import os
from pathlib import Path
import platform
import random
import shutil
import signal
import subprocess
import time

CONFIG = {"mode", "actors", "helpers", "operations_per_actor", "capacity", "retained"}
FIELDS = CONFIG | {"version", "request_poll_limit", "objects", "catalog_bytes", "logical_object_bytes", "destructions",
                   "payload_teardown_bytes", "catalog_teardown_bytes", "counter_owners",
                   "actor_progress", "helper_progress"}
ACTOR = {"actor", "completed", "allocation_polls", "max_allocation_polls", "reads", "checksum"}
HELPER = {"helper", "polls", "released_objects", "released_roots"}


def digest(path):
    with Path(path).open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def unique(items):
    result = {}
    for key, value in items:
        if key in result:
            raise ValueError(f"duplicate JSON key: {key}")
        result[key] = value
    return result


def packet(text):
    return json.loads(text, object_pairs_hook=unique)


def integers(row, fields):
    if any(type(row[key]) is not int or row[key] < 0 for key in fields):
        raise ValueError("expected nonnegative integers")


def configuration(case):
    if not isinstance(case, dict) or set(case) != CONFIG:
        raise ValueError("unexpected configuration schema")
    integers(case, CONFIG - {"mode"})
    if (case["mode"] not in ["native", "cooperative"] or not 1 <= case["actors"] <= 16
            or not 1 <= case["helpers"] <= 8 or not 1 <= case["operations_per_actor"] <= 10000000
            or not 1 <= case["capacity"] <= 16384 or not 0 <= case["retained"] <= 64
            or case["capacity"] < case["actors"] * (case["retained"] + 1)):
        raise ValueError("invalid workload bounds")
    return case


def validate(row, case):
    configuration(case)
    if not isinstance(row, dict) or set(row) != FIELDS:
        raise ValueError("unexpected result schema")
    integers(row, FIELDS - {"mode", "actor_progress", "helper_progress"})
    if any(row[key] != value for key, value in case.items()) or row["version"] != 2:
        raise ValueError("workload identity mismatch")
    if row["request_poll_limit"] != max(1000000, case["capacity"] * 1024):
        raise ValueError("request watchdog differs from configuration")
    operations = case["operations_per_actor"]
    objects = case["actors"] * operations
    if (row["objects"] != objects or row["destructions"] != objects or not row["catalog_bytes"]
            or row["logical_object_bytes"] < 32 or row["payload_teardown_bytes"] != 0
            or row["catalog_teardown_bytes"] != 0 or row["counter_owners"] != 1):
        raise ValueError("incomplete destruction/accounting teardown")
    if (not isinstance(row["actor_progress"], list) or len(row["actor_progress"]) != case["actors"]
            or not isinstance(row["helper_progress"], list) or len(row["helper_progress"]) != case["helpers"]):
        raise ValueError("missing participant progress")
    k = min(operations, case["retained"])
    reads = operations + k * (k + 1) // 2 + max(operations - case["retained"], 0) * case["retained"]
    for actor, stats in enumerate(row["actor_progress"]):
        if not isinstance(stats, dict) or set(stats) != ACTOR:
            raise ValueError("unexpected actor schema")
        integers(stats, ACTOR)
        if (stats["actor"] != actor or stats["completed"] != operations or stats["reads"] != reads
                or stats["checksum"] != actor * operations + operations * (operations - 1) // 2
                or not 1 <= stats["max_allocation_polls"] <= row["request_poll_limit"]
                or not operations - 1 + stats["max_allocation_polls"] <= stats["allocation_polls"]
                <= operations * stats["max_allocation_polls"]):
            raise ValueError("invalid individual actor progress")
    for helper, stats in enumerate(row["helper_progress"]):
        if not isinstance(stats, dict) or set(stats) != HELPER:
            raise ValueError("unexpected helper schema")
        integers(stats, HELPER)
        if stats["helper"] != helper or stats["polls"] < max(1, stats["released_objects"] + stats["released_roots"]):
            raise ValueError("invalid individual helper progress")
    if any(sum(s[key] for s in row["helper_progress"]) != objects for key in ["released_objects", "released_roots"]):
        raise ValueError("incomplete physical slot release")
    return row


def dump(path, value):
    path.write_text(json.dumps(value, indent=2) + "\n")


def run(args):
    cases = packet(args.cases.read_text())
    if not isinstance(cases, list) or not cases:
        raise ValueError("need a nonempty case list")
    for case in cases:
        configuration(case)
    if len({json.dumps(case, sort_keys=True) for case in cases}) != len(cases):
        raise ValueError("duplicate cases")
    if not 1 <= args.repeats <= 100 or not 1 <= args.timeout <= 86400:
        raise ValueError("invalid repeat/timeout bound")
    if not args.cpus or len(set(args.cpus)) != len(args.cpus) or not set(args.cpus) <= os.sched_getaffinity(0):
        raise ValueError("need distinct available CPUs")
    with args.binary.open("rb") as stream:
        wasm = stream.read(4) == b"\0asm"
    if wasm != bool(args.wasmtime) or (wasm and any(c["mode"] != "cooperative" for c in cases)):
        raise ValueError("WASI requires its runner and cooperative cases")
    provenance = packet(args.provenance.read_text())
    if provenance["binary_sha256"] != digest(args.binary) or provenance["source_archive_sha256"] != digest(args.sources):
        raise ValueError("build provenance mismatch")
    out = args.output.resolve()
    out.mkdir(parents=True, exist_ok=False)
    inputs = {}
    for name, source in [("binary", args.binary), ("build.json", args.provenance), ("sources.tar.gz", args.sources),
                         ("cases.json", args.cases), ("leaf_contention.py", Path(__file__)), ("wasmtime", args.wasmtime)]:
        if source:
            shutil.copy2(source, out / name)
            inputs[name] = digest(out / name)
    schedule = [(repeat, case) for repeat in range(args.repeats) for case in range(len(cases))]
    random.Random(args.seed).shuffle(schedule)
    dump(out / "metadata.json", dict(version=1, qualification=False, target="WASI" if wasm else "native",
        inputs=inputs, cases=cases, cpus=args.cpus, repeats=args.repeats, seed=args.seed, timeout=args.timeout,
        host=platform.platform(), scope="finite leaf stress; elapsed includes startup, checks and teardown; no performance claim"))
    dump(out / "schedule.json", schedule)
    failures = []
    with (out / "raw.jsonl").open("w") as raw:
        for index, (repeat, case_id) in enumerate(schedule):
            case = cases[case_id]
            arguments = [case["mode"], *[str(case[k]) for k in ["actors", "helpers", "operations_per_actor", "capacity", "retained"]]]
            command = ([str(out / "wasmtime"), "run", "-W", "max-wasm-stack=8388608", "--argv0", "leaf_contention", str(out / "binary")]
                       if wasm else ["leaf_contention"]) + arguments
            row = dict(index=index, repeat=repeat, case=case_id, command=command, status="failed")
            started = time.monotonic_ns()
            child = subprocess.Popen(command, executable=None if wasm else str(out / "binary"),
                stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, start_new_session=True,
                preexec_fn=lambda: os.sched_setaffinity(0, args.cpus))
            try:
                stdout, stderr = child.communicate(timeout=args.timeout)
                row.update(stdout=stdout, stderr=stderr, exit_code=child.returncode)
                if child.returncode:
                    raise ValueError("process failed")
                row["result"] = validate(packet(stdout), case)
                row["status"] = "ok"
            except subprocess.TimeoutExpired:
                os.killpg(child.pid, signal.SIGKILL)
                stdout, stderr = child.communicate()
                row.update(stdout=stdout, stderr=stderr, exit_code=child.returncode, error="watchdog timeout; incomplete work")
            except (ValueError, TypeError, KeyError) as error:
                row["error"] = str(error)
            row["process_elapsed_ns"] = time.monotonic_ns() - started
            if row["status"] != "ok":
                failures.append(index)
            raw.write(json.dumps(row) + "\n")
            raw.flush()
            print(f"stress {index + 1}/{len(schedule)} case={case_id} repeat={repeat} {row['status']}", flush=True)
    unchanged = all(digest(out / name) == expected for name, expected in inputs.items())
    dump(out / "completion.json", dict(failures=failures, inputs_unchanged=unchanged, qualification=False,
        raw_sha256=digest(out / "raw.jsonl"), schedule_sha256=digest(out / "schedule.json"), metadata_sha256=digest(out / "metadata.json")))
    return int(bool(failures) or not unchanged)


def checked_rows(out):
    metadata = packet((out / "metadata.json").read_text())
    completion = packet((out / "completion.json").read_text())
    schedule = packet((out / "schedule.json").read_text())
    rows = [packet(line) for line in (out / "raw.jsonl").read_text().splitlines()]
    if (not completion["inputs_unchanged"] or any(digest(out / n) != h for n, h in metadata["inputs"].items())
            or any(digest(out / name) != completion[key] for name, key in [("raw.jsonl", "raw_sha256"),
                ("schedule.json", "schedule_sha256"), ("metadata.json", "metadata_sha256")])):
        raise ValueError("changed frozen evidence")
    expected = {(r, c) for r in range(metadata["repeats"]) for c in range(len(metadata["cases"]))}
    if len(schedule) != len(expected) or {tuple(entry) for entry in schedule} != expected or len(rows) != len(schedule):
        raise ValueError("incomplete schedule")
    failures = []
    for index, (row, entry) in enumerate(zip(rows, schedule)):
        if row["index"] != index or [row["repeat"], row["case"]] != entry:
            raise ValueError("row identity mismatch")
        if row["status"] == "ok":
            result = validate(packet(row["stdout"]), metadata["cases"][row["case"]])
            if row["exit_code"] != 0 or result != row["result"]:
                raise ValueError("raw process evidence differs from result")
        elif row["status"] == "failed":
            failures.append(index)
        else:
            raise ValueError("invalid process status")
    if failures != completion["failures"]:
        raise ValueError("failure accounting mismatch")
    return metadata, rows, failures


def report(args):
    metadata, rows, failures = checked_rows(args.output)
    lines = ["# Canonical leaf contention", "", metadata["scope"], "",
             f"Target: {metadata['target']}; CPUs: {metadata['cpus']}; failures: {len(failures)}/{len(rows)}.", "",
             "Every successful process verifies each actor's exact identities and completed work, all payload destructions, both slot-release counts and complete logical-credit refund.", "",
             "| Case | Mode | Actors / helpers | Slots / retained per actor | Successful runs | Objects per run |",
             "|---:|---|---:|---:|---:|---:|"]
    summary = []
    for i, case in enumerate(metadata["cases"]):
        selected = [r for r in rows if r["case"] == i]
        good = [r["result"] for r in selected if r["status"] == "ok"]
        lines.append(f"| {i} | {case['mode']} | {case['actors']} / {case['helpers']} | {case['capacity']} / {case['retained']} | {len(good)}/{len(selected)} | {case['actors'] * case['operations_per_actor']} |")
        summary.append(dict(case=i, **case, successes=len(good), runs=len(selected),
            allocation_polls_per_object=[sum(a['allocation_polls'] for a in r['actor_progress']) / r['objects'] for r in good],
            max_allocation_polls=[max(a['max_allocation_polls'] for a in r['actor_progress']) for r in good]))
    lines += ["", "This is finite controlled-leaf stress, not a fairness proof, pause/latency result, general graph/callback test, G1/ZGC comparison or sustained-load qualification."]
    if failures:
        lines += ["", "Failed process indices: " + ", ".join(map(str, failures)) + ". No success-only graph is generated."]
    (args.output / "REPORT.md").write_text("\n".join(lines) + "\n")
    dump(args.output / "summary.json", dict(cases=summary, failures=failures, qualification=False))
    with (args.output / "progress.csv").open("w", newline="") as stream:
        writer = csv.writer(stream)
        writer.writerow(["index", "case", "repeat", "status", "actor", "completed", "allocation_polls", "max_allocation_polls", "reads", "checksum"])
        for row in rows:
            actors = row.get("result", {}).get("actor_progress", [{}]) if row["status"] == "ok" else [{}]
            for actor in actors:
                writer.writerow([row[k] for k in ["index", "case", "repeat", "status"]] + [actor.get(k, "") for k in ["actor", "completed", "allocation_polls", "max_allocation_polls", "reads", "checksum"]])
    if args.plot and not failures:
        import matplotlib.pyplot as plt
        figure, axes = plt.subplots(1, 2, figsize=(12, 5), layout="constrained")
        for axis, field, title in zip(axes, ["allocation_polls_per_object", "max_allocation_polls"],
                                     ["Allocation polls per completed object", "Largest request poll count"]):
            for row in summary:
                axis.scatter([row["case"]] * len(row[field]), row[field], s=25, alpha=.65, color="#0369a1")
            axis.set_xticks(range(len(summary)), [str(r["case"]) for r in summary])
            axis.set_xlabel("Case (configuration in REPORT.md)")
            axis.set_ylabel("Poll count; each point is one process")
            axis.set_title(title)
            axis.set_yscale("log")
            axis.grid(axis="y", alpha=.2)
        figure.suptitle(f"{metadata['target']} finite leaf stress — counts, not latency")
        for extension in ["svg", "png"]:
            figure.savefig(args.output / f"progress.{extension}", dpi=160)
    return int(bool(failures))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="command", required=True)
    execute = commands.add_parser("run")
    for name in ["binary", "provenance", "sources", "cases", "output"]:
        execute.add_argument("--" + name, type=Path, required=True)
    execute.add_argument("--wasmtime", type=Path)
    execute.add_argument("--cpus", type=int, nargs="+", required=True)
    execute.add_argument("--repeats", type=int, default=3)
    execute.add_argument("--timeout", type=int, default=600)
    execute.add_argument("--seed", type=int, default=20261115)
    summarize = commands.add_parser("report")
    summarize.add_argument("--output", type=Path, required=True)
    summarize.add_argument("--plot", action="store_true")
    args = parser.parse_args()
    return run(args) if args.command == "run" else report(args)


if __name__ == "__main__":
    raise SystemExit(main())
