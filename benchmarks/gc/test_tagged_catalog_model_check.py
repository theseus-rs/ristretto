"""Reject incomplete/fabricated evidence and partially applied fault mutations."""
from pathlib import Path
import tempfile
import unittest

from tagged_catalog_model_check import (
    CATALOG_MODELS, FAULTS, apply_changes, standalone_manifest, valid_model_result,
)


def passed(count=CATALOG_MODELS, **values):
    fields = dict(passed=count, failed=0, ignored=0, measured=0, filtered=0)
    fields.update(values)
    return (f'running {count} tests\n'
            'test result: ok. {passed} passed; {failed} failed; {ignored} ignored; '
            '{measured} measured; {filtered} filtered out; finished in 0.01s\n').format(**fields)


def failed():
    return ('running 1 test\ntest protocol ... FAILED\nleft: Retry\n'
            'test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; '
            '54 filtered out; finished in 0.01s\n')


class TaggedCatalogModelResultTests(unittest.TestCase):
    def test_positive_requires_the_complete_declared_suite(self):
        self.assertTrue(valid_model_result(0, passed()))
        self.assertTrue(valid_model_result(0, passed(6), expected=6))
        self.assertFalse(valid_model_result(0, passed(20)))
        self.assertFalse(valid_model_result(0, passed(CATALOG_MODELS - 1)))
        self.assertFalse(valid_model_result(101, passed()))
        self.assertFalse(valid_model_result(0, passed().replace(f'running {CATALOG_MODELS}', f'running {CATALOG_MODELS - 1}')))

    def test_ignored_filtered_failed_or_measured_positive_cannot_qualify(self):
        for field in ['ignored', 'filtered', 'failed', 'measured']:
            with self.subTest(field=field):
                self.assertFalse(valid_model_result(0, passed(**{field: 1})))

    def test_duplicate_summaries_and_missing_execution_header_are_rejected(self):
        self.assertFalse(valid_model_result(0, passed() + passed()))
        self.assertFalse(valid_model_result(0, passed().split('\n', 1)[1]))
        self.assertFalse(valid_model_result(0, f'running {CATALOG_MODELS} tests\n' + passed()))
        self.assertFalse(valid_model_result(101, failed() + failed(), 'protocol', 'left: Retry'))

    def test_negative_requires_matching_executed_test_and_witness(self):
        self.assertTrue(valid_model_result(101, failed(), 'protocol', 'left: Retry'))
        self.assertFalse(valid_model_result(101, failed(), 'other', 'left: Retry'))
        self.assertFalse(valid_model_result(101, failed(), 'protocol', 'left: Committed'))
        self.assertFalse(valid_model_result(101, failed(), 'protocol', None))

    def test_compiler_diagnostics_quoting_test_output_cannot_pass(self):
        quoted = '\n'.join('  | ' + line for line in failed().splitlines())
        self.assertFalse(valid_model_result(101, quoted, 'protocol', 'left: Retry'))
        quoted = '\n'.join('  | ' + line for line in passed().splitlines())
        self.assertFalse(valid_model_result(0, quoted))
        self.assertFalse(valid_model_result(101, 'could not compile: left: Retry',
                                            'protocol', 'left: Retry'))

    def test_success_timeout_wrong_count_or_ignored_negative_cannot_pass(self):
        for code in [0, -9, 1]:
            self.assertFalse(valid_model_result(code, failed(), 'protocol', 'left: Retry'))
        for old, new in [('running 1 test', 'running 2 tests'), ('0 ignored', '1 ignored'),
                         ('0 passed', '1 passed'), ('1 failed', '2 failed')]:
            self.assertFalse(valid_model_result(101, failed().replace(old, new),
                                                'protocol', 'left: Retry'))

    def test_failed_name_must_be_an_executed_line(self):
        log = failed().replace('test protocol ... FAILED', '  | test protocol ... FAILED')
        self.assertFalse(valid_model_result(101, log, 'protocol', 'left: Retry'))

    def test_multi_file_fault_preflight_is_atomic_on_a_stale_second_anchor(self):
        with tempfile.TemporaryDirectory() as temporary:
            source = Path(temporary)
            (source / 'first.rs').write_text('first')
            (source / 'second.rs').write_text('second')
            changes = [('first.rs', 'first', 'changed'), ('second.rs', 'stale', 'changed')]
            with self.assertRaises(ValueError):
                apply_changes(source, changes, write=True)
            self.assertEqual((source / 'first.rs').read_text(), 'first')
            self.assertEqual((source / 'second.rs').read_text(), 'second')

    def test_preflight_does_not_mutate_and_refuses_ambiguous_anchors(self):
        with tempfile.TemporaryDirectory() as temporary:
            source = Path(temporary)
            file = source / 'state.rs'
            file.write_text('before\nbefore')
            with self.assertRaises(ValueError):
                apply_changes(source, [('state.rs', 'before', 'after')], write=True)
            self.assertEqual(file.read_text(), 'before\nbefore')
            file.write_text('before')
            changes = [('state.rs', 'before', 'middle'), ('state.rs', 'middle', 'after')]
            self.assertEqual(apply_changes(source, changes), {'state.rs': 'after'})
            self.assertEqual(file.read_text(), 'before')
            apply_changes(source, changes, write=True)
            self.assertEqual(file.read_text(), 'after')

    def test_copied_crate_can_detach_from_parent_workspace_without_duplicate_tables(self):
        with tempfile.TemporaryDirectory() as temporary:
            manifest = Path(temporary) / 'Cargo.toml'
            for workspace in ['', '[workspace]\n\n']:
                manifest.write_text(workspace + '[package]\nname = "fixture"\n')
                standalone_manifest(manifest, Path('/recorded/checker'))
                content = manifest.read_text()
                self.assertEqual(content.count('[workspace]'), 1)
                self.assertIn('loom = { path = "/recorded/checker" }', content)
            with self.assertRaises(ValueError):
                standalone_manifest(manifest, Path('/other/checker'))

    def test_current_source_preflight_covers_every_declared_fault(self):
        root = Path(__file__).resolve().parents[2] / 'ristretto_gc/validation/catalog'
        with tempfile.TemporaryDirectory() as temporary:
            import shutil
            source = Path(temporary)
            for short, name in [('catalog', 'catalog-tagged-prototype'),
                                ('pool', 'catalog-tagged-pool-prototype')]:
                shutil.copytree(root / name / 'src', source / short / 'src')
            for name, _, _, changes, _, _ in FAULTS:
                with self.subTest(name=name):
                    apply_changes(source, changes)


if __name__ == '__main__':
    unittest.main()
