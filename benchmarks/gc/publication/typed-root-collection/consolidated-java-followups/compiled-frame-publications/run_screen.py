"""Repeat the prepared-method screen with two separately built VM test executables."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import random
import sys

HERE = Path(__file__).resolve().parent
GC = next(parent for parent in HERE.parents if (parent / "cgroups.py").is_file())
sys.path.insert(0, str(GC))
from cgroups import Budget
from report import validate
from run import execute


def sha256(path):
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for option in ("before", "after", "jdk", "classes", "cgroup-parent", "output"):
        parser.add_argument("--" + option, required=True, type=Path)
    args = parser.parse_args()
    policy = json.loads((HERE / "policy.json").read_text())
    builds = {arm: getattr(args, arm).resolve() for arm in ("before", "after")}
    classes, jdk = args.classes.resolve(), args.jdk.resolve()
    inputs = [*builds.values(), jdk / "bin/java", jdk / "lib/modules",
              classes / "GcBench.class", classes / "GcBench$Node.class",
              classes / "GcBench$Histogram.class", classes / "GcBench$1.class",
              classes / "GcTelemetry.class", HERE / "policy.json"]
    hashes = {str(path): sha256(path) for path in inputs}
    out = args.output.resolve()
    out.mkdir(parents=True, exist_ok=False)
    (out / "inputs.json").write_text(json.dumps(hashes, indent=2) + "\n")

    rng = random.Random(policy["seed"])
    schedule = []
    for pair in range(policy["pairs"]):
        block = [(pair, workload, arm) for workload in policy["workloads"] for arm in builds]
        rng.shuffle(block)
        schedule.extend(block)
    (out / "schedule.json").write_text(json.dumps(schedule, indent=2) + "\n")
    for index, (pair, workload, arm) in enumerate(schedule):
        directory = out / f"{index:02d}-{workload}-{arm}"
        directory.mkdir()
        env = dict(os.environ, JAVA_HOME=str(jdk), GC_PROBE_CLASSES=str(classes),
                   GC_PROBE_WORKLOAD=workload, GC_PROBE_MS=str(policy["seconds"] * 1000),
                   GC_PROBE_WARMUP_MS=str(policy["warmup_seconds"] * 1000))
        command = [str(builds[arm]), "frozen_java_operation_backend_probe", "--nocapture"]
        budget = Budget(args.cgroup_parent, policy["rss_mib"] * 2**20, len(policy["cores"]) * 100000)
        try:
            row = execute(command, directory, 90, set(policy["cores"]), env, budget)
            row["cgroup"] = budget.snapshot()
        finally:
            budget.close()
        stdout = (directory / "stdout.txt").read_text()
        stderr = (directory / "stderr.txt").read_text()
        packets = [line.removeprefix("GC_RESULT ") for line in stdout.splitlines() if line.startswith("GC_RESULT ")]
        if (len(packets) != 1 or row["status"] != "ok" or "1 passed; 0 failed" not in stdout
                or "probe entries=" in stderr or row["cgroup"]["memory_events"]["oom_kill"]):
            raise ValueError(f"invalid timed run in {directory}")
        data = json.loads(packets[0])
        validate(data, workload, 0, policy["seconds"], policy["seed"])
        if data["warmup_ns"] != policy["warmup_seconds"] * 10**9:
            raise ValueError(f"incorrect warmup in {directory}")
        row.update(pair=pair, workload=workload, arm=arm, measurement=data,
                   output_sha256={f.name: sha256(f) for f in directory.iterdir()})
        with (out / "raw.jsonl").open("a") as stream:
            stream.write(json.dumps(row) + "\n")
    if any(sha256(Path(name)) != digest for name, digest in hashes.items()):
        raise ValueError("an input changed during the screen")
    (out / "integrity.json").write_text(json.dumps({"inputs_unchanged": True, "forks": len(schedule)}) + "\n")


if __name__ == "__main__":
    main()
