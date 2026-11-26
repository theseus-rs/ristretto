"""Reject false passes at the state-model evidence boundary."""
import unittest

from tagged_pool_model_check import valid_model_result


class TaggedPoolModelResultTests(unittest.TestCase):
    def test_positive_requires_all_declared_models(self):
        self.assertTrue(valid_model_result(0, "test result: ok. 6 passed; 0 failed;"))
        self.assertFalse(valid_model_result(0, "test result: ok. 0 passed; 0 failed;"))
        self.assertFalse(valid_model_result(0, "test result: ok. 5 passed; 0 failed;"))
        self.assertFalse(valid_model_result(101, "test result: ok. 6 passed; 0 failed;"))

    def test_negative_requires_an_executed_matching_failure(self):
        log = ("test invariant ... FAILED\nleft: 0\n"
               "test result: FAILED. 0 passed; 1 failed;")
        self.assertTrue(valid_model_result(101, log, "invariant", "left: 0"))
        self.assertFalse(valid_model_result(0, log, "invariant", "left: 0"))
        self.assertFalse(valid_model_result(101, log, "different_test", "left: 0"))
        self.assertFalse(valid_model_result(101, log, "invariant", "left: 99"))

    def test_compile_error_cannot_satisfy_a_negative_control(self):
        log = "error: could not compile; source includes left: 0"
        self.assertFalse(valid_model_result(101, log, "invariant", "left: 0"))

    def test_successful_fault_variant_cannot_satisfy_negative_control(self):
        log = ("left: 0\ntest invariant ... ok\n"
               "test result: ok. 1 passed; 0 failed;")
        self.assertFalse(valid_model_result(0, log, "invariant", "left: 0"))


if __name__ == "__main__":
    unittest.main()
