import unittest
from catalog_accounting_check import HEADER, valid_audit_result


class CatalogAccountingResultTests(unittest.TestCase):
    def positive(self):
        return 'Running `accounting`\n' + HEADER + '\n' + '\n'.join(
            f'{n},{m},1,4096,512,4608,9,true' for n in [1, 64, 1024, 16384] for m in [0, 1, 7])

    def test_requires_complete_successful_audit(self):
        log = self.positive()
        self.assertTrue(valid_audit_result(0, log, 'native'))
        self.assertFalse(valid_audit_result(101, log, 'native'))
        self.assertFalse(valid_audit_result(0, '\n'.join(log.splitlines()[:-1]), 'native'))

    def test_rejects_inconsistent_accounting_and_early_refund(self):
        for old, new in [('4608', '4607'), ('true', 'false'), (',9,', ',8,')]:
            self.assertFalse(valid_audit_result(0, self.positive().replace(old, new), 'native'))

    def test_negative_requires_execution_and_target_failure(self):
        log = 'Running `accounting`\ncredit returned before final backing free'
        self.assertTrue(valid_audit_result(101, log, 'native', 'credit returned before final backing free'))
        self.assertTrue(valid_audit_result(134, log, 'wasi', 'credit returned before final backing free'))
        for code in [0, -9, 1]:
            self.assertFalse(valid_audit_result(code, log, 'native', 'credit returned before final backing free'))

    def test_compilation_failure_or_wrong_witness_is_not_a_passing_fault(self):
        witness = 'computed backing differs from allocator requests'
        self.assertFalse(valid_audit_result(101, 'could not compile: ' + witness, 'native', witness))
        self.assertFalse(valid_audit_result(101, 'Running `accounting`\nother failure', 'native', witness))


if __name__ == '__main__':
    unittest.main()
