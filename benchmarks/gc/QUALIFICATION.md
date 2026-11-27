# Native qualification evidence, policy v2

The v2 milestone checks observed native stops below 1 ms and declared latency
and resource limits. It reports G1/ZGC ratios as diagnostics. The full project
goal additionally requires zero GC-induced global stops and superior measured
throughput and latency on the declared native peer workloads. `qualify.py`
evaluates short-run evidence and always reports `qualification: false`.
Capacity, progress and endurance require separate evidence.
[Plan and success criteria](../../ristretto_gc/PLAN.md#success-criteria).

## Finite search for delivered throughput

`prepare_scheduled_ring.py --policy delivered-capacity` freezes a search for
the highest offered rate at which every fork completes every scheduled request
by its deadline. It records late completions, unstarted work and the existing
p99.9 and peak-backlog limits separately. A fork below the predeclared sample
floor invalidates the point. Endpoint confirmation requires three fresh forks
per endpoint in the staged peer protocol, observed collection activity in every
confirmation fork, and a passing/failing bracket within the declared tolerance.

The peer plan uses the medium persistent Java ring, 100 warmups, 3 forks per
rate, an initial rate of 16,384/s and a 5% tolerance. Each fork runs for at
least 60 seconds. Before the first fork at a rate, the coordinator computes the
duration as the larger of that minimum and 12,500 divided by the smaller of the
offered and initial rates, rounded up to the next second. It adds 180 seconds
for the watchdog. The same duration applies to every fork at that rate. The
plan allows up to eight search rates and two confirmation rates per collector.
Long-running sustainability and extreme-tail qualification require separate
measurements. [Frozen peer plan](../../../ristretto-gc-evidence/delivered-capacity-peers-v1/plan/plan.json).

On a shared host, `run_scheduled_capacity.py --point-windows DIRECTORY` can
keep a search alive between quiet measurement windows. The coordinator records
the next point in `waiting_for_window`. An atomic rename of
`DIRECTORY/<point>.json` containing the matching `point`, `plan_sha256` and
`quiet_window_id` starts all forks for that point. The rate, duration and fork
count come from the frozen search. Each fork records its actual window ID.

**No current runtime emits the complete packet below.** The runner and checker
implement transport and validation, not runtime instrumentation or a proof of
collector progress. Existing lifetime counters, a JIT flag and GC phase logs cannot
substitute for the required evidence. `test_qualification.py` uses explicitly
synthetic fixtures; its passing case is not a measured GC result.

## Partial publication-writer producer (diagnostic only)

Linux targets with native 64-bit atomics can build `gc_compare` with
`--features experimental-pause-recorder`. The recorder is absent from normal
builds. `run.py --pause-recording --pause-capacity 65536` explicitly selects its
diagnostic transport; instrumented rows are retained outside timing groups and
cannot satisfy the v2 acceptance checker. Run recorder-overhead experiments
separately before using instrumentation in qualification measurements.

The collector binds a preallocated recorder through `Configuration.pause_recorder`
before any VM initialization. Every successful legacy publication writer emits
request/acquisition/release timestamps using Linux `CLOCK_MONOTONIC`; declaration
order drops the actual writer guard before sampling release. Request time precedes
`try_write`, so acquisition/rendezvous cost remains included. Failed attempts take
no event slot and are counted separately. The recorder uses fixed atomic slots and
clock calls, with no hot-path allocation, mutex, retry loop, or waits for another
producer. Recording costs and the fixed buffer remain charged to the process.

`RISTRETTO_PAUSES` schema 2 is separate from the acceptance packet and the
previous archived recorder-only schema 1. It is explicitly `diagnostic_only: true`,
`qualification: false`, `scope: instrumented_collector_lifetime`, and
`capture_complete: false`. It covers the instrumented collector through shutdown,
not uninstrumented executable startup. `instrumented_capture_complete` describes
only that source's recording: overflow, unfinished events, clock failures, failed
shutdown or inconsistent acquisition counts prevent it. Snapshotting consumes the
recorder after every owner has gone; a still-live collector prevents export rather
than silently losing later events. The source audit is always incomplete, with
`allocation_recovery` and `gc_safepoint` explicitly uncovered. Mutator admission
waits remain a separate unrecorded metric. Concurrent-publication bypass produces
no writer events; that is not proof of zero equivalent/global stalls.

Identity setup occurs before collector capture. The runner creates a fresh run ID
and an exec launcher records the actual VM PID; the PID survives exec and is checked
against the runtime packet. The receipt is retained and hashed with process output.
The runtime computes its own binary hash and a recursive sorted class-bytecode
manifest hash. The runner verifies both plus the frozen source-archive hash, policy
hash, runtime/workload arguments and recorder capacity. The source hash is a runner
assertion checked against frozen build provenance, not a self-attestation of source
contents by the running executable. The launcher's startup cost is charged to the
same process resource envelope.

`pauses.py` validates binding and preserves failing/overflowed raw traces. Empty
writer streams have no observed maximum (`null`), never an invented zero-STW
result. It merges overlapping/touching intervals before testing the exclusive
1 ms budget. Boundary-crossing events retain their entire request-to-release
durations rather than clipping away a budget violation. This does not make the
partial recorder qualification eligible.

### Explicit Java measurement boundaries and monotonic time

`GcTelemetry.beginMeasurement()` and `endMeasurement()` are ordinary Java methods
whose peer implementation calls `System.nanoTime()`. The shared `GcBench` calls them
only at its existing measurement-start/end points, after warmup and before reporting.
Both source files are compiled identically for all runtimes and included in the
bytecode manifest. A diagnostic Linux Ristretto build registers explicit intrinsic
implementations of those two boundary methods. Other builds/platforms run the same
ordinary Java fallback; no benchmark-specific JIT eligibility is introduced.

Each hook returns exactly the monotonic timestamp recorded in `measurement_window`.
The runner verifies that end minus start equals the Java `GC_RESULT.elapsed_ns`.
It never infers a workload phase from stdout-arrival timestamps. Missing, duplicate,
open or out-of-order boundaries cannot satisfy measured-window binding. The native
producer stores one begin/end pair using a single atomic state transition per
boundary, with no wait/retry and no new collector lock.

A recorder-local atomic counts completed collections on the JVM heap. Both sides
of each boundary timestamp sample that counter, exposing boundary races explicitly.
The guaranteed in-window count is end-before minus begin-after; the possible count
is end-after minus begin-before. Startup/warmup/reporting collections do not become
measured exposure. Zero observed in-window completions remain zero. Both execution tiers report completions from the same
`ristretto_gc::GarbageCollector`.

`measurement_boundaries` retains observer start/end times and both counter samples;
`gc_exposure` retains the conservative lower/upper completed-count bounds. Observer
calls occur twice at the boundaries, outside the per-operation loop. Some dispatch
and recorder work remains inside Java elapsed time and is charged to it; the spans
are diagnostic, not an independently measured overhead correction. Recorder/JIT
profiler costs still require separate controlled experiments. This work does not
produce measured compiled coverage or a complete acceptance packet.

The previous `System.nanoTime()` intrinsic incorrectly used `SystemTime` since the
Unix epoch. It now shares the monotonic source with the hooks and Linux recorder:
Linux `CLOCK_MONOTONIC`; other clocked native/WASI hosts use one process-local
`Instant` origin. Epochs are arbitrary and must not be combined across processes or
hosts. Bare wasm reports the unavailable clock without invoking a panicking host
clock; no fabricated timestamps are supplied. `System.currentTimeMillis()` retains
its wall-clock epoch behavior. This correctness change and the new shared bytecode
require fresh baselines: historical wall-clock timing is not reclassified as new
monotonic evidence. Source-clock tests execute natively and under WASI; finite Java
fixtures check exact boundary elapsed time and exclude pre-/post-window collections.

## Freeze the policy before measurement

Supply `run.py --acceptance-policy POLICY.json` with the enforced cgroup options.
The runner validates the policy against its workload/resource options and copies
its exact bytes to `acceptance-policy.json` before compiling the workload or
launching measured children. Metadata records the policy SHA-256. The native
binary must have the build provenance and source archive accepted by `freeze_build`.
The policy, binary, classes and source artifacts are hashed and checked after the
run and again by the evaluator. Choose workload limits before tuning, independently
of peer results; the runner provides no implicit acceptance limits.

The JSON root has exactly these fields:

| Field | Contract |
|---|---|
| `version` | Integer `2`. |
| `candidate` | A supported native production JIT runtime, such as `ristretto-candidate-jit`. Both `g1` and `zgc` must be in the run. |
| `resources` | `heap_mib`, `memory_bytes`, `cpu_quota_us`, `cpu_period_us`, `cpus`. Positive integer budgets, unique CPU IDs; period is 100000 µs and swap is disabled. Values must match runner options and each cgroup. |
| `workloads` | Limits for every selected primary workload: `allocation`, `roots`, `mixed`. The `control` workload is excluded from qualification. |
| `coordination_audit` | Reviewed audit object described below, or null for exploratory runs that cannot pass. |

Each workload object has exactly `response_p999_ns`, `response_p9999_ns`,
`peak_backlog`, and `required_methods`. Response limits are positive integers with
p99.99 at least p99.9; backlog is a nonnegative integer. The checker compares the
histogram quantile's conservative upper bound with the declared limit on fixed-rate
runs. These response limits are distinct from the native STW limit, which is fixed
at strictly less than 1,000,000 ns and cannot be relaxed in the policy.

Each required method has exactly `class`, `name`, `descriptor`, `min_entries`,
`entries_per_operation`, and `backedges_per_operation`. Names/descriptors use JVM
identities. `min_entries` is positive; per-operation counts are nonnegative integers.
The manifest must include the measurement loop, operation, constructor and hot
dependencies required by J0; a minimal arbitrary manifest is not an audited hot-path
declaration. This checker cannot infer manifest completeness from method names.
Required compiled entries must reach the greater of `min_entries` and completed
operations times `entries_per_operation`. Compiled backedges must reach completed
operations times `backedges_per_operation`.

The audit has exactly `recorder`, `binary_sha256`, `source_archive_sha256`,
`coverage_complete`, `unbounded_global_blocking`, `review_reference`, and
`global_stop_sources`. Hashes must match the frozen native build; the two status
fields are booleans. Sources must contain each of `gc_safepoint`, `publication`,
and `allocation_recovery` exactly once. A pass requires complete source coverage
and no unbounded global blocking. The review must explain how every equivalent
heap-wide stall is covered, including startup, allocation recovery and shutdown.
`review_reference` identifies the actual review; it does not generate or prove one.
The current stalled-admission/callback limitations remain grounds for an incomplete
audit. Setting a boolean cannot resolve those limitations.

## Per-process capture

The runner supplies a fresh `RISTRETTO_QUALIFICATION_RUN_ID` and the frozen
`RISTRETTO_QUALIFICATION_POLICY_SHA256` to each child. A future recorder must emit
exactly one stdout line `GC_QUALIFICATION ` followed by a JSON object. The runner
retains that object alongside `GC_RESULT`, stdout/stderr, resource capture and GC
logs. The evaluator checks hashes and equality with captured output; adding a
summary to `raw.jsonl` alone cannot satisfy the contract. Candidate runtime-mode
observations must also agree with the captured `RISTRETTO_GC` record. A diagnostic
`RISTRETTO_EXECUTION` profile cannot qualify timing.

Packet fields are:

| Field | Contract |
|---|---|
| `schema_version` | Integer `1`, separate from acceptance-policy version. |
| `run_id`, `policy_sha256` | Exact identities supplied by the runner. |
| `window` | `clock: "process_monotonic_ns"` and integer `process_start_ns`, `measurement_start_ns`, `measurement_end_ns`, `process_end_ns` in that order. The measurement duration equals `GC_RESULT.elapsed_ns`. |
| `gc_exposure` | `scope: "measurement"`, integer `collections_completed >= 1`, required for candidate and both peers. Bootstrap-only collections are insufficient. |
| `pauses` | Complete candidate global-stop capture described below. Peer pause lengths do not apply to the candidate's acceptance budget. |
| `execution` | Candidate measured-window production JIT coverage described below. |

Capture must cover all managed execution from initialization through teardown; a
recorder cannot label only the timed loop as the whole process. The reviewed
implementation must establish the boundaries and event accounting. Hash binding
detects inconsistent artifacts; it does not independently establish the truth of
recorder assertions. Peer measured-window GC exposure also needs a producer; the
existing HotSpot log summary is whole-process diagnostic data.

### Global stops

`pauses` contains `schema_version: 1`, `scope: "whole_process"`, the audited
`recorder` identifier, `capture_complete: true`, sorted `global_stop_sources`,
`dropped_events: 0`, `open_events: 0`, `clock_resolution_ns`, `capture_start_ns`,
`capture_end_ns`, and `events`. Capture bounds must equal the process bounds. The
clock resolution must be positive and finer than the 1 ms acceptance budget.
Recorder accuracy and overhead still need independent validation.

Every event has `source`, `requested_ns`, `mutators_stopped_ns`, and `released_ns`.
All timestamps must be ordered and lie within capture. Request-to-release time
includes rendezvous. Equivalent heap-wide publication/recovery blocking belongs in
this stream even if the collector never explicitly parks every mutator. Local
payload waits, individual allocation waits and cooperative slices stay separate.

The evaluator merges overlapping or touching request-to-release intervals before
checking continuous stop duration. A maximum of exactly 1 ms fails. It reports raw
event count, merged stop count, durations, maximum, total stopped time, fraction of
the process window, clock resolution and raw event rendezvous time. Overlapping
event rendezvous sums are descriptive and are not additive with merged stop time.
Raw component timestamps remain in the packet.

An empty stream passes this check only with complete capture and the matching
reviewed audit. It reports `zero_observed_stops: true` and null percentiles, not an
inferred universal zero-STW guarantee. Nonempty distributions have descriptive
percentiles only; small event counts cannot establish extreme-tail population
claims. Every observed stop must meet the budget regardless of percentiles.

### Compiled execution

`execution` contains `schema_version: 1`, `scope: "measurement"`, `mode: "jit"`,
`ready_before_measurement: true`, `manifest_sha256`, `unexpected_fallbacks: 0`,
positive `generated_code_bytes`, nonnegative `compilation_ns`, and `methods`.
The manifest hash uses `acceptance.canonical_hash(required_methods)`: sorted-key,
compact JSON with non-finite values prohibited.

Each method record has `class`, `name`, `descriptor`, a positive unique `instance`,
and nonnegative integer `compiled_entries`, `compiled_backedges`,
`interpreter_entries`, and `intrinsic_entries`. Duplicate identities/instances fail
as ambiguous coverage. Required methods must satisfy their declared counts with
zero interpreter/intrinsic entries. All counts refer to measured execution, not
the lifetime profile or compiled-code installation alone. Unexpected fallback
fails, and a missing required method fails.

This first contract is deliberately restrictive. Inlined methods, approved native
helpers, loader-specific duplicate names and frames entered before measurement
need explicit evidence semantics before they can qualify. Do not relabel lifetime
entries as measured entries. Time-weighted attribution, fallback reasons, mixed
transitions, code-cache histories and validated compilation stability remain J0/J5
work beyond this short checker.

The diagnostic VM checkpoint API now reports scoped counter differences and taken
native backedges. Its scope is explicitly `checkpoint_interval`, with separate
per-method counter cuts and native snapshot costs. The `gc_compare` diagnostic
interval covers the whole main invocation, including warmup and reporting. Neither
that interval nor lifetime backedge totals satisfy this measured-window contract.
Compiled-code presence at a checkpoint also does not establish production dispatch
readiness. Keep these diagnostic records separate from qualification packets.

## Evaluate without changing historical evidence

```sh
python3 benchmarks/gc/qualify.py target/new-comparison
python3 benchmarks/gc/qualify.py target/historical-comparison --policy-version 1
```

Outputs default to `gates-v2.json` and `gates-v1.json`. Both refuse existing paths;
`--output NEW_PATH` permits another evaluation without replacing earlier reports.
Reports include evaluator/input hashes and the policy version. Historical v1
retains the original peer-ratio thresholds and is labeled `historical_only`.

V2 requires a complete declared schedule, at least five independent paired forks,
successful exits, validated checksums/histograms, no unfinished work, equal enforced
budgets without OOM, and one million completions per fixed-rate process. Missing,
duplicate, failed or censored pairs are never dropped to improve ratios. Undefined
or resolution-limited ratios are labeled unavailable. Valid throughput/response
ratios remain diagnostics even when absolute candidate checks fail.

No v2 short pass establishes sustainable capacity, steady-state warmup, adequate
recorder overhead, bounded memory/backlog histories, stalled-participant progress,
the three paired hour trials or separate day trial. No native verdict applies to
WASI/bare Wasm or browser host-turn budgets. Keep all those gates open and retain
defaults until the complete plan is qualified.
