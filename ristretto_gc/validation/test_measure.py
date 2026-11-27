import json
from pathlib import Path
from tempfile import TemporaryDirectory
import unittest

from measure import command_for, parser, record_measurement, validate


class DriverTests(unittest.TestCase):
    def options(self, *extra):
        return parser().parse_args(["--binary", "driver", "--build-provenance", "provenance.json",
                                   "--process-memory-mib", "2048", "--output", "result.json", *extra])

    def test_positional_protocol_keeps_late_options_in_their_actual_slots(self):
        o = self.options("--old-collection-interval", "4", "--independent-phases",
                         "--concurrent-publication", "--local-allocation", "--telemetry-ms", "250")
        args = command_for(o, 9000)
        self.assertEqual(len(args), 28)
        self.assertEqual(args[3], "9000")
        self.assertEqual(args[16], "true")
        self.assertEqual(args[21], "true")
        self.assertEqual(args[24:28], ["true", "250", "async", "4"])
        o.endurance_array_write = "element"
        self.assertEqual(command_for(o, 9000)[28:], ["element"])
        o.workload = "recovery"
        args = command_for(o, 0)
        self.assertEqual(len(args), 27)
        self.assertEqual(args[15], "true")
        self.assertEqual(args[16], "4096")
        self.assertEqual(args[21], "true")
        self.assertEqual(args[22], "true")
        self.assertEqual(args[25:27], ["true", "4"])

    def test_failed_process_keeps_valid_histogram_and_failure_status(self):
        values = {"operations": 2, "seconds": 1, "final_live_objects": 0,
                  "final_occupied_bytes": 0, "roots": {}, "operation": {"histogram_us": {"103": 2}}}
        with TemporaryDirectory() as temp:
            directory = Path(temp)
            output = directory / "stdout.txt"
            output.write_text(json.dumps(values))
            row = {"status": "failed", "exit_code": 101, "cgroup": {"memory_events": {}}}
            record_measurement(row, directory, "recovery", "generic")
            self.assertEqual((row["status"], row["exit_code"], row["measurement"]), ("failed", 101, values))
            output.write_text("")
            failed = {"status": "failed", "error": "original failure"}
            record_measurement(failed, directory, "recovery", "generic")
            self.assertEqual((failed["status"], failed["error"]), ("failed", "original failure"))
            self.assertIn("measurement_error", failed)
            success = {"status": "ok"}
            record_measurement(success, directory, "recovery", "generic")
            self.assertEqual(success["status"], "invalid")

    def test_retention_backlog_and_missing_histogram_samples_fail(self):
        clean = {"operations": 2, "offered_operations": 2, "backlog_operations": 0, "seconds": 1,
                 "final_live_objects": 0, "final_occupied_bytes": 0, "roots": {"final_pages": 0},
                 "operation_histogram_us": {"1": 2}, "arrival_histogram_us": {"2": 2}}
        validate(clean, "endurance")
        validate({**clean, "array_write": "element"}, "endurance", "element")
        with self.assertRaisesRegex(ValueError, "array write"):
            validate(clean, "endurance", "element")
        for field, value in [("backlog_operations", 1), ("final_live_objects", 1),
                             ("operation_histogram_us", {"1": 1})]:
            with self.assertRaises(ValueError):
                validate({**clean, field: value}, "endurance")
        with self.assertRaises(ValueError):
            validate({**clean, "roots": {"final_in_flight_work": [0, 1]}}, "endurance")


if __name__ == "__main__":
    unittest.main()
