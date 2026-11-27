import copy
import json
import unittest
from pathlib import Path
from tempfile import TemporaryDirectory

from graph_lifecycle import FIELDS, TIMES, checked, digest, invocation, make_schedule, packet, validate, validate_counts


def measurement():
    row = {key:1 for key in FIELDS}
    row.update(version=1, mode='canonical-graph', rounds=3, capacity=8, degree=2, objects=6,
               payload_bytes=24, logical_object_bytes=128, catalog_bytes=65536, poll_limit=1000000,
               allocation_polls=40, write_polls=20, read_polls=20, helper_polls=100,
               released_objects=6, released_roots=9, released_edges=12, drops=6, checksum=9,
               teardown_bytes=0, teardown_catalog_bytes=0, lifecycle_ns=10)
    return row


def counts():
    return [dict(version=1, mode='canonical-graph', rounds=3, capacity=8, sample=i,
                 allocations=12, allocated_bytes=4096, releases=12, released_bytes=4096,
                 live_before=32, live_after=32, peak_above_baseline=1024) for i in range(3)]


class GraphLifecycleTests(unittest.TestCase):
    def test_complete_workload_and_allocator_lifetimes(self):
        self.assertEqual(validate(measurement(),3,8),measurement())
        self.assertEqual(validate_counts(counts(),3,8),counts())

    def test_exact_schema_and_integer_types(self):
        for change in [lambda r:r.pop('drops'),lambda r:r.update(extra=0),lambda r:r.update(drops=True),
                       lambda r:r.update(read_ns=1.5),lambda r:r.update(catalog_bytes=-1)]:
            row=measurement();change(row)
            with self.assertRaises(ValueError):validate(row,3,8)

    def test_every_integrity_and_release_field_is_required(self):
        for key in ['version','rounds','capacity','degree','objects','payload_bytes','poll_limit',
                    'released_objects','released_roots','released_edges','drops','checksum',
                    'teardown_bytes','teardown_catalog_bytes']:
            row=measurement();row[key]+=1
            with self.subTest(key=key),self.assertRaises(ValueError):validate(row,3,8)

    def test_missing_operations_or_storage_cannot_pass(self):
        for key in ['logical_object_bytes','catalog_bytes','allocation_polls','write_polls','read_polls','helper_polls']:
            row=measurement();row[key]=0
            with self.subTest(key=key),self.assertRaises(ValueError):validate(row,3,8)

    def test_timing_fields_and_nested_boundaries(self):
        for key in TIMES:
            row=measurement();row[key]=0
            with self.subTest(key=key),self.assertRaises(ValueError):validate(row,3,8)
        row=measurement();row['lifecycle_ns']=3
        with self.assertRaises(ValueError):validate(row,3,8)

    def test_foreign_mode_and_duplicate_json_fields(self):
        row=measurement();row['mode']='legacy'
        with self.assertRaises(ValueError):validate(row,3,8)
        with self.assertRaises(ValueError):packet('{"drops":6,"drops":0}')

    def test_allocator_samples_require_complete_ordered_balanced_ownership(self):
        for index,key,value in [(0,'sample',1),(0,'releases',11),(0,'released_bytes',4095),
                                (0,'live_after',33),(1,'live_before',33),(0,'peak_above_baseline',0),
                                (0,'allocations',False),(0,'mode','legacy')]:
            rows=counts();rows[index][key]=value
            with self.subTest(key=key),self.assertRaises(ValueError):validate_counts(rows,3,8)
        with self.assertRaises(ValueError):validate_counts(counts()[:2],3,8)

    def test_argv_zero_is_fixed_for_both_targets(self):
        command,executable=invocation('/frozen/long-path/binary',3,8)
        self.assertEqual(command,['graph_lifecycle','3','8'])
        self.assertEqual(executable,'/frozen/long-path/binary')
        command,executable=invocation('/frozen/binary',3,8,'/tools/wasmtime')
        self.assertIn('--argv0',command)
        self.assertEqual(command[-3:],['/frozen/binary','3','8'])
        self.assertIsNone(executable)

    def test_schedule_has_exact_sequential_indices_and_one_case_per_repeat(self):
        rows=make_schedule([2,8,64],3,'timing',17)
        self.assertEqual([r['index'] for r in rows],list(range(9)))
        self.assertEqual({(r['repeat'],r['capacity']) for r in rows},{(n,c) for n in range(3) for c in [2,8,64]})
        self.assertEqual(rows,make_schedule([2,8,64],3,'timing',17))
        self.assertEqual(len(make_schedule([2,8,64],3,'counts',17)),3)

    def test_frozen_report_rejects_missing_failed_changed_or_substituted_processes(self):
        def fixture(directory):
            inputs={}
            for name,data in [('binary',b'\x7fELFfixture'),('sources.tar.gz',b'fixture source'),('graph_lifecycle.py',b'fixture runner'),('leaf_lifecycle.py',b'fixture helpers')]:
                path=directory/name;path.write_bytes(data);inputs[name]=dict(path=str(path),sha256=digest(path))
            path=directory/'build.json';path.write_text(json.dumps(dict(binary_sha256=inputs['binary']['sha256'],source_archive_sha256=inputs['sources.tar.gz']['sha256'])))
            inputs['build.json']=dict(path=str(path),sha256=digest(path))
            metadata=dict(version=1,target='native',kind='timing',qualification=False,rounds=3,capacities=[8],repeats=2,seed=17,inputs=inputs)
            schedule=make_schedule([8],2,'timing',17)
            rows=[]
            for case in schedule:
                command,executable=invocation(directory/'binary',3,8)
                rows.append(dict(**case,command=command,executable=executable,exit_code=0,timeout=False,valid=True,stdout=json.dumps(measurement()),measurement=measurement()))
            return metadata,schedule,rows
        def save(directory,metadata,schedule,rows):
            (directory/'metadata.json').write_text(json.dumps(metadata))
            (directory/'schedule.json').write_text(json.dumps(schedule))
            (directory/'raw.jsonl').write_text(''.join(json.dumps(r)+'\n' for r in rows))
            result=dict(processes=2,failures=[],inputs_unchanged=True,qualification=False,files={n:digest(directory/n) for n in ['metadata.json','schedule.json','raw.jsonl']})
            (directory/'result.json').write_text(json.dumps(result))
        mutations=[lambda m,s,r:r.pop(),lambda m,s,r:r[0].update(exit_code=1),lambda m,s,r:r[0].update(timeout=True),
                   lambda m,s,r:r[0].update(command=['other']),lambda m,s,r:r[0]['measurement'].update(cleanup_ns=2),
                   lambda m,s,r:r[1].update(repeat=0),lambda m,s,r:m.update(target='WASI'),
                   lambda m,s,r:s.reverse(),lambda m,s,r:m.update(repeats=True)]
        with TemporaryDirectory() as temporary:
            directory=Path(temporary);metadata,schedule,rows=fixture(directory);save(directory,metadata,schedule,rows)
            self.assertEqual(len(checked(directory)[1]),2)
            for mutation in mutations:
                m,s,r=copy.deepcopy((metadata,schedule,rows));mutation(m,s,r);save(directory,m,s,r)
                with self.assertRaises(ValueError):checked(directory)
            save(directory,metadata,schedule,rows)
            (directory/'raw.jsonl').write_text('changed after capture')
            with self.assertRaises(ValueError):checked(directory)


if __name__=='__main__':unittest.main()
