import unittest
from root_refund_order import validate


def fixture():
    return [dict(version=1, mode=mode, checkpoints=2, refunds=1, last_free=free,
                 last_refund=free+1, frees=15, refunded_bytes=1000,
                 before_charge=1000, after_charge=0, valid=True)
            for mode, free in [('leaf', 18), ('graph', 42)]]


class RootRefundOrderEvidence(unittest.TestCase):
    def test_complete_audit(self):
        rows = fixture(); self.assertEqual(validate(rows), rows)

    def test_missing_or_reversed_modes(self):
        for rows in [fixture()[:1], list(reversed(fixture()))]:
            with self.assertRaises(ValueError): validate(rows)

    def test_actual_ordinary_arc_drop_gap(self):
        rows = fixture(); rows[0].update(last_free=19, last_refund=18)
        with self.assertRaisesRegex(ValueError, 'backing freed after'): validate(rows)

    def test_equal_events_do_not_prove_order(self):
        rows = fixture(); rows[1]['last_refund'] = rows[1]['last_free']
        with self.assertRaisesRegex(ValueError, 'backing freed after'): validate(rows)

    def test_missing_or_wrong_refund(self):
        for key, value in [('refunds', 0), ('refunds', 2), ('refunded_bytes', 999), ('before_charge', 0), ('after_charge', 1)]:
            rows = fixture(); rows[1][key] = value
            with self.assertRaisesRegex(ValueError, 'credit refund'): validate(rows)

    def test_missing_free_or_checkpoint(self):
        for key, value in [('frees', 0), ('last_free', 0), ('checkpoints', 1)]:
            rows = fixture(); rows[0][key] = value
            with self.assertRaises(ValueError): validate(rows)

    def test_bad_types_and_schema(self):
        for key, value in [('extra', 0), ('refunds', True), ('last_free', -1), ('valid', 1)]:
            rows = fixture(); rows[0][key] = value
            with self.assertRaises(ValueError): validate(rows)

    def test_explicit_failure_rejected(self):
        rows = fixture(); rows[0]['valid'] = False
        with self.assertRaisesRegex(ValueError, 'reported a failure'): validate(rows)

    def test_replayed_sequence_rejected(self):
        rows = fixture(); rows[1].update(last_free=18, last_refund=19)
        with self.assertRaisesRegex(ValueError, 'sequence did not advance'): validate(rows)


if __name__ == '__main__': unittest.main()
