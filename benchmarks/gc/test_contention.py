"""A completed teardown cannot erase an unfinished reclamation cycle."""
import copy
import unittest
from contention import validate, validate_resources, validate_fixture_hashes


def evidence():
    return {'policy': 'candidate', 'kind': 'full', 'producers': 4, 'cycles_requested': 2,
        'cycles_completed': 1, 'cycle_ns': [100], 'unfinished_cycle_ns': 200,
        'elapsed_ns': 310, 'allocated_objects': 32, 'barrier_operations': 1000,
        'live_before_cleanup': 17, 'checksum_ok': True, 'diagnostics': True,
        'teardown_live': 0, 'teardown_bytes': 0}


class ContentionEvidenceTests(unittest.TestCase):
    def test_censored_cycle_survives_clean_teardown(self):
        validate(evidence(), 'candidate', 'full', 4, 2)

    def test_missing_work_and_false_completion_are_rejected(self):
        for field, value in [('unfinished_cycle_ns', None), ('cycles_completed', 2),
                             ('allocated_objects', 16), ('cycle_ns', []),
                             ('elapsed_ns', 299), ('teardown_bytes', 8),
                             ('producers', 1), ('checksum_ok', False)]:
            result = copy.deepcopy(evidence())
            result[field] = value
            with self.assertRaises(ValueError, msg=field):
                validate(result, 'candidate', 'full', 4, 2)

    def test_finished_run_has_all_cycle_samples_and_no_censor(self):
        result = evidence()
        result.update(cycles_completed=2, cycle_ns=[100, 200], unfinished_cycle_ns=None)
        validate(result, 'candidate', 'full', 4, 2)

    def test_working_set_cannot_silently_fall_back_to_one_edge(self):
        result = evidence()
        with self.assertRaises(ValueError):
            validate(result, 'candidate', 'full', 4, 2, targets=16)
        result['targets'] = 16
        validate(result, 'candidate', 'full', 4, 2, targets=16)
        result['live_before_cleanup'] = 1
        with self.assertRaises(ValueError):
            validate(result, 'candidate', 'full', 4, 2, targets=16)

    def test_root_churn_is_distinct_from_barrier_contention(self):
        result = evidence()
        with self.assertRaises(ValueError):
            validate(result, 'candidate', 'full', 4, 2, operation='roots')
        result.update(operation='roots', producer_operations=1000, barrier_operations=0)
        validate(result, 'candidate', 'full', 4, 2, operation='roots')
        with self.assertRaises(ValueError):
            validate(result, 'candidate', 'full', 4, 2)
        for field, value in [('producer_operations', 0), ('barrier_operations', 1000)]:
            corrupt = dict(result, **{field: value})
            with self.assertRaises(ValueError, msg=field):
                validate(corrupt, 'candidate', 'full', 4, 2, operation='roots')
        del result['producer_operations']
        with self.assertRaises(ValueError):
            validate(result, 'candidate', 'full', 4, 2, operation='roots')

    def test_barrier_counter_matches_producer_counter(self):
        result = evidence()
        result.update(operation='barriers', producer_operations=1000)
        validate(result, 'candidate', 'full', 4, 2)
        result['producer_operations'] = 999
        with self.assertRaises(ValueError):
            validate(result, 'candidate', 'full', 4, 2)

    def test_clean_exit_does_not_hide_memory_ceiling_events(self):
        validate_resources({'memory_events': {'max': 0, 'oom': 0, 'oom_kill': 0}})
        for name in ['max', 'oom', 'oom_kill', 'oom_group_kill']:
            with self.assertRaises(ValueError, msg=name):
                validate_resources({'memory_events': {name: 1}})

    def test_changed_or_unrecorded_fixture_cannot_form_a_pair(self):
        validate_fixture_hashes({'before': {'fixture_sha256': 'same'}, 'after': {'fixture_sha256': 'same'}})
        for after in [{}, {'fixture_sha256': 'different'}]:
            with self.assertRaises(ValueError):
                validate_fixture_hashes({'before': {'fixture_sha256': 'same'}, 'after': after})


if __name__ == '__main__':
    unittest.main()
