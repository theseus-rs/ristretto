import copy
import itertools
import unittest
from ring_comparison import validate_admission, validate_canonical, validate_matrix, validate_matrix_admission
from shared_ring_comparison import cycle_bounds, validate_shared
from test_ring_comparison import canonical


def fixture():
    old, identity, admission, plan = canonical()
    admission.update(schema_version=2,window_kind='shared_heap_invocation_v1',max_payload_bytes=64*1024*1024,
                     heap_cleanup_polls=4_000_000,minimum_primary_cycles=1)
    plan['mode']='measure'
    packet = {k:old[k] for k in ('qualification','diagnostic_only','identity','service_recording_enabled','warmup_invocations',
        'clock','clock_resolution_ns','method_profiles_before_warmup','method_profiles_before_samples','method_profiles_after_samples',
        'global_capture_complete','scheduled_latency_available','sustainable_throughput_available')}
    packet.update(schema_version=2,sample_mode='measure',scope='shared_heap_invocation_v1',
        collector_lifecycle='one_heap_and_context_across_warmup_and_samples',completed_samples=1,
        expected_result=admission['expected_result'],checksum_verified=True,heap_cleanup_verified=True,
        samples=[{'start_clock_ns':1000,'end_clock_ns':1020,'elapsed_ns':20,'result':admission['expected_result'],
                  'allocations':30,'released_objects':20,'helper_polls':100,'native_slices':72}],
        preparation={'admission_ns':1,'compilation_ns':2,'heap_setup_and_attach_ns':3},
        heap_detach_cleanup_and_drop_ns=4,program_teardown_ns=5,
        primary_cycle_batch={'scope':'sample_batch_including_driver_gaps_and_observer','before_start':{'lower':2,'upper':2},
            'after_start':{'lower':2,'upper':2},'before_end':{'lower':5,'upper':5},'after_end':{'lower':5,'upper':5},
            'start_clock_ns':990,'end_clock_ns':1030,'elapsed_ns':40,'completed_primary_cycles':{'lower':3,'upper':3}},
        heap={'max_payload_bytes':64*1024*1024,'reserved_catalog_bytes':16384,'cleanup_limit':4_000_000,'payload_bytes_before_cleanup':128},
        workload_counts={'warmup_allocations':0,'warmup_released_objects':0,'warmup_helper_polls':0,'warmup_native_slices':0,
            'sample_allocations':30,'sample_released_objects':20,'cleanup_polls':100,'released_during_cleanup':10},
        in_invocation_gc_exposure_proven=False)
    return packet,identity,admission,plan


class SharedRingTests(unittest.TestCase):
    def test_shared_allocation_release_and_compiled_census_valid(self):
        result=validate_shared(*fixture())
        self.assertTrue(result['heap_cleanup_verified'])
        self.assertTrue(result['compiled_method_census_valid'])
        self.assertEqual(result['primary_cycle_batch_bounds'],{'lower':3,'upper':3})
        self.assertFalse(result['global_capture_complete'])

    def test_all_existing_and_shared_scopes_remain_distinct(self):
        legacy=canonical();validate_canonical(*legacy)
        with self.assertRaises(ValueError):validate_shared(*legacy)
        args=fixture()
        with self.assertRaises(ValueError):validate_canonical(*args)
        for change in ({'schema_version':1},{'window_kind':'full_invocation_v1'},{'max_payload_bytes':64*1024*1024+1},
                       {'minimum_primary_cycles':0},{'heap_cleanup_polls':0}):
            admission=copy.deepcopy(args[2]);admission.update(change)
            with self.assertRaises(ValueError):validate_admission(admission)

    def test_missing_forged_inconsistent_or_zero_exposure_reject(self):
        changes=[lambda p:p.update(primary_cycle_batch=None),
            lambda p:p['primary_cycle_batch'].update(before_end=None),
            lambda p:p['primary_cycle_batch']['completed_primary_cycles'].update(lower=4),
            lambda p:p['primary_cycle_batch']['before_end'].update(lower=1,upper=1),
            lambda p:p['primary_cycle_batch']['after_start'].update(lower=2,upper=5),
            lambda p:p['primary_cycle_batch'].update(scope='one_invocation'),
            lambda p:p.update(in_invocation_gc_exposure_proven=True),
            lambda p:p.update(global_capture_complete=True)]
        for change in changes:
            args=fixture();change(args[0])
            with self.subTest(change=change),self.assertRaises(ValueError):validate_shared(*args)

    def test_timestamps_bind_every_call_to_batch(self):
        changes=[lambda p:p['samples'][0].update(start_clock_ns=980,elapsed_ns=40),
            lambda p:p['samples'][0].update(end_clock_ns=1040,elapsed_ns=40),
            lambda p:p['samples'][0].update(elapsed_ns=19),
            lambda p:p['primary_cycle_batch'].update(elapsed_ns=39),
            lambda p:p['primary_cycle_batch'].update(end_clock_ns=None),
            lambda p:p['samples'][0].update(start_clock_ns=True)]
        for change in changes:
            args=fixture();change(args[0])
            with self.subTest(change=change),self.assertRaises(ValueError):validate_shared(*args)

    def test_whole_heap_accounting_and_census_reject_forgery(self):
        changes=[lambda p:p['workload_counts'].update(released_during_cleanup=9),
            lambda p:p['workload_counts'].update(cleanup_polls=4_000_001),
            lambda p:p['workload_counts'].update(warmup_native_slices=1),
            lambda p:p['samples'][0].update(allocations=29),
            lambda p:p['samples'][0].update(native_slices=71),
            lambda p:p['method_profiles_after_samples'].pop(0),
            lambda p:p['method_profiles_after_samples'][1].update(backedges=0),
            lambda p:p['heap'].update(max_payload_bytes=1),
            lambda p:p['heap'].update(reserved_catalog_bytes=0),
            lambda p:p.update(heap_cleanup_verified=False)]
        for change in changes:
            args=fixture();change(args[0])
            with self.subTest(change=change),self.assertRaises(ValueError):validate_shared(*args)

    def test_wasi_unknown_resolution_is_available_only_as_unknown(self):
        args=fixture();p,identity,admission,plan=args
        admission['tier']=identity['tier']='interpreted';p['identity']=copy.deepcopy(identity)
        p['clock']='process_instant_ns';p['clock_resolution_ns']=None
        p['preparation']['compilation_ns']=None;p['samples'][0]['native_slices']=0
        for key in ('method_profiles_before_warmup','method_profiles_before_samples','method_profiles_after_samples'):p[key]=None
        self.assertTrue(validate_shared(*args)['valid'])
        p['clock']='clock_monotonic_ns'
        with self.assertRaises(ValueError):validate_shared(*args)

    def test_warmup_shape_check_remains_counter_and_timer_free(self):
        args=fixture();p,_,_,plan=args
        plan['mode']=p['sample_mode']='warmup-check'
        for key in ('samples','workload_counts','primary_cycle_batch','method_profiles_before_warmup','method_profiles_before_samples','method_profiles_after_samples'):p[key]=None
        for key in p['preparation']:p['preparation'][key]=None
        p['heap_detach_cleanup_and_drop_ns']=p['program_teardown_ns']=None;p['heap']['payload_bytes_before_cleanup']=None
        self.assertFalse(validate_shared(*args)['timing_samples_available'])
        p['primary_cycle_batch']={}
        with self.assertRaises(ValueError):validate_shared(*args)

    def test_cycle_bounds_enclose_monotone_clock_histories(self):
        # Independent enumeration: six true counters at observation/clock boundaries.
        for history in itertools.combinations_with_replacement(range(3),6):
            a,start,b,c,end,d=history
            choices=[[(lo,hi) for lo in range(value+1) for hi in range(value,3)] for value in (a,b,c,d)]
            for selected in itertools.product(*choices):
                batch={'scope':'sample_batch_including_driver_gaps_and_observer','start_clock_ns':0,'end_clock_ns':10,'elapsed_ns':10}
                for key,(lo,hi) in zip(('before_start','after_start','before_end','after_end'),selected):batch[key]={'lower':lo,'upper':hi}
                batch['completed_primary_cycles']={'lower':max(0,selected[2][0]-selected[1][1]),'upper':selected[3][1]-selected[0][0]}
                bounds=cycle_bounds(batch,True)
                self.assertLessEqual(bounds['lower'],end-start);self.assertGreaterEqual(bounds['upper'],end-start)

    def test_matrix_downgrade_and_admission_lifetime_mismatch_reject(self):
        plan={'schema_version':2,'qualification':False,'check_only':False,'warmup_check':False,'builds':{},'steps':7,
            'warmup_invocations':100,'samples':30,'allocation_samples':3,'forks':3,'workloads':['small'],
            'heap_bytes':64*1024*1024,'watchdog_seconds':120,'resources_sha256':'a'*64,'admission_sha256':{},'cases':[],
            'scopes':{'canonical':'shared_heap_invocation_v1','hotspot':'workload_call_deferred_gc_v1'},
            'canonical_gc_cycles_available':False,'canonical_primary_cycle_scope':'sample_batch_including_driver_gaps_and_observer','canonical_primary_cycle_required':True,'scheduled_latency_available':False,'sustainable_throughput_available':False,
            'body_diagnostics_in_this_matrix':False}
        validate_matrix(plan);validate_matrix_admission(plan,fixture()[2])
        with self.assertRaises(ValueError):validate_matrix_admission(plan,canonical()[2])
        plan['schema_version']=1
        with self.assertRaises(ValueError):validate_matrix(plan)
        plan['scopes']['canonical']='full_invocation_v1'
        del plan['canonical_primary_cycle_scope'],plan['canonical_primary_cycle_required']
        del plan['watchdog_seconds'],plan['warmup_check']
        validate_matrix(plan)

if __name__=='__main__':unittest.main()
