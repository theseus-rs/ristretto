"""Capacity bounds must retain overload, noise, failed trials and censored tails."""
import json
from pathlib import Path
import tempfile
import unittest

from capacity import bounds, classify, next_rate
from report import expected_checksum


def point(rate, status):
    return {"rate": rate, "status": status}


def trial(repeat, completed=100, tail=100, backlog=0):
    histogram = {"max_ns": tail * 1000, "buckets_us": [[tail, completed]]}
    return {"runtime": "ristretto-candidate", "workload": "allocation", "rate": 100,
            "repeat": repeat, "status": "ok", "cgroup": {"memory_events": {"oom": 0}, "limits": {"memory.max": 1024}},
            "measurement": {"schema": 1, "workload": "allocation", "rate": 100, "seed": 42,
                            "duration_ns": 10**9, "elapsed_ns": 10**9, "completed": completed,
                            "offered": 100, "unfinished": 100 - completed, "peak_backlog": backlog,
                            "checksum": expected_checksum("allocation", completed),
                            "service": histogram, "response": histogram}}


class CapacityTests(unittest.TestCase):
    def classify(self, rows, unchanged=True):
        with tempfile.TemporaryDirectory() as temp:
            directory = Path(temp)
            options = {"runtimes": ["ristretto-candidate"], "workloads": ["allocation"],
                       "rates": [100], "seconds": 1, "seed": 42, "repeats": 2}
            (directory / "metadata.json").write_text(json.dumps({"options": options}))
            (directory / "raw.jsonl").write_text("\n".join(map(json.dumps, rows)))
            (directory / "integrity.json").write_text(json.dumps({"inputs_unchanged": unchanged}))
            return classify(directory, 1000, 10000, 50)

    def test_every_fork_must_pass_without_pooling(self):
        self.assertEqual(self.classify([trial(0), trial(1)])["status"], "pass")
        for bad in [trial(1, completed=99), trial(1, tail=1001), trial(1, backlog=2)]:
            result = self.classify([trial(0), bad])
            self.assertEqual(result["status"], "fail")
            self.assertEqual(len(result["forks"]), 2)
            self.assertTrue(result["forks"][1]["reasons"])

    def test_invalid_evidence_cannot_supply_a_capacity_upper_bound(self):
        for rows in [[trial(0)], [trial(0), trial(0)],
                     [trial(0), {**trial(1), "status": "timeout"}],
                     [trial(0), trial(1, completed=49)]]:
            self.assertEqual(self.classify(rows)["status"], "invalid")
        self.assertEqual(self.classify([trial(0), trial(1)], False)["status"], "invalid")

    def test_input_identity_and_resource_enforcement_are_required(self):
        for change in [{"runtime": "g1"}, {"rate": 101}, {"cgroup": None},
                       {"cgroup": {"memory_events": {"oom": 1}, "limits": {}}},
                       {"cgroup": {"memory_events": {}, "limits": {"memory.max": 2048}}}]:
            self.assertEqual(self.classify([trial(0), {**trial(1), **change}])["status"], "invalid")

    def test_expansion_contraction_and_bisection_are_bounded(self):
        self.assertEqual(next_rate([], 100, 1000, .05), 100)
        self.assertEqual(next_rate([point(100, "pass")], 100, 1000, .05), 200)
        self.assertEqual(next_rate([point(100, "fail")], 100, 1000, .05), 50)
        self.assertEqual(next_rate([point(100, "pass"), point(200, "fail")], 100, 1000, .05), 150)
        for points in [[point(100, "pass"), point(104, "fail")], [point(1000, "pass")], [point(1, "fail")]]:
            self.assertIsNone(next_rate(points, 100, 1000, .05))

    def test_noisy_confirmation_and_nonmonotonic_results_are_not_hidden(self):
        for points in [[point(100, "fail"), point(200, "pass")],
                       [point(100, "pass"), point(200, "fail"), point(100, "fail")],
                       [point(100, "pass"), point(200, "fail"), point(200, "pass")]]:
            self.assertEqual(bounds(points)["status"], "inconsistent")
            self.assertIsNone(next_rate(points, 100, 1000, .05))
        self.assertEqual(bounds([point(100, "pass"), point(200, "invalid")])["status"], "invalid")


if __name__ == "__main__":
    unittest.main()
