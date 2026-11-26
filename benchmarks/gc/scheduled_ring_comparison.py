"""Independent fixed-arrival checks. Finite observations never certify capacity."""
import json
from canonical_service import fields, integer
from ring_comparison import SIZES, _profiles, validate_admission
from shared_ring_comparison import cycle_bounds
from report import quantile
SECOND = 10**9


def validate_plan(plan):
    fields(plan, 'schema_version run_id mode warmup_invocations duration_seconds arrival_rate max_completions', 'scheduled plan')
    if (type(plan['schema_version']) is not int or plan['schema_version'] != 2
            or plan['mode'] not in {'scheduled-check','scheduled-measure'}
            or not isinstance(plan['run_id'],str) or not plan['run_id']):
        raise ValueError('unsupported scheduled plan')
    for key, low, high in [('warmup_invocations',0,128),('duration_seconds',1,86400),
                           ('arrival_rate',1,SECOND),('max_completions',1,SECOND)]:
        if integer(plan[key],key,low)>high: raise ValueError('scheduled plan bound exceeded')
    if plan['mode']=='scheduled-check' and (plan['warmup_invocations']!=0 or plan['max_completions']!=3):
        raise ValueError('functional check requires three untimed calls and no warmup')
    return plan


def arrivals(offset, rate, offered):
    return min(offered, ((offset+1)*rate+SECOND-1)//SECOND)


def histogram(value, count):
    fields(value,'max_ns buckets_us','latency histogram')
    maximum=integer(value['max_ns'],'histogram maximum')
    rows=value['buckets_us']
    if not isinstance(rows,list) or len(rows)>4096: raise ValueError('unbounded histogram')
    total=0; previous=-1
    for row in rows:
        if not isinstance(row,list) or len(row)!=2: raise ValueError('invalid histogram bucket')
        upper=integer(row[0],'bucket upper',previous+1);total+=integer(row[1],'bucket count',1);previous=upper
        if upper>1024:
            exponent=upper.bit_length()-1;width=1<<(exponent-6)
            if (upper-(1<<exponent))%width: raise ValueError('noncanonical histogram bucket')
    if total!=count or (bool(rows)!=(count>0)): raise ValueError('histogram count mismatch')
    if rows:
        us=(maximum+999)//1000
        if us<=1024: expected=us
        else:
            exponent=us.bit_length()-1;width=1<<(exponent-6)
            expected=(1<<exponent)+((us-(1<<exponent)+width-1)//width)*width
        if rows[-1][0]!=expected: raise ValueError('histogram maximum has wrong bucket')
    elif maximum!=0: raise ValueError('empty histogram claims maximum')


def schedule(value, plan, completed):
    validate_plan(plan)
    if plan['mode']=='scheduled-check':
        if value is not None or completed!=3: raise ValueError('untimed scheduled check leaked clock or count')
        return None
    fields(value,'duration_ns start_clock_ns end_clock_ns deadline_clock_ns offered completed deadline_completed '
           'deadline_backlog after_deadline_completions unfinished peak_backlog completion_cap_reached status service response timeline','schedule')
    duration=plan['duration_seconds']*SECOND;offered=plan['duration_seconds']*plan['arrival_rate']
    start=integer(value['start_clock_ns'],'offer start');end=integer(value['end_clock_ns'],'offer end',start+duration)
    if integer(value['deadline_clock_ns'],'deadline')!=start+duration or integer(value['duration_ns'],'duration')!=duration:
        raise ValueError('offer deadline/duration changed')
    for key in ('offered','completed','deadline_completed','deadline_backlog','after_deadline_completions','unfinished','peak_backlog'):
        integer(value[key],key)
    on_time=value['deadline_completed'];late=completed-on_time
    if (value['offered']!=offered or value['completed']!=completed or not 0<=on_time<=completed<=min(offered,plan['max_completions'])
            or value['deadline_backlog']!=offered-on_time or value['unfinished']!=offered-completed
            or value['after_deadline_completions']!=late or late not in (0,1)
            or not value['unfinished']<=value['peak_backlog']<=offered):
        raise ValueError('offered/deadline/backlog/completion accounting differs')
    if type(value['completion_cap_reached']) is not bool: raise ValueError('cap state must be boolean')
    cap=value['completion_cap_reached']
    if cap and (completed!=plan['max_completions'] or completed>=offered or late):
        raise ValueError('cap state inconsistent')
    expected='completion_cap' if cap else 'unfinished' if completed<offered else 'complete'
    if value['status']!=expected: raise ValueError('schedule terminal state differs')
    histogram(value['service'],completed);histogram(value['response'],completed)
    if value['response']['max_ns']<value['service']['max_ns'] or value['response']['max_ns']>end-start:
        raise ValueError('latency maximum impossible')
    # Completion-relative samples are bounded progress observations, not arrival events.
    timeline=value['timeline']
    if not isinstance(timeline,list) or len(timeline)>128: raise ValueError('unbounded progress timeline')
    last_time=-1;last_count=0
    for row in timeline:
        if not isinstance(row,list) or len(row)!=3:raise ValueError('invalid progress row')
        offset=integer(row[0],'progress offset',last_time+1);count=integer(row[1],'progress count',last_count+1)
        backlog=integer(row[2],'progress backlog')
        if (offset>end-start or count>completed or backlog!=max(0,arrivals(offset,plan['arrival_rate'],offered)-count)
                or backlog>value['peak_backlog']):raise ValueError('progress hides arrivals or exceeds terminal state')
        last_time,last_count=offset,count
    return {'offered':offered,'completed':completed,'deadline_completed':on_time,'deadline_backlog':offered-on_time,
        'after_deadline_completions':late,'unfinished':offered-completed,'status':expected,
        'deadline_completed_ops_s':on_time/plan['duration_seconds'],'completed_ops_s':completed*SECOND/(end-start),
        'elapsed_ns':end-start,'peak_backlog':value['peak_backlog'],
        'service_p50_us':quantile(value['service'],5000) if completed else None,
        'response_p50_us':quantile(value['response'],5000) if completed else None,
        'response_p999_bucket_us':quantile(value['response'],9990) if completed else None,
        'extreme_tail_qualified':False}


def common(packet, admission, plan):
    validate_admission(admission);validate_plan(plan)
    if (admission['schema_version']!=3 or type(packet['schema_version']) is not int or packet['schema_version']!=1
            or packet['qualification'] is not False or packet['global_capture_complete'] is not False
            or packet['sustainable_throughput_available'] is not False or packet['checksum_verified'] is not True
            or packet['arrival_policy']!='fixed_index_release_logical_fifo_v1'
            or packet['sample_mode']!=plan['mode']
            or type(packet['warmup_invocations']) is not int or packet['warmup_invocations']!=plan['warmup_invocations']
            or type(packet['expected_result']) is not int or packet['expected_result']!=admission['expected_result']):
        raise ValueError('scheduled protocol/identity/claim mismatch')


def validate_canonical(packet, identity, admission, plan):
    fields(packet,'schema_version qualification identity sample_mode scope collector_lifecycle arrival_policy warmup_invocations '
           'clock clock_resolution_ns expected_result checksum_verified heap_cleanup_verified data workload_counts heap preparation '
           'heap_detach_cleanup_and_drop_ns program_teardown_ns global_capture_complete sustainable_throughput_available','scheduled canonical')
    common(packet,admission,plan)
    if (packet['scope']!='shared_heap_scheduled_v1' or packet['collector_lifecycle']!='one_heap_and_context_across_warmup_and_schedule'
            or packet['heap_cleanup_verified'] is not True or json.dumps(packet['identity'],sort_keys=True)!=json.dumps(identity,sort_keys=True)):
        raise ValueError('scheduled canonical lifetime or artifact binding differs')
    if packet['clock'] not in {'clock_monotonic_ns','process_instant_ns'}:raise ValueError('unsupported clock')
    if packet['clock_resolution_ns'] is not None:integer(packet['clock_resolution_ns'],'clock resolution',1)
    elif packet['clock']=='clock_monotonic_ns':raise ValueError('native clock resolution missing')
    measured=plan['mode']=='scheduled-measure'
    fields(packet['preparation'],'admission_ns compilation_ns heap_setup_and_attach_ns','preparation')
    for key,value in {**packet['preparation'],'heap_detach_cleanup_and_drop_ns':packet['heap_detach_cleanup_and_drop_ns'],
                      'program_teardown_ns':packet['program_teardown_ns']}.items():
        if measured and (key!='compilation_ns' or admission['tier']=='compiled'):integer(value,key)
        elif value is not None:raise ValueError('unavailable phase must remain null')
    data=packet['data'];profiles=('method_profiles_before_warmup','method_profiles_before_samples','method_profiles_after_samples')
    fields(data,'completed checksum schedule primary_cycle_batch '+' '.join(profiles),'scheduled data')
    completed=integer(data['completed'],'completed')
    if type(data['checksum']) is not int or data['checksum']!=admission['expected_result']*completed:raise ValueError('checksum differs')
    timing=schedule(data['schedule'],plan,completed)
    bounds=cycle_bounds(data['primary_cycle_batch'],measured)
    if bounds['lower']<admission['minimum_primary_cycles']:raise ValueError('missing primary-cycle exposure')
    if measured and any(data['primary_cycle_batch'][k]!=data['schedule'][k] for k in ('start_clock_ns','end_clock_ns')):
        raise ValueError('cycle clocks differ from actual schedule boundaries')
    heap=packet['heap'];fields(heap,'max_payload_bytes reserved_catalog_bytes cleanup_limit payload_bytes_before_cleanup','heap')
    for k,v in heap.items():integer(v,k)
    if (heap['max_payload_bytes']!=admission['max_payload_bytes'] or heap['cleanup_limit']!=admission['heap_cleanup_polls']
            or heap['reserved_catalog_bytes']==0 or heap['payload_bytes_before_cleanup']>heap['max_payload_bytes']):raise ValueError('heap bound differs')
    counts=packet['workload_counts'];fields(counts,'warmup sample cleanup_polls released_during_cleanup','counts')
    allocations=2*(SIZES[admission['method']][0]+admission['argument'])
    for phase,n in [('warmup',plan['warmup_invocations']),('sample',completed)]:
        fields(counts[phase],'allocations released_objects helper_polls native_slices','phase counts')
        for k,v in counts[phase].items():integer(v,k)
        if counts[phase]['allocations']!=allocations*n or (n==0 and any(counts[phase].values())):raise ValueError('logical allocation model differs')
    cleanup=integer(counts['cleanup_polls'],'cleanup polls');released=integer(counts['released_during_cleanup'],'cleanup releases')
    if (cleanup>heap['cleanup_limit'] or released>cleanup or counts['warmup']['allocations']+counts['sample']['allocations']
            != counts['warmup']['released_objects']+counts['sample']['released_objects']+released):raise ValueError('heap cleanup balance differs')
    if admission['tier']=='compiled':
        for key,n in zip(profiles,(0,plan['warmup_invocations'],plan['warmup_invocations']+completed)):
            _profiles(data[key],admission['method'],admission['argument'],n)
        for left,right,phase in [(data[profiles[0]],data[profiles[1]],'warmup'),(data[profiles[1]],data[profiles[2]],'sample')]:
            if any(b[k]<a[k] for a,b in zip(left,right) for k in ('entries','resumes','backedges')):raise ValueError('method census reversed')
            if sum(b['entries']+b['resumes']-a['entries']-a['resumes'] for a,b in zip(left,right))!=counts[phase]['native_slices']:
                raise ValueError('native slice census differs')
    elif any(data[k] is not None for k in profiles) or any(counts[p]['native_slices'] for p in ('warmup','sample')):
        raise ValueError('interpreted capture claims native execution')
    return {'valid':True,'qualification':False,'scope':packet['scope'],'schedule':timing,'primary_cycle_batch_bounds':bounds,
            'gc_activity_observed':True,'heap_cleanup_verified':True,'global_capture_complete':False}


def validate_peer(packet, admission, plan):
    fields(packet,'schema_version qualification run_id sample_mode scope arrival_policy warmup_invocations expected_result '
           'checksum_verified completed checksum schedule gc_observation_scope gc_collection_counts global_capture_complete '
           'sustainable_throughput_available','scheduled peer')
    common(packet,admission,plan)
    if packet['run_id']!=plan['run_id'] or packet['scope']!='retained_peer_heap_scheduled_v1':raise ValueError('peer scope or run differs')
    count=integer(packet['completed'],'completed')
    if type(packet['checksum']) is not int or packet['checksum']!=count*admission['expected_result']:raise ValueError('peer checksum differs')
    timing=schedule(packet['schedule'],plan,count)
    if packet['gc_observation_scope']!='whole_schedule_batch_including_driver_gaps':raise ValueError('peer GC scope differs')
    rows=packet['gc_collection_counts']
    if not isinstance(rows,list) or not rows:raise ValueError('peer GC observations missing')
    names=set();activity=False
    for row in rows:
        fields(row,'name before after','GC bean')
        if not isinstance(row['name'],str) or not row['name'] or row['name'] in names:raise ValueError('invalid GC bean inventory')
        names.add(row['name'])
        if (row['before'] is None)!=(row['after'] is None):raise ValueError('partial GC count')
        if row['before'] is not None:
            a=integer(row['before'],'GC before');b=integer(row['after'],'GC after',a);activity|=b>a
    return {'valid':True,'qualification':False,'scope':packet['scope'],'schedule':timing,
            'gc_activity_observed':activity,'heap_cleanup_verified':None,'global_capture_complete':False}
