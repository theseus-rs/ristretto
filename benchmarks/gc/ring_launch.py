"""Enter the declared Linux resource envelope, record actual PID, then exec."""
import json
import os
from pathlib import Path
import sys
import hashlib


def digest(path):
    with Path(path).open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def cgroup_mount(group):
    """Reject ordinary directories masquerading as resource-control files."""
    group = Path(group).resolve()
    found = []
    for line in Path("/proc/self/mountinfo").read_text().splitlines():
        before, after = line.split(" - ", 1)
        if after.split()[0] != "cgroup2":
            continue
        parts = before.split()
        def unescape(value):
            for code, char in (("\\040", " "), ("\\011", "\t"), ("\\012", "\n"), ("\\134", "\\")):
                value = value.replace(code, char)
            return value
        point, mount_root = Path(unescape(parts[4])), unescape(parts[3])
        if group.is_relative_to(point):
            found.append((point, mount_root))
    if not found:
        raise ValueError("resource directory is not on a cgroup-v2 mount")
    return max(found, key=lambda item: len(str(item[0])))


def launch(root):
    plan = json.loads((root / "launch-plan.json").read_text())
    requested = plan["resources"]
    group = plan["cgroup"]
    if requested is not None:
        os.sched_setaffinity(0, requested["cpu_affinity"])
    if group is not None:
        point, mount_root = cgroup_mount(group)
        Path(group, "cgroup.procs").write_text(str(os.getpid()))
    actual = {"enforced": requested is not None and group is not None,
              "cpu_affinity": sorted(os.sched_getaffinity(0)), "cgroup": group,
              "cpu_max": None, "memory_max": None, "pids_max": None,
              "cgroup_mount": None, "cgroup_root": None, "cgroup_membership": None}
    if group is not None:
        memberships = [line[3:] for line in Path("/proc/self/cgroup").read_text().splitlines() if line.startswith("0::")]
        expected_membership = os.path.normpath(str(Path(mount_root) / Path(group).relative_to(point)))
        if memberships != [expected_membership]:
            raise ValueError("process did not enter its requested cgroup")
        actual.update(cgroup_mount=str(point), cgroup_root=mount_root, cgroup_membership=memberships[0])
        for key, name in (("cpu_max", "cpu.max"), ("memory_max", "memory.max"), ("pids_max", "pids.max")):
            actual[key] = Path(group, name).read_text().strip()
        expected = {"cpu_max": f'{requested["cpu_quota_us"]} {requested["cpu_period_us"]}',
                    "memory_max": str(requested["memory_max_bytes"]), "pids_max": str(requested["pids_max"])}
        if any(actual[key] != value for key, value in expected.items()):
            raise ValueError("actual cgroup limits differ from frozen request")
    if plan["sample_mode"] != "check" and not actual["enforced"]:
        raise ValueError("measured/allocation runs require a CPU and memory envelope")
    (root / "resources.json").write_text(json.dumps(actual, indent=2) + "\n")
    command = plan["command"]
    receipt = {"schema_version": 1, "run_id": plan["run_id"], "mode": plan["mode"],
               "host_process_id": os.getpid(), "command": command,
               "host_executable_sha256": digest(command[0]),
               "binary_sha256": digest(root / plan["binary_name"]),
               "launcher_sha256": digest(Path(__file__))}
    (root / "launch.json").write_text(json.dumps(receipt, indent=2) + "\n")
    os.execv(command[0], command)


if __name__ == "__main__":
    launch(Path(sys.argv[1]))
