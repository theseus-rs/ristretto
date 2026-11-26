"""Independently bind explicit execution-window evidence to native launch artifacts."""
import argparse
import json
from pathlib import Path
from canonical_service import digest, fields, integer, workload_digest
from canonical_service_artifacts import read_json, verify_build
from execution_measurement import packet, validate
from pauses import validate_trace
from report import validate as validate_workload


def admission_check(value):
    fields(value, "schema_version run_id fixture policy workload duration_ms warmup_ms rate seed "
           "heap_mib workers pause_capacity method_limit watchdog_seconds require_compiled", "admission")
    if type(value["schema_version"]) is not int or value["schema_version"] != 1:
        raise ValueError("unsupported execution admission schema")
    if not isinstance(value["run_id"], str) or not value["run_id"]:
        raise ValueError("missing unique run identity")
    if value["fixture"] not in {"probe", "benchmark"} or value["policy"] not in {"candidate", "candidate-jit"}:
        raise ValueError("unsupported fixture or policy")
    if value["workload"] not in {"control", "allocation", "roots", "mixed"}:
        raise ValueError("unsupported workload")
    for key in ("duration_ms", "heap_mib", "pause_capacity", "method_limit", "watchdog_seconds"):
        integer(value[key], key, 1)
    for key in ("warmup_ms", "rate", "workers", "seed"):
        integer(value[key], key)
    if value["workers"] > 1 or value["seed"] >= 2**31 or value["method_limit"] > 65536:
        raise ValueError("unsupported execution admission bounds")
    if type(value["require_compiled"]) is not bool or (value["require_compiled"] and not value["policy"].endswith("-jit")):
        raise ValueError("compiled requirement conflicts with requested runtime")
    return value


def command_for(root, admission, jdk):
    return [str(root / "program"), jdk, str(root / "classes"), str(admission["heap_mib"]),
            str(admission["workers"]), admission["policy"], admission["workload"],
            *(str(admission[key]) for key in ("duration_ms", "warmup_ms", "rate", "seed"))]


def expected_identity(root):
    root = Path(root).resolve()
    build = verify_build(root)
    if (build["target"] != "native" or build["binary_name"] != "program"
            or build["features"] != ["experimental-execution-measurement"]):
        raise ValueError("build does not provide the native explicit execution-window producer")
    admission = admission_check(read_json(root / "admission.json"))
    run, launch = read_json(root / "run.json"), read_json(root / "launch.json")
    fields(run, "schema_version run_id mode host_process_id exit_code command launcher_sha256 "
           "admission_sha256 workload_sha256 host_executable_sha256 watchdog_seconds timed_out "
           "launch_plan_sha256 java_receipt_sha256 stdout_sha256 stderr_sha256", "native run")
    fields(launch, "schema_version run_id mode host_process_id command host_executable_sha256 "
           "binary_sha256 launcher_sha256", "native exec")
    if (type(run["schema_version"]) is not int or run["schema_version"] != 1
            or type(launch["schema_version"]) is not int or launch["schema_version"] != 1
            or type(run["exit_code"]) is not int or run["exit_code"] != 0
            or run["timed_out"] is not False or run["mode"] != "native"):
        raise ValueError("failed, timed out or unsupported native launch")
    integer(run["host_process_id"], "actual Popen PID", 1)
    for key in ("run_id", "mode", "host_process_id", "command", "host_executable_sha256", "launcher_sha256"):
        if run[key] != launch[key] or type(run[key]) != type(launch[key]):
            raise ValueError(f"actual Popen/exec identity mismatch: {key}")
    for key, filename in (("admission_sha256", "admission.json"), ("launch_plan_sha256", "launch-plan.json"),
                          ("java_receipt_sha256", "java.json"), ("stdout_sha256", "stdout.txt"), ("stderr_sha256", "stderr.txt")):
        if run[key] != digest(root / filename):
            raise ValueError(f"run artifact changed: {filename}")
    if (run["run_id"] != admission["run_id"] or run["watchdog_seconds"] != admission["watchdog_seconds"]
            or type(run["watchdog_seconds"]) is not int
            or run["workload_sha256"] != workload_digest(root / "classes")
            or run["host_executable_sha256"] != build["binary_sha256"]
            or launch["binary_sha256"] != build["binary_sha256"]):
        raise ValueError("native run/admission/classes/executable identity mismatch")
    manifest = read_json(root / "source-manifest.json")
    if (digest(root / "launcher.py") != run["launcher_sha256"]
            or run["launcher_sha256"] != manifest.get("benchmarks/gc/canonical_service_launch.py")):
        raise ValueError("exec launcher differs from archived source")
    java = read_json(root / "java.json")
    fields(java, "jdk_home jdk_files source_files class_files command exit_code", "Java compilation")
    if java["exit_code"] != 0 or type(java["exit_code"]) is not int:
        raise ValueError("Java fixture did not compile")
    if set(java["jdk_files"]) != {"bin/java", "bin/javac", "lib/modules"}:
        raise ValueError("incomplete JDK identity")
    for name, hashed in java["jdk_files"].items():
        if digest(Path(java["jdk_home"]) / name) != hashed:
            raise ValueError(f"JDK bytes changed/unavailable: {name}")
    fixture = "benchmarks/gc/fixtures/execution_window/GcBench.java" if admission["fixture"] == "probe" else "benchmarks/gc/GcBench.java"
    expected_sources = [fixture, "benchmarks/gc/GcTelemetry.java"]
    if set(java["source_files"]) != set(expected_sources):
        raise ValueError("wrong Java fixture source set")
    for name, hashed in java["source_files"].items():
        if digest(root / "fixture-source" / name) != hashed or manifest.get(name) != hashed:
            raise ValueError("Java source differs from exact archived source")
    classes = {p.relative_to(root / "classes").as_posix(): digest(p) for p in sorted((root / "classes").rglob("*.class"))}
    if classes != java["class_files"] or not classes:
        raise ValueError("Java classfiles differ from javac receipt")
    javac = [str(Path(java["jdk_home"]) / "bin/javac"), "--release", "8", "-Xlint:-options", "-d", str(root / "classes"),
             *(str(root / "fixture-source" / name) for name in expected_sources)]
    if java["command"] != javac or run["command"] != command_for(root, admission, java["jdk_home"]):
        raise ValueError("Java compile or native exec command differs from admitted inputs")
    plan = read_json(root / "launch-plan.json")
    fields(plan, "run_id mode command watchdog_seconds admission_sha256", "launch plan")
    if plan != {key: run[key] for key in plan}:
        raise ValueError("launch plan differs from executed run/watchdog")
    return {"run_id": admission["run_id"], "policy_sha256": run["admission_sha256"],
            "source_archive_sha256": build["source_archive_sha256"], "binary_sha256": build["binary_sha256"],
            "workload_sha256": run["workload_sha256"], "process_id": run["host_process_id"],
            "recorder_capacity": admission["pause_capacity"], "runtime": "ristretto-" + admission["policy"],
            **{key: admission[key] for key in ("workload", "duration_ms", "warmup_ms", "rate", "seed")}}, admission


def check(directory):
    root = Path(directory).resolve()
    identity, admission = expected_identity(root)
    stdout = (root / "stdout.txt").read_text()
    if admission["fixture"] == "probe":
        result = packet(stdout, "GC_WINDOW_PROBE ")
        fields(result, "schema_version warmup completed checksum after elapsed_ns", "probe result")
        expected = {"schema_version": 1, "warmup": 135, "completed": 7, "checksum": 70, "after": 176}
        if any(type(result[key]) is not int or result[key] != value for key, value in expected.items()):
            raise ValueError("Java probe checksum/count ground truth failed")
        fixture = "probe"
    else:
        result = packet(stdout, "GC_RESULT ")
        validate_workload(result, admission["workload"], admission["rate"], admission["duration_ms"] / 1000, admission["seed"])
        fixture = admission["workload"]
    pause = packet(stdout, "RISTRETTO_PAUSES ")
    pause_result = validate_trace(pause, identity, result["elapsed_ns"])
    if (pause["measurement_boundaries"]["state"] != "closed" or pause["measurement_boundaries"]["errors"]
            or pause["instrumented_capture_complete"] is not True):
        raise ValueError("lost/open/overflowed pause evidence or missing measured interval")
    execution = packet(stdout, "RISTRETTO_EXECUTION_MEASUREMENT ")
    fields(execution, "schema_version identity capture", "execution envelope")
    if type(execution["schema_version"]) is not int or execution["schema_version"] != 1 or execution["identity"] != identity:
        raise ValueError("execution envelope does not bind the actual launch")
    capture = execution["capture"]
    checked = validate(capture, result["elapsed_ns"], fixture, result["completed"], admission["require_compiled"])
    if capture["limits"]["methods"] != admission["method_limit"]:
        raise ValueError("runtime recorder cap differs from admission")
    if ({"start_ns": capture["begin"]["timestamp_ns"], "end_ns": capture["end"]["timestamp_ns"]}
            != pause["measurement_window"]):
        raise ValueError("execution and pause producers used different measurement boundaries")
    return {"schema_version": 1, "identity": identity, "fixture": fixture, "execution": checked,
            "pause": pause_result, "qualification": False, "diagnostic_only": True}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("directory", type=Path)
    args = parser.parse_args()
    print(json.dumps(check(args.directory), indent=2))


if __name__ == "__main__":
    main()
