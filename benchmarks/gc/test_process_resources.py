import copy
import json
import os
from pathlib import Path
import signal
import subprocess
import sys
import tempfile
import unittest
import uuid
from unittest.mock import patch

from canonical_service import digest
from process_resources import supported, validate_resources, wait_resources
from ring_artifacts import process_resources


@unittest.skipUnless(supported(), "Linux wait4 and pidfd required")
class ProcessResourceTests(unittest.TestCase):
    def launch(self, code, timeout=10):
        child = subprocess.Popen([sys.executable, "-c", code],
                                 stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        receipt = wait_resources(child, timeout, run_id=str(uuid.uuid4()))
        self.assertEqual(receipt["host_process_id"], child.pid)
        self.assertEqual(receipt["exit_code"], child.returncode)
        self.assertEqual(validate_resources(receipt, child.pid, child.returncode, receipt["run_id"], timeout), receipt)
        return receipt

    def test_exit_signal_and_watchdog_have_distinct_outcomes(self):
        normal = self.launch("raise SystemExit(7)")
        self.assertEqual(normal["exit_code"], 7)
        self.assertFalse(normal["timed_out"])
        killed = self.launch("import os,signal; os.kill(os.getpid(), signal.SIGTERM)")
        self.assertEqual(killed["exit_code"], -signal.SIGTERM)
        self.assertFalse(killed["timed_out"])
        timeout = self.launch("import signal; signal.pause()", timeout=.1)
        self.assertEqual(timeout["exit_code"], -signal.SIGKILL)
        self.assertTrue(timeout["timed_out"])

    def test_large_then_small_child_rss_is_not_cumulative_parent_accounting(self):
        large = self.launch("data = bytearray(64 * 1024 * 1024); assert data[-1] == 0")
        small = self.launch("data = bytearray(1024 * 1024); assert data[-1] == 0")
        self.assertGreater(large["peak_rss_bytes"], small["peak_rss_bytes"] + 32 * 1024 * 1024)
        self.assertGreater(small["peak_rss_bytes"], 0)
        self.assertNotEqual(large["host_process_id"], small["host_process_id"])

    def test_another_live_child_is_not_reaped_or_attributed(self):
        sleeper = subprocess.Popen([sys.executable, "-c", "import signal; signal.pause()"])
        try:
            other = self.launch("raise SystemExit(3)")
            self.assertNotEqual(other["host_process_id"], sleeper.pid)
            self.assertIsNone(sleeper.poll())
        finally:
            sleeper.kill()
            sleeper.wait()

    def test_exec_preserves_exact_pid(self):
        with tempfile.TemporaryDirectory() as directory:
            output = Path(directory, "pid")
            child = subprocess.Popen([sys.executable, "-c",
                "import os,sys; os.execv(sys.executable, [sys.executable, '-c', "
                "\"import os,pathlib; pathlib.Path(os.environ['PID_OUTPUT']).write_text(str(os.getpid()))\"])"] ,
                env={**os.environ, "PID_OUTPUT": str(output)})
            receipt = wait_resources(child, 10, run_id="resource-test")
            self.assertEqual(int(output.read_text()), receipt["host_process_id"])
            self.assertFalse(receipt["guest_only"])
            self.assertEqual(receipt["exit_code"], 0)

    def test_already_reaped_and_wrong_wait_pid_reject(self):
        child = subprocess.Popen([sys.executable, "-c", "pass"])
        child.wait()
        with self.assertRaisesRegex(ValueError, "already reaped"):
            wait_resources(child, 10, run_id="resource-test")
        child = subprocess.Popen([sys.executable, "-c", "pass"])
        real_wait = os.wait4
        def wrong_pid(pid, options):
            actual, status, usage = real_wait(pid, options)
            return actual + 1, status, usage
        with patch("process_resources.os.wait4", side_effect=wrong_pid):
            with self.assertRaisesRegex(RuntimeError, "another process"):
                wait_resources(child, 10, run_id="resource-test")
        self.assertIsNotNone(child.returncode)

    def test_unsupported_metrics_are_null(self):
        child = subprocess.Popen([sys.executable, "-c", "pass"])
        with patch("process_resources.supported", return_value=False):
            receipt = wait_resources(child, 10, run_id="resource-test")
        self.assertFalse(receipt["available"])
        self.assertIsNone(receipt["peak_rss_bytes"])
        validate_resources(receipt, child.pid, 0, "resource-test", 10)
        receipt["peak_rss_bytes"] = 0
        with self.assertRaises(ValueError):
            validate_resources(receipt, child.pid, 0, "resource-test", 10)

    def test_invalid_binding_units_status_and_scope_reject(self):
        receipt = self.launch("pass")
        mutations = {"run_id": "another-run", "host_process_id": receipt["host_process_id"] + 1,
                     "peak_rss_bytes": receipt["peak_rss_bytes"] + 1,
                     "raw_wait_status": 7 << 8, "scope": "measurement_window",
                     "user_cpu_seconds": float("nan"), "minor_faults": -1,
                     "available": 1, "guest_only": True, "process_tree_total": True,
                     "measurement_window_only": True, "timeout_seconds": 11}
        for key, value in mutations.items():
            with self.subTest(field=key):
                altered = {**receipt, key: value}
                with self.assertRaises(ValueError):
                    validate_resources(altered, receipt["host_process_id"], 0, receipt["run_id"], 10)

    def test_source_bound_receipt_and_legacy_unavailability(self):
        receipt = self.launch("pass")
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source = Path(__file__).parent
            manifest = {}
            for original, saved in (("run_ring_sample.py", "controller.py"),
                                    ("process_resources.py", "resource-observer.py")):
                (root / saved).write_bytes((source / original).read_bytes())
                manifest["benchmarks/gc/" + original] = digest(root / saved)
            (root / "source-manifest.json").write_text(json.dumps(manifest))
            (root / "process-resources.json").write_text(json.dumps(receipt))
            (root / "launch-plan.json").write_text(json.dumps({"run_id": receipt["run_id"],
                "mode": "native", "watchdog_seconds": 10}))
            run = {"schema_version": 2, "run_id": receipt["run_id"], "mode": "native",
                   "host_process_id": receipt["host_process_id"], "exit_code": 0,
                   "command": [], "launcher_sha256": "a" * 64, "admission_sha256": "b" * 64,
                   "workload_sha256": "c" * 64, "host_executable_sha256": "d" * 64,
                   "controller_sha256": digest(root / "controller.py"),
                   "resource_observer_sha256": digest(root / "resource-observer.py"),
                   "process_resources_sha256": digest(root / "process-resources.json"),
                   "launch_plan_sha256": digest(root / "launch-plan.json")}
            self.assertEqual(process_resources(root, run), receipt)
            altered = {**receipt, "host_process_id": receipt["host_process_id"] + 1}
            (root / "process-resources.json").write_text(json.dumps(altered))
            run["process_resources_sha256"] = digest(root / "process-resources.json")
            with self.assertRaisesRegex(ValueError, "PID/exit"):
                process_resources(root, run)
            for field, value in (("run_id", "stale-run"), ("timeout_seconds", 11)):
                altered = {**receipt, field: value}
                (root / "process-resources.json").write_text(json.dumps(altered))
                run["process_resources_sha256"] = digest(root / "process-resources.json")
                with self.subTest(field=field), self.assertRaises(ValueError):
                    process_resources(root, run)
            altered = copy.deepcopy(receipt)
            altered["timed_out"] = True
            (root / "process-resources.json").write_text(json.dumps(altered))
            run["process_resources_sha256"] = digest(root / "process-resources.json")
            with self.assertRaisesRegex(ValueError, "watchdog"):
                process_resources(root, run)
            (root / "process-resources.json").write_text(json.dumps(receipt))
            run["process_resources_sha256"] = digest(root / "process-resources.json")
            (root / "controller.py").write_text("changed")
            with self.assertRaisesRegex(ValueError, "binding"):
                process_resources(root, run)
            legacy = {k: v for k, v in run.items() if k not in
                      {"controller_sha256", "resource_observer_sha256", "process_resources_sha256", "launch_plan_sha256"}}
            legacy["schema_version"] = 1
            with self.assertRaisesRegex(ValueError, "schema-2"):
                process_resources(root, legacy)
            del manifest["benchmarks/gc/process_resources.py"]
            (root / "source-manifest.json").write_text(json.dumps(manifest))
            with self.assertRaisesRegex(ValueError, "legacy"):
                process_resources(root, legacy)
            (root / "process-resources.json").unlink()
            self.assertIsNone(process_resources(root, legacy))

    def test_wasmtime_observation_is_bound_to_host_process(self):
        executable = os.environ.get("RISTRETTO_TEST_WASMTIME")
        if not executable:
            self.skipTest("set RISTRETTO_TEST_WASMTIME for real host execution")
        with tempfile.TemporaryDirectory() as directory:
            module = Path(directory, "empty.wat")
            module.write_text('(module (func (export "_start")))')
            child = subprocess.Popen([executable, "run", str(module)],
                                     stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
            receipt = wait_resources(child, 10, run_id="resource-test")
            self.assertEqual(receipt["exit_code"], 0)
            self.assertEqual(receipt["host_process_id"], child.pid)
            self.assertFalse(receipt["guest_only"])
            self.assertGreater(receipt["peak_rss_bytes"], 0)


if __name__ == "__main__":
    unittest.main()
