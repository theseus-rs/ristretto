import copy
import json
from pathlib import Path
import tempfile
import unittest
from scheduled_ring_comparison import arrivals, histogram, schedule, validate_plan, validate_canonical, validate_peer
from ring_comparison import validate_admission
from run_scheduled_capacity import classify, confirmed_interval, point_window, read_point_window
from test_shared_ring_comparison import fixture as shared_fixture


def plan():
    return dict(schema_version=2,run_id='scheduled-test',mode='scheduled-measure',warmup_invocations=0,
                duration_seconds=1,arrival_rate=3,max_completions=1000)


def scheduled():
    # Independent event model: releases [0,333333333,666666666], serial service 400 ms.
    return dict(duration_ns=10**9,start_clock_ns=0,end_clock_ns=1200000000,deadline_clock_ns=10**9,
        offered=3,completed=3,deadline_completed=2,deadline_backlog=1,after_deadline_completions=1,
        unfinished=0,peak_backlog=1,completion_cap_reached=False,status='complete',
        service={'max_ns':400000000,'buckets_us':[[401408,3]]},
        response={'max_ns':533333334,'buckets_us':[[401408,1],[466944,1],[540672,1]]},
        timeline=[[400000000,1,1],[800000000,2,1],[1200000000,3,0]])


def canonical():
    old,identity,admission,_=shared_fixture()
    admission.update(schema_version=3,window_kind='shared_heap_scheduled_v1',tier='interpreted')
    identity['tier']='interpreted';identity['run_id']='scheduled-test'
    p=plan();p['mode']='scheduled-check';p['max_completions']=3
    packet={k:copy.deepcopy(old[k]) for k in ('qualification','clock','clock_resolution_ns','checksum_verified','heap_cleanup_verified',
        'heap','global_capture_complete','sustainable_throughput_available','expected_result')}
    packet.update(schema_version=1,identity=identity,sample_mode='scheduled-check',scope='shared_heap_scheduled_v1',
        collector_lifecycle='one_heap_and_context_across_warmup_and_schedule',arrival_policy='fixed_index_release_logical_fifo_v1',
        warmup_invocations=0,preparation={'admission_ns':None,'compilation_ns':None,'heap_setup_and_attach_ns':None},
        heap_detach_cleanup_and_drop_ns=None,program_teardown_ns=None,
        data={'completed':3,'checksum':admission['expected_result']*3,'schedule':None,
            'method_profiles_before_warmup':None,'method_profiles_before_samples':None,'method_profiles_after_samples':None,
            'primary_cycle_batch':copy.deepcopy(old['primary_cycle_batch'])},
        workload_counts={'warmup':dict(allocations=0,released_objects=0,helper_polls=0,native_slices=0),
                         'sample':dict(allocations=90,released_objects=80,helper_polls=300,native_slices=0),
                         'cleanup_polls':100,'released_during_cleanup':10})
    for key in ('start_clock_ns','end_clock_ns','elapsed_ns'):packet['data']['primary_cycle_batch'][key]=None
    return packet,identity,admission,p


class ScheduledTests(unittest.TestCase):
    def test_release_count_matches_enumeration_without_consumer(self):
        for rate in (1,3,7,11,999,1001):
            release=[i*10**9//rate for i in range(rate*2)]
            for offset in (0,1,333333332,333333333,666666666,999999999,10**9,2*10**9):
                self.assertEqual(arrivals(offset,rate,len(release)),sum(t<=offset for t in release))

    def test_deadline_backlog_and_inflight_finish_remain_separate(self):
        value=schedule(scheduled(),plan(),3)
        self.assertEqual(value['deadline_completed'],2)
        self.assertEqual(value['deadline_backlog'],1)
        self.assertEqual(value['after_deadline_completions'],1)
        self.assertEqual(value['unfinished'],0)

    def test_continuously_busy_consumer_cannot_reduce_offered(self):
        for mutate in [lambda p:p.update(offered=2),lambda p:p.update(deadline_backlog=0),
                       lambda p:p.update(deadline_completed=3),lambda p:p.update(after_deadline_completions=0),
                       lambda p:p['timeline'][1].__setitem__(2,0),lambda p:p.update(peak_backlog=0),
                       lambda p:p.update(unfinished=1),lambda p:p.update(deadline_clock_ns=999999999)]:
            value=scheduled();mutate(value)
            with self.assertRaises(ValueError):schedule(value,plan(),3)

    def test_cap_and_unstarted_work_cannot_be_disguised(self):
        p=plan();p['max_completions']=1;v=scheduled();v.update(end_clock_ns=10**9,completed=1,deadline_completed=1,
            deadline_backlog=2,after_deadline_completions=0,unfinished=2,peak_backlog=2,status='completion_cap',completion_cap_reached=True,
            timeline=[[400000000,1,1]],service={'max_ns':400000000,'buckets_us':[[401408,1]]},
            response={'max_ns':400000000,'buckets_us':[[401408,1]]})
        self.assertEqual(schedule(v,p,1)['status'],'completion_cap')
        v['status']='complete'
        with self.assertRaises(ValueError):schedule(v,p,1)

    def test_bounded_histogram_counts_and_bucket_maximum(self):
        histogram({'max_ns':1024001,'buckets_us':[[1040,1]]},1)
        for value in [{'max_ns':1024001,'buckets_us':[[1024,1]]},{'max_ns':1024001,'buckets_us':[[1041,1]]},
                      {'max_ns':1,'buckets_us':[[1,2]]},{'max_ns':1,'buckets_us':[]},
                      {'max_ns':2,'buckets_us':[[1,1],[1,1]]}]:
            with self.assertRaises(ValueError):histogram(value,1)

    def test_check_scope_counters_cleanup_and_primary_exposure(self):
        args=canonical();self.assertTrue(validate_canonical(*args)['valid'])
        changes=[lambda p:p.update(heap_cleanup_verified=False),lambda p:p['workload_counts']['sample'].update(allocations=89),
            lambda p:p['workload_counts'].update(released_during_cleanup=9),lambda p:p['data'].update(checksum=1),
            lambda p:p['data'].update(schedule={}),lambda p:p.update(global_capture_complete=True),
            lambda p:p['data']['primary_cycle_batch']['completed_primary_cycles'].update(lower=0)]
        for change in changes:
            args=canonical();change(args[0])
            with self.assertRaises(ValueError):validate_canonical(*args)

    def test_scope_and_plan_downgrades_rejected(self):
        args=canonical()
        for update in [{'schema_version':2},{'window_kind':'shared_heap_invocation_v1'},{'max_payload_bytes':1}]:
            admission=copy.deepcopy(args[2]);admission.update(update)
            with self.assertRaises(ValueError):validate_admission(admission)
        for update in [{'duration_seconds':0},{'arrival_rate':0},{'max_completions':True},{'mode':'measure'}, {'samples':1}]:
            p=plan();p.update(update)
            with self.assertRaises(ValueError):validate_plan(p)

    def test_screen_limits_remain_separate_from_valid_execution(self):
        row={'valid':True,'schedule':schedule(scheduled(),plan(),3),'gc_activity_observed':True}
        limits={'minimum_completions':10000,'p999_us':1000,'backlog_us':10000}
        self.assertEqual(classify([copy.deepcopy(row)],limits,3)['status'],'invalid')
        limits['minimum_completions']=3
        self.assertEqual(classify([copy.deepcopy(row)],limits,3)['status'],'fail')
        self.assertFalse(classify([copy.deepcopy(row)],limits,3)['qualification'])

    def test_peer_zero_gc_is_explicit_not_inferred_as_exposure(self):
        _,_,admission,p=canonical()
        value=dict(schema_version=1,qualification=False,run_id=p['run_id'],sample_mode=p['mode'],scope='retained_peer_heap_scheduled_v1',
            arrival_policy='fixed_index_release_logical_fifo_v1',warmup_invocations=0,expected_result=admission['expected_result'],
            checksum_verified=True,completed=3,checksum=3*admission['expected_result'],schedule=None,
            gc_observation_scope='whole_schedule_batch_including_driver_gaps',gc_collection_counts=[dict(name='collector',before=1,after=1)],
            global_capture_complete=False,sustainable_throughput_available=False)
        self.assertFalse(validate_peer(value,admission,p)['gc_activity_observed'])
        value['gc_collection_counts'][0]['after']=2
        self.assertTrue(validate_peer(value,admission,p)['gc_activity_observed'])

    def test_delivered_capacity_retains_separate_latency_and_sample_floor_results(self):
        timing = schedule(scheduled(), plan(), 3)
        row = {'valid': True, 'schedule': timing, 'gc_activity_observed': True}
        limits = {'minimum_completions': 3, 'p999_us': 1000, 'backlog_us': 10000}
        self.assertEqual(classify([row], limits, 3, delivered=True)['status'], 'fail')
        timing.update(deadline_completed=3, deadline_backlog=0, after_deadline_completions=0)
        result = classify([row], limits, 3, delivered=True)
        self.assertEqual((result['status'], result['latency_screen_status']), ('pass', 'fail'))
        limits['minimum_completions'] = 4
        self.assertEqual(classify([row], limits, 3, delivered=True)['status'], 'invalid')

    def test_delivered_capacity_window_uses_seed_and_preserves_watchdog_margin(self):
        protocol = dict(policy='delivered-capacity', seconds=60, watchdog_seconds=240,
                        target_offers=12500, search={'initial_rate': 32})
        self.assertEqual(point_window(protocol, 64), (391, 571))
        self.assertEqual(point_window(protocol, 16), (782, 962))
        protocol['search']['initial_rate'] = 16384
        self.assertEqual(point_window(protocol, 16384), (60, 240))
        protocol['policy'] = 'fixed-pilot'
        self.assertEqual(point_window(protocol, 16), (60, 240))

    def test_confirmed_delivered_bracket_requires_activity_and_resolution(self):
        lo = dict(rate=100, status='pass', gc_activity_in_every_fork=True)
        hi = dict(rate=105, status='fail', gc_activity_in_every_fork=True)
        policy = {'tolerance': .05}
        self.assertTrue(confirmed_interval([lo, hi, lo, hi], policy))
        for change in [{'rate': 106}, {'gc_activity_in_every_fork': False}, {'status': 'pass'}]:
            self.assertFalse(confirmed_interval([lo, hi, lo, {**hi, **change}], policy))
        self.assertFalse(confirmed_interval([{**hi, 'rate': 90}, lo, lo, hi], policy))

    def test_window_belongs_to_the_declared_point_and_plan(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            point = 'medium-g1-point00-rate16384'
            path = root / (point + '.json')
            self.assertIsNone(read_point_window(root, point, 'plan'))
            path.write_text('[]')
            with self.assertRaisesRegex(ValueError, 'must be an object'):
                read_point_window(root, point, 'plan')
            record = dict(point=point, plan_sha256='plan', quiet_window_id='morning')
            for change in [dict(point='other'), dict(plan_sha256='other'), dict(quiet_window_id=' ')]:
                path.write_text(json.dumps({**record, **change}))
                with self.assertRaises(ValueError):
                    read_point_window(root, point, 'plan')
            path.write_text(json.dumps(record))
            self.assertEqual(read_point_window(root, point, 'plan'), 'morning')

if __name__=='__main__':unittest.main()
