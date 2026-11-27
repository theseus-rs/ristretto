"""Run one bounded fork. Defaults to an untimed functional check."""
import argparse
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import time
import uuid
from canonical_service import digest, workload_digest
from canonical_service_artifacts import read_json, verify_build
from ring_artifacts import (check_canonical, check_peer, peer_command, validate_resources, verify_peer_build)
from ring_comparison import validate_admission, validate_plan
from ring_launch import cgroup_mount
from process_resources import wait_resources


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--build", type=Path, required=True)
    p.add_argument("--classes", type=Path)
    p.add_argument("--admission", type=Path, required=True)
    p.add_argument("--output", type=Path, required=True)
    p.add_argument("--mode", choices=["native", "wasi", "peer"], required=True)
    p.add_argument("--sample-mode", choices=["check", "warmup-check", "measure", "allocation", "scheduled-check", "scheduled-measure"], default="check")
    p.add_argument("--duration-seconds", type=int, default=10)
    p.add_argument("--arrival-rate", type=int, default=32)
    p.add_argument("--max-completions", type=int, default=1000000000)
    p.add_argument("--warmup", type=int, default=0)
    p.add_argument("--samples", type=int, default=1)
    p.add_argument("--wasmtime", type=Path)
    p.add_argument("--collector", choices=["G1", "Z"])
    p.add_argument("--heap-bytes", type=int, default=64 * 1024 * 1024)
    p.add_argument("--resources", type=Path)
    p.add_argument("--cgroup-parent", type=Path)
    p.add_argument("--timeout", type=int, default=120)
    args = p.parse_args()
    if args.timeout <= 0:
        p.error("--timeout must be positive")
    admission = validate_admission(read_json(args.admission))
    if args.mode == "peer" and admission["schema_version"] in (2, 3) and args.heap_bytes != admission["max_payload_bytes"]:
        raise ValueError("peer/shared payload heap limits differ")
    scheduled = args.sample_mode.startswith("scheduled-")
    if scheduled != (admission["schema_version"] == 3):
        raise ValueError("scheduled admission and plan scopes must match")
    plan = validate_plan({"schema_version": 2, "run_id": str(uuid.uuid4()), "mode": args.sample_mode,
                          "warmup_invocations": args.warmup, "duration_seconds": args.duration_seconds,
                          "arrival_rate": args.arrival_rate, "max_completions": args.max_completions}) if scheduled else validate_plan({"schema_version": 1, "run_id": str(uuid.uuid4()), "mode": args.sample_mode,
                          "warmup_invocations": args.warmup, "samples": args.samples}, peer=args.mode == "peer")
    request = validate_resources(read_json(args.resources)) if args.resources else None
    if args.sample_mode not in {"check", "scheduled-check"} and (request is None or args.cgroup_parent is None):
        raise ValueError("measurement/allocation requires explicit equal CPU/memory limits in a delegated cgroup")
    if (request is None) != (args.cgroup_parent is None):
        raise ValueError("resource envelope and delegated cgroup must be supplied together")
    root = args.output.resolve()
    root.mkdir(parents=True, exist_ok=False)
    shutil.copy2(args.admission, root / "admission.json")
    (root / "sample-plan.json").write_text(json.dumps(plan, indent=2) + "\n")
    if args.mode == "peer":
        build = verify_peer_build(args.build)
        if args.collector is None or args.heap_bytes <= 0:
            raise ValueError("peer requires collector and a positive heap bound")
        for name in ("peer-build.json", "sources.tar.gz", "source-manifest.json"):
            shutil.copy2(args.build / name, root / name)
        for name in ("classes", "shared-classes", "jdk-artifacts"):
            shutil.copytree(args.build / name, root / name)
        for name, expected in build["jdk_sha256"].items():
            if digest(Path(build["jdk"]) / name) != expected:
                raise ValueError("JDK installation changed since peer build")
        configuration = {"collector": args.collector, "heap_bytes": args.heap_bytes,
                         "active_processors": len(request["cpu_affinity"]) if request else 1}
        (root / "peer-configuration.json").write_text(json.dumps(configuration, indent=2) + "\n")
        command = peer_command(root, Path(build["jdk"]), admission, plan, configuration)
        binary_name = "classes/CanonicalRingPeer.class"
        workload = build["shared_workload_sha256"]
    else:
        build = verify_build(args.build)
        if args.sample_mode in {"measure", "scheduled-measure"} and "--release" not in build["command"]:
            raise ValueError("measurement requires a release artifact")
        if (args.mode == "wasi") != (build["binary_name"] == "program.wasm"):
            raise ValueError("artifact differs from requested host mode")
        if admission["tier"] == "compiled" and ("compiled" not in build["features"] or args.mode != "native"):
            raise ValueError("compiled artifact/host required")
        if args.classes is None:
            raise ValueError("canonical runtime requires exact workload classfiles")
        for name in ("build.json", "sources.tar.gz", "source-manifest.json", build["binary_name"]):
            shutil.copy2(args.build / name, root / name)
        shutil.copytree(args.classes, root / "classes", symlinks=True)
        workload = workload_digest(root / "classes")
        binary_name = build["binary_name"]
        if args.mode == "native":
            command = [str(root / "program"), str(root)]
        else:
            if args.wasmtime is None:
                raise ValueError("WASI requires an explicit host executable")
            command = [str(args.wasmtime.resolve()), "run", "--dir", f"{root}::/capture", str(root / "program.wasm"), "/capture"]
    group = None
    if request is not None:
        cgroup_mount(args.cgroup_parent)
        group = args.cgroup_parent.resolve() / ("ring-" + plan["run_id"])
        group.mkdir()
        (group / "memory.max").write_text(str(request["memory_max_bytes"]))
        (group / "cpu.max").write_text(f'{request["cpu_quota_us"]} {request["cpu_period_us"]}')
        (group / "pids.max").write_text(str(request["pids_max"]))
    launcher = Path(__file__).with_name("ring_launch.py")
    shutil.copy2(launcher, root / "launcher.py")
    source_manifest = read_json(root / "source-manifest.json")
    for original, preserved in ((Path(__file__), "controller.py"),
                                (Path(__file__).with_name("process_resources.py"), "resource-observer.py")):
        if digest(original) != source_manifest.get("benchmarks/gc/" + original.name):
            raise ValueError("resource controller differs from frozen build source")
        shutil.copy2(original, root / preserved)
    host_hash = digest(command[0])
    shutil.copy2(command[0], root / "host-executable")
    (root / "launch-plan.json").write_text(json.dumps({"run_id": plan["run_id"], "mode": args.mode,
        "sample_mode": plan["mode"], "command": command, "resources": request,
        "cgroup": str(group) if group else None, "binary_name": binary_name,
        "watchdog_seconds": args.timeout}, indent=2) + "\n")
    env = {key: value for key, value in os.environ.items() if key not in
           {"JAVA_TOOL_OPTIONS", "JDK_JAVA_OPTIONS", "_JAVA_OPTIONS", "LD_PRELOAD", "LD_LIBRARY_PATH"}}
    with (root / "stdout.txt").open("wb") as stdout, (root / "stderr.txt").open("wb") as stderr:
        watchdog_start = time.monotonic() if args.sample_mode == "warmup-check" else None
        process = subprocess.Popen([sys.executable, str(root / "launcher.py"), str(root)], stdout=stdout, stderr=stderr, env=env)
        usage = wait_resources(process, args.timeout, run_id=plan["run_id"])
        code = process.returncode
        (root / "process-resources.json").write_text(json.dumps(usage, indent=2) + "\n")
        if watchdog_start is not None:
            (root / "watchdog.json").write_text(json.dumps({"purpose": "timeout_sizing_only",
                "whole_child_process_seconds": time.monotonic() - watchdog_start,
                "includes_startup_hashing_compilation_and_teardown": True,
                "invocation_timings_available": False, "qualification": False}, indent=2) + "\n")
    run = {"schema_version": 2, "run_id": plan["run_id"], "mode": args.mode, "host_process_id": process.pid,
           "exit_code": code, "command": command, "launcher_sha256": digest(root / "launcher.py"),
           "admission_sha256": digest(root / "admission.json"), "workload_sha256": workload,
           "host_executable_sha256": host_hash,
           "controller_sha256": digest(root / "controller.py"),
           "resource_observer_sha256": digest(root / "resource-observer.py"),
           "process_resources_sha256": digest(root / "process-resources.json"),
           "launch_plan_sha256": digest(root / "launch-plan.json")}
    (root / "run.json").write_text(json.dumps(run, indent=2) + "\n")
    if group is not None:
        if args.sample_mode != "warmup-check":
            observed = {name: (group / name).read_text() for name in ("memory.peak", "memory.events", "cpu.stat", "pids.peak") if (group / name).exists()}
            (root / "resource-after.json").write_text(json.dumps(observed, indent=2) + "\n")
        group.rmdir()
    result = check_peer(root) if args.mode == "peer" else check_canonical(root)
    (root / "checked.json").write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
