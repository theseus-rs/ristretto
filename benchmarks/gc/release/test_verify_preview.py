import json
import subprocess
import sys
from pathlib import Path
import unittest
from unittest.mock import patch

from verify_preview import check_report, command_for, run


def example(compiled=False):
    return dict(result=64, cleared_witnesses=2, bytes_freed=128, collections_completed=1,
                interpreted_entries=0 if compiled else 25, compiled_entries=25 if compiled else 0,
                compiled_backedges=8 if compiled else 0)


class ReportChecks(unittest.TestCase):
    def test_both_execution_modes(self):
        for compiled in (False, True):
            report = json.dumps(example(compiled))
            check_report(report, compiled)
            with self.assertRaises(ValueError):
                check_report(report, not compiled)

    def test_missing_wrong_duplicate_and_empty_reclamation_are_rejected(self):
        fields = example()
        for key in fields:
            with self.subTest(key=key), self.assertRaises(ValueError):
                check_report(json.dumps({k: v for k, v in fields.items() if k != key}), False)
        for key in ("bytes_freed", "collections_completed", "cleared_witnesses", "result"):
            with self.subTest(key=key), self.assertRaises(ValueError):
                check_report(json.dumps(dict(fields, **{key: 0})), False)
        report = json.dumps(fields)
        for malformed in (report.replace('"result": 64', '"result":64,"result":64'),
                          report + report, "noise" + report):
            with self.assertRaises(ValueError):
                check_report(malformed, False)

    def test_tier_commands_keep_locked_features_and_explicit_modes(self):
        for tier in ("native-interpreted", "native-compiled", "wasi-interpreted"):
            command = command_for(tier)
            self.assertIn("--locked", command)
            self.assertIn("--no-default-features", command)
            self.assertEqual("experimental-reference-dispatch" in command[command.index("--features")+1],
                             tier == "native-compiled")
            self.assertEqual("wasm32-wasip2" in command, tier == "wasi-interpreted")


class ProcessChecks(unittest.TestCase):
    def test_success_and_nonzero_exit(self):
        self.assertEqual(run([sys.executable, "-c", "print('example')"], Path.cwd(), 5),
                         "example\n")
        with self.assertRaisesRegex(RuntimeError, "status 7"):
            run([sys.executable, "-c", "raise SystemExit(7)"], Path.cwd(), 5)

    def test_watchdog_terminates_and_reaps_child(self):
        created = []
        original_popen = subprocess.Popen

        def track(*args, **kwargs):
            child = original_popen(*args, **kwargs)
            created.append(child)
            return child

        with patch("verify_preview.subprocess.Popen", side_effect=track):
            with self.assertRaisesRegex(RuntimeError, "exceeded"):
                run([sys.executable, "-c", "import time; time.sleep(60)"], Path.cwd(), 0.1)
        self.assertEqual(len(created), 1)
        self.assertIsNotNone(created[0].returncode)
        self.assertNotEqual(created[0].returncode, 0)


if __name__ == "__main__":
    unittest.main()
