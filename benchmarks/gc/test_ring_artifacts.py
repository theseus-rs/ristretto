import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest
from canonical_service import digest
from ring_artifacts import peer_command, validate_resources
from ring_launch import cgroup_mount


class RingArtifactTests(unittest.TestCase):
    def test_peer_logging_is_confined_to_separate_diagnostic_forks(self):
        admission = {"method": "small", "argument": 7, "expected_result": 1}
        configuration = {"collector": "G1", "heap_bytes": 67108864, "active_processors": 2}
        for mode in ("check", "warmup-check", "allocation", "measure"):
            plan = {"run_id": "test", "mode": mode, "warmup_invocations": 0, "samples": 1}
            command = peer_command(Path("/capture"), Path("/jdk"), admission, plan, configuration)
            self.assertEqual(any(arg.startswith("-Xlog:") for arg in command), mode in {"check", "allocation"})
            self.assertEqual("-XX:+LogCompilation" in command, mode in {"check", "allocation"})

    def test_ordinary_directory_cannot_claim_kernel_resource_enforcement(self):
        with tempfile.TemporaryDirectory() as directory:
            for name in ("cgroup.procs", "memory.max", "cpu.max", "pids.max"):
                Path(directory, name).write_text("1")
            with self.assertRaises(ValueError): cgroup_mount(directory)

    def test_real_untimed_launcher_pid_and_resource_receipt(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            shutil.copy2("/bin/true", root / "program")
            shutil.copy2(Path(__file__).with_name("ring_launch.py"), root / "launcher.py")
            command = [str(root / "program")]
            (root / "launch-plan.json").write_text(json.dumps({"run_id": "pid-check", "mode": "native", "sample_mode": "check",
                "command": command, "resources": None, "cgroup": None, "binary_name": "program"}))
            process = subprocess.Popen([sys.executable, str(root / "launcher.py"), str(root)])
            self.assertEqual(process.wait(timeout=10), 0)
            receipt = json.loads((root / "launch.json").read_text())
            self.assertEqual(receipt["host_process_id"], process.pid)
            self.assertEqual(receipt["host_executable_sha256"], digest(root / "program"))
            resources = json.loads((root / "resources.json").read_text())
            self.assertFalse(resources["enforced"])
            self.assertEqual(resources["cpu_affinity"], sorted(os.sched_getaffinity(0)))
            self.assertIsNone(resources["cgroup_membership"])

    def test_invalid_or_duplicate_cpu_limits_reject(self):
        for cpus in ([], [1, 1], [True], [2, 1]):
            with self.assertRaises(ValueError):
                validate_resources({"cpu_affinity": cpus, "cpu_quota_us": 100000, "cpu_period_us": 100000,
                                    "memory_max_bytes": 1024, "pids_max": 10})


if __name__ == "__main__": unittest.main()
