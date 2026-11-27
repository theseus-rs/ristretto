"""Execute a separately approved fixed-arrival plan; preserve every invalid/overload point."""
import argparse
import json
from pathlib import Path
import subprocess
import sys
import time
from capacity import bounds, next_rate
from canonical_service import digest, workload_digest
from canonical_service_artifacts import read_json
from ring_artifacts import check_canonical, check_peer
from scheduled_ring_comparison import validate_plan


def point_window(plan, rate):
    """Freeze duration and watchdog before any fork at the offered rate."""
    seconds = plan['seconds']
    if plan['policy'] == 'delivered-capacity':
        anchor = min(rate, plan['search']['initial_rate'])
        seconds = max(seconds, (plan['target_offers'] + anchor - 1) // anchor)
    return seconds, seconds + (plan['watchdog_seconds'] - plan['seconds'])


def classify(forks, limits, rate, delivered=False):
    if not forks or any(not row.get('valid') for row in forks):
        return {'status':'invalid','reason':'missing or invalid process evidence','forks':forks}
    status='pass'
    delivery_status='pass'
    for row in forks:
        timing=row['schedule'];reasons=[]
        if timing is None or timing['status']=='completion_cap':
            return {'status':'invalid','reason':'missing schedule or completion cap reached','forks':forks}
        if timing['completed']<limits['minimum_completions']:
            return {'status':'invalid','reason':'below predeclared screening sample floor','forks':forks}
        if (timing['deadline_completed'] != timing['offered'] or timing['deadline_backlog']
                or timing['after_deadline_completions'] or timing['unfinished']):
            reasons.append('unfinished at offer deadline')
            delivery_status='fail'
        if timing['response_p999_bucket_us']>limits['p999_us']:reasons.append('response histogram p99.9 screen limit')
        if timing['peak_backlog']*10**6/rate>limits['backlog_us']:reasons.append('peak logical backlog time screen limit')
        row['screen_reasons']=reasons
        if reasons:status='fail'
    result = {'status':status,'forks':forks,'gc_activity_in_every_fork':all(f['gc_activity_observed'] for f in forks),
              'qualification':False}
    if delivered:
        result.update(status=delivery_status, latency_screen_status=status)
    return result


def confirmed_interval(points, policy):
    lower, upper = points[-2:]
    return (lower['status'] == 'pass' and upper['status'] == 'fail'
            and lower['gc_activity_in_every_fork'] and upper['gc_activity_in_every_fork']
            and upper['rate'] - lower['rate'] <= max(1, lower['rate'] * policy['tolerance'])
            and bounds(points)['status'] == 'bounded')


def read_point_window(directory, point, plan_sha256):
    """Return an available execution window for this exact predeclared point."""
    try:
        record = read_json(directory / (point + '.json'))
    except FileNotFoundError:
        return None
    if not isinstance(record, dict):
        raise ValueError('measurement window must be an object')
    window = record.get('quiet_window_id')
    if (record.get('point') != point or record.get('plan_sha256') != plan_sha256
            or not isinstance(window, str) or not window.strip()):
        raise ValueError('measurement window belongs to another plan or point')
    return window


def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('directory',type=Path)
    p.add_argument('--wasmtime',type=Path,required=True);p.add_argument('--cgroup-parent',type=Path,required=True)
    p.add_argument('--quiet-window-id',required=True)
    p.add_argument('--point-windows', type=Path,
                   help='schedule each complete rate point in a separate quiet window on a shared host')
    a=p.parse_args();root=a.directory.resolve();plan=read_json(root/'plan.json')
    delivered = plan['policy'] == 'delivered-capacity'
    if (plan['schema_version'] != (2 if delivered else 1) or plan['scope'] != 'shared_heap_scheduled_v1'
            or plan['qualification'] is not False):raise ValueError('plan scope')
    if delivered and not plan['screen_limits']['minimum_completions'] <= plan['target_offers'] <= 86400:
        raise ValueError('invalid delivered-capacity offer target')
    if a.point_windows and (not delivered or not a.point_windows.is_dir()):
        raise ValueError('point windows require a delivered-capacity plan and an existing directory')
    plan_hash=digest(root/'plan.json')
    for name,sha in plan['admission_sha256'].items():
        if digest(root/'admissions'/name)!=sha:raise ValueError('admission changed')
    if digest(root/'resources.json')!=plan['resources_sha256'] or workload_digest(root/'classes')!=plan['workload_sha256']:raise ValueError('inputs changed')
    for family,b in plan['builds'].items():
        receipt=Path(b['directory'])/('peer-build.json' if family=='peer' else 'build.json')
        if digest(receipt)!=b['receipt_sha256']:raise ValueError('build receipt changed')
    output=root/'execution';output.mkdir(exist_ok=False);(output/'runs').mkdir()
    report={'schema_version':1,'qualification':False,'quiet_window_id':a.quiet_window_id,'plan_sha256':plan_hash,
            'started_unix':time.time(),'searches':[],'processes':[],'status':'running'}
    def save():
        # A point report can read this file between forks while the search is live.
        pending = output / 'results.json.tmp'
        pending.write_text(json.dumps(report, indent=2) + '\n')
        pending.replace(output / 'results.json')
    save()
    def trial(workload,runtime,rate,point_index,confirmation=False):
        results=[]
        seconds, watchdog = point_window(plan, rate)
        window = a.quiet_window_id
        if a.point_windows:
            point_id = f'{workload}-{runtime}-point{point_index:02d}-rate{rate}'
            report['waiting_for_window'] = point_id
            save()
            print('WAITING FOR WINDOW ' + point_id, flush=True)
            while (window := read_point_window(a.point_windows, point_id, plan_hash)) is None:
                time.sleep(.5)
            del report['waiting_for_window']
        for fork in range(plan['forks']):
            case=f'{workload}-{runtime}-point{point_index:02d}-rate{rate}-fork{fork}'
            dest=output/'runs'/case
            family='peer' if runtime in ('g1','zgc') else 'wasi' if runtime.startswith('wasi') else 'native'
            tier='compiled' if runtime=='native-compiled' else 'interpreted'
            command=[sys.executable,str(Path(__file__).with_name('run_ring_sample.py')),'--build',plan['builds'][family]['directory'],
                '--admission',str(root/'admissions'/f'{workload}-{tier}.json'),'--output',str(dest),'--mode',family,
                '--sample-mode','scheduled-measure','--warmup',str(plan['warmup']),'--duration-seconds',str(seconds),
                '--arrival-rate',str(rate),'--max-completions',str(plan['max_completions']),
                '--resources',str(root/'resources.json'),'--cgroup-parent',str(a.cgroup_parent),'--timeout',str(watchdog)]
            if family=='peer':command+=['--collector','G1' if runtime=='g1' else 'Z','--heap-bytes',str(64*1024*1024)]
            else:command+=['--classes',str(root/'classes')]
            if family=='wasi':command+=['--wasmtime',str(a.wasmtime)]
            row={'id':case,'workload':workload,'runtime':runtime,'rate':rate,'fork':fork,'command':command,'started_unix':time.time()}
            if a.point_windows:
                row['quiet_window_id'] = window
            report['processes'].append(row);save();print('START '+case,flush=True)
            with (output/f'{case}.log').open('w') as log:child=subprocess.run(command,stdout=log,stderr=subprocess.STDOUT)
            row.update(exit_code=child.returncode,ended_unix=time.time())
            try:
                if child.returncode:raise ValueError(f'child exit {child.returncode}')
                # Recheck actual packet and resource binding, not just cached checked.json.
                result=check_peer(dest) if family=='peer' else check_canonical(dest)
                if read_json(dest/'launch-plan.json')['watchdog_seconds']!=watchdog:raise ValueError('watchdog changed')
                if read_json(dest/'launch-plan.json')['resources']!=read_json(root/'resources.json'):raise ValueError('resource envelope changed')
                receipt=read_json(dest/'run.json')
                if (receipt['admission_sha256']!=plan['admission_sha256'][f'{workload}-{tier}.json']
                        or receipt['workload_sha256']!=plan['workload_sha256']):raise ValueError('consumed workload/admission differs')
                build_receipt=read_json(dest/('peer-build.json' if family=='peer' else 'build.json'))
                if build_receipt['source_manifest_sha256']!=plan['builds'][family]['source_manifest_sha256']:raise ValueError('consumed source differs')
                after=read_json(dest/'resource-after.json')
                events=dict(line.split() for line in after.get('memory.events','').splitlines())
                if not events or any(int(events.get(key,'0')) for key in ('oom','oom_kill')):raise ValueError('missing memory events or OOM')
                raw=read_json(dest/'sample-plan.json');validate_plan(raw)
                if (raw['arrival_rate']!=rate or raw['duration_seconds']!=seconds or raw['warmup_invocations']!=plan['warmup']
                        or raw['max_completions']!=plan['max_completions']):raise ValueError('actual scheduled input differs')
                result.update(fork=fork,run_directory=str(dest));results.append(result);row['valid']=True
            except (OSError,ValueError,TypeError,KeyError) as error:
                row.update(valid=False,error=str(error));save();report['status']='invalid';save();raise
            print('PASS '+case,flush=True);save()
        point=classify(results,plan['screen_limits'],rate,delivered);point.update(rate=rate,confirmation=confirmation);return point
    try:
        for workload in plan['workloads']:
            for runtime in plan['runtimes']:
                search={'workload':workload,'runtime':runtime,'points':[],'confirmed':False};report['searches'].append(search)
                if plan['policy']=='fixed-pilot':
                    for rate in plan['rates']:
                        search['points'].append(trial(workload,runtime,rate,len(search['points'])));save()
                elif plan['policy']=='adaptive' or delivered:
                    policy=plan['search']
                    for _ in range(policy['max_points']):
                        rate=next_rate(search['points'],policy['initial_rate'],policy['maximum'],policy['tolerance'])
                        if rate is None:break
                        search['points'].append(trial(workload,runtime,rate,len(search['points'])));save()
                    interval=bounds(search['points'])
                    if interval['status']=='bounded':
                        search['points'].append(trial(workload,runtime,interval['lower'],len(search['points']),True));save()
                        search['points'].append(trial(workload,runtime,interval['upper'],len(search['points']),True));save()
                        if delivered:
                            search['confirmed']=confirmed_interval(search['points'], policy)
                        else:
                            search['confirmed']=search['points'][-2]['status']=='pass' and search['points'][-1]['status']=='fail'
                else:raise ValueError('unknown capacity policy')
                search['bounds']=bounds(search['points']);save()
        report['status']='completed'
    except (ValueError, TypeError, KeyError, OSError, subprocess.SubprocessError) as error:
        report.update(status='invalid', error=str(error))
        raise
    finally:
        report['ended_unix']=time.time();report['plan_unchanged']=digest(root/'plan.json')==plan_hash;save()
    print(json.dumps({'status':report['status'],'processes':len(report['processes']),'qualification':False}),flush=True)

if __name__=='__main__':main()
