"""Freeze a fixed-arrival pilot or adaptive search. Executes no workloads."""
import argparse
import json
from pathlib import Path
import shutil
from canonical_service import digest, workload_digest
from canonical_service_artifacts import read_json, verify_build
from ring_artifacts import validate_resources, verify_peer_build
from ring_comparison import validate_admission

RUNTIMES=['g1','zgc','native-compiled','native-interpreted','wasi-interpreted']


def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--inputs',type=Path,required=True,help='frozen shared ring classes/admissions/resources')
    for family in ('native','wasi','peer'):p.add_argument('--'+family+'-build',type=Path,required=True)
    p.add_argument('--output',type=Path,required=True)
    p.add_argument('--workloads',nargs='+',choices=['small','medium','large'],default=['medium'])
    p.add_argument('--policy',choices=['fixed-pilot','adaptive','delivered-capacity'],default='fixed-pilot')
    p.add_argument('--runtimes',nargs='+',choices=RUNTIMES,default=RUNTIMES,
                   help='runtime group sharing the same initial rate for a delivered-capacity search')
    p.add_argument('--target-offers',type=int,default=12500,
                   help='minimum offer target at the smaller of the offered and initial rates')
    p.add_argument('--rates',nargs='+',type=int,default=[16,32,64,128])
    p.add_argument('--seconds',type=int,default=10)
    p.add_argument('--forks',type=int,default=3)
    p.add_argument('--warmup',type=int,default=100)
    p.add_argument('--watchdog-seconds',type=int,default=120)
    p.add_argument('--initial-rate',type=int,default=32)
    p.add_argument('--max-rate',type=int,default=131072)
    p.add_argument('--max-points',type=int,default=16)
    p.add_argument('--tolerance',type=float,default=.05)
    # Same defaults as capacity.py; provisional screen limits, never acceptance budgets.
    p.add_argument('--p999-us',type=int,default=1000)
    p.add_argument('--backlog-us',type=int,default=10000)
    p.add_argument('--minimum-completions',type=int,default=10000)
    a=p.parse_args()
    if (len(set(a.workloads))!=len(a.workloads) or a.rates!=sorted(set(a.rates)) or not all(1<=n<=10**9 for n in a.rates)
        or not 1<=a.seconds<=86400 or not 2<=a.forks<=32 or not 0<=a.warmup<=128
        or a.watchdog_seconds<a.seconds+30 or not 1<=a.initial_rate<=a.max_rate<=10**9
        or not 1<=a.max_points<=32 or not 0<a.tolerance<1
        or min(a.p999_us,a.backlog_us,a.minimum_completions)<=0):raise ValueError('invalid predeclared protocol')
    if len(set(a.runtimes)) != len(a.runtimes):
        raise ValueError('duplicate runtime')
    if a.policy == 'delivered-capacity':
        if not a.minimum_completions <= a.target_offers <= 86400:
            raise ValueError('target offers must cover the sample floor and fit a one-request/s point')
    elif a.runtimes != RUNTIMES:
        raise ValueError('runtime subsets require a delivered-capacity search')
    builds={};source=None
    for family in ('native','wasi','peer'):
        path=getattr(a,family+'_build').resolve();b=verify_peer_build(path) if family=='peer' else verify_build(path)
        if family!='peer' and '--release' not in b['command']:raise ValueError('release artifacts required')
        if source is None:source=b['source_manifest_sha256']
        if source!=b['source_manifest_sha256']:raise ValueError('runtime source snapshots differ')
        name='peer-build.json' if family=='peer' else 'build.json'
        builds[family]={'directory':str(path),'receipt_sha256':digest(path/name),'source_manifest_sha256':source}
    root=a.output.resolve();root.mkdir(parents=True,exist_ok=False)
    shutil.copytree(a.inputs/'classes',root/'classes');(root/'admissions').mkdir()
    manifest=read_json(a.native_build/'source-manifest.json')
    for name in ('CanonicalRingWorkload.class','CanonicalRingWorkload$Node.class'):
        if digest(root/'classes'/name)!=manifest['ristretto_vm/tests/fixtures/canonical/'+name]:raise ValueError('shared bytecode changed')
    origins={}
    for workload in a.workloads:
        for tier in ('interpreted','compiled'):
            name=f'{workload}-{tier}.json';v=validate_admission(read_json(a.inputs/'admissions'/name))
            if v['schema_version']!=2 or v['argument']!=257:raise ValueError('requires original persistent 257-step workload')
            origins[name]=digest(a.inputs/'admissions'/name)
            v.update(schema_version=3,window_kind='shared_heap_scheduled_v1');validate_admission(v)
            (root/'admissions'/name).write_text(json.dumps(v,indent=2)+'\n')
    resources=validate_resources(read_json(a.inputs/'resources.json'))
    # Preserve exact whole-system envelope, including serialization/hash.
    shutil.copy2(a.inputs/'resources.json',root/'resources.json')
    plan={'schema_version':1,'qualification':False,'scope':'shared_heap_scheduled_v1','policy':a.policy,
        'builds':builds,'workloads':a.workloads,'runtimes':a.runtimes,'rates':a.rates,'seconds':a.seconds,
        'forks':a.forks,'warmup':a.warmup,'watchdog_seconds':a.watchdog_seconds,'max_completions':10**9,
        'resources_sha256':digest(root/'resources.json'),'workload_sha256':workload_digest(root/'classes'),
        'admission_sha256':{n:digest(root/'admissions'/n) for n in origins},'origin_admission_sha256':origins,
        'search':{'initial_rate':a.initial_rate,'maximum':a.max_rate,'max_points':a.max_points,'tolerance':a.tolerance},
        'screen_limits':{'p999_us':a.p999_us,'backlog_us':a.backlog_us,'minimum_completions':a.minimum_completions},
        'screen_limit_origin':'capacity.py defaults unless explicitly overridden before launch; not qualification budgets',
        'offer_policy':'fixed_index_release_logical_fifo_v1','request_drain_policy':'no new call after deadline; finish at most one in flight',
        'heap_drain_policy':'separate final context detach and complete payload/catalog cleanup',
        'global_capture_complete':False,'scheduled_extreme_tails_qualified':False,'sustainable_throughput_qualified':False}
    if a.policy == 'delivered-capacity':
        plan.update(schema_version=2, target_offers=a.target_offers)
    (root/'plan.json').write_text(json.dumps(plan,indent=2)+'\n')
    print(json.dumps({'plan_sha256':digest(root/'plan.json'),'executed':False,
        'fixed_processes':len(a.rates)*len(a.workloads)*len(RUNTIMES)*a.forks if a.policy=='fixed-pilot' else None},indent=2))

if __name__=='__main__':main()
