"""Capture finite explicit-window diagnostics, preserving launch and source custody."""
import argparse
import json
import os
from pathlib import Path
import shutil
import signal
import subprocess
import sys
import tarfile
import uuid
from canonical_service import digest, workload_digest
from canonical_service_artifacts import verify_build
from execution_measurement_artifacts import admission_check, check, command_for


def write(path, value):
    path.write_text(json.dumps(value, indent=2) + "\n")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--build", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--jdk", type=Path, required=True)
    parser.add_argument("--fixture", choices=["probe", "benchmark"], default="probe")
    parser.add_argument("--policy", choices=["candidate", "candidate-jit"], default="candidate")
    parser.add_argument("--workload", choices=["control", "allocation", "roots", "mixed"], default="control")
    parser.add_argument("--duration-ms", type=int, default=100)
    parser.add_argument("--warmup-ms", type=int, default=100)
    parser.add_argument("--method-limit", type=int, default=4096)
    parser.add_argument("--watchdog-seconds", type=int, default=120)
    parser.add_argument("--require-compiled", action="store_true")
    args = parser.parse_args()
    build, root, jdk = args.build.resolve(), args.output.resolve(), args.jdk.resolve()
    receipt = verify_build(build)
    root.mkdir(parents=True, exist_ok=False)
    for name in ("build.json", "source-manifest.json", "sources.tar.gz", "program"):
        shutil.copy2(build / name, root / name)
    admission = admission_check({"schema_version": 1, "run_id": str(uuid.uuid4()), "fixture": args.fixture,
        "policy": args.policy, "workload": args.workload, "duration_ms": args.duration_ms, "warmup_ms": args.warmup_ms,
        "rate": 0, "seed": 7, "heap_mib": 64, "workers": 0, "pause_capacity": 65536,
        "method_limit": args.method_limit, "watchdog_seconds": args.watchdog_seconds,
        "require_compiled": args.require_compiled})
    write(root / "admission.json", admission)
    fixture = "benchmarks/gc/fixtures/execution_window/GcBench.java" if args.fixture == "probe" else "benchmarks/gc/GcBench.java"
    sources = [fixture, "benchmarks/gc/GcTelemetry.java"]
    with tarfile.open(root / "sources.tar.gz", "r:gz") as archive:
        for name in sources + ["benchmarks/gc/canonical_service_launch.py"]:
            path = root / "fixture-source" / name if name in sources else root / "launcher.py"
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(archive.extractfile(name).read())
    (root / "classes").mkdir()
    command = [str(jdk / "bin/javac"), "--release", "8", "-Xlint:-options", "-d", str(root / "classes"),
               *(str(root / "fixture-source" / name) for name in sources)]
    jdk_files = {name: digest(jdk / name) for name in ("bin/java", "bin/javac", "lib/modules")}
    with (root / "javac.stdout").open("w") as out, (root / "javac.stderr").open("w") as err:
        compilation = subprocess.run(command, stdout=out, stderr=err, timeout=60, check=False)
    java = {"jdk_home": str(jdk), "jdk_files": jdk_files, "command": command, "exit_code": compilation.returncode,
            "source_files": {name: digest(root / "fixture-source" / name) for name in sources},
            "class_files": {p.relative_to(root / "classes").as_posix(): digest(p) for p in sorted((root / "classes").rglob("*.class"))}}
    write(root / "java.json", java)
    if compilation.returncode:
        raise ValueError("Java compilation failed; evidence preserved")
    command = command_for(root, admission, str(jdk))
    plan = {"run_id": admission["run_id"], "mode": "native", "command": command,
            "watchdog_seconds": admission["watchdog_seconds"], "admission_sha256": digest(root / "admission.json")}
    write(root / "launch-plan.json", plan)
    environment = {key: value for key, value in os.environ.items()
                   if key not in {"JAVA_TOOL_OPTIONS", "JDK_JAVA_OPTIONS", "_JAVA_OPTIONS", "CLASSPATH", "JAVA_HOME", "RISTRETTO_JDK_HOME"}}
    environment.update(LC_ALL="C", JAVA_HOME=str(jdk), RISTRETTO_JDK_HOME=str(jdk),
                       RISTRETTO_QUALIFICATION_RUN_ID=admission["run_id"],
                       RISTRETTO_QUALIFICATION_POLICY_SHA256=plan["admission_sha256"],
                       RISTRETTO_SOURCE_ARCHIVE_SHA256=receipt["source_archive_sha256"],
                       RISTRETTO_PAUSE_CAPACITY=str(admission["pause_capacity"]),
                       RISTRETTO_EXECUTION_METHOD_LIMIT=str(admission["method_limit"]))
    timed_out = False
    with (root / "stdout.txt").open("w") as out, (root / "stderr.txt").open("w") as err:
        process = subprocess.Popen([sys.executable, str(root / "launcher.py"), str(root)],
                                   stdout=out, stderr=err, env=environment, start_new_session=True)
        print(json.dumps({"directory": str(root), "pid": process.pid, "run_id": admission["run_id"]}), flush=True)
        try:
            exit_code = process.wait(timeout=admission["watchdog_seconds"])
        except subprocess.TimeoutExpired:
            timed_out = True
            os.killpg(process.pid, signal.SIGKILL)
            exit_code = process.wait()
    run = {"schema_version": 1, **plan, "host_process_id": process.pid, "exit_code": exit_code,
           "launcher_sha256": digest(root / "launcher.py"), "workload_sha256": workload_digest(root / "classes"),
           "host_executable_sha256": receipt["binary_sha256"], "timed_out": timed_out,
           "launch_plan_sha256": digest(root / "launch-plan.json"), "java_receipt_sha256": digest(root / "java.json"),
           "stdout_sha256": digest(root / "stdout.txt"), "stderr_sha256": digest(root / "stderr.txt")}
    write(root / "run.json", run)
    try:
        result = check(root)
    except (ValueError, KeyError, OSError) as error:
        write(root / "check.json", {"passed": False, "error": str(error), "qualification": False})
        raise
    write(root / "check.json", {"passed": True, **result})
    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
