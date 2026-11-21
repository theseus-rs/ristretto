import subprocess
import sys
from pathlib import Path
import unittest
from unittest.mock import patch

from verify_preview import check_report, command_for, parse_report, run


def example(compiled=False):
    available = "false" if compiled else "true"
    entries, backedges = (25, 8) if compiled else (0, 0)
    return (f"Report {{ result: 64, allocations: 16, instruction_count_available: {available}, "
            "entry_initialization: InitializationWork { allocations: 0, instructions: 0 }, "
            "released_objects: 16, payload_bytes_after_cleanup: 0, catalog_bytes_after_drop: 0, "
            f"compiled_entries: {entries}, compiled_backedges: {backedges} }}")


class ReportChecks(unittest.TestCase):
    def test_nested_counts_do_not_overwrite_invocation_counts(self):
        report = check_report(example(), False)
        self.assertEqual(report["allocations"], 16)
        self.assertEqual(report["entry_initialization"]["allocations"], 0)

    def test_compiled_report_checks_actual_compiled_census(self):
        check_report(example(True), True)
        with self.assertRaises(ValueError):
            check_report(example(), True)

    def test_invalid_values_missing_fields_and_duplicate_authority_reject(self):
        for field in ("result: 64", "allocations: 16", "released_objects: 16",
                      "payload_bytes_after_cleanup: 0", "catalog_bytes_after_drop: 0"):
            with self.subTest(field=field), self.assertRaises(ValueError):
                check_report(example().replace(field, field.split(":")[0] + ": -1", 1), False)
        for malformed in (example().replace("result: 64, ", ""),
                          example().replace("result: 64", "result: 64, result: 64"),
                          example().replace("allocations: 16", "allocations: 0"),
                          example().replace("allocations: 16", "allocations: true"),
                          example() + "\n" + example(), "noise\n" + example(),
                          example()[:-1], example() + " garbage"):
            with self.subTest(malformed=malformed), self.assertRaises(ValueError):
                check_report(malformed, False)

    def test_nested_duplicate_fields_reject(self):
        with self.assertRaises(ValueError):
            parse_report("Report { nested: Work { count: 0, count: 0 } }")

    def test_tier_commands_keep_locked_features_and_explicit_modes(self):
        for tier in ("native-interpreted", "native-compiled", "wasi-interpreted"):
            command = command_for(tier)
            self.assertIn("--locked", command)
            self.assertIn("--no-default-features", command)
            self.assertEqual("--compiled" in command, tier == "native-compiled")
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
