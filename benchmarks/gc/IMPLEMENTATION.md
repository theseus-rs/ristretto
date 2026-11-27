# GC implementation results — 2026-11-20

Evidence availability notice (2026-11-15 22:23 UTC): the workspace `target`
directory disappeared, including historical GC raw results, graphs and archives.
The source changes remain. Results recorded below are historical observations;
links under target are unavailable and cannot serve as current qualification evidence.
See [the recovery record](EVIDENCE_RECOVERY.md). Fresh component validation is
complete outside Cargo build storage; no full qualification gate is closed by
the earlier summaries.

## Current integration and attribution (2026-11-20)

The combined source now includes per-request scheduling, publication of the
actual number of Java reference edges, and independently deliverable allocation
and Java Reference Handler wakeups. The GC library passed 373 native and 266
executed WASI checks. The VM passed 85 native and 65 executed WASI checks,
including native compiled Java. Both crates passed strict lint on both targets.
[Integration receipt](../../../ristretto-gc-evidence/integration-v8/summary.json).

Nine actual fine-attribution captures independently validate entry, setup,
cleanup and teardown costs. The instrumented request path dominates entry;
97.38–98.82% of request-gap duration follows Pending polls. Hundreds of thousands
of recording events perturb each invocation, so no ordinary speedup or pause
claim follows from the traces. The separate ordinary invocation benchmark found
1.93–2.25× native and 2.36–2.90× WASI speedups from request scheduling on the
retained ring. The experiment predates allocation by field layout. Shared-heap
integration, global pause coverage, sustained throughput and endurance remain
open. [Measurement record and graphs](CANONICAL_MEASUREMENTS.md#request-scheduling-under-the-ordinary-invocation-timer).

## Measurements and cycle observation (2026-11-16)

The [current measurement record](CANONICAL_MEASUREMENTS.md) links the allocation
and G1/ZGC graphs, exact sources and raw results. Immutable metadata links reduce
small-ring allocator requests from 22,322 to 5,897 interpreted and from 34,323 to
10,289 compiled. All 54 after-change count samples pass; preparation costs are
reported separately. These counters establish no elapsed-time improvement.

The pre-optimization comparison completed 45 timing forks and 18 separate peer
allocation diagnostics, with all 1,350 measured checksums passing. Native compiled
invocations take 6–15% longer than interpreted invocations under the same
lifecycle. The much larger peer gap includes different heap setup/cleanup and
deferred-GC scopes, so it cannot isolate collector cost. The finite run supplies
no sustained-throughput, extreme-tail, RSS or global-pause qualification.

Passive primary-cycle observation and schema-2 service windows are also merged.
Six actual captures validate independently derived bounds around the recorder's
entry clock boundaries. The combined observer, window and metadata checkpoint
passes 61 native and 32 WASI tests, scoped strict lint and formatting; its frozen
source is `ristretto-gc-evidence/integration-v5/`. Ordinary timed binaries leave
recording disabled and cycle counts unavailable. Broader VM integration, complete
pause coverage and sustained/endurance gates remain open; defaults are unchanged.

The subsequent optimized-source comparison also passes 63 processes and 1,350
samples using unchanged inputs and harness. Native median invocation changes are
−1.92% to +0.15%; WASI changes are −2.85% to −0.63%. The large allocation-count
reduction does not establish an elapsed-time speedup. The current measurement
record links the updated peer and before/after graphs, all fork ranges and source
differences. Cost attribution is the next performance task.

Reference arrays are now merged in both native tiers and the WASI interpreter.
The combined checkpoint passes 72 native and 41 WASI checks, including the
34-row reference-array oracle, plus strict library/test lint. The original frozen
array source passes six Miri checks (including full-size cancellation), eleven
ASan and eleven TSan checks. Its separate finite revision passes one Miri and six
Tree checks, with cancellation scoped to a nine-element tree and identical
production code; the original full-size Miri success remains separately bound.
`ristretto-gc-evidence/integration-v6/clean-sources.tar.gz` is the clean source
artifact for this checkpoint. Schema-2 process-resource receipts subsequently
pass twenty Python checks and five actual runtime captures, including rejection
of rehashed stale-run/wrong-watchdog receipts. These development-artifact resource
checks make no performance or RSS-ceiling claim; older timing evidence is unchanged.

## Java integration checkpoints (2026-11-15)

The feature-gated [canonical Java tier](https://github.com/theseus-rs/ristretto/blob/0497123e05056546bc526ecc49c081f934f38d64/ristretto_vm_research/CANONICAL_JAVA.md)
runs shared javac graph bytecode on native and WASI; each platform passes twelve
integration cases and a parked-future cancellation check. The independent
[ordinary-VM JIT slice](../../ristretto_vm/EXPERIMENTAL_JIT.md) verifies HotSpot
parity and actual compiled execution with legacy roots. The request-only compiler
ABI passes its JIT regression suite and links the shared methods without legacy
heap helpers. These source checkpoints are integrated; their combined source
passes thirteen native and twelve WASI integration cases, cancellation on both
platforms, Clippy and formatting. Receipts and executed binaries are retained in
`/home/brianheineman/development/ristretto-gc-evidence/integration-v1/`;
`combined-summary.json` identifies the exact source snapshot and the corrected
test-output parser expectation.

The next integrated checkpoint adds canonical compiled frames: typed token
identity, frame-owned roots, cancellation-safe request/result handoff and separate
native-slice quotas. The merged configuration with clock/pause recording passes
eighteen native and thirteen WASI Java integration cases, five/three ownership
cases, 699/381 GC test results, lint, formatting and 58 Python checker tests.
`/home/brianheineman/development/ristretto-gc-evidence/integration-v2/summary.json`
binds this exact source and retained executions. The runnable graph example also
passes in native interpreted/compiled and WASI interpreted modes; requesting
compiled execution on WASI explicitly rejects. These example checks are finite
correctness demonstrations.
The third integration checkpoint admits all eight primitive-array kinds and
broader numeric execution. Each native tier and the WASI interpreter passes 301
HotSpot oracle rows, including array faults, floating boundaries and implicit
narrow returns. The merged source passes 23/17 Java integration cases, 7/4
ownership cases, 706/388 GC results, 508 JIT results and strict lint. The original
module-order formatting failure is retained beside its passing correction in
`/home/brianheineman/development/ristretto-gc-evidence/integration-v3/summary.json`.
The isolated VM checkpoint also passes Miri/Tree Borrows and host-runtime
ASan/TSan checks; those tools do not instrument generated native instructions.

The shared [retained-ring workload](canonical-ring-oracle/README.md) passes 21
cases on each execution path, matches HotSpot and an independent sequence model,
and reclaims all objects after exercising live-ring slot reuse. Its source-bound
results and executed binaries are retained in
`/home/brianheineman/development/ristretto-gc-evidence/canonical-ring-v2/`.
The subsequent quiet-host allocation check passes 18 G1/ZGC processes (three
forks per collector and size). Every checksum matches the independent model and
actual thread allocation exceeds minimum payload demand. Aggregate counts include
headers and launcher overhead; they do not prove every individual allocation
survives optimization. Source, JVM artifacts, bytecode, process receipts and raw
outputs are retained in `canonical-ring-v2/peer-allocation-exposure/`. This is
allocation exposure only, with no throughput, latency or in-window GC assertion.

This remains a restricted local-task tier with one mutator and a dedicated heap.
The service recorder, atomic managed-byte accessor, VM hooks and positive 63-bit
token leasing are merged. The combined checkpoint passes 29/21 native/WASI Java
integration cases, 22/17 ownership/module cases, targeted clock/recorder/accounting
checks and strict lint with recording enabled and disabled. Its exact source and
explicit comment-only lint correction are bound by
`/home/brianheineman/development/ristretto-gc-evidence/integration-v4/summary.json`.
The isolated token checkpoint passes Miri 14 and ASan 20; telemetry's additional
memory checks remain separate. Independent export validation now passes six
combined-source graph/array captures across native interpreted/compiled and WASI
interpreted execution, eleven checker tests and a real recorder-overflow rejection.
Every successful invocation matches its checksum, releases all allocated objects
and leaves zero payload/catalog charge. The exact source, artifacts and raw
captures are retained in
`/home/brianheineman/development/ristretto-gc-evidence/integration-service-v1/summary.json`.
These checks ran alongside development work; their recorded times support no
performance conclusion. That earlier export has no canonical cycle accounting;
the schema-2 update described above now supplies primary-epoch bounds. Global
pause coverage remains incomplete.
These changes close no performance or endurance gate.


## Experimental canonical caller API (2026-11-15)

The opt-in `ristretto_gc::canonical` API exposes the existing typed graph operations
through transparent wrappers, preserving charged layouts and incremental scheduling.
686 native / 379 WASI regressions, external clients under four memory tools,
compiler lifetime/exclusive-request controls and native/bare-Wasm smoke runs pass.
No performance claim follows from exposing the interface. VM payload/layout and
execution integration, callbacks, allocation costs, compaction, JIT and all full
qualification gates remain open; defaults are unchanged. See
[the report](../../target/gc-implementation/canonical-client-REPORT.md) and
[contract](../../ristretto_gc/CANONICAL_API.md).

## Shared-root final-owner correction (2026-11-15)

The experimental shared-root owner now frees its Arc backing before releasing the
unique root and refunding catalog credit. A leaf/graph audit checks this exact final
alias boundary on native, WASI and four memory tools; restoring ordinary Arc Drop
reproduces the previous defect. The original evidence remains preserved.

All regression, ownership, accounting and 90 finite contention processes pass.
The fresh 308 timing processes and 66 allocator samples measure about 10.6 ns native /
3.0 ns WASI per owning copy/drop with zero copy allocation. Logical roots still add
one allocation, and native lifecycle capacities 8/64/256 regress about 0.8%/0.7%/1.3%
against the phase baseline. All results and graphs are in the
[corrected report](../../target/gc-implementation/canonical-shared-root-v2-REPORT.md).
The new audit feature is excluded from timing binaries. Defaults, VM integration,
callback, compaction, JIT and full qualification gates remain unchanged/open.
The older shared-root entry below is historical.

## Shared typed-root ownership (2026-11-15)

Experimental leaf/graph roots now share one logical token across value copies.
Native/WASI regression, affected memory, ownership and accounting checks pass, as
do all 90 finite contention processes. Clone/drop costs about 11.6 ns native and
3.15 ns WASI versus legacy 23.3 / 12.0 ns. Logical-root creation adds one allocation;
the unchanged full lifecycle regresses about 1.3%/1.2% at native capacities 64/256.
All 308 timing processes and 66 allocator samples remain in
[the report and graphs](../../target/gc-implementation/canonical-shared-root-REPORT.md).
This is a prerequisite for admitted callers, with allocation overhead and all VM,
callback, compaction, JIT and full qualification work still open. Defaults are
unchanged. See [the contract](../../ristretto_gc/CANONICAL_SHARED_ROOTS.md).

## One-slot phase decisions with bounded yields (2026-11-15)

The experimental catalog eliminates two backing reservations and the final head
replacement for effect-free phases. Two local facade yields preserve the existing
effect-bearing preparation window. The full scoped correctness matrix, 73 catalog
and six pool models, 68 intended negative controls and 90 finite contention processes
pass. The 2,112 accounting rows are unchanged. All 224 final paired timing processes
and 48 separate allocator samples validate; all capacities and regressions remain
in [the report and graphs](../../target/gc-implementation/canonical-phase-REPORT.md).
These are short lifecycle diagnostics. VM payloads/callers, callbacks, compaction, JIT and all full qualification gates remain open; defaults are unchanged.
See [the contract](../../ristretto_gc/CANONICAL_PHASE_DECISIONS.md).

## Bounded inventory cleanup and helped publication (2026-11-15)

Physical empty-slot scans now stop after at most eight observations, an occupied
slot or a failed observation. Helpers can finish announced allocation publication;
an exact-result retry fixes a native partial-effect race. All 90 final finite native/WASI stress processes pass (6,927,000 real objects).
The 13-command regression matrix, 16 memory commands and 17 intended fault processes
pass. Reused raw transition models and the bounded retry fixture have separate scopes.

All 224 paired timing processes and 48 separate allocator samples validate.
At capacity 256, paired lifecycle time ratios are 0.673 native and 0.771 WASI (lower is better). All capacities, regressions and intervals are retained in the report. Allocator request counts, requested bytes and peak bytes are unchanged.
See [the full report and graphs](../../target/gc-implementation/canonical-sweep-REPORT.md)
and [the contract](../../ristretto_gc/CANONICAL_SWEEP_BATCH.md). The rejected v2 results
remain separate. These checks do not close VM integration, callbacks, JIT, pause,
G1/ZGC or endurance qualification; defaults are unchanged.

## Source-indexed canonical traversal (2026-11-15)

The graph runtime now selects catalog-enforced source ranges. Publication rejects
wrong placement, and complete traversal visits D positions rather than all E. All
regression and affected memory checks, 68 catalog models and 12 intended fault
processes pass. Seven paired blocks per target contain 224 fresh timing processes;
48 separate allocator samples balance. At capacity 256, lifecycle time falls about
19% native and 11% WASI, with unchanged request counts. Sparse cleanup still consumes
about 94% of runtime. The capacity-2 WASI interval includes no change.

See [the paired report, raw data and standalone graphs](../../target/gc-implementation/canonical-indexed-REPORT.md)
and [the invariant](../../ristretto_gc/CANONICAL_INDEXED_EDGES.md). Remaining sweep/
controller costs, VM integration, callbacks, JIT and every full qualification gate
remain open; defaults are unchanged.

## Canonical graph runtime and cost screen (2026-11-15)

The optional internal runtime now allocates real primitive payloads with complete
catalog-owned graph edges, coupled writes, independently rooted reads and incremental
cyclic cleanup. The native/WASI regression and memory checks, 20 intended fault
processes and 44 CLI checks pass. The new `graph_lifecycle` runner freezes source/binary
provenance and validates complete raw timing/counting records; all 56 timing processes
and 24 allocator samples pass. Each invocation counts 4,056 allocations and releases
for 2,000 objects including setup/teardown/reporting. Sparse-catalog cleanup dominates:
at 256 slots it consumes about 95% of runtime. These are diagnostic medians, not
G1/ZGC comparisons or latency/endurance qualification. Defaults are unchanged.

See [the report and standalone plots](../../target/gc-implementation/canonical-graph-REPORT.md)
and [the representation/ownership contract](../../ristretto_gc/CANONICAL_GRAPH.md).
Source-indexed traversal, VM payloads/callers, callbacks, JIT and full-plan gates remain open.

## Canonical edge inventory (2026-11-15)

The tagged catalog adds non-owning strong edges with coupled deletion/insertion
barriers, exact child-root activation, source-owned lifetime and complete bounded
Full-domain traversal. Final catalog tests pass 223 native / 206 WASI; memory tools,
66 corrected catalog models, 42 distinct checker faults and seven portable edge
faults pass. The 840 accounting rows have zero lifecycle backing requests, but
larger layouts increase costs even when edge storage is disabled. Enabling E also
selects conservative reserves. Traversal currently scans O(E) slots per source.

These are correctness and metadata results, not G1/ZGC or latency/throughput
qualification. Real graph payload/caller integration and all full-plan gates remain
open, with defaults unchanged. See [the report and graphs](../../target/gc-implementation/canonical-edge-REPORT.md)
and [the protocol](../../ristretto_gc/CANONICAL_EDGES.md).

Current acceptance follows [PLAN.md](../../ristretto_gc/PLAN.md#success-criteria):
every observed native STW pause must be under 1 ms, including rendezvous and
equivalent heap-wide stalls. Zero STW is preferred. Sustainable whole-JVM throughput,
scheduled-arrival tail latency, compiled hot-path coverage, resource bounds,
correctness, reclamation progress and endurance still require qualification.
G1/ZGC ratios and fixed percentage improvements are not acceptance gates.

This delivery includes measured overhead reductions and reclamation-progress fixes;
the combined qualification remains incomplete. Historical sections and reports
retain their original criteria and results. `qualify.py` now defaults to a versioned
v2 evidence checker with diagnostic peer ratios, predeclared absolute limits and
an exclusive 1 ms native pause budget. The historical evaluator remains available
as v1. Runtime producers still lack complete pause and measured-window execution
packets, so existing runs cannot pass v2. Defaults remain unchanged.

## Controlled leaf contention (2026-11-15)

The new finite stress driver validates each actor's identities and work, actual
payload destruction, both physical slot-release counts and final logical credits.
All 78 final native/WASI processes pass, covering 31.11 million objects. The
15-command matrix, nine intended fault processes and 39 CLI checks also pass.
An initial fixed-watchdog failure at 16,384 slots is preserved; the revised,
capacity-scaled diagnostic still observes requests requiring millions of polls.
No collector speedup or latency/endurance gate is claimed. Default algorithms and
the transition protocol are unchanged. See [the report](../../target/gc-implementation/canonical-contention-REPORT.md)
and [the API/limits](../../ristretto_gc/CANONICAL_CONTENTION.md). Complete graph
inventory and admitted-caller integration are the next boundary.

## Bounded root-scan batching (2026-11-15)

The concrete cost screen identified cleanup as the dominant canonical cost. An
isolated count diagnostic found roughly two root-scan submissions per reclaimed
object. The helper now examines at most eight consecutive slots per scan, stopping
at the first root/target effect. No pooled storage, wrapper field or controller
commit protocol changes; single-slot APIs retain their behavior.

| Target | Canonical lifecycle after/before | Cleanup after/before | Canonical after time / legacy after |
|---|---:|---:|---:|
| Native | 0.808–0.819 | 0.756–0.761 | 3.65–4.31 |
| WASI | 0.792–0.811 | 0.730–0.755 | 2.10–2.20 |

Seven independent paired processes per mode/batch/target produce 252 accepted
million-operation samples. Every canonical lifecycle interval favors batching.
Helper calls fall from about 42–44 to 30–32 per object. Native legacy controls
remain near 1; WASI legacy also improves 1–2.5%, so not every observed difference
can be attributed solely to the changed path. Setup/teardown remain separate;
lifecycle includes actual batch reclamation and validation.

All 36 canonical counting rounds exactly match prior allocation/free requests,
bytes and peak requested storage; all 54 rounds balance at teardown. The 27-command
matrix, 59 catalog/six pool models, 32 checker faults and eight native/WASI fault
executions pass. This supports the internal optimization while preserving the large
remaining regression versus legacy. No full-GC/JIT/peer/endurance gate closes and
defaults remain unchanged. See
[the report](../../target/gc-implementation/canonical-cleanup-REPORT.md) and
[the protocol](../../ristretto_gc/CANONICAL_ROOT_BATCH.md).

## Concrete leaf lifecycle cost (2026-11-15)

The new controlled fixture includes real reclamation instead of comparing allocation
alone. Across seven independent process blocks and three batch sizes, canonical
lifecycle time is 4.53–5.30× legacy native and 2.61–2.72× WASI. Cleanup occupies 75–76%
of canonical time and requires about 42–44 helper polls per object; allocation needs
12 polls. The result directs the next optimization toward reclamation protocol cost.
It does not justify changing defaults or selecting the internal runtime publicly.

| Target | Legacy lifecycle ops/s | Canonical lifecycle ops/s | Canonical time / legacy |
|---|---:|---:|---:|
| Native | 1.83–2.16 million | 0.403–0.412 million | 4.53–5.30 |
| WASI | 1.58–1.71 million | 0.599–0.627 million | 2.61–2.72 |

The separate 54-round allocator audit records about 2.00 requests per canonical
payload versus 3.25–3.34 legacy, including amortized setup/teardown. All requested-byte
balances return to baseline. At batch 16384, canonical requested bytes per payload
are 421 native  / 307 WASI versus370 /246 legacy because catalog setup is included in
the 100000-operation audit. Logical catalog/wrapper reservation alone is 17,705,824
native  / 15,083,796 WASI bytes; this is not RSS or the 40-byte logical payload charge.
Timing uses one million operations, includes batch cleanup/validation and reports
setup/teardown separately. Comparisons are paired within target; cross-target values
are not a native-versus-Wasm execution claim.

All 14 frozen validation commands, six intended fault processes and 52 CLI checks
pass; all 126 timed samples pass integrity checks. The bare target compiles and
rejects this timer without a host clock. Sources, toolchains, binaries, raw rows,
exploratory paired 95% intervals and standalone CSV/SVG/PNG are recorded in
[the report](../../target/gc-implementation/canonical-leaf-cost-REPORT.md).
No graph/callback, admitted-caller, JIT, pause, peer or endurance gate closes.

## Current-source model runner (2026-11-15)

The checked-in tagged catalog runner now reproduces 55 catalog and six pool models
and 28 intended fault failures, including all 25 historical controls. It preflights
exact mutation anchors and restores standalone workspace roots in copied crates.
Distinct build directories prevent stale binary reuse across source variants.
Strict results and 15 Python tests reject incomplete/ignored/filtered suites,
diagnostic-string false passes, compile failures and unmatched fault witnesses.
All 30 executed cases passed their required checks; their binaries and hashes are
recorded. `--preflight-only` reports non-execution explicitly. Production dependencies,
collector behavior and defaults are unchanged. See
[the evidence report](../../target/gc-implementation/canonical-checker-REPORT.md).
This is model reproducibility, not a G1/ZGC performance or endurance qualification.

## Automatic leaf allocation retries (2026-11-15)

One internal allocation request now retains the exact payload through private
object/root announcement collisions and full/contended root quota. Each poll tries
one candidate; a full heap needs helper progress, root release or cancellation.
Published locations never move, and reserved reference roles cannot rotate. The
explicit-slot APIs preserve their error behavior.

The 39-command frozen matrix, 55 corrected models, 16 intended native/WASI fault
processes, two intended model faults and four compiler rejections pass. Leaf memory
tools each pass 60 fixtures; WASI release passes 51. Ten catalog fixtures, including the nine new cases, also
pass both interpreters and sanitizers. Five new typed fixtures cover real payload
retention, one-time preparation, helper-assisted progress, cancellation and reuse.

All 744 existing accounting rows keep their requested backing/allocation counts.
Participant hints add 16 native / 8 WASI logical bytes per quota. All 816 measured
lifecycle rows, including 96 forced-collision rows, request zero new backing.
Forty-eight production-layout rows keep the runtime-wrapper supplement unchanged.
The [storage graph](../../target/gc-implementation/canonical-retry-accounting-v1/retry-reservation-v2.svg)
shows this reservation cost; it does not measure RSS, payload preparation or time.

See [the protocol](../../ristretto_gc/CANONICAL_RETRY.md) and
[the evidence report](../../target/gc-implementation/canonical-retry-REPORT.md).
Public selection, scheduling, graphs, callback limits, admitted callers, JIT and
all GC comparison/endurance gates remain open. Defaults are unchanged.

## Heap-bound leaf runtime and wrapper reservation (2026-11-15)

An internal runtime owner now binds allocation, maintenance and typed reads to one
collector/catalog. Its private catalog and non-cloneable quotas bound owning
wrappers. A checked setup supplement reserves their storage through the existing
catalog charge; per-allocation/poll budget operations are unchanged. Roots and
participants retain the charge after runtime-owner drop, through heap shutdown and
through blocked actual payload destruction. Reads reject a foreign catalog even
when it shares the same collector.

All 35 frozen validation commands pass: native 567 default / 646 all-feature tests
(one existing ignore each), WASI 282 / 339, both GC/catalog Clippy targets, bare-Wasm
library/examples, both format checks, catalog 179 native / 165 WASI and pool 12 / 7
tests. All 55 leaf fixtures pass strict-provenance Miri, Tree Borrows, ASan and TSan;
release controls pass 55 native / 46 WASI. The two new generic supplemental-budget
fixtures also pass both Miri modes. The eight new leaf fixtures cover independent
layout census, budget rejection, quotas, exact-catalog reads, retained participants,
shutdown, and stopped helper/last-root destruction. Development's unnecessary
qualified import failed compilation and was corrected before freezing.

Four ownership/accounting mutations fail on both release targets; an overflow
mutation also fails both targets (ten intended fault processes). A positive
compiler control and eight expected rejections protect heap/view lifetimes,
private catalog/participant state, unique runtime ownership and exclusive requests.
The four integrated main checks pass. All 52 previously verified corrected models
are reused only for nine byte-identical transitive Rust inputs. Byte charges,
wrapper lifetimes and refund order are excluded from those models and are tested
concretely here; no new model execution is claimed.

All 672 previous catalog rows are unchanged. The 72 supplemental rows keep the
same requested backing and setup allocation count, add exactly the declared credit,
and refund after final free. All 720 catalog lifecycle rows request zero new
backing. Payload preparation and runtime-wrapper construction are outside those
numeric audit loops.

A temporary print-only harness, compiled without cfg(test), records 48 actual
production-layout rows on native and WASI. Seven mutators and two helpers add
656 native / 420 WASI logical reservation bytes. At 16,384 records and 64 roots,
these bring the respective totals to 5,835,824 / 5,043,684 bytes. These are requested
backing plus inline reservations, not RSS or performance results. The candidate
exposes no new public layout-audit API.

Public backend/package selection, automatic slot retry, scheduling, graphs,
conditional references with real payloads, callback bounds, admitted callers, JIT
and whole-GC qualification remain open. Borrowed frames, external owners, payloads
and physical-memory enforcement remain separately accountable. Defaults are unchanged.

[Protocol](../../ristretto_gc/CANONICAL_RUNTIME.md) ·
[Report and graph](../../target/gc-implementation/canonical-runtime-REPORT.md) ·
[Verification](../../target/gc-implementation/canonical-runtime-verification.json).

## Incremental canonical leaf maintenance (2026-11-15)

An internal reserved helper now rotates root, object and phase work, cleaning
abandoned preparations and dropped roots, acknowledging reference-free trace work,
retiring eligible garbage and releasing physical slots. Pending commands retain
preparation. Every completed collection requires a full consecutive object sweep
in its retirement epoch. A forced red schedule exposed a delayed Finish rebasing
into a newer unswept epoch; FinishEpoch now retains the original epoch through
preparation and retries. Raw trusted callers retain the unbound Finish API.

The frozen V1 31-command matrix passes: native 567 default / 638 all-feature tests
(one existing ignore each), WASI 282 / 333, GC and standalone-catalog Clippy on both
targets, bare-Wasm library/examples, formatting, catalog 177 native / 163 WASI and
pool 12 / 7 tests. All 47 scoped leaf fixtures pass strict-provenance Miri, Tree
Borrows, ASan and TSan; release controls pass 47 native / 40 WASI fixtures. The new
fixtures cover cancelled prefixes, complete sweep retirement, live roots, repeated
reuse, multiple helpers, helper replacement, retained revoked-root views, and
stopped allocation/destruction while another helper reclaims unrelated storage.

A mutation exposed a test gap: a reset fixture used literal floor 4 although the
retirement constant is 5. V2 corrects only tests and first establishes a certified
sweep. Its seven checks pass: both release targets, GC Clippy on both targets,
formatting, and the corrected fixture in both Miri modes. Another development
mutation disabled physical root cleanup but survived because logical root scanning
also processes removal requests; the final fault disables their shared durable
intent. Both surviving variants and the original red race remain recorded.

V3 changes only the concurrent epoch model. The broad finish/restart/advance model
hit its debug exploration deadline with 51 other models completed; its optimized
attempt is recorded separately. The replacement models the deciding final-advance
boundary, while the sequential stale-preparation case executes the complete
finish/restart sequence. All 52 current-source corrected-checker models pass,
without added branch or preemption limits. Both new models catch the omitted epoch
guard. Models exclude real pooled pointer protection, the complete facade state
machine, concrete typed views and capacities. V3 catalog Clippy on both targets
and formatting pass. Source patches delimit reuse of unchanged production inputs.

Nine runtime fault variants fail their intended assertions on native and WASI
release builds (18 processes). A positive compiler control and four expected
rejections protect heap lifetime, participant encapsulation and unique helper
ownership. Four integrated main checks pass. The 672 catalog accounting rows are
exactly unchanged from the allocation stage; all 648 lifecycle rows request zero
new backing and all refunds follow final free. Maintenance wrapper storage,
concrete payload allocation, RSS and execution time are outside those rows.

Public heap binding and runtime accounting, automatic slot selection, scheduling,
graphs, conditional references with real payloads, callback bounds, admitted callers,
JIT and whole-GC qualification remain open. Bounded catalog work per poll does not
bound arbitrary Drop/allocator time or prove starvation freedom under unbounded
contention. Defaults remain unchanged.

[Protocol](../../ristretto_gc/CANONICAL_MAINTENANCE.md) ·
[Report](../../target/gc-implementation/canonical-maintenance-REPORT.md) ·
[Verification](../../target/gc-implementation/canonical-maintenance-verification.json).

## Canonical leaf allocation transactions (2026-11-14)

An internal allocation request now connects real descriptor preparation to coupled
canonical root publication and a unique typed result. One participant retains the
whole transaction. Cancelled announced records carry durable abandonment intent;
helpers clean only exact Prepared membership through the controller. Discarding a
result after commit wins leaves publication helpable and requests root removal.
A stopped origin does not prevent the exercised unrelated payload reclamation.

The frozen V1 28-command matrix passes: native 567 default / 625 all-feature tests
(one existing ignore each), WASI 282 / 322, GC Clippy on both targets, bare-Wasm
library/examples, formatting, standalone catalog/pool units, 34 leaf fixtures under
strict-provenance Miri, Tree Borrows, ASan and TSan, and release controls with
34 native / 29 WASI fixtures. V3 changes only a new test's nested-if formatting and
a pre-existing native test-hook cfg. Its seven standalone checks pass, including
177 native / 163 WASI catalog tests, both Clippy configurations, formatting, and
the two new catalog fixtures under both memory interpreters. The four integrated
main checks pass. Explicit reuse records separate test-only changes from V1 inputs.

All 50 current-source corrected catalog models pass, including abandonment versus
commit and a late intent after slot reuse. Two model mutations fail as intended.
Six runtime mutations fail their intended assertions on native and WASI release
(12 processes). A compiler positive control and seven expected rejections protect
participant/heap borrows, typed-result ownership and view lifetime. Development
setup/lint failures, the inactive-domain cleanup fixture correction, and the first
model-fault path-parser failure remain recorded. Models do not cover pooled pointer
protection, the complete facade state machine, concrete typed views or capacities.

The 672 allocator rows include 144 new catalog-only allocation-transaction rows;
all exercised lifecycle allocation counts remain zero. Real payload preparation
is outside those catalog loops. Fresh baseline comparison across 144 rooted rows
finds no setup-storage change for byte/aligned records. Large-record logical setup
storage increases by up to 1.205% native / 0.498% WASI. At N=16,384, K=8, M=64,
512-byte records add 262,528 native bytes and 448 WASI bytes. Requested backing and
inline reservations are measured, not RSS, throughput or latency.

Public backend selection, automatic slot selection, fair helper maintenance,
general graphs, callback bounds, admitted callers, JIT and every whole-GC
qualification gate remain open. Defaults are unchanged.

[Protocol](../../ristretto_gc/CANONICAL_ALLOCATION.md) ·
[Report and graph](../../target/gc-implementation/canonical-allocation-REPORT.md).

## Canonical leaf root access (2026-11-14)

The optional leaf adapter now provides internal typed views tied to an exact live
root, its admitted record and the matching heap. An acquire publication check and
immutable TypeId check precede pointer recovery. Views retain existing borrowed
owners and acquire no independent pool share. New reads observing a revoked root
fail; an already admitted view can survive retirement and slot reuse while a
reserved helper reclaims unrelated allocations.

The fresh frozen 18-command matrix passes: full native 567 default / 613 all-feature
(one existing ignore each), full WASI 282 / 311, both Clippy configurations,
bare-Wasm library/examples, formatting, standalone catalog/pool unit suites,
22 leaf fixtures under each scoped Miri/Tree/ASan/TSan run, and release controls
with 22 native / 18 WASI fixtures. The six access fixtures also pass strict-provenance
Miri. The 48 current-source catalog models pass with the recorded corrected checker;
they do not model concrete typed pointers or view lifetimes.

Four access-check mutations fail their intended assertions on both release targets.
An integer-reconstructed payload pointer fails strict-provenance Miri. A compiler
positive control and six rejections protect root/heap/view lifetimes. The earlier
development runs retain their lint/fixture failures and two excluded stale/baseline
executions after a shared target-directory mistake. Final validation uses a fresh
build directory and asserts the expected access-fixture counts.

No allocation-count, throughput, latency or RSS improvement is claimed. Public
allocation/result transfer, graphs, callbacks, fair maintenance, admitted callers,
JIT and all qualification gates remain open; defaults are unchanged.

[Protocol](../../ristretto_gc/CANONICAL_LEAVES.md#typed-reads-through-canonical-roots) ·
[Report](../../target/gc-implementation/canonical-leaf-access-REPORT.md).

## Concrete canonical leaf ownership (2026-11-14)

An optional internal adapter transfers an eligible private allocation into one
canonical record without entering legacy indexes. Its borrowed guard validates the
exact heap and retains the sole cleanup owner. Sixteen native and 13 WASI fixtures
cover eligibility, cancellation, collisions, foreign admission, stopped helpers,
stopped actual payload destruction, unrelated reclamation, slot reuse and refund.
Public Gc allocation/result handling and collector selection remain unchanged.

Frozen V2 full suites pass 567 default / 606 all-feature native tests (one existing
ignore each), and 282 default / 305 all-feature WASI tests. Both Clippy configurations,
bare-Wasm library/examples, formatting and standalone catalog/pool checks pass.
V3 adds only the independent opaque-Trace rejection fixture; its 16 native / 13 WASI focused debug
and release controls, both Clippy configurations and formatting pass. Each scoped
Miri, Tree Borrows, ASan and TSan run passes all 16 fixtures with only the new feature
enabled. V2/V3 all-feature Miri attempts were stopped and remain incomplete.

Eight release fault variants fail their intended assertions on both targets; the
first WASI fault parser rejected panic-abort output and is retained alongside the
corrected runner. A positive compiler control and six rejections protect guard
lifetimes, unique ownership and private metadata/certificates. The 48 prior corrected
catalog models are reused only after checking exact included source and the
checker-specific lock override. The incidental uncorrected standalone model runs
are excluded from model claims. No model covers concrete payload admission here.

No timing, allocation-count, RSS or whole-GC performance improvement is claimed.
Graph/root-result integration, arbitrary callbacks, packaging, admitted callers,
JIT and every qualification gate remain open. Defaults are unchanged.

[Protocol](../../ristretto_gc/CANONICAL_LEAVES.md) ·
[Report](../../target/gc-implementation/canonical-leaf-REPORT.md).

## Rooted object-array projections permit unrelated collection

The optional certificate now covers a projected parent whose direct strong fields
stay independently rooted and which needs no other trace actions. Object-array
slice views qualify; their Value elements preserve ownership under safe replacement,
cloning and extraction. Conditional target objects keep their separate tracing.
Whole-value writers and arbitrary callbacks remain opaque. The original guard
revokes before demotion and barriers; layout, pinning and defaults are unchanged.

A frozen prior-runtime fixture fails held-array progress. All 25 validation commands
pass: native GC 503/527, WASI 250/261, classloader 769/775 with 14 existing ignores,
four VM GC/reference and one array-access integration. Miri and Tree Borrows pass
five new collector fixtures across 12 configurations without an overlay. ASan/TSan
each pass 18 collector and four classloader cases. WASI runs four rooted-array,
two primitive-projection and nine ownership cases. Native/WASI Clippy, bare-Wasm,
benchmark harness, format and diff checks pass. Two runtime and two model negative
controls fail their intended assertions.

Stock Loom 0.7.2 skips one new composition model because its RMW/store atomicity bug
produces a false post-join result. The unchanged state and all six models pass with
the recorded unmerged [upstream checker fix](https://github.com/tokio-rs/loom/pull/391).
`benchmarks/gc/trace_certificate_check.py` reproduces this isolated check, including
the skipped regression; workspace dependencies and production atomics are unchanged.
The model assumes the root barrier protocol in one active epoch; it is not a proof
of admission or phase closure.

Eight fresh builds reproduce default native/WASI binaries byte for byte. All 1,344
feature-before/after timing processes pass, with unchanged cell/guard sizes.
Native one-element object-array slice borrowing costs about 1.7–2.4% more; the compact/context interval includes no change. Native length-32 slice ratios remain approximately 1.000–1.001 with intervals spanning one. WASI is mixed: gated one-element slices rise 4.0% and counted-host length-32 slices rise 1.4%, while other slice intervals include one. Most controls are near one, with retained shifts including +3.3% native counted-host length-32 element borrowing and +2.1% WASI gated length-32 primitive loads. These controls do not execute the newly certified array mapping, so their shifts cannot be assigned to its certificate stores. The delivery establishes scoped stalled-writer progress with measured costs, not overall throughput or latency improvement.

The feature remains disabled. Linear synchronous slice rooting/demotion, general
writer/reader/callback progress, D/E and all qualification gates remain open.

[Protocol](../../ristretto_gc/PUBLICATION.md#rooted-object-array-projections-2026-11-14) ·
[Report and graphs](../../target/gc-implementation/rooted-array-REPORT.md) ·
[Verification](../../target/gc-implementation/rooted-array-verification.json).

## Borrowed allocation attachment context

Attachment borrows the collector's immutable weak reference instead of cloning and
dropping a temporary weak. Retained hooks still clone their context. Native frozen
disassembly confirms the temporary increment/decrement pair is absent. Public
ownership contracts, registrations, admission and defaults are unchanged.

All 20 checks pass: native GC 502/521, actual WASI 250/258, classloader 771 passed/14
ignored, nine WASI ownership, four VM GC/reference and 19 context tests. Miri and
Tree Borrows each pass six construction fixtures, including reentrant, stalled and
unwinding attachment. ASan/TSan each pass 15 construction/ownership and 26 publication
fixtures. Native/WASI Clippy, bare-Wasm, format and diff checks pass.

All 1,008 paired timing processes pass. Fresh baseline builds reproduce the prior
foreign-guard binaries. Native allocation time falls 1.7–2.3% and managed-cell
allocation 1.4–2.7%; WASI allocation falls about 2.1% in two policies and stays flat
in the third, while managed-cell allocation falls 1.2–2.5%. Control shifts and small
regressions are retained with exploratory intervals; no overall throughput or
latency claim follows. No pause, D/E, peer, capacity or endurance gate is closed.

[Report and graphs](../../target/gc-implementation/borrowed-owner-REPORT.md) ·
[Verification](../../target/gc-implementation/borrowed-owner-verification.json).

## Root guards validate their allocation heap

Creating a root guard in a foreign collector could replace its source heap's root
during guard transfer. Guard creation now checks the associated collector before
registration; `Gc::as_root` returns a mismatch error and the infallible constructor
panics before registering. The check adds no weak clone, allocation, lock or atomic
operation. Four regressions cover rejection/cleanup, valid conversions and foreign
replacement of a guard's inner handle.

All 21 checks pass, including native/WASI, VM, sanitizers and Miri/Tree Borrows with
identical compiled-source hashes. All 1,008 timing processes pass; fresh baseline
builds reproduce the prior binaries. Most ratios remain near one, with about +1.2%
native candidate allocation, +1.5% WASI candidate cell allocation and a +5.6% WASI
leaf-write control. Full intervals, diagnostics, wall-time and raw evidence remain
in the report. This closes the preceding foreign-guard defect, while publication,
general writer progress and qualification remain open.

[Report and graphs](../../target/gc-implementation/foreign-guard-REPORT.md) ·
[Verification](../../target/gc-implementation/foreign-guard-verification.json).

## Owner-aware tracing and nested-cell ownership

Three ownership losses are repaired: foreign strong fields losing source roots,
nested container cells missing mutation hooks, and moved cells retaining hooks for
their former owner. `Trace` now requires unsafe implementation and owner-aware
attachment/demotion, with paired rooting before mutable escape. Same-heap Java
conditional objects rebind their queue owner without strengthening their referent.
All 21 checks pass, including native/WASI, VM integrations, Miri, Tree Borrows and
sanitizers; four compile-fail examples enforce the newly unsafe boundaries.

The 1,008-process paired screen keeps matching operation bodies, with only unsafe
syntax adapted for the old API. Native ordinary controls are broadly flat. Nested
writes rise from about 15 ns to 223–251 ns natively and 18 ns to 166–175 ns on WASI:
the old path omitted owning preparation and barriers. Small mixed WASI control
changes and the full intervals remain in the report. This is a correctness cost,
not a general throughput improvement or qualification result.

Conservative owning containers/cross-heap cycles and cross-heap Java reference queue
delivery remain limits. A separate foreign-guard mismatch was reproduced against
this snapshot; its repair is tracked next. Canonical publication, general writer
progress, JIT/pause evidence and endurance remain open.

[Report and graphs](../../target/gc-implementation/owner-binding-REPORT.md) ·
[Verification](../../target/gc-implementation/owner-binding-verification.json).

## Manual root removal and explicit revocation

Numeric root removal now affects only manual registrations, and all-registration
revocation requires unsafe code. A kind bit in the existing generation field
preserves entry/token sizes and stale-removal protection. A frozen prior-runtime
fixture reproduces safe numeric removal invalidating an owning handle. All 20
final validation checks pass across native/WASI, VM integrations, Miri, Tree
Borrows and sanitizers.

The 672-process paired comparison uses identical benchmark sources; fresh before
builds reproduce the preceding binaries byte-for-byte. Native root acquisition
rises 1.7–3.2%, with smaller allocation/pending-transfer changes and flat shared
clones. WASI changes are mostly mixed/within intervals. All raw data, diagnostics
and wall-time graphs, frozen sources and the negative fixture are preserved.
This is a correctness repair with measured costs; admission, general writer
progress and qualification remain open. Demotion/Trace contracts are the next
ownership audit boundary.

[Report and graphs](../../target/gc-implementation/root-revocation-REPORT.md) ·
[Verification](../../target/gc-implementation/root-revocation-verification.json).

## Owning generic clones and guard escapes

Generic clones of borrowed `Gc` fields now acquire roots. Owning handles and
`GcRootGuard::clone_gc` share registrations; guard Clone retains one registration,
and mutable guard exposure roots its inner handle before movement. Exact identity
and logical liveness checks under admission prevent stale edge resurrection.
Three frozen prior-runtime regressions reproduce the lost graphs. Twenty final
validation checks pass across native/WASI, VM integrations, Miri, Tree Borrows,
ASan and TSan. This does not close the remaining ownership-contract audit or D/E.

The 1,008-process paired screen preserves actual registration work by borrowing
an edge in the same benchmark on both revisions. Native root acquisition costs
8.5–10.6% more. Shared owning clones include 4.7–8.0% native regressions (one interval
includes one); WASI changes are smaller/mixed. Edge copies cost 4.3–5.1× native /
6.0–6.6× WASI and overwrite 1.60–1.71× because escaping handles now own their roots.
Those comparisons measure the correctness cost, not equally safe alternatives.
All processes pass checksums and zero-object teardown. Raw data, diagnostics-off/on
and wall-time graphs, frozen source/binaries, interrupted attempts and negative
fixtures are preserved. No qualification gate is closed.

[Report and graphs](../../target/gc-implementation/edge-clone-REPORT.md) ·
[Verification](../../target/gc-implementation/edge-clone-verification.json).

## Scoped primitive projections permit unrelated collection

The optional `experimental-trace-projections` feature lets old, young and full
tracing finish while a typed primitive-array view is held. Its certificate requires
the entire parent to remain free of tracing actions. The parent writer publishes
after successful mapping and revokes before cleanup/unlock; each scan uses a fresh
atomic CAS. Nested mapping, failure and unwind retain that lifetime. Whole-value,
object and reference-array writers retain their ordinary graph requirements, and
all projections still pin storage against relocation.

Native/WASI core and classloader matrices, four production-state Loom models, VM
reference/array integrations and sanitizers pass. Regular Miri passes four fixtures.
The full Tree Borrows run times out; a separately frozen test-only overlay with
8 repeated busy probes instead of 1,000 passes all 12 settings with identical
production code. The timeout is retained. Two negative mutations reject missing
revocation and certification. This is a scoped progress result, not complete D/E.

The 1,344-process paired screen compares feature off/on in the same source.
Default binaries reproduce the prior runtime byte-for-byte with the same expanded
benchmark. Cells grow by 8 bytes natively and 4 on WASI, with unchanged guard sizes.
At length 32, native typed borrow medians increase 6–9% and primitive byte views
9–19%. WASI primitive changes are smaller/mixed. Reference controls also include
regressions: gated native point borrowing rises 3.2% and counted/compact/context
WASI point borrowing 7.0%. Full paired intervals, controls and layout rows are in
the report; intervals containing one do not establish equivalence.

Aggregate operation loops exclude collection, setup, validation, startup and
Wasmtime compilation. No builds, tests, counters, edits or Miri overlap timing.
There is no application throughput/latency, pause, physical-memory, capacity,
G1/ZGC or endurance qualification claim. The feature stays disabled by default.

[Report and graphs](../../target/gc-implementation/trace-projection-REPORT.md) ·
[Verification](../../target/gc-implementation/trace-projection-verification.json).

## Whole-value ownership and narrow managed mutations

The earlier element-extraction repair did not cover moving an entire `Reference`
through its managed writer. That movement could lose a reachable transitive graph.
Managed guards now restore owning strong handles through `Trace::root` before
unrestricted mutable access or mapping; ownership follows the moved fields and
cleanup demotes those left inside. External `Trace` implementations that demote
fields must restore them in `root`. Conditional fields retain their semantics.

This repair initially made point writes acquire all fields, costing 37–39× more
on native and 46–67× more on WASI for the 32-field benchmark. The preserved initial
672-process screen records that regression. Safe typed guard operations now avoid
unrelated root acquisition for field/array replacement and root only the selected
value for point borrowing. Interpreter and experimental compiled field stores,
shared `aastore` and multidimensional initialization use them. Java checks,
barriers and async boundaries remain intact.

The final 1,344-process screen compares the migrated callers against both the
original and the initial whole-value repair, with seven randomized pairs for
four operations, two lengths, three policies and both targets. Gated 32-field
stores return to about 94 ns/op native and 48 ns/op WASI, versus approximately
97/48 ns originally and 3,602/3,151 ns in the initial repair. Native point-borrow
medians at length 32 are 0.5–1.2% above original; WASI medians are 2.7–4.0% below.
Intervals and all controls are retained; many overlap one. Gated slices remain
slightly slower than original. These are aggregate operation costs on a retained
fixed graph, including caller migration, without timed collection. They establish
no whole-JVM, pause, capacity, memory, G1/ZGC or endurance result.

Native/WASI ownership matrices, GC/classloader tests, VM field/array semantics,
Miri and sanitizers pass; separate negative mutations reproduce both ownership
failures. Whole-value and slice acquisition remain synchronous and linear. Other
unrestricted native/embedding borrows retain that cost. This fixes a prerequisite
for graph-progress work; arbitrary held writers and admitted publication still
prevent complete D/E progress.

[Report and all graphs](../../target/gc-implementation/narrow-mutation-REPORT.md) ·
[Verification](../../target/gc-implementation/narrow-mutation-verification.json).

## Shared identity insertion: late helpers cannot replay retirement

The optional index now has shared insertion intents with immutable retry proposals
and unique bucket versions. Another helper can complete a stopped insertion;
retirement permanently invalidates earlier insertion CAS attempts. Fourteen native
index tests, eleven shared-state Loom models, native/WASI integration suites, Miri
under both aliasing models, ASan and TSan pass. Mutations removing the replacement
certificate or expected-header check fail their forced schedules. The collector
still uses its existing admission and authoritative object map.

All 1,344 processes in two frozen paired screens pass. Against the exclusive marker
revision, diagnostics-off native allocation costs rise 25–27%, wall costs 13–14%,
and handles 1–3%. WASI allocation rises about 11%, wall costs 6–7%, while handles
fall 8–11%. Collection results are mixed. Native allocation costs remain about twice
the default map and wall costs about 2.5 times the default; WASI allocation is
29–32% more expensive. The feature stays disabled. This is a protocol prerequisite,
not an overhead improvement or completed stage-D gate. Separate allocation counters
and all paired intervals are preserved. Default binaries remain byte-identical.

[Reports and graphs](../../target/gc-implementation/identity-help-REPORT.md) ·
[Native default comparison](../../target/gc-implementation/identity-help-default-native-mechanisms-screen/mechanisms-diagnostics-False.svg) ·
[WASI default comparison](../../target/gc-implementation/identity-help-default-wasm-mechanisms-screen/mechanisms-diagnostics-False.svg) ·
[Validation](../../ristretto_gc/VALIDATION.md#shared-identity-insertion-intents-2026-11-13).

## Embedded identity links: fewer allocations, default still rejected

The optional identity index now embeds link-state markers in its owning nodes.
It removes three publication allocations per object and all index-owned collection
allocations from the initial prototype. Seven shared-state Loom models and native,
WASI, Miri, ASan and TSan checks cover exact-link deletion, safe head-return cases
and the private owning-pointer adapter. The default binaries are byte-for-byte
identical to the previous delivery; the feature remains disabled.

Two frozen matrices contain 1,344 successful processes with seven independent
pairs per condition. Native allocation operations improve 3–4% against the initial
prototype, while handles regress 3–5%; WASI allocation changes are small/mixed and
handles regress 9–12%. Cooperative full-graph collection improves 5–9% native and
10–15% WASI. Against the default map, native allocation operations still cost
57–60% more, allocation wall time 118–119% more and handles 36–38% more. WASI
allocation and handles also regress, although its full-graph collection improves
6–8%. Diagnostics-off ratios are exploratory, with unadjusted process-pair
intervals. Counter-only forks and timing forks are separate. These results justify
continuing representation/publication work, not a default change or a pause claim.

[Results, all matrices and counter data](../../target/gc-implementation/identity-markers-REPORT.md) ·
[Native default comparison](../../target/gc-implementation/identity-markers-default-native-mechanisms-screen/mechanisms-diagnostics-False.svg) ·
[WASI default comparison](../../target/gc-implementation/identity-markers-default-wasm-mechanisms-screen/mechanisms-diagnostics-False.svg) ·
[Validation and limits](../../ristretto_gc/VALIDATION.md#embedded-identity-link-markers-2026-11-13).

## Initial owning identity index: useful protocol, rejected default

The disabled-by-default `ristretto_gc/experimental-identity-index` feature adds
owned identity buckets, exact-link retirement, resumable sweep removal and
cancellation-safe maintenance. It removes that index's mutex dependency while
leaving canonical object membership and publication admission unchanged. Five
production-state Loom models, native/WASI integration checks, Miri and TSan support
the scoped ownership protocol; a rejected stale-predecessor mutation demonstrates
the new model schedule's failure detection.

Three frozen paired screens contain 2,016 validated processes. The retained feature
versus restored default costs 63–70% more native allocation time and 119–125% more
allocation wall time with diagnostics off. Handle costs rise 26–29%. WASI allocation
costs rise 15–19%, wall costs 44–48%. Separate allocation counters find approximately
four extra system allocation requests per object plus 1.5904 per object during
collection in the 10,000-object fixtures. Default maps remain in use; this is not
an overhead improvement or qualification milestone. Canonical membership work must
reduce/replace this separate representation before proposing a default migration.

[Complete reports, graphs and limits](../../target/gc-implementation/identity-index-REPORT.md) ·
[Native graph](../../target/gc-implementation/identity-index-retained-native-mechanisms/mechanisms-diagnostics-False.svg) ·
[WASI graph](../../target/gc-implementation/identity-index-retained-wasm-mechanisms/mechanisms-diagnostics-False.svg).

## Owned append records and invocation-sensitive collection costs

The fallback tracing queue now has prepared append records that concurrent helpers
can complete exactly once. Cancellation, consumption and late replay retain their
ownership/counting rules. This is a stage-D prerequisite: current collector calls
keep the records local and retain admission. Canonical descriptor membership,
record discovery, phase-coupled acknowledgement and E's inaccessible graph problem
remain open. [Validation](../../ristretto_gc/VALIDATION.md#owned-append-records-and-invocation-sensitive-collection-costs)
records the production-state models, forced schedules, portable checks and sanitizers.

The initial collection screen suggested large native worker improvements and a
WASI regression. A 63-process control then changed only the native invocation-name
length (11/102/108 bytes) for each frozen binary. The outlined candidate's ratio to
the baseline changed from 1.181 [1.111, 1.195] to 0.808 [0.795, 0.838] and 1.104
[1.074, 1.121] in the counted, four-worker wide fixture. These intervals support
different results for those particular invocations, not a general improvement.
The fixture retains its argument strings; exact allocator/cache attribution is
unproven. The driver now passes `argv[0]=mark_cycles` on both targets, independently
of result-directory or binary names. A real-process harness test checks this.

Sixteen diagnostic forks reported zero fallback appends in cooperative tree/wide
cases. Outlining the append function improved the initial WASI cost. Separately
aligning its counters did not fix native sensitivity and was removed. Aligning the
native queue header reduced the candidate's absolute variation across the three
tested invocation lengths, but did not establish general layout independence.
Choosing the first append predecessor privately avoids an unnecessary atomic
selection. Its cooperative timing changes cannot be attributed to executed append
operations; code generation/layout effects remain relevant. All variants are retained.

The final comparison uses separate fresh Cargo target directories, matching
fixtures/toolchains, seven independent process pairs per condition, 8,191 nodes,
three warmup collections and 32 measured full collections. Native runs use zero
or four workers on fixed caller/worker CPUs; WASI uses zero. Both publication
policies and diagnostics modes are included. All 336 final processes validate
work, contents and teardown; frozen binaries/runner remain unchanged.

Selected retained after/before collection-time ratios (95% paired intervals):

| Target and workload | Workers | Diagnostics | Ratio |
|---|---:|---|---|
| Native gated tree | 0 | off | 0.941 [0.929, 0.959] |
| Native counted wide | 0 | off | 0.943 [0.934, 0.948] |
| Native counted tree | 4 | off | 1.041 [1.029, 1.048] |
| Native gated wide | 4 | off | 1.073 [1.026, 1.084] |
| Native counted wide | 4 | on | 0.976 [0.969, 1.007] |
| WASI gated tree | 0 | off | 1.060 [1.051, 1.081] |
| WASI gated wide | 0 | off | 1.080 [1.050, 1.095] |
| WASI counted wide | 0 | off | 1.063 [1.042, 1.072] |

The [complete native report](../../target/gc-implementation/helpable-append-retained-native-screen/REPORT.md)
and [WASI report](../../target/gc-implementation/helpable-append-retained-wasm-screen/REPORT.md)
include every condition. Exported graphs are available as
[native SVG](../../target/gc-implementation/helpable-append-retained-native-screen/mark-cycles.svg),
[native PNG](../../target/gc-implementation/helpable-append-retained-native-screen/mark-cycles.png),
[WASI SVG](../../target/gc-implementation/helpable-append-retained-wasm-screen/mark-cycles.svg) and
[WASI PNG](../../target/gc-implementation/helpable-append-retained-wasm-screen/mark-cycles.png).
The [verification record](../../target/gc-implementation/helpable-append-verification.json)
binds all six paired screens (2,016 processes), three layout controls (189 processes),
16 diagnostic forks, source archives, build commands and checks. Earlier screens
retain their original invocation policy and must not be pooled with normalized runs.

The implementation is retained for the durable-work protocol prerequisite.
Multiworker and WASI regressions remain open. These exploratory full-collection
costs do not establish sustainable application capacity, STW pauses, peer-relative
performance or endurance qualification; no default-policy change follows.

## Typed exception matching leaves generated code before waiting

Experimental typed-handler lookup now yields an owning request per table entry.
The request retains the throwing BCI and a distinct continuation for duplicate
catch types, shares ordinary request fuel, and preserves live locals and the
pending throwable. Loader waits and payload contention can yield with no generated
frame retained. Resolution failures propagate out of the method instead of entering
a later catch-all. Native missing catch classes now become Java linkage errors
in interpreter and compiled paths, including cached compiled resolution failures.

The eight-configuration matrix passes 16 held-loader schedules with full collection,
migration and cancellation/completion, plus 16 held-writer request schedules. Held
writers still prevent full tracing; this change does not complete collector D/E.
Interpreter, resumed and synchronous compiled results agree on the covered cases.
Seven exact exported classes pass 17 HotSpot checks each under `-Xint` and `-Xcomp`
with verification enabled; the [oracle](ExceptionMatchOracle.java) checks semantics.
See [validation](../../ristretto_gc/VALIDATION.md#owning-typed-exception-matching-2026-11-09)
and the [frozen evidence](../../target/gc-implementation/exception-request-verification.json).
No new performance graph is claimed. Request/compilation costs, production handler
eligibility, remaining helper cooperation and pause/capacity/endurance gates remain
open, with defaults unchanged.

## Helpable publication requires atomic effects, not early acknowledgement

Eleven design models now distinguish four rejected publication shortcuts from
seven reduced atomic-commit properties. A completed bit cannot prevent a helper
which previously read it from replaying stale mark/index mutations. Marking does
not prove its traversal is queued, and atomic per-object membership alone does
not coordinate phase closure. These witnesses guide the next canonical descriptor
membership and catalog/phase work. They do not change the runtime or demonstrate
reclamation progress in the collector. Existing 28 admission models still pass.

See the [design and limits](../../ristretto_gc/validation/HELPABLE_PUBLICATION.md)
and [frozen evidence](../../target/gc-implementation/helpable-publication-verification.json).
There is no timing comparison or new performance graph for these model-only changes;
D/E, native/Wasm runtime qualification and sustained-load gates remain open.

## Reference constants resolve through owning requests

Experimental compiled string/class `ldc` and `ldc_w` now leave the generated frame
before awaiting resolution, interning and mirror allocation. Their context owns
locals, residual operands and returned identities across suspension. Numeric
constants retain inline lowering. A shared interpreter/runtime resolver preserves
interned strings, UTF-16 code units and class mirrors without target initialization;
native missing-class literals now produce the correct linkage error.

The eight-configuration live-VM matrix passes 64 forced loader waits with migration,
cancellation and unrelated reclamation, plus interpreted/resumed/synchronous semantic
comparisons and ready-loop cooperation. Exact exported classes pass 24 HotSpot
checks under each of `-Xint` and `-Xcomp`; 13 portable constant tests pass on WASI.
See [validation](../../ristretto_gc/VALIDATION.md#owning-reference-constants-2026-11-09)
and the [frozen verification record](../../target/gc-implementation/constant-request-verification.json).
The [oracle](ConstantOracle.java) checks semantics, not GC speed. No new performance
graph is claimed for this capability change; request overhead, broader resolution
semantics, production eligibility, D/E and J5/H qualification remain open.

## Mutable array extraction retains owning roots

Historical element-only repair; the whole-value follow-up above supersedes its
ownership boundary and adds narrow managed-guard operations.

Moving a value out through a mutable array element previously left an unrooted
local that could be reclaimed after its source disappeared. Mutable elements,
slices and consuming iterators now acquire roots before exposing values. Guard
cleanup demotes values left inside. `ArrayElements::set` preserves replacement
barriers without rooting discarded edges; Java field stores, `aastore` and
multidimensional initialization use it. The 168-case native/WASI extraction matrix,
iterator lifetime, unwind and cancellation checks pass. This does not permit
collection past a held whole-value writer; the general D/E protocol is still open.

The corrected cost screen has 672 successful processes, seven randomized pairs per
condition, with frozen sources/binaries and one child at a time on CPU 0. It compares
four operations, two lengths and three policies on native and WASI. Timings cover
the operation loop on a fixed live graph; setup, collection, validation, process
startup and Wasmtime compilation are excluded. The old binary lacks the extraction
ownership guarantee; its source stays alive in this cost control. No whole-JVM,
latency, sustainable-capacity, memory-ceiling or endurance claim follows.

Representative gated-host results at length 32 are below. Ratios above one mean
more time. Intervals are exploratory paired-bootstrap 95% intervals, unadjusted
for multiple comparisons; the linked reports retain every policy and control.

| Target | Operation | Before ns/op | After ns/op | Ratio [95% CI] |
| --- | --- | ---: | ---: | ---: |
| Native | Field store | 95.8 | 94.8 | 0.992 [0.987, 1.003] |
| Native | Element extraction | 97.4 | 180.4 | 1.840 [1.807, 1.860] |
| Native | Mutable reference slice | 105.1 | 3,621.2 | 34.595 [33.668, 34.808] |
| Native | Read control | 121.8 | 121.8 | 1.003 [0.997, 1.014] |
| WASI | Field store | 49.6 | 48.2 | 0.971 [0.947, 0.975] |
| WASI | Element extraction | 48.1 | 130.6 | 2.709 [2.688, 2.741] |
| WASI | Mutable reference slice | 94.9 | 3,142.5 | 33.311 [32.038, 35.538] |
| WASI | Read control | 104.1 | 103.9 | 0.997 [0.992, 1.003] |

Mutable borrowing has a substantial correctness cost, especially when a slice
exposes many references. This is retained as an ownership repair, with cheaper
bulk ownership and bounded cooperative access still required. Replacement-only
field stores avoid that cost; intervals containing one do not establish parity.
Native and Wasm work retain separate contracts, and no defaults changed.

The first pair of screens is invalid: a shared Cargo target directory supplied
identical before/after binaries. Their 672 results and invalidation records remain
available. A fresh candidate target directory fixes that source/binary mismatch,
and the harness now rejects identical inputs. Only these corrected artifacts are
used above: [native report](../../target/gc-implementation/edge-extraction-native-screen-2/REPORT.md),
[native graph](../../target/gc-implementation/edge-extraction-native-screen-2/mutation-ownership.svg),
[WASI report](../../target/gc-implementation/edge-extraction-wasi-screen-2/REPORT.md),
[WASI graph](../../target/gc-implementation/edge-extraction-wasi-screen-2/mutation-ownership.svg),
and [verification record](../../target/gc-implementation/edge-extraction-verification.json).

## Array accesses yield on contention and release guards before errors

Experimental primitive/reference loads, primitive stores and `arraylength` now
return owning requests. Runtime lock contention yields, error construction runs
without array guards, and loaded reference results retain ownership through return.
Synchronous embedding shares these checks and avoids its bridge on uncontended
success. Interpreter and compiled boolean stores also fix a reproduced low-bit
narrowing error. The exact exported fixture passes 217 HotSpot checks in both modes.

The eight-configuration native matrix has 112 suspended schedules. Full collection
continues during 64 loader waits and 16 held-read-lock store waits. The other 32
held-writer cases verify cancellation/migration, but the writer still prevents full
marking. Initial timeout and bounded failure evidence are preserved; D/E is open.
Request overhead and retry costs are unmeasured. This change does not qualify
production coverage, pauses or sustained performance, and supplies no new timing
graph. See [validation](../../ristretto_gc/VALIDATION.md#owning-array-access-and-guard-free-errors-2026-11-09)
and the [verification record](../../target/gc-implementation/array-access-verification.json).

## Reference stores share checked, cancellable semantics

Interpreter `aastore` previously accepted incompatible references; the compiled
helper reported type errors before bounds errors. Both now share correct ordering,
resolved component identities and guard-free resolution awaits. Recursive component
checks also preserve valid multidimensional/interface-array covariance. Experimental
compiled stores return owning requests before resolution or throwable construction.

Sixteen held-loader schedules across eight native configurations demonstrate rooted
operands, unchanged pending destinations, migration, cancellation and unrelated
reclamation. Portable interpreter cases pass on WASI, and the exact exported Java
fixture passes 38 HotSpot checks in interpreted and compiled modes. This change
adds required checks to the interpreter; their cost and continuation overhead remain
unmeasured. No new timing graph or performance gate follows from these checks.
Production coverage and the remaining helper/progress/qualification work stay open.
See [validation](../../ristretto_gc/VALIDATION.md#checked-reference-stores-and-owning-requests-2026-11-09)
and the [verification record](../../target/gc-implementation/reference-store-verification.json).

## Type checks can suspend outside generated frames

Experimental `checkcast` and `instanceof` now return owning requests before class
resolution and assignability can await. Their operand remains rooted across
migration/cancellation, and ready requests consume the continuation budget.
Thirty-two held-loader schedules in eight policy/storage/worker configurations
preserve the operand and reclaim unrelated garbage. Differential execution covers
null, arrays, interfaces, target non-initialization, cast errors and wide state;
93 exact-bytecode HotSpot checks pass in interpreted and compiled modes.

This removes another synchronous bridge from experimental compiled application
paths, including the cast used by the roots workload. It does not expand production
eligibility or measure performance. No new throughput/latency graph is warranted by
these correctness checks. Remaining array/constant/exception helpers, production
coverage, collector progress and sustained-load qualification stay open. See
[validation](../../ristretto_gc/VALIDATION.md#owning-compiled-type-checks-2026-11-09)
and the [bound evidence](../../target/gc-implementation/type-check-continuation-verification.json).

## Allocation registration releases admission before index waits

Allocation registration now prepares the complete object/identity/young write set
before changing phase marks or index membership. A failed try-lock releases all
partial guards and admission before waiting and retrying. Counted admission starts
after preparation; the default gate retains its reader-before-index order, with
nonwaiting attempts. The existing graph, reference and allocation-credit protocol
is unchanged. Thirty-six forced configurations reproduce and fix admitted index
waits: identity/young waits permit subsequent full cycles, and object waits permit
unrelated active-sweep reclamation. A held object shard still prevents complete
enumeration. This does not implement helping for preempted admitted publishers.

All 1,120 frozen measurement processes pass integrity and teardown checks, with
seven randomized pairs per condition and one child at a time on CPU 0. Each native
and WASI lane runs control, allocation, owning allocation and roots under five
policies, with diagnostics off/on and 100,000 operations. The baseline includes the
preceding mutator-buffer changes. WASI uses the same fixture, with its old module
built from an isolated, hash-verified source reconstruction. The runner freezes
both modules, Wasmtime/version and harness source. Runtime compilation/startup is
outside the guest timings. No physical-memory ceiling or latency qualification
is implied by these mechanism screens.

Diagnostics-off after/before operation-time ratios are below; values above one
mean more time. Intervals are exploratory paired 95% intervals, unadjusted for
multiple comparisons. Reports retain all other policies and diagnostics modes.

| Target / workload | Policy | Operation ratio [95% CI] |
| --- | --- | ---: |
| Native allocation | baseline | 1.013 [1.006, 1.031] |
| Native allocation | contexts | 1.018 [1.016, 1.024] |
| Native allocation | candidate | 1.028 [1.017, 1.037] |
| Native owning allocation | counted | 1.022 [1.018, 1.027] |
| Native owning allocation | candidate | 1.017 [0.983, 1.029] |
| Native roots | baseline | 1.015 [1.003, 1.031] |
| Native roots | candidate | 1.006 [0.978, 1.019] |
| WASI allocation | baseline | 0.999 [0.981, 1.003] |
| WASI allocation | contexts | 1.026 [1.010, 1.033] |
| WASI allocation | candidate | 1.015 [1.004, 1.022] |
| WASI owning allocation | candidate | 1.033 [1.009, 1.051] |
| WASI roots | baseline | 0.960 [0.928, 0.972] |
| WASI roots | candidate | 1.039 [0.998, 1.053] |

The change has an allocation cost, typically 1–3% in the affected cases. Including
between-batch collection, native candidate allocation is 1.012 [1.000, 1.022] and
WASI candidate allocation is 1.005 [0.992, 1.009]. Root controls change in both
directions, and intervals containing one do not establish parity. Disassembly after
timing shows `prepare_registration` and `try_registration` are already inlined into
`register`; the cold wait remains separate. The native registration body grows
from 6,413 to 6,932 bytes. This inspection does not attribute the measured regression
to any particular instruction or prove that outlining would improve it.

The implementation is retained for the demonstrated contention progress, with
allocation overhead still open for optimization. Map-node allocation, preempted
successful admissions, other root/reference publication and opaque graph callbacks
still need the complete D/E protocol. No default, STW, sustainable-capacity or
endurance gate is closed. Commands and evidence are bound in the
[verification record](../../target/gc-implementation/registration-prepare-verification.json);
see [native tables](../../target/gc-implementation/registration-prepare-native-screen/REPORT.md),
[WASI tables](../../target/gc-implementation/registration-prepare-wasi-screen/REPORT.md)
and [correctness validation](../../ristretto_gc/VALIDATION.md).

![Native allocation and root time ratios, diagnostics off](../../target/gc-implementation/registration-prepare-native-screen/mechanisms-diagnostics-False.svg)

![WASI allocation and root time ratios, diagnostics off](../../target/gc-implementation/registration-prepare-wasi-screen/mechanisms-diagnostics-False.svg)

## Mutator barrier insertion and overflow yield on contention

Buffered mutator barriers now try their buffer/ready locks before publishing a
reservation. Overflow uses prepared, matching record claims and stops when required
indexes are busy. Both paths can fall back to direct publication; the explicit
flush API keeps its synchronous behavior. Forty forced configurations verify
graph preservation, unrelated collection under held empty-buffer/ready locks,
old-to-young notification and lossless contended overflow. This advances D/E but
does not remove stalled admissions, inaccessible nonempty records, target-index
waits or graph-callback blocking. See [validation](../../ristretto_gc/VALIDATION.md).

The frozen comparison runs seven randomized process pairs per condition, one child
at a time on CPU 0. All 784 processes pass integrity/teardown checks: 560 native
mechanism runs plus 112 native and 112 WASI full-graph runs. Both diagnostics modes
are retained. Mechanism operation time excludes between-batch collection; its wall
metric includes that work. Graph timing covers every collection phase, excluding
construction, three warmup cycles, checks and teardown. Graphs use 4,097 nodes,
32 measured cycles and zero workers; their barrier shape has 32 records/context
and no overflow. No physical-memory cap, STW or arrival latency was measured.

The table shows diagnostics-off after/before time ratios, with exploratory paired
95% intervals. Values above one mean more time. Intervals are not adjusted for
multiple comparisons; each ratio is a median of process-pair ratios.

| Target / workload | Policy | Time ratio [95% CI] |
| --- | --- | ---: |
| Native barrier64, operation | contexts | 1.090 [1.078, 1.095] |
| Native barrier64, operation | local | 1.088 [1.080, 1.089] |
| Native barrier64, operation | candidate | 1.011 [1.007, 1.037] |
| Native roots, operation | candidate | 1.004 [0.999, 1.013] |
| Native overwrite, operation | candidate | 1.003 [0.990, 1.033] |
| Native wide, collection | gated | 0.955 [0.924, 0.959] |
| Native wide, collection | counted | 0.957 [0.943, 0.992] |
| Native barrier, collection | gated | 0.981 [0.980, 0.996] |
| Native barrier, collection | counted | 1.005 [0.992, 1.007] |
| WASI wide, collection | gated | 0.990 [0.984, 1.012] |
| WASI wide, collection | counted | 1.002 [0.989, 1.007] |
| WASI barrier, collection | gated | 0.980 [0.972, 0.999] |
| WASI barrier, collection | counted | 0.997 [0.993, 1.010] |

The context/local overflow regressions remain visible in wall time too: 1.074
[1.061, 1.078] and 1.073 [1.065, 1.077]. Root/overwrite intervals include no change.
The native wide control improves despite not exercising buffered publication;
this is a complete-path result, not an isolated causal measurement of the lock
change. Most WASI intervals include no change, which does not establish parity.
The earlier preparation batch's regression is not erased by this newer baseline.
The implementation is retained for its forced-contention progress, with overflow
cost still open for optimization and no qualification/default decision implied.

Preserved evidence includes [mechanism tables](../../target/gc-implementation/mutator-buffer-mechanisms/REPORT.md),
[native collection tables](../../target/gc-implementation/mutator-buffer-native-screen/REPORT.md),
[WASI collection tables](../../target/gc-implementation/mutator-buffer-wasi-screen/REPORT.md),
and the [verification record](../../target/gc-implementation/mutator-buffer-verification.json).
The initial plotting import failure is retained; graphs use its complete saved
measurements without rerunning them. Raw data, both diagnostics modes and SVG/PNG
exports are stored beside the reports.

![Native mechanism time ratios, diagnostics off](../../target/gc-implementation/mutator-buffer-mechanisms/mechanisms-diagnostics-False.svg)

![Native full-graph collection ratios](../../target/gc-implementation/mutator-buffer-native-screen/mark-cycles.svg)

![WASI full-graph collection ratios](../../target/gc-implementation/mutator-buffer-wasi-screen/mark-cycles.svg)

## Experimental field suspension and checked resolution

Field loads/stores now return an owning request before class resolution or static
initialization can suspend. Synchronous embedding and async execution share the
interpreter's field resolver and checked storage. This also corrects the former
name-only lookup and symbolic-class initialization: descriptors, access, field kind,
final writes and actual declaring-class initialization now use the shared rules.
Reference ownership precedes completion, and managed guards do not span awaits.

The native matrix covers primitive bits and narrowing, null/reference values,
inherited statics and linkage/null ordering, initialization cancellation without
replay, migration and unrelated reclamation. Both publication policies, storage
modes and zero/one worker are exercised. A checked transfer-size limit rejects
pathological continuation expansion; the large native spill fixture now uses opaque
array helpers to test backend frame metadata independently of that limit.

HotSpot verifies 52 checks on the exact exported classes. This is experimental
correctness/progress work, with no new performance graphs or qualification result.
State-transfer and guard overhead, complete Java/relocation semantics, remaining
helper/native suspension, production hot paths, collector D/E and sustained-load
qualification remain required. See [validation](../../ristretto_gc/VALIDATION.md).

## Experimental allocation suspension

Experimental object and array instructions now return an owning allocation request
before resolving classes, initializing objects, building arrays or awaiting memory
recovery. Saved locals, residual operands and frame roots survive suspension and
migration; completion resumes without replay. `new`-triggered initialization now
uses the existing cancellation guard outside a generated frame. Array construction
does not initialize its component. Synchronous embedding shares the allocator via
its existing bridge. Production reference eligibility and GC defaults are unchanged.

Ready allocation/monitor requests now consume and carry local fuel. The previous
monitor dispatcher refreshed fuel after every request, permitting an infinite loop
of ready operations to monopolize a poll. A bounded subprocess checks cancellation
of both loops. Native and owning argument buffers use the method's actual maximum
request size. The shared multidimensional builder yields after 64 nodes across
depths, covering many empty children that do not trigger byte-initialization yields.

Native tests cover both publication policies, both storage modes and zero/one GC
worker; workerless pressure checks preserve residual roots and verify bounded
assistance, cancellation and exactly-once completion. Portable interpreter tests
exercise large and multidimensional construction, cancellation, reclamation and
subsequent completion. The exact exported Java fixture passes 79 HotSpot checks in
interpreter and compiled modes. These are correctness results, not timing evidence.
Continuation/buffer costs, other helper suspension, full relocation/semantic
coverage, production hot paths, collector D/E and external/endurance qualification
remain open. See [validation](../../ristretto_gc/VALIDATION.md).

## Experimental monitor suspension

Generated `monitorenter`/`monitorexit` now execute the VM's actual monitor operations.
The previous lowering only popped the operand. Experimental activations return an
owning request before acquisition, then restore the post-instruction state and
check pending exceptions. The synchronous API uses real helpers. Production
reference/monitor eligibility and collector defaults remain unchanged.

Interpreter and compiled monitor entries now own their object roots across
suspension and migration. Cancelling a host scope releases its outstanding entries;
inner cancellation preserves older outer entries. Focused tests check ownership
while held, reentrancy, blocked contenders, null/ownership errors, catch-all release,
thread-state restoration, dead-local/residual-root lifetime and unrelated
reclamation. The native-capacity fixture now explicitly uses the synchronous API
and calls `Thread.holdsLock` inside its protected region. Earlier argument-monitor
post-release checks did not establish acquisition; their evidence is corrected in
VALIDATION.md without overwriting the historical record.

These are correctness checks, not a performance result. Full synchronization and
wait/notify cancellation, allocation/helper suspension, native reentry, production
hot paths and measured continuation/ledger costs remain required, alongside the
collector progress and external/endurance gates. See
[validation](../../ristretto_gc/VALIDATION.md).

## Experimental handler suspension

Methods with exception tables can now suspend at direct calls and selected handler
entries. Handler edges consume local fuel because repeated exceptions can form
loops without any backward branch. Exception dispatch uses a separate target map,
and yielding publishes the selected handler's live locals and throwable operand.
Resumption enters that handler without repeating exception selection or ownership
transfer. The production eligibility gate is unchanged.

Focused generated-code tests retain handler-only locals and exceptions across
collection and migration, release obsolete throwables, and cancel calls made from
a handler through interpreted/compiled callees. They also verify unrelated
reclamation, nested-frame cleanup and reentrant method-monitor release. The
native-overflow fixture then used monitor instructions to select a synchronous
boundary; the later audit found that those opcodes were no-ops. The monitor
follow-up corrects the implementation and fixture evidence. Java-budget tests
continue to cover cooperative recursion.

This advances J1–J3's suspension and root-lifetime work. Explicit monitors,
mid-helper/native cancellation, complete relocation and semantic qualification,
production hot paths, measured call/root/poll costs and the external/endurance
gates remain open. No performance result follows from these correctness checks.
See [validation](../../ristretto_gc/VALIDATION.md#handler-continuations-and-exception-only-loops-2026-11-09).

## Experimental call suspension and initializer cancellation

Experimental static, special, virtual and interface calls can now return their
generated caller frame before resolution and nested execution. An owning activation
retains typed locals, remaining operands and argument bits; the runtime context
retains the published roots. Completion resumes immediately after the invocation.
The public synchronous embedding API and methods with exception tables retain
the synchronous helper path. Production reference/call eligibility is unchanged.

Call suspension permits cancellation before entering a callee and while an
interpreted or compiled callee is suspended, including migration between OS threads.
Tests verify exactly-once call side effects, roots, synchronized-method cleanup and
unrelated reclamation. Temporary stack segments protect nested future polling/drop;
the Java slot budget still limits logical recursion. The native-overflow regression
now explicitly retains a synchronous generated wrapper so it continues to test
native capacity separately from cooperative recursion.

Cancelling class initialization previously left the initializing thread's claim
in place indefinitely. An owning cleanup guard now marks that class failed and
wakes waiters; partially executed initialization is never replayed. Both interpreted
and experimental compiled callers exercise this path, with reentry and abandoned
waiters checked separately. The full instrumented VM suite passes 981 tests with
two existing ignores. See [validation](../../ristretto_gc/VALIDATION.md#call-continuations-and-initialization-cancellation-2026-11-09).

This is additional J1/J3 correctness work. Handler/helper suspension, complete
relocation and native-reentry coverage, production hot paths, call/polling overhead,
pause telemetry and capacity/endurance gates remain open. No benchmark timing or
new performance graph is inferred from these checks.

## Experimental reference-loop suspension

Reference and allocation loops can now return their native frame while retaining
one owning runtime context. Before yielding, generated code publishes the resume
destination's live reference locals and operands. Consumed reference conditions
and discarded helper results do not stay rooted for the suspension. The borrowed
context pointer is refreshed per slice and cleared before return, allowing the
owning context and future to move between OS threads.

Live-VM tests cover collection between slices, reference returns, pending exceptions,
and cancellation that releases roots, frames and method monitors. Both publication
policies and zero/one worker are exercised. Experimental tests inject compiled code;
the production gate remains limited to its previously admitted primitive methods.
The exact finite fixture bytecode also has a maintained HotSpot oracle.

This completes another part of J1/J2's lifetime work. Direct calls, handler loops,
mid-helper cancellation, relocation, polling/root-transfer overhead and required
benchmark hot-path coverage remain open. Helpers inside resumable loops still run
synchronously, so bytecode fuel cannot establish a wall-time or GC pause bound.
No performance comparison or default-policy change follows from these correctness
checks. See [validation](../../ristretto_gc/VALIDATION.md#reference-continuations-and-suspended-roots-2026-11-09)
and the [verification record](../../target/gc-implementation/reference-continuation-verification.json).

## Resumable production primitive loops

Static primitive loops now compile through the normal production gate. Taken
backward/self edges consume a local budget. On exhaustion, generated code saves
typed locals and remaining operands into invocation-owned cells and returns.
The VM publishes the next bytecode offset, yields its executor, and resumes that
destination without replaying instructions. Each slice checks native stack space.
Dropping a suspended invocation cancels it with no generated frame still running.
Forward-only bodies retain their direct execution path. Compilation reuses one
target map instead of copying the CFG at each backward edge.

The backend checks BCI-zero and nested loops, switches, category-two values,
NaN payloads/signed zero, reused locals, independent/migrated activations, invalid
arguments and stack rejection before entry. A bounded VM subprocess covers both
publication policies, zero/one worker, and direct/interpreted callers. Suspended
compiled loops keep owning arguments alive, allow unrelated reclamation, resume
on another OS thread and release frames/monitors on cancellation. A one-worker
Tokio executor remains responsive and aborts an infinite compiled loop. Ordinary
and instrumented builds exercise production admission, with no injected code.

The updated Java oracle agrees with 388 results from 26 numeric fixture classes
in HotSpot interpreter and compiled modes. The separate infinite-loop lifecycle
classes are tested in the bounded Rust subprocess, not run indefinitely on HotSpot.
`compiled_resumes` is a diagnostic counter separate from method invocations and
backedges; interval subtraction preserves that distinction. Historical missing
resume counts remain unavailable. No comparison graph or timing claim is derived
from these correctness runs.

The method-size budget bounds bytecode visits and declared-state copying, not wall
time or STW pauses. Reference/call-bearing continuations, native/helper cancellation,
polling/transfer cost, required benchmark hot-path coverage and hour/day qualification
remain open. Public synchronous JIT execution still runs to completion; native VM
dispatch uses the cooperative API. Wasm retains its existing interpreter contract.
See [validation](../../ristretto_gc/VALIDATION.md#resumable-production-primitive-loops-2026-11-08)
and the [verification record](../../target/gc-implementation/primitive-continuation-verification.json).

## Production primitive control flow

The native production gate now admits static primitive methods with strictly
forward integer conditions, `goto`/`goto_w` and switches. Every switch destination,
including its default, must advance the bytecode offset. These methods use the
rootless primitive ABI and cannot revisit a bytecode or call another Java method.
The finite instruction bound is not a measured wall-time or pause guarantee.
Native loops still need bounded cooperation and cancellation support; reference
operations, calls, handlers and monitors retain their production restrictions.

Six VM tests exercise ordinary compilation and dispatch without injecting code
into the cache. Twenty-five classes execute compiled code and a finite-loop class
falls back to the interpreter. The 388 differential results cover all twelve
integer conditions, both jump widths, signed switch extremes, duplicate/empty
switch cases, local and category-two operand merges, reused locals, long extrema,
NaNs and signed zero. HotSpot agrees on the exact exported class files in both
interpreter and compiled modes. The original four fixtures failed at the previous
production eligibility gate; that evidence is retained.

The instrumented native VM library suite passes 972 tests with two existing
ignores. This is an incremental J2/J3 capability change. It does not establish
compiled benchmark hot-path coverage, a collector performance improvement, the
sub-millisecond pause target or sustained-load qualification. No collector policy
or comparison workload changed. The earlier execution reports below describe
their frozen builds and remain historical evidence.

See the [validation record](../../ristretto_gc/VALIDATION.md#production-forward-primitive-control-flow-2026-11-08)
and [fixture hashes](../../target/gc-implementation/forward-primitive-classes/manifest.json).

## Taken backedges and scoped execution diagnostics

The opt-in execution profiler now counts taken native branches to the same or an
earlier bytecode offset, including conditional and switch destinations. Forwarding
blocks preserve Java operands separately from Cranelift's implicit local-variable
parameters. Counters remain owned with the generated function and shared by clones;
concurrent generated calls retain every count. Ordinary builds omit this code.

VM checkpoints expose interval counter differences without resetting lifetime
totals. They preserve method-instance identities, compiled-code presence at the
starting snapshot and generated-code bytes at both boundaries. Native snapshot
costs are reported; Wasm checkpoints need no host clock. The cuts are not atomic
across the VM, and compilation duration is charged when compilation finishes.
These diagnostic intervals cannot satisfy the qualification measurement-window
contract or prove compilation readiness.

Six native and three WASI forks of the unchanged shared Java workload pass content,
mode and profile checks under two CPU cores of quota, a 64 MiB logical heap and a
512 MiB process-memory ceiling. Inputs remain unchanged. The native main-invocation
interval includes main-class loading, warmup, timed work and reporting; it excludes
activity recorded before the checkpoint. Its result is decisive for the current
execution gap:

| Native JIT workload | Lifetime compiled entries | Main-invocation compiled entries | Main-invocation compiled backedges |
|---|---:|---:|---:|
| Allocation | 4,522 | 0 | 0 |
| Roots | 4,508 | 0 | 0 |
| Mixed | 4,522 | 0 | 0 |

The lifetime compiled methods are `Byte.toUnsignedInt` and `WeakHashMap.indexFor`.
All required benchmark hot methods remain interpreted. Lifetime-minus-interval
counts include activity both before and after the interval; they should not be
presented as a precise bootstrap phase measurement. WASI reports no compiled
activity and null checkpoint timings. These are coverage diagnostics, not
throughput, latency, STW or sustainable-capacity qualification.

The full instrumented native VM library suite passes 966 tests. Instrumented JIT
suites pass 227 library tests, 241 integration tests and one doctest in debug and
release; ordinary JIT suites also pass. Three optimized VM allocation-pressure
fixtures pass after an initial instrumentation error was caught by the verifier
and corrected. Two WASI checkpoint tests, bare-Wasm compilation, native/WASI
linting and all 70 Python harness tests pass. J0–J5 and the overall goal remain open.

[Native report](../../target/gc-implementation/execution-windows-native/REPORT.md) ·
[Native coverage graph](../../target/gc-implementation/execution-windows-native/coverage.svg) ·
[WASI report](../../target/gc-implementation/execution-windows-wasi/REPORT.md) ·
[WASI coverage graph](../../target/gc-implementation/execution-windows-wasi/coverage.svg)

## Versioned qualification evidence contract

The runner can freeze a v2 acceptance policy before measurement, bind it to the
native binary/source provenance, identify each process and retain output hashes.
The evaluator verifies captured packets, compiled entries/backedges for required
hot methods, compilation readiness, measured GC exposure, resource limits and
scheduled-arrival latency/backlog limits. It includes rendezvous in every stop,
merges overlapping/touching stop intervals and rejects exactly 1 ms or longer.
An empty stream needs complete capture and a matching reviewed coordination audit;
it produces no invented percentiles. Peer ratios have no acceptance thresholds.
Reports use new versioned filenames and refuse overwrites.

This is checker and transport infrastructure. It does not implement a recorder,
prove an audit, expand production JIT coverage, establish warmup stability or
qualify performance. [QUALIFICATION.md](QUALIFICATION.md) describes the contract
and remaining producer work. Current global blocking limitations cannot be waived
by declaring a complete audit. Historical reports remain immutable evidence under
their original criteria; new synthetic tests only validate checker behavior.
All 68 Python harness tests pass. Replaying four frozen native datasets reproduces
every original v1 gate field; v2 rejects their missing policy/evidence, and the
historical files remain unchanged. No new runtime benchmark was run for this batch.
See the [validation record](../../ristretto_gc/VALIDATION.md#versioned-short-run-qualification-checker-2026-11-08).

## Required barrier preparation yields on busy indexes

Collector drains prepare the required object/remembered shards with try-locks
before claiming a matching buffered record. A busy shard leaves that record and
its pending reservation visible, so the collector worker can attempt other work.
Eight forced schedules now reclaim separately retired storage on that same worker
before the required index is unlocked. The preceding implementation blocks and
fails this test. Sixteen existing application schedules preserve pending work or
publish through the ordinary/FIFO queues, and four young-start schedules preserve
an old owner's new child across delayed remembered insertion. Explicit mutator
flush/overflow is unchanged. An unfinished required record still prevents closure.

The implementation passes 432 native GC checks, 224 WASI checks and 208 optimized
library checks, plus twelve context tests under Miri. Native/WASI linting, bare-Wasm
compilation and fifty Python harness checks pass. Portable tests validate a changed
top record and separately account for an identical replacement reservation.

The new `mark_cycles` barrier shape publishes one required edge per leaf before
collection. Each process uses 4,097 nodes, 32 measured full cycles, three warmups,
32-record contexts and zero background workers. Enabled diagnostics must observe
exactly 131,072 buffered and flushed records, with no overflow. Mark totals,
checksum, completed cycles and zero-byte teardown remain mandatory. The broad-root
control has no buffered traffic. This measures publication plus complete collection
cost, not isolated barrier instructions, application throughput or pause latency.

The first 224-process screen found a broad-graph regression on native and WASI.
The prepared helper had been inlined into `step_inner`, growing its native symbol
from 27,825 to 31,222 bytes. Keeping the helper out of line reduces that symbol to
28,674 bytes; drain timing now includes failed preparation attempts as well as
successful application. A separate paired comparison finds a clear WASI broad-
graph improvement over the first candidate; native intervals remain inconclusive.
Those two changes are measured together, so their individual effects are not isolated.

The selected candidate's final comparison against the original baseline passes
all 224 processes. Diagnostics-off collection-time ratios are:

| Workload | Policy | Native ratio [95% interval] | WASI ratio [95% interval] |
|---|---|---:|---:|
| Buffered edges | Gated | 1.006 [1.001, 1.016] | 1.007 [1.005, 1.020] |
| Buffered edges | Counted | 1.015 [1.011, 1.049] | 1.012 [1.003, 1.015] |
| Broad control | Gated | 1.031 [1.021, 1.133] | 0.973 [0.968, 0.995] |
| Broad control | Counted | 1.052 [1.045, 1.080] | 1.005 [0.981, 1.035] |

Ratios below one favor the candidate. Seven randomized adjacent process pairs per
condition give exploratory, unadjusted intervals. Required publication costs about
0.6–1.5% more in these selected diagnostics-off cases. The native broad control
still regresses by about 3–5%; that remains open. All diagnostics-enabled results
are retained in the reports. No throughput-superiority claim follows from the
contention improvement.

A further 224-process experiment added a zero-pending check inside the drain.
Audit found the caller already performs this check. The redundant change and its
test were removed; its measurements remain available as a rejected experiment.
In total, all 896 measured processes validate, with frozen inputs unchanged and
no overlapping edits/builds/tests/profiling. The native regression, admitted stalled
publishers, callback reservations, host-allocation stalls and sustained-load/peer
qualification remain unresolved.

[Selected native report](../../target/gc-implementation/barrier-prepare-final-native/REPORT.md) ·
[Native graph](../../target/gc-implementation/barrier-prepare-final-native/mark-cycles.svg) ·
[Selected WASI report](../../target/gc-implementation/barrier-prepare-final-wasi/REPORT.md) ·
[WASI graph](../../target/gc-implementation/barrier-prepare-final-wasi/mark-cycles.svg) ·
[All checks, failures, builds and experiments](../../target/gc-implementation/barrier-prepare-verification.json)

## Avoid empty probes and reuse available worker queues

Claims now skip ordinary queues whose advisory count is zero and only rewrite a
count after removing work. A missed concurrent insertion makes that claim yield;
closure still checks publication, claims and pending/in-flight reservations. When
its local queue and injector are unavailable, a producer probes the other worker
queues once each before using the atomic FIFO. Capacity is rechecked under each
acquired mutex, and no worker queue exceeds 256 tasks. This preserves progress when
all ordinary queues are held while avoiding many fallback-node allocations.
Ordinary deque storage can still grow under its mutex.

Separate instrumented builds count events, not performance. Across three runs per
condition, multiworker tree fallback publications fall from roughly 12,000–15,000
with empty-probe skipping to 3,500–4,400 with available-queue publication; broad
runs fall from roughly 12,000–14,000 to zero. Empty lock acquisitions are almost
eliminated apart from races after a nonzero hint. These cumulative counts include
warmup/teardown and their atomic instrumentation can change scheduling.

Three paired screens pass all 1,008 processes: the empty-probe change, the
available-queue change, and the complete implementation against the original
blocking-queue baseline. All use matching frozen fixtures, profiles and dependencies,
8,191 nodes, 32 measured full collections and seven process pairs per condition.
Native diagnostics-off multiworker ratios for the available-queue change range
from 0.863 to 0.871. Against the original baseline, cooperative ratios range from
0.905 to 0.964; broad multiworker ratios are 0.858 [0.837, 0.871] gated and 0.825
[0.802, 0.839] counted. The tree is inconclusive at 1.021 [0.990, 1.078] gated and
1.008 [0.950, 1.041] counted. The earlier clear 14% regression is not reproduced by
the final screen, but these intervals do not prove parity or superiority. Seven of
eight final WASI intervals include one; the gated broad diagnostics-off ratio is
0.960 [0.858, 0.994]. All intervals are exploratory and unadjusted for multiplicity.

[Final native report](../../target/gc-implementation/mark-cycles-publication-final-native/REPORT.md) ·
[Final native graph](../../target/gc-implementation/mark-cycles-publication-final-native/mark-cycles.svg) ·
[Final WASI report](../../target/gc-implementation/mark-cycles-publication-final-wasi/REPORT.md) ·
[Final WASI graph](../../target/gc-implementation/mark-cycles-publication-final-wasi/mark-cycles.svg) ·
[Empty-probe ablation](../../target/gc-implementation/queue-empty-verification.json) ·
[Available-queue ablation and verification](../../target/gc-implementation/queue-spare-verification.json)

Validation passes 428 native GC checks, 222 WASI checks, 204 optimized library
checks, four native VM GC/reference checks, two WASI VM checks, and the new Miri
queue test. Sixteen forced application configurations verify eight metadata stalls
keep pending protection and eight held-empty-injector cases complete collection
and reclaim unrelated garbage. Four of the latter explicitly observe a FIFO item;
four observe an ordinary queued item and no FIFO item. Background workers remain
zero in all sixteen. The unit fixture also holds every ordinary queue and consumes
its fallback publication before releasing any holder.

Two test schedules were corrected: a received completion no longer depends on the
sender having exited, and each repeated-barrier cycle now waits for every producer
to execute a barrier after that snapshot becomes active. Earlier failures remain
in the evidence. These changes do not remove unfinished admission/callback
reservations, host-allocation stalls or nonempty queue-holder limitations. The
screens measure fixed-graph collection time, not application throughput, pauses,
G1/ZGC comparisons, peak memory or endurance.

## Repeated graph collection measures the queue change

The new `mark_cycles` driver times complete collections of an 8,191-node eight-way
tree or broad-root graph, with three warmup and 32 measured cycles. Each cycle must
mark all nodes; checksum and full reclamation checks run outside timing. Native
runs use zero or four registered workers; WASI uses zero. The two isolated builds
differ only in the queue implementation/module wiring, with an identical fixture,
toolchain, release profile and dependency versions. The reduced build workspaces
prune the lock file to the same 68 packages; all identities match the main lock.

All 224 native and 112 WASI processes validate. Seven independently shuffled
process pairs per condition produce exploratory, unadjusted 95% intervals. With
diagnostics disabled, native cooperative collection ratios range from 0.918 to
0.962. Four-worker broad-graph ratios are 0.936 [0.924, 0.979] with gated publication
and 0.891 [0.848, 0.903] with counted publication. The four-worker gated tree
**regresses**, at 1.140 [1.050, 1.144]; counted tree is inconclusive at 1.046
[0.965, 1.096]. All eight WASI intervals include one. Diagnostics-enabled results
remain separate in the reports and show the same gated-tree regression.

[Native report](../../target/gc-implementation/mark-cycles-queue-native/REPORT.md) ·
[Native graph](../../target/gc-implementation/mark-cycles-queue-native/mark-cycles.svg) ·
[WASI report](../../target/gc-implementation/mark-cycles-queue-wasi/REPORT.md) ·
[WASI graph](../../target/gc-implementation/mark-cycles-queue-wasi/mark-cycles.svg) ·
[Build/run verification](../../target/gc-implementation/mark-cycles-verification.json)

This measures the net collection path, including all phases and waits. It does
not isolate fallback publication, which has no dedicated counter, or measure an
application allocation rate, STW pause, scheduled-arrival latency, peak memory or
endurance. Multiworker contention depends on scheduling. The gated-tree regression
remains unresolved; the queue's forced progress benefit is not a uniform throughput
win. Native and WASI are separate within-target comparisons, not runtime rankings.
The existing Java lane remains necessary for G1/ZGC comparisons.

## Mark publication progresses around a held empty queue

Required mark producers now try their local/injector queues, then publish to an
owning atomic FIFO on contention. Other producers and consumers can help a linked
publication whose producer has paused before updating the tail. Ordinary queues
publish counts under their mutex; validated zero counts let closure skip a held
empty queue. Nonempty contention and unfinished admissions remain conservative.
The [protocol](../../ristretto_gc/PUBLICATION.md#contended-mark-publication) explains
tombstones, owning references, claim exclusion and iterative node destruction.

The baseline required-publication fixture times out. The updated forced barrier
matrix completes and reclaims unrelated garbage while the empty injector remains
held in all four publication/phase combinations. Its four metadata-contention cases
still preserve the pending barrier and allow unrelated collector steps to return.
Queue tests also cover stalled linked producers, stale predecessor/head readers,
8,000 exactly-once concurrent deliveries, ordinary/fallback fairness, immediate
payload release and 100,000-node destruction on a 128 KiB native stack.

Validation passes 427 native GC checks, 221 WASI checks, 203 optimized library
checks, four native VM GC/reference checks and two WASI VM checks. Five bounded
Loom models cover selected queue and closure schedules, not ArcSwap internals.
Two Miri tests cover owning FIFO payloads and real mark continuations. Native/WASI
Clippy and bare-Wasm checks pass. The
[verification record](../../target/gc-implementation/queue-publication-verification.json)
retains the baseline and intermediate failures as well as final source/log hashes.

This batch is a correctness/progress change, with performance still unmeasured.
It adds two fixed nodes and a counter per generation and one node allocation per
fallback publication. Ordinary fast queues retain their storage allocation; host
allocation and CAS retries have no hard latency bound. An admitted publisher stalled
before linking, a nonempty ordinary queue holder or an opaque callback still
prevents some progress. No external comparison or sustained-load gate is passed.

## Compiled entry validates argument shape

A controlled regression showed that a compiled `(I)I` method accepted an empty
argument list and loaded the inline buffer's zero-filled slot. The test stayed
within that initialized eight-slot buffer; it did not deliberately execute the
out-of-bounds loads possible with longer mismatched descriptors.

`Function` now shares immutable argument tags derived by the same parameter walk
that initializes generated locals. Entry rejects missing/excess values before
marshalling or stack checks, and checks each value's representation during the
existing conversion pass. Instance receivers occupy the first value; `long` and
`double` each occupy one ABI value despite using two JVM local slots. Rejections
precede generated entry and execution counters. Small calls retain the inline
buffer, and large calls reserve one argument vector before conversion.

Portable stub checks cover six arities and all five representations across both
buffer paths, including rejection before native stack validation. Real generated
methods cover descriptor kinds, receiver position, wide arguments, missing/excess
values and cloned functions. The large-frame subprocess now uses separate,
correctly typed zero/nine-argument functions so both paths still exercise actual
stack exhaustion and subsequent recovery.

The VM bridge checks padded and unpadded wide-argument lists through the production
primitive-method gate. Rejected object arguments preserve caller roots; releasing
the caller makes the object collectible. These checks validate argument shape and
ownership at rejection. They do not validate arbitrary raw reference addresses,
custom helper ABI/lifetime, or native context pointers. Production opcode
eligibility remains unchanged, and Wasm continues using the interpreter.

Both debug and optimized JIT suites pass 464 checks (227 library plus 237
integration). The full release VM suite passes 962 library checks, and the added
bridge check passes separately, for 963 distinct library checks; four native
GC/reference checks and two WASI VM checks also pass. WASI passes 29 portable JIT
checks. Native JIT/VM and WASI JIT Clippy, bare-Wasm JIT, formatting and diff checks
pass. The [verification record](../../target/gc-implementation/jit-arguments-verification.json)
records the baseline failure, exact commands, source/log hashes and an intermediate
fixture correction: a VM argument clone adds its own root, and the cached emergency
throwable must be initialized before measuring per-call root changes.
Signature storage and the count/type branches add costs that have not been
performance-qualified. This safety prerequisite does not establish the complete
compiled-frame contract, GC progress, G1/ZGC superiority or endurance gates.

## Barrier drains release collector coordination

Collector steps now try ready-context and buffer locks before claiming a record.
A last-record pop claims both required locks before changing the buffer or removing
its ready owner; contention leaves the record and pending reservation intact.
Ready selection rotates among contexts, and a failed claim consumes a work unit.
Drain steps alternate with ordinary tracing/phase work. Required application and
final context-owner release execute outside collector phase, cycle, coordinator,
claim and publication guards. Terminal paths yield on pending records instead of
performing nested drains. Optional completed-barrier certification also skips busy
metadata after required application succeeds.

The held-buffer baseline timed out. Twenty-four forced native configurations now
cover both publication policies and both phase policies with zero background
workers: eight hold a buffer or ready registry, eight stall required application
on metadata or a mark injector, and eight stall final context cleanup on registry
removal or retired-root statistics. A held buffer keeps its record while another
context drains and its parent/child tracing completes. Held application retains
pending work and excludes closure while competing steps yield with coordination
and publication free. After acknowledgement, held final-owner cleanup permits
three unrelated full-collection requests to finish per configuration. Portable
tests cover transactional last-record ownership and optional certification.

Validation passes 414 native GC checks, 219 WASI checks and 195 optimized library
tests, plus four native VM GC/reference checks and two WASI VM checks. Native/WASI
Clippy and bare-Wasm examples also pass. The
[verification record](../../target/gc-implementation/barrier-drain-verification.json)
retains source/log hashes, the pre-fix failure, and an intermediate fixture failure:
polling the ready registry had competed with the very try-claim being observed.
The corrected test uses a passive notification immediately after the actual pop.
These deadlines are correctness assertions, not performance measurements.

Required application can still occupy its executing thread and prevent phase
closure. Mutator explicit flush/overflow retains blocking pops and any enclosing
admission. The scheduling atomic and changed division of work between steps are
unmeasured costs; defaults have not changed. This batch does not establish zero
STW, superiority to G1/ZGC, or sustained-load qualification.

## Context scans progress around registry, cache and cleanup stalls

Context root scans now read a published immutable registry view, and retain one
current context while traversing its slots. Attachment publishes its empty context
before returning a handle; later root insertion keeps its existing barrier. This
removes the registry lock and weak-owner upgrade from each root read. Context
preparation and final owner release execute after publication admission, cycle and
coordinator guards have been released, ending that collector step.

Unused allocation-cache ranges and credits are detached under the cache mutex,
then released/refunded outside it. A busy cache is skipped for that scan and remains
charged. A held-cache baseline prevented full collection from finishing. The new
four-case matrix completes three full-collection requests per configuration while
4,080 logical credit bytes and 16,320 unused descriptor-range bytes remain retained;
logical charge stays at 4,096 bytes. Once released, the next collection returns
unused capacity without resetting cache hit/refill counters. These bytes describe
the fixture, not RSS or a general allocation-size guarantee.

Four held-registry cases preserve the context's live graph while collecting
unrelated garbage. Eight final-owner cases stall cleanup on either registry removal
or retired-root accounting while another collector completes unrelated full
collections. All sixteen schedules cover both publication and phase policies,
with no registered background workers. Portable tests cover immutable-view reads
under the registry writer, ownership until the cursor advances, and accounting for
pages that outlive their context owner.

Registry publication clones its map on each context attach/remove, adding work and
allocation proportional to the context count. Older views can remain reader-owned.
A detached context and its bounded empty-page cache can also remain owned by a scan
until that cursor advances. These memory and cold-path costs are unqualified;
context setup/teardown throughput and general performance have not been measured.
The context cleanup operation can still occupy its executing thread while stalled.

Validation passes 409 native GC checks, 217 WASI checks and 190 optimized GC library
tests. Four native VM GC/reference checks and two WASI VM checks also pass.
Native/WASI Clippy and the bare-Wasm build pass. The
[verification record](../../target/gc-implementation/context-progress-verification.json)
retains commands, hashes and intermediate failures. An optimized run exposed a
fixture flaw in the earlier slot-root test: its first step could already read the
root, so a later held-shard timeout could occur during child tracing. The test now
holds metadata before any collector step; the corrected eight-case matrix passes.

This advances full-cycle progress for these context stalls. Required-record
buffers, queue publication, context destruction from other paths, admitted
publishers and opaque callbacks still require work. No zero-STW, G1/ZGC superiority,
latency-tail or sustained-load gate is established by these tests.

## Slot and context roots retry busy metadata

A held metadata shard reproduced a slot-root scan retaining its coordinator while
waiting. Slot/context enumeration now rewinds the selected slot when the metadata
lookup is busy and yields its step. Retry re-reads the address from the retained
page, so slot removal or reuse cannot leave a saved stale address. Tree roots use
the same metadata try-lookup. The successful read-to-shade operation keeps its
existing admission/phase protection, and no extra cursor storage is needed.

Eight native schedules cover both root stores, publication policies and phase
policies. Each releases coordination/admission before the held shard is released,
then preserves the rooted parent and child through collection and reclaims them
after their owners are dropped. A portable cursor test covers replacement and
removal at the first and last page slots, without replaying other slots. The
baseline's coordinated case retained its coordinator but held no publication
admission; its observation does not establish a publisher blocked by a writer.

The GC suites pass 404 native checks, 215 WASI checks and 185 optimized library
tests. Four native VM GC/reference checks and two WASI VM checks also pass.
Native/WASI Clippy and the bare-Wasm example build pass. The
[verification record](../../target/gc-implementation/root-metadata-verification.json)
retains source/log hashes, commands and the failing baseline. These are progress
and correctness checks, with no new performance or endurance measurements.
Context registry/cache operations, mark-queue publication and reference processing
still have waits; stalled admissions and opaque callbacks still exclude closure.

## Metadata contention yields without losing sweep work

A held object-index shard reproduced a collector waiting with both its coordinator
and legacy publication writer retained. Index scans now try one shard per unit,
retain a position for every shard, and rotate past busy shards without marking them
complete. Failed downstream lookups restore the selected entry's previous position
before rotating. Retry re-reads the index, including after address reuse.

Retirement acquires object, identity, young and remembered shards before removing
any entry. Promotion acquires both membership shards before changing age or
cross-domain marks. A busy guard leaves the operation uncommitted. Tree-root,
cross-root, remembered-owner, reset, old-handoff and compaction metadata scans also
yield instead of waiting on their selected index. Successful root read-to-shade
operations retain their existing admission protection.

All 48 forced sweep configurations pass: gated/counted publication, coordinated/
independent phases, young/full collection and each of four held indexes. Counted
cases additionally hold an actual publication admission. Unrelated garbage is
reclaimed during the same sweep, the blocked object remains intact and its identity
is preserved, and it is reclaimed exactly once after release. Eight promotion
schedules verify unchanged age/membership before retry and exactly one subsequent
promotion. The tree-root retry test covers both publication policies; portable map/
set tests cover one, two and sixteen shards plus replacement at a retried address.
No background worker is registered in the forced native schedules.

Validation passes 402 native GC checks, 214 WASI GC checks and 183 optimized GC
library tests. Four native VM GC/reference checks and two WASI VM checks also
pass. Native/WASI Clippy and the bare-Wasm example build pass. Each cursor
is 176 bytes on Linux x86-64 and 88 bytes on wasm32, up from two machine words;
ten cursors add 1,600/800 bytes per collector. There is no per-object cursor cost.
The [verification record](../../target/gc-implementation/index-progress-verification.json)
retains commands, source/log hashes and the failing baseline.

This proves progress in other shards within an active sweep, including while a
counted admission is held. It does not prove cycle completion around a stalled
publisher or callback. The subsequent root-metadata change above removes another
lookup wait. Queue publication, context registry/cache operations, reference
registries, statistics and other buffer/index waits remain. Single-shard heaps yield
but have no other index shard to visit. No new throughput, latency-tail, sustained
load or G1/ZGC qualification is claimed; test deadlines are not benchmark samples.

## Collector steps yield on publication and queue contention

A forced schedule reproduced a collector retaining its coordinator and publication
writer while waiting for a held mark queue. An unrelated root registration/removal
could not finish until that queue was released. Single-queue claims also waited.
Collector claims now try every queue without blocking, and termination probes
return an explicit unknown result for contention. Mark, reference, sweep and
compaction paths yield on that result without advancing their phase or treating it
as queued work. Work remains in its original queue.

Mark and relocation closure also try their claim writers. An actual mark claim
paused after dequeue and before its in-flight reservation now keeps closure
excluded while the competing collector step releases global coordination. The
relocation regression checks the corresponding terminal interlock. Legacy
publication-writer acquisition now tries once and yields with its request/cursor
intact. Counted mode still bypasses that writer. Its active-count protocol is
unchanged.

Forty held-queue configurations cover both publication/phase policies, one/three
queues, empty/nonempty marking queues and the reference/sweep/compaction boundaries.
Ten held-reader configurations cover both phase policies and cycle start plus the
four terminal phases. There are zero registered background workers in these forced
schedules. They verify that the step yields, coordination/admission is released,
unrelated already-marked root operations complete, phases stay put, and normal
tracing/reclamation completes after release. These timeouts are test bounds, not
latency percentiles or performance measurements.

**Scope correction:** the held-publication-reader baseline reproduced a parked
collector/coordinator; another publisher still completed. The pinned native
recursive lock permits readers despite a waiting writer. The blocked unrelated
root operation was reproduced by the separate held-queue/held-writer fixture.

The native GC suite passes 398 checks and the WASI GC suite passes 212. Native and
WASI Clippy and the bare-Wasm example build pass. The release GC library passes
179 tests; four native VM GC/reference integration checks and two WASI VM checks
also pass. The [verification record](../../target/gc-implementation/publication-yield-verification.json)
contains commands, source/log hashes, the forced schedules and retained failures.

This removes specific waits from collector steps. It does not let a collection
close around a stalled admitted publisher or an unfinished tracing callback.
Queue publication, index/buffer waits and other work within a held publication
writer still need redesign. Continually overlapping admissions can still starve
closure. The change does not satisfy the zero-STW, throughput, tail-latency or
endurance gates; test wall times are not used to generate performance graphs.

## Generated native frame bounds

Compiled functions now retain the backend's frame span after register allocation,
plus ABI setup. `Function::execute` requires known native capacity for that frame
and a 256 KiB runtime reserve before counting or entering generated code. The VM
maps capacity failure to `StackOverflowError`; an outer compiled invocation can
allocate a segment sized for its frame, while nested invocations reject exhaustion.
Missing/inconsistent metadata and dynamic/tail-call frame forms are rejected before
an executable function is published. Metadata rejection releases the allocated code.

A null stack-map entry exposes Cranelift's frame span, including explicit slots,
register spills, saved registers and outgoing arguments. Reference-bearing methods
attach one map to their first existing root-publication call. Helper-free methods
retain a cold metadata call behind an impossible valid-slice length. Valid calls do
not execute that helper, but the branch and frame setup have a cost that remains
unmeasured. These maps do not replace the VM's owning root registry.

| Linux x86-64 fixture | Declared Java locals | Recorded native frame |
| --- | ---: | ---: |
| Pure arithmetic, values can be recomputed | 2,050 | 48 bytes |
| Opaque helper results retained across calls | 2,050 | 8,272 bytes |
| Explicit backend test allocation | Not a Java fixture | 1,048,608 bytes |

The large-frame subprocess rejects both argument-buffer paths on a 512 KiB segment
without entering generated code, then executes both paths on a 4 MiB segment. The
available space exceeds the generic red zone, so the frame size is what causes
rejection. The spill fixture consumes every retained value. Another regression
requires missing frame metadata to fail, and the VM checks outer growth for a
frame larger than its default segment. The bounded subprocess runner is shared
through `ristretto_test_util`; its timeout and output-capture tests now run there.

Validation passes 225 backend unit tests, 233 instruction/integration tests, 962
release VM tests, four native GC/reference integration checks, and two WASI checks.
The shared test utility passes five checks. Debug JIT tests, native/WASI Clippy,
formatting and diff checks pass. The [verification record](../../target/gc-implementation/jit-frame-size-verification.json)
retains commands, source/log hashes, measured frame sizes, and failed intermediate
probes. These are correctness checks, not performance comparisons.

The layout audit covers Cranelift 0.137.1 x64/aarch64/riscv64; local execution covers
Linux x86-64. Arbitrary helper stack consumption, native/JNI reentry, other native
hosts, and performance remain unqualified. Production bytecode eligibility is
unchanged. Wasm continues to use its interpreter.

## Native recursion guard and synchronized-entry correction

An isolated release subprocess reproduced a host abort during compiled recursion
on a 1 MiB OS-thread stack, before exhausting the Java slot budget. Compiled VM
execution now tracks synchronous native nesting on the current OS thread. Nested
Java dispatch rejects low available stack with `StackOverflowError`; the first
compiled entry can acquire a temporary 1 MiB segment. Throwable construction polls
with a separate stack reserve on the same thread, preserving the active GC context.
Existing throwable propagation and the preallocated OOM path avoid that allocation.
Cold exception-construction futures are boxed to keep their state out of ordinary
interpreter/dispatch futures.

This protects the tested compiled call chains. The generated-frame gap is addressed
by the size-aware check above. Arbitrary native callbacks, interpreter-only native
reentry and every supported OS/CPU still need qualification. Production call
eligibility remains closed. No throughput or latency improvement is claimed from
these correctness checks.

The stronger regression also found that `ACC_SYNCHRONIZED` static methods silently
skipped locking when their class mirror was absent. Dispatch now creates the mirror
before acquiring its monitor. A production-compiled primitive method verifies mirror
creation, blocking behind another owner, cancellation before generated entry, and
successful reuse. Recursive tests now require an actual monitor.

**Correction to the earlier stack-budget report:** its synchronized fixture allowed
`None` and skipped the monitor assertions. That run did not qualify monitor cleanup.
The corrected tests below replace that claim; the earlier raw logs are retained.

Validation passes 963 release VM tests (three ignored), 960 default-build VM tests
(three ignored), six native GC/reference integration checks (one ignored), and
two WASI lifecycle checks. Both release and default builds pass twelve bounded
native-recursion subprocesses. Native/WASI Clippy and formatting/diff checks pass.
The [verification record](../../target/gc-implementation/jit-native-stack-verification.json)
contains source/log hashes, commands, the matrix, the initial abort and failed
intermediate probes. The [monitor correction](../../target/gc-implementation/jit-stack-budget-monitor-correction.json)
identifies the earlier claim it supersedes. Generated-frame sizing is handled by
the follow-up above. Full native reentry remains open, along with stalled-publisher/callback progress,
production workload coverage, G1/ZGC performance gates and sustained qualification.

## Compiled frames share the Java stack budget

A production-path regression showed a compiled method with 128 declared local
slots returning normally under an eight-slot budget. Compiled frames now charge
`max_locals + max_stack + 1`, just like interpreted frames. Rejection occurs before
taking a pooled frame or entering generated code; pop/unwind releases the charge.
The method now rejects insufficient capacity and repeatedly succeeds at its exact
budget, with generated-entry counts checking both outcomes.

Four new tests cover sixteen publication-policy/execution-mode combinations for
caught/uncaught recursive overflow, three synchronized mixed-mode configurations,
and twelve recursive pressure configurations. Backtraces contain only entered
frames and the exact call-site byte offsets. After unwinding, arguments become
reclaimable while returned throwables stay owned, then those throwables become
reclaimable too. Monitor ownership and Java stack accounting return to empty, and
the same thread can execute again. The original monitor assertion was conditional;
see the correction above. Monitor bytecodes remain outside production eligibility.

The pressure fixture preserves five distinct per-frame arrays while a leaf
allocates 62.5 MiB in a 32 MiB heap. Both publication policies, zero/one registered
worker and mixed interpreted/compiled methods reclaim discarded allocations before
return; the result checks every retained array. HotSpot passes twenty checks on
the four exported classes, including overflow and monitor recovery using its own
stack. Exhaustion depths and capacities are not being compared.

Validation passes 957 release VM tests (one ignored), 106 default-build JIT/helper
checks (one ignored), all four focused diagnostic tests, six native GC/reference
integration tests (one ignored) and two WASI lifecycle tests. Native/WASI Clippy
and format/diff checks pass. The [verification record](../../target/gc-implementation/jit-stack-budget-verification.json)
preserves source/log hashes, matrix dimensions and the failing production regression.

[Focused regressions](../../target/gc-implementation/tests-jit-stack-budget-backtrace.log) ·
[Exact-class HotSpot checks](../../target/gc-implementation/jit-stack-budget-hotspot/manifest.json)

The logical Java budget does not bound the native Rust/generated call stack. Full
native-stack qualification remains open despite the compiled recursion guard above,
as do the broader GC
progress, compiled coverage, G1/ZGC performance and endurance gates. Production
eligibility and collector defaults remain unchanged. No performance result is
claimed for this correctness fix.

## Experimental interface calls

The native backend now lowers `invokeinterface` through the owning call bridge.
A live-frame regression first reproduced the unsupported instruction. The runtime
shares the interpreter's interface selector and bounded receiver cache, preserving
membership, inherited/default selection and selected-method error ordering.
The compiler also rejects inconsistent argument counts: wide Java values occupy
two parameter slots but one runtime argument cell. Production eligibility is unchanged.

Eight new tests cover seven receiver classes, inherited/default methods, twelve
linkage/null/access/abstract/membership outcomes, native primitive-array calls,
primitive bits, reference return lifetime and caught/uncaught exception ownership.
The pressure matrix checks both publication policies, zero/one registered worker
and interpreted/compiled callees. Each call allocates 62.5 MiB in a 32 MiB heap,
reclaims discarded arrays before return and retains distinct caller/callee arrays.

HotSpot 25 passes 42 checks on the exact exported classes. Its first run rejected
a mismatch fixture whose receiver parameter was too broad for the referenced
class. The corrected fixture uses that class and a subclass receiver; both VMs
now reach the intended runtime error. The failed export and logs remain preserved.
These are correctness checks, not G1/ZGC throughput or pause measurements.

Validation passes 953 release VM tests (one ignored), 102 focused JIT/helper tests
in both default and diagnostic builds (one ignored each), 455 backend checks, six
native GC/reference integration tests (one ignored) and two WASI lifecycle tests.
Native/WASI Clippy and format/diff checks pass. The three earlier HotSpot drivers
also pass against this export: 23 static, 17 special and 40 virtual checks, for
122 total. Source hashes, passing checks and preserved failed attempts are in the
[verification record](../../target/gc-implementation/jit-interface-call-verification.json).

[Interface-call cross-check](../../target/gc-implementation/jit-interface-call-hotspot-corrected/manifest.json) ·
[Focused checks](../../target/gc-implementation/tests-jit-interface-call-corrected-fixtures.log)

All four ordinary invocation forms now have experimental lowering. Dynamic and
signature-polymorphic behavior, complete access/initialization, private/nestmate
and conflicting-default cases, recursion, reentrancy, monitors, relocation and
production coverage remain unfinished. Global progress and external performance/
endurance gates remain open. Existing graphs describe their frozen source versions;
this change makes no new speedup claim.

## Experimental virtual calls

The native backend now lowers `invokevirtual` through the existing owning call ABI.
A live-frame regression first reproduced the unsupported instruction. The runtime
now shares the interpreter's virtual target selector and existing four-entry
receiver-class cache; the interpreter fast path is unchanged. Receiver overrides,
inherited implementations, private targets and interface defaults keep their
selection rules. Primitive-array receivers retain ownership during class loading.

Eight new live-frame tests cover seven receiver classes, cached and uncached
selection, native `Object.getClass`, private/default targets, linkage/null/abstract
error ordering, primitive bits, reference return ownership and caught/uncaught
exceptions. The pressure matrix checks both publication policies, zero/one
registered worker and interpreted/compiled overrides. Each call allocates 62.5 MiB
within a 32 MiB heap, reclaims discarded arrays before returning, and preserves
distinct caller and callee arrays. Exceptional exits release receiver roots while
the returned throwable remains alive, then release its last owner.

HotSpot 25 passes 40 virtual-call checks against the exact exported fixture classes,
plus the 23 static and 17 special-call regressions. These are verification runs,
not performance measurements. Production compilation coverage remains unqualified.

Validation passes 945 release VM tests (one ignored), 94 focused JIT/helper tests
in both default and diagnostic builds (one ignored each), 455 backend checks, six
native GC/reference integration tests (one ignored) and two WASI VM lifecycle
tests. Native/WASI Clippy and workspace format/diff checks pass.

[Virtual-call verification](../../target/gc-implementation/jit-virtual-call-hotspot/manifest.json) ·
[JIT/helper checks](../../target/gc-implementation/tests-jit-virtual-call-all-jit.log)

All three direct invocation forms in the comparison fixture now have experimental
lowering. This does not establish full transitive coverage, signature-polymorphic
behavior, access/initialization semantics, recursion, monitors, relocation or
performance/endurance gates. Interface and dynamic calls remain unsupported by the
backend, and the production eligibility gate remains unchanged. Existing benchmark
graphs retain their original frozen-source meaning; no new speedup is claimed.

## Experimental constructors and special calls

Shared call lowering now supports `invokespecial`, including an owned receiver in
the outgoing buffer. The runtime uses the interpreter's special-target selector
and normal frame dispatcher. A real compiled fixture initially failed because the
backend rejected `invokespecial`; a second fixture exposed stack simulation wrongly
requiring a class-method reference for an interface-default call. Both now pass.

Eight new live-frame tests cover interpreted/compiled constructors, superclass
selection that excludes receiver overrides, null receivers, interface defaults,
private calls, primitive bits/reference results, constructor exceptions and the
receiver-inclusive 255-slot limit. Constructor allocation pressure preserves a
newly created array reachable through the receiver while reclaiming discarded
arrays before return. All eight policy/worker/callee combinations allocate 62.5 MiB
within a 32 MiB heap, with actual zero/one worker registrations checked. Returned
graphs and throwable owners survive the handoff and become reclaimable after release.

HotSpot 25 passes 17 special-call checks on the exact exported class files; all 23
static-call checks still pass on the same export. These are verifier/interpreter
correctness runs, not G1 latency or throughput measurements.

[Special-call verification](../../target/gc-implementation/jit-special-call-hotspot/manifest.json) ·
[Static-call regression](../../target/gc-implementation/jit-special-static-regression-hotspot/manifest.json) ·
[JIT/helper checks](../../target/gc-implementation/tests-jit-special-call-all-jit.log)

Validation passes 937 release VM tests (one ignored), 86 focused JIT/helper tests
in both default and diagnostic builds (one ignored in each), and 221 backend unit,
233 integration and one documentation test. Native/WASI Clippy and workspace
format/diff checks pass. Wasm remains an interpreter target; this does not claim
execution of the native call ABI there.

The [benchmark call inventory](../../target/gc-implementation/jit-workload-call-inventory/inventory.json)
preserves the source, four `--release 8` class files, hashes and `javap` output.
It contains 17 `invokespecial`, 22 `invokestatic` and 60 `invokevirtual` instruction
sites; none directly use `invokeinterface` or `invokedynamic`. `operation` has
three special-call sites, including the node constructor and assertion failures.
`main` has 46 virtual-call sites and remains a coverage dependency. These are
static site counts, not executed counts or a complete inventory of transitive JDK
dependencies. Virtual dispatch is the next direct workload call-support gap.

Production eligibility stays closed for calls. Other dispatch forms, full Java
access/initialization semantics, recursion, native reentrancy, monitors, relocation,
global progress and performance/endurance gates remain unfinished. Existing graphs
retain their original frozen-source meaning; this work makes no new speed claim.

## Experimental static calls through real VM frames

The native backend now lowers `invokestatic` to the existing VM dispatcher, with
owned arguments/results across interpreted, compiled and intrinsic callees.
Outgoing cells preserve primitive bits and stable reference identities; nested
activations own separate buffers. The shared resolver no longer initializes a
symbolic subclass before resolving and validating the method. Static invocation
initializes its actual declaring class.

Ten focused tests cover argument boundaries, mixed-width values, null/reference
returns, inherited and interface targets, wrong invocation kind, native calls,
exception ownership, and collection while the caller waits. The pressure matrix
checks both publication policies, zero/one registered worker and interpreted or
compiled callees; each invocation allocates 62.5 MiB in a 32 MiB heap, preserves a
caller-only operand and advances reclamation before returning. HotSpot 25 runs 23
checks against the same 19 exported fixture class files with full verification
and `-Xint`. The preserved earlier export failure used unsupported 45.0 fixtures;
the passing fixtures use Java 5 or Java 8 formats and are rerun in Ristretto too.

[Exact-class HotSpot verification](../../target/gc-implementation/jit-static-call-hotspot-verification-java5/manifest.json) ·
[Focused VM checks](../../target/gc-implementation/tests-jit-static-call-boundaries.log)

Validation: the release VM library suite passes 929 tests (one ignored); all 10
focused checks pass both with and without execution profiling. The final diagnostic
run explicitly asserts native entries for the inherited, interface and intrinsic
callers too. Backend suites pass 221 unit, 233 integration and one documentation
test. Native GC/reference integration suites pass six tests (one ignored), and
Wasmtime passes both WASI VM lifecycle tests. Native/WASI Clippy, workspace format
and diff checks pass. The initial Wasmtime command lacked the component's required
HTTP imports; the corrected runner enables them and uses the local preopened JDK.

The production eligibility gate remains unchanged. Calls still incur descriptor
conversion, argument allocation and an async bridge; no performance improvement
is claimed for this work. Other invocation forms, full access/initialization and
recursive/native/reentrant behavior, monitors, relocation and whole-workload
qualification remain open. Earlier benchmark graphs describe their frozen
implementations, not these new backend capabilities.

## JIT exception transfer releases ownership on unwind

A regression test reproduced a leaked root when an exception consumer panicked.
Pending exceptions now carry an owning handle, with pointer/owner replacement
paired under a context-local lock. Cleanup runs after unlocking; consumers receive
an owning handle, including during unwind.
The VM transfers the owner directly into its throwable result. Native VM tests,
the JDK reference matrix and native/Wasm Clippy pass. This fixes helper ownership;
production JIT eligibility remains restricted and no speedup is claimed.

Helper results now retain one owning handle per identity instead of appending a
numeric registration on every return. The reproduced 2,000-registration repeated
load retains one owner; concurrent distinct-result lifetime and cleanup checks pass.
Helpers without frame publication still retain distinct results until invocation
return. The subsequent frame handoff below replaces that set during covered
compiled execution. Neither change is a measured throughput result.

## Actual compiled execution is now observable

An opt-in build records production rejection reasons, compilation state and actual
generated-code entries, plus interpreted/intrinsic entries. The comparison runner
rejects diagnostic builds as timing inputs. A separate coverage runner checks the
shared workload's counts and produces graphs. Initial forks caught missing
interpreter fast-path recording; that path now has an exact-count regression test.
The preserved failed screen must not be treated as complete coverage evidence.
Phase-separated coverage, loop records, full dependency manifests and CPU attribution
remain open work. The production eligibility gate is unchanged.

The audit also fixed a reproduced mixed-width `dup2_x2` stack omission in simulation
and generated code. A test now consumes the complete stack result; native compiled
VM execution and HotSpot's verifier agree. This is a prerequisite correctness fix,
not evidence of GC or whole-JVM performance superiority.

The corrected baseline validates all nine native/WASI forks with frozen inputs
and no memory-ceiling events. Every required method has zero compiled entries;
native JIT forks enter only two small library methods. Counts include startup,
warmup and output. They establish a coverage gap, not a timing result.

[Native coverage report](../../target/gc-implementation/execution-coverage-current-native/REPORT.md) ·
[Native graph](../../target/gc-implementation/execution-coverage-current-native/coverage.svg) ·
[WASI coverage report](../../target/gc-implementation/execution-coverage-current-wasi/REPORT.md) ·
[WASI graph](../../target/gc-implementation/execution-coverage-current-wasi/coverage.svg)

## Compiler reference kinds and control flow

References now retain their Java category through operand stacks, block parameters
and helper continuations. New failing regressions exposed missed successors after
backward branches, unchecked fallthrough merges, incorrect exception-handler
stacks, dropped switch operands and rejected negative switch keys. The corrected
worklist and lowering pass backend tests and live interpreter comparisons.
HotSpot 25 verifies 33 matching results and rejects the invalid merge fixture.
Legacy `jsr`/`ret` lowering is rejected pending return-address analysis.

These compiler corrections do not qualify reference-bearing production methods.
The eligibility gate stays closed, and the earlier frozen
coverage report predates these compiler changes. No new performance claim is made.

Local lowering also supports legal Java slot reuse across types and reuse of a
wide argument's second slot. Reference and long SSA variables remain distinct.
The two primitive regressions now execute through the existing production gate;
explicitly interpreted execution and HotSpot agree. Reference-local liveness now
feeds the experimental publication protocol described below.

## Initial compiled-frame root transfer

A real compiled loop previously exhausted a 32 MiB heap while retaining discarded
helper results. Frame publication now replaces those owners at bytecode boundaries.
Four cases each complete 62.5 MiB of allocations before returning, preserving a
live local or operand under gated and counted publication. Transfer-failure and
stalled-context tests pass, along with backend/VM checks and native/WASI Clippy.

Backward local liveness now releases references after their last possible read,
including across Java slot reuse. Two approximately 20 MiB arrays allocated in
sequence previously exhausted a 32 MiB heap; the compiled regression now passes
under both publication policies. An additional allocating loop preserves a local
used only by its exception handler. HotSpot verifies matching bytecode/results.
Handler edges remain conservative. Small snapshots now reuse map storage and
unchanged owners, preparing missing owners before committing a changed set.
Larger sets still prepare a replacement map; runs of existing-owner copies share
one context lock, released before missing-identity validation and cleanup.
The production gate remains closed for these methods.
This establishes reclamation within the covered invocations; it does not qualify
all call/argument/suspension lifetimes, global progress, sustained load or G1/ZGC
superiority. Earlier timing/coverage artifacts predate this compiler work.

## Frame-publication helper cost

**Worker-count correction:** the three recorded schema-1 screens used the native
VM's one-worker configuration. Their zero-worker metadata label was incorrect;
`threads=0` selects one at VM startup. They recorded zero completed collections,
but did not sample worker registrations or steps. All reports now carry correction
notices. Schema 2 explicitly stops/joins workers and validates zero registrations;
fresh matched measurements under that contract are recorded below.

The paired `frame_roots.py` screen measures the real FFI helper with startup,
warmup, spills and instruction execution outside timing. Schema 2 verifies the
absence of native collector workers. Both binaries
must carry source provenance and the identical fixture. Every fork checks the
published set, contents and reclamation; missing/failed pairs remain visible.

The retained seven-pair, 196-process screen reduces gated stable-set medians from
55.68 to 13.15 ns/call (one root) and 223.20 to 26.64 ns/call (eight roots), with
similar counted results. The 64-root control has no clear change. Gated changing
sets remain slightly slower: paired ratios 1.038 [1.031, 1.065] for one root and
1.007 [1.001, 1.014] for eight. These are exploratory helper costs, not whole-JVM
throughput or latency tails. No broader performance gate is closed.

The first two screens are retained: one exposed a duplicated failed lookup, and
its initial correction added a per-entry branch that slowed the 64-root control.
Preparing the first missed owner before the loop removes that branch; the remaining
changing-set cost is still recorded. The ownership fast path is bounded to 32
input entries and current owners, and releases removed owners outside its lock.

[Report](../../target/gc-implementation/frame-publication-paired-hoisted/REPORT.md) ·
[Graph](../../target/gc-implementation/frame-publication-paired-hoisted/frame-roots.svg)

The subsequent schema-2 comparison uses the prior small-set reuse implementation
as its baseline. Small changing sets now preserve their map; a sole replacement
uses local handles, disjoint sets drain old entries directly, and the large fallback
shares one lock across already-owned inputs. Validation and destruction remain
outside that lock. Failed-transfer and post-context lifetime checks pass, as do
916 optimized VM tests, 68 focused checks and native/WASI Clippy.

Three exploratory screens and a fixed-binary confirmation preserve 784 valid
processes with no changed inputs or incomplete cells. The first attempt regressed
eight-root replacement, and the second still regressed the 64-root fallback;
those reports remain available. The confirmation uses seven process pairs with
one million calls and 100,000 warmup calls per process, with zero registered GC
workers verified before/after each measurement. It reduces changing-one-root cost
by 9–12% and stable-64-root cost by about 30%; all other intervals include one.
Gated medians are 212.41 → 190.53 ns and 1555.41 → 1078.28 ns, respectively.
These are helper costs, not latency tails, whole-JVM throughput or qualification
against G1/ZGC. Spare map capacity, sustained memory and contention remain unqualified.

[Schema-2 confirmation report](../../target/gc-implementation/frame-replacement-confirmation/REPORT.md) ·
[Schema-2 confirmation graph](../../target/gc-implementation/frame-replacement-confirmation/frame-roots.svg)

## Tree-root identity survives numeric wrap

A reproduced failure let a revoked owning guard remove a replacement tree root
after numeric-ID reuse. Internal tree registrations now include a non-reused
generation, and queued references carry the owner's allocation identity. Stale
cleanup preserves replacement roots; transfer rejects expired owner identities.
Tree entries retain only address and generation, avoiding unused tracing fields
and type-capability callbacks under publication admission. Native, Wasm, Miri and
JVM-reference checks pass. Tree locks and the measured contention failures remain.

All 980 paired timing processes pass. Allocation/root intervals include no change;
candidate handles cost +5.2% [1.0%, 7.1%] and pending transfer +8.9% [5.9%, 12.3%].
The latter now validates allocation identity. Forty native/Wasm allocator-count
processes show effectively unchanged requested bytes and process peaks in the
ephemeral fixture. The 60-second load screen completes 600,000 arrivals with clean
teardown, 58/502 µs operation p99.9/p99.99 and 32.0 MiB charged process peak.
These results support retaining the correctness fix, not a performance-win claim.

[Tree/queue timing](../../target/gc-implementation/mechanisms-tree-generation/REPORT.md) ·
[Timing graph](../../target/gc-implementation/mechanisms-tree-generation/mechanisms-diagnostics-False.svg) ·
[Native allocation counts](../../target/gc-implementation/allocations-tree-generation-native/REPORT.md) ·
[Wasm allocation counts](../../target/gc-implementation/allocations-tree-generation-wasi/REPORT.md) ·
[Load graph](../../target/gc-implementation/pressure-tree-generation/pressure-0.svg)

## Root acquisition and deletion progress through a stalled writer

Slot-root enumeration now uses an atomic page view. Acquisition reserves invisible
space before admission, then validates and publishes under admission. Three full
collections finish while a preparation writer is deliberately stalled; a weak
upgrade rejects a target reclaimed during that wait. Resident metadata remains
accounted for even when a cursor outlives its context.

Owning deletion now claims its checked generation atomically and publishes its
barrier before releasing occupancy. It owns the page through cleanup after
admission; busy writers receive deferred cleanup. A forced test drops the final
root and reclaims its graph through three full cycles while a writer remains
paused. Generation packing preserves registration size. All 383 GC tests, native
VM/reference tests, Clippy, focused Miri and actual Wasm checks pass.

The combined protocol has a measured cost versus the revision before these root
changes. Seven paired process trials show candidate allocation operation time
275.11 → 302.07 ns (paired ratio 1.096 [1.090, 1.100]), roots 79.97 → 97.09 ns
(1.259 [1.207, 1.262]), handles +16.5% and weak upgrade +13.4%. Ratios summarize
process pairs, not the displayed medians. Guard cloning is unchanged; control has
a broad +3.6% [1.9%, 16.5%] interval. All 840 processes pass. The implementation
remains experimental with a net overhead regression. Compatibility/tree removal,
other admitted waits and potentially large metadata cleanup still require work.

A 60-second load screen completes all 600,000 arrivals with clean teardown,
68/492 µs operation p99.9/p99.99 and 29.8 MiB charged process peak. It does not
qualify the longer sustained-load or external-comparison gates.

[Combined root report](../../target/gc-implementation/mechanisms-root-protocol-combined/REPORT.md) ·
[Mechanism graph](../../target/gc-implementation/mechanisms-root-protocol-combined/mechanisms-diagnostics-False.svg) ·
[Load graph](../../target/gc-implementation/pressure-root-protocol-combined/pressure-0.svg)

The new root-churn contention screen exposes the remaining progress failure.
With four producers, sixteen resident targets and 1,024 requested collections per
run, all 60 processes pass correctness but fourteen hit the ten-second deadline.
The candidate completes all collections in only three of five runs per generation
after these changes (previously two full and four young). Counted/candidate
completion ratios remain censored; higher achieved producer rates are not evidence
of reliable reclamation. This is a saturated mechanism screen with diagnostics,
not matched-arrival or endurance qualification.

[Root contention report](../../target/gc-implementation/contention-root-protocol/REPORT.md) ·
[Root contention graph](../../target/gc-implementation/contention-root-protocol/contention.svg)

## Cooperative Wasm comparison policy

The WASI comparison candidate now omits worker-oriented coordination/queues and
uses one metadata/region shard, while retaining counted publication and its root,
buffering, allocation and compaction policies. All 120 paired Java processes
validate across allocation, roots and mixed payloads, at saturation and 1,000
scheduled arrivals/s. The experiment isolates policy changes after the queue fix.

Candidate saturation throughput improves by 7.9% [7.4%, 9.3%], 11.8% [10.4%, 15.4%]
and 7.0% [6.5%, 10.5%] respectively; baseline controls include no change. Roots
peak process memory falls about 5%, while fixed-rate mixed peak grows 4.2%
[2.5%, 7.5%]. Fixed-rate tails remain too sparsely sampled and variable for a
latency claim. This remains an experimental benchmark configuration. The new
paired-revision reporter preserves complete samples, censoring and analysis code;
all 35 Python benchmark tests pass.

The browser fixture adopts the same removal of independent mark/phase claims
and stealing queues. All nine Chromium trials complete 100,000/100,000 requests
with clean content/reference/finalization checks and teardown. Candidate
request-turn p99.9 is 16–20 µs, baseline 20–21 µs and control 6 µs; arrival
p99.9 remains 4.7–5.0 ms across policies. This is a current-policy screen,
not a paired browser revision comparison or sustained qualification.

[Candidate comparison](../../target/gc-implementation/cooperative-policy-candidate-comparison/REPORT.md) ·
[Paired graph](../../target/gc-implementation/cooperative-policy-candidate-comparison/revision.svg) ·
[Baseline control](../../target/gc-implementation/cooperative-policy-baseline-comparison/REPORT.md)

[Browser report](../../target/gc-implementation/browser-cooperative-policy/REPORT.md) ·
[Browser graph](../../target/gc-implementation/browser-cooperative-policy/browser.svg)

## Pending-reference transfer

Queue consumers retain the existing root through transfer with a stack guard,
without keeping an outer publication admission. Three full cycles now finish
while transfer is deliberately paused, preserving the queued owner's child graph
under both policies. Error cleanup covers every root store. All 372 GC tests,
native VM references, Clippy, two Miri cases, seven actual WASI tests and bare-Wasm
smoke pass.

All 700 mechanism processes validate. Candidate empty polling changes from 21.39
to 11.74 ns (ratio 0.555 [0.541, 0.574]); populated transfer changes from 156.32
to 149.39 ns (0.959 [0.926, 0.962]). Control shifts 21%, so this does not establish
an across-the-board speedup. External/load measurements below precede this change.

[Transfer report](../../target/gc-implementation/mechanisms-pending-transfer/REPORT.md) ·
[Transfer graph](../../target/gc-implementation/mechanisms-pending-transfer/mechanisms-diagnostics-False.svg)

## Refreshed native, WASI and browser results

All 120 native and 40 WASI Java processes validate on the frozen root-guard revision,
with equal CPU/memory limits, five paired forks, two-second measurements and
one-second warmup. Native interpreter candidate throughput is 171,056/s for
allocation and 10,126/s for roots; G1 records 25,535,588/s and 15,381,422/s, and
ZGC records 30,703,440/s and 15,128,520/s. Candidate JIT records 155,273/s and
9,613/s. Both candidate modes fail every short performance comparison. These are
whole-JVM measurements; the gap includes interpreter/JIT/runtime costs.
The reference-bearing `GcBench.operation` method is ineligible under the current
production JIT gate, so enabling the JIT does not compile the main allocation
workload. Compiled-frame root tracing and control-flow/call support remain required
before widening eligibility; focused backend tests do not establish that safety.

WASI candidate allocation/roots medians are 79,553/6,267 per second versus
baseline 86,380/8,693. Fixed-rate roots tails remain censored by unfinished work.
Nine browser trials finish every request with clean teardown; candidate request-turn
p99.9 is 16–20 µs, baseline 16–26 µs, control 6 µs. The separate 60-second payload
driver completes 600,000 arrivals without backlog, with 57/504 µs operation
p99.9/p99.99 and 32.4 MiB charged process peak. None qualifies sustained superiority.
These measurements precede the subsequent pending-reference transfer change.

[Native report](../../target/gc-implementation/java-native-root-guard/REPORT.md) ·
[Native throughput graph](../../target/gc-implementation/java-native-root-guard/graphs/native-throughput.svg) ·
[Native latency graph](../../target/gc-implementation/java-native-root-guard/graphs/native-latency.svg) ·
[WASI report](../../target/gc-implementation/java-wasi-root-guard/REPORT.md) ·
[Browser graph](../../target/gc-implementation/browser-root-guard/browser.svg) ·
[Load graph](../../target/gc-implementation/pressure-root-guard/pressure-0.svg)

## Root-guard deletion barrier

A guard's matching inner edge now relies on its still-owned registration to
publish deletion, removing a duplicate weak-collector upgrade and barrier.
Replaced/foreign edges retain their barrier. Transfer and unwind preserve exact
registration ownership. All 370 GC tests, native VM references, Clippy, three Miri
cases, fourteen actual WASI tests and bare-Wasm smoke pass.

All 1,260 paired mechanism processes pass. Candidate guard clone/drop falls from
27.49 to 18.16 ns (ratio 0.661 [0.655, 0.672]); temporary allocation falls from
289.45 to 280.29 ns (0.962 [0.954, 0.971]). Other tested mechanism intervals include
one; control changes about 1%. These collector-only results do not establish a
Java throughput or sustained-load improvement.

[Root-guard report](../../target/gc-implementation/mechanisms-root-guard-barrier/REPORT.md) ·
[Root-guard graph](../../target/gc-implementation/mechanisms-root-guard-barrier/mechanisms-diagnostics-False.svg)

## Identity lookup before admission

Identity-handle reconstruction now looks up an address hint before admission,
then revalidates identity, type and logical eligibility under admission. A paused
lookup therefore permits full collection without exposing stale storage. Forced
schedules cover both a live graph and a target retired during the pause, under
both publication policies. All 369 GC tests and the native/Miri/WASI handle checks
pass. All 560 mechanism processes pass; candidate handle time changes from 105.7
to 102.9 ns (ratio 0.972 [0.970, 0.986]), while root/allocation intervals include one.

[Handle report](../../target/gc-implementation/mechanisms-identity-admission/REPORT.md) ·
[Handle graph](../../target/gc-implementation/mechanisms-identity-admission/mechanisms-diagnostics-False.svg)

## Barrier working sets and optional cache isolation

The retained cache has two recent records plus sixteen entries in four sets.
Exact epochs and addresses are required; dirty notifications remain independent.
Sustained misses trigger a bounded bypass. The cache has its own try-lock, so a
paused cache holder cannot delay required barrier records or their acknowledgement.
Native context storage increases by 768 bytes to 6,144 bytes, included in diagnostics.

All 1,120 mechanism processes pass. Candidate one/four/sixteen-edge barrier ratios
are 0.963 [0.950, 0.979], 0.338 [0.329, 0.342] and 0.629 [0.598, 0.814]. The
sixty-four-edge ratio is 1.011 [0.99952, 1.01736]; allocation/root/overwrite
intervals include one. The initial fixed cache's 8% sixty-four-edge regression is
preserved in its own report. Control variation is about 7%; this is not a Java
throughput comparison.

All 120 final contention processes finish, but broad intervals do not establish a
sustained speedup. A separate 60-second run completes all 600,000 arrivals with
zero backlog and clean managed/root/cache/retired teardown. The revision passes
368 GC tests, GC/classloader/types/VM Clippy, native reference checks, Miri,
eight WASI cache tests and bare-Wasm smoke. The forced held-cache test completes
three full cycles under both publication policies.

[Mechanism report](../../target/gc-implementation/mechanisms-barrier-cache-isolation/REPORT.md) ·
[Mechanism graph](../../target/gc-implementation/mechanisms-barrier-cache-isolation/mechanisms-diagnostics-False.svg) ·
[16-target contention](../../target/gc-implementation/contention-barrier-cache-isolation16/REPORT.md) ·
[64-target contention](../../target/gc-implementation/contention-barrier-cache-isolation64/REPORT.md) ·
[Load evidence](../../target/gc-implementation/endurance-barrier-cache-isolation.json)

## Weak-slot contention and constructor publication

Weak operations release publication admission before waiting on a slot lock and
retry their decisions afterward. New and cloned slot allocation also precedes
admission. Owner binding follows descriptor registration and admission release;
temporary roots and the pending-owner count keep the conditional graph protected.
Reference processing skips pending owners before trying the slot lock and checks
again after locking. A paused binding no longer holds unrelated full cycles open.

Three forced native schedules cover young reclamation with upgrade/clone/owner
waiters, weak decisions crossing a wait, and three full cycles during blocked
owner binding. They pass under both policies. The final revision passes 364 GC
tests, the updated Loom reference model, GC/classloader/types/VM Clippy, native VM
GC/reference tests, Miri weak and construction/unwind checks, nineteen actual WASI
reference tests and bare-Wasm smoke. Non-construction slot holders and stalled
successful admissions can still defer progress.

All 700 final mechanism processes pass. Candidate weak-upgrade, allocation and
barrier ratios include one; roots show a small increase (1.006 [1.004, 1.031])
while control changes about 9%. The revision is retained for reclamation progress.
A 60-second load check completes 600,000 requests with zero backlog and empty
managed/root/cache/retired teardown; operation p99.9/p99.99 are 57/497 µs and
charged process peak is 32.2 MiB. This does not qualify sustained performance.

[Final operation report](../../target/gc-implementation/mechanisms-weak-publication-final/REPORT.md) ·
[Final operation graph](../../target/gc-implementation/mechanisms-weak-publication-final/mechanisms-diagnostics-False.svg) ·
[Final load evidence](../../target/gc-implementation/endurance-weak-publication-final.json) ·
[Pressure graph](../../target/gc-implementation/pressure-weak-publication-final/pressure-0.svg)

## Weak identities and metadata ownership

Weak handles now keep the original nonreused object ID and validate it in the
address index. This fixes stale-edge weak creation after address reuse and lets
collected targets release metadata while cleared weak handles remain alive.
Internal metadata ownership uses `triomphe::Arc`, without a weak counter; graph
roots, storage leases and phase/slot decisions retain their existing contracts.
The rejected compact-handle representation remains removed.

All 1,680 mechanism processes pass. Candidate weak upgrade changes from 143.9 to
127.8 ns, paired ratio 0.881 [0.879, 0.898], while allocation/root/barrier changes
are inconclusive. Forty native allocation-count processes show eight fewer
requested bytes per ephemeral allocation and 216 fewer retained bytes per cleared
weak handle, with no clear process-peak reduction. Eight actual WASI runs show
about 120 fewer retained bytes per cleared weak handle and unchanged ephemeral
allocation size. Counted retained bytes include all live system allocations.

The revision passes 361 GC tests, GC/classloader/types/VM Clippy, native VM
GC/reference checks, eight Miri lifetime cases, ten actual WASI reference tests
and bare-Wasm smoke. These results do not qualify G1/ZGC superiority or eliminate
stalled publication and opaque tracing callbacks.

[Operation report](../../target/gc-implementation/mechanisms-strong-metadata/REPORT.md) ·
[Operation graph](../../target/gc-implementation/mechanisms-strong-metadata/mechanisms-diagnostics-False.svg) ·
[Allocation report](../../target/gc-implementation/allocations-strong-metadata/REPORT.md) ·
[Weak retention report](../../target/gc-implementation/weak-retention-strong-metadata/REPORT.md)

## Implemented changes

- Counted publication admits and stamps in one atomic CAS; phase closure competes on the same word. Overflow fails before admission; revision saturation rebases without changing phases.
- Allocation-cache operations borrow the current context without transient Arc ownership updates. Cache credit, refill and cancellation rules are preserved.
- Barrier insertion checks only the two most recent records, coalescing identical address/epoch/kind records. Alternating deletion edges and dirty owners keep distinct reservations. Popped records cannot absorb later writes.
- Busy incremental clearing transfers its offset, finalized state and charged bytes to the rotating retirement queue, allowing unrelated sweep work and later cycles to complete.
- Requested full collection drains an earlier young snapshot before starting, preventing repeated import of floating roots. Mutators keep running.
- Optional inclusive timings attribute descriptor publication, root acquisition/release, barrier recording and drain. The Java-payload endurance/recovery drivers export them.
- Benchmark tooling adds paired bootstrap intervals, per-child cgroup CPU/memory limits, CPU/memory time series, short comparison gates, and the previously missing endurance/provenance scripts used by CI.

## Collector-only before/after measurements

Seven randomized process pairs per condition, 100,000 operations per process, one mutator on CPU 0. The combined policy below uses counted publication, context roots/buffers and local descriptor caching. It is a smaller feature set than the end-to-end VM candidate. Operation time excludes between-batch collection; raw records also retain wall time. Diagnostics are disabled in this table.

| Mechanism | Before ns/op | After ns/op | After/before median ratio [95% CI] |
|---|---:|---:|---:|
| allocation | 324.0 | 302.5 | 0.933 [0.922, 0.953] |
| barrier | 82.6 | 35.5 | 0.431 [0.424, 0.447] |
| control | 2.3 | 2.4 | 1.048 [0.994, 1.052] |
| overwrite | 195.0 | 113.3 | 0.581 [0.565, 0.605] |
| roots | 99.4 | 92.4 | 0.932 [0.925, 0.959] |

All 700 final mechanism processes passed checksum and teardown checks. Intervals resample process pairs; they are exploratory and have no multiple-comparison adjustment. The baseline immediate-barrier condition regressed about 0.7%; every condition remains in the report. Instrumentation increases combined-policy operation time about 1.51× for allocation, 2.25× for roots, 2.07× for barriers and 1.71× for overwrites. Do not compare diagnostic-on samples against diagnostic-off samples as implementation speedups.

[Full table and raw evidence](../../target/gc-implementation/mechanisms-final/REPORT.md) · [Graph](../../target/gc-implementation/mechanisms-final/mechanisms-diagnostics-False.svg) · [Instrumentation graph](../../target/gc-implementation/mechanisms-final/mechanisms-diagnostics-True.svg)

Frozen binary SHA-256:

- Before: `d4075879b42653c81dd33712fdd24a13ab2783a1717b96be39943704910f26c7`
- Final: `02c95530019048b98aa0736a87a1c61b36be0af82cfd8232795b1f8d3c1c9a5a`

Source archives accompany the binaries under `target/gc-implementation/before` and `after-final`. An intermediate adjacent-only coalescing experiment is retained under `mechanisms`; its inconclusive overwrite result led to the bounded two-record window.

## Shared Java workload

Final native: 80 processes; final WASI: 40 processes. Five forks per condition, two seconds measured after one second warmup, 64 MiB logical heap, CPUs 0 and 2, a two-core total CPU quota, 512 MiB charged-memory ceiling, and no swap. Native Ristretto uses two GC workers; WASI is cooperative. All processes passed workload checks, all inputs remained unchanged, and every offered request completed by the measurement boundary. These short screens do not validate warmup stability or extreme tails.

HotSpot uses its default JIT; Ristretto interprets the identical Java 8 bytecode with the same Corretto 25.0.5 class library. These numbers include interpreter/compiler and runtime costs and cannot rank isolated collector algorithms.

| Native runtime | Allocation ops/s | 16-node root workload ops/s |
|---|---:|---:|
| g1 | 25,598,735 | 15,443,304 |
| ristretto-baseline | 147,195 | 10,347 |
| ristretto-candidate | 135,054 | 9,439 |
| zgc | 30,867,143 | 15,445,849 |

| WASI runtime | Allocation ops/s | 16-node root workload ops/s |
|---|---:|---:|
| wasm-baseline | 76,947 | 8,358 |
| wasm-candidate | 67,420 | 5,625 |

The VM candidate remains slower than baseline in these saturation screens. At 5,000 offered root operations/s, native candidate arrival p99.9 was 296 µs versus 29,952 µs for baseline, but G1/ZGC were about 130/132 µs. WASI candidate root p99.9 regressed to 307,200 µs versus 208,896 µs for baseline. These are medians of per-process quantiles with only 10,000 requests per fixed-rate run, not validated extreme-tail estimates. The complete feature combination is therefore not promoted to defaults.

`qualify.py` correctly exits nonzero: both external throughput targets fail and fixed-rate runs lack the required million completions per run. Overall qualification remains false.

[Native report](../../target/gc-implementation/java-native-final/REPORT.md) · [Native graphs](../../target/gc-implementation/java-native-final/graphs/native-throughput.svg) · [Gate failures](../../target/gc-implementation/java-native-final/gates.json) · [WASI report](../../target/gc-implementation/java-wasi-final/REPORT.md) · [WASI graphs](../../target/gc-implementation/java-wasi-final/graphs/wasm-throughput.svg)

## Correctness and portability

- 323 GC tests pass, including production-state Loom admission models, context migration/cancellation, overflow, construction, reference semantics and the new clearing-progress regressions.
- The concurrent allocation regression passed 100 isolated repetitions after the full-start handoff fix. Each repetition exercises four publication/generation configurations.
- Clippy with warnings denied passes for GC targets and the VM/endurance/recovery examples.
- Native and WASI release builds pass. The final bare-Wasm `gc_smoke` export executes successfully in Wasmtime; this is correctness evidence, not browser event-loop qualification.
- Python protocol tests and a real cgroup OOM probe verify failure retention and that a requested memory ceiling is enforced. The OOM probe is intentionally a failed child, not a benchmark failure.

## Instrumented pressure checks

A 60-second candidate run at 10,000 allocations/s completed all 600,000
requests with zero final backlog and zero managed objects, root pages, retired
bytes, pending records or reservations after teardown. It used two workers, a
64 MiB logical heap, the same two-core/512 MiB process ceilings, compact payloads,
context/local caches, counted publication and independent phase/mark/relocation
work. Diagnostics were enabled. Operation p99.9 was 55 µs;
arrival p99.9 was 443 µs; whole-process peak cgroup memory was
30.4 MiB. Samples describe this short run only.

Five-second recovery checks also passed, including cooperative async recovery:

| Recovery mode | Workers | Completed operations | Operation p99.9 µs | Executor poll p99.9 µs | Timer delay p99.9 µs |
|---|---:|---:|---:|---:|---:|
| async | 0 | 1,725,604 | 29 | 42 | 910 |
| async | 2 | 2,217,220 | 9 | 38 | 1104 |
| sync | 2 | 1,948,734 | 7 | 2688 | 3584 |

These single trials are not paired speedup estimates or sustained-load qualification.
The async threaded driver also asserts that ordinary allocations did not enter
synchronous recovery. The earlier one-second harness smoke outputs are retained;
the first parser rejected their map-shaped histograms, which was corrected and
covered by protocol tests before the final pressure runs.

[Raw endurance evidence](../../target/gc-implementation/endurance-final.json) ·
[Memory/backlog graph](../../target/gc-implementation/pressure-graphs-final/pressure-0.svg)

## Remaining release gates

Counted admission still cannot close a phase while a publisher remains admitted. Blocking tracing and reference-bearing managed write guards still require a complete progress/access protocol. Metadata/slab allocation, root retirement batching, validated sustainable capacity, broader held-out workloads, browser host-turn measurements, and statistical memory-trend gates remain open. Three paired one-hour trials and a separate 24-hour qualification were not run because preceding performance/progress gates have not passed.

## Follow-up: construction and reclamation progress

Conditional-owner attachment now roots eligible referents before publishing a
per-slot pending count. Enumeration skips that slot while its owner is unpublished;
it can finish full collections and reclaim unrelated graphs. Success binds the
owner's exact identity before temporary roots are released. Failure leaves the
owner unbound. Identity and closed weak/retirement checks prevent resurrection.
Reentrant collection and repeated owner assignment are covered for all five
reference kinds under both publication and phase policies.

Sweep finalization, clearing and destruction now run outside cycle, coordinator
and publication locks. Executing descriptors remain charged. Continuations return
to their original sweep only if its epoch still matches; otherwise they enter the
retirement queue. Forced schedules hold each callback through three later full
collections and verify exactly-once destruction. Normal callbacks still consume
the remaining step budget. Deferred clearing also releases the coordinator in
the compatibility policy. This isolates these callbacks; it does not preempt them
or remove tracing reservations. Threadless Wasm still requires bounded callbacks.

The progress revision passed 327 GC tests, native VM tests, the five-JDK reference
matrix, GC/benchmark Clippy, bare-Wasm smoke, and WASI construction/clearing checks.
Miri passed conditional construction/unwind and blocked full clearing. The subsequent
allocation-owner revision has separate checks and measurements below.

Seven independent mechanism pairs per condition (700 successful processes) compare
this progress revision against the preceding delivery. Candidate allocation
operation time was unchanged within the interval (ratio 1.004, 95% CI 0.974–1.015).
Total allocation-batch time including collection increased by 8.0% (ratio 1.080,
CI 1.043–1.086). Root and overwrite timings were approximately unchanged. The added
ownership transfer has a measured cost; it is not a throughput improvement.

[Progress comparison](../../target/gc-implementation/mechanisms-progress/REPORT.md) ·
[Collection-inclusive graph](../../target/gc-implementation/mechanisms-progress/mechanisms-wall-diagnostics-False.svg)

Native JIT variants now select a multithreaded executor with one worker, check
`VM::is_jit_enabled`, and emit the actual mode. The runner rejects a mismatched
runtime label. Existing interpreter modes remain separately selectable. A single
two-second smoke fork completed correctly: allocation throughput was about 135,628
ops/s interpreted versus 125,881 with JIT enabled; root churn was about 9,617 versus
9,559. This is preliminary, single-fork evidence, not a speedup or compiler-coverage
claim. Unsupported methods can still interpret. It does not close the HotSpot gap.

[JIT smoke evidence](../../target/gc-implementation/jit-progress-smoke/REPORT.md)

Production `ristretto_jit::Compiler::can_compile` currently restricts compilation
to static, straight-line primitive methods. Reference operations and general
control flow remain excluded while compiled-frame GC participation and control-flow
correctness are incomplete. Enabling the compiler therefore does not compile the
allocation-heavy benchmark operation. Its end-to-end gap includes this execution
limitation; it cannot be attributed solely to the collector.

## Allocation-owner measurements

Erasing initialized descriptor storage directly removes the extra boxed destructor
closure. The revision passed 329 GC tests, Clippy, and Miri storage-ownership,
destructor-unwind and legacy-construction fixtures. Seven paired mechanism trials
per condition (700 successful processes) found no established allocation timing
change: candidate operation ratio 1.001 (95% CI 0.979–1.022), collection-inclusive
ratio 0.993 (0.977–1.025). This did not establish recovery of the preceding progress
revision's collection cost.

An independent allocator-count lane uses the same driver source for both revisions,
100,000 32-byte payloads and five pairs per policy. All twenty processes passed
checksum and teardown checks. Candidate allocator requests fell from 3.350 to 2.350
per allocation (ratio 0.702); baseline fell from 4.333 to 3.333. Candidate requested
bytes fell by about 8 bytes per allocation. Candidate whole-process cgroup peak
memory fell from 43.79 to 40.54 MiB; the paired peak-memory ratio was 0.930
(95% CI 0.926–0.931). The allocation phase retains garbage until subsequent
collection, and has no background worker. Requested bytes omit allocator headers
and fragmentation; charged peak memory covers the whole process. Counted runs
are separate from timing runs.

[Allocation timing](../../target/gc-implementation/mechanisms-erased/REPORT.md) ·
[Allocator counts](../../target/gc-implementation/allocations-erased/REPORT.md) ·
[Count and memory graphs](../../target/gc-implementation/allocations-erased/allocations.svg)

## Leaf tracing and owning-handle follow-up

Declared reference-free values now complete mark/handoff claims without invoking
tracing or borrowing storage. Managed, standard and portable lock wrappers propagate
that declaration. Forced tests retain a write guard through old/young/full collection,
check that its storage stays pinned, and verify bounded, exactly-once element
destruction. Custom trace actions remain enabled by default.

Owning `Gc` handles now retain only their shared root registration. Heap-edge
handles retain weak collector access instead. Conversions preserve the deletion
barrier and establish the destination ownership mode before releasing the source.
Identity reconstruction registers directly after validation. The follow-up passes
333 GC tests, GC Clippy, native VM GC/reference checks, twelve WASI regression tests,
bare-Wasm smoke, and four Miri tests covering leaf contention and root conversion.

Seven paired mechanism trials per condition (840 successful processes) compare
owning-handle changes with the preceding leaf revision. The historical clone lane
created its root outside the context; it measures shared `Arc` owners. Candidate cloning time
fell from 17.91 to 10.54 ns/op: ratio 0.588 (95% CI 0.586–0.595). Root registration
fell from 92.94 to 84.99 ns/op: ratio 0.916 (0.911–0.922). Allocation time changed
from 304.84 to 299.96 ns/op; its collection-inclusive ratio includes one in its
interval. The control changed by about 8%, and barrier time rose by about 1%; small
timing differences should not be overinterpreted. These are unadjusted exploratory
intervals, and operation timing excludes between-batch collection.

[Mechanism report](../../target/gc-implementation/mechanisms-owner/REPORT.md) ·
[Mechanism graphs](../../target/gc-implementation/mechanisms-owner/mechanisms-diagnostics-False.svg)

The new five-fork native comparison preserves the same two-second measurement,
one-second warmup, 64 MiB logical heap, two-core CPU ceiling and 512 MiB physical
ceiling. All eighty processes succeed. Median saturated allocation throughput is
141,288 ops/s for candidate, 151,724 for baseline, 25.46 million for G1 and 30.89
million for ZGC. Root-workload throughput is 9,829, 10,901, 15.36 million and 15.44
million respectively. At 5,000 offered operations/s, candidate arrival p99.9 is
120 µs for allocation and 318 µs for roots; G1 is 167/169 µs and ZGC 149/174 µs.
These short trials still fail the declared external gate, with 52 failed checks,
including throughput and insufficient extreme-tail samples.

All forty WASI trials also execute successfully. Candidate saturation reaches
66,727 allocation ops/s and 6,365 root-workload ops/s, versus baseline 76,162 and
8,763. The 5,000/s root workload leaves unfinished requests under both policies;
its completed-request latency is censored and cannot pass the latency gate.

[Native comparison](../../target/gc-implementation/java-native-owner/REPORT.md) ·
[Native graphs](../../target/gc-implementation/java-native-owner/graphs/native-throughput.svg) ·
[WASI comparison](../../target/gc-implementation/java-wasi-owner/REPORT.md) ·
[WASI graphs](../../target/gc-implementation/java-wasi-owner/graphs/wasm-throughput.svg)

The fresh 60-second instrumented pressure run completes all 600,000 offered requests
with zero final backlog, managed objects, occupied bytes and final root/cache/retired
counters. Whole-process cgroup peak memory is 32.13 MiB. This is a short correctness
and pressure check, not the required repeated one-hour and separate 24-hour runs.

The capacity tool's real-process smoke runs two forks per point and finds a
candidate native root-workload screening bracket of 8,000 passing versus 8,500
failing operations/s, with both endpoints confirmed by fresh forks. The declared
limits are arrival p99.9 ≤1 ms, peak backlog equivalent to ≤10 ms of offered work,
and a 1,000-completion sample floor. At 8,500/s, one initial fork passes and one
fails; the point fails because every fork must pass. Both confirmations at 8,500/s
fail. All overload points remain visible. Two-second adaptive points do not establish
sustainable capacity, tail qualification or a peer comparison.

[Pressure evidence](../../target/gc-implementation/endurance-owner.json) ·
[Pressure graphs](../../target/gc-implementation/pressure-graphs-owner/pressure-0.svg) ·
[Capacity evidence](../../target/gc-implementation/capacity-owner-smoke/REPORT.md) ·
[Capacity graph](../../target/gc-implementation/capacity-owner-smoke/capacity-roots.svg)

## Rejected root-record cache experiment

A bounded per-context cache reused empty `Arc` owner records with uniqueness
checks and nonblocking cache locks. It passed the GC suite, four dedicated lifetime,
contention and detach regressions, Clippy, and three Miri tests. Nevertheless, seven
paired mechanism trials per condition found candidate allocation 5.6% slower
(95% CI 3.9–7.2%), root registration 24.5% slower (22.9–25.6%), and cloning 15.3%
slower (13.8–17.1%). Its bookkeeping cost outweighed removing an allocator request.
The prototype was removed from production source. Its frozen source and binaries,
all 840 timing processes and twenty allocator-count processes remain available;
no failed comparison was discarded.

[Rejected timing report](../../target/gc-implementation/mechanisms-root-records/REPORT.md) ·
[Rejected allocator-count report](../../target/gc-implementation/allocations-root-records/REPORT.md)

## Rejected fixed owner-slot experiment

A second prototype used 32 preallocated context slots, with an ordinary shared
owner fallback. It passed 339 GC tests, Clippy, three Miri lifetime tests, a Miri
construction test, and nine WASI root tests. It removed one system allocation per
short-lived root: candidate allocator calls fell from 2.35024 to 1.35024 per managed
allocation. Requested bytes fell from 492.34 to 428.34; peak physical memory was
unchanged. Root registration was about 1.3% slower. The initial clone fixture created
an unscoped owner, so it could not evaluate slot-backed clones.

The corrected driver creates the shared root inside the context and verifies its
registration. Identical corrected driver source was overlaid on the accepted
baseline. Seven paired trials per condition then showed candidate clones 22.6%
slower (95% CI 18.7–25.1%). A follow-up combined slot claim/retirement with the
reference count and used an increment/release path like `Arc`. Its production
atomic state passed Loom and the full 339-test GC suite. In 840 fresh processes,
allocation showed no improvement: ratio 1.018 (0.991–1.022); root registration was
1.005 (0.989–1.073), and scoped cloning still regressed to 1.068 (1.065–1.252).
The fixed-slot code was removed. The corrected benchmark fixture remains; earlier
clone results retain their narrower unscoped-owner interpretation.

[First slot timing](../../target/gc-implementation/mechanisms-owner-slots/REPORT.md) ·
[Allocator counts](../../target/gc-implementation/allocations-owner-slots/REPORT.md) ·
[Corrected scoped clones](../../target/gc-implementation/mechanisms-owner-slots-scoped/REPORT.md) ·
[Combined-state timing](../../target/gc-implementation/mechanisms-owner-slots-packed/REPORT.md)

## Compact metadata follow-up

Immutable tracing operations now use a shared per-type table. Metadata reuses its
original value pointer for identity lookup and typed finalizer dispatch; finalizers
no longer allocate a captured closure. Leaf tracing, per-object work budgets,
callback isolation and exactly-once destruction are unchanged.

All 333 GC tests, Clippy, the Miri constructor/unwind matrix, eleven actual WASI
regressions and bare-Wasm smoke pass. Seven paired mechanism trials per condition
produce 840 successful processes. Candidate allocation changes from 300.5 to
296.7 ns/op, but the paired ratio 0.991 (95% CI 0.967–1.011) does not establish a
speedup. Collection-inclusive allocation is 1.001 (0.974–1.011). Root registration
is about 0.8% slower, and the allocation-free control changes by about 20%; small
timing differences remain sensitive to code generation and this short fixture.

The independent twenty-process allocator-count lane shows unchanged request
counts and 32 fewer requested bytes per managed allocation: candidate 492.34 to
460.33 bytes. Whole-process charged peak falls from 40.54 to 37.54 MiB, paired
ratio 0.926 (0.923–0.926). Baseline peak falls from 38.79 to 35.79 MiB. This fixture
has no finalizers; the additional removed finalizer allocation is a code change,
not a measured result in these counts. The memory reduction justifies retaining
this revision without claiming a throughput win.

[Timing report](../../target/gc-implementation/mechanisms-compact-metadata/REPORT.md) ·
[Allocator and memory report](../../target/gc-implementation/allocations-compact-metadata/REPORT.md) ·
[Memory graph](../../target/gc-implementation/allocations-compact-metadata/allocations.svg)

## Reusing root publication admission

Typed identity resolution, raw typed resolution and weak upgrades reuse one
collector-specific token from validation through root insertion. Native regressions
observe exactly one active admission during insertion and reject foreign-collector
tokens. The full suite passes 335 tests. The initial representation added a collector
pointer to every publication guard and regressed allocation about 2.3%. A separate
root-publication token preserves ordinary allocation/barrier guard size.

The final 1,120-process screen compares identical extended drivers with the compact
metadata revision. Candidate handle resolution improves from 125.2 to 105.6 ns/op:
paired ratio 0.844 (95% CI 0.815–0.853). Weak upgrades improve from 166.8 to 149.0:
ratio 0.893 (0.886–0.897). Allocation is 1.002 (0.997–1.009), collection-inclusive
allocation 0.999 (0.994–1.005), and ordinary root registration 0.993 (0.991–1.004).
The control is about 5% slower, so small secondary changes remain inconclusive.
These are single-mutator mechanism measurements, not external GC qualification.
A stalled outer admission still blocks phase closure.

[Initial token screen](../../target/gc-implementation/mechanisms-shared-admission/REPORT.md) ·
[Final token screen](../../target/gc-implementation/mechanisms-root-admission/REPORT.md) ·
[Final graphs](../../target/gc-implementation/mechanisms-root-admission/mechanisms-diagnostics-False.svg)

## Retained object-identity batching

The allocator now reserves 64 IDs per thread from the process-wide atomic counter.
Its 16-byte local range contains no collector ownership; unused values are never
returned, IDs can have gaps and do not encode cross-thread allocation order. The
terminal partial range is usable without wrapping, and previously reserved ranges
survive global exhaustion. A cold fallback handles unavailable TLS during teardown.

The isolated production-allocator screen has 42 successful processes: seven
independent pairs, 1/2/4 producers, one million IDs per producer, four CPUs/quota and
256 MiB process caps. It includes output writes to preallocated vectors; validation
and sorting are outside timing. Median wall ns/ID changes from 12.426 to 7.576 for
one producer, 15.168 to 2.932 for two and 15.350 to 2.164 for four. Corresponding
paired ratios are 0.639 [0.194, 0.713], 0.167 [0.118, 0.260] and 0.143
[0.101, 0.177]. This demonstrates less shared-counter contention, not GC or Java
throughput superiority.

The 1,680-process GC operation screen passes without changed inputs. Candidate
general allocation ratio is 0.996 [0.977, 1.004], fresh roots 0.995 [0.983, 1.001]
and barriers 1.004 [0.980, 1.011]. Owning allocation has a small ratio improvement,
0.982 [0.965, 0.999], while control varies about 9%; small differences remain
inconclusive. The implementation passes 360 GC tests, four Miri cases, three actual
WASI identity tests, bare-Wasm smoke, native VM/reference tests and all-target
GC/classloader/types/VM Clippy. All 28 Python harness tests pass.

A final 60-second native check completes 600,000/600,000 requests with no final
backlog and empty managed, root, cache and retired state. Its process peak is
30.6 MiB. This single short run is not a memory-improvement or endurance claim.
The G1/ZGC and browser reports below precede the ID-only change; their failed
qualification gates remain outstanding.

[ID report](../../target/gc-implementation/identities-batched/REPORT.md) ·
[ID graph](../../target/gc-implementation/identities-batched/identities.svg) ·
[GC operation report](../../target/gc-implementation/mechanisms-identity-batch/REPORT.md) ·
[GC operation graph](../../target/gc-implementation/mechanisms-identity-batch/mechanisms-diagnostics-False.svg) ·
[Short load evidence](../../target/gc-implementation/endurance-identity-batch.json)

## Rejected compact metadata-backed handles

This experiment is removed from the implementation. Storing metadata ownership in
heap edges removes the inline object identity and shrinks native handles from 32
to 24 bytes. The first prototype leaves root identities inline and looks up
metadata when unrooting; its 1,120-process screen raises candidate unroot cost from
29.5 to 40.4 ns (paired ratio 1.369 [1.309, 1.399]). The revised prototype puts
metadata into the owning registration, removes that lookup, and also shrinks Wasm
handles from 24 to 12 bytes. Its 1,120-process screen restores unroot cost but raises
fresh-root cost from 78.8 to 87.3 ns (ratio 1.105 [1.094, 1.125]). Owning clones and
allocation are roughly unchanged. Control varies 11–14%, so small differences
are inconclusive. The native VM `Value` measures 24 bytes in the revised prototype;
no prior `Value` size reduction is inferred without a matching layout measurement.

Forty randomized paired native Java processes compare the same candidate policy
in the retained barrier build and revised prototype. All complete successfully
with unchanged inputs, two CPUs/quota, 64 MiB managed heaps, 512 MiB process caps,
five pairs, two-second measurements and one-second warmup. Root saturation
throughput is lower: paired after/before ratio 0.970 [0.952, 0.985]. Process RSS is
higher: 1.061 [1.051, 1.072]. Allocation throughput is inconclusive at 1.014
[0.981, 1.045], while saturation RSS rises to 1.045 [1.019, 1.112]. Saturation root
p99.99 measurements also worsen, with insufficient samples for tail qualification.
The combined result does not justify retaining either representation. The original
core is restored; lifetime tests and the extended comparison harness remain.

The revised prototype passed 356 GC tests, GC/classloader Clippy, native VM GC and
the JDK reference matrix, ten actual WASI layout/lifetime/root-slot tests and Miri
lifetime plus legacy constructor unwind/finalizer checks. These checks establish
neither performance superiority nor the full no-global-blocking objective.

[Initial mechanism report](../../target/gc-implementation/mechanisms-metadata-edge/REPORT.md) ·
[Revised mechanism report](../../target/gc-implementation/mechanisms-metadata-root/REPORT.md) ·
[Java report](../../target/gc-implementation/java-native-metadata-root/REPORT.md) ·
[Paired ratios](../../target/gc-implementation/java-native-metadata-root/paired.json) ·
[Throughput graph](../../target/gc-implementation/java-native-metadata-root/graphs/native-throughput.svg) ·
[Memory graph](../../target/gc-implementation/java-native-metadata-root/graphs/native-memory.svg)

## Refreshed final barrier comparison

The final edge-certification binaries are frozen in `barrier-final/`. The native
screen has 120 successful processes: five pairs, two workloads, saturation and
5,000 offered operations/s, with two-second measurements and one-second warmup.
All runtimes use two CPUs, a two-core quota, 64 MiB managed limits and 512 MiB
process caps. Source and binary integrity pass. Candidate interpreted saturation
throughput is 172,043/s for allocation and 9,963/s for roots, versus G1
25,557,413/s and 15,456,268/s and ZGC 30,503,809/s and 14,868,838/s. The JIT-enabled
candidate reaches 156,853/s and 9,627/s. Both Ristretto modes fail all twelve short
performance gates; sample-count failures also prevent extreme-tail qualification.
JIT enablement does not mean unsupported allocation methods are compiled. These
results measure runtime execution as well as GC.

The separate 40-process WASI screen also passes execution and integrity checks.
Candidate saturation is 81,034/s allocation and 6,297/s roots, below baseline
87,457/s and 8,683/s. Both policies censor some fixed-rate root trials. Nine browser
trials complete all requests: candidate request-turn p99.9 is 20–25 µs, baseline
20 µs, and arrival p99.9 is roughly 4.8–4.9 ms. The final native 60-second screen
completes 600,000/600,000 requests with zero final backlog and empty managed, root,
cache and retired state; process peak is 34.2 MiB. Hour/day qualification remains
outstanding, as do stalled-publication/callback progress and the external gates.

[Native report](../../target/gc-implementation/java-native-barrier/REPORT.md) ·
[Throughput graph](../../target/gc-implementation/java-native-barrier/graphs/native-throughput.svg) ·
[Latency graph](../../target/gc-implementation/java-native-barrier/graphs/native-latency.svg) ·
[Native gates](../../target/gc-implementation/java-native-barrier/gates.json) ·
[JIT gates](../../target/gc-implementation/java-native-barrier/gates-jit.json) ·
[WASI report](../../target/gc-implementation/java-wasi-barrier/REPORT.md) ·
[WASI graph](../../target/gc-implementation/java-wasi-barrier/graphs/wasm-throughput.svg) ·
[Browser report](../../target/gc-implementation/browser-barrier-final/REPORT.md) ·
[Browser graph](../../target/gc-implementation/browser-barrier-final/browser.svg) ·
[Short load evidence](../../target/gc-implementation/endurance-barrier-final.json)

## Actual browser host-turn evidence

The follow-up [request-turn report](../../target/gc-implementation/browser-request-turns/REPORT.md)
and [graph](../../target/gc-implementation/browser-request-turns/browser.svg) separate
request-bearing turns from idle posted tasks. All fifteen trials complete every
request and pass teardown. Candidate request-turn p99.9 is 16–25 µs, baseline
20–21 µs, and control 6–10 µs, with about 5 µs clock increments. This percentile
includes the GC step in request-bearing turns, but excludes maintenance-only turns.
Its raw all-turn, heartbeat and scheduling channels remain available. The earlier
screen below uses the original all-turn metric and should not be compared directly.

Fifteen sandboxed Chromium trials (five each for baseline, candidate and observer
control) complete all 100,000 measured arrivals per trial at 10,000/s. Each has a
ten-second measured interval, two-second warmup, a 16 MiB managed limit, two CPU
cores/quota and a 512 MiB process-tree limit. Original retained values, weak clearing,
exactly-once finalization and empty managed/root/retired/barrier teardown all pass.
Source, Wasm, browser and asset hashes remain unchanged.

Arrival p99.9 ranges from 4.832–4.928 ms across all modes. Host-turn work p99.9 is
6 µs as a histogram bucket upper bound in every fork. Idle posted turns dominate
that histogram, so it does not establish a GC-work or request-turn latency bound;
the follow-up clock probe also observes about 5 µs positive increments. The current
driver records request-bearing turns separately and uses those in its graphs.
Maximum heartbeat gaps are 6.535–7.745 ms for
control, 6.610–8.085 ms for baseline and 7.130–9.140 ms for candidate. Process-tree
peaks are about 394–398 MiB, dominated by Chromium. Final linear memory is 1.125
MiB for control, 2.125 MiB for baseline and 2.375 MiB for candidate. Candidate grows
by two linear-memory events during each measured phase; the other modes do not.
The persistent linear-memory size is distinct from live managed bytes, which reach
zero after cooperative teardown. These observations establish a working host-turn
fixture, not an extreme-tail, memory-plateau or performance qualification.

[Browser report](../../target/gc-implementation/browser-screen/REPORT.md) ·
[Browser graph](../../target/gc-implementation/browser-screen/browser.svg) ·
[Raw browser trials](../../target/gc-implementation/browser-screen/raw.jsonl)

## Completed-edge certification and reclamation under contention

Counted edge barriers can now certify a no-op from the phase/revision clock and
exact-epoch retention. Every phase transition advances the revision; numeric
revision reuse permanently disables certification, preserving counted fallback.
This revision initially used a two-entry context cache, extended and isolated
in the working-set revision above. Its entries require
Strong retention or exemption from young tracing, never suppress dirty-owner
notifications, and are disabled before object-epoch reuse. A paused certificate
owns neither admission nor an object shard. Ordinary publication, new work and
failed checks retain admission; stalled admitted publishers and arbitrary tracing
callbacks remain limitations.

The retained protocol passes 354 GC tests, including 25 production-state Loom
models, native VM/reference tests, four Miri cases, eight actual WASI tests and
bare-Wasm smoke. Forced tests complete three full cycles while a check is paused,
complete a cached check with its object shard held, preserve dirty notifications,
reject stale epochs/revision reuse and reclaim during four continuous producers.
The final small dispatch change keeps legacy routing explicit and passes the same
354-test native suite and Clippy. Its 420-process paired check shows no clear
barrier-time change relative to the protocol binary.

All 60 processes in the larger contention screen complete: five independent pairs,
four saturated producers, 1,024 collections per process, two CPUs and a 256 MiB
process cap. Candidate full completion-time ratio is 0.192 [0.031, 0.290], and young
ratio 0.138 [0.100, 0.198]. Counted unbuffered ratios are 0.077 [0.052, 0.121] and
0.082 [0.073, 0.095]. Candidate barrier rates also rise (full 8.268 to 8.950 Mops/s,
young 10.115 to 13.835 Mops/s). These are saturation screens, not comparisons at a
matched arrival rate or application latency qualification.

The 1,260-process mechanism screen gives candidate edge barriers 37.2 → 14.6 ns
(ratio 0.393 [0.384, 0.404]) and reference overwrite 114.2 → 93.7 ns (0.817
[0.808, 0.836]). Legacy barrier cost rises about 3.8% in that screen; control varies
about 8–15%, so small differences are inconclusive. No whole-VM speedup is inferred.

Two preceding revisions are superseded. Deferred revision stamping alone still
stalled the forced contention test. Moving shard waits and caching completed work
fixed that test, but three counted-policy runs in the larger paired screen missed
the ten-second deadline. Those censored observations remain in the evidence and
have no completion-time ratio. The retained read-only certificate removes the
admission traffic for already satisfied work; its larger screen has no censoring.

[Contention report](../../target/gc-implementation/contention-barrier-certification/REPORT.md) ·
[Contention graph](../../target/gc-implementation/contention-barrier-certification/contention.svg) ·
[Mechanism report](../../target/gc-implementation/mechanisms-barrier-certification/REPORT.md) ·
[Mechanism graph](../../target/gc-implementation/mechanisms-barrier-certification/mechanisms-diagnostics-False.svg) ·
[Final dispatch check](../../target/gc-implementation/mechanisms-barrier-dispatch/REPORT.md) ·
[Superseded deadline failures](../../target/gc-implementation/contention-barrier-progress/REPORT.md)

## Root ownership without weak counters

Shared root registrations use `triomphe::Arc` 0.1.17, with its `std`
feature and default features disabled. These registrations never have weak
observers. This revision initially left metadata ownership on standard `Arc`/`Weak`;
the subsequent weak-identity change below also removes metadata's weak counter.
Collector ownership retains standard `Arc`/`Weak`.
[Triomphe documentation](https://docs.rs/triomphe/0.1.17/) describes the
single-counter ownership type. No custom pointer arithmetic or raw conversion was
introduced. The retained revision passes 341 GC tests, GC/classloader/types/VM
Clippy, native VM/reference tests, two Miri lifetime cases, thirteen actual WASI
publication/root-slot tests and bare-Wasm smoke.

All 1,260 paired mechanism processes pass with frozen binaries unchanged. Candidate
root registration changes from 84.9 to 78.0 ns (paired ratio 0.919 [0.913, 0.924]),
identity handles 110.3 to 104.1 ns (0.950 [0.931, 0.952]), weak acquisition 151.2 to
145.6 ns (0.952 [0.949, 0.969]), and allocation 295.0 to 285.1 ns (0.971
[0.964, 0.974]). Owning clones remain approximately 10.5 ns; their interval includes
one. Control varies about 4%; these are exploratory mechanism results, not evidence
of Java throughput superiority. Twenty allocation-count processes show eight fewer
requested bytes per operation (candidate 444.338 to 436.338), unchanged allocation
counts and no process-peak reduction in this fixture.

[Root-counter timing](../../target/gc-implementation/mechanisms-strong-root/REPORT.md) ·
[Root-counter graph](../../target/gc-implementation/mechanisms-strong-root/mechanisms-diagnostics-False.svg) ·
[Allocation counts](../../target/gc-implementation/allocations-strong-root/REPORT.md)

## Reference-free managed writes

Scalar managed cells and Java primitive-array variants now skip owner notification
after their normal unroot hook. A conservative per-value query lets a variant
become reference-bearing later and resume its barrier. New tests cover exactly
that old-to-young transition, completion while the legacy publication writer is
held, and relocation recopying after primitive-array writes. The retained revision
passes 341 GC tests, GC/classloader Clippy, native VM/reference tests, Miri,
six actual WASI publication tests and bare-Wasm smoke.

The new `leaf_write` mechanism writes a managed scalar while marking is active.
The exact extended driver is overlaid onto the frozen preceding shared-type source
for the before binary. All 1,260 processes pass. Candidate operation cost falls
from 66.1 to 14.1 ns (paired ratio 0.216 [0.213, 0.217]); wall time including
between-batch collection falls to 0.251 [0.248, 0.254]. Reference-bearing overwrite
has ratio 0.988 [0.988, 1.034], so no change is established there. Control varies
about 7%; the large leaf-write result is retained, without claiming small changes
or transferring the microbenchmark result to whole-VM performance.

[Leaf-write report](../../target/gc-implementation/mechanisms-leaf-barriers/REPORT.md) ·
[Leaf-write graph](../../target/gc-implementation/mechanisms-leaf-barriers/mechanisms-diagnostics-False.svg)

## Shared type and finalizer information

Metadata now shares immutable type identity and its optional finalizer pointer.
An atomic claim preserves one invocation, including after a caught panic; the
callback runs outside collector coordination. Plain and finalizable allocations
of the same Rust type remain distinct. The implementation passes 339 GC tests,
Clippy, two Miri cases, native VM/reference tests, actual WASI and bare-Wasm checks.

Twenty allocation-count processes compare the previous compact metadata binary
with the shared-type binary. The intervening admission changes preserve allocation
counts/layout. Candidate requested bytes fall from 460.331 to 444.338 per operation,
with 2.350 allocation calls unchanged. Candidate process peak falls from 37.543
to 36.039 MiB (ratio 0.960, paired 95% interval 0.958–0.961). Baseline peak falls
from 35.789 to 34.293 MiB. The fixture has no finalizers; it measures the per-object
metadata reduction, not finalizer-heavy application performance.

The direct timing comparison uses the preceding `prepared-root` revision. All
1,120 processes pass with frozen inputs unchanged. Candidate root time ratio is
1.001 [0.995, 1.008], identity conversion 1.011 [0.974, 1.030], and allocation
0.996 [0.981, 1.016]. Control timing changes about 3%; no operation-time win is
claimed. The change is retained for its measured memory reduction.

[Memory report](../../target/gc-implementation/allocations-shared-type/REPORT.md) ·
[Timing report](../../target/gc-implementation/mechanisms-shared-type/REPORT.md) ·
[Timing graph](../../target/gc-implementation/mechanisms-shared-type/mechanisms-diagnostics-False.svg)

## Rejected deferred-root batching experiment

An opt-in bounded queue kept pending root registrations enumerable, stored claimed
batches durably for cancellation/retry, and allowed unrelated cycles to finish
while a drain was stalled. Five new regressions covered graph retention, cancelled
partial removal, slot reuse, overflow, queue contention and four-worker contention.
The 342-test GC suite and Clippy passed; two Miri cases, four actual WASI tests and
the bare-Wasm smoke passed for the initial drain design. The subsequent shard-batch
variant passed the native suite and Clippy, but was not separately qualified by Miri/Wasm.

Both variants were rejected and removed. Each comparison used the same frozen
binary with batching disabled versus a 32-entry limit, seven pairs, eight
mechanisms and 1,120 successful processes. Candidate roots slowed by 10.8%
(95% paired interval 10.3–11.0%) initially, and 8.4% (7.0–9.8%) with grouped shard
removal. Candidate allocation slowed by 3.3% and 15.0%, respectively. Including
collection in wall time did not recover the regressions. The retained implementation
has no deferred-root buffer option. These results do not establish that all possible
batching designs are slower.

[Initial experiment](../../target/gc-implementation/mechanisms-root-retirement-batch/REPORT.md) ·
[Grouped shard experiment](../../target/gc-implementation/mechanisms-root-retirement-sharded/REPORT.md) ·
[Grouped shard graph](../../target/gc-implementation/mechanisms-root-retirement-sharded/mechanisms-diagnostics-False.svg)

Evidence under `target/` is ignored by Git; preserve the source archives, binaries and result directories before cleaning build artifacts. The CI workflow uploads its corresponding frozen binaries and raw run directories.

## Discoverable catalog and phase-operation experiments

The isolated [catalog packages](https://github.com/theseus-rs/ristretto/blob/0497123e05056546bc526ecc49c081f934f38d64/ristretto_gc/validation/catalog/README.md)
compare immutable-tree snapshots with a fixed slot registry and prepared descriptor
operations. Both domain counts, durable work and the cell effect participate in
one helpable controller transition. Stopped/late helpers, cancellation, slot reuse,
exhaustion and concurrent count integrity have native and model evidence. The
packages are not called by the production collector.

Ten frozen check commands pass, including 11 native descriptor fixtures, eight
models, actual WASI and both Miri modes. Three deliberately broken variants fail.
The 60-process allocation-count screen uses five resident sizes, three repeats and
both targets. The descriptor design holds at six allocation requests for a new
record/insertion and 18 for a full lifecycle; tree insertion rises from three to
18 requests, and its lifecycle from ten to 70, across 1–16,384 residents. The
descriptor costs more at the smallest point. Repeated counts match exactly.

The [figure](../../target/gc-implementation/catalog-candidate-allocations.svg) and
[frozen report](../../target/gc-implementation/catalog-candidate-REPORT.md) retain
bytes, raw counts and failed controls. There are no throughput or latency claims.
Registry setup, real payload/root/metadata ownership, budget enforcement and cold
ArcSwap thread bookkeeping are excluded. The tree's final-snapshot destruction and
ArcSwap's internal reader scans prevent a bounded-clockless-work claim. Real
ownership and complete admission/phase migration remain next steps; no default,
pause, peer, capacity or endurance gate is closed.

Source freezing now includes nested `Cargo.lock` files so these isolated packages'
dependency pins are archived. A provenance regression and the full 78-test Python
harness pass; generated output directories remain excluded from their own archive.

## Preallocated catalog state experiment

The [pooled variant](https://github.com/theseus-rs/ristretto/blob/0497123e05056546bc526ecc49c081f934f38d64/ristretto_gc/validation/catalog/catalog-pooled-prototype/README.md)
replaces per-update control/membership allocation with fixed backing. Owning
expected references prevent premature reuse; final reference release acquires prior
owners and delays reuse until destruction completes. Preparation failure and
cancellation return private slots. The pool and protocol have separate exact-state
models and native/Miri/Tree Borrows/sanitizer checks, with failed mutation evidence.
The production collector does not use these packages.

All 90 isolated count processes pass. Tree/unpooled results reproduce the previous
60 processes exactly. Pooled record/insertion uses one request instead of six; a
whole lifecycle uses one instead of 18 (56 native / 32 WASI requested bytes). At
16,384 residents the pooled representation retains 2,644,200 native / 1,917,044
WASI requested bytes after setup, versus 1,704,168 / 1,114,244 unpooled. The
[figure](../../target/gc-implementation/catalog-pooled-allocations.svg) includes both
allocation traffic and the increased backing. These are requested-byte counters,
not RSS, throughput or latency results.

The [evidence report](../../target/gc-implementation/catalog-pooled-REPORT.md)
contains the frozen sources, 13 native/ten WASI catalog fixtures, eight protocol
models, separate pool checks, sanitizers and corrected-checker results. The first
shared-target reproduction runner was rejected by an unexpectedly passing negative
control; the final runner isolates build directories and preserves all outcomes.
Actual metadata/initial roots, byte charging, participant limits and progress
reserves remain required. No runtime/default or qualification gate changes.


### Descriptor-owned allocation charges (2026-11-14)

The runtime now transfers reserved logical bytes to metadata before visibility and
refunds them after actual payload destruction. The old post-publication unwind
failure and all 24 passing validation commands are preserved in the
[evidence report](../../target/gc-implementation/allocation-credit-REPORT.md).

The separate 1,008-process mechanism screen retains a native collection-inclusive
allocation cost increase of 0.6–2.4%, mixed WASI/control results and all intervals.
Forty-eight count processes retain small variable temporary-request increases while
verifying their release and unchanged retained requested bytes. Exported PNG/SVG
figures, raw processes and complete before/after provenance are linked in the
report. This is correctness and cost attribution, not G1/ZGC, pause, capacity or
endurance qualification. Canonical publication and initial-root migration remain
open; experimental defaults are unchanged.


### Guarded refunds and borrowed reclamation (2026-11-14)

Two scoped cleanup guards repair payload/finalizer diagnostic-unwind charge leaks;
normal reclamation reuses a validated collector borrow for refund. The first
candidate passed its existing checks but failed an added finalizer case and was
rejected before timing. The [final evidence](../../target/gc-implementation/refund-guard-REPORT.md)
contains 22 passing check commands, nine focused ownership fixtures, both original
failures, 1,008 timing processes and 48 separate counter processes.

Against the preceding descriptor-owned-charge revision, native collection-inclusive
allocation time decreases 0.2–1.4% and managed-cell allocation decreases 0.8–1.0%.
WASI is mixed: managed-cell operations improve 1.6–2.0%, while leaf writes regress
9–16%. The unresolved control regression and all intervals remain in the report.
The counter differences are temporary and retained requested bytes match; these are
not RSS measurements. No whole-JVM, pause, comparison, capacity or endurance gate
is qualified, and experimental defaults remain unchanged.

### WASI leaf-write layout diagnosis (2026-11-14)

The [follow-up report and graph](../../target/gc-implementation/wasi-leaf-refund-diagnosis/REPORT.md)
retain 336 additional exact-binary measurements. Longer default-engine runs
confirm the leaf-write slowdown. A separate diagnostic change to Wasmtime's
function alignment removes the clear regression in this screen. The shared
benchmark body differs only in integer constants, while its generated code moves.
This supports layout sensitivity; it does not identify the hardware cause or fix
the default-engine result. All controls, diagnostics-on regressions, schedules and
raw measurements remain available. No runtime or benchmark defaults change.

### Construction root batches (2026-11-14)

The new `construction_batches` fixture performs twelve managed allocations through
nested attachment scopes per operation. Both frozen revisions use identical fixture
source. Moving registration batches saves about eight allocator requests per
operation, with unchanged retained requested bytes. The 1,176-process paired screen
reduces its operation time by 8.9–16.3% native and 10.1–13.0% WASI; several WASI write
controls regress and remain visible. See the [report and graphs](../../target/gc-implementation/root-batch-REPORT.md).

The 23-command matrix validates this scoped transfer/removal change. A separate
audit still fails initial-parent-root cancellation on both revisions; its preliminary
repair is isolated from these measurements. No runtime default or qualification gate
changes, and these allocator counts and loop timings are not RSS, scheduled latency
or observed global pauses.

### Ordered initial-root cleanup (2026-11-14)

The initial-parent cancellation follow-up is now integrated. Constructors finish
metadata hooks before installing that root and retain a metadata owner across
publication, so cancellation removes the registration before private storage drops.
All 26 validation commands pass; the [report](../../target/gc-implementation/initial-root-REPORT.md)
preserves the original leak, the first repair's cleanup-order failure, final sources
and exact binaries.

All 72 counter processes retain matching net/retained requested bytes. The separate
1,176-process paired screen lowers allocation-operation medians by 1.1–3.2% native
and 2.6–6.2% WASI, with the native candidate interval including no change. Native
wall times are mixed. WASI candidate nested construction has 4.3% higher wall time
despite 4.4% lower operation time; WASI baseline overwrites regress 2.3%. Other
control regressions, all intervals and diagnostics-on results remain visible in
the exported PNG/SVG figures and raw data. The baseline binaries are reused exactly
from the preceding batch revision, with identical benchmark/harness source.

This repairs local ownership ordering, not shared publication or stalled-callback
progress. No whole-JVM, pause, G1/ZGC, capacity or endurance gate is qualified.

### Tagged numeric catalog (2026-11-14)

The isolated tagged catalog now propagates fallible ownership acquisition through
all links, including optional discovery. Pending preserves an installed request's
completion result. The final matrix passes native/WASI, Miri/Tree Borrows and
sanitizers; 12 protocol models, five pool models and nine failing fault variants
provide scoped evidence. The report retains the initial lost-result regression,
fence/TSan limitation, timed-out combined model and ineffective first fault schedule.

The [new cost fixture](https://github.com/theseus-rs/ristretto/blob/0497123e05056546bc526ecc49c081f934f38d64/ristretto_gc/validation/catalog/catalog-tagged-cost-prototype/README.md)
separates counting and timing binaries. All 48 counter and 112 timing processes
complete. It removes one allocator request per numeric lifecycle and lowers median
lifecycle time 51–53% native / 24–25% WASI, while increasing requested setup backing
about 40% / 79% at 16,384 residents. Bounded-search retries are included. The
[report and plots](../../target/gc-implementation/catalog-tagged-REPORT.md) expose
both sides of the tradeoff. Participant bounds, charged progress reserves, real
graph/phase integration and whole-collector qualification remain open.

### Resumable catalog preparation (2026-11-14)

Private preparation now retains successful slots across bounded polls. A frozen
failure shows the old retry path making no progress with six reusable free slots
for a three-slot proposal. Cancellation, stale phases, failed final allocation
and cross-thread handoff have native/WASI, Miri/Tree Borrows, sanitizer and model
coverage. The current checker completes 14 protocol models, five pool models and
11 expected fault failures.

The [before/after report](../../target/gc-implementation/catalog-resume-REPORT.md)
retains unchanged allocator/backing counts and a 9–10% native / 15–16% WASI
lifecycle-time regression. Its 224 paired timing processes include the unchanged
pooled control. These are progress/cost results for an isolated numeric catalog;
participant quotas, charged reserves and production integration remain open.

### Bounded numeric participants (2026-11-14)

The standalone tagged catalog now offers non-cloneable participants, separate helper
registrations, derived pool capacities, copied identity keys and resumable private
cell creation. A participant's request storage drops before registration is returned.
The matrix passes native/WASI, both Miri modes, sanitizers and bare-Wasm; 16 catalog
models, five pool models and 16 executed fault controls pass their expected outcomes.
Failed multi-helper fixture schedules remain in the evidence.

The matched-capacity screen runs 48 counter processes and 112 timing processes.
Both raw and bounded modes issue zero loop allocations; bounded adds one setup
request (160 bytes native, 96 WASI). Its complete lifecycle is 37–38% slower native
and 98–102% slower WASI. These policy/API costs remain visible; no performance gate
is completed. Individual insertion stage boundaries differ and the bounded mode's
legacy retry column does not count polls. Raw artifacts and graphs are in the
[participant report](../../target/gc-implementation/catalog-quota-REPORT.md).
Byte charges, exhaustion policy, production integration and qualification remain open.

### Retained participant polls (2026-11-14)

Pending numeric facade state now stays in place; successful preparation reuses its
owning membership observation. The eight-command native/WASI/memory-safety matrix
passes, as do seven focused fault controls. Unchanged core/pool/model evidence is
reused with an explicit source comparison; all 86 Python harness tests pass.
48 new counter processes exactly match prior counts. 224 new timing processes
lower facade lifecycle cost by 19–20% native and about 26% WASI, retaining control
variation and higher absolute cost than the raw API. These combined changes are
not individually attributed. See the [report and figures](../../target/gc-implementation/catalog-poll-REPORT.md).
Byte budgets, production integration and qualification remain required.

### Charged catalog backing (2026-11-14)

The numeric facade now supports an external logical budget for calculated backing
and reserved inline participant storage. Its opaque owner frees the outer Arc before
backing destruction and credit refund. The 14-command matrix and separate actual
allocator/order audits pass on native/WASI, Miri/Tree and sanitizers. All 90 harness
tests pass; 16 catalog/five pool models and 18 existing faults re-execute, and two
new accounting faults fail on both targets. The frozen Arc layout is an audited
toolchain assumption, not a stable ABI or RSS contract.

48 counter processes preserve loop counts; private-budget convenience adds one setup
allocation (48 native / 24 WASI bytes). 224 paired timing processes retain a small
native regression and opposite WASI changes: facade time -9–10%, raw control about
+5%, without attributing a cause. See the [report and graphs](../../target/gc-implementation/catalog-charge-REPORT.md).
Finite exhaustion and real heap/graph/phase integration remain required before GC
qualification. Run `catalog_accounting_check.py` for the separate positive/fault audit.


### Finite catalog generations (2026-11-14)

The numeric facade rejects preparation after permanent selected-slot generation
exhaustion, drops private state and preserves installed completion. A reduced-limit
red driver and three new broken variants exercise the failure boundary. The
22-command native/Wasm/memory-tool matrix, six pool/16 catalog models, all 23 distinct
faults and 90 harness tests pass. The model runner accepts explicit catalog/pool
source paths for isolated candidates without changing production dependencies.

48 new counters retain zero loop allocations and unchanged retry counts. Facade
setup adds 56 bytes native / 44 WASI; raw setup adds 48/36. 224 paired timing
processes retain a native facade +2.87% regression at 16,384 records, wider intervals
in other native cases and mixed WASI changes. See the [report and figures](../../target/gc-implementation/catalog-exhaust-REPORT.md).
The raw API executes changed pool code and is not an unchanged control. This does
not establish indefinite service, contention fairness or GC qualification; real
heap/graph/reference integration and sustained-load gates remain required.


### Generation budgets encoded at setup (2026-11-14)

The pool initializes generations from the configured allocation budget and uses
the constant maximum in claims. This removes a metadata field/read while retaining
terminal failure and installed completion. The final 22-check matrix and 90 harness
tests pass. Six pool models and nine distinct pool/facade/accounting fault variants
execute; 16 unchanged abstract catalog models are reused with explicit source scope.
A test-only Clippy correction and its runtime-equivalence comparison are retained.

48 counters preserve loop allocation/retry counts; backing shrinks by 24 bytes on
native and WASI. 224 paired processes show 3–4% native lifecycle improvements and
small retained WASI regressions. See the [report/figures](../../target/gc-implementation/catalog-limit-REPORT.md).
The comparison is against the completed finite-exhaustion candidate, not G1/ZGC
or the production collector. Full graph/reference integration and qualification
remain open.


### Canonical retention and work (2026-11-14)

The isolated catalog now commits epoch, retention and traversal obligations in one
immutable membership effect. Six floors follow Strong/Final/Phantom/retirement
boundaries; stronger discovery supersedes work, and exact acknowledgement cannot
clear a newer ticket. 60 native / 53 WASI fixtures, both Miri modes, sanitizers,
20 catalog / six pool models, 27 distinct fault variants and 90 harness tests pass.
Separate tracing lifecycle audits confirm unchanged system allocation/free counts.

The 224-process paired screen retains regressions against the setup-budget revision:
native facade +3.73–6.07%, WASI +3.29–4.49% (one-record interval crosses zero), with
raw API increases also reported. Membership backing adds 24 bytes per slot in these
builds. All 48 counter processes retain zero loop allocations. These costs use the
unchanged numeric fixture; they do not measure payload tracing or GC throughput.
See the [report and graphs](../../target/gc-implementation/catalog-trace-REPORT.md).
Real metadata/initial roots, graph ownership, birth/generation exemptions, complete
reference processing, production admission and qualification remain open.


### Sealed production publication ownership (2026-11-14)

Every constructor now transfers prepared metadata, its optional initial registration
and completed construction roots into one private record. Scoped helpers can publish
or cancel that record without origin TLS; conditional binding uses its exact batch.
Cancellation preserves child graphs through payload destruction and removes the
parent registration first. The shared heap borrow still excludes collector destruction.

All 30 correctness/tool checks pass, including native/WASI, both Miri modes,
sanitizers and targeted VM coverage. Four negative variants fail on both targets.
The unchanged mechanism fixtures compare exact prior binaries with this candidate:
72 counter and 1,176 randomized timing processes pass. See the
[full costs and figures](../../target/gc-implementation/publication-owner-REPORT.md).
Allocation time rises 3.14–4.18% native / 5.75–7.73% WASI, and nested construction
uses more allocator requests while requesting fewer bytes. These regressions are
retained; net and post-collection requested bytes match in all counter groups.
This stage adds no new runtime atomic protocol or metadata layout field. Its record
is consumed once under existing admission; shared helpable publication, collector-owned
record shutdown, canonical graph/reference state and qualification remain open.


### Rejected inline construction capture (2026-11-14)

A new `leaf_allocation` mechanism explicitly declares no managed references while
leaving the original allocation fixture unchanged. Both revisions rebuild the
expanded fixture. The candidate combines reference-free TLS bypass with four inline
captured registrations. All 31 validation commands, 72 counter processes and 1,344
paired timing processes pass; five executed fault variants fail on both targets.

Reject the inline representation: allocation operation time rises 6.5–8.7% native /
7.0–9.5% WASI, and nested construction also slows. About four fewer allocator requests
per nested operation come with larger type layouts, not demonstrated lower RSS.
See the [complete report, intervals and exported graphs](../../target/gc-implementation/capture-inline-REPORT.md).
The follow-up restores vector storage and measures TLS bypass separately. No
superiority, latency, contention-progress or endurance gate is completed.


### Optional reference-free construction scopes (2026-11-14)

Vector storage is restored. Constructors use an optional scope to bypass TLS for
explicitly reference-free types while preserving enclosing construction and original
type layouts. The 25-command matrix and five native/WASI fault pairs pass.

72 counter and 1,344 fresh timing processes compare exact pre-inline baseline binaries
with this candidate, using the same expanded leaf fixture. Explicit-leaf operation
time falls 1.5–3.0% native (one interval includes zero) and 1.6–5.6% WASI (all below
zero). Retain the WASI baseline allocation +3.6% and leaf-write +5.8–7.4% control
regressions alongside these gains. See [all intervals, counts and graphs](../../target/gc-implementation/leaf-scope-REPORT.md).
No allocator-request reduction is established; net and post-collection retained
requested bytes match in all groups. This is an integration candidate, not a pause,
peer-superiority, sustained-load or complete progress qualification.

## Shared private publication ownership (2026-11-14)

Prepared resources can move into a shared header whose admitted handles borrow
the matching live collector. The final handle closes admission permanently and
runs ordered private cleanup; retained raw headers preserve neither payload nor
collector lifetime. The private slot lock is released before callbacks. A new
forced-stall fixture confirms logical payload credit remains held while private
destruction is stopped, while unrelated garbage is reclaimed.

All 13 commands pass: native GC 538/562 (one existing ignore), actual WASI 268/279,
eight ASan/TSan shared fixtures, Clippy, bare-Wasm and formatting. Both Miri modes
cover seven byte-identical frozen development fixtures plus the newly appended
stalled-destructor fixture. Four production-state models pass with the frozen
corrected checker; seven distinct mutations yield 11 runtime and three model
negative failures. The initial model witness mismatch and all failures are retained.

Ordinary constructors still use the exclusive stack Publication. A retained handle
prevents exclusive restoration, so stopped-publisher progress and canonical commit
remain open. No performance claim follows; backing/participant accounting, real
graph/reference integration and all qualification gates remain open.

[Report](../../target/gc-implementation/shared-publication-REPORT.md) ·
[Verification](../../target/gc-implementation/shared-publication-verification.json).

## Borrowed shared descriptor views (2026-11-14)

Admitted shared-publication handles can borrow their exact prepared descriptor
without locking the private result slot. The view lifetime retains the handle's
metadata and construction-graph protection; the raw header gains no payload owner.
One documented pointer dereference is covered by all eleven shared fixtures under
both Miri modes, ASan and TSan. Ordinary constructor paths remain unchanged.

All 13 commands pass: native GC 541/565 (one existing ignore), actual WASI 270/281,
Clippy, bare-Wasm, format and diff. A forced native result-slot stall permits other
helpers' descriptor preparation and unrelated reclamation. A deliberate result-lock
fault is caught on native/WASI, and actual-source compile-fail fixtures reject both
escaping the handle and dropping it while the view is still used. Four unchanged
admission models have exact-hash scoped reuse; they do not model the new pointer.

This supplies a borrowed preparation view, not canonical publication. Announced
work still needs graph-safe cancellation and phase-coupled membership/traversal.
No performance or qualification gate closes; defaults remain unchanged.

[Report](../../target/gc-implementation/shared-view-REPORT.md) ·
[Verification](../../target/gc-implementation/shared-view-verification.json).

## Effect ownership precedes the commit decision (2026-11-14)

The isolated catalog prepares its fallible effect owner before deciding COMMIT.
Failed preparation remains cancellable; applying the exact-version effect and
recording its certificate then require no further fallible acquisition. A failed
strong effect CAS proves that effect already occurred because canonical controllers
are the only writers and old membership versions are never reinstalled.

A frozen red fixture catches the old irreversible decision before failed preparation.
Eleven prototype checks pass (61 native, 54 WASI, 58 in each Miri mode and 61 in each
sanitizer), including backing accounting. The 22 corrected-checker models include a
forced late-CAS schedule; three variants produce seven expected negative processes.
Integrated Clippy, the added stock-Loom model, format and diff checks also pass.

The catalog still contains only numeric records. Actual graph guards, canonical
payload/phase/reference integration, admitted callers and qualification remain open.
Ordinary runtime paths and defaults are unchanged; no performance claim follows.

[Report](../../target/gc-implementation/commit-prefix-REPORT.md) ·
[Verification](../../target/gc-implementation/commit-prefix-verification.json).

## Heterogeneous prepared publication headers (2026-11-14)

Erased headers share the existing typed allocation through an Arc trait object.
Their helpers validate the same live heap, use the same admitted count, borrow
descriptor views and dispatch the original typed final cleanup. No strong collector
owner, second header allocation or payload cast is added.

The 13-command matrix passes: native GC 544/568 (one existing ignore), actual WASI
272/283, fourteen shared fixtures in both Miri modes and both sanitizers, Clippy,
bare-Wasm, format and diff. Three faults produce six native/WASI runtime failures;
two compiler checks enforce erased-view borrowing. Four unchanged admission models
have exact-hash scoped reuse. Mixed-type, shared-backing and cross-thread fixtures
exercise the new erased interface.

Ordinary constructors are unchanged. Announced storage/graph lifetime, canonical
commit, phase/reference integration and full qualification remain open. No
performance or default-selection claim follows.

[Report](../../target/gc-implementation/erased-publication-REPORT.md) ·
[Verification](../../target/gc-implementation/erased-publication-verification.json).

## Announced private descriptor storage (2026-11-14)

Descriptor storage may now outlive cancellation without keeping a private payload
alive. A matching admitted helper is required for every storage view. Announcement
shares the handle-count word and permanently prevents legacy restoration. Final
private cleanup destroys the payload before releasing child roots and conditional
reservations. The interface creates no descriptor storage leases; its unsafe
cancellation contract explicitly excludes outstanding leases.

All thirteen matrix commands pass: native GC 555/579 (one existing ignore), actual
WASI 277/288, all twenty shared fixtures in both Miri modes and sanitizers, Clippy,
bare-Wasm and format. Nine corrected models pass. Nine distinct fault variants
produce seventeen expected runtime/model failures, plus two lifetime compiler
checks. Integration matched 1,494 compiled files before two recorded safety-comment
clarifications; integrated all-features tests and Clippy also pass.

Logical payload credit remains held through a stalled destructor while unrelated
reclamation progresses. Retained metadata/header physical backing is not yet fully
charged. Ordinary constructors do not select this private interface; canonical
publication, graph/phase/reference integration and all qualification remain open.

[Report](../../target/gc-implementation/announced-storage-REPORT.md) ·
[Verification](../../target/gc-implementation/announced-storage-verification.json).

## Canonical numeric allocation domains (2026-11-14)

The isolated tagged catalog now couples domain activity/scope, generation, births,
retention and exact trace tickets with pending counts. Explicit checked start/finish
replace implicit cycle restart. Promotion preserves young work while publishing the
old handoff. Ordinary discovery cannot revive another domain's logically dead object.

All eleven matrix commands pass: native 74, actual WASI 67, each Miri mode 71,
each sanitizer 74, Clippy, bare-Wasm, allocation/refund audits and format. Twenty-seven
corrected models pass; nine mutants produce four model and eighteen runtime failures.
The original expensive late-helper models remain incomplete evidence; a forced
pre-CAS schedule replaces that one case, with exact unchanged-library evidence.

Ninety-six allocator-count processes retain zero lifecycle allocations and complete
teardown refunds. Requested setup backing grows 8.1–11.8% native / 8.4–12.5% WASI;
16,384 residents add 512 KiB. These are numeric catalog backing costs, not GC timings
or RSS. Actual payload/graph, root and reference integration and all qualification
remain open. Ordinary collector publication and defaults remain unchanged.

[Report](../../target/gc-implementation/catalog-domain-REPORT.md) ·
[Backing graph](../../target/gc-implementation/catalog-domain-cost-v2/backing-change.svg) ·
[Verification](../../target/gc-implementation/catalog-domain-verification.json).

## Guarded retained-record publication (2026-11-14)

The isolated catalog owns generic records. Each helper admits a context-borrowing
record guard before its own decision/effect, then publishes a cleanup-only latch
before certification. Closed/foreign records cancel undecided work; certified head
release needs no graph admission. Numeric insertion methods remain unit-record-only.
Preflight rejection returns the exact non-cloneable record for retry.

V1's eleven-command matrix passes (native80/WASI72, Miri76 and sanitizers80). V2's
record-return change passes full native81/WASI73 and all current retained-record
fixtures under both Miri modes (six) and sanitizers (seven), plus Clippy/bare/audits/
format. Thirty-one corrected models pass; six mutants produce five model and twelve
runtime failures, with three separate compiler checks. Twenty-four numeric accounting
rows are unchanged from the domain-state predecessor. No timing claim follows.

The guard fixture is synthetic. Real heap/identity/graph composition, unique payload
binding, bounded real-record participants, later announcement-failure handling,
canonical roots/references and every qualification gate remain open. Production
collector publication and defaults are unchanged.

[Report](../../target/gc-implementation/record-admission-REPORT.md) ·
[Verification](../../target/gc-implementation/record-admission-verification.json).

## Bounded opaque-record accounting (2026-11-14)

The isolated generic facade's72 record allocator rows match requested backing for
three inline layouts on native/WASI; all24 numeric rows remain unchanged. There are
nine setup requests, zero lifecycle requests and one refund after backing release.
The [backing graph](../../target/gc-implementation/bounded-record-backing.svg) is an
allocator audit, not a timing or GC comparison. See the
[report](../../target/gc-implementation/bounded-record-REPORT.md) for the full matrix,
model limits and retained failures. Concrete GC integration and full qualification
remain open; no G1/ZGC or sustained-load claim is changed.

## Canonical root accounting (2026-11-14)

The isolated root protocol now has288 before/after accounting rows, including144
root-enabled native/WASI cases. All audited lifecycles allocate no system backing
and refund credit after the last free. Root setup uses 12 requests; zero-root setup 9.
The [graph](../../target/gc-implementation/canonical-root-backing.svg) reports logical
charge increases for root/retained-target capacity, not timings or RSS. See
[the report](../../target/gc-implementation/canonical-root-REPORT.md) for validation,
scoped model limits and replay inputs. Actual collector/G1/ZGC/endurance qualification
is unchanged and remains open.


## Conditional-reference custody is now helpable in the isolated catalog

The isolated controller now includes owner-bound conditional decisions, construction
lifetimes and exactly-once notification transfer. Independent reference/construction
quotas and exact-object reserved roots preserve helper progress with stopped mutators
and occupied caller-root permits. All 18 matrix commands and four main-tree checks
pass, with 48 scoped models, 36 runtime/two model faults and nine compiler rejections.
The 528 allocator rows match backing and refund ordering. The 288 reference-enabled
rows exercise allocation-free lifecycles; setup uses 17 system requests.

The [cost graph and report](../../target/gc-implementation/canonical-reference-REPORT.md)
show increased memory, including Q=0 inline-layout costs. For N=16,384,K=8,M=64 and
one-byte records, native charge increases from 5,282,232 bytes to 5,309,696 with Q=0,
5,373,344 with Q=1 and5,475,680 with Q=64. Corresponding WASI values are 4,755,256,
4,780,064,4,839,056 and4,931,664. These are requested backing/inline-owner counts,
not RSS or timing. No GC throughput, pause or peer advantage is inferred. Actual
runtime integration, JIT and complete performance/endurance qualification remain open.
