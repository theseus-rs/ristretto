"""Browser reports must preserve censored work and reject inconsistent evidence."""
import copy
import unittest
from browser import quantile, validate


def evidence():
    options = {'rate': 100, 'warmup': 1, 'seconds': 1, 'policy': 'candidate', 'heap_mib': 16, 'poll_budget': 64}
    def phase(completed):
        histogram = {'count': completed, 'max_ns': 10000, 'buckets_us': [[10, completed]]}
        return {'offered': 100, 'completed': completed, 'unfinished': 100 - completed,
                'peak_backlog': 100 - completed, 'turns': completed,
                'histograms': {name: copy.deepcopy(histogram) for name in ['arrival', 'service', 'poll', 'turn_work', 'request_turn_work', 'turn_gap', 'heartbeat_gap', 'frame_gap', 'long_task']},
                'series': [{'offered': 100, 'completed': completed, 'backlog': 100 - completed}]}
    final = dict.fromkeys(['errors', 'live_objects', 'managed_bytes', 'retired_bytes', 'root_pages',
                          'pending_barriers', 'pending_request'], 0)
    final.update(completed=197, checksum=197 * 42, finalizations=1, finished=1)
    return options, {'schema': 1, 'options': options, 'cross_origin_isolated': True,
        'warmup': phase(100), 'measurement': phase(97), 'after_teardown': final,
        'initial_memory_bytes': 65536, 'final_memory_bytes': 131072}


class BrowserEvidenceTests(unittest.TestCase):
    def test_unfinished_requests_remain_explicit_even_after_successful_teardown(self):
        options, result = evidence()
        validate(result, options)
        self.assertEqual(result['measurement']['unfinished'], 3)
        self.assertEqual(quantile(result['measurement']['histograms']['arrival'], .999), 10)

    def test_lost_samples_false_completions_and_retained_storage_are_rejected(self):
        mutations = [lambda r: r['measurement'].__setitem__('unfinished', 0),
            lambda r: r['measurement']['histograms']['arrival'].__setitem__('count', 98),
            lambda r: r['after_teardown'].__setitem__('retired_bytes', 1),
            lambda r: r['after_teardown'].__setitem__('finalizations', 0),
            lambda r: r.__setitem__('cross_origin_isolated', False),
            lambda r: r['measurement']['histograms'].pop('poll'),
            lambda r: r['measurement']['histograms'].__setitem__('request_turn_work', {'count': 0, 'max_ns': 0, 'buckets_us': []}),
            lambda r: r['measurement']['histograms'].__setitem__('request_turn_work', {'count': 1, 'max_ns': 10000, 'buckets_us': [[10, 1]]}),
            lambda r: r['measurement']['histograms']['arrival'].__setitem__('max_ns', 999999)]
        for mutate in mutations:
            options, result = evidence()
            mutate(result)
            with self.assertRaises(ValueError):
                validate(result, options)

    def test_empty_histograms_do_not_manufacture_zero_latency(self):
        self.assertIsNone(quantile({'count': 0, 'buckets_us': [], 'max_ns': 0}, .999))
        self.assertEqual(quantile({'count': 100, 'buckets_us': [[1, 99], [64, 1]], 'max_ns': 64000}, .999), 64)


if __name__ == '__main__':
    unittest.main()
