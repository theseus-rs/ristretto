"""Artifact checks shared by the finite launcher and independent offline checker."""
import json
from pathlib import Path
import tarfile
from canonical_service import digest, fields, integer, observation, sha, unique_object, validate_trace, workload_digest


def read_json(path):
    return json.loads(Path(path).read_text(), object_pairs_hook=unique_object)


def verify_build(directory):
    root = Path(directory)
    receipt = read_json(root / "build.json")
    fields(receipt, "schema_version source_archive_sha256 source_manifest_sha256 binary_sha256 "
           "binary_name target features command toolchain source_unchanged qualification", "build receipt")
    if (receipt["schema_version"] != 1 or type(receipt["schema_version"]) is not int
            or receipt["source_unchanged"] is not True or receipt["qualification"] is not False
            or receipt["binary_name"] not in {"program", "program.wasm"}):
        raise ValueError("unsupported/unverified build receipt")
    for key, filename in [("source_archive_sha256", "sources.tar.gz"),
                          ("source_manifest_sha256", "source-manifest.json"),
                          ("binary_sha256", receipt["binary_name"])]:
        sha(receipt[key], key)
        if digest(root / filename) != receipt[key]:
            raise ValueError(f"build artifact differs: {filename}")
    manifest = read_json(root / "source-manifest.json")
    if not isinstance(manifest, dict) or not manifest:
        raise ValueError("source manifest missing")
    import hashlib
    seen = set()
    with tarfile.open(root / "sources.tar.gz", "r:gz") as archive:
        for member in archive:
            name = member.name
            if (not member.isfile() or name in seen or name not in manifest
                    or Path(name).is_absolute() or ".." in Path(name).parts):
                raise ValueError("invalid/duplicate/unmanifested source archive member")
            seen.add(name)
            sha(manifest[name], "source member digest")
            with archive.extractfile(member) as stream:
                if hashlib.file_digest(stream, "sha256").hexdigest() != manifest[name]:
                    raise ValueError(f"source member changed: {name}")
    if seen != set(manifest):
        raise ValueError("source archive is incomplete")
    return receipt


def validate_admission(admission):
    fields(admission, "schema_version tier class method argument configuration limits recorder_capacity "
           "expected_result require_compiled_backedges window_kind", "admission")
    if type(admission["schema_version"]) is not int or admission["schema_version"] != 1:
        raise ValueError("admission schema")
    if admission["tier"] not in {"interpreted", "compiled"} or admission["window_kind"] != "entry_body_v1":
        raise ValueError("unsupported admitted tier/window")
    for key in ["class", "method"]:
        if not isinstance(admission[key], str) or not admission[key]:
            raise ValueError("entry identity missing")
    for key in ["argument", "expected_result"]:
        if type(admission[key]) is not int or not -(2**31) <= admission[key] < 2**31:
            raise ValueError("Java int outside admitted range")
    integer(admission["recorder_capacity"], "recorder capacity", 1)
    if type(admission["require_compiled_backedges"]) is not bool:
        raise ValueError("backedge requirement must be boolean")
    fields(admission["configuration"], "objects roots mutators helpers edges_per_object", "configuration")
    for key, value in admission["configuration"].items():
        integer(value, key, 1)
    if admission["configuration"]["mutators"] != 1 or admission["configuration"]["helpers"] != 1:
        raise ValueError("telemetry scope requires one caller/helper")
    fields(admission["limits"], "instructions request_polls cleanup_polls frames native_slices array_bytes", "limits")
    for key, value in admission["limits"].items():
        integer(value, key, 1)
    return admission


def expected_identity(directory):
    root = Path(directory)
    build = verify_build(root)
    admission = validate_admission(read_json(root / "admission.json"))
    run = read_json(root / "run.json")
    fields(run, "schema_version run_id mode host_process_id exit_code command launcher_sha256 "
           "admission_sha256 workload_sha256 host_executable_sha256", "run receipt")
    launch = read_json(root / "launch.json")
    fields(launch, "schema_version run_id mode host_process_id command host_executable_sha256 "
           "binary_sha256 launcher_sha256", "exec receipt")
    if (type(run["schema_version"]) is not int or run["schema_version"] != 1
            or type(launch["schema_version"]) is not int or launch["schema_version"] != 1
            or type(run["exit_code"]) is not int or run["exit_code"] != 0):
        raise ValueError("unsupported/failed launch")
    integer(run["host_process_id"], "actual Popen PID", 1)
    for key in ["run_id", "mode", "host_process_id", "command", "host_executable_sha256", "launcher_sha256"]:
        if run[key] != launch[key] or type(run[key]) != type(launch[key]):
            raise ValueError(f"Popen/exec receipts differ: {key}")
    if (run["admission_sha256"] != digest(root / "admission.json")
            or run["workload_sha256"] != workload_digest(root / "classes")
            or launch["binary_sha256"] != build["binary_sha256"]):
        raise ValueError("launched admission/classes/executable changed")
    command = run["command"]
    if not isinstance(command, list) or not all(isinstance(arg, str) for arg in command):
        raise ValueError("invalid exec arguments")
    if run["mode"] == "native":
        expected = [str((root / "program").resolve()), str(root.resolve())]
        if (command != expected or run["host_executable_sha256"] != build["binary_sha256"]
                or build["binary_name"] != "program"):
            raise ValueError("native command differs from actual executable/inputs")
        guest_pid = run["host_process_id"]
    elif run["mode"] == "wasi":
        expected_tail = ["run", "--dir", f"{root.resolve()}::/capture",
                         str((root / "program.wasm").resolve()), "/capture"]
        if (command[1:] != expected_tail or build["binary_name"] != "program.wasm"
                or build["target"] != "wasm32-wasip2"):
            raise ValueError("WASI command differs from module/guest inputs")
        guest_pid = None
    else:
        raise ValueError("unsupported launch mode")
    # The host executable is preserved independently so later host replacement
    # cannot silently change the command's identity at offline validation time.
    if digest(root / "host-executable") != run["host_executable_sha256"]:
        raise ValueError("preserved host executable differs")
    source_manifest = read_json(root / "source-manifest.json")
    if (digest(root / "launcher.py") != run["launcher_sha256"]
            or run["launcher_sha256"] != source_manifest.get("benchmarks/gc/canonical_service_launch.py")):
        raise ValueError("launcher source differs from frozen build source")
    return {"run_id": run["run_id"], "mode": run["mode"], "tier": admission["tier"],
            "host_process_id": run["host_process_id"], "guest_process_id": guest_pid,
            "host_executable_sha256": run["host_executable_sha256"],
            "binary_sha256": build["binary_sha256"],
            "source_archive_sha256": build["source_archive_sha256"],
            "source_manifest_sha256": build["source_manifest_sha256"],
            "workload_sha256": run["workload_sha256"], "admission_sha256": run["admission_sha256"],
            "build_receipt_sha256": digest(root / "build.json"),
            "launch_receipt_sha256": digest(root / "launch.json")}, admission


def check(directory):
    identity, admission = expected_identity(directory)
    return validate_trace(observation(Path(directory) / "stdout.txt"), identity, admission)


if __name__ == "__main__":
    import argparse
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("directory", type=Path)
    args = parser.parse_args()
    print(json.dumps(check(args.directory), indent=2))
