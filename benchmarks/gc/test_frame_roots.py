import unittest
from frame_roots import parse_measurement, summarize, validate


class FramePublicationTests(unittest.TestCase):
    def measurement(self):
        return dict(schema_version=2, workload="stable1", policy="gated", operations=100,
                    warmup=10, checksum=100, target_count=1, payload_checksum=42,
                    survivors=0, collections_during=0, execution_profile=False,
                    registered_workers_before=0, registered_workers_after=0,
                    qualification=False, elapsed_ns=1000)

    def test_rejects_wrong_identity_or_incomplete_work(self):
        for key, value in [("workload", "stable8"), ("operations", 99), ("checksum", 99),
                           ("survivors", 1), ("payload_checksum", 0), ("collections_during", 1),
                           ("registered_workers_before", 1), ("registered_workers_after", 1),
                           ("schema_version", 1), ("execution_profile", True), ("elapsed_ns", 0), ("checksum", True)]:
            data = self.measurement()
            data[key] = value
            with self.subTest(key=key), self.assertRaises(ValueError):
                validate(data, "stable1", "gated", 100, 10)

    def test_measurement_must_appear_once(self):
        for stdout in ["", "FRAME_PUBLICATION {}\nFRAME_PUBLICATION {}"]:
            with self.assertRaises(ValueError):
                parse_measurement(stdout)

    def test_failed_or_missing_pair_cannot_be_dropped_from_summary(self):
        rows = [dict(workload="stable1", policy=p, repeat=i, revision=r, status="ok",
                     measurement=self.measurement())
                for p in ["gated", "counted"] for i in range(2) for r in ["before", "after"]]
        self.assertEqual(len(summarize(rows, 2, ["stable1"], 42)["results"]), 2)
        rows[-1]["status"] = "timeout"
        result = summarize(rows, 2, ["stable1"], 42)
        self.assertEqual(result["failed_processes"], [7])
        self.assertEqual(result["incomplete_cells"], [dict(workload="stable1", policy="counted")])
        self.assertEqual(len(result["results"]), 1)
        self.assertEqual(summarize(rows[:-1], 2, ["stable1"], 42)["incomplete_cells"], result["incomplete_cells"])
        with self.assertRaises(ValueError):
            summarize(rows + [rows[0]], 2, ["stable1"], 42)


if __name__ == "__main__":
    unittest.main()
