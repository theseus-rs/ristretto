"""Finite diagnostic canonical capture; not a throughput or endurance runner."""
import argparse
import json
from pathlib import Path
import shutil
import subprocess
import sys
import uuid
from canonical_service import digest, workload_digest
from canonical_service_artifacts import check, read_json, validate_admission, verify_build


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--build", type=Path, required=True)
    parser.add_argument("--classes", type=Path, required=True)
    parser.add_argument("--admission", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--mode", choices=["native", "wasi"], required=True)
    parser.add_argument("--wasmtime", type=Path)
    args = parser.parse_args()
    build = verify_build(args.build)
    admission = validate_admission(read_json(args.admission))
    if (args.mode == "wasi") != (build["binary_name"] == "program.wasm"):
        raise ValueError("host mode differs from built artifact")
    if admission["tier"] == "compiled" and "compiled" not in build["features"]:
        raise ValueError("artifact was not built with the compiled capture tier")
    root = args.output.resolve()
    root.mkdir(parents=True, exist_ok=False)
    for name in ["build.json", "source-manifest.json", "sources.tar.gz", build["binary_name"]]:
        shutil.copy2(args.build / name, root / name)
    shutil.copy2(args.admission, root / "admission.json")
    shutil.copytree(args.classes, root / "classes", symlinks=True)
    workload = workload_digest(root / "classes")  # Reject symlink classfiles.
    launcher = Path(__file__).with_name("canonical_service_launch.py")
    shutil.copy2(launcher, root / "launcher.py")
    if args.mode == "native":
        command = [str(root / "program"), str(root)]
    else:
        if args.wasmtime is None:
            raise ValueError("WASI requires an explicit Wasmtime executable")
        if admission["tier"] != "interpreted":
            raise ValueError("compiled canonical WASI is unsupported")
        command = [str(args.wasmtime.resolve()), "run", "--dir", f"{root}::/capture",
                   str(root / "program.wasm"), "/capture"]
    host_hash = digest(command[0])
    shutil.copy2(command[0], root / "host-executable")
    run_id = str(uuid.uuid4())
    (root / "launch-plan.json").write_text(json.dumps({"run_id": run_id, "mode": args.mode, "command": command}, indent=2) + "\n")
    with (root / "stdout.txt").open("wb") as stdout, (root / "stderr.txt").open("wb") as stderr:
        process = subprocess.Popen([sys.executable, str(root / "launcher.py"), str(root)], stdout=stdout, stderr=stderr)
        try:
            code = process.wait(timeout=120)
        except subprocess.TimeoutExpired:
            process.kill()
            code = process.wait()
    run = {"schema_version": 1, "run_id": run_id, "mode": args.mode, "host_process_id": process.pid,
           "exit_code": code, "command": command, "launcher_sha256": digest(root / "launcher.py"),
           "admission_sha256": digest(root / "admission.json"), "workload_sha256": workload,
           "host_executable_sha256": host_hash}
    (root / "run.json").write_text(json.dumps(run, indent=2) + "\n")
    result = check(root)
    result["validator_sha256"] = digest(Path(__file__).with_name("canonical_service.py"))
    (root / "checked.json").write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
