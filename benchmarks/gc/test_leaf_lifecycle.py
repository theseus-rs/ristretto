"""Reject plausible but incomplete lifecycle/count results before reporting."""
import unittest
from leaf_lifecycle import packet, validate, validate_counts


def valid(mode="canonical-auto"):
    row = dict(version=1, mode=mode, operations=7, batch=4, payload_bytes=32,
               logical_object_bytes=64, heap_limit=67108864, operation_ns=50,
               cleanup_ns=50, lifecycle_ns=110, setup_ns=10, teardown_ns=10,
               checksum=28, allocation_polls=70, helper_polls=140,
               released_objects=7, released_roots=7, catalog_bytes=1000,
               payload_peak=256, teardown_bytes=0, teardown_catalog_bytes=0)
    if mode == "legacy":
        for key in ["allocation_polls", "helper_polls", "released_objects", "released_roots", "catalog_bytes"]:
            row[key] = 0
    return row


def counts():
    return [dict(version=1, mode="canonical-auto", operations=7, batch=4, round=i,
                 allocations=30, allocated_bytes=1000, releases=30, released_bytes=1000,
                 live_before=100, live_after=100, peak_above_baseline=700) for i in range(3)]


class ResultIntegrity(unittest.TestCase):
    def test_accept_complete_modes(self):
        for mode in ["legacy", "canonical-explicit", "canonical-auto"]:
            validate(valid(mode), mode, 7, 4)

    def test_reject_omitted_or_miscounted_work(self):
        faults = dict(checksum=27, released_objects=6, released_roots=6, payload_peak=192,
                      teardown_bytes=1, teardown_catalog_bytes=1, allocation_polls=0, helper_polls=0)
        for key, value in faults.items():
            with self.subTest(key=key), self.assertRaises(ValueError):
                validate(valid() | {key: value}, "canonical-auto", 7, 4)

    def test_reject_identity_or_configuration_change(self):
        for key, value in dict(version=2, mode="legacy", operations=8, batch=8, payload_bytes=64,
                               heap_limit=134217728).items():
            with self.subTest(key=key), self.assertRaises(ValueError):
                validate(valid() | {key: value}, "canonical-auto", 7, 4)

    def test_reject_noninteger_or_negative_data(self):
        for value in [True, 1.0, "1", -1, None, float("nan")]:
            with self.subTest(value=value), self.assertRaises(ValueError):
                validate(valid() | {"setup_ns": value}, "canonical-auto", 7, 4)

    def test_reject_missing_or_extra_schema(self):
        row = valid()
        del row["teardown_bytes"]
        for value in [row, valid() | {"extra": 0}, []]:
            with self.assertRaises(ValueError):
                validate(value, "canonical-auto", 7, 4)

    def test_reject_timing_excluding_cleanup(self):
        with self.assertRaises(ValueError):
            validate(valid() | {"lifecycle_ns": 50}, "canonical-auto", 7, 4)
        with self.assertRaises(ValueError):
            validate(valid() | {"cleanup_ns": 0}, "canonical-auto", 7, 4)

    def test_reject_duplicate_json_keys_or_extra_packet(self):
        for text in ['{"checksum":27,"checksum":28}', '{}\n{}']:
            with self.assertRaises(ValueError):
                packet(text)

    def test_counts_accept_balanced_and_expose_retention(self):
        validate_counts(counts(), "canonical-auto", 7, 4)
        rows = counts()
        for i, row in enumerate(rows):
            row.update(released_bytes=990, live_before=100+i*10, live_after=110+i*10)
        # Boundedness is reported from real deltas, not manufactured by the parser.
        validate_counts(rows, "canonical-auto", 7, 4)

    def test_counts_reject_hidden_or_unexplained_retention(self):
        for key, value in dict(released_bytes=990, live_after=110, live_before=90,
                               peak_above_baseline=-1, allocations=0, round=2).items():
            rows = counts()
            rows[1][key] = value
            with self.subTest(key=key), self.assertRaises(ValueError):
                validate_counts(rows, "canonical-auto", 7, 4)

    def test_counts_require_three_unique_rounds(self):
        for rows in [counts()[:2], counts() + counts()[:1], [counts()[0]]*3]:
            with self.assertRaises(ValueError):
                validate_counts(rows, "canonical-auto", 7, 4)


if __name__ == "__main__":
    unittest.main()
