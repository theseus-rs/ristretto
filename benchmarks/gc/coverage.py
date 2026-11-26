#!/usr/bin/env python3
"""Run separate diagnostic forks and report actual execution of required hot methods.

Lifetime and checkpoint-interval counts are not CPU shares or performance results.
Taken backedges are explicit control-flow edges, not inferred loop iterations.
The main-invocation interval still includes warmup and reporting; it is not the
measurement window. Missing/failed forks remain visible.
"""
import argparse
import json
import os
from pathlib import Path
import platform
import shutil
import subprocess
import time
from types import SimpleNamespace

from cgroups import Budget
from report import validate
from run import ROOT, HERE, capture, command_for, digest, execute, freeze_build, freeze_sources, optional_text


CORE = [("GcBench", "main", "([Ljava/lang/String;)V"),
        ("GcBench", "operation", "(IJI)I"),
        ("GcBench$Histogram", "record", "(J)V")]
NODE = ("GcBench$Node", "<init>", "(ILGcBench$Node;)V")


def coverage(profile, workload, completed):
    if (profile.get("schema_version") != 1 or profile.get("diagnostic_only") is not True
            or profile.get("qualification") is not False):
        raise ValueError("expected explicit diagnostic execution profile")
    seen = set()
    for method in profile["methods"]:
        if method["instance"] in seen:
            raise ValueError("duplicate method instance")
        seen.add(method["instance"])
        for key in ["instance", "compiled_entries", "interpreter_entries", "intrinsic_entries",
                    "queued_requests", "compilation_attempts", "compilation_successes",
                    "compilation_failures", "compilation_ns", "generated_code_bytes"]:
            if type(method[key]) is not int or method[key] < 0:
                raise ValueError("invalid execution counter")
        if method["compiled_entries"] and not method["compilation_successes"]:
            raise ValueError("compiled entries without compiled code")
        for key in ["compiled_backedges", "compiled_resumes"]:
            if key in method:
                if type(method[key]) is not int or method[key] < 0:
                    raise ValueError("invalid native activity counter")
                if method[key] and not method["compiled_entries"]:
                    raise ValueError("lifetime native activity without a compiled entry")
    required = CORE + ([NODE] if workload == "roots" else [])
    result = []
    for identity in required:
        instances = [m for m in profile["methods"]
                     if (m["class"], m["name"], m["descriptor"]) == identity]
        # The fixture has one loader. Do not merge an unrelated same-name method
        # into apparent coverage; other applications need loader-aware manifests.
        if len(instances) > 1:
            raise ValueError("ambiguous required method instances")
        if not instances:
            result.append({"method": ".".join(identity[:2]) + identity[2], "status": "missing"})
            continue
        method = instances[0]
        total = sum(method[k] for k in ["compiled_entries", "interpreter_entries", "intrinsic_entries"])
        minimum = 1 if identity == CORE[0] else completed * (16 if identity == NODE else 2 if identity == CORE[2] else 1)
        if total < minimum:
            raise ValueError("entry counts cannot account for completed fixture work")
        result.append({"method": ".".join(identity[:2]) + identity[2],
                       "status": "compiled_observed" if method["compiled_entries"] else "no_compiled_entry",
                       **method})
    return {"required_methods": result, "qualification": False,
            "coverage_gate_passed": False,
            "unresolved": ["measurement-phase coverage and hot-loop execution remain unqualified",
                           "complete dependency/observer manifest and CPU attribution pending"]}


def interval_profile(profile):
    """Validate diagnostic deltas without mistaking prior compilation for missing code."""
    if (profile.get("schema_version") != 1 or profile.get("diagnostic_only") is not True
            or profile.get("qualification") is not False or profile.get("scope") != "checkpoint_interval"):
        raise ValueError("expected diagnostic checkpoint interval")
    for field in ["boundary_span_ns", "start_snapshot_ns", "end_snapshot_ns"]:
        value = profile[field]
        if value is not None and (type(value) is not int or value < 0):
            raise ValueError("invalid snapshot timing")
    times = [profile[k] for k in ["boundary_span_ns", "start_snapshot_ns", "end_snapshot_ns"]]
    if any(t is None for t in times) != all(t is None for t in times):
        raise ValueError("partial snapshot timing")
    if times[0] is not None and times[0] < times[1] + times[2]:
        raise ValueError("snapshot boundaries exceed the enclosing interval")
    seen = set()
    for method in profile["methods"]:
        for key in ["instance", "compiled_entries", "compiled_backedges", "interpreter_entries",
                    "intrinsic_entries", "queued_requests", "compilation_attempts",
                    "compilation_successes", "compilation_failures", "compilation_ns",
                    "generated_code_bytes", "code_bytes_at_start", "code_bytes_at_end"]:
            if type(method[key]) is not int or method[key] < 0:
                raise ValueError("invalid interval counter")
        if method["instance"] == 0 or method["instance"] in seen:
            raise ValueError("duplicate/invalid interval method instance")
        seen.add(method["instance"])
        for key in ["present_at_start", "compiled_before_start"]:
            if type(method[key]) is not bool:
                raise ValueError("invalid interval readiness")
        if method["compiled_before_start"] and not method["present_at_start"]:
            raise ValueError("compilation predates an absent method")
        if method["code_bytes_at_start"] + method["generated_code_bytes"] != method["code_bytes_at_end"]:
            raise ValueError("code-byte interval mismatch")
        if "compiled_resumes" in method and (type(method["compiled_resumes"]) is not int or method["compiled_resumes"] < 0):
            raise ValueError("invalid native resume counter")
        if (method["compiled_entries"] or method["compiled_backedges"] or method.get("compiled_resumes", 0)) and not method["code_bytes_at_end"]:
            raise ValueError("compiled activity without recorded code")
    return profile


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, required=True, help="frozen execution-profile build with provenance")
    parser.add_argument("--jdk", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--wasmtime", type=Path)
    parser.add_argument("--policies", nargs="+", choices=["baseline", "candidate", "baseline-jit", "candidate-jit"], default=["candidate-jit"])
    parser.add_argument("--workloads", nargs="+", choices=["allocation", "roots", "mixed"], default=["allocation", "roots", "mixed"])
    parser.add_argument("--seconds", type=int, default=1)
    parser.add_argument("--warmup", type=int, default=1)
    parser.add_argument("--seed", type=int, default=42)
    parser.add_argument("--heap-mib", type=int, default=64)
    parser.add_argument("--rss-mib", type=int, default=512)
    parser.add_argument("--workers", type=int, default=1)
    parser.add_argument("--cpus", type=int, nargs="+", default=[0, 2])
    parser.add_argument("--cgroup-parent", type=Path)
    parser.add_argument("--cpu-quota", type=float, default=2)
    parser.add_argument("--timeout", type=int, default=180)
    parser.add_argument("--plot", action="store_true")
    args = parser.parse_args()
    if (args.seconds <= 0 or args.warmup < 0 or args.workers < 0 or args.heap_mib <= 0
            or args.rss_mib <= 0 or args.cpu_quota <= 0 or args.timeout <= 0
            or not set(args.cpus) <= os.sched_getaffinity(0)):
        parser.error("invalid duration, resource budget or CPUs")
    wasm = args.binary.read_bytes()[:4] == b"\0asm"
    if wasm != bool(args.wasmtime) or (wasm and any(p.endswith("-jit") for p in args.policies)):
        parser.error("Wasm requires --wasmtime and interpreter policies")
    if wasm:
        args.workers = 0
    args.jdk = args.jdk.resolve()
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    binary = output / ("gc_compare.wasm" if wasm else "gc_compare")
    artifacts = freeze_build(args.binary.resolve(), binary)
    if wasm:
        runtime = output / "wasmtime"
        shutil.copy2(args.wasmtime, runtime)
        args.wasmtime = runtime
        artifacts.append(runtime)
    classes = output / "classes"
    classes.mkdir()
    environment = os.environ.copy()
    for key in ["JAVA_TOOL_OPTIONS", "_JAVA_OPTIONS", "JDK_JAVA_OPTIONS", "CLASSPATH", "JAVA_HOME", "RISTRETTO_JDK_HOME"]:
        environment.pop(key, None)
    environment.update(LC_ALL="C", JAVA_HOME=str(args.jdk), RISTRETTO_JDK_HOME=str(args.jdk))
    subprocess.run([str(args.jdk / "bin/javac"), "--release", "8", "-Xlint:-options", "-d", str(classes), str(HERE / "GcBench.java"), str(HERE / "GcTelemetry.java")], env=environment, check=True)
    artifacts.extend(classes.glob("*.class"))
    sources = freeze_sources(output)
    hashes = {str(p.relative_to(output)): digest(p) for p in artifacts}
    jdk_hashes = {name: digest(args.jdk / name) for name in ["lib/modules", "bin/javac", "bin/java"]}
    metadata = {"schema_version": 1, "diagnostic_only": True, "qualification": False, "target": "wasm" if wasm else "native",
                "created_utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
                "host": platform.platform(), "cpuinfo": optional_text("/proc/cpuinfo"),
                "rustc": capture(["rustc", "--version"]),
                "wasmtime": capture([str(args.wasmtime), "--version"]) if wasm else None,
                "options": {k: str(v) if isinstance(v, Path) else v for k, v in vars(args).items()},
                "java": capture([str(args.jdk / "bin/java"), "-version"], environment),
                "required_core": CORE, "required_roots_extra": NODE, "source_sha256": sources,
                "artifacts_sha256": hashes, "jdk_sha256": jdk_hashes}
    (output / "metadata.json").write_text(json.dumps(metadata, indent=2) + "\n")
    rows = []
    for policy in args.policies:
        for work in args.workloads:
            directory = output / f"{policy}-{work}"
            directory.mkdir()
            options = SimpleNamespace(**{**vars(args), "rate": 0, "workload": work})
            command = command_for(("wasm-" if wasm else "ristretto-") + policy, options, classes, directory, binary, binary)
            row = {"policy": policy, "workload": work, "command": command, "diagnostic_only": True}
            budget = Budget(args.cgroup_parent, args.rss_mib * 1024**2, round(args.cpu_quota * 100000)) if args.cgroup_parent else None
            try:
                row.update(execute(command, directory, args.timeout, args.cpus, environment, budget))
                if budget:
                    row["cgroup"] = budget.snapshot()
                    if any(row["cgroup"]["memory_events"].get(k, 0) for k in ["max", "oom", "oom_kill"]):
                        row["status"] = "memory_limit"
            finally:
                if budget:
                    budget.close()
            if row["status"] == "ok":
                try:
                    lines = (directory / "stdout.txt").read_text().splitlines()
                    def one(prefix):
                        found = [json.loads(s[len(prefix):]) for s in lines if s.startswith(prefix)]
                        if len(found) != 1:
                            raise ValueError(f"expected one {prefix}")
                        return found[0]
                    measurement = one("GC_RESULT ")
                    validate(measurement, work, 0, args.seconds, args.seed)
                    if one("RISTRETTO_GC ")["jit_enabled"] is not policy.endswith("-jit"):
                        raise ValueError("execution mode mismatch")
                    profile = one("RISTRETTO_EXECUTION ")
                    row.update(coverage=coverage(profile, work, measurement["completed"]))
                    (directory / "profile.json").write_text(json.dumps(profile, indent=2) + "\n")
                    if any(line.startswith("RISTRETTO_EXECUTION_INTERVAL ") for line in lines):
                        interval = interval_profile(one("RISTRETTO_EXECUTION_INTERVAL "))
                        row["execution_interval"] = {"scope": interval["scope"], "label": interval["label"]}
                        (directory / "interval.json").write_text(json.dumps(interval, indent=2) + "\n")
                except (ValueError, KeyError, TypeError) as error:
                    row.update(status="invalid", error=str(error))
            rows.append(row)
            (output / "results.json").write_text(json.dumps(rows, indent=2) + "\n")
            print(f"{policy} {work}: {row['status']}", flush=True)
    unchanged = (all(digest(ROOT / name) == value for name, value in sources.items())
                 and all(digest(output / name) == value for name, value in hashes.items())
                 and all(digest(args.jdk / name) == value for name, value in jdk_hashes.items()))
    (output / "integrity.json").write_text(json.dumps({"inputs_unchanged": unchanged}) + "\n")
    lines = ["# Execution coverage diagnostic", "", "Lifetime entries include bootstrap, warmup and reporting. These are invocation counts, not CPU shares. Native resumes are reported separately and do not inflate invocation counts. Taken backedges count selected backward control-flow edges, not loop iterations. Instrumented timing is excluded from performance comparisons.",
             "New producers also retain interval.json for the main invocation; this still includes main-class loading, warmup and reporting and is not the measurement window. Missing historical backedge/resume counts are shown as unavailable, not zero.",
             "The full coverage gate remains unmet: measurement-phase coverage, hot-loop execution and the complete dependency manifest remain unqualified.", "",
             "| Policy | Workload | Method | Compiled | Resumes | Backedges | Interpreted | Intrinsic | Rejection |", "|---|---|---|---:|---:|---:|---:|---:|---|"]
    plotted = []
    for row in rows:
        if row["status"] != "ok":
            lines.append(f"\nFailed fork: {row['policy']} / {row['workload']}: {row.get('error', row['status'])}")
            continue
        for method in row["coverage"]["required_methods"]:
            counts = [method.get(k, 0) for k in ["compiled_entries", "interpreter_entries", "intrinsic_entries"]]
            lines.append(f"| {row['policy']} | {row['workload']} | `{method['method']}` | {counts[0]} | {method.get('compiled_resumes', 'unavailable')} | {method.get('compiled_backedges', 'unavailable')} | {counts[1]} | {counts[2]} | {method.get('rejection_reason', method['status'])} |")
            plotted.append((f"{row['policy']} / {row['workload']} / {method['method']}", counts))
    lines.extend(["", f"Frozen inputs unchanged: {unchanged}. Qualification: false."])
    (output / "REPORT.md").write_text("\n".join(lines) + "\n")
    if args.plot and plotted:
        import matplotlib.pyplot as plt
        fig, axis = plt.subplots(figsize=(12, max(4, len(plotted) * .35)), layout="constrained")
        left = [0.0] * len(plotted)
        for i, label in enumerate(["Compiled", "Interpreted", "Intrinsic"]):
            shares = [100 * counts[i] / sum(counts) if sum(counts) else 0 for _, counts in plotted]
            axis.barh(range(len(plotted)), shares, left=left, label=label)
            left = [a + b for a, b in zip(left, shares)]
        axis.set_yticks(range(len(plotted)), [name for name, _ in plotted], fontsize=7, parse_math=False)
        axis.set_xlim(0, 100)
        axis.set_xlabel("Lifetime invocation share (%) — not CPU time or loop coverage")
        valid = sum(row["status"] == "ok" for row in rows)
        axis.set_title(f"Required hot-method execution: {valid}/{len(rows)} valid diagnostic forks")
        axis.legend()
        for suffix in ["svg", "png"]:
            fig.savefig(output / f"coverage.{suffix}", dpi=160)
        plt.close(fig)
    return int(not unchanged or any(r["status"] != "ok" for r in rows))


if __name__ == "__main__":
    raise SystemExit(main())
