# Persistent heap ring comparison (schema 2)

This is a distinct finite protocol built on the shared-heap canonical API. It does not relabel schema-1 `full_invocation_v1` results. `prepare_ring_comparison.py --shared-heap` creates schema-2 admissions and a schema-2 matrix; omitting the flag retains the dedicated-heap protocol. All measurement runs still require an externally coordinated quiet window.

## Lifetime and timing boundaries

Admit the exact two shared ring classfiles once and compile once for native compiled execution. Create one heap with `Program::with_heap`, then attach one interpreted or compiled context. The same heap, mutator/helper registrations and compiled frame table survive all warmup and sample invocations. Each `invoke` releases its Java frame roots, while garbage and collection work can carry into later invocations. Invocation limits restart on each call. No per-invocation collector is constructed and no per-invocation drain is forced.

The canonical scope `shared_heap_invocation_v1` times `invoke` using two monotonic clock reads. Call start/end clocks and elapsed time are retained. Checksum validation and bounded sample-buffer writes occur after the end clock. Input hashing, class admission, compilation, heap setup/context attachment, final context detachment/heap drain/catalog drop and program teardown are excluded from call clocks and reported as separate preparation/teardown durations. The sample buffer is reserved before heap setup. No logging or span recorder runs in invocation clocks. Existing compiled execution-profile counters remain enabled and are not assumed free.

HotSpot keeps the existing `workload_call_deferred_gc_v1` scope and observer: one JVM heap across warmup and samples, with no forced per-call cleanup. The Java observer, workload source/classfiles, allocation diagnostics and ordinary timed-process logging flags remain unchanged. Both runtimes now retain a heap across calls, but their representations, collection policies, caller dispatch and final teardown behavior remain different. This protocol matches heap lifetime; it does not assert identical physical heap accounting.

## Bounds and final accounting

Schema-2 admissions explicitly require:

- `max_payload_bytes: 67108864`: canonical managed payload-accounting ceiling, passed directly to `with_heap`.
- `heap_cleanup_polls: 4000000` in the initial matrix: final shared-workload drain budget, distinct from per-call request/native/instruction limits.
- `minimum_primary_cycles: 1`: a strictly positive conservative lower bound on primary-cycle completions during the sample batch for checked/timed captures.
- `window_kind: shared_heap_invocation_v1`, no service recorder, one mutator and one helper.

The peer must use `-Xms67108864 -Xmx67108864`. The controller and independent checker reject a peer heap differing from the declared canonical payload ceiling. The catalog still has separately admitted slot/edge/root/participant quotas (40/136/520 slots for small/medium/large, 128 roots, three edge positions). Each packet reports `reserved_catalog_bytes` from `Configuration::charged_bytes`; this pre-reserved catalog charge is outside the 64 MiB payload ceiling. Rust/program/JIT metadata, allocator overhead, stacks, external runtime state and RSS are separate. The common envelope remains CPUs 0/1, two-CPU quota, 512 MiB cgroup charged-memory ceiling and 256 pids. The Linux v2 process resource receipt reports complete child lifetime including reaped descendants; WASI means Wasmtime host, never guest-only RSS or simultaneous process-tree total.

Each successful invocation reports the actual result, logical allocations, helper polls, released slots and native slices exposed by `Invocation`. A call can release garbage allocated by earlier calls; no per-call allocations-equal-releases assertion is valid. Warmup and sample allocations/releases are accumulated separately. After all contexts detach, `with_heap` drains payloads and verifies zero managed payload plus full catalog-credit refund before returning success. The driver also requires warmup + sample allocations to equal warmup + sample + final-cleanup releases. Final cleanup polls, final releases and observed payload bytes before cleanup are retained. `heap_cleanup_verified` records this successful API contract, not a fabricated numeric post-drop read or physical-memory return claim.

## Passive primary-cycle observations

All cycle observation occurs between Java invocations. Four single-attempt `primary_cycles()` observations bracket a separate batch clock: before-start observation, start clock, after-start observation; execute all timed calls; before-end observation, end clock, after-end observation. Missing/failed observations abort the capture. This batch includes driver gaps and two inner observer calls; its duration is not substituted for summed invocation clocks. Observer overhead and temporary-owner destruction are not assumed zero.

The independent checker verifies a feasible nondecreasing counter history through all four lower/upper intervals, then recomputes:

```
lower = max(0, before_end.lower - after_start.upper)
upper = after_end.upper - before_start.lower
```

Both emitted bounds must match; the lower bound must meet the positive admission requirement. Every timed call must lie within the actual batch clock interval, in order, with elapsed equal to end minus start. These are completed primary full-epoch counter changes, not all GC domains, individual-invocation exposure, cycles whose entire work started/finished inside the batch, slot-release counts or pause durations. The secondary bootstrap domain is not added. HotSpot retains per-bean sample-batch observations including driver gaps; separate diagnostic-process logs are never promoted to timed-process proof.

Compiled execution preserves all seven sorted method identities, including unexecuted methods, at before-warmup/before-samples/after-samples snapshots. Exact model entries/backedges and monotone resumes are required; aggregate sample and warmup native slices must equal corresponding entries plus resumes. Per-call array/call/profile fields unavailable in `Invocation` are not invented. Interpreted execution must report no compiled census or native slices.

## Finite checks and planned comparison

Use the same builder and launcher, with source-bound release binaries and separate source-root Cargo caches. `--shared-heap --check --steps 257` checks the exact comparison input with no invocation timers; unlike legacy `--check`, shared checks retain the requested steps to exercise real collection. `--shared-heap --warmup-check --workloads large` runs the 100+30 shape without clocks, primary-cycle observations or exported work/census counters; only correctness and final cleanup are emitted. The supervisor's whole-child duration only sizes the watchdog. Functional resource metrics are not comparison results.

The initial timed matrix remains five runtimes × three sizes × three forks, each with 100 warmups and 30 samples, plus 18 separate G1/ZGC allocation-diagnostic forks with 100 warmups and three observed invocations. Keep 257 steps, the same expected checksums, 120-second child watchdog and original deterministic fork order. A canonical sample batch that fails the exposure or cleanup requirement fails the run; gates are not relaxed after observing results.

This is closed-loop finite invocation evidence. Sustainable capacity, open-loop scheduled latency/extreme tails, resource endurance, arbitrary concurrent Java tasks, global/equivalent GC pauses, three paired hours and the separate 24-hour qualification remain outside this matrix. Global capture completeness stays false.
