"""Invalid work accounting and incomplete pairs must not produce a favorable graph."""
import unittest
import shutil
import subprocess
import tempfile
from pathlib import Path
from mark_cycles import INVOCATION_NAME, invocation, summarize, validate


def evidence():
    return dict(schema=1, shape='tree', policy='gated', nodes=127, cycles=3, workers=0,
                diagnostics=True, marked=381, completed=3, checksum=8128,
                elapsed_ns=100000, trace_tasks=381, teardown_live=0, teardown_bytes=0)


class MarkCycleEvidenceTests(unittest.TestCase):
    @unittest.skipUnless(shutil.which('sh'), 'requires a native shell executable')
    def test_native_invocation_name_is_independent_of_executable_path(self):
        with tempfile.TemporaryDirectory() as directory:
            for name in ['short', 'long-path-' * 12]:
                executable = Path(directory) / name
                executable.symlink_to(shutil.which('sh'))
                command, selected = invocation(executable, ['-c', 'printf "%s" "$0"'])
                child = subprocess.run(command, executable=selected, capture_output=True,
                                       text=True, check=True)
                self.assertEqual(child.stdout, INVOCATION_NAME)

    def test_buffered_shape_requires_exact_records_and_no_overflow(self):
        for diagnostics in [False, True]:
            data = dict(evidence(), shape='barrier', diagnostics=diagnostics,
                        trace_tasks=381 if diagnostics else 0,
                        barrier_buffered=378 if diagnostics else 0,
                        barrier_flushed=378 if diagnostics else 0, barrier_overflows=0)
            validate(data, 'barrier', 'gated', 127, 3, 0, diagnostics)
            for field in ['barrier_buffered', 'barrier_flushed', 'barrier_overflows']:
                for value in [data[field] + 1, True, None]:
                    with self.assertRaises(ValueError, msg=field):
                        validate(dict(data, **{field: value}), 'barrier', 'gated', 127, 3, 0, diagnostics)
            with self.assertRaises(ValueError):
                validate(dict(data, workers=1), 'barrier', 'gated', 127, 3, 1, diagnostics)

    def test_exact_work_and_cleanup_are_required(self):
        validate(evidence(), 'tree', 'gated', 127, 3, 0, True)
        for field, value in [('marked', 380), ('completed', 2), ('checksum', 0), ('workers', 1),
                             ('teardown_live', 1), ('teardown_bytes', 1), ('elapsed_ns', 0),
                             ('elapsed_ns', float('nan')), ('trace_tasks', 380), ('schema', True),
                             ('diagnostics', 1)]:
            with self.assertRaises(ValueError, msg=field):
                validate(dict(evidence(), **{field: value}), 'tree', 'gated', 127, 3, 0, True)

    def test_mixed_requires_the_whole_garbage_cohort_and_its_bytes(self):
        data = dict(evidence(), shape='mixed', swept=93, freed=744)
        validate(data, 'mixed', 'gated', 127, 3, 0, True)
        for key in ('swept', 'freed'):
            with self.assertRaises(ValueError):
                validate(dict(data, **{key: data[key] - 1}), 'mixed', 'gated', 127, 3, 0, True)

    def test_disabled_diagnostics_must_have_no_tasks(self):
        data = dict(evidence(), diagnostics=False, trace_tasks=0)
        validate(data, 'tree', 'gated', 127, 3, 0, False)
        with self.assertRaises(ValueError):
            validate(dict(data, trace_tasks=1), 'tree', 'gated', 127, 3, 0, False)

    def test_failed_missing_or_duplicate_pairs_are_rejected(self):
        rows = [dict(repeat=r, shape='tree', policy='gated', workers=0, diagnostics=True,
                     binary=b, status='ok', measurement=evidence())
                for r in range(2) for b in ['before', 'after']]
        conditions = [('tree', 'gated', 0, True)]
        self.assertEqual(summarize(rows, conditions, 2, 42)[0]['after_before']['median'], 1)
        for corrupt in [rows[:-1], rows + [rows[0]], rows[:-1] + [dict(rows[-1], status='failed')]]:
            with self.assertRaises(ValueError):
                summarize(corrupt, conditions, 2, 42)


if __name__ == '__main__':
    unittest.main()
