import copy
import json
from pathlib import Path
import tempfile
import unittest

import leaf_contention as screen


def fixture():
    case = dict(mode="native", actors=2, helpers=2, operations_per_actor=3, capacity=4, retained=1)
    row = dict(version=2, request_poll_limit=1000000, **case, objects=6, catalog_bytes=10000, logical_object_bytes=40,
        destructions=6, payload_teardown_bytes=0, catalog_teardown_bytes=0, counter_owners=1,
        actor_progress=[dict(actor=i, completed=3, allocation_polls=36, max_allocation_polls=12,
                            reads=6, checksum=i * 3 + 3) for i in range(2)],
        helper_progress=[dict(helper=i, polls=80, released_objects=3, released_roots=3) for i in range(2)])
    return case, row


class EvidenceTests(unittest.TestCase):
    def test_complete_individual_progress(self):
        case, row = fixture()
        self.assertIs(screen.validate(row, case), row)

    def test_actor_failure_cannot_be_hidden_in_aggregate(self):
        case, row = fixture()
        row["actor_progress"][0]["completed"] -= 1
        row["actor_progress"][1]["completed"] += 1
        with self.assertRaisesRegex(ValueError, "individual actor"):
            screen.validate(row, case)

    def test_exact_identity_and_retained_reads(self):
        case, row = fixture()
        for field in ["checksum", "reads", "actor", "allocation_polls", "max_allocation_polls"]:
            bad = copy.deepcopy(row)
            bad["actor_progress"][0][field] += -1 if field == "max_allocation_polls" else 1
            with self.subTest(field=field), self.assertRaises(ValueError):
                screen.validate(bad, case)

    def test_every_physical_and_logical_teardown_check(self):
        case, row = fixture()
        for field in ["request_poll_limit", "objects", "destructions", "payload_teardown_bytes", "catalog_teardown_bytes", "counter_owners"]:
            bad = copy.deepcopy(row)
            bad[field] += 1
            with self.subTest(field=field), self.assertRaises(ValueError):
                screen.validate(bad, case)
        for field in ["released_objects", "released_roots"]:
            bad = copy.deepcopy(row)
            bad["helper_progress"][0][field] -= 1
            with self.subTest(field=field), self.assertRaisesRegex(ValueError, "slot release"):
                screen.validate(bad, case)

    def test_integer_and_schema_checks(self):
        case, row = fixture()
        for value in [True, 6.0, -1, "6"]:
            bad = copy.deepcopy(row)
            bad["destructions"] = value
            with self.subTest(value=value), self.assertRaises(ValueError):
                screen.validate(bad, case)
        for field in row:
            bad = copy.deepcopy(row)
            del bad[field]
            with self.subTest(field=field), self.assertRaises(ValueError):
                screen.validate(bad, case)

    def test_missing_or_duplicate_participants(self):
        case, row = fixture()
        for key in ["actor_progress", "helper_progress"]:
            bad = copy.deepcopy(row)
            bad[key].pop()
            with self.assertRaisesRegex(ValueError, "missing participant"):
                screen.validate(bad, case)
            bad = copy.deepcopy(row)
            bad[key][1] = copy.deepcopy(bad[key][0])
            with self.assertRaises(ValueError):
                screen.validate(bad, case)

    def test_duplicate_json_rejected(self):
        with self.assertRaisesRegex(ValueError, "duplicate JSON"):
            screen.packet('{"destructions": 6, "destructions": 0}')

    def test_zero_retention_and_partial_ring(self):
        case, row = fixture()
        for retained, reads in [(0, 3), (1, 6), (2, 8), (4, 9)]:
            case["retained"] = row["retained"] = retained
            case["capacity"] = row["capacity"] = 2 * (retained + 1)
            for actor in row["actor_progress"]:
                actor["reads"] = reads
            screen.validate(row, case)

    def test_bad_configuration(self):
        case, _ = fixture()
        for field, value in [("mode", "bogus"), ("actors", 17), ("helpers", 0),
                             ("operations_per_actor", 10000001), ("capacity", 3), ("retained", 65)]:
            bad = case | {field: value}
            with self.subTest(field=field), self.assertRaises(ValueError):
                screen.configuration(bad)

    def test_large_catalog_has_declared_watchdog(self):
        case, row = fixture()
        case["capacity"] = row["capacity"] = 16384
        row["request_poll_limit"] = 16384 * 1024
        row["actor_progress"][0].update(max_allocation_polls=3000000, allocation_polls=3000002)
        screen.validate(row, case)
        row["request_poll_limit"] = 1000000
        with self.assertRaisesRegex(ValueError, "watchdog differs"):
            screen.validate(row, case)
        row["request_poll_limit"] = 16384 * 1024
        row["actor_progress"][0].update(max_allocation_polls=16384 * 1024 + 1, allocation_polls=16384 * 1024 + 3)
        with self.assertRaisesRegex(ValueError, "individual actor"):
            screen.validate(row, case)

    def test_raw_schedule_status_and_hashes_are_rechecked(self):
        case, result = fixture()
        with tempfile.TemporaryDirectory() as name:
            out = Path(name)
            metadata = dict(inputs={}, repeats=1, cases=[case])
            row = dict(index=0, repeat=0, case=0, status="ok", exit_code=0, result=result,
                       stdout=json.dumps(result))

            def write(rows, failures=None):
                screen.dump(out / "metadata.json", metadata)
                screen.dump(out / "schedule.json", [[0, 0]])
                (out / "raw.jsonl").write_text("".join(json.dumps(r) + "\n" for r in rows))
                screen.dump(out / "completion.json", dict(inputs_unchanged=True, failures=failures or [],
                    **{key: screen.digest(out / path) for key, path in [
                        ("raw_sha256", "raw.jsonl"), ("schedule_sha256", "schedule.json"), ("metadata_sha256", "metadata.json")]}))

            write([row])
            self.assertEqual(screen.checked_rows(out)[2], [])
            write([])
            with self.assertRaisesRegex(ValueError, "incomplete schedule"):
                screen.checked_rows(out)
            write([row | {"exit_code": 1}])
            with self.assertRaisesRegex(ValueError, "raw process"):
                screen.checked_rows(out)
            write([row | {"status": "failed"}])
            with self.assertRaisesRegex(ValueError, "failure accounting"):
                screen.checked_rows(out)
            write([row | {"status": "failed"}], [0])
            self.assertEqual(screen.checked_rows(out)[2], [0])
            write([row])
            (out / "raw.jsonl").write_text("\n")
            with self.assertRaises(ValueError):
                screen.checked_rows(out)


if __name__ == "__main__":
    unittest.main()
