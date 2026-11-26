"""Freeze a finite fork/warmup matrix; this command executes no workload."""
import argparse
import json
from pathlib import Path
import tarfile
from canonical_service import digest
from canonical_service_artifacts import read_json, verify_build
from ring_artifacts import validate_resources, verify_peer_build
from ring_comparison import MODEL, SIZES, validate_admission, validate_matrix


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--native-build", type=Path, required=True)
    p.add_argument("--wasi-build", type=Path, required=True)
    p.add_argument("--peer-build", type=Path, required=True)
    p.add_argument("--output", type=Path, required=True)
    mode = p.add_mutually_exclusive_group()
    mode.add_argument("--check", action="store_true")
    mode.add_argument("--warmup-check", action="store_true")
    p.add_argument("--workloads", nargs="+", choices=list(SIZES), default=list(SIZES))
    p.add_argument("--steps", type=int, default=257)
    p.add_argument("--shared-heap", action="store_true", help="one canonical heap across warmup and samples; schema 2")
    p.add_argument("--heap-cleanup-polls", type=int, default=4_000_000)
    p.add_argument("--warmup", type=int, default=100)
    p.add_argument("--samples", type=int, default=30)
    p.add_argument("--allocation-samples", type=int, default=3)
    p.add_argument("--forks", type=int, default=3)
    p.add_argument("--resources", type=Path)
    p.add_argument("--heap-bytes", type=int, default=64 * 1024 * 1024)
    p.add_argument("--watchdog-seconds", type=int, default=120)
    args = p.parse_args()
    if args.check:
        steps, warmup, samples, forks = args.steps if args.shared_heap else 7, 0, 1, 1
    else:
        steps, warmup, samples, forks = args.steps, args.warmup, args.samples, 1 if args.warmup_check else args.forks
        if args.resources is None:
            raise ValueError("measurement plans require a frozen resource envelope")
    if not (0 <= steps <= 1_000_000 and 0 <= warmup <= 128 and 1 <= samples <= 128 and 1 <= forks <= 32):
        raise ValueError("finite matrix bounds exceeded")
    if not 1 <= args.allocation_samples <= 128:
        raise ValueError("allocation diagnostic sample cap exceeded")
    if not 1 <= args.watchdog_seconds <= 3600:
        raise ValueError("finite process watchdog must be in 1..3600 seconds")
    if args.shared_heap and (args.heap_bytes != 64*1024*1024 or args.heap_cleanup_polls <= 0):
        raise ValueError("shared protocol requires 64 MiB payload/peer heap and a positive final cleanup limit")
    request = validate_resources(read_json(args.resources)) if args.resources else None
    native, wasi, peer = verify_build(args.native_build), verify_build(args.wasi_build), verify_peer_build(args.peer_build)
    if (native["binary_name"] != "program" or "compiled" not in native["features"]
            or wasi["binary_name"] != "program.wasm"):
        raise ValueError("native compiled-capable and WASI interpreter artifacts required")
    if not args.check and any("--release" not in build["command"] for build in (native, wasi)):
        raise ValueError("timing matrix requires release artifacts")
    root = args.output.resolve()
    root.mkdir(parents=True, exist_ok=False)
    (root / "classes").mkdir()
    with tarfile.open(args.native_build / "sources.tar.gz") as archive:
        for name in ("CanonicalRingWorkload.class", "CanonicalRingWorkload$Node.class"):
            (root / "classes" / name).write_bytes(archive.extractfile("ristretto_vm/tests/fixtures/canonical/" + name).read())
    (root / "admissions").mkdir()
    for workload in args.workloads:
        retained = SIZES[workload][0]
        for tier in ("interpreted", "compiled"):
            admission = {"schema_version": 1, "tier": tier, "class": "CanonicalRingWorkload", "method": workload,
                         "argument": steps, "expected_result": MODEL.expected(workload, steps), "recorder_capacity": 0,
                         "require_compiled_backedges": True, "window_kind": "full_invocation_v1",
                         "configuration": {"objects": retained * 4 + 8, "roots": 128, "mutators": 1, "helpers": 1, "edges_per_object": 3},
                         "limits": {"instructions": 4_000_000, "request_polls": 1_000_000, "cleanup_polls": 4_000_000,
                                    "frames": 64, "native_slices": 4_000_000, "array_bytes": 8 * 1024 * 1024}}
            if args.shared_heap:
                admission.update(schema_version=2, window_kind="shared_heap_invocation_v1",
                                 max_payload_bytes=args.heap_bytes, heap_cleanup_polls=args.heap_cleanup_polls,
                                 minimum_primary_cycles=1)
            validate_admission(admission)
            (root / "admissions" / f"{workload}-{tier}.json").write_text(json.dumps(admission, indent=2) + "\n")
    if request is not None:
        (root / "resources.json").write_text(json.dumps(request, indent=2) + "\n")
    cases = []
    runtimes = ["g1", "zgc", "native-compiled", "native-interpreted", "wasi-interpreted"]
    for workload in args.workloads:
        for fork in range(forks):
            if not (args.check or args.warmup_check):
                for runtime in ("g1", "zgc"):
                    cases.append({"id": f"{workload}-fork{fork}-{runtime}-allocation", "workload": workload,
                                  "fork": fork, "runtime": runtime, "mode": "allocation"})
            order = runtimes[fork % len(runtimes):] + runtimes[:fork % len(runtimes)]
            for runtime in order:
                cases.append({"id": f"{workload}-fork{fork}-{runtime}", "workload": workload, "fork": fork,
                              "runtime": runtime, "mode": "check" if args.check else "warmup-check" if args.warmup_check else "measure"})
    builds = {}
    for family, path, receipt in (("native", args.native_build, native), ("wasi", args.wasi_build, wasi), ("peer", args.peer_build, peer)):
        filename = "peer-build.json" if family == "peer" else "build.json"
        builds[family] = {"directory": str(path.resolve()), "receipt_sha256": digest(path / filename),
                          "source_manifest_sha256": receipt["source_manifest_sha256"]}
    plan = {"schema_version": 2 if args.shared_heap else 1, "qualification": False, "check_only": args.check or args.warmup_check, "warmup_check": args.warmup_check, "builds": builds,
            "steps": steps, "warmup_invocations": warmup, "samples": samples, "allocation_samples": args.allocation_samples, "forks": forks,
            "workloads": args.workloads, "heap_bytes": args.heap_bytes,
            "watchdog_seconds": args.watchdog_seconds,
            "resources_sha256": digest(root / "resources.json") if request else None,
            "admission_sha256": {file.name: digest(file) for file in sorted((root / "admissions").glob("*.json"))},
            "cases": cases, "scopes": {"canonical": "shared_heap_invocation_v1" if args.shared_heap else "full_invocation_v1", "hotspot": "workload_call_deferred_gc_v1"},
            "canonical_gc_cycles_available": False, "scheduled_latency_available": False,
            "sustainable_throughput_available": False, "body_diagnostics_in_this_matrix": False}
    if args.shared_heap:
        plan.update(canonical_primary_cycle_scope="sample_batch_including_driver_gaps_and_observer",
                    canonical_primary_cycle_required=not args.warmup_check)
    validate_matrix(plan)
    (root / "plan.json").write_text(json.dumps(plan, indent=2) + "\n")
    print(json.dumps({"plan": str(root / "plan.json"), "sha256": digest(root / "plan.json"), "cases": len(cases), "executed": False}, indent=2))


if __name__ == "__main__":
    main()
