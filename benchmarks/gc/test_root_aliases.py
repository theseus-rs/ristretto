import unittest
from root_aliases import validate, validate_counts


def fixture(mode='canonical-leaf'):
    report = dict(version=1, mode=mode, copies=16, clone_drop_ns=100, backing_bytes=64,
                  backing_slots=0 if mode == 'legacy' else 2,
                  inline_total_bytes=0 if mode == 'legacy' else 1000,
                  charged_bytes=0 if mode == 'legacy' else 1128, drops=1,
                  checksum=0xa4c97015ee12853b, teardown_bytes=0, teardown_catalog_bytes=0)
    rows = [dict(checkpoint=label, allocations=a, bytes=b, releases=r, released_bytes=rb, live=l, peak=p)
            for label,a,b,r,rb,l,p in [
                ('prepared-before', 2, 100, 0, 0, 100, 100),
                ('prepared-after', 3, 164, 0, 0, 164, 164),
                ('prepared-dropped', 3, 164, 1, 64, 100, 164),
                ('clones-before', 100, 5000, 10, 1000, 4000, 4100),
                ('clones-after', 100, 5000, 10, 1000, 4000, 4100)]]
    return rows + [report]


class RootAliasEvidence(unittest.TestCase):
    def test_all_modes_accept_complete_counts(self):
        for mode in ['legacy', 'canonical-leaf', 'canonical-graph']:
            rows = fixture(mode)
            self.assertEqual(validate_counts(rows, mode, 16), rows)

    def test_rejects_incomplete_or_reordered_checkpoints(self):
        for rows in [fixture()[:-1], list(reversed(fixture()))]:
            with self.assertRaises(ValueError): validate_counts(rows, 'canonical-leaf', 16)

    def test_rejects_extra_schema_and_bool_counts(self):
        for key, value in [('extra', 1), ('drops', True)]:
            rows = fixture(); rows[-1][key] = value
            with self.assertRaises(ValueError): validate_counts(rows, 'canonical-leaf', 16)

    def test_rejects_wrong_workload_and_leaked_payload(self):
        for key, value in [('copies', 15), ('checksum', 0), ('drops', 0), ('teardown_bytes', 1), ('clone_drop_ns', 0)]:
            rows = fixture(); rows[-1][key] = value
            with self.assertRaises(ValueError): validate_counts(rows, 'canonical-leaf', 16)

    def test_rejects_missing_pending_or_live_root_charge(self):
        rows = fixture(); rows[-1]['charged_bytes'] -= 64
        with self.assertRaisesRegex(ValueError, 'storage charge'): validate_counts(rows, 'canonical-leaf', 16)

    def test_rejects_layout_claim_that_agrees_with_wrong_charge(self):
        rows = fixture(); rows[-1]['backing_bytes'] = 32; rows[-1]['charged_bytes'] = 1064
        with self.assertRaisesRegex(ValueError, 'actual wrapper backing'): validate_counts(rows, 'canonical-leaf', 16)

    def test_rejects_missing_or_double_free(self):
        for key, value in [('releases', 0), ('releases', 2), ('released_bytes', 0), ('live', 164)]:
            rows = fixture(); rows[2][key] = value
            with self.assertRaisesRegex(ValueError, 'release exactly once'): validate_counts(rows, 'canonical-leaf', 16)

    def test_rejects_allocator_work_during_clone(self):
        for key in ['allocations', 'bytes', 'releases', 'released_bytes', 'peak']:
            rows = fixture(); rows[4][key] += 1
            with self.assertRaisesRegex(ValueError, 'allocator work'): validate_counts(rows, 'canonical-leaf', 16)

    def test_rejects_invalid_requested_bounds(self):
        for mode, copies in [('unknown', 16), ('canonical-leaf', 0), ('canonical-leaf', True), ('canonical-leaf', 10000001)]:
            with self.assertRaises(ValueError): validate(fixture()[-1], mode, copies)


if __name__ == '__main__': unittest.main()
