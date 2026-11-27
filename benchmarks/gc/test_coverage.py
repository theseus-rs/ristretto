import copy
import unittest

from coverage import CORE, NODE, coverage, interval_profile


def fixture():
    methods = []
    for index, (owner, name, descriptor) in enumerate(CORE + [NODE], 1):
        methods.append({"instance": index, "class": owner, "name": name, "descriptor": descriptor,
                        "compiled_entries": 0, "interpreter_entries": 1000, "intrinsic_entries": 0,
                        "queued_requests": 0, "compilation_attempts": 0, "compilation_successes": 0,
                        "compilation_failures": 0, "compilation_ns": 0, "generated_code_bytes": 0,
                        "rejection_reason": "unqualified_instruction", "compilation_state": "rejected"})
    return {"schema_version": 1, "diagnostic_only": True, "qualification": False, "methods": methods}


class CoverageTests(unittest.TestCase):
    def test_missing_historical_backedges_are_not_invented(self):
        result = coverage(fixture(), "roots", 10)
        self.assertNotIn("compiled_backedges", result["required_methods"][0])
        self.assertNotIn("compiled_resumes", result["required_methods"][0])
        for value in [-1, True, 1]:
            profile = fixture()
            profile["methods"][0]["compiled_backedges"] = value
            with self.assertRaises(ValueError):
                coverage(profile, "roots", 10)

    def test_resumes_do_not_count_as_new_invocations(self):
        profile = fixture()
        profile["methods"][0].update(compiled_entries=1, compiled_resumes=100,
                                     compilation_successes=1, interpreter_entries=0)
        result = coverage(profile, "roots", 10)
        self.assertEqual(result["required_methods"][0]["compiled_entries"], 1)
        self.assertEqual(result["required_methods"][0]["compiled_resumes"], 100)
        for value in [-1, True]:
            profile["methods"][0]["compiled_resumes"] = value
            with self.assertRaises(ValueError):
                coverage(profile, "roots", 10)

    def test_interval_allows_prior_compilation_and_already_entered_loops(self):
        method = fixture()["methods"][0]
        method.update(compiled_entries=0, compiled_backedges=10, compiled_resumes=2, interpreter_entries=0,
                      present_at_start=True, compiled_before_start=True,
                      code_bytes_at_start=100, code_bytes_at_end=100)
        interval = {"schema_version": 1, "diagnostic_only": True, "qualification": False,
                    "scope": "checkpoint_interval", "label": "test", "methods": [method],
                    "boundary_span_ns": 10, "start_snapshot_ns": 2, "end_snapshot_ns": 3}
        self.assertEqual(interval_profile(interval), interval)
        self.assertFalse(interval_profile(interval)["qualification"])
        # Bare Wasm counters need no host clock.
        clockless = dict(interval, boundary_span_ns=None, start_snapshot_ns=None, end_snapshot_ns=None)
        self.assertEqual(interval_profile(clockless), clockless)
        for mutate in [lambda p: p.update(scope="measurement"),
                       lambda p: p.update(boundary_span_ns=4),
                       lambda p: p.update(start_snapshot_ns=None),
                       lambda p: p["methods"][0].update(code_bytes_at_end=99),
                       lambda p: p["methods"][0].update(present_at_start=False),
                       lambda p: p["methods"][0].update(compiled_backedges=True),
                       lambda p: p["methods"][0].update(compiled_resumes=-1)]:
            changed = copy.deepcopy(interval)
            mutate(changed)
            with self.assertRaises(ValueError):
                interval_profile(changed)

    def test_compilation_and_lifetime_entries_do_not_qualify_coverage(self):
        profile = fixture()
        profile["methods"][0].update(compilation_successes=1, compilation_attempts=1, compilation_state="compiled")
        result = coverage(profile, "roots", 10)
        self.assertEqual(result["required_methods"][0]["status"], "no_compiled_entry")
        profile["methods"][0]["compiled_entries"] = 1
        result = coverage(profile, "roots", 10)
        self.assertEqual(result["required_methods"][0]["status"], "compiled_observed")
        self.assertFalse(result["coverage_gate_passed"])
        self.assertFalse(result["qualification"])

    def test_missing_methods_remain_visible(self):
        profile = fixture()
        profile["methods"] = []
        result = coverage(profile, "roots", 10)
        self.assertEqual(len(result["required_methods"]), 4)
        self.assertTrue(all(r["status"] == "missing" for r in result["required_methods"]))

    def test_corrupt_counts_ambiguous_identity_and_wrong_build_are_rejected(self):
        for mutate in [lambda p: p.update(diagnostic_only=False),
                       lambda p: p["methods"][0].update(compiled_entries=1),
                       lambda p: p["methods"][0].update(interpreter_entries=-1),
                       lambda p: p["methods"][0].update(interpreter_entries=True),
                       lambda p: p["methods"][1].update(interpreter_entries=9),
                       lambda p: p["methods"].append(copy.deepcopy(p["methods"][0])),
                       lambda p: p["methods"].append({**p["methods"][0], "instance": 99})]:
            profile = fixture()
            mutate(profile)
            with self.assertRaises(ValueError):
                coverage(profile, "roots", 10)


if __name__ == "__main__":
    unittest.main()
