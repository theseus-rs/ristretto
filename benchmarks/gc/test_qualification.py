"""Synthetic evidence tests: these fixtures are not measured qualification results."""
import copy
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

from acceptance import (GLOBAL_SOURCES, canonical_hash, resource_limits,
                        validate_pauses, validate_policy)
from qualify import digest, evaluate
from report import expected_checksum
from run import freeze_acceptance_policy, qualification_observation

CANDIDATE = "ristretto-candidate-jit"


def policy():
    return {
        "version": 2, "candidate": CANDIDATE,
        "resources": {"heap_mib": 64, "memory_bytes": 512 * 1024**2,
                      "cpu_quota_us": 200000, "cpu_period_us": 100000, "cpus": [0, 1]},
        "workloads": {"allocation": {
            "response_p999_ns": 1_000_000, "response_p9999_ns": 2_000_000,
            "peak_backlog": 100,
            "required_methods": [
                {"class": "GcBench", "name": "main", "descriptor": "([Ljava/lang/String;)V",
                 "min_entries": 1, "entries_per_operation": 0, "backedges_per_operation": 1},
                {"class": "GcBench", "name": "operation", "descriptor": "(IJI)I",
                 "min_entries": 1, "entries_per_operation": 1, "backedges_per_operation": 0}]}},
        "coordination_audit": {
            "recorder": "synthetic-test-recorder", "binary_sha256": "0" * 64,
            "source_archive_sha256": "1" * 64, "coverage_complete": True,
            "unbounded_global_blocking": False, "review_reference": "SYNTHETIC TEST ONLY",
            "global_stop_sources": sorted(GLOBAL_SOURCES)}}


def packet(run_id, policy_sha, rules, count, elapsed):
    methods = []
    for instance, required in enumerate(rules, 1):
        methods.append({**{k: required[k] for k in ["class", "name", "descriptor"]},
                        "instance": instance,
                        "compiled_entries": max(required["min_entries"], count * required["entries_per_operation"]),
                        "compiled_backedges": count * required["backedges_per_operation"],
                        "interpreter_entries": 0, "intrinsic_entries": 0})
    end = 2 * 10**9 + elapsed
    return {
        "schema_version": 1, "run_id": run_id, "policy_sha256": policy_sha,
        "window": {"clock": "process_monotonic_ns", "process_start_ns": 0,
                   "measurement_start_ns": 10**9, "measurement_end_ns": 10**9 + elapsed,
                   "process_end_ns": end},
        "pauses": {"schema_version": 1, "scope": "whole_process",
                   "recorder": "synthetic-test-recorder", "capture_complete": True,
                   "global_stop_sources": sorted(GLOBAL_SOURCES), "dropped_events": 0,
                   "open_events": 0, "clock_resolution_ns": 1,
                   "capture_start_ns": 0, "capture_end_ns": end, "events": []},
        "execution": {"schema_version": 1, "scope": "measurement", "mode": "jit",
                      "ready_before_measurement": True, "manifest_sha256": canonical_hash(rules),
                      "unexpected_fallbacks": 0, "generated_code_bytes": 512,
                      "compilation_ns": 0, "methods": methods},
        "gc_exposure": {"scope": "measurement", "collections_completed": 2}}


def fixture(path):
    """Intentionally slower candidate: v2 absolute gates pass; legacy peer gates fail."""
    for name, content in [("gc_compare", b"synthetic-binary"),
                          ("gc_compare.sources.tar.gz", b"synthetic-build-sources"),
                          ("sources.tar.gz", b"synthetic-runner-sources")]:
        (path / name).write_bytes(content)
    build = {"binary_sha256": digest(path / "gc_compare"),
             "source_archive_sha256": digest(path / "gc_compare.sources.tar.gz")}
    (path / "gc_compare.provenance.json").write_text(json.dumps(build))
    plan = policy()
    plan["coordination_audit"].update(build)
    options = {"workloads": ["allocation"], "rates": [0, 1_000_000], "repeats": 5,
               "seconds": 1, "seed": 42, "heap_mib": 64, "rss_mib": 512,
               "cpu_quota": 2, "cpus": [0, 1], "runtimes": [CANDIDATE, "g1", "zgc"]}
    original = path / "declared.json"
    original.write_text(json.dumps(plan))
    declaration = freeze_acceptance_policy(original, path, options)
    artifacts = {p.name: digest(p) for p in path.iterdir() if p.name != "declared.json"}
    metadata = {"options": options, "acceptance_policy": declaration,
                "artifacts_sha256": artifacts, "source_archive_sha256": digest(path / "sources.tar.gz")}
    (path / "metadata.json").write_text(json.dumps(metadata))
    (path / "integrity.json").write_text('{"inputs_unchanged":true}')
    rows = []
    for repeat in range(5):
        for rate in options["rates"]:
            for runtime in options["runtimes"]:
                count = 10_000_000 if rate == 0 and runtime != CANDIDATE else 1_000_000
                us = 500 if runtime == CANDIDATE else 1
                hist = {"max_ns": us * 1000, "buckets_us": [[us, count]]}
                data = {"schema": 1, "workload": "allocation", "rate": rate, "seed": 42,
                        "completed": count, "offered": rate, "unfinished": 0,
                        "duration_ns": 10**9, "elapsed_ns": 10**9,
                        "checksum": expected_checksum("allocation", count),
                        "service": copy.deepcopy(hist), "response": hist, "peak_backlog": 1}
                run_id = f"{repeat}-{rate}-{runtime}"
                row = {"runtime": runtime, "workload": "allocation", "rate": rate, "repeat": repeat,
                       "status": "ok", "exit_code": 0, "target": "native", "directory": "run-" + run_id,
                       "run_id": run_id, "measurement": data,
                       "cgroup": {"limits": resource_limits(plan), "memory_peak": 1024**2,
                                  "memory_events": {"oom": 0, "oom_kill": 0}},
                       "gc_observation": {"ristretto": {"jit_enabled": True}}}
                row["qualification_evidence"] = packet(run_id, declaration["sha256"],
                    plan["workloads"]["allocation"]["required_methods"], count, data["elapsed_ns"])
                rows.append(row)
    (path / "schedule.json").write_text(json.dumps([
        [r["repeat"], r["workload"], r["rate"], r["runtime"]] for r in rows]))
    write_rows(path, rows)
    return rows, plan


def write_rows(path, rows):
    for row in rows:
        output = path / row["directory"]
        output.mkdir(exist_ok=True)
        contents = "GC_RESULT " + json.dumps(row["measurement"]) + "\n"
        if "qualification_evidence" in row:
            contents += "GC_QUALIFICATION " + json.dumps(row["qualification_evidence"]) + "\n"
        if row["runtime"].startswith("ristretto-"):
            contents += "RISTRETTO_GC " + json.dumps(row["gc_observation"]["ristretto"]) + "\n"
            if row["gc_observation"].get("execution_profile"):
                contents += "RISTRETTO_EXECUTION {}\n"
        (output / "stdout.txt").write_text(contents)
        (output / "stderr.txt").write_text("")
        row["output_sha256"] = {name: digest(output / name) for name in ["stdout.txt", "stderr.txt"]}
    (path / "raw.jsonl").write_text("".join(json.dumps(row) + "\n" for row in rows))


class QualificationPolicyTests(unittest.TestCase):
    def test_peer_ratios_are_diagnostic_while_absolute_evidence_is_required(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp)
            fixture(path)
            result = evaluate(path)
            self.assertTrue(result["short_comparison_gate"], result["failures"])
            self.assertFalse(result["qualification"])
            self.assertEqual(result["policy_version"], 2)
            self.assertTrue(all("pass" not in r and "threshold" not in r for r in result["results"]))
            self.assertTrue(any(r["candidate_peer_ratio"]["median"] == .1 for r in result["results"]))
            self.assertTrue(any(r["candidate_peer_ratio"]["median"] == 500 for r in result["results"]))
            old = evaluate(path, CANDIDATE, policy_version=1)
            self.assertFalse(old["short_comparison_gate"])
            self.assertTrue(old["historical_only"])
            self.assertEqual({r["threshold"] for r in old["results"]}, {1.1, .8, 1.05})

    def test_missing_telemetry_and_lifetime_coverage_never_mean_zero_stw(self):
        changes = [
            lambda r: r.pop("qualification_evidence"),
            lambda r: r["qualification_evidence"].pop("pauses"),
            lambda r: r["qualification_evidence"]["pauses"].update(capture_complete=False),
            lambda r: r["qualification_evidence"]["execution"].update(scope="whole process"),
            lambda r: r["qualification_evidence"]["execution"].update(scope="checkpoint_interval"),
            lambda r: r["qualification_evidence"]["execution"].update(ready_before_measurement=False),
            lambda r: r["qualification_evidence"].update(gc_exposure={"scope": "whole process", "collections_completed": 10}),
            lambda r: r["gc_observation"].update(execution_profile=True),
            lambda r: r["gc_observation"]["ristretto"].update(jit_enabled=False),
        ]
        for change in changes:
            with self.subTest(change=change), tempfile.TemporaryDirectory() as tmp:
                path = Path(tmp); rows, _ = fixture(path)
                change(rows[0]); write_rows(path, rows)
                result = evaluate(path)
                self.assertFalse(result["short_comparison_gate"])
                self.assertTrue(result["failures"])

    def test_pause_boundary_is_exclusive_and_rendezvous_is_included(self):
        for duration, passed in [(999_999, True), (1_000_000, False), (1_000_001, False)]:
            with self.subTest(duration=duration), tempfile.TemporaryDirectory() as tmp:
                path = Path(tmp); rows, _ = fixture(path)
                rows[0]["qualification_evidence"]["pauses"]["events"] = [
                    {"source": "publication", "requested_ns": 100,
                     "mutators_stopped_ns": 100 + duration - 1, "released_ns": 100 + duration}]
                write_rows(path, rows)
                result = evaluate(path)
                self.assertEqual(result["short_comparison_gate"], passed, result["failures"])
                if passed:
                    pauses = result["processes"][0]["pauses"]
                    self.assertEqual(pauses["max_ns"], duration)
                    self.assertEqual(pauses["event_rendezvous_ns"], duration - 1)

    def test_overlapping_global_stops_cannot_hide_a_long_continuous_stop(self):
        p = packet("id", "a" * 64, [], 1, 10**9)["pauses"]
        p["events"] = [{"source": "publication", "requested_ns": 0,
                        "mutators_stopped_ns": 100, "released_ns": 700_000},
                       {"source": "gc_safepoint", "requested_ns": 600_000,
                        "mutators_stopped_ns": 610_000, "released_ns": 1_200_000}]
        with self.assertRaisesRegex(ValueError, "not below"):
            validate_pauses(p, policy()["coordination_audit"], (0, 3 * 10**9))
        p["events"][1]["released_ns"] = 900_000
        result = validate_pauses(p, policy()["coordination_audit"], (0, 3 * 10**9))
        self.assertEqual((result["event_count"], result["stop_count"], result["total_stopped_ns"]), (2, 1, 900_000))

    def test_zero_stops_require_complete_audit_and_capture_and_have_no_percentiles(self):
        pauses = packet("id", "a" * 64, [], 1, 10**9)["pauses"]
        audit = policy()["coordination_audit"]
        result = validate_pauses(pauses, audit, (0, 3 * 10**9))
        self.assertTrue(result["zero_observed_stops"])
        self.assertIsNone(result["percentiles_ns"])
        for changed in [None, dict(audit, coverage_complete=False), dict(audit, unbounded_global_blocking=True)]:
            with self.assertRaises(ValueError):
                validate_pauses(pauses, changed, (0, 3 * 10**9))
        for field, value in [("dropped_events", 1), ("open_events", 1), ("capture_start_ns", False),
                             ("capture_end_ns", 1), ("clock_resolution_ns", 1_000_000),
                             ("global_stop_sources", ["gc_safepoint"]), ("events", None),
                             ("dropped_events", True), ("capture_complete", 1)]:
            with self.subTest(field=field), self.assertRaises(ValueError):
                validate_pauses(dict(pauses, **{field: value}), audit, (0, 3 * 10**9))

    def test_compiled_entries_backedges_and_process_window_must_account_for_work(self):
        changes = [
            lambda p: p["execution"]["methods"][0].update(compiled_backedges=0),
            lambda p: p["execution"]["methods"][1].update(compiled_entries=1),
            lambda p: p["execution"]["methods"][1].update(interpreter_entries=1),
            lambda p: p["execution"]["methods"].append(copy.deepcopy(p["execution"]["methods"][0])),
            lambda p: p["execution"]["methods"][1].update(instance=1),
            lambda p: p["execution"].update(unexpected_fallbacks=1),
            lambda p: p["execution"].update(manifest_sha256="0" * 64),
            lambda p: p["window"].update(measurement_end_ns=2 * 10**9 + 1),
            lambda p: p.update(run_id="a different process"),
            lambda p: p.update(policy_sha256="0" * 64),
            lambda p: p.update(window=None),
            lambda p: p.update(gc_exposure=[]),
            lambda p: p["execution"]["methods"].append(None),
        ]
        for change in changes:
            with self.subTest(change=change), tempfile.TemporaryDirectory() as tmp:
                path = Path(tmp); rows, _ = fixture(path)
                change(rows[0]["qualification_evidence"]); write_rows(path, rows)
                self.assertFalse(evaluate(path)["short_comparison_gate"])

    def test_absolute_limits_resource_bounds_and_failed_pairs_still_fail(self):
        changes = [
            lambda rows: rows[0].update(status="timeout"),
            lambda rows: rows[0].update(exit_code=1),
            lambda rows: rows.append(copy.deepcopy(rows[0])),
            lambda rows: rows.pop(),
            lambda rows: rows[0]["cgroup"]["memory_events"].update(oom_kill=1),
            lambda rows: rows[0]["cgroup"]["limits"].update(**{"cpu.max": "300000 100000"}),
            lambda rows: rows[0]["cgroup"].update(memory_peak=1024**3),
            lambda rows: rows[3]["measurement"].update(peak_backlog=101),
            lambda rows: rows[3]["measurement"]["response"].update(buckets_us=[[2001, 1_000_000]], max_ns=2_001_000),
        ]
        for change in changes:
            with self.subTest(change=change), tempfile.TemporaryDirectory() as tmp:
                path = Path(tmp); rows, _ = fixture(path)
                change(rows); write_rows(path, rows)
                self.assertFalse(evaluate(path)["short_comparison_gate"])

    def test_policy_is_frozen_and_old_evidence_is_not_reclassified(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp); fixture(path)
            original = (path / "acceptance-policy.json").read_bytes()
            (path / "declared.json").write_text("{}")
            self.assertEqual((path / "acceptance-policy.json").read_bytes(), original)
            (path / "acceptance-policy.json").write_text("{}")
            self.assertFalse(evaluate(path)["short_comparison_gate"])
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp); fixture(path)
            metadata = json.loads((path / "metadata.json").read_text())
            metadata.pop("acceptance_policy")
            (path / "metadata.json").write_text(json.dumps(metadata))
            result = evaluate(path, CANDIDATE)
            self.assertFalse(result["short_comparison_gate"])
            self.assertTrue(any("not reclassified" in reason for reason in result["failures"]))

    def test_audit_build_and_frozen_artifacts_must_match(self):
        for name in ["gc_compare", "gc_compare.provenance.json", "gc_compare.sources.tar.gz", "sources.tar.gz"]:
            with self.subTest(name=name), tempfile.TemporaryDirectory() as tmp:
                path = Path(tmp); fixture(path)
                (path / name).write_bytes(b"changed")
                self.assertFalse(evaluate(path)["short_comparison_gate"])

    def test_unbacked_packets_and_changed_output_cannot_qualify(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp); rows, _ = fixture(path)
            (path / rows[0]["directory"] / "stdout.txt").write_text("changed output\n")
            self.assertFalse(evaluate(path)["short_comparison_gate"])
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp); rows, _ = fixture(path)
            rows[0]["qualification_evidence"]["pauses"]["open_events"] = 1
            (path / "raw.jsonl").write_text("".join(json.dumps(r) + "\n" for r in rows))
            result = evaluate(path)
            self.assertFalse(result["short_comparison_gate"])
            self.assertTrue(any("differs from captured" in e for e in result["failures"]))
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp); rows, _ = fixture(path)
            # JSON booleans and numbers compare equal in Python but are different evidence.
            original = copy.deepcopy(rows)
            rows[0]["qualification_evidence"]["schema_version"] = True
            write_rows(path, rows)
            original[0]["output_sha256"] = rows[0]["output_sha256"]
            (path / "raw.jsonl").write_text("".join(json.dumps(r) + "\n" for r in original))
            result = evaluate(path)
            self.assertFalse(result["short_comparison_gate"])
            self.assertTrue(any("differs from captured" in e for e in result["failures"]))

    def test_audit_cannot_be_reused_for_a_different_native_build(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp); rows, plan = fixture(path)
            plan["coordination_audit"]["binary_sha256"] = "f" * 64
            (path / "acceptance-policy.json").write_text(json.dumps(plan))
            policy_sha = digest(path / "acceptance-policy.json")
            metadata = json.loads((path / "metadata.json").read_text())
            metadata["acceptance_policy"]["sha256"] = policy_sha
            metadata["artifacts_sha256"]["acceptance-policy.json"] = policy_sha
            (path / "metadata.json").write_text(json.dumps(metadata))
            for row in rows:
                row["qualification_evidence"]["policy_sha256"] = policy_sha
            write_rows(path, rows)
            result = evaluate(path)
            self.assertFalse(result["short_comparison_gate"])
            self.assertTrue(any("audit does not match" in e for e in result["failures"]))

    def test_peer_gc_exposure_is_required_but_candidate_pause_budget_is_not_a_peer_gate(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp); rows, _ = fixture(path)
            rows[1]["qualification_evidence"]["pauses"]["events"] = [
                {"source": "gc_safepoint", "requested_ns": 0,
                 "mutators_stopped_ns": 10_000_000, "released_ns": 20_000_000}]
            write_rows(path, rows)
            self.assertTrue(evaluate(path)["short_comparison_gate"])
            rows[1]["qualification_evidence"].pop("gc_exposure")
            write_rows(path, rows)
            self.assertFalse(evaluate(path)["short_comparison_gate"])

    def test_policy_rejects_implicit_or_peer_relative_limits(self):
        for mutate in [lambda p: p.update(throughput_ratio=1.1),
                       lambda p: p.update(version=1), lambda p: p.update(candidate="ristretto-candidate"),
                       lambda p: p["workloads"]["allocation"].update(required_methods=[]),
                       lambda p: p["workloads"]["allocation"].update(response_p999_ns=float("nan")),
                       lambda p: p["resources"].update(cpu_quota_us=True)]:
            p = policy(); mutate(p)
            with self.assertRaises(ValueError):
                validate_policy(p)

    def test_invalid_pause_order_and_adjacent_stops_are_rejected(self):
        pauses = packet("id", "a" * 64, [], 1, 10**9)["pauses"]
        audit = policy()["coordination_audit"]
        for event in [
            {"source": [], "requested_ns": 0, "mutators_stopped_ns": 0, "released_ns": 1},
            {"source": "publication", "requested_ns": 10, "mutators_stopped_ns": 0, "released_ns": 11},
            {"source": "publication", "requested_ns": 0, "mutators_stopped_ns": 10, "released_ns": 9},
            {"source": "publication", "requested_ns": 3 * 10**9, "mutators_stopped_ns": 3 * 10**9,
             "released_ns": 3 * 10**9 + 1},
        ]:
            pauses["events"] = [event]
            with self.subTest(event=event), self.assertRaises(ValueError):
                validate_pauses(pauses, audit, (0, 3 * 10**9))
        pauses["events"] = [
            {"source": "publication", "requested_ns": 0, "mutators_stopped_ns": 0, "released_ns": 500_000},
            {"source": "publication", "requested_ns": 500_000, "mutators_stopped_ns": 500_000, "released_ns": 1_000_000},
        ]
        with self.assertRaisesRegex(ValueError, "not below"):
            validate_pauses(pauses, audit, (0, 3 * 10**9))

    def test_zero_resolution_ratios_are_unavailable_not_improvements(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp); rows, _ = fixture(path)
            for row in rows:
                row["measurement"]["response"] = {"max_ns": 0, "buckets_us": [[0, row["measurement"]["completed"]]]}
                row["measurement"]["service"] = copy.deepcopy(row["measurement"]["response"])
            write_rows(path, rows)
            result = evaluate(path)
            self.assertTrue(result["short_comparison_gate"], result["failures"])
            tails = [r for r in result["results"] if r["metric"] != "throughput"]
            self.assertTrue(all(r["candidate_peer_ratio"] is None for r in tails))
            self.assertTrue(all("resolution-limited" in r["unavailable_reason"] for r in tails))

    def test_policy_resource_mismatch_is_rejected_before_capture(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp); fixture(path)
            options = json.loads((path / "metadata.json").read_text())["options"]
            for key, value in [("heap_mib", 128), ("cpu_quota", 1), ("rss_mib", 256),
                               ("cpus", [1]), ("workloads", ["allocation", "mixed"])]:
                destination = path / key
                destination.mkdir()
                with self.subTest(key=key), self.assertRaises(ValueError):
                    freeze_acceptance_policy(path / "declared.json", destination, dict(options, **{key: value}))
                self.assertFalse((destination / "acceptance-policy.json").exists())

    def test_marker_does_not_invent_missing_data_or_accept_duplicate_packets(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp)
            (path / "stdout.txt").write_text("RISTRETTO_GC {}\n")
            self.assertIsNone(qualification_observation(path))
            (path / "stdout.txt").write_text("GC_QUALIFICATION {}\n" * 2)
            with self.assertRaises(ValueError):
                qualification_observation(path)

    def test_cli_preserves_historical_and_existing_versioned_reports(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp); fixture(path)
            (path / "gates.json").write_text("historical evidence\n")
            command = [sys.executable, str(Path(__file__).with_name("qualify.py")), str(path)]
            first = subprocess.run(command, capture_output=True, text=True)
            self.assertEqual(first.returncode, 0, first.stderr)
            saved = (path / "gates-v2.json").read_bytes()
            second = subprocess.run(command, capture_output=True, text=True)
            self.assertNotEqual(second.returncode, 0)
            self.assertEqual((path / "gates-v2.json").read_bytes(), saved)
            self.assertEqual((path / "gates.json").read_text(), "historical evidence\n")
            old = subprocess.run(command + ["--policy-version", "1", "--candidate", CANDIDATE],
                                 capture_output=True, text=True)
            self.assertEqual(old.returncode, 1)
            self.assertTrue(json.loads((path / "gates-v1.json").read_text())["historical_only"])


if __name__ == "__main__":
    unittest.main()
