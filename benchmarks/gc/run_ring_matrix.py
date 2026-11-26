"""Execute a previously frozen finite matrix; quiet-host scheduling is external."""
import argparse
import json
from pathlib import Path
import subprocess
import sys
from canonical_service import digest
from canonical_service_artifacts import read_json
from ring_comparison import validate_matrix, validate_matrix_admission


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("directory", type=Path)
    p.add_argument("--wasmtime", type=Path, required=True)
    p.add_argument("--cgroup-parent", type=Path)
    p.add_argument("--timeout", type=int)
    args = p.parse_args()
    root = args.directory.resolve()
    plan = validate_matrix(read_json(root / "plan.json"))
    timeout = plan.get("watchdog_seconds", args.timeout if args.timeout is not None else 120)
    if args.timeout is not None and args.timeout != timeout:
        raise ValueError("requested watchdog differs from frozen matrix")
    if not plan["check_only"] and args.cgroup_parent is None:
        raise ValueError("timing/allocation matrix requires delegated resource controls")
    for family, value in plan["builds"].items():
        name = "peer-build.json" if family == "peer" else "build.json"
        if digest(Path(value["directory"]) / name) != value["receipt_sha256"]:
            raise ValueError("frozen build receipt changed")
    for name, value in plan["admission_sha256"].items():
        if digest(root / "admissions" / name) != value:
            raise ValueError("frozen admission changed")
        validate_matrix_admission(plan, read_json(root / "admissions" / name))
    if plan["resources_sha256"] is not None and digest(root / "resources.json") != plan["resources_sha256"]:
        raise ValueError("frozen resource envelope changed")
    runs = root / "runs"
    runs.mkdir(exist_ok=False)
    results = []
    for case in plan["cases"]:
        runtime = case["runtime"]
        family = "peer" if runtime in {"g1", "zgc"} else "wasi" if runtime.startswith("wasi") else "native"
        tier = "compiled" if runtime == "native-compiled" else "interpreted"
        command = [sys.executable, str(Path(__file__).with_name("run_ring_sample.py")), "--build", plan["builds"][family]["directory"],
                   "--admission", str(root / "admissions" / f'{case["workload"]}-{tier}.json'), "--output", str(runs / case["id"]),
                   "--mode", family, "--sample-mode", case["mode"], "--warmup", str(plan["warmup_invocations"]),
                   "--samples", str(plan["allocation_samples"] if case["mode"] == "allocation" else plan["samples"]), "--timeout", str(timeout)]
        if family == "peer":
            command += ["--collector", "G1" if runtime == "g1" else "Z", "--heap-bytes", str(plan["heap_bytes"])]
        else:
            command += ["--classes", str(root / "classes")]
            if family == "wasi": command += ["--wasmtime", str(args.wasmtime.resolve())]
        if plan["resources_sha256"] is not None:
            if args.cgroup_parent is None: raise ValueError("configured envelope requires delegated cgroup")
            command += ["--resources", str(root / "resources.json"), "--cgroup-parent", str(args.cgroup_parent.resolve())]
        with (runs / (case["id"] + ".log")).open("w") as log:
            code = subprocess.run(command, stdout=log, stderr=subprocess.STDOUT).returncode
        if code == 0:
            usage = read_json(runs / case["id"] / "checked.json").get("process_resources")
            if usage is not None and usage["timeout_seconds"] != timeout:
                code = 1
        results.append({"id": case["id"], "command": command, "exit_code": code,
                        "checked_sha256": digest(runs / case["id"] / "checked.json") if code == 0 else None})
        (root / "results.json").write_text(json.dumps({"plan_sha256": digest(root / "plan.json"), "results": results,
                                                     "complete": len(results) == len(plan["cases"]) and code == 0, "qualification": False}, indent=2) + "\n")
        print(case["id"], code, flush=True)
        if code != 0: raise SystemExit(code)


if __name__ == "__main__":
    main()
