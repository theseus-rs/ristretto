import copy
import unittest

from report import expected_checksum
from revisions import compare


BEFORE, AFTER = "wasm-candidate-before", "wasm-candidate"


def fixture():
    options = {"runtimes": [BEFORE, AFTER], "workloads": ["allocation"],
               "rates": [10], "repeats": 2, "seconds": 1, "seed": 42}
    rows = []
    for repeat in range(2):
        for runtime in [BEFORE, AFTER]:
            data = {"schema": 1, "workload": "allocation", "rate": 10, "seed": 42,
                    "completed": 10, "offered": 10, "unfinished": 0, "duration_ns": 10**9,
                    "elapsed_ns": 10**9, "checksum": expected_checksum("allocation", 10),
                    "service": {"max_ns": 1000, "buckets_us": [[1, 10]]},
                    "response": {"max_ns": 2000, "buckets_us": [[2, 10]]}}
            rows.append({"workload": "allocation", "rate": 10, "runtime": runtime,
                         "repeat": repeat, "target": "wasm", "status": "ok",
                         "measurement": data,
                         "cgroup": {"limits": {"memory.max": 1024**2, "cpu.max": "100000 100000"},
                                    "memory_peak": 1024, "memory_events": {}}})
    return {"options": options}, rows, {"inputs_unchanged": True}


class RevisionTests(unittest.TestCase):
    def test_complete_pairs_are_an_exploratory_comparison(self):
        metadata, rows, integrity = fixture()
        result = compare(metadata, rows, integrity, BEFORE, AFTER)
        self.assertFalse(result["qualification"])
        self.assertTrue(result["resource_caps_enforced"])
        group = result["groups"][0]
        self.assertFalse(group["tail_samples_sufficient"])
        self.assertEqual(group["metrics"]["throughput_ops_s"]["after_before"]["median"], 1)

    def test_missing_failed_duplicate_or_corrupt_members_cannot_be_dropped(self):
        metadata, rows, integrity = fixture()
        failed, corrupt = copy.deepcopy(rows), copy.deepcopy(rows)
        failed[0]["status"] = "timeout"
        corrupt[0]["measurement"]["checksum"] += 1
        for bad in [rows[:-1], rows + [rows[0]], failed, corrupt]:
            with self.subTest(rows=bad), self.assertRaises(ValueError):
                compare(metadata, bad, integrity, BEFORE, AFTER)

    def test_target_budget_integrity_and_exhaustion_must_match(self):
        for mutation in ["target", "budget", "oom", "integrity"]:
            metadata, rows, integrity = fixture()
            if mutation == "target":
                rows[0]["target"] = "native"
            elif mutation == "budget":
                rows[0]["cgroup"]["limits"]["memory.max"] *= 2
            elif mutation == "oom":
                rows[0]["cgroup"]["memory_events"]["oom"] = 1
            else:
                integrity["inputs_unchanged"] = False
            with self.subTest(mutation=mutation), self.assertRaises(ValueError):
                compare(metadata, rows, integrity, BEFORE, AFTER)

    def test_censored_tails_have_no_ratio_but_throughput_retains_unfinished_work(self):
        metadata, rows, integrity = fixture()
        data = rows[1]["measurement"]
        data.update(completed=5, unfinished=5, checksum=expected_checksum("allocation", 5))
        for metric in ["service", "response"]:
            data[metric]["buckets_us"][0][1] = 5
        group = compare(metadata, rows, integrity, BEFORE, AFTER)["groups"][0]
        self.assertFalse(group["complete"])
        self.assertEqual(group["unfinished_after"], [5, 0])
        self.assertIsNone(group["metrics"]["response_p999_us"]["after_before"])
        self.assertIsNotNone(group["metrics"]["throughput_ops_s"]["after_before"])

    def test_uncapped_results_and_zero_resolution_are_explicit(self):
        metadata, rows, integrity = fixture()
        for row in rows:
            row["cgroup"] = None
            for metric in ["service", "response"]:
                row["measurement"][metric] = {"max_ns": 0, "buckets_us": [[0, 10]]}
        result = compare(metadata, rows, integrity, BEFORE, AFTER)
        self.assertFalse(result["resource_caps_enforced"])
        group = result["groups"][0]
        self.assertNotIn("charged_peak_mib", group["metrics"])
        self.assertIsNone(group["metrics"]["response_p999_us"]["after_before"])

    def test_same_unknown_or_single_pair_selection_is_rejected(self):
        metadata, rows, integrity = fixture()
        for before, after in [(BEFORE, BEFORE), (BEFORE, "missing")]:
            with self.assertRaises(ValueError):
                compare(metadata, rows, integrity, before, after)
        metadata["options"]["repeats"] = 1
        with self.assertRaises(ValueError):
            compare(metadata, rows, integrity, BEFORE, AFTER)


if __name__ == "__main__":
    unittest.main()
