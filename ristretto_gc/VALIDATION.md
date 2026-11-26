# Validation

Evidence availability notice (2026-11-15 22:23 UTC): the workspace `target`
directory disappeared, including historical GC raw results, graphs and archives.
The source changes remain. Results recorded below are historical observations;
links under target are unavailable and cannot serve as current qualification evidence.
See [the recovery record](../benchmarks/gc/EVIDENCE_RECOVERY.md). Fresh validation is being rebuilt outside
Cargo build storage; no full qualification gate is closed by the earlier summaries.


## Experimental canonical caller API (2026-11-15)

The 2,226-file candidate passes six regression commands: 686 native tests (one
existing ignore), 379 WASI tests, native/WASI Clippy, formatting and bare-Wasm checks.
One layout fixture on each target verifies all transparent owning/view wrappers.
Six native/five WASI external-client fixtures exercise cycles, independent roots,
full-quota null progress, cancellation/reuse, admission/type/foreign failures,
retained-owner credit and native concurrent aliases. All six pass strict Miri,
Tree Borrows, ASan and TSan in four fresh commands without timeout. This is separate
from the previously verified 88 internal fixtures; no expanded fresh census is claimed.

One positive compiler control accepts non-Clone aliases and surviving participants;
four controls reject invalid view/owner lifetimes and overlapping mutator requests.
Native and bare-Wasm smoke processes execute cyclic reachability and full teardown.
The initial public-Debug/re-export lint failure is preserved. Raw catalog/pool models
are reused through unchanged inputs. No default, performance or qualification gate
changes. See [the API contract](CANONICAL_API.md) and
[the report](../target/gc-implementation/canonical-client-REPORT.md).

## Shared-root final-owner correction (2026-11-15)

Ordinary Arc Drop in shared-root v1 refunded catalog credit before freeing the
populated outer owner when the last alias outlived runtime/participants. The new
`Arc::into_inner` teardown frees backing first, with no external Weak observers.
The original native/WASI defect and evidence remain preserved and superseded.

The 2,223-file v2 candidate passes 679 native tests (one existing ignore), 373 WASI
tests, native/WASI Clippy, bare-Wasm checks, formatting and 39 Python tests. All
88 payload fixtures pass each of strict Miri, Tree Borrows, ASan and TSan. Four
additional final-owner audit commands make 17 passing memory commands; their paired
leaf/graph cases are not added to the 88-test census. Native/WASI final-owner audits
also pass. Eight intended runtime/audit negative processes cover leaks, missing
pending backing and restoring ordinary Arc Drop. One positive/two negative compiler
controls pass. Raw catalog/pool evidence is reused only through unchanged inputs.

All 90 finite stress processes reclaim 6,927,000 real objects. All 308 timing
processes and 66 allocator samples validate. Added root allocations and native
lifecycle regressions at capacities 8/64/256 remain explicit. The v2 Clippy failure
and a stale-binary provenance preflight rejection (zero audit executions) are retained.
See [the corrected report](../target/gc-implementation/canonical-shared-root-v2-REPORT.md)
and [contract](CANONICAL_SHARED_ROOTS.md). Defaults and full qualification gates
are unchanged; the older shared-root entry below is historical.

## Shared typed-root ownership (2026-11-15)

The frozen candidate passes 679 native GC tests (one existing ignore), 373 WASI
GC tests, native/WASI Clippy, formatting, bare-Wasm library/examples and 30 Python
tests. Strict Miri, Tree Borrows, ASan and TSan each pass the exact same 88 payload
fixtures in 13 commands, with no timeout. Tree access uses disjoint 22/21 partitions
of all 43 tests. One positive and two negative compiler controls establish alias
support and borrowed-view lifetimes. Six intended native/WASI runtime/audit negatives
cover last-owner leaks and missing pending-wrapper charges. The earlier non-Clone
root rejection is superseded. Raw catalog/pool models and memory checks are reused
against 83 unchanged Rust/Cargo inputs and the unchanged model checker.

All 90 finite contention processes, six allocator correctness probes, 84 clone
and 224 lifecycle timing processes, and 18 clone / 48 lifecycle allocator samples
validate. Root creation adds allocation requests, and native lifecycle capacities
64/256 regress about 1.3%/1.2%; both remain visible. No default or full qualification
gate changes. See [the report](../target/gc-implementation/canonical-shared-root-REPORT.md)
and [the contract](CANONICAL_SHARED_ROOTS.md).

## One-slot phase decisions with bounded yields (2026-11-15)

The revised candidate passes 675 native GC tests (one existing ignore), 370 WASI GC
tests, 253 native / 233 WASI catalog tests, native/WASI Clippy, formatting, bare-Wasm
library/examples and 21 Python tests. Strict Miri and Tree Borrows each pass 248
catalog tests; ASan and TSan each pass 253. Each memory tool passes the same 84
payload tests in disjoint groups. All 73 corrected catalog and six pool models,
60 native fault controls and eight WASI fault controls pass. The raw models and
facade stopped/cancelled-request fixtures have separate scopes; broad historical
Finish/restart and unrestricted recovery exploration remain incomplete.

All 90 finite contention processes, 2,112 accounting rows, 224 paired timing
processes and 48 allocator samples validate. Accounting rows are unchanged;
every performance regression is retained. Finite native requests still reach
833,993 polls. The unpaced candidate's regressions and all development/capture
failures remain separate from accepted evidence. VM payloads/callers, callbacks, compaction, JIT and all full qualification gates remain open; defaults are unchanged.
See [the report](../target/gc-implementation/canonical-phase-REPORT.md) and
[the protocol](CANONICAL_PHASE_DECISIONS.md).

## Bounded inventory cleanup and helped publication (2026-11-15)

The final candidate passes 675 native GC tests (one existing ignore), 370 WASI GC
tests, 242 native / 222 WASI catalog tests, native/WASI Clippy, formatting, bare-Wasm
lib/examples and 21 Python tests. Strict Miri, Tree Borrows, ASan and TSan each pass
84 leaf/graph and 27 catalog fixtures. All 17 intended fault processes pass (nine
native, eight WASI); 52 mutation anchors validate. Seventy catalog and six pool model
results are reused against exact compiled inputs. The bounded allocation retry is
outside those transition models and has its own deterministic native stopped-helper
fixture. Unrestricted recovery exploration and the historical broad Finish/restart
model remain incomplete; these results do not establish general starvation freedom.

All 90 final finite native/WASI stress processes pass (6,927,000 real objects). All 224 paired timing processes and
48 allocator samples pass. At capacity 256, paired lifecycle time ratios are 0.673 native and 0.771 WASI (lower is better). All capacities, regressions and intervals are retained in the report. Native v2 failed 12/27 contention processes and
is rejected; its timings, the scan-only cooperative stall, model timeouts and
excluded development failures are preserved separately. No watchdog was raised.
See [the report](../target/gc-implementation/canonical-sweep-REPORT.md) and
[the invariant](CANONICAL_SWEEP_BATCH.md). Full qualification remains open.

## Source-indexed canonical traversal (2026-11-15)

The frozen candidate passes 670 native GC tests (one existing ignore), 365 WASI GC
tests, 236 native / 217 WASI catalog tests, native/WASI Clippy, formatting and bare-Wasm
lib/examples. Strict Miri, Tree Borrows, ASan and TSan each pass the affected 13 graph
and 40 edge fixtures. All 68 corrected catalog models pass; six unchanged pool models
are reused with exact input hashes. Six selected faults fail as intended on both
native and WASI, all 49 mutation anchors validate, and all 21 Python tests pass.
The initial model-stack failure, stale anchors and runner import error are preserved
and excluded from successful evidence. No permutation or preemption cap was added.

All 224 paired timing processes and 48 allocator samples pass. At capacity 256,
lifecycle time falls about 19% native and 11% WASI, but cleanup still dominates.
This is scoped correctness and cost evidence, not pause/JVM/endurance qualification.
See [the report](../target/gc-implementation/canonical-indexed-REPORT.md) and
[the immutable-placement invariant](CANONICAL_INDEXED_EDGES.md).

## Canonical graph runtime (2026-11-15)

The frozen candidate passes 669 native GC tests (one existing ignore), 364 WASI GC
tests, 226 native / 209 WASI catalog tests, native/WASI Clippy, formats and bare-Wasm
lib/example checks. Strict Miri, Tree Borrows, ASan and TSan each pass 78 GC and 30
catalog edge tests. Ten new fault conditions fail as intended on both native and
WASI; 44 invalid release-example invocations are rejected. All 21 Python tests pass.
The 66 catalog/six pool corrected models are reused with exact matching input hashes;
new graph/facade methods are outside that model scope. The first accounting fault's
compiler rejection is preserved and excluded; its corrected mutation reaches the
intended assertion. See [the receipt/report](../target/gc-implementation/canonical-graph-REPORT.md).

All 56 cost-screen processes and 24 allocator samples pass integrity and exact cleanup.
This establishes scoped real graph authority and cyclic reclamation, not whole-VM,
pause or sustained-load qualification. Sparse-catalog cleanup remains expensive;
[the contract](CANONICAL_GRAPH.md) lists callback, representation and integration limits.

## Canonical edge inventory (2026-11-15)

The final catalog suite passes 223 native / 206 WASI tests, native/WASI Clippy and
formatting. All 66 concrete GC and 27 edge fixtures pass strict-provenance Miri,
Tree Borrows, ASan and TSan. The full GC regression passes 657 native tests (one
existing ignore) and 354 WASI tests; bare-Wasm compilation passes. Corrected models
pass all 66 catalog and six pool cases. Across the original checker and corrected
tail, 42 distinct intended fault controls validate; seven portable edge faults also
validate on WASI. All 840 allocator-accounting rows have zero lifecycle requests
and exact final refunds. Failed early fixtures, the overbroad original fault mutation,
runner mistakes and a terminated grouped memory driver are preserved and explained
in [the report](../target/gc-implementation/canonical-edge-REPORT.md).

This establishes scoped edge transactions, source-owned lifetime and complete
Full-domain inventory traversal. The real graph runtime, VM callers, arbitrary
callbacks, JIT and full performance/endurance qualification remain open. Traversal
and metadata costs are explicit limitations; see [the contract](CANONICAL_EDGES.md).

## Real leaf contention (2026-11-15)

The optional diagnostic now exercises typed allocations, retained identity reads,
root eviction and physical reclamation with 1–16 native/cooperative mutators and
1–8 helpers. The final 15-command matrix passes: 356 native / 245 WASI all-feature
library tests, 11 Python checks, native/WASI lint, bare compilation and formatting,
five new fixtures under both Miri modes and ASan/TSan, and release fixtures/examples.
Nine deliberate fault processes and 39 CLI checks pass. Catalog/pool model inputs
are unchanged and their previous scoped evidence is retained with exact hashes.

All 78 final stress processes pass, covering 31.11 million allocations, destructions
and object/root releases with zero final logical payload/catalog credit. The first
fixed-watchdog revision failed 3/24 native processes; these failures are preserved.
Version 2 reports a capacity-scaled watchdog and preserves original errors across
peer cancellation. Large native requests still reach 3,186,936 polls, so this is not
a latency improvement or full sustained-load qualification. See
[the contract](CANONICAL_CONTENTION.md) and
[the evidence/graphs](../target/gc-implementation/canonical-contention-REPORT.md).
General graphs/callbacks, admitted callers, JIT and full qualification remain open.

## Bounded root-scan batches (2026-11-15)

The optional leaf helper now uses a scan of at most eight slots per preparation.
It stops at the first root or target effect and retains the original controller
snapshot, single-effect storage and certificate semantics. An unsuccessful owning
observation cancels the private preparation without advancing any cursor. The
existing single-slot raw/facade APIs remain available.

All 27 frozen commands pass: native all-feature 652 tests (one existing ignore),
WASI 345, catalog 196/181, pool 12/7, native/WASI Clippy, bare compilation and format.
All 61 leaf fixtures pass strict-provenance Miri, Tree Borrows, ASan and TSan;
release runs 61 native / 52 WASI. The eight new scan-boundary fixtures also pass
both interpreters and sanitizers. They cover empty/end bounds, failed discovery,
cancellation, late/superseding publication, root/target effect stopping and reserved
construction custody. Eleven model-runner and ten lifecycle-validator Python tests
pass. Four release binaries pass 52 CLI checks.

The corrected checker executes all 59 catalog and six pool models plus 32 expected
fault failures; all 34 cases pass and their binaries are retained. Four new models
cover competing publication, root Drop, stopped shading and failed/cancelled scans.
Eight separate native/WASI runtime fault executions detect an exceeded batch bound,
missing target/root effect stop, and failed discovery treated as an empty slot.
The first fault-driver attempt expected WASI exit 1 and a completed test summary;
assertion panics actually abort with exit 134. That failed driver record is preserved.
The corrected driver uses --nocapture and requires the exact running test, panic
witness and expected exit/trap, rather than accepting any Wasm trap.

All 54 separate allocator rounds return to their initial requested-byte balance.
The 36 canonical rounds exactly match the prior build's request/free counts,
requested/released bytes and peak requested storage. No RSS claim is made. The
252-process paired screen uses the same million-operation primitive workload,
seven independent pairs per condition, fixed CPU 0 / argv0 and randomized order.
All samples and frozen-input checks pass; no other work overlaps measured children.
Canonical lifecycle time improves 18–21%, with every paired interval below 1.
Native legacy controls remain near unchanged; WASI controls also improve 1–2.5%.
Canonical remains slower than legacy, and no latency, pause, graph, contention,
JIT, G1/ZGC or endurance gate closes. See
[the report and graphs](../target/gc-implementation/canonical-cleanup-REPORT.md).

## Concrete leaf lifecycle cost fixture (2026-11-15)

The optional `experimental-canonical-leaves` diagnostic compares legacy, canonical
explicit-slot and canonical automatic-slot allocation of the same 32-byte primitive
payload. It checks a checksum, complete per-batch payload-credit return, exact
canonical object/root releases and final catalog/payload teardown. Timing includes
allocation, initial rooting, typed read, root drop, actual cleanup and validation;
setup/teardown are separate. No generic runtime backend is exposed by this fixture.

All 14 frozen commands pass: all-feature native 652 tests (one existing ignore),
WASI 345, native/WASI Clippy, bare-Wasm library/examples, format, ten Python result
validation tests, the new fixture under strict-provenance Miri, Tree Borrows and ASan,
and native/WASI release tests and example builds. The catalog transition source is
unchanged; prior model and concurrent memory evidence is retained by exact hash,
not represented as newly executed. The bare-Wasm build rejects timing at runtime
before accessing a missing host clock.

Six deliberate faults fail at the intended checksum or incomplete-reclamation
checks on native/WASI. The early fault driver assumed native exit 101 for WASI's
exit 1; a later mutation failed compilation due to unnecessary qualification. Both
development failures are preserved and excluded. The final six executed failures
have exact witnesses and copied binaries. Four release binaries pass 52 CLI cases,
including every mode and rejection of malformed/out-of-range arguments. Parser
controls reject incomplete cleanup, identity/configuration changes, bad timing
boundaries, duplicate JSON, false/noninteger data and inconsistent allocator counts.

The separate counting executable records 54 warmed complete heap lifetimes, including
setup, teardown and the result String, with zero outstanding requested-byte growth.
These are requested allocation bytes, not RSS. Seven randomized process blocks per
batch/mode/target produce 126 uninstrumented million-operation samples. CPU 0,
argv0, toolchains, source/binary/runner hashes and the predeclared policy are recorded;
no builds, tests, counting or archives overlap measured processes. All lifecycles
exceed 100 ms. Graphs and CSV retain the large canonical time regressions. These are
single-mutator reference-free costs, not operation-tail latency, pauses, contention,
whole-JVM G1/ZGC comparisons or sustained-load qualification. See
[the report](../target/gc-implementation/canonical-leaf-cost-REPORT.md).

## Current-source tagged model checker (2026-11-15)

`benchmarks/gc/tagged_catalog_model_check.py` now uses the current 55-model catalog
suite and six-model pool suite. It preserves all 25 historical fault controls,
updates generic participant/slot-access/trace-source mutation anchors, and adds
cell/root rotation and fixed reference-role faults. All 61 models and 28 deliberately
broken variants pass their required checks. Failures must come from exactly one
executed matching fixture with its invariant witness; compile errors, timeouts,
ignored/filtered positives, duplicate summaries and diagnostic strings cannot pass.
Fifteen Python tests validate those evidence boundaries and mutation preflight.

The runner checks every exact mutation anchor before any build. Copied crates have
standalone workspace roots and distinct build directories, preventing parent-workspace
resolution or stale cross-copy binaries. The recorded Loom RMW correction, source
hashes, explicit build environment, logs and all 30 executed binaries are preserved.
`--preflight-only` explicitly reports that it executed no models. The production
workspace dependencies and collector implementation are unchanged.

Run the current checked-in command:

```sh
python3 benchmarks/gc/tagged_catalog_model_check.py --output target/tagged-catalog-current
```

The command requires an unused output directory and the already installed offline
Loom 0.7.2 source (or `--loom-source`). The corrected checker models exact transition
source; it excludes actual pooled pointer protection, concrete payloads, facade
capacities, arbitrary callbacks and whole-GC progress. See
[the evidence report](../target/gc-implementation/canonical-checker-REPORT.md).
No performance measurement or full qualification gate closes in this follow-up.

## Automatic canonical allocation retries (2026-11-15)

The internal leaf runtime now chooses discovery locations incrementally while
retaining exact payload/record ownership. Full or contended root quota also keeps
the request. Explicit-slot errors and fixed reference-role locations retain their
semantics. Each poll tries one bounded catalog step; full arrays require helper
work, root release or cancellation. This does not establish scheduling fairness,
callback bounds, public integration or whole-GC latency/throughput.

All 39 frozen commands pass: native 567 default / 651 all-feature tests (one
existing ignore each), WASI 282 / 344, catalog 188 / 173, pool 12 / 7, both target
Clippy checks, bare-Wasm library/examples and format. All 60 leaf fixtures pass
strict-provenance Miri, Tree Borrows, ASan and TSan; release passes 60 native / 51
WASI. Ten focused catalog fixtures (nine new and one existing retry fixture) also pass
all four memory tools. The five new
typed fixtures cover full object arrays, stale physical roots, full root quota,
cancellation and repeated helper cycles with one payload preparation per value.

All 55 current-source transition models pass with the corrected Loom checker;
three new cases cover cell/root announcement races and fixed reference roles.
Sixteen native/WASI fault processes and two model faults fail at their intended
invariants. A positive compiler control and four rejected programs protect private
slot words and exclusive request lifetimes. Development compile errors (two slot
accesses, a swapped fixture argument, tuple-drop borrow and an unnecessary
qualification) and the initial preparation-counter fixture failure are preserved.
The counter fixture initially used a collector without region preparation; its
explicit compact-heap configuration now exercises that hook before freezing. The first driver then rejected its expected count of nine when
catalog Miri successfully ran ten tests: the substring selected one existing retry
fixture too. V2 checks the exact ten names and completes the remaining three
commands without changing frozen sources. The rejected count assertion is retained.

All 744 previous accounting rows retain requested backing and allocation counts.
Participant reservations grow by 16 native / 8 WASI bytes per quota. Another 96
rows force both cell and root collisions; all 816 catalog lifecycle rows request
zero new backing. Forty-eight production-layout rows confirm the unchanged runtime
supplement. The [storage graph](../target/gc-implementation/canonical-retry-accounting-v1/retry-reservation-v2.svg)
shows logical participant growth, excluding payloads, caller storage, RSS and time.
See [the protocol](CANONICAL_RETRY.md) and
[the evidence report](../target/gc-implementation/canonical-retry-REPORT.md).
No timed benchmark or full qualification gate closes in this stage.

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

[Protocol](CANONICAL_RUNTIME.md) ·
[Report and graph](../target/gc-implementation/canonical-runtime-REPORT.md) ·
[Verification](../target/gc-implementation/canonical-runtime-verification.json).

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

[Protocol](CANONICAL_MAINTENANCE.md) ·
[Report](../target/gc-implementation/canonical-maintenance-REPORT.md) ·
[Verification](../target/gc-implementation/canonical-maintenance-verification.json).

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

[Protocol](CANONICAL_ALLOCATION.md) ·
[Report and graph](../target/gc-implementation/canonical-allocation-REPORT.md).

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

[Protocol](CANONICAL_LEAVES.md#typed-reads-through-canonical-roots) ·
[Report](../target/gc-implementation/canonical-leaf-access-REPORT.md).

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

[Protocol](CANONICAL_LEAVES.md) ·
[Report](../target/gc-implementation/canonical-leaf-REPORT.md).

## Rooted object-array projections (2026-11-14)

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

[Protocol](PUBLICATION.md#rooted-object-array-projections-2026-11-14) ·
[Report and graphs](../target/gc-implementation/rooted-array-REPORT.md) ·
[Verification](../target/gc-implementation/rooted-array-verification.json).

## Borrowed allocation attachment context (2026-11-14)

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

[Report and graphs](../target/gc-implementation/borrowed-owner-REPORT.md) ·
[Verification](../target/gc-implementation/borrowed-owner-verification.json).

## Root guards validate their allocation heap (2026-11-14)

A frozen weak-probe regression showed that creating a guard in a different
collector and transferring it could discard the source heap's valid root.
`create_root_guard` now rejects a collector mismatch before registration;
`Gc::as_root` returns `InvalidPointer` before cloning/registering. The borrowed
pointer comparison adds no weak clone, allocation, lock or atomic operation.
Valid guard conversions and replacing a guard with a foreign owning handle retain
independent protection.

All 21 checks pass: native GC 502/521, actual WASI 250/258, classloader 771 passed/14
ignored, nine WASI ownership, four VM GC/reference and 19 context tests. ASan/TSan
each pass 19 root/clone/guard tests and 26 publication tests. Miri and Tree Borrows
each pass the four new fixtures in the frozen prototype; every compiled source
hash matches the integrated candidate, with only documentation differences. Three
fixtures run on WASI; its aborting panic mode omits caught-panic cleanup. Native/WASI
Clippy, bare-Wasm, harness, format and diff checks pass.

All 1,008 paired timing processes pass. Identical benchmark sources and fresh
builds reproduce the prior baseline binaries byte for byte. Ratios are mostly near
one with mixed intervals; native candidate allocation rises about 1.2%, WASI candidate
cell allocation about 1.5%, and a WASI counted leaf-write control about 5.6%.
The latter does no construction inside operation timing; the control is retained
without attributing its change to per-write guard checks. No general performance,
STW, D/E or endurance qualification is inferred.

[Protocol](PUBLICATION.md#root-guards-validate-their-allocation-heap-2026-11-14) ·
[Report and graphs](../target/gc-implementation/foreign-guard-REPORT.md) ·
[Verification](../target/gc-implementation/foreign-guard-verification.json).

## Owner-aware tracing and nested-cell ownership (2026-11-14)

`Trace` is now an unsafe implementation contract. Unsafe attachment and demotion
receive the actual heap/descriptor through `TraceOwner`; raw `Gc::unroot` is unsafe.
Same-heap fields demote, foreign strong fields retain source ownership, builtin
containers forward attachment, and nested cells detach/rebind when moved. Java
instance/array fields forward the owner; moved same-heap conditional objects enqueue
their new owner without strengthening their referent. Three frozen prior-runtime
weak-probe regressions and four compile-boundary failures establish the old defects.

All 21 checks pass: native GC 498 default/517 combined-feature tests, actual WASI
247/255, classloader 771 passed/14 ignored, nine WASI ownership tests, four VM
GC/reference integrations and 19 context tests. Miri and Tree Borrows each pass
nine core fixtures without a source overlay (four concurrent iterations under
Miri versus 32 natively). Existing exposed-provenance warnings remain. ASan and
TSan each pass 21 ownership/attachment tests and 26 publication tests. Native/WASI
Clippy, bare-Wasm compilation, harness, format and diff checks pass.

All 1,008 paired timing processes pass. Operation bodies match; the recorded
compatibility patch changes only unsafe declaration/call syntax for the old API.
Native ordinary controls are broadly flat. Correct nested writes rise 14.4–16.1x
(15 ns to 223–251 ns), and WASI rises 9.1–9.9x (18 ns to 166–175 ns), because the
old path omitted owning preparation and mutation hooks. WASI controls have small
mixed changes. Reports retain diagnostics, wall-time graphs and all raw evidence.
These screens establish neither scheduled tails nor a global-pause bound.

Conservative owning keys/set/ordinary-lock fields and cross-heap cycles remain
retention limits; cross-heap Java queue delivery is unqualified. A separate
foreign-guard regression still fails against this snapshot and has an isolated
repair prototype. No general writer-root certificate, D/E or qualification gate
is closed by this delivery.

[Protocol](PUBLICATION.md#owner-aware-tracing-and-nested-cell-ownership-2026-11-14) ·
[Report and graphs](../target/gc-implementation/owner-binding-REPORT.md) ·
[Verification](../target/gc-implementation/owner-binding-verification.json).

## Manual root removal and explicit revocation (2026-11-14)

Safe numeric removal now accepts only `add_root` manual registrations. Owning and
internal roots retain exact tagged-generation removal. A kind bit and 63-bit
non-reused sequence fit the existing field; tree entries remain 16 bytes on native
and Wasm and owning tokens do not grow. Revoking every registration for an address
is unsafe, with an executable compile-fail contract. The prior frozen runtime
reproduces numeric removal invalidating a live owning handle before payload access.

All 20 final checks pass: native GC 485 default/504 combined-feature tests; actual
WASI GC 240/248; classloader 769 passed, 14 ignored; seven WASI ownership tests;
four VM GC/reference integrations and 19 runtime-context tests; native/WASI Clippy,
bare-Wasm, formatting and diff checks. Miri and Tree Borrows each pass both
revocation fixtures. Existing weak-reference exposed-provenance warnings remain.
ASan and TSan each pass 17 owning/root/revocation and 26 publication tests. Unit
fixtures cover cross-kind ID reuse, sequence boundaries/exhaustion and layout.

All 672 paired timing processes pass with unchanged frozen inputs and clean
teardown. Fresh before builds reproduce the prior delivery byte-for-byte. Native
root acquisition increases 1.7–3.2%; allocation and pending-reference transfer
changes are smaller, and shared cloning remains flat. WASI changes are mostly
mixed/within intervals, with about a 1% gated pending-transfer increase. Reports
retain diagnostics-off/on, operation and wall-time graphs. Checks, builds and the
independent next-contract compile witness finish before timing. Operation timing
excludes between-batch collection; no tail-latency or STW result is inferred.

Demotion, attachment and custom Trace safety contracts still need enforcement and
audit. This repair retains admission/tree locking and closes no D/E, JIT, pause,
peer, capacity or endurance qualification gate.

[Protocol](PUBLICATION.md#manual-root-removal-and-explicit-revocation-2026-11-14) ·
[Report and graphs](../target/gc-implementation/root-revocation-REPORT.md) ·
[Verification](../target/gc-implementation/root-revocation-verification.json).

## Owning generic clones and guard escapes (2026-11-14)

`Gc::clone` now roots borrowed edges; owning clones and `GcRootGuard::clone_gc`
share registrations. Guard cloning preserves one registration. Mutable guard
exposure roots its inner handle before `mem::replace` can move it out. Edge rooting
checks immutable identity and logical liveness under admission, canceling reserved
slots on rejection. Frozen prior-runtime fixtures reproduce all three escape bugs
using weak probes before payload access. Cycle fixtures use managed writes and
paired ownership hooks, with stronger teardown checks.

All 20 final checks pass: native GC 480 default/499 combined-feature tests;
classloader 769 passed, 14 ignored; four VM GC/reference integrations; 19 runtime
context tests; actual WASI GC 236/244 tests and seven classloader ownership tests;
native/WASI Clippy, bare-Wasm, formatting and diff checks. Six deterministic
ownership fixtures pass regular Miri (161 s) and Tree Borrows (242 s), without a
reduced overlay. Existing weak-handle exposed-provenance warnings remain. TSan and
ASan each pass 27 clone/cycle/root/contention tests and four cell tests. Seven core
ownership fixtures cover 12 policy settings, cancellation and concurrent movement.
The pre-guard revision's interrupted validation and initial failures are retained.

All 1,008 paired timing processes pass. The same benchmark explicitly borrows an
edge on both frozen revisions to preserve root-registration work. Diagnostics-off
native root acquisition rises 8.5–10.6%; shared owning clones rise roughly 4.7–8.0%,
with one interval including one. Guard-clone changes are smaller. Edge copying now
costs 4.3–5.1× native / 6.0–6.6× WASI; overwrite costs 1.60–1.71×. These two cases
intentionally add ownership that the prior runtime lacked. WASI root acquisition
and shared-clone changes are smaller/mixed. Collection is excluded from operation
time and included in separate wall-time results. No checks/builds/source edits
or counter runs overlap timing. All raw records, inputs and graphs are retained.

Public demotion/revocation and custom Trace safety contracts still need enforcement
and audit. Existing admission remains; no D/E, pause, JIT, peer, capacity or
endurance gate is closed. Experimental defaults are unchanged.

[Protocol](PUBLICATION.md#owning-generic-clones-and-guard-escapes-2026-11-14) ·
[Report and graphs](../target/gc-implementation/edge-clone-REPORT.md) ·
[Verification](../target/gc-implementation/edge-clone-verification.json).

## Scoped primitive projection certificates (2026-11-13)

`experimental-trace-projections` is disabled by default. Typed primitive-array views
publish an empty-graph certificate only after successful mapping and revoke it
before writer cleanup/unlock. Each certified scan performs an atomic CAS. Nested
mapping, failure and unwind retain this lifetime; later unrestricted/reference-
bearing writers still require tracing. The feature does not permit relocation.

Four Loom models compile production certificate transitions. Core fixtures cover
12 publication/phase/root-storage settings, later graph replacement, failure,
unwind and a stopped native writer. Classloader fixtures cover 180 typed/raw views
across eight primitive variants. Default/combined-feature GC suites pass 473/492
native and 231/239 actual WASI tests. Classloader suites pass 767/769 tests (14
ignored each); feature-enabled VM checks pass four GC/reference integrations and
the generated array-access continuation matrix. Native/WASI Clippy, bare-Wasm,
formatting and diff checks pass. Separate negative sources reject omitted
revocation and disabled certification.

Regular Miri passes all four core fixtures. The original Tree Borrows matrix times
out at 900 seconds; its partial log and orphan-interpreter cleanup are preserved.
A separate validation-only source overlay reduces repeated busy probes from 1,000
to 8, retaining all 12 settings, explicit Busy checks, later graph replacement and
teardown. That Tree Borrows matrix passes; all production source hashes match.
Native and regular Miri retain 1,000 probes. Existing weak-handle exposed-provenance
warnings limit both Miri results; they are not strict-provenance proofs. TSan passes
four projection, four guard and 25 construction/contention tests. ASan passes four
projection, four guard, two classloader projection and eight ownership tests.

All 1,344 paired timing processes pass. Managed Reference cells grow from 296 to
304 bytes natively and 144 to 148 on WASI; write/mapped guard sizes are unchanged.
At length 32, native typed borrowing costs 6–9% more and byte views 9–19% more.
WASI primitive results and reference controls are mixed, including regressions.
Default release binaries reproduce the preceding runtime byte-for-byte using the
same expanded benchmark. One timing child runs at a time without overlapping
builds, checks, counters, source edits or Miri. The feature remains opt-in, with no
pause, JIT, G1/ZGC, capacity or endurance qualification and no D/E gate closure.

[Protocol](PUBLICATION.md#scoped-certificates-for-trace-free-projections-2026-11-13) ·
[Results and graphs](../target/gc-implementation/trace-projection-REPORT.md) ·
[Verification](../target/gc-implementation/trace-projection-verification.json).

## Whole-value ownership and narrow mutations (2026-11-13)

Managed guards now restore strong ownership before unrestricted mutable access or
mapping. Whole `Reference` movement therefore retains its transitive graph after
the source and context disappear. Narrow field/array stores preserve barriers
without rooting unrelated edges; point borrowing roots only its selected value.
Custom `Trace` implementations that demote fields must implement `Trace::root`.
No locked graph scan is skipped.

Checks pass: 469 default/484 optional-index native GC tests, 231/237 actual WASI
GC tests, 767 classloader tests (14 ignored), four VM GC/reference integrations,
14 field-cache, six field-instruction, 33 array-instruction and 13 `aastore`-matched
tests, plus generated field and array-store continuation matrices. Native/WASI
Clippy, bare-Wasm compilation, formatting and diff checks pass. Miri checks narrow
mutation under both aliasing models and whole-move unwind. TSan passes four guard
and 25 construction/contention tests; ASan passes eight ownership and four guard
tests. Ownership includes 192 whole-value and 192 element-access configurations,
12 narrow-store settings, mixed narrow/whole access, cancellation and teardown.
An initially empty field-test filter is preserved; its correction ran six tests.
Both whole-move and point-move negative source mutations lose the expected graph.

The initial 672-process whole-value screen found a 37–39× native / 46–67× WASI
32-field-store regression. The safe narrow APIs and VM/benchmark caller migration
remove this amplification. A separate 1,344-process screen compares the final
candidate with both original and whole-value-repair sources: the gated 32-field
store is approximately 94 ns on native and 48 ns on WASI, close to original.
All processes pass teardown checks. Small mixed read/slice changes remain, including
slightly slower gated slices; whole/slice root acquisition is still linear.
Sources, source-specific build directories, binaries, exact commands, raw samples,
paired intervals and SVG/PNG graphs are preserved. Builds, checks, counters and
edits do not overlap timing. No pause, G1/ZGC, production-coverage, capacity or
endurance criterion is qualified, and D/E remains open.

[Protocol](PUBLICATION.md#whole-value-ownership-and-narrow-mutation-2026-11-13) ·
[Results and graphs](../target/gc-implementation/narrow-mutation-REPORT.md) ·
[Verification](../target/gc-implementation/narrow-mutation-verification.json).

## Shared identity insertion intents (2026-11-13)

The optional identity index now permits concurrent helpers to complete a prepared
insertion. Each immutable attempt owns an expected bucket version and a privately
initialized candidate. Every head replacement first certifies the old head's
insertion; a helper rechecks that certificate after observing a different header.
Fresh headers, including empty headers, prevent a late CAS from becoming valid
again after retirement. Retargeting replaces the entire private attempt rather
than rewriting a potentially published successor. Collector registration currently
keeps these intents local and retains its existing admission; canonical metadata,
record discovery and phase-coupled completion are still required stage-D work.

The feature passes 481 native GC tests, 235 actual WASI tests, 764 classloader
checks (14 ignored), four VM GC/reference integrations, native/WASI Clippy and
bare-Wasm compilation. Eleven production-state Loom models add competing helpers,
retargeting, a winner stopped immediately after insertion, and a delayed CAS after
retirement/head return. Two separately frozen mutations fail these forced models:
omitting the old-head insertion store, and comparing a freshly loaded header
instead of the expected header. Both failures are preserved independently of the
green implementation.

Fourteen native index tests cover cancellation, contended shared records, old
removal/marker schedules and iterative 100,000-node destruction. Four helpers
publish 500 shared intents, each removed exactly once. Forced helpers stopped
before/after their CAS retain exactly two index nodes across 1,000 later pairs;
dropping the final intent releases both. The counter excludes header/attempt/flag
backing. Six Miri cases pass its default aliasing model; the concurrent-helper case
also passes Tree Borrows. ASan and TSan pass all fourteen index cases; TSan also
passes 25 construction/contention integrations and the stopped collector-reader
case. These are scoped implementation checks, not a proof of allocator progress
or of complete collector closure while an admitted publisher is stopped.

All 1,344 performance processes pass across two frozen paired native/WASI matrices.
With diagnostics off, shared insertion costs 25–27% more native allocation time
and 13–14% more allocation wall time than the exclusive marker revision; native
handles cost 1–3% more. WASI allocation costs about 11% more, wall costs 6–7% more,
and handles 8–11% less. Cooperative collection differences versus that revision
are small/mixed; native multiworker results include a 12% regression in gated wide
graphs. Against the default map, native allocation costs about twice as much,
wall costs 145–150% more and handles 37–38% more. WASI allocation costs 29–32% more,
wall costs 55–57% more, and handle differences vary. Cooperative collection costs
4–10% more native and 4–10% less WASI. These exploratory process-pair ratios and
unadjusted intervals do not measure STW or scheduled-arrival application latency.

Forty-eight separate counter-only forks show the representation cost. In the
native baseline, 10,000-object ephemeral fixture, allocation requests per object
are 7.3236 (shared), 4.3330 (exclusive markers), and 3.3325 (map); collection
requests are 0.4096, zero and zero. Shared insertion allocates an insertion flag,
node, proposed header and attempt; retargeting prepares another candidate and
head unlinking allocates a fresh header. Empty-header setup also has a cost outside
the measured allocation loop. Full backing accounting remains open. No builds,
tests, counter forks or source edits overlap timing runs. Default native/WASI
release binaries again reproduce the prior default byte-for-byte. The feature
stays disabled; no pause, JIT, peer, capacity or endurance gate is qualified.

[Protocol](PUBLICATION.md#experimental-owning-identity-index) ·
[Results and graphs](../target/gc-implementation/identity-help-REPORT.md) ·
[Verification](../target/gc-implementation/identity-help-verification.json).

## Embedded identity-link markers (2026-11-13)

The intermediate exclusive-insertion revision stores successor/deletion state using immutable
marker bytes inside each owning node allocation. Preparation allocates one node;
logical deletion and physical unlinking allocate no link states. The private
ArcSwap adapter restores the exact original Arc pointer and count. This changes
the representation, not the current exclusive insertion or phase-admission rules.
Its head-return ABA argument and adapter contract are documented in
[PUBLICATION.md](PUBLICATION.md#experimental-owning-identity-index).

The feature passes 473 native GC tests, 234 actual WASI tests, 764 classloader
tests (14 ignored), four VM GC/reference integrations, native/WASI Clippy and
bare-Wasm compilation. Seven bounded Loom models compile the shared production
state machine, adding head-return schedules for delayed insertion and deletion.
Ten native index tests include simultaneous ordinary/marker ArcSwap borrows,
fast-borrow-slot exhaustion, concurrent link-state changes, stalled cursors and
100,000-node small-stack teardown. Four Miri cases pass under its default aliasing
model; the concurrent marker test also passes Tree Borrows. ASan and TSan pass all
ten index tests; TSan also passes 25 construction/contention integrations and the
real-collector stopped-reader case. The separate stale-predecessor negative variant
still fails its forced Loom schedule. These checks do not model ArcSwap internals
or prove unbounded progress. Default native/WASI release binaries reproduce the
preceding delivery byte-for-byte.

All 1,344 processes in two paired native/WASI matrices pass with frozen inputs
unchanged. Seven independent pairs per condition compare the new representation
with both the initial prototype and the default map. With diagnostics off, native
allocation operation costs improve 3–4% against the initial prototype, but wall
costs including reclamation improve less than 1%; handle operations regress 3–5%.
WASI allocation changes are small/mixed and handles regress 9–12%. Cooperative
full-graph collection improves 5–9% native and 10–15% WASI against that prototype.

Against the default map, native allocation operation costs remain 57–60% higher,
wall costs 118–119% higher and handle costs 36–38% higher. WASI allocation costs
remain 15–20% higher, wall costs 43–45% higher and handles 16–20% higher. Native
cooperative collection remains 4–6% higher; multiworker results are mixed. WASI
collection is 6–8% lower. These exploratory, unadjusted process-pair intervals
measure mechanisms and full-cycle cost, not application latency or STW pauses.
The feature stays disabled by default.

Forty-eight separate counter-only processes cover both targets, two policies,
1,000/10,000 objects and ephemeral/retained-weak cases. In the native baseline
10,000-object ephemeral fixture, publication allocation requests per object change
from 7.3198 to 4.3330, versus 3.3337 for the map; collection requests change from
1.5904 to zero. Requested publication bytes change from 499.6696 to 413.8536, versus
406.6216 for the map. These are allocator requests, not RSS or complete physical
budget accounting. Performance runs overlap no builds, tests, counters or source
edits. An output-path collision aborted one matrix before starting a child; its
log is retained and both remaining matrices succeeded at separate output paths.
Canonical publication, stalled-publisher/writer progress, JIT coverage, pauses,
peer comparisons and capacity/endurance qualification remain open.

[Results and graphs](../target/gc-implementation/identity-markers-REPORT.md) ·
[Verification](../target/gc-implementation/identity-markers-verification.json).

## Owning identity-index prototype (2026-11-13)

An integrated prototype is retained behind the disabled-by-default Cargo feature
`ristretto_gc/experimental-identity-index`. Versioned owning links couple logical
deletion to its frozen successor, validate the exact predecessor when unlinking,
and detach retired history. Sweep removal is resumable; a dirty bitmap drives
bounded maintenance and a cancelled cursor returns its unfinished bits. Numeric
lookup remains an accelerator for the authoritative object-address map. This does
not replace phase admission or complete canonical publication/graph retention.

The default passes 460 native GC and 229 WASI tests; the feature passes 468 and 233.
Both pass 764 classloader tests (14 ignored), four VM GC/reference integrations and
native/WASI Clippy. Feature checks include bare-Wasm compilation, two Miri cases,
seven index TSan tests, 25 contention/construction integrations and the stopped
collector-reader TSan case. Five production-state Loom models pass with bounds of
two preemptions and 10,000 permutations/branches. ArcSwap internals are not modeled.
A frozen negative variant uses a fresh predecessor link instead of the expected
version: the forced late-remover model exceeds its branch bound, followed by Loom
cleanup abort. That rejected variant is separate from the green implementation.

Seven index tests exercise private cancellation, exact identity/address removal,
resumable visibility, cancelled maintenance, stopped removers/readers, 2,000
contended insert/remove pairs and iterative 100,000-node teardown on a 128 KiB
stack. The actual collector reclaims payloads through 33 full cycles with a stopped
identity reader, retaining one index node until that reader resumes. Earlier
native/WASI failures exposed excessive empty-directory maintenance work; replacing
directory walks with dirty-word claims makes the original deadline/overlap tests
pass without relaxing their thresholds. Initial failures remain in the evidence.

The initial representation is rejected as a default replacement. Three matrices
(initial, restored-default check, retained feature/default comparison) contain
2,016 validated processes with frozen binaries, fixed invocation names and seven
process pairs per condition. In the retained diagnostics-off comparison, native
allocation costs are 63–70% higher, allocation wall costs 119–125% higher, and
handle costs 26–29% higher. WASI allocation costs are 15–19% higher and allocation
wall costs 44–48% higher. Cooperative collection costs rise about 14–15% native
and 6–8% WASI; native multiworker results are mixed. The default restoration screen
is close to the previous implementation but does not establish exact equivalence.
Thirty-two separate counter-only runs show approximately four additional system
allocation requests per object and 1.5904 additional collection requests per object
in 10,000-object fixtures. These counters do not isolate every timing cause or
qualify RSS/backing bounds. No builds/tests/counter runs or source edits overlap
performance measurement. No pause, peer, capacity or endurance gate is claimed.

[Results and graphs](../target/gc-implementation/identity-index-REPORT.md) ·
[Verification](../target/gc-implementation/identity-index-verification.json) ·
[Protocol](PUBLICATION.md#experimental-owning-identity-index).


## Owned append records and invocation-sensitive collection costs (2026-11-09)

The fallback tracing FIFO now supports an owning prepared append record shared by
helpers. Preparation selects its first predecessor privately; a helper retargets
only after that predecessor's next link is permanently occupied. Consumers record
insertion before tombstoning the link, so a producer stopped before/after linking
can resume after consumption without inserting again. Last-owner cancellation
refunds an unlinked preparation. Positive reservation counts may include private
records, so closure inspects actual links under the existing external interlocks.
The collector still keeps its current append records local and retains admission;
discoverable registration records and phase-coupled completion remain D work.

Twelve queue tests cover cancellation, wrong-queue rejection, unwind, competing
helpers/consumers, predecessor changes, stale retries, payload ownership and
bounded retained history. A stopped original helper retains four/five nodes after
1,000 later push/pop pairs, returning to two after release. Four helpers and four
consumers deliver 2,000 shared records exactly once; the existing 8,000-item
producer/consumer stress and 100,000-node small-stack destruction test also pass.
A preserved counterexample variant removes the consumer insertion store and fails
the forced late-helper schedule. It is a rejected implementation, not a failing
current collector test.

Eight bounded Loom models compile the production queue state machine. The three
new models cover shared-record publication, consumption/retargeting with late
helpers, and private cancellation after phase closure. Existing closure and claim
models remain. Bounds are two preemptions and at most 10,000 permutations/branches;
ArcSwap internals and whole-collector progress remain outside their proof scope.

The retained implementation passes 455 native GC checks, 229 WASI GC checks,
764 classloader checks (14 ignored) and four native VM GC/reference integrations.
Native/WASI Clippy, bare-Wasm compilation, two focused Miri checks, twelve queue
TSan tests and 25 existing TSan integration checks pass. Five benchmark-harness
tests include a real native process invoked through differently sized executable
paths while receiving the same `argv[0]`. All protocol, test and binary revisions,
including the deliberate negative variant and an initial missing build-script
invocation, are preserved in the
[verification record](../target/gc-implementation/helpable-append-verification.json).

Six 336-process paired screens, three 63-process invocation/layout controls and
16 untimed diagnostic forks are retained. Diagnostic forks observed no fallback
appends in cooperative tree/wide workloads. Merely changing the invocation-name
length reversed the native comparison, so the runner now fixes that name on native
and WASI and records the executable separately. Outlining the fallback helped the
initial WASI screen; aligning the counters did not resolve the native sensitivity
and was removed. Native queue-header alignment reduced absolute sensitivity in the
three tested layouts. Private predecessor selection then changed cooperative costs
despite no fallback execution, so those changes cannot be attributed to eliminated
executed append CAS operations.

The retained fixed-name screen improves cooperative native collection cost by
about 5–7%, but native multiworker results are mixed, including a roughly 7%
gated-wide regression. WASI costs regress about 4–8%. These are complete graph
collection timings, not allocation throughput, application response latency or
STW measurements. They do not qualify any performance/progress/endurance gate.
See the [reports and graphs](../benchmarks/gc/IMPLEMENTATION.md#owned-append-records-and-invocation-sensitive-collection-costs).

## Owning typed exception matching (2026-11-09)

The initial real-VM regression showed typed matching completing inside generated
code. Experimental compilation now returns an argument-free `ExceptionMatchRequest`
before resolver/assignability waits. Distinct table-entry resume tokens retain the
throwing BCI, including duplicate types and a throwing instruction that already has
its own runtime continuation. The pending throwable and live locals remain owned
by the context, and every ready match consumes shared fuel. Synchronous embedding
uses the same matcher through its explicit executor bridge.

An eight-configuration native matrix covers both publication and storage policies,
zero/one collector worker and one executor worker. Sixteen forced real loader waits
verify pending throwable/local survival and unrelated reclamation through full GC,
future migration, cancellation and handler resumption on a different OS thread.
Sixteen held-payload-writer schedules verify yielding and cancellation/completion;
they do not claim full collection while the writer remains held. The existing E
counterexample and graph-retention work remain open.

Ordered and duplicate handlers, catch-all fallthrough, native missing catch types,
source/result identities and target non-initialization have interpreter, resumed
and synchronous compiled checks. A missing catch-class regression originally
escaped as a native loader error. Interpreter lookup now propagates a Java linkage
throwable, while compiled resolution caches `NoClassDefFoundError` and exits the
current method's table on resolution failure. Broader Java-loader errors/access
checks are not established by this native missing-class case. The semantics follow
[JVMS exception handling](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.10)
and [athrow](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.athrow).
Seven exact exported classes pass 17 independent HotSpot checks in each of `-Xint`
and `-Xcomp` with `-Xverify:all`. HotSpot may resolve the missing catch class during
verification; that case establishes the escaping error type, not matching timing.

Three backend regressions verify distinct typed resume sites, propagation past a
catch-all on lookup failure, and bounded handler continuation expansion. Two
portable interpreter exception tests pass on WASI. The full native JIT suite passes
485 library/integration/doc tests (one ignored). The profiling-enabled VM suite
passes 1,002 library tests (three ignored) and four GC/reference integration tests.
A final fixture-only addition checks cached linkage failures while holding the
loader lock; its matrix is rerun separately, and the preceding full-suite binary
and exact fixture variant are preserved. Native/WASI lint and bare-Wasm JIT API
compilation also pass. Source/binary hashes, exact fixtures and failed attempts are
retained in the
[verification record](../target/gc-implementation/exception-request-verification.json).

This introduces no new atomic or unsafe ownership protocol. It reuses the existing
continuation/frame roots and does not expand production handler eligibility.
Throwable construction/validation and other native helpers still need cooperation;
request and compilation overhead, collector D/E, compiled workload coverage,
sub-millisecond pauses and sustained-load qualification remain open. No GC or JIT
policy defaults change, and no performance claim follows from these tests.

## Helpable publication design checks (2026-11-09)

Eleven Loom design tests pass: four expected counterexamples to rejected designs
and seven positive reduced atomic-cell checks. The first red run demonstrates
that replaying the production mark primitive after early acknowledgement can
rewind a newer epoch. Further witnesses cover stale registry insertion after
identity reuse, missing traversal after mark publication and an uncoordinated
announcement crossing closure. These schedules remove protection deliberately;
they do not establish a defect in the currently admitted runtime.

Positive checks cover competing helpers, a stopped submitter, cancellation,
retirement/reuse, overflow and one-record two-domain announcement/closure coupling.
They do not model a scalable registry, graph ownership, payload lifetime or physical
reclamation. The existing 28 admission/mark models also pass, as does targeted lint.
No production Rust source changes in this batch, and no runtime/performance claim
follows. [The design record](validation/HELPABLE_PUBLICATION.md) specifies the next
canonical-membership and catalog/phase work, with D/E and all qualification gates
still open. [Frozen evidence](../target/gc-implementation/helpable-publication-verification.json)
preserves the initial compile failure, actual red witness, passing models and inputs.

## Owning reference constants (2026-11-09)

The initial live-VM regression failed because the backend rejected string constants.
Experimental string/class `ldc` and `ldc_w` now return an argument-free owning request
before awaiting resolution, interning or mirror construction. Numeric constants stay
inline. The interpreter and both compiled execution APIs share resolution. A second
regression found native missing-class literals becoming `InternalError`; this path
now raises `NoClassDefFoundError`, including through a matching exception handler.

The native matrix runs under both publication policies, host/compact storage and
zero/one collector worker on a one-worker executor. Its 64 forced waits cover narrow
and wide cold-string/class loads, completion and cancellation. A residual reference
is the only live copy after its input local is cleared and the external owner drops.
It survives repeated full collection while the real loader lock is held, unrelated
garbage dies, the future migrates to another OS thread, and cancellation releases
the operand. Successful completion checks helper-result lifetime before resumption
and escaped result ownership after context teardown. Already-ready constant loops
consume fuel and cancel with no frame retained.

Semantic fixtures compare interpreted, resumed and synchronous compiled results,
interned/mirror identity, MUTF-8 including unpaired UTF-16 surrogates, pool indices
above 255, category-two residuals and caught/uncaught errors. A target with an
infinite `<clinit>` proves its class literal does not initialize it. The exact
exported classes pass 24 independent HotSpot checks in both `-Xint` and `-Xcomp`
with `-Xverify:all`. Thirteen portable constant instruction tests execute on WASI.
The full native suites pass 999 VM library tests (three ignored), four GC/reference
integration tests and 482 JIT library/integration/doc tests (one ignored). Native/WASI
lint, bare-Wasm JIT API compilation and formatting checks pass. The
[verification record](../target/gc-implementation/constant-request-verification.json)
records full affected-suite results, source/binary hashes and preserved failures.

No new atomic or unsafe protocol was introduced; requests reuse existing frame/root
ownership and continuation machinery. This is capability/correctness evidence,
not a timing comparison or production JIT qualification. Broader Java-loader error
translation/access checks, unsupported method-type/handle/dynamic constants,
exception-matching cooperation, request overhead and collector D/E remain open.
No policy defaults, pause bounds or sustained-load gates are qualified here.

## Mutable array extraction owns escaping values (2026-11-09)

A before-fix regression lost a still-held extracted `Value` after its source array
was dropped and three full collections completed. It fails at a weak-liveness
assertion before dereferencing the reclaimed handle. Mutable element, slice and
iterator access now acquires ownership before exposing values. Guard cleanup
demotes retained elements. Replacement-only stores preserve dirty cards and
deletion barriers without rooting discarded edges; Java field stores, `aastore`
and multidimensional initialization use this path.

The matrix covers seven extraction APIs × two publication policies × host/compact
storage × three root stores × immediate/one-record barriers: 168 cases each on
native and WASI. It drops the source and execution context, preserves a transitive
graph through three full cycles, verifies contents, then releases the extracted
owner and checks reclamation and zero managed bytes. Separate tests cover
unconsumed iterator values, mapped guards, invalid-index input ownership, no extra
root registrations for replacement stores, native unwind and portable future
cancellation. Existing sparse-card and dirty-copy checks still pass.

Final validation passes 741 classloader library tests (nine ignored), 23 classloader
integration tests (five network-dependent tests ignored), 996 VM library tests
(three ignored), and four GC/reference integration tests. Focused native checks
pass nine tests; eight applicable tests execute under WASI. An initial WASI run
aborted at the panic-unwind fixture because this target cannot catch that panic;
unwind is now native-only, with a separate cancellation fixture on both targets.
Native/WASI lint, the new example's lint, bare-Wasm classloader compilation,
formatting and diff checks pass. No new unsafe code or atomic protocol was added;
these checks use the existing root/barrier machinery rather than a new Loom model.

The corrected paired screen runs 336 native and 336 WASI processes, seven pairs
per condition, on CPU 0 with one child at a time and no overlapping builds/tests.
It freezes source archives, binaries, harnesses and Wasmtime. Its fixed live graph
compares field stores, element extraction, whole-slice borrowing and reads at
lengths 1/32 under gated host, counted host and counted compact/context policies.
Collection, setup, integrity checking and process startup are outside the measured
interval. Every process checks contents and zero-object/zero-byte teardown. This
is aggregate mechanism cost, not latency, capacity, equal-RSS or endurance evidence.

An earlier 672-process attempt mistakenly compared identical baseline binaries:
the shared Cargo target directory reused artifacts for the candidate despite a
different source tree. Both original screen directories retain `INVALIDATED.json`,
raw data and reports. Candidate binaries were rebuilt in a fresh isolated target
directory; their distinct hashes and source binding are recorded. The harness now
rejects identical before/after inputs before launching a child, with a negative
check. Only the `*-screen-2` results support the reported comparison.

Native mutable-element cost rises about 1.8–2.0×; WASI rises about 2.3–2.7×. A
32-element reference slice rises about 34.5× native and 25–37× WASI because every
exposed element acquires ownership. Field stores remain near their previous cost;
the full tables retain controls and all intervals. This repair is retained for
correctness, with bulk ownership cost and cooperation explicitly unfinished.
Whole-value writers, tracing callbacks and stalled admissions still need D/E's
general protocol. No pause, JIT coverage or sustained-load gate is qualified.

See [results and graphs](../benchmarks/gc/IMPLEMENTATION.md#mutable-array-extraction-retains-owning-roots)
and the [verification record](../target/gc-implementation/edge-extraction-verification.json)
for frozen inputs, commands, failures and the exact scope of this evidence.

## Owning array access and guard-free errors (2026-11-09)

Experimental compiled primitive/reference loads, primitive stores and `arraylength`
now suspend through owning requests. The shared runtime implementation separates
scalar access from throwable construction; no array guard survives a wait or error
allocation. Payload acquisition tries the lock and yields on contention. The
synchronous API shares the implementation and avoids the bridge on uncontended
success. Reference loads acquire ownership before releasing the array guard and
transfer the result out of the completed context. No production eligibility changed.

Two before-fix tests reproduced boolean stores retaining `2` instead of `0` in both
interpreter and JIT. Both now apply the [JVM low-bit rule](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.bastore).
Generated fixtures compare interpreter, resumed JIT and synchronous JIT across all
nine array component kinds: signed/narrowed integers, float/double raw bits, signed
zero, subnormals, infinities and quiet NaNs. They check null-before-bounds ordering,
unchanged failed stores, wide/residual state, caught bounds errors and a ready-only
infinite store loop. The exported class passes 217 HotSpot checks under both `-Xint`
and `-Xcomp`, with full bytecode verification.

The native matrix covers both publication policies, host/compact storage and zero/
one registered worker. Its 112 suspended schedules retain the exact request, test
migration and cancel or complete while the resource is held. Sixty-four class-loader
waits during error construction leave the array writable and permit full collection
of unrelated garbage. Sixteen held-read-lock store cases also permit full collection.
A separate reference-result case in every configuration releases the source array
while preserving the returned child, then reclaims the child after its owner drops.

Thirty-two held-write-lock load/length cases establish request cooperation only.
The first attempt timed out after 300 seconds; bounded follow-up confirmed a full
cycle could not finish within 100,000 steps. The managed `Reference` enum can contain
edges, so its held writer blocks tracing even for a primitive-array variant. The
final fixture records bounded collector probes, then checks cancellation/migration
and successful collection after unlock. This evidence explicitly leaves D/E open;
a passing request test is not a passing reclamation-under-stalled-writer gate.

The old 1,024-call native-spill fixture now exceeds the existing 65,536-cell
continuation expansion limit. The live-value fixture uses 248 opaque reads and
retains its >4 KiB native-frame assertion and exact computed sum; the separate
oversized-state rejection fixture remains. Earlier loop/handler test drivers now
execute array requests and count the added initial-store resume, preserving their
root, exception, migration and backedge assertions. Backend test helpers now perform their
own null checks, matching the runtime contract after inline checks were removed.

Final checks pass 996 native VM library tests (three ignored), four GC/reference
integration tests, 232 JIT library tests (one ignored), 249 JIT integration tests
and one doctest. Actual WASI execution passes all 11 byte-array interpreter tests,
including the boolean regression. Native/WASI lint, bare-Wasm JIT compilation,
formatting and diff checks pass. The final full VM run records all eight configurations,
112 suspended schedules and 32 incomplete held-writer probes. Initial compile/lint
failures, the oversized spill fixture, stale manual drivers/counts and the first
failed full VM run are retained alongside the successful reruns.

The [verification record](../target/gc-implementation/array-access-verification.json)
binds commands, failures, exact oracle input, full source archives and frozen
binaries. No new unsafe implementation or atomic protocol was added. Performance,
STW telemetry, production hot-path coverage, full relocation and endurance remain
unqualified. No timing graph or throughput claim follows from these semantic and
cooperation checks. Constants, exception matching, native reentry and collector
publication/graph progress remain required work.

## Checked reference stores and owning requests (2026-11-09)

Two reproduced errors preceded this change: interpreter `aastore` accepted an
incompatible `Object` in `String[]`, and the JIT reported `ArrayStoreException`
instead of an out-of-bounds error when both checks failed. Interpreter and compiled
paths now share null/bounds/type ordering and resolve the actual array component
without initializing it. A third reproduced failure rejected legal `int[][]` to
`Cloneable[]` covariance; assignability now recurses over resolved component
identities instead of comparing rank and descriptor suffixes. Three older unit
fixtures now use actual array classes instead of attaching `Object` metadata to
array payloads.

Experimental compiled `aastore` returns an owning request with array, signed index
and value cells. Type resolution and throwable construction run after its native
frame returns; ready requests consume the shared continuation fuel. The interpreter
enters its async boundary and owns popped operands through the wait. Neither path
holds managed payload guards across resolution. Commit reacquires the destination
and rechecks class identity and bounds before the existing managed write barrier.
Synchronous embedding shares the checked implementation through its existing bridge.

Sixteen held-loader schedules cover completion/cancellation under both publication
policies, host/compact storage and zero/one registered worker. They verify exact
request cells/BCI, writable and unchanged pending destinations, rooted array/value,
unrelated reclamation, migration and cancellation while the loader remains held.
After completion, dropping the compiled context preserves the stored edge through
collection; teardown releases both objects. Portable interpreter tests separately
verify pending-operand ownership, guard release and cancellation. Sibling-loader
components absent from name lookup retain their distinct identities and remain
uninitialized. Covariance tests include primitive arrays, interfaces and differing
ranks. These are forced-progress checks, not full relocation or pause qualification.

Real interpreter, resumed JIT and synchronous JIT fixtures agree on failures and
unchanged destinations; generated methods also retain wide residual state, catch
store errors and yield from an infinite ready-store loop. The exact exported class
passes 38 independent HotSpot checks under both `-Xint` and `-Xcomp`, with full
verification. The oracle covers finite semantics, not Ristretto root lifetime.

Final checks pass 993 native VM library tests (three ignored), four GC/reference
integration tests, 232 JIT library tests (one ignored), 249 JIT integration tests
and one doctest. Actual WASI execution passes 42 interpreter object tests, including
the shared store/cancellation cases. Native/WASI lint, bare-Wasm JIT compilation,
formatting and diff checks pass. No new unsafe code or atomic protocol was added.
The [verification record](../target/gc-implementation/reference-store-verification.json)
preserves the original failures, commands, exact oracle classes, frozen binaries,
source snapshots and final archive. The initial test compile and unused-import
failures are retained too.

Required interpreter type checks and request transfers have unmeasured costs. This
batch makes no performance claim and closes no qualification gate. Remaining array
loads/primitive stores/length, constant helpers, exception matching and native
reentry need separate cooperation work. Full semantics/relocation, production hot
paths, collector D/E, complete pause capture and J5/H endurance remain open. No
default changed. See the [reproduction commands](../benchmarks/gc/README.md).

## Owning compiled type checks (2026-11-09)

Experimental `checkcast` and `instanceof` now return owning runtime requests before
symbolic resolution or assignability can wait. The activation preserves the exact
bytecode, target index, object identity, locals and residual operands; the existing
context snapshot retains references. Ready requests consume the shared local fuel.
Synchronous embedding uses the same checks, and assignability is shared with the
interpreter. Null bypasses resolution, target classes remain uninitialized, and
cast errors publish owned throwables before dispatch. Production eligibility is
unchanged. No new atomic protocol or unsafe code was introduced.

The new regression fails before the change because a cast-only method cannot leave
its native frame. The final matrix covers both publication policies, host/compact
storage and zero/one registered worker. Thirty-two forced class-loader waits cover
both instructions and completion/cancellation in all eight configurations. While
resolution is pending, two full collections preserve the context-owned operand and
reclaim an unrelated array. The pending future migrates to another OS thread;
cancellation releases its roots while the loader remains locked. Completion
preserves the returned identity, and final teardown makes the operand reclaimable.
These are forced-progress checks, not pause measurements or full relocation proof.

Interpreter, resumed generated code and synchronous generated code agree on null,
primitive/reference/multidimensional arrays, class/interface targets and cast errors.
Additional fixtures preserve wide locals and residual operands, catch cast errors,
skip missing-type resolution for null and yield from an infinite ready-only type-check
loop. The exact exported finite classes pass 93 HotSpot checks under both `-Xint`
and `-Xcomp` with full verification. The independent oracle does not exercise
Ristretto suspension or rooting. Reproduction commands are in the
[benchmark README](../benchmarks/gc/README.md).

The instrumented native JIT suite passes 232 library tests (one ignored), 249
integration tests and one doctest. The native VM library passes 987 tests (three
ignored), and its GC/reference integration suites pass four tests. Actual WASI
execution passes all 38 interpreter object-instruction tests. Bare-Wasm JIT
compilation and native/WASI lint checks pass. The first WASI attempt used
an uninstalled `wasm32-wasip1` target; the successful execution uses the installed
`wasm32-wasip2` target. The
[verification record](../target/gc-implementation/type-check-continuation-verification.json)
retains that failed attempt, the reproduced regression, intermediate test/lint
failures, commands, final source archive and oracle evidence.

This is correctness and cooperation work. No timing comparison, pause bound,
production hot-method coverage or endurance result is claimed. Array and constant
helpers, exception matching, native reentry, full resolution/relocation semantics,
J1–J5 qualification and collector D/E remain open. Defaults remain unchanged.

## Allocation index preparation without admitted waits (2026-11-09)

Metadata registration now prepares object, identity and young-index guards before
changing marks or index membership. Failure drops the whole partial write set and
any legacy admission, waits on the unavailable index alone, then retries. Counted
admission begins only after all guards are available. Successful publication keeps
the existing phase/birth marks, reference ordering and queued tracing rules; every
retry samples current phase state. Construction roots, unpublished storage and
logical credits retain their existing ownership through the wait.

A forced schedule fails on the original implementation: the identity-index waiter
retains global admission and collection cannot progress. The final test passes 36
configurations across both publication/phase policies, all three root stores and
held object/identity/young indexes. The 24 identity/young cases finish the current
cycle and three subsequent full cycles while allocation remains blocked. The 12
object-index cases reclaim unrelated garbage in the active sweep; the held shard
still prevents complete enumeration. Failed preparation exposes no partial object
or identity registration. Resumption preserves the child's value, installs the
correct identity/young membership, and teardown destroys every fixture object once
and refunds all logical bytes. Five-second test timeouts detect stalled schedules;
they do not establish pause bounds. A portable six-case rollback test additionally
checks released locks, no counted revision change on failure and successful retry.

Final validation passes 434 native GC tests across library, integration/model and
doctest suites, 214 release library tests, and 227 actual WASI GC tests. Classloader
suites pass 758 tests (14 ignored); the instrumented VM library passes 985 (three
ignored). Native GC/reference and WASI VM fixtures pass four and two tests.
ThreadSanitizer passes the 36 forced schedules plus 25 existing concurrent,
generation and construction tests. Miri passes the portable rollback test's six
configurations. Native/WASI Clippy, bare-Wasm compilation, formatting and diff checks
pass. No new atomic state machine or unsafe code was introduced.

The mechanism runner now supports a distinct WASI lane, freezes Wasmtime and its
version, preserves the harness source, and rejects mixed targets. The 73 Python
harness tests pass, including retained simulated subprocess failures; real before/
after WASI allocation smoke checks also pass. Those smoke checks overlapped
correctness jobs and are excluded from timing evidence.

The frozen screen uses seven randomized process pairs, CPU 0, 100,000 operations,
four workloads, five policies and diagnostics off/on on each target: all 1,120
processes pass identity, work, checksum, teardown and unchanged-input checks.
No build, test or profiler overlaps these timings. Native baseline/candidate
allocation operation ratios are 1.013 [1.006, 1.031] and 1.028 [1.017, 1.037]; WASI
candidate allocation and owning-allocation ratios are 1.015 [1.004, 1.022] and
1.033 [1.009, 1.051]. Roots and collection-inclusive wall costs are mixed. These
exploratory unadjusted intervals describe net paths, not isolated causal costs.
Post-measurement disassembly confirms preparation is already inlined; it does not
identify the source of the remaining allocation regression. See the
[complete comparison and graphs](../benchmarks/gc/IMPLEMENTATION.md#allocation-registration-releases-admission-before-index-waits).

The [verification record](../target/gc-implementation/registration-prepare-verification.json)
binds all builds, source snapshots, commands, failures, raw measurements and figures.
The WASI baseline was built from an isolated reconstruction of the frozen source;
all 2,019 source files were verified before freezing its module. The change remains
for its demonstrated progress with the allocation cost unresolved. Map-node
allocation and preemption after successful admission can still block closure;
inaccessible object shards and opaque graph callbacks remain D/E limitations.
The single-mutator/cooperative screens impose no physical-memory cap, exclude
process/runtime startup, and measure no STW or scheduled-arrival latency. Defaults,
compiled-coverage requirements and all external/endurance gates remain unchanged.

## Mutator buffer insertion and overflow under contention (2026-11-09)

Ordinary buffered barriers now try the context buffer and, when registering its
first record, the ready registry before changing either the buffer or pending
count. A busy lock leaves ownership unchanged and uses direct publication.
Overflow prepares the required metadata/remembered indexes before claiming each
record, stops at contention, retries insertion once, then falls back directly.
Pending reservations cover publication throughout. Explicit `ExecutionContext::flush()`
retains its synchronous contract. No new atomic protocol or persistent cache was
introduced, and no defaults changed.

Three native tests exercise 40 forced configurations: 24 root-deletion cases across
both publication/phase policies, all three root policies and held buffer/ready
locks; eight old-to-young notification cases; and eight overflow cases with held
object/remembered shards. The empty-lock cases finish collection while the lock
remains held, preserve the required graph and reclaim unrelated garbage. Overflow
leaves the original record and reservation intact, returns the mutator through an
unrelated target, then drains and validates both graphs after release. The original
root-deletion test fails against the frozen old implementation. Its two-second
timeout detects a blocked schedule; it is not a latency acceptance threshold.

The final native GC suites pass 432 tests including models and doctests; the release
library passes 212. Actual WASI execution passes 226 GC tests. Classloader suites
pass 758 tests (14 ignored), the instrumented VM library passes 985 (three ignored),
and native GC/reference and WASI VM fixtures pass four and two respectively.
ThreadSanitizer passes the three new tests and one existing mutator test. Miri
passes all 13 context/coalescing tests, including transactional busy insertion;
the initial incorrect filter ran zero tests and is retained without counting as
coverage. Native/WASI Clippy, bare-Wasm compilation, formatting and diff checks
pass. An initial lint failure from unnecessary test annotations is also retained.

Frozen before/after binaries completed 784 measurement processes, sequentially on
CPU 0 with seven randomized pairs per condition and no overlapping builds/tests.
The 560 native mechanism processes cover control, roots, 64-target barriers and
managed overwrites under five policies with diagnostics off/on. The 112 native and
112 WASI graph processes each use 4,097 nodes, 32 measured full cycles and zero
workers, with wide/barrier shapes and both publication policies. All checksums,
work counts, teardown checks and frozen-input checks pass. The first launcher
failed only at plotting after all mechanism measurements completed; the graphs
were recovered from those saved summaries using the existing plotting environment.
No measurement was discarded or repeated for that failure.

The overflow-heavy context/local policies cost about 9% more operation time and
7% more total time per operation with diagnostics off. The integrated candidate
policy costs about 1% more on that workload; root and overwrite intervals include
no change. Native wide-graph collection time falls about 4%; most WASI intervals
include no change. These are net-path exploratory comparisons, not attribution to
a specific lock instruction, pause measurements or evidence of parity. The
previous preparation batch's native regression remains separate evidence; this
comparison does not rerun its earlier baseline. See the
[tables and graphs](../benchmarks/gc/IMPLEMENTATION.md#mutator-barrier-insertion-and-overflow-yield-on-contention).

The [verification record](../target/gc-implementation/mutator-buffer-verification.json)
binds sources, binaries, commands, raw failures and measurements. This change is
retained for the demonstrated contention progress, with its overflow cost explicit.
Inaccessible nonempty records still exclude closure. Required direct publication,
host allocation, explicit flush and other cleanup can still wait, and stalled
admitted publishers or graph callbacks remain D/E blockers. The screens impose no
physical-memory ceiling and establish no whole-JVM, STW, sustainable-capacity or
endurance gate. Memory bounds under sustained contention remain unqualified.

## Field continuations and shared checked resolution (2026-11-09)

Experimental `getstatic`, `putstatic`, `getfield` and `putfield` return owning
requests before resolution or class initialization can await. Native frames are
absent during suspension; locals, residual operands, receiver/value identities and
primitive bits remain in owning state. The runtime context retains roots until
completion or cancellation. Field access itself uses the existing managed guard,
without carrying that guard across an await. Synchronous embedding shares the same
semantics through its existing bridge.

The former JIT name-only field path now shares the interpreter's checked resolver
and storage: descriptor matching, access, static/instance kind, final writes,
receiver restrictions and primitive narrowing. Static operations initialize the
actual declaring class. Generated null checks follow resolution; cached Java field
linkage failures preserve their type on repeated accesses. Other host-resolution
failures retain their existing formatted-error representation.

The bounded subprocess covers eight combinations of gated/counted publication,
host/compact storage and zero/one registered worker. Eighteen methods check all
field types through interpreted, resumable compiled and synchronous execution,
including narrow integers, surrogate code units, floating NaN bits/signed zero and
reference/null values. Additional generated fixtures verify inherited statics,
descriptor/kind/access/final-write errors and their order before null checks.
Get/put initialization is completed or cancelled after migration: waiters wake,
partial effects are not replayed, live roots survive and unrelated garbage is
reclaimed. Cancellation releases the pending value; completed stores preserve it.

The first JIT suite was explicitly terminated after a 1,024-field spill fixture
expanded thousands of locals into excessive continuation code. Compilation now
checks the product of resume sites and declared typed local/operand capacity,
rejecting more than 65,536 transfer cells before emitting saves/restores. A new
regression covers rejection. The spill fixture now uses opaque long-array reads,
retaining its full result/counter and native-frame-size checks independently of
continuations. This limit is not a wall-time bound.

The full debug and release VM suites pass 985 tests with three ignored. The
instrumented JIT passes 232 library tests, 249 integration tests and one doctest in
both builds; one library benchmark is ignored. The profiling-disabled field matrix
also passes, with ordinary JIT suites passing 231/245/1 and the same ignored library
benchmark. The exact exported classes pass 52 HotSpot checks under both
`-Xverify:all -Xint` and `-Xverify:all -Xcomp`. Failed setup/lint attempts and the
initial full-VM diagnostic-count failure are retained: field publication adds one
resume to the existing suspended-reference fixture, whose expected count is fixed.
The old test-only narrowing helper was removed; actual typed field tests replace
its isolated assertions.

The existing portable constructor/initialization fixture passes in actual WASI
execution under both publication policies and storage modes. Thirty-one WASI
JIT/fallback tests, bare-Wasm compilation and native/WASI Clippy pass. These portable
checks exercise shared interpreter code or fallback behavior, not native generated
fields. The interpreter's existing cache-hit return was restored after the broad
debug run and during the release build, avoiding an added weak-thread upgrade;
the verification record distinguishes its focused final checks from those broad runs.

Commands, final suite counts, selected source snapshots and raw failures are bound
in the [verification record](../target/gc-implementation/field-continuation-verification.json).
Production reference eligibility and GC defaults remain unchanged. Field transfer,
checked resolution/cache and guard costs remain unmeasured. Remaining helper/native
suspension, full Java/relocation coverage, production hot paths, collector D/E,
pause telemetry and short/capacity/hour/day qualification remain open. These checks
establish no STW, throughput, latency or endurance gate.

## Allocation continuations and bounded ready requests (2026-11-09)

Experimental `new`, `newarray`, `anewarray` and `multianewarray` now return owning
allocation requests before class resolution/initialization, direct construction or
publication can await. Locals, remaining operands and dimension bits are saved;
the runtime context retains their reference roots. Result/exception ownership is
installed before completion bits, and resume joins after the allocation without
replaying earlier effects. Synchronous embedding delegates to the same allocator
through its existing runtime bridge. Native and owning request buffers now use the
method's actual maximum arity instead of a fixed 255-cell native buffer.

Runtime requests consume local fuel. The VM carries the remainder across ready
allocation and monitor requests, yielding at exhaustion; ordinary calls already
yield before dispatch. This fixes a gap in the preceding monitor implementation:
refreshing fuel after each immediately ready request could monopolize one poll.
The multidimensional builder also shares a 64-node budget across recursion depths,
including empty children that do not trigger byte-based initialization yields.
No managed guard crosses the recursive await, and pending parents remain rooted.

The bounded native matrix covers gated/counted publication, host/compact storage
and zero/one registered worker. It checks all eight primitive array types, zeroes,
negative sizes, object/reference/multidimensional results, typed catches and weak
root lifetime. Large construction suspends at its allocation BCI, migrates to
another OS thread and releases cancelled logical/storage reservations. Ready-only
allocation and monitor loops yield and cancel. Workerless pressure checks retain a
residual array while recovery performs one bounded assist per poll; cancellation
releases it and resumed allocation preserves exactly-once Java effects. Synchronous
recovery/full-collection counters do not increase in the checked allocation interval.

Component arrays do not initialize their class. `new` does enter an initializer,
and cancellation marks it failed, releases a waiting thread and makes a subsequent
`new` report `NoClassDefFoundError` without repeating the initializer's increment.
These lifetime checks do not establish full linking/access/initialization semantics.

The portable interpreter fixture checks 1 MiB byte initialization, 512 empty child
arrays and a three-level graph with empty leaves. After priming class resolution,
successive polls must yield in construction; cancellation and collection release
storage, and a later invocation completes with correct contents. Both publication
policies and storage modes are covered without a GC worker.

The full instrumented VM suite passes 985 tests with three ignored fixture tests
in debug and release. Instrumented JIT suites pass 231 library tests, 249 integration
tests and one doctest in both builds; the library benchmark remains ignored.
The portable constructor fixture passes in native and actual WASI execution across
all four configurations. Thirty-one WASI JIT/fallback tests, bare-Wasm VM compilation
and native/WASI Clippy also pass. Wasm evidence does not execute native JIT code.
The expanded allocation matrix also passes without execution profiling; ordinary
JIT suites pass 230/245/1, with the existing library benchmark ignored. Formatting
and whitespace checks pass.

The exact exported allocation fixture passes 79 HotSpot checks under both
`-Xverify:all -Xint` and `-Xverify:all -Xcomp`. Failed setup/compilation/lint attempts
are preserved. The initial pressure assertion incorrectly required a persistent
waiter in workerless mode; that path unregisters before its bounded assist and
self-wakes. The corrected check requires exactly one assist and no synchronous full
collection, rather than treating a zero waiter count as evidence of recovery.

Validation commands and final suite counts are recorded with source/fixture hashes
in the [verification record](../target/gc-implementation/allocation-continuation-verification.json).
Production reference/call eligibility and collector defaults remain unchanged.
Other synchronous helpers, sentinel/context setup, native/reentrant cancellation,
full semantic/relocation coverage, measured continuation/root costs and compiled
hot-path evidence remain open. No STW, performance, capacity or endurance gate is
established by this batch; stalled admission/callback progress still needs D/E.

## Explicit monitor semantics and suspension (2026-11-09)

The experimental JIT no longer discards `monitorenter` and `monitorexit` operands.
It emits owning monitor requests, returns its native frame, and resumes directly
after the operation without replay. The VM uses the interpreter's shared monitor
entry/exit operations. Null and ownership errors are reported at the originating
BCI and enter the existing pending-exception/handler path. The synchronous
embedding API executes the same operations through runtime helpers.

A logical-thread ledger now owns explicit monitor entries and their object roots.
Host-execution boundaries remember acquisition serials; cancellation releases
outstanding entries since the boundary in reverse order. An inner cancelled scope
cannot consume older outer entries. Root cleanup and contender wakeup occur outside
the ledger lock. Waiting acquisitions restore runnable thread status on cancellation.
Normal Java completion retains the existing non-structured exit policy and relies
on generated cleanup handlers; this change does not enforce structured locking.

A bounded 120-second subprocess covers eight configurations: interpreted/generated
execution, gated/counted publication, and zero/one registered GC worker. It checks
actual in-body ownership with `Thread.holdsLock`, reentrancy, competing-owner
exclusion, null/ownership errors, catch-all cleanup, OS-thread migration, cancellation
while holding/waiting, runnable-state restoration, and subsequent thread reuse.
Four additional direct-activation configurations drop all Java argument/context
owners after publishing an empty frame snapshot: the acquired monitor alone retains
its object until cancellation. They check nested cancellation, residual reference
operands across monitor requests, return and eventual reclamation. Held application
monitors do not retain separately allocated unrelated garbage in these fixtures.

The native-stack fixture now deliberately invokes the synchronous embedding API;
monitor opcodes no longer serve as an incidental continuation exclusion. Its
wrappers call `Thread.holdsLock` inside the protected region before recursion.
Caught and uncaught native overflow release the argument and recursive method
monitors. **Correction:** the previous handler batch's argument-monitor check
proved only a free final state because explicit opcodes were no-ops. That evidence
never proved acquisition. Its real synchronized-method guard and native-capacity
checks remain valid; the historical snapshot/report is preserved unchanged.

The full instrumented VM suite passes 984 tests with three ignored fixture tests
in debug and release. Instrumented JIT suites pass 231 library tests, 249
integration tests and one doctest in both builds; ordinary JIT passes 230/245/1.
The existing JIT library benchmark remains ignored. The expanded focused monitor
matrix also passes without execution profiling. A new portable VM integration test
executes monitor contention/cancellation, exception cleanup, unrelated reclamation
and reuse in both native and actual WASI execution under both publication policies.
Thirty-one portable WASI JIT/fallback tests, bare-Wasm VM compilation, native/WASI
Clippy and formatting checks pass. Wasm evidence does not execute native JIT code.

The exact exported monitor fixture passes 14 HotSpot checks; the expanded stack
oracle passes 32, now including both explicit wrappers. Both pass `-Xverify:all`
with `-Xint` and `-Xcomp`. An earlier 20-check stack run is retained with its narrower
scope. Initial test compilation/lint failures are preserved. No previous virtual-
call HotSpot discrepancy is reclassified. The
[verification record](../target/gc-implementation/monitor-continuation-verification.json)
contains commands, logs, source/fixture hashes and selected before/after snapshots.

Production reference/call/monitor eligibility and collector defaults remain
unchanged. Wait/notify cancellation, arbitrary unbalanced bytecode, native monitor
reentry, the complete Java memory-model matrix, allocation/helper suspension and
relocation still need qualification. Ledger allocation/retained capacity and
continuation overhead are unmeasured. These correctness results establish no STW,
latency, sustainable-capacity, G1/ZGC performance or endurance pass; J0–J5 and the
collector publication/callback progress gates remain open.

## Handler continuations and exception-only loops (2026-11-09)

Experimental exception-table methods now use owning continuations at calls and
selected handler entries. The compiler gives exception dispatch a separate target
map and charges each selected handler against the local budget. This covers cycles
formed entirely by exception edges, including methods with no explicit backward
branch. Before returning, generated code publishes the handler's live reference
locals and single throwable operand. Resume restores that state without repeating
exception matching or taking the pending owner a second time.

Eight direct generated-code configurations cover both publication policies,
zero/one registered worker and completion/cancellation. A handler-only live local
and its throwable survive collection between slices. Typed matching skips a
nonmatching entry; successive iterations release obsolete throwables. Migration
preserves the continuation, returning transfers the array owner and cancellation
releases both roots. Three handler suspensions produce three native resumes and
zero explicit-backedge counts, as required by that diagnostic counter's contract.

Sixteen live-VM configurations catch an exception from an interpreted/compiled
callee, then suspend before or during a second call inside the handler. After
clearing the observation fields, the local array and caught throwable survive
collection while unrelated garbage is reclaimed. Migration and cancellation release
roots, nested frames and reentrant synchronized-method ownership; the logical
thread remains usable. Finite calls return the exact caught exception. Both tests
run inside externally bounded 90-second subprocesses.

The native-capacity fixture now uses a balanced explicit monitor region, with
normal and catch-all release paths, to retain a synchronous generated boundary.
Handler presence alone no longer selects that boundary. Its checks require both
the recursive class monitor and wrapper argument monitor to be reusable after
caught/uncaught overflow. The separate Java-budget matrix continues to exercise
cooperative recursion.

Correction from the subsequent monitor audit: the JIT's explicit monitor opcodes
were no-ops in that snapshot. The argument-monitor check above proved only that
it was free after execution, not that the wrapper acquired it. The actual
synchronized-method guard and native stack-bound checks remain valid. The monitor
follow-up replaces the no-ops, checks `Thread.holdsLock` inside the wrapper,
and explicitly invokes the synchronous embedding API for native-stack testing.
The historical verification record is preserved, not reclassified.

Initial test failures are retained: missing fixture imports/mirror conversion,
and an undersized fixture operand stack that omitted its handler path. The latter
now uses the existing exception-aware maximum-stack analysis. No runtime stack
check was loosened. Production reference/call/handler eligibility remains closed;
explicit monitor instructions and other helpers retain synchronous execution.
These correctness checks do not establish mid-helper/native cancellation, complete
relocation safety, measured overhead, pause bounds or sustained-load qualification.

The full instrumented VM suite passes 983 tests with two existing ignores in
debug and release. Instrumented JIT suites pass 231 library tests, 249 integration
tests and one doctest in both builds; ordinary JIT tests pass 230/245/1. Each native
JIT library suite retains its existing ignored benchmark. The two new tests also
pass without execution profiling. The stack export passes six tests plus its
explicitly invoked ignored subprocess fixture. Thirty-one WASI JIT/fallback tests,
bare-Wasm VM compilation, native/WASI Clippy and formatting checks pass.

The exact exported handler and revised stack fixtures pass 6 and 20 HotSpot checks,
respectively, under both `-Xverify:all -Xint` and `-Xverify:all -Xcomp`. This does not
resolve or reclassify the previously recorded tiered virtual-call discrepancy.
Generated-code evidence remains Linux x86-64; Wasm checks cover portable/fallback
behavior. Commands, raw logs, initial failures and source/fixture hashes are in the
[verification record](../target/gc-implementation/handler-continuation-verification.json).

## Call continuations and initialization cancellation (2026-11-09)

Experimental methods without exception tables or explicit monitor opcodes now
return a pending request at static, special, virtual and interface invokes. The
activation owns argument cells, typed locals and residual operands; its runtime
context retains the caller roots while normal VM dispatch resolves and awaits the
callee. Resumption checks the pending exception and consumes the returned value
without replay. The synchronous embedding API retains helper dispatch, and the
production call/reference gate remains closed.

The loop fixture manually stops at successive calls, forces collection and checks
local/operand state and exactly-once side effects. Additional runs use 0, 1 and
10,000 iterations, matching the maintained Java oracle. Sixteen bounded lifecycle
configurations exercise both publication policies, zero/one registered worker,
interpreted/compiled callees and cancellation before/after callee entry. The caller's
reference survives with only the published frame snapshot retaining it; unrelated
garbage is reclaimed. Migration and cancellation release roots, Java frames and
reentrant method-monitor ownership, and the same logical thread remains usable.

Four initializer-cancellation configurations cover both policies and interpreted/
compiled callers. A partially executed initializer remains claimed during same-thread
reentry and after a waiting attempt is cancelled. Cancelling its owner marks the
class failed, releases the waiting attempt and makes retries fail without repeating
the first side effect. This fixes a prior ownership gap in async initialization.

The full instrumented VM suite passes 981 tests with two existing ignores. Its
first run exposed a stale native-recursion fixture: handler-free direct calls now
release native frames, so the old test recursed toward its much larger Java slot
limit and hit the subprocess deadline. The corrected fixture explicitly keeps a
synchronous generated wrapper; the independent Java-budget matrix continues to
exercise cooperative calls. The native matrix passes on 1/2 MiB Linux OS stacks
with both publication policies and three mixed modes, preserving the native
overflow message, root/throwable lifetime and monitor cleanup. A separate bounded
test polls 2,049 nested future owners, migrates them to a smaller OS stack and
drops every owner; it also rejects low capacity beneath a synchronous caller.

These checks do not qualify call overhead, production hot-path coverage, handler
or mid-helper cancellation, complete relocation/native-reentry behavior, pauses or
sustained capacity. Collector defaults remain unchanged.

The six maintained HotSpot oracles pass all 150 checks with `-Xverify:all -Xint`.
Five also pass their 110 checks under `-Xcomp`, including the eight new call-loop
checks. The virtual-call oracle fails its nineteenth check under tiered `-Xcomp`:
an `invokevirtual` resolving to a static method with a null receiver throws
`NullPointerException`. A separate diagnostic reproduces this with C1-only
compilation; interpreter and C2-only execution instead throw
`IncompatibleClassChangeError`. The complete virtual oracle passes all 40 checks
with `-Xcomp -XX:-TieredCompilation`. Ristretto retains its expected linking-error
precedence, consistent with [JVMS 6.5 invokevirtual](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.invokevirtual).
The failed tiered run is retained as a cross-runtime discrepancy, not reclassified
as passing or removed from the oracle. No all-mode HotSpot agreement is claimed.

Instrumented JIT suites pass 231 library tests, 249 integration tests and one
doctest in debug and release; the ordinary build passes 230/245/1. Each native JIT
library suite retains its existing ignored benchmark. All 36 VM call tests pass
in ordinary debug and instrumented debug/release builds. Six Java stack tests
(plus the explicitly invoked ignored child) and four native stack tests pass in
release. Thirty-one WASI JIT/fallback tests, bare-Wasm VM compilation, native/WASI
Clippy and formatting checks pass. Generated-code evidence remains Linux x86-64;
portable checks do not execute native JIT code or establish a native Miri result.
Commands, source/fixture hashes, the selected source snapshot, initial failures
and the HotSpot discrepancy are preserved in the
[verification record](../target/gc-implementation/call-continuation-verification.json).

## Reference continuations and suspended roots (2026-11-09)

Experimental reference/allocation loops now retain one owning runtime context
across native slices. Generated code publishes the destination's live roots before
returning; the borrowed context address is refreshed on each entry and cleared
before suspension. Production reference/call eligibility and collector defaults
remain unchanged.

Real generated-code tests force collection between slices with a newly allocated
reference held only in a local or operand. Eight publication-policy/worker/location
configurations preserve that reference across migration and transfer its owner on
return, while discarded allocation results are released. A backward reference
condition consumes the only live reference; it becomes reclaimable before the
next resume, despite stale identities remaining in unused continuation cells.
A helper failure after resumption transfers the pending throwable past context
destruction and releases it after the returned owner is dropped.

A separate subprocess, bounded externally to 60 seconds, exercises direct and
interpreted callers, both publication policies and zero/one worker. It clears the
only static observation of a compiled allocation, then proves that the suspended
operand survives, unrelated garbage is reclaimed, and the future can migrate.
Cancellation releases that root, Java frames and synchronized-method monitor; the
logical thread executes another method afterward. The test explicitly installs
experimental code in the cache and is not production-admission evidence.

The independent Java oracle passes 12 results from the four finite exported
classes under both HotSpot `-Xverify:all -Xint` and `-Xverify:all -Xcomp`. The
infinite lifecycle class is exercised only by the bounded Rust subprocess.
The full instrumented VM suite passes 977 tests with two existing ignores.
Instrumented JIT suites pass 230 library tests, 249 integration tests and one
doctest in debug and release; the ordinary build passes 229/245/1. Each JIT library
suite retains its existing ignored benchmark. Thirty WASI JIT/fallback tests,
bare-Wasm VM compilation, native/WASI Clippy and formatting checks pass.
The four new VM tests pass in ordinary debug and instrumented debug/release
builds. Three optimized allocation-pressure tests also pass, including live local,
live operand and handler-only root retention during reclamation.
Check results, initial failures and source/fixture hashes are recorded in the
[verification record](../target/gc-implementation/reference-continuation-verification.json).
These correctness checks do not establish performance, pause bounds, complete
helper/call cancellation, relocation safety or sustained-load qualification.


## Resumable production primitive loops (2026-11-08)

Native static primitive loops now use an owning continuation through the normal
compiler and dispatcher. Positive backward-edge budgets bound execution slices;
typed locals and remaining operands survive suspension. Resume restores the next
BCI without replay, checks native stack capacity, and may run on another OS thread.
The VM publishes the suspended bytecode offset before yielding. Forward-only code
needs no continuation allocation. Reference/call bodies retain their restrictions.

Eight generated-code tests cover BCI-zero reentry, nested loops with several fuel
budgets, explicit/default switch destinations, wide operands, NaN payloads/signed
zero, slot reuse, independent activations, migration/code ownership and a droppable
self-loop. Two Rust-ABI tests cover argument rejection, single completion and
native stack rejection without consuming continuation state; the portable one
also runs in WASI. Instrumented tests keep invocation counts separate from native
resumes and count each taken edge once.

The VM's bounded child exercises eight lifecycle configurations: gated/counted
publication, zero/one registered worker, and direct/interpreted callers. Each
keeps an unused reference argument alive while suspended, reclaims unrelated
garbage, migrates/resumes, reports the suspended BCI, then releases the argument,
Java frames and synchronized-method monitor on cancellation. The Java thread is
reused afterward. An additional one-worker Tokio case aborts an infinite compiled
loop while another task progresses. The child has a 60-second external deadline.
There is no test cache insertion for these production methods.

HotSpot `-Xverify:all` agrees on 388 cases from the 26 exported numeric fixture
classes under both `-Xint` and `-Xcomp`. The two additional lifecycle classes are
tested by the bounded Rust child. Profile/checkpoint packets now expose
`compiled_resumes`; Python validation permits a resume without a new invocation
inside an interval, rejects corrupt counters and preserves missing historical
fields. No native or Wasm performance/qualification fork is claimed for this batch.

Initial API spelling and test lint failures are preserved. The full VM library
suite passes 973 tests with two existing ignores. Instrumented JIT suites pass
230 library tests, 249 integration tests and one doctest in both debug and release;
the ordinary build passes 229/245/1. Each library suite retains its existing
ignored benchmark. All seven production-control-flow and lifecycle VM tests pass
in ordinary debug and instrumented debug/release builds. Three optimized
allocation-pressure tests, 30 WASI JIT/fallback tests and 71 Python harness tests
also pass. Native/WASI Clippy, bare-Wasm VM compilation and formatting checks pass.
Final-source hashes, logs and fixture details are in the
[verification record](../target/gc-implementation/primitive-continuation-verification.json).
Reference-bearing continuations, arbitrary native/helper cancellation, cooperation
overhead, required benchmark hot-path compilation, complete pause telemetry and
sustained-load qualification remain open. Native generated-code evidence is from
Linux x86-64; there is no generated-code Miri or all-host qualification claim.

## Production forward primitive control flow (2026-11-08)

The native production gate admits static primitive methods with forward integer
conditions, narrow/wide jumps and table/lookup switches. Every explicit destination
and default must advance. No instruction can repeat within such a method, and no
Java calls are admitted. The existing primitive ABI needs no frame-owned reference
handoff for this subset. This is an instruction bound, not a wall-time bound or a
pause result. Backward/self edges, reference operations, calls and handlers retain
their restrictions; native loop cooperation/cancellation remains unfinished.

Six live-VM tests cover 25 methods compiled through the normal compiler/cache and
dispatcher, plus one finite loop that remains interpreted. No test injects a
compiled function into the cache. Instrumented checks assert exact compiled and
interpreted entry counts and zero compiled backedges. There are 388 cases covering
all twelve integer conditions, both jump widths, extrema at both ends of signed
switch ranges, duplicate/empty lookup targets, wide operand/local merges, slot
reuse, long extrema, NaNs and signed zero. HotSpot with `-Xverify:all` agrees on
the exact exported class files in both `-Xint` and `-Xcomp` modes.

The original four production fixtures failed at the old admission gate. An initial
profile-field typo and test-only lint failures are also preserved. The complete
instrumented VM library suite passes 972 tests with two existing ignores.
The instrumented JIT suites pass 228 library checks, 241 integration checks and
one doctest in both debug and release; the ordinary JIT build passes 227/237/1.
Each JIT library suite retains its existing ignored benchmark. All six new VM
tests pass in ordinary debug and instrumented debug/release builds. Three
optimized allocation-pressure tests also pass, preserving live locals, operands
and handler-only references while reclamation advances. Native Clippy, bare-Wasm
VM compilation and 29 WASI JIT/fallback tests pass. Native generated-code evidence
is from Linux x86-64; this is not qualification of every supported host.
No timing or shared-workload qualification run is attributed to this change.
The [verification record](../target/gc-implementation/forward-primitive-verification.json)
records source, logs, class/oracle hashes and final checks. The
[implementation report](../benchmarks/gc/IMPLEMENTATION.md#production-primitive-control-flow)
keeps the broader J2/J3/J5 and endurance gates open.

## Taken compiled backedges and diagnostic intervals (2026-11-08)

Native `execution-profile` builds count selected explicit branches to the same or
an earlier Java bytecode offset. Conditional fallthrough and forward switch cases
do not count. A function owns the counter allocation for the lifetime of its code
and clones. Real generated-code tests cover loops returning to BCI zero, conditional
and wide/unconditional edges, duplicate switch destinations, preserved operands,
clone lifetime and four concurrent callers producing exactly 20,000 backedges.
No production eligibility or collector default changes.

`VM::execution_profile_checkpoint` and `execution_profile_since` provide opaque,
VM-specific counter boundaries. Tests execute warmed and newly compiled methods
through the real dispatcher, proving that prior calls are excluded, lifetime
totals remain intact and code compiled during the interval is not recorded as
present at its start. Independent checkpoints and abandoned/foreign ownership are
tested. These are per-method snapshot cuts, not atomic VM-wide phase boundaries.
Native boundary costs are explicit; Wasm has no clock calls in this API and emits
null timings. Recorded code presence is not a production-readiness assertion.

The first full VM run caught five Cranelift verifier failures in allocation/root
fixtures: forwarding blocks copied implicit SSA local parameters as Java operands.
The fix captures explicit operand types before SSA construction and forwards only
those operands, leaving Cranelift to carry locals. The focused failure and complete
failed run are retained. The corrected full suite passes 966 tests with two ignored;
the optimized local-root, operand-root and handler-only-root pressure tests all pass.
An initial missing import and pointer-cast lint diagnostic are also retained.

Instrumented debug and release JIT suites each pass 227 library checks, 241
integration checks and one doctest; the ordinary build passes 226/237/1. Each
library suite retains its existing ignored benchmark. Two portable checkpoint
tests execute successfully in WASI. Bare-Wasm VM compilation, native/WASI Clippy,
formatting/diff checks, Python compilation and all 70 Python harness tests pass.
Miri cannot execute the generated code used by these backedge tests; no such
generated-code Miri claim is made.

Fresh frozen native/WASI diagnostic builds run six/three shared-workload forks
sequentially, without concurrent builds, tests, edits or other profiling runs.
Each uses CPUs 0/2, two cores of total quota, a 64 MiB logical heap, a 512 MiB
cgroup ceiling, one-second warmup and one-second timed work. The diagnostic
measurements are excluded from timing comparisons. All forks pass, inputs remain
unchanged and Java class hashes match the preceding shared-workload snapshot.
SVG/PNG invocation-coverage graphs are retained alongside full lifetime and
main-invocation profiles. Native JIT lifetime compiled entries are 4,522/4,508/4,522
for allocation/roots/mixed, while every main-invocation interval has zero compiled
entries and backedges. The two compiled lifetime methods are `Byte.toUnsignedInt`
and `WeakHashMap.indexFor`. The hot application methods remain interpreted.
WASI interval timings are null and all compiled counters are zero.

See [results and graphs](../benchmarks/gc/IMPLEMENTATION.md#taken-backedges-and-scoped-execution-diagnostics)
and the [verification record](../target/gc-implementation/execution-windows-verification.json).
Exact measurement-phase capture, complete pause recording, safe production hot-path
compilation, recorder overhead, capacity, contention progress and hour/day
qualification remain open. Lifetime subtraction alone cannot distinguish
pre-invocation from post-invocation background activity.

## Versioned short-run qualification checker (2026-11-08)

The current plan accepts observed native STW pauses strictly below 1 ms, including
rendezvous and equivalent heap-wide stalls. It does not require superiority or
fixed percentage improvements over G1/ZGC. The new v2 checker preserves peer ratios
and confidence intervals as diagnostics while requiring frozen absolute workload
limits, equal enforced resource ceilings, a matching coordination audit, complete
pause capture and measured-window compiled execution/GC exposure. It rejects
exactly 1 ms and longer stops, merging overlapping/touching intervals first.
Empty complete streams have no invented percentiles; missing streams cannot pass.

The runner freezes the policy, native build/source evidence, process identities
and output hashes. Evaluation rechecks artifacts and captured process packets,
including runtime mode and typed JSON equality. The historical evaluator remains
available as v1. New reports have versioned filenames, evaluator/input hashes and
exclusive creation; neither mode overwrites prior reports. HotSpot phase durations
are now labeled `gc_phase_pause_ms`, with `global_pause_ms` null because phase
summaries alone omit rendezvous and full event coverage.

All 68 Python harness tests pass, including missing/incomplete evidence, invalid
timestamps, the exclusive pause boundary, overlapping/adjacent stops, coverage and
readiness failures, audit/build mismatch, resource/backlog/latency limits, failed
or missing pairs, typed output binding, resolution-limited ratios and historical
report preservation. The added cases use synthetic fixtures, not measurements.
One new fixture initially set response time below service time; the existing
validator correctly rejected it. The corrected zero-resolution fixture sets both
to zero and verifies unavailable ratios without claiming an improvement.
Python compilation and diff checks pass; no production Rust changes or new
runtime performance measurements were made for this batch.

Replaying four frozen native comparison datasets reproduces every original v1
gate field exactly. Original gate reports, tables and graphs remain unchanged.
V2 rejects all four because the policy and required runtime evidence were not
predeclared/captured. No earlier run is reclassified as passing. The
[verification record](../target/gc-implementation/qualification-v2-verification.json)
preserves hashes, checker results and historical replays.

J5 remains open. The [evidence contract](../benchmarks/gc/QUALIFICATION.md) does not
supply runtime recorders, expand production JIT eligibility, validate audit claims,
establish warmup/compilation stability or qualify recorder overhead. Real complete
pause/measurement-window coverage and GC packets remain missing, and known global
blocking remains unresolved. Capacity, memory/backlog histories, forced-stall
progress, native/Wasm qualification and hour/day trials still need completion.

## Required barrier indexes prepared before claim (2026-11-08)

Collector drains now peek without transferring ownership, try required indexes,
and pop only a matching address/epoch/kind. Contention leaves the record, ready
owner and pending reservation intact. Dirty preparation holds the remembered
shard before observing young activity, so a new young scan cannot pass a delayed
insertion. Prepared metadata remains owned through acknowledgement; its final
release occurs outside index and collector coordination guards. Explicit mutator
flush/overflow is unchanged. Pending records still prevent phase closure.

The new regression fails on the frozen preceding implementation: a collector step
blocks on metadata, and releasing a separately retired object's storage lease
cannot make that worker reclaim it before the metadata holder is released. All
eight candidate configurations (publication policy, phase policy, object/remembered
index) yield and reclaim that object while pending remains one and the full ticket
does not advance. Sixteen earlier application configurations now leave metadata-
blocked records buffered; both ordinary-spare and FIFO queue routes still complete
collection and reclaim unrelated garbage before releasing an empty injector.

Four forced schedules pause a dirty preparation after observing young inactivity,
start a young cycle, and keep its remembered scan at the held shard. Resuming the
insertion retains the child without full tracing independently marking it. The
fixture removes the earlier promotion/idle remembered entry to require this
notification. Two portable tests revalidate changed address/epochs/kind and account
separately for an identical replacement after another consumer claims the original.
All twelve context/coalescing tests pass Miri.

Checks pass 432 native GC tests across 39 binaries, 224 WASI tests across 31, and
208 optimized library tests. Native/WASI Clippy, bare-Wasm examples, formatting,
diff checks and all 50 Python harness tests pass. The forced young fixture first
incorrectly expected idle mutation to buffer; it now starts full collection before
the explicit dirty notification. That fixture failure, the long-benchmark lint
failure and the genuine blocking baseline are retained. An earlier extra-dereference
compiler error is recorded in the tool transcript.

The `mark_cycles` barrier shape includes publication and all full-collection phases:
one distinct required edge per leaf per cycle, 32 records per context, zero workers,
one-unit collector steps, exact buffered/flushed counts when diagnostics is enabled,
and no overflow. It retains checksum, marked/completed totals and zero-byte teardown
assertions. The paired runner refuses missing or inconsistent barrier accounting.
Frozen before/after sources differ in collector/context code; fixtures, release
profiles and all 68 isolated dependency identities match. Forty-eight small native/
WASI smoke processes pass. Timings from these smoke checks are not evidence.

The first paired screen exposes native/WASI broad-control regressions. Keeping the
preparation helper out of line shrinks the native collector-step symbol from 31,222
to 28,674 bytes (original 27,825). Drain timing is also moved to include required
preparation, including failed attempts; acknowledged-record counts are unchanged.
The combined adjustment improves the WASI broad control against the first candidate,
but native comparison intervals include one. Sixteen further native/WASI smoke
processes pass for this candidate.

The selected final screen reports diagnostics-off native ratios 1.006/1.015 for
buffered edges and 1.031/1.052 for broad controls (gated/counted). WASI gives
1.007/1.012 and 0.973/1.005 respectively. Every interval except the final counted
WASI broad control excludes one. Required publication is slightly slower, and the
native broad-control regression remains unresolved. No acceptance claim follows.
See the [results and graphs](../benchmarks/gc/IMPLEMENTATION.md#required-barrier-preparation-yields-on-busy-indexes)
for all intervals, enabled-diagnostics cases, timing scope and restrictions.

A zero-pending shortcut inside the drain was also measured and then removed:
the caller already checks pending, so this was redundant. Its extra portable test
and Miri run passed but are not part of the selected implementation. All 896
measured processes across the initial, adjusted, final and rejected comparisons
validate; no measured run overlaps source edits, builds, tests or profiling.
The selected benchmark collector/context/fixture exactly matches final production
sources. Later test-only assertions require failed preparation timing and zero
acknowledged records while the index is held. Four native VM GC/reference and two
WASI VM checks also pass. The [verification record](../target/gc-implementation/barrier-prepare-verification.json)
preserves intermediate outcomes and identifies the selected candidate.

## Empty claims and alternate ordinary publication (2026-11-08)

Claims skip zero-count queues as a scheduling hint; only actual dequeues update the
count. Required publication tries each remaining bounded worker queue once after
its local/injector attempts fail, then retains the existing FIFO fallback. No lock
wait, capacity-bound relaxation or termination-protocol bypass is introduced.
The [protocol](PUBLICATION.md#contended-mark-publication) separates advisory hints
from closure requirements and records the remaining allocation limitations.

The held-empty-queue baseline reports two failed mutex probes; the new claim reports
none and still sees later publication. A second baseline puts an item in the FIFO
despite an available worker queue. The candidate publishes it to that queue,
preserves its continuation/version/young flag/budget, and respects a different
queue's full bound. Holding every ordinary queue still permits publication and
consumption through the FIFO. The new test also passes Miri.

Sixteen forced barrier-application configurations use one or three ordinary queues,
both publication policies and both phase policies, with zero background workers.
Eight metadata stalls keep pending=1 and prevent closure while unrelated steps
return. Eight held-empty-injector schedules complete a full ticket and reclaim
unrelated garbage before releasing the mutex. Four explicitly observe one pending
FIFO item; four observe one ordinary item and zero FIFO items before collection.
All preserve the deleted live graph through its snapshot and end at zero charged
bytes. Existing nonempty-contention and claim/reservation tests continue passing.

Two prior test fixtures failed under concurrent builds. One consumed the completion
message and then inferred a stall from the sender not yet being dropped; it now
uses the successful receive. The repeated-edge fixture could stop producers before
all had run after their start barrier. Each cycle now starts its snapshot, then
requires an acknowledged barrier from every producer before collector progress.
The cleanup/deadline assertions remain. Initial fixture compilation also corrected
a qualified helper call and added a scoped long-test lint expectation. All failure
logs are retained.

The final native GC suite passes 428 checks across 39 binaries; WASI passes 222
across 31; the optimized library passes 204. Four native VM GC/reference and two WASI
VM checks pass. Native/WASI Clippy, bare-Wasm examples, formatting and diff checks
pass. Test-only route observations added after benchmark builds do not change the
production code measured by those frozen binaries.

Separate diagnostic copies count fallback publication, empty locks and busy probes;
their elapsed times are not used for performance conclusions. Three uninstrumented
paired screens pass all 1,008 processes. The final comparison against the pre-FIFO
baseline improves native cooperative and broad multiworker cases, while native
multiworker trees remain inconclusive. Seven of eight final WASI intervals include
one. Reports retain all diagnostics-enabled cases, raw runs and unadjusted 95%
process-pair intervals. This does not prove parity in the tree case or qualify
application performance, zero-STW behavior, G1/ZGC results or sustained load.

[Empty-probe verification](../target/gc-implementation/queue-empty-verification.json) ·
[Final queue verification and graphs](../target/gc-implementation/queue-spare-verification.json)

## Repeated graph collection screen (2026-11-08)

`examples/mark_cycles.rs` constructs an eight-way tree or broad-root graph, checks
its checksum, warms three complete collections, and times 32 full collections of
8,191 nodes. It asserts exact completion and marked-object totals, retained graph
content, actual registered worker counts and zero managed bytes/objects after
teardown. Construction, checks and teardown are outside timing. All collection
phases and waits remain inside. Native smoke checks pass 16 debug configurations;
the two optimized revisions pass 48 combined native/WASI configurations. Native/
WASI Clippy, bare-Wasm check and formatting pass. All 49 Python harness checks pass,
including rejection of failed/incomplete/duplicate pairs and incorrect work totals.

Frozen builds differ only in `work.rs`, queue module wiring and the two new queue
files. The benchmark, profiles and dependency versions match. Initial isolated
`--locked` builds refused the reduced workspace; pruning both locks offline yields
the same 68 packages, with no identity absent from the original lock. Both builds
then use `--locked`. Initial example compilation corrected an Arc collector
parameter, error variant and unnecessary qualification; an unneeded lint expectation
was removed. The first WASI measurement command omitted Wasmtime and was rejected
before creating a run; the corrected command supplies and freezes the runner.
All intermediate logs are retained.

All 336 measured processes validate (224 native, 112 WASI); binaries and runner
remain unchanged. Seven process pairs per condition provide exploratory 95%
intervals without multiplicity adjustment. Native diagnostics-off cooperative
ratios span 0.918–0.962. Four-worker broad graphs improve in this screen, but the
gated tree regresses to 1.140 [1.050, 1.144]. The counted tree and all eight WASI
conditions are inconclusive. Raw results, reports, graphs and source hashes are
linked by the [verification record](../target/gc-implementation/mark-cycles-verification.json).

This is collection throughput for fixed live graphs, not application throughput,
GC pause latency or G1/ZGC qualification. Affinity is enforced; no physical-memory
ceiling or peak-memory measurement is supplied. Fallback frequency/allocation cost
is not isolated. The native tree regression remains an open performance issue;
no endurance or complete no-global-blocking claim follows from these runs.

## Contended mark publication (2026-11-08)

The saved baseline times out when a required publisher encounters the held injector.
Producers now try the ordinary queues and use an owning atomic FIFO on contention.
A linked node is consumable even when its producer has not advanced the tail.
Detached links become non-null tombstones, preventing delayed producers from
appending to abandoned nodes. Successful claims remove payload ownership promptly;
iterative destruction uses `Arc::into_inner` to bound native/Wasm stack use.

Six queue tests cover FIFO payload release, a producer stalled after linking,
a producer delayed before linking to a detached predecessor, a delayed head reader,
100,000-node destruction on a 128 KiB native stack, and four producers/four consumers
delivering 8,000 items exactly once. The delayed reader retains three nodes while
held and two after release. Integrated tests check lossless publication around a
held injector, alternating ordinary/fallback claims, continuation versions/offsets,
young-edge flags, budgets and zero retained entry capacity after draining.

Eight forced barrier-application configurations cover both publication and phase
policies with zero background workers. The four held-empty-injector cases now
complete a full collection and reclaim unrelated garbage before releasing the
lock. The four held-metadata cases still retain pending=1 and exclude closure while
unrelated collector steps return. The existing forty queue/phase configurations
continue checking nonempty contention, empty-queue closure and live graph retention.
Zero ordinary-queue counts bypass mutexes only with the existing admission,
claim and pending/in-flight termination protocol.

Five bounded Loom models include the production queue protocol with individually
linearizable owning-link operations. They cover producer order/exact delivery,
racing consumers, detached links, counted publication/closure and the
popped-before-reservation claim handshake. Each uses preemption bound 2 and at
most 10,000 permutations/branches. They do not model ArcSwap internals or prove
system-wide progress. Loom substitutes `try_unwrap` for unavailable `into_inner`;
stack-safe destruction is checked by the native long-chain test instead.

The final native GC suite passes 427 checks across 38 binaries; WASI passes 221
across 31. The optimized library passes 203 checks. Four native VM GC/reference
checks and two WASI VM checks pass. Miri separately passes the generic FIFO
payload-release test and integrated continuation/fairness test, without permissive
provenance. Native/WASI Clippy and bare-Wasm examples pass. The
[verification record](../target/gc-implementation/queue-publication-verification.json)
records commands, source/log hashes, the genuine baseline timeout, model bounds
and intermediate compile/lint corrections.

There is no new performance or endurance qualification in this correctness batch.
Two fixed nodes and a shared counter are allocated per generation; each fallback
publication allocates another node. Diagnostics exclude the fixed nodes from entry
capacity and do not measure their allocator overhead. Host allocation, nonempty
ordinary queue holders, admitted unfinished publishers and opaque callbacks retain
progress limitations. Test deadlines are not latency measurements.

## Compiled-entry argument validation (2026-11-08)

The baseline `(I)I` fixture accepted no arguments; its load remained within the
zero-initialized inline buffer. No intentionally out-of-bounds call was executed.
Compiled functions now retain expected value tags from the exact descriptor walk
used to emit local initialization. Count rejection precedes buffer construction
and native stack checks; kind checks share the existing marshalling pass. Generated
code and execution counters are untouched by a rejected call. Clone ownership
includes immutable signature metadata.

Portable tests cover expected counts 0, 1, 8, 9, 10 and 255, all five value tags,
and both inline/heap buffers. Native tests also place an impossible native-stack
requirement behind malformed arguments to prove error priority. Generated methods
cover counts through 127, primitive/object/array descriptor kinds, wide values and
instance receiver position. The wide static descriptor takes ten ABI values in
twelve JVM local slots. The instance descriptor takes receiver/long/double as
three values. That receiver fixture checks shape only, with stub root helpers;
it does not model a valid Java virtual call on null.

The VM bridge uses an eligible primitive method, tests padded/unpadded wide values,
checks rejected calls never enter, verifies caller roots survive rejection and
that released rejected references are reclaimed. Native-frame exhaustion still
executes both correctly typed buffer paths. The
[verification record](../target/gc-implementation/jit-arguments-verification.json)
retains the genuine baseline failure and intermediate compile/lint corrections.
The latter were import qualification, a wildcard match and the alignment annotation
for a counter pointer owned by the test. The first VM fixture also counted the
argument clone's root as a leak; the corrected fixture samples after cloning,
warms the emergency-throwable cache and checks after argument release separately.

Debug and optimized JIT each pass 227 library and 237 integration checks. The full
release VM suite passes 962 library checks; the added bridge check passes separately,
for 963 distinct library checks. Four native GC/reference checks, two WASI VM
checks and 29 WASI JIT checks pass. Native JIT/VM and WASI JIT Clippy, bare-Wasm JIT,
formatting and diff checks pass. The ignored large-frame child is exercised by its
bounded parent subprocess test; it is not counted as an independent extra pass.

This change validates count and representation, not arbitrary raw reference
addresses, helper/context ABI or lifetime. Existing production eligibility remains
conservative. Wasm runs portable Rust ABI stubs and the interpreter; it does not
run Cranelift-generated code. Per-function signature storage and entry validation
costs remain unmeasured. No GC progress or external performance gate is qualified.

## Barrier claim, application and cleanup progress (2026-11-08)

The held-buffer baseline timed out before draining another context's record.
Collector claims now try buffer/ready locks transactionally and rotate; each
attempt consumes budget. Required application and final owner cleanup execute
outside collector coordination. Drain turns alternate with normal phase/tracing
work. Terminal boundaries yield on pending records. Optional post-application
certification skips a busy metadata shard.

Twenty-four native configurations exercise both publication and phase policies,
with zero registered background workers. Eight claim schedules hold either a
buffer or the ready registry; the held-buffer cases also require the other
context's parent and child tracing to finish before release. Eight application
schedules hold metadata or the mark injector: the claimed record keeps pending=1,
no full ticket completes, the coordinator/publication remain available, counted
admission is zero and another collector's steps return. Eight final-owner schedules
hold registry removal or retired-root statistics after acknowledgement; three
unrelated full-collection requests finish while the cleanup thread is stalled.
All schedules verify graph retention or reclamation and zero final charged bytes.
Portable tests check failed last-record claims preserve both ownership and pending
count, and optional certification does not wait after required work completes.
The latter uses a synthetic stale-epoch record to isolate that optional lookup;
it is not a model of actual concurrent publication.

The first application fixture incorrectly polled the try-ready hint, which could
make the drainer legitimately yield. It now observes a passive post-pop notification;
the older explicit-flush fixture uses the same notification. A later fairness
assertion initially failed to compile because it accessed collector-private fields;
it now checks public completed-tracing statistics. Both intermediate logs are
retained alongside the genuine held-buffer baseline failure.

The native GC suite passes 414 checks, WASI 219, and the optimized library 195.
Four native VM GC/reference checks and two WASI VM checks pass, as do native/WASI
Clippy, bare-Wasm examples, formatting and diff checks. The
[verification record](../target/gc-implementation/barrier-drain-verification.json)
records commands, hashes and forced observations. Test deadlines are not latency
samples. Required application, mutator flush/overflow, admitted publishers and
opaque callbacks still retain waits/reservations. No performance, zero-STW or
endurance gate is qualified by this batch.

## Context enumeration and cleanup progress (2026-11-08)

The held-allocation-cache baseline prevented a full collection from finishing.
Context enumeration now uses a published immutable registry and retains one current
context per generation cursor. Cache preparation and final owner release execute
after publication admission, cycle and coordinator guards are dropped. A busy cache
retains its charged capacity without delaying the scan. A successful release moves
unused ranges/credit out of the cache mutex, then returns ranges and refunds credit.

Sixteen forced native configurations cover both publication/phase policies:
four hold the context registry, four hold the allocation cache, and eight stall
final context cleanup on either registry removal or retired-root statistics.
Three full-collection requests finish per configuration while the contention
persists, with unrelated garbage reclaimed. All run with zero registered background
workers. The held-cache fixture retains 4,080 logical credit bytes and 16,320 unused
descriptor-range bytes while its total logical charge remains 4,096 bytes. After
release, the next collection returns unused capacity and preserves hit/refill
counters. Portable checks exercise view reads under the writer, cursor ownership,
and accounting after final context-owner release with a page still pinned.

An optimized run exposed an earlier slot-root fixture assumption: a first step can
read the root immediately when it occupies the first page/shard. Its later timeout
therefore did not necessarily exercise a root lookup; child tracing could be waiting
on metadata with coordination already released. The corrected fixture holds the
metadata shard before the first collector step. The eight-case matrix passes.
The earlier held-cache test remains a genuine pre-fix failure; logs are retained.

The native GC suite passes 409 checks, WASI passes 217, and the optimized library
passes 190. Four native VM GC/reference checks and two WASI VM checks also pass.
Native/WASI Clippy and the bare-Wasm build pass. Commands, hashes,
observations and intermediate failures are in the
[verification record](../target/gc-implementation/context-progress-verification.json).
These test deadlines are not latency samples or performance comparisons.

The registry view clones its map at attachment/removal, with old views potentially
retained by readers. A scan can retain its current detached context and empty-page
cache until advancing. These costs and setup/teardown throughput remain unqualified.
Cleanup can still occupy its executing thread; other context-destruction paths,
required buffers, queue publication and opaque callbacks retain waits. Stalled
admission still excludes phase closure. The full zero-STW/performance/endurance
goal remains unmet.

## Slot/context root metadata retry (2026-11-08)

The held-shard baseline reproduced a coordinated slot-root collector step timing
out with its coordinator unavailable; that path held no publication admission.
Slot/context scans now rewind only the selected slot before yielding on a busy
object-metadata lookup. Retry reads the slot again, handling removal or generation
reuse without keeping a stale address. Tree enumeration shares the try-lookup.
Successful root read-to-shade operations retain their admission/phase protection.

Eight native schedules cover both stores, publication policies and phase policies.
They require the steps to return and release coordination/admission before releasing
the metadata shard, then verify parent/child survival and eventual reclamation.
The portable first/last-slot test retries after address replacement and removal,
requires fresh lookup and checks that other roots are visited once. It adds no
cursor fields or heap allocations to the root scan.

The GC suites pass 404 native checks, 215 WASI checks and 185 optimized library
tests. Four native VM GC/reference checks and two WASI VM checks also pass.
Native/WASI Clippy and the bare-Wasm examples build pass. The
[verification record](../target/gc-implementation/root-metadata-verification.json)
retains source/log hashes, commands and the pre-fix failure. Context registry/cache,
queue publication, reference-processing and other waits remain. A stalled admitted
publisher or opaque callback can still exclude closure. No new throughput,
latency-tail, endurance or external collector qualification is claimed.

## Resumable index contention (2026-11-08)

The held-object-shard baseline timed out with both the coordinator and legacy
publication writer unavailable. Object/set scans now retain per-shard positions,
try one shard at a time and rotate on contention. A failed downstream operation
restores the selected entry's previous position. Retirement acquires every required
index before removal; promotion acquires membership before changing age or marks.
The previous root-read test expected a blocked metadata read to retain admission;
it now requires that read to yield with the root cursor unchanged. Successful
read-to-shade operations still hold their existing admission.

The 48-case sweep matrix covers both publication policies, both phase policies,
young/full collection and held object/identity/young/remembered indexes. Sixteen
counted cases also hold publication admission. With no registered workers, another
shard's garbage is reclaimed before release. The deferred object's identity and
storage remain intact, and collection reclaims it exactly once after release.
Eight promotion cases require unchanged age/membership on contention and exactly
one completed promotion afterward. Two tree-root cases retain the rooted graph.
Portable map/set cases cover one, two and sixteen shards and fresh lookup after
address replacement. Timeout assertions follow fixture release and thread joins.

The native GC all-target suite passes 402 checks, the WASI suite passes 214, and
the optimized GC library passes 183. Four native VM GC/reference checks and two
WASI VM checks also pass. Native/WASI Clippy and the bare-Wasm examples
build pass. Per-index cursors measure 176 bytes on Linux x86-64 and 88 on wasm32;
relative to the prior two-word cursor, ten per-collector cursors add 1,600/800 bytes.
This fixed memory tradeoff and the progress checks are not throughput or latency
measurements. Commands, source/log hashes and the retained baseline are in the
[verification record](../target/gc-implementation/index-progress-verification.json).

Other shards can progress during an active sweep, even with a held counted
admission; cycle closure and subsequent cycles still require that admission to
finish. The root-metadata follow-up above removes slot/context lookup waits.
Queue publication, context registry/cache operations, reference registries,
statistics, callbacks and buffer operations retain other waits. No zero-STW,
G1/ZGC superiority or sustained-load qualification is established.

## Collector synchronization yields (2026-11-08)

Forced regressions reproduced a stalled mark queue holding the coordinator and
legacy publication writer, an actual dequeued/unreserved mark claim holding global
coordination, and a relocation claim holding its coordinator. The first case also
prevented another thread's already-marked root operation from finishing. A separate
held-publication-reader test reproduced a collector/coordinator wait, while the
other publisher still finished; this is consistent with the pinned native
recursive lock's documented reader preference. It is not evidence of a queued
writer blocking those new readers.

Mark claims now try every queue, including a sole injector. Empty checks distinguish
busy from nonempty and yield without changing terminal phases. Terminal mark and
relocation claim writers, and every legacy collector publication-writer acquisition,
also try once and yield on contention. Existing queue contents, cursor ownership,
pending/in-flight reservations, phase observations and reference semantics remain
in force. `publication_write_wait` is retained for historical report compatibility;
failed writer attempts now count as `publication_retries`. Zero writer wait time
therefore cannot be interpreted as a zero-STW result.

The regressions exercise 40 held-queue and 10 held-reader configurations, an actual
paused mark claim, two held relocation-claim configurations and the portable
single-queue work/budget test. Tests explicitly release and join stalled threads
before evaluating the expected timeout failure. The first exploratory mark-claim
failure unwound before that cleanup and has a recorded child disconnect; the
corrected pre-fix run and final test use ordered cleanup.

The native GC all-target suite passes 398 checks; the WASI GC suite passes 212.
Native/WASI Clippy and the bare-Wasm example build pass. The release GC library
passes 179 tests; native VM GC/reference integration passes four checks and WASI VM
lifecycle passes two. Raw before/after logs are retained under
`target/gc-implementation/` in the
[verification record](../target/gc-implementation/publication-yield-verification.json).
These are correctness/progress checks with test timeouts, not isolated benchmarks,
latency distributions, sustained-load qualification or external GC comparisons.
A stalled admitted publisher or opaque tracing callback still prevents relevant
closure. Queue publication and several index/buffer waits can still occur inside
admission or a held writer. The no-global-blocking goal remains open.

## Small changing compiled-frame root sets (2026-11-08)

The new regression test reproduces replacing a small map despite available capacity
(`tests-frame-small-replacement-before.log`). The changing-set path now prepares
distinct missing owners in bounded inline storage, reserves for every insertion
before deletion, retains unchanged entries and drops removed owners after unlocking.
All prior owners remain published during validation. Reservation includes all new
entries because deletion may leave tombstones instead of immediately reusable space.
Larger sets retain the transactional replacement-map path.

The 68 focused checks pass, including rollback after preparing a valid owner then
encountering a wrong-typed identity, discarded-object reclamation under a held frame
mutex, and transitions through 1/8/31/32/33 roots. Real compiled reference returns
also cover nine arguments and long arguments with `Unused` slot placeholders;
fixtures now calculate `max_locals` from their descriptors and instructions.
The separate bare-Wasm smoke passes the worker-count lifecycle assertions.
The corrected schema-2 fixture passes all 14 workload/policy smoke cases with zero
registered workers and post-context reclamation. These are correctness checks;
the matched performance screens below measure helper cost separately.

The first corrected workerless screen (`frame-replacement-paired`, seed 45) has
196 valid processes and unchanged inputs. It shows no clear single-root improvement
and an eight-root counted regression: ratio 1.040 [1.008, 1.054]. The gated 64-root
control also shifts by 1.018 [1.010, 1.033]. This candidate is not a performance win.
A follow-up drains a disjoint old set without membership scans, avoids moving the
inline preparation buffer into its iterator, and handles a sole changed root
without batch buffers. Its 196-process screen (`frame-replacement-paired-drain-single`,
seed 46) improves changing one-root costs by 9–14% and eight-root costs by 3–5%,
but retains a gated 64-root regression (1.028 [1.011, 1.032]). Both full optimized
candidate suites pass 916 tests, with the measurement fixture ignored.

The subsequent large fallback shares one context lock across runs of already-owned
inputs, instead of acquiring it separately for each owner. It releases the lock
before every missing-identity validation and before old/temporary-owner destruction.
Expanded rollback cases mix an existing owner, a valid new owner and an invalid
identity, then check survival and temporary reclamation across the inline limit.
The retained version passes all 916 default-feature optimized VM tests (one
benchmark ignored), the 68 focused diagnostic checks and native/WASI Clippy.
Evidence uses `tests-frame-fallback-lock-{vm,release-vm}.log` and
`clippy-frame-fallback-lock-{native,wasi}.log` under `target/gc-implementation/`.

The retained 196-process screen (`frame-replacement-paired-fallback-lock`, seed 47)
reduces changing-one-root cost by about 10% and stable-64-root cost by 28–29%.
A second 196-process confirmation uses the same frozen binaries, seed 48, one
million measured calls and 100,000 warmup calls per process (ten times the initial
screen). No builds, tests, profiling or source edits overlap either timed run.
All four screens preserve every process and have zero failures/incomplete cells
and unchanged binary/source/JDK inputs: 784 valid processes in total.

The confirmation's gated changing-one-root median is 212.41 → 190.53 ns/call,
paired ratio 0.906 [0.895, 0.910]; counted is 217.28 → 188.76 ns,
0.881 [0.859, 0.922]. The gated 64-root fallback is 1555.41 → 1078.28 ns,
0.697 [0.679, 0.765]; counted is 1524.21 → 1057.09 ns,
0.700 [0.681, 0.707]. Every other confirmation interval includes one; in particular,
eight-root replacement has no clear improvement. Intervals are exploratory,
unadjusted paired-process intervals, not latency-tail or universal-speedup claims.

These compare with the prior small-set reuse implementation, using the corrected
workerless schema-2 fixture in both binaries. They do not reclassify the historical
schema-1 results below. The optimized maps can retain spare insertion capacity until
context teardown. Sustained memory, contention, whole-invocation cost, Wasm timing,
mixed frames and external GC superiority remain unqualified; production eligibility
and the collector's global progress limitations are unchanged.

[Confirmation report](../target/gc-implementation/frame-replacement-confirmation/REPORT.md) ·
[Confirmation graph](../target/gc-implementation/frame-replacement-confirmation/frame-roots.svg)

## Compiled return/exception ownership and worker-count correction (2026-11-08)

Real compiled-code counters reproduced redundant registrations: a simple array
return used two instead of one, an uncaught argument exception used three instead
of two, and a caught argument exception used four instead of two. A separate
pending-owner test exposed a new registration at handler entry when that identity
was absent from the frame map. The before logs are `tests-jit-{return,exception,
caught,handler}-transfer-before.log` under `target/gc-implementation/`.

Completed invocation contexts now move a published reference owner into the returned
VM value. Missing identities still undergo typed validation while caller arguments
remain owned. Pending exceptions share a frame owner when available; handler entry
moves the pending owner into the helper-result set, and uncaught propagation moves
it into the VM error. Both argument-buffer paths consume the context on return.
The obsolete raw exception-consumer and raw helper-retention APIs were removed.

The real compiled fixtures now use one registration for new-array and argument
returns, zero for null, and two for caught/uncaught argument exceptions. They
preserve returned values across collection after context and argument cleanup,
and reclaim them after the result is dropped. Handler-owner, sentinel, unwind,
concurrent replacement, typed fallback and unused-result cleanup checks also pass.
These are ownership/registration checks; no elapsed-time improvement is claimed.

An attempted workerless allocation test failed its assist assertion and exposed a
fixture error: native `threads=0` selects one worker when `VM::initialize` calls
`start()`. A new `registered_workers()` snapshot exposes the actual recovery-worker
registration count without instrumentation. Its lifecycle test covers start,
idempotent start, stop and restart with configured counts 0/1/3. Cooperative tests
now explicitly stop/join the startup worker and verify zero registered workers.

All 917 optimized VM tests and 66 focused debug checks pass (one benchmark ignored),
along with all 14 collector lifecycle tests, native/WASI Clippy and 46 benchmark
harness tests. The four allocation-pressure fixtures pass all 16 policy/worker
combinations. They require swept objects and freed bytes before return; a complete
cycle is not required once cooperative recovery has reclaimed enough storage.
Workerless cases verify assists and unchanged worker-step counts. Evidence is in
`tests-jit-owner-transfer-release-vm.log`, `tests-jit-owner-transfer-verified-workers.log`,
`tests-jit-transfer-worker-count.log`, `clippy-jit-owner-transfer-verified-{native,wasi}.log`
and `tests-jit-transfer-bench-schema.log` under `target/gc-implementation/`.

The earlier frame-publication timing screens therefore used a one-worker native
configuration. Their raw counters show zero completed collections in the timed
region; they did not sample worker steps or registrations. The matched relative
helper-cost results remain observations for that configuration, **not workerless
qualification** or proof of no in-progress collection. Original metadata and
binary/source archives are preserved; `CORRECTION.json` and report notices accompany
all three screens. The old graph caption needs that qualification.

Fixture schema 2 explicitly stops workers before timing, records their count before
and after, and rejects nonzero counts. The Python harness rejects schema 1 instead
of silently relabeling old data. Fresh matched schema-2 timings are recorded above;
the previous graphs must not be presented as workerless results.

## Small compiled-frame publication sets (2026-11-08)

`tests-frame-publication-before.log` reproduces rebuilding map storage for an
unchanged root set. `publish_frame_roots` now reuses existing owners when both the
input and current map have at most 32 entries and every non-null identity is
already owned. Subsets remove obsolete owners into a stack buffer and release
them after unlocking. Nulls and duplicates are treated as set members, so equal
input lengths cannot accidentally retain omitted objects. New identities and
larger frames retain transactional preparation/replacement.

The first detected miss is validated/prepared before the fallback loop. This
avoids repeating its context lookup and avoids a per-entry branch in large frames.
The unsafe sole-executor contract protects that negative lookup. An expanded
rollback schedule prepares a valid new owner before a later invalid typed identity;
it preserves the original roots and reclaims the temporary object after its
external owner is dropped. The context-lock stall, live-local/operand/handler,
dead-local reclamation and concurrent helper ownership tests continue to pass.

All 62 focused VM/JIT checks pass (one benchmark intentionally ignored), along with
native profiling/default and WASI Clippy. The retained frozen release binary passes
all 910 default-feature VM tests, with the benchmark intentionally ignored. The
additional rollback schedule was added afterward and passes in the focused debug
suite; it changes test assertions only. Logs use `tests-frame-publication-*` and
`clippy-frame-publication-*` under `target/gc-implementation/`.

`benchmarks/gc/frame_roots.py` freezes/validates binary-source pairs, requires the
same benchmark fixture in both archives, pins each process to one CPU and retains
all raw outputs, resources, failures and incomplete pairs. Its 46-test Python
suite includes integrity and incomplete-grid rejection. The Rust fixture makes
an indirect call to the real FFI helper and validates identity sets, contents and
post-context reclamation. These original schema-1 screens used one configured native
worker, despite an incorrect zero-worker label; see the correction above. Measured
completed collections were zero. Startup, warmup, spills and instruction execution
are excluded; absence of in-progress collector work was not independently checked.

Three exploratory screens each completed 196 valid processes (seven independent
pairs × seven workloads × two publication policies). All source/binary/JDK inputs
remained unchanged. The first screen (`frame-publication-paired`, seed 42) exposed
an extra lookup on misses. Caching it inside the loop (`frame-publication-paired-final`,
seed 43) removed that signal but regressed the 64-root control by about 6%.
Preparing it before the loop is retained in `frame-publication-hoisted` and the
`frame-publication-paired-hoisted` screen (seed 44).

The retained gated medians are 55.68 → 13.15 ns for one stable root and
223.20 → 26.64 ns for eight. Paired ratios are 0.238 [0.227, 0.250] and
0.118 [0.114, 0.123]. Counted publication has similar reductions. The 64-root
control ratios are 0.976 [0.955, 1.005] gated and 0.992 [0.979, 1.071] counted.
**Changing roots still have a tradeoff:** gated one-root replacement is
1.038 [1.031, 1.065] and eight-root replacement 1.007 [1.001, 1.014]; counted
intervals include one. These are unadjusted exploratory paired-process intervals,
not a claim that every path improved. The small-set threshold was fixed at 32
throughout; the intermediate results are retained rather than discarded.

[Retained report](../target/gc-implementation/frame-publication-paired-hoisted/REPORT.md)
and [graph](../target/gc-implementation/frame-publication-paired-hoisted/frame-roots.svg)
show every condition. The benchmark does not establish whole-invocation throughput,
latency tails, contention, Wasm performance, global progress or G1/ZGC superiority.
Production reference eligibility and J1/J2 qualification remain open.

## Reference-local liveness (2026-11-08)

`tests-jit-frame-liveness-before.log` reproduces a compiled `OutOfMemoryError`:
a roughly 20 MiB array remains retained in its reference SSA variable after the
Java local slot is reused for an int, preventing the next roughly 20 MiB allocation
in a 32 MiB heap. Backward reference-local liveness now excludes that dead owner;
the fixture completes under both gated and counted publication.

The worklist follows normal branches, switches, backedges and exception handlers.
Reference loads generate uses; all stores/increments kill the prior reference kind
on normal paths, including both slots of long/double stores. Exception uses merge
after normal kills and conservatively cover all protected instructions. Seven
analysis tests cover last reads, path-dependent replacement, loops, handler-only
uses, sparse Java slots across bitset words, every local-write form and fail-closed
state/edge limits. This is distinct from class verification and initialization.

The real compiled allocation fixtures retain a live local, a live operand, or a
local read only by an exception handler while allocating 62.5 MiB in a 32 MiB heap.
Each passes both publication policies, with collection observed before return.
The feature-enabled backend passes 221 unit, 233 integration and one doc test;
61 focused VM/JIT checks and native/WASI Clippy also pass. Evidence uses
`tests-jit-frame-liveness-{backend,vm,initial}.log` and
`clippy-jit-frame-liveness-{native,wasi}.log` in `target/gc-implementation/`.

HotSpot 25 with `-Xverify:all -Xint -Xmx32m` verifies 41 matching results, including
the four heap-pressure fixtures, and rejects the invalid-merge fixture. The
23 generated classes, bytecode/hash manifest and generator are retained in
`jit-frame-liveness-hotspot/` and `check-jit-frame-liveness-hotspot.py`; output is
`tests-jit-frame-liveness-hotspot.log` in the same evidence directory.

All 912 optimized release VM library tests pass, including the heap-pressure
and ownership regressions (`tests-jit-frame-liveness-release-vm.log`). The default
backend configuration passes 220 unit, 233 integration and one doc test; native
Clippy also passes with execution profiling disabled (`*-default.log`).

Production eligibility is unchanged. Caller-owned argument Values remain roots;
callee liveness does not remove those external owners. The publication follow-up
above reduces map allocation for small repeated/subset sets. Boundary reduction,
mixed-call/argument/suspension lifetimes, relocation, collection
polls and global progress still require implementation or qualification. No timing
comparison or J1/J2 completion is claimed by these correctness tests.

## Initial compiled-frame root handoff (2026-11-08)

`tests-jit-frame-loop-before.log` reproduces `OutOfMemoryError` in a compiled loop
whose discarded arrays remain rooted until invocation return. The new handoff
spills current operand identities and conservative reference locals at bytecode
boundaries. It prepares replacement owners before releasing old ones, keeps prior
roots on validation failure, and drops obsolete owners outside the context mutex.
The production allowlist is unchanged; its primitive methods emit no handoff calls.

Two real-VM generated-code fixtures preserve a live reference respectively in a
local and below loop operands. Each runs under gated and counted publication,
allocating 8,000 arrays (62.5 MiB payload) in a 32 MiB managed heap within one
invocation. All four complete, read the preserved value and observe collection
before return. This is a reclamation/lifetime test, not a throughput comparison.
A separate forced test retains the context mutex across full collections: the
published reference survives and the omitted result is reclaimed. Invalid typed
identities leave the old root set installed; invalid spill buffers set an exception
without discarding roots. Oversized reference spill buffers are rejected.

The feature-enabled backend run passes 214 unit, 233 integration and one doc test;
59 focused VM/JIT checks and native/WASI Clippy pass. Logs are
`tests-jit-frame-roots-{backend,vm}.log`, `tests-jit-frame-loop-initial.log` and
`clippy-jit-frame-roots-{native,wasi}.log` under `target/gc-implementation/`.
The ABI requires one executor per invocation context. Argument owners survive
entry, helper results remain owned until publication, and return conversion runs
before context cleanup; pending exceptions have a separate owning handle.

All 910 optimized release VM library tests also pass, including both allocating
loop fixtures and the ownership regressions. Default backend tests and native
Clippy pass with diagnostics disabled. The final equivalent classification split
keeps root requirements independent of production eligibility, so enabling a new
opcode later cannot silently turn off its root transfer; all backend tests,
59 focused VM checks and Clippy pass again after that split. Release evidence is
`tests-jit-frame-roots-release-vm.log`; the final checks use `*-gate-*` logs.

At this initial snapshot all declared reference locals were retained, including
dead locals after slot/type reuse. The liveness follow-up above removes that
retention. A fresh replacement map is still allocated at each instruction boundary.
Cheaper publication, relocation and mixed-call/suspension coverage,
and the collector's global progress gates remain open. This does not close J1/J2
or permit reference-bearing production execution.

## Typed local reuse prerequisite (2026-11-08)

Three backend regressions reproduced rejection of valid bytecode: a local slot
reused for int/float, a long argument's second slot reused for float, and a
reference/float/reference slot crossing a branch merge. Local lowering now maps
each `(Java slot, Java kind)` to its own SSA variable. A separate test ensures a
reference and long never share that variable even though both use I64 registers.
This fixes representation; it does not supply liveness or initialization analysis.

All 447 feature-enabled JIT checks and 55 focused VM/JIT checks pass, along with
native and WASI Clippy. A live-VM test exercises both primitive fixtures through
the unchanged production eligibility gate and compares an explicitly interpreted
VM. Counters assert one compiled entry and one interpreter entry in the separate
VMs; an initial test caught that `Frame::execute` also uses production dispatch,
so that entry point is not used as an interpreter oracle. HotSpot 25 accepts the
four added results (37 total) with verification enabled. The class-bytecode/hash
manifest is in `target/gc-implementation/jit-local-kinds-hotspot/`; its sibling
generator is `check-jit-local-kinds-hotspot.py`. Failure and verification logs use
`tests-jit-local-*` and `clippy-jit-local-kinds-*` in the parent directory.

Reference/control-flow production eligibility remains closed. No runtime frame
root ABI, liveness qualification or performance improvement is claimed.

The broader verification after typed-local lowering passes 906 VM library tests,
three native GC integration tests, and the reference/finalization matrix across
JDK 8/11/17/21/25 and the configured publication/overlap/compaction policies
(three driver tests pass, one subprocess fixture is intentionally ignored).
Default JIT tests and native Clippy with the feature disabled also pass.

## JIT reference kinds and control-flow prerequisites (2026-11-08)

Reference and long values previously collapsed into the same I64 stack type.
Java kinds now survive stack simulation, block parameters, duplication and helper
continuations; references remain category one. Eight stack-form fixtures and
branch/helper-continuation tests exercise the distinction. Invalid long-valued
monitor fixtures now assert rejection rather than executing a backend no-op.

Three additional regressions failed before the CFG correction: a backward-reached
block's successors were absent, a reference/long fallthrough merge was unchecked,
and a handler inherited operands left by the throwing instruction. A worklist now
checks every incoming stack state and resets handlers to one reference, as required
by [JVMS exception handling](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.10).
States are retained at block boundaries rather than copied at every instruction.
Legacy `jsr`/`ret` lowering is rejected until return-address analysis exists; it was
already excluded by the production gate. Switch regressions additionally exposed
negative-key conversion failures and missing arguments below the consumed key.
Forwarding blocks preserve those arguments, including their reference kinds.

The final feature-enabled backend run passes 213 unit, 229 integration and one
documentation test. The live VM differential test exercises 16 generated-code
calls against interpreted results using actual helpers; its execution counters
confirm those calls entered compiled code. HotSpot 25 with `-Xverify:all` validates
33 results across the stack/control-flow fixtures and rejects the invalid merge.
The independent class files, bytecode hex and SHA-256 manifest are retained in
`target/gc-implementation/jit-control-flow-hotspot/`; its generator and logs are
`check-jit-control-flow-hotspot.py` and `tests-jit-control-flow-hotspot.log` in the
parent directory. Before/final backend logs use `tests-jit-worklist-*` and
`tests-jit-switch-stack-before.log`; the live differential log is
`tests-jit-worklist-vm.log`.

These checks do not establish compiled-frame root publication, local-variable
liveness, collection polls or mixed-call correctness. Eligibility remains closed
for these reference/control-flow methods, and no performance improvement is claimed.

The complete VM run subsequently passes all 903 library tests. Default JIT checks
(212 unit, 229 integration, one documentation test), feature-enabled/disabled native
Clippy and WASI Clippy pass. The live differential fixture was split into smaller
builders for Clippy and rerun afterward; behavior is unchanged.

## Owning helper-result registrations (2026-11-08)

Repeated helper returns previously appended a new numeric tree-root registration
on every load. A regression retained 2,000 registrations for one identity. The
context now retains one owning handle per non-reused object identity; already
owning results share their registration. Preparation and replacement cleanup run
outside the context mutex. A four-thread test also verifies three distinct results
survive collection and become reclaimable after context destruction. All 54 JIT/VM
helper checks and native/WASI Clippy pass. Logs are `tests-jit-helper-owners-*` and
`clippy-jit-helper-owners-*` under `target/gc-implementation/`.

This bounds repeated registrations, not distinct-result retention: a long allocating
invocation can still accumulate one owner per object. Runtime frame liveness and
early release remain necessary. No timing improvement or progress gate is claimed.

## Execution coverage and mixed-width stack correction (2026-11-08)

The opt-in `execution-profile` feature records distinct method instances, production
eligibility restrictions, compilation queue/result state, compilation wall time,
generated code bytes and interpreter/intrinsic/generated-code entries. Generated
entry counters increment at the native call boundary and are shared by function
clones. Recording is compiled out without the feature. Metadata/code retention
and recorder locks make these diagnostic builds unsuitable for timing comparisons;
`run.py` now rejects their output. `coverage.py` freezes inputs, validates completed
work against entry counts, retains failed forks and exports invocation-share graphs.
Its manifest and lifetime counters do not qualify phase/loop coverage or CPU shares.

All 901 feature-enabled VM library tests and 433 JIT checks initially passed,
along with three actual WASI function/counter tests and native/WASI Clippy.
The first six-fork native screen nevertheless exposed missing interpreter fast-path
events: its three interpreted forks failed count validation. The three JIT-enabled
forks validated and recorded zero compiled entries for every required hot method.
The failed evidence remains in `execution-coverage-native/`; it is not a valid
complete coverage baseline. Recording now includes the actual fast frame push,
and the existing recursive-trampoline test asserts all six expected entries across
both dispatch paths. Feature-enabled/disabled native and WASI Clippy pass afterward.

A subsequent stack audit reproduced a separate backend bug in `dup2_x2` form 3.
Both simulation and lowering omitted one duplicated category-one value below a
wide value. The prior simulation test asserted the truncated stack; the generated
test inspected only its top. The corrected test consumes the entire result, and
failed before the fix with stack underflow. Both implementations now preserve the
full sequence required by [JVMS dup2_x2](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.dup2_x2).
All 434 JIT checks pass, as does a real VM test confirming the method enters compiled
code through the unchanged production gate. HotSpot 25 with `-Xverify:all` accepts
the identical operation sequence and its expected result. The reference-kind and
control-flow corrections above follow this earlier snapshot; runtime frame roots
remain outstanding.

The corrected coverage baseline completes all nine forks: interpreted and JIT-enabled
candidate on native, plus cooperative candidate on actual WASI, across allocation,
roots and mixed workloads. CPU affinity is 0/2, quota two, managed heap 64 MiB and
process ceiling 512 MiB. All workload checksums and entry-count checks pass, all
inputs remain unchanged, and no memory-ceiling event occurs. Every required method
has zero compiled entries. Native JIT forks record execution of only two small
library methods; Wasm records no compiled entries and reports unsupported target.
These are one-second diagnostic windows after one-second warmup, with lifetime
entry counts, not throughput/latency or sustained-load qualification.

`execution-profile-current/` freezes the corrected native/Wasm binaries and sources.
`execution-coverage-current-{native,wasi}/` contains reports, raw output, profiles,
resource observations and SVG/PNG graphs. Both graphs were inspected. The initial
failed baseline remains separate. This establishes the current coverage failure;
it does not complete J0's phase/loop/CPU attribution or J1–J5.

Evidence under `target/gc-implementation/`: `tests-execution-profile-*`,
`clippy-execution-profile-*`, `tests-execution-coverage-harness.log` (43 tests),
`tests-jit-dup2-x2-{before,native-before,final,vm,hotspot}.log` and
`check-dup2-x2-hotspot.py`. The initial diagnostic build/source pairs are retained
in `execution-profile-final/`. No throughput or tail-latency improvement is claimed.

## JIT pending-exception ownership (2026-11-08)

A reproduced unwind leaked the numeric root detached by an exception consumer.
The pending exception now owns a rooted handle. A context-local mutex pairs its
pointer with that owner during replacement/extraction; prior-owner deletion and
consumer callbacks run after unlocking. The extracted owner releases its root on
unwind. The VM transfers that owner directly into a throwable result, avoiding
another identity lookup and registration. The unused raw extraction API, which
released its root before returning the handle, has been removed.

The failure is retained in `tests-jit-exception-unwind-before.log`; the fixed test
and concurrent replacement/transfer checks pass. All 898 VM library tests, the
native GC/reference matrix, 33 helper tests, JIT conversion tests, native Clippy
and WASI Clippy pass. The final owner-transfer cleanup was checked again with the
helper/conversion tests and both Clippy targets. Logs are under
`target/gc-implementation/tests-jit-exception-*` and `clippy-jit-exception-*`.
There is no timing claim. The production eligibility gate and invocation-long
transient-root retention are unchanged; this does not qualify compiled frames.

## Checked tree registrations and queued owner identities (2026-11-08)

A forced native wrap test reproduced a stale owning tree guard deleting a newer
registration after explicit revocation and numeric-ID reuse. Internal tree owners
now carry a checked 64-bit generation. Owning, construction, compatibility-snapshot
and queue-transfer cleanup all validate it under the existing tree mutex; generation
exhaustion fails before reuse. Public numeric IDs retain their API and are invalid
after removal. Tree entries need only an address and generation because root scans
resolve tracing metadata separately. Their payload is 16 bytes on native and Wasm,
and the owning registration retains its previous maximum size.

Pending references now store their owner's allocation identity instead of its raw
address. Transfer checks identity and type; stale queue cleanup also checks its
tree generation. A test explicitly revokes a queued owner, reuses the numeric root
ID and injects a stale identity-to-address hint. Transfer rejects that hint and
preserves the replacement registration. This avoids relying on host allocator reuse.

All 388 GC tests pass after the storage change, followed by an additional native
and WASI test proving that tree-root changes no longer call type-capability hooks
under admission. The admission-count test now observes actual tree insertion via
a test-only probe, preserving its one-admission check after the unused type query
was removed. Native GC/classloader/types/VM Clippy, WASI Clippy, 24 targeted WASI
root/reference tests, bare-Wasm build, three Miri cases and native VM/JDK-reference
tests pass. The later capability-hook case adds one native and one WASI check.
Tree synchronization and publication closure under contention remain unresolved;
these identity fixes do not establish nonblocking progress or a speedup.

Evidence: `target/gc-implementation/tests-tree-wrap-{before,after}.log` retains the
reproduced failure and fix. `tests-tree-generation-final.log`,
`tests-tree-generation-type-hook.log`, `tests-tree-generation-wasi*.log`,
`miri-tree-generation-*.log` and `tests-tree-generation-vm.log` record validation.
The first compact-entry test run failed the obsolete type-query probe; its log is
preserved alongside the replacement probe's passing run.

The frozen comparison completes 980 successful mechanism processes: seven pairs,
seven workloads, CPU 0, 100,000 operations and both instrumentation settings.
Candidate allocation and roots have ratios 1.014 [0.978, 1.067] and 1.006
[0.945, 1.049]; neither establishes a change. Handles rise to 1.052
[1.010, 1.071]. Pending transfer rises to 1.089 [1.059, 1.123] in candidate and
1.073 [1.060, 1.120] in baseline, consistent with the added identity lookup.
Control shifts about -11% in candidate, with an interval including one. The
correctness change is retained with its measured cost, without a speedup claim.

Forty additional processes compare native and actual WASI allocator counts, five
pairs per policy/target. Inputs and frozen Wasmtime remain unchanged. Native
requested bytes per operation are 401.766 → 401.744 in baseline and 428.338
unchanged in candidate; Wasm baseline is 278.814 → 278.834 and candidate 304.298
unchanged. Process peaks are effectively unchanged. This ephemeral fixture does
not establish a process-memory benefit from smaller native tree entries: its tree
root population is small. Wasm guest allocator counts exclude Wasmtime; process
peaks include it. The runner now freezes its analysis code and optional runtime.

A 60-second payload screen finishes all 600,000 arrivals with zero final backlog
and clean teardown. Operation p99.9/p99.99 is 58/502 µs, arrival p99.9/p99.99 is
503/1,168 µs and charged process peak is 32.0 MiB, using CPUs 0/2, quota two,
managed 64 MiB and process 512 MiB caps. This is not sustained-load qualification.

Frozen evidence: `tree-generation-{before,final}/`, `mechanisms-tree-generation/`,
`allocations-tree-generation-{native,wasi}/`, `endurance-tree-generation.json`,
`pressure-tree-generation/` and `tree-generation-status.json`, under
`target/gc-implementation/`. The preceding source archive was verified before
building its native/Wasm allocation-count binaries; both revisions use the same
fixtures. Graphs are supplied as SVG and PNG.

## Conditional root stamping experiment rejected (2026-11-08)

A prototype admitted ordinary root acquisition/deletion without an entry revision
stamp, adding a stamp only when its barrier published work or stronger retention.
Identity and weak validation kept their unconditional stamps. A new portable test
checked both white-child publication and redundant rooting of a retained child.
All 386 GC tests, native and Wasm Clippy, actual WASI, Miri and native VM/JDK-reference
checks passed. This experiment was nevertheless removed because the measured
benefit was not established. Retained Rust source hashes match the frozen preceding
compatibility-lookup revision exactly; its 385-test verification remains applicable.

All 840 mechanism processes pass. Candidate allocation has paired ratio 0.975
[0.942, 1.010], roots 1.024 [0.997, 1.027], handles 0.975 [0.938, 1.042] and
guard cloning 1.002 [0.981, 1.014]. Weak upgrade has 0.986 [0.931, 0.994], but
control shifts to 0.868 [0.863, 0.915], preventing attribution of that small change.
The same 60-process root-churn screen passes correctness with unchanged inputs;
both revisions have four collection deadline misses. Candidate full completion is
3/5 before and after, and young completion is 3/5 versus 4/5. Counted full worsens
from 5/5 to 4/5; counted young completes 5/5, with a broad completion-time interval
including one. These results do not establish reliable reclamation or a speedup.

The prototype also completes a 60-second, 600,000-arrival load screen with clean
teardown and 33.4 MiB charged process peak. This is evidence for the rejected
prototype, not the retained implementation, and does not qualify sustained load.

Evidence: `target/gc-implementation/root-stamps-{before,after}/`,
`mechanisms-root-stamps/`, `contention-root-stamps/`, `endurance-root-stamps.json`,
`pressure-root-stamps/` and `root-stamps-status.json`. Frozen archives preserve the
removed implementation and its additional correctness test.

## Compatibility lookup avoids the slot writer (2026-11-08)

Compatibility root removal now finds address/generation pairs through the published
page view, without the slot writer mutex. The production slot helper publishes
each new generation with release ordering and checks the generation on both sides
of its address read. A Loom model rejects pairing a reused slot's new generation
with the previous address. Claimed generations and owned pages still prevent a
late removal from clearing a replacement or touching freed page storage.

Forced native schedules finish compatibility removal and three full reclamation
cycles while a slot writer remains paused, for both publication policies and slot
stores. All 385 GC tests, native GC/classloader/types/VM Clippy, 21 actual WASI
root/context/reference tests, WASI Clippy, bare-Wasm build and Miri's compatibility
reuse test pass. This cold-path change has no measured speedup claim. Compatibility
removal still retains outer admission, context/tree registry synchronization and
snapshot allocation; this does not establish global nonblocking progress.

Evidence: `target/gc-implementation/tests-compat-root-{loom,progress,complete,wasi}.log`,
`clippy-compat-root-{native,wasi}.log`, `build-compat-root-bare.log` and
`miri-compat-root-reuse.log`.

## Root churn under contention remains unqualified (2026-11-08)

The native contention fixture now measures either edge barriers or fresh root
registration/release. Heap-edge inputs are protected by separate resident owning
roots, ensuring that each root operation actually registers rather than merely
cloning an existing owner. The driver verifies matching fixture hashes for both
revisions, rejects mislabelled operation counts and memory-ceiling events, and
preserves unfinished cycles as censored results. All 39 Python harness tests pass.

The combined root protocol below is compared with the pre-read-view revision,
with only the identical extended fixture overlaid on the archived prior sources.
All original source hashes are verified before building that baseline. Five
randomized pairs run four producers, sixteen promoted targets, 1,024 requested
collections and a ten-second deadline, CPUs 0/2 with two-core quota and a 256 MiB
process cap. Diagnostics are enabled. All 60 processes pass correctness and
teardown, inputs remain unchanged, and no process hits its memory ceiling.

| Policy | Generation | Complete runs before / after (of five) | Producer Mops/s before / after |
| --- | --- | ---: | ---: |
| Baseline | Full | 5 / 5 | 2.274 / 2.402 |
| Baseline | Young | 5 / 5 | 2.415 / 2.561 |
| Counted | Full | 2 / 4 | 2.372 / 2.592 |
| Counted | Young | 3 / 5 | 2.495 / 2.471 |
| Candidate | Full | 2 / 3 | 3.099 / 3.307 |
| Candidate | Young | 4 / 3 | 3.157 / 3.326 |

Fourteen runs hit the collection deadline. No completion-time ratio is reported
for counted/candidate conditions. Baseline full/young ratios have intervals that
include one. Higher producer throughput also offers more contention; these are
saturated producers, not a matched-arrival latency comparison. The result does
not establish reliable reclamation under sustained root churn or a performance
improvement. Removing the demonstrated writer waits has not solved publication
closure under contention.

Evidence: `target/gc-implementation/contention-root-protocol/` contains the report,
raw samples, frozen binary/source pairs, analysis source and SVG/PNG graph;
`root-protocol-contention-overlay.json` records baseline reconstruction.

## Atomic slot deletion and combined root protocol (2026-11-08)

Owning slot deletion now claims its generation without the writer mutex, clears
the address and publishes the deletion barrier before making the slot reusable.
Its page remains owned throughout. Cleanup happens after admission, using try-lock
or deferred work. Packed slot/generation tokens preserve registration size and fail
closed at generation exhaustion. A designated cache page remains indexed while
live, avoiding a second empty page. This supersedes the deletion limitation in the
preparation-only experiment below; compatibility/tree and admitted metadata/queue
waits remain. Cleanup is not bounded in the shard's page count or repeated requests.

A forced native schedule holds the actual slot writer while another thread drops
the last root. Deletion and three full collections finish and reclaim its graph
before that writer resumes, for both publication policies and both slot stores.
A production-helper Loom model checks stale removal against slot reuse. Packed
terminal generation/slot, page lifetime, context retirement and sparse-page cleanup
tests pass. The final revision passes all 383 GC tests, native GC/classloader/types/
VM Clippy, native VM/reference tests, 21 actual WASI root/context/reference tests,
WASI Clippy and bare-Wasm build. Two focused Miri cases and eight actual WASI unit
cases cover the packed token and retained pages.

The combined read-view/preparation/deletion protocol is compared with the frozen
revision before all three changes: seven randomized pairs, six mechanisms,
100,000 operations, CPU 0, 840 successful processes with unchanged inputs.

| Candidate mechanism | Before / after ns | Paired after/before, 95% interval |
| --- | ---: | ---: |
| Allocation | 275.11 / 302.07 | 1.096 [1.090, 1.100] |
| Roots | 79.97 / 97.09 | 1.259 [1.207, 1.262] |
| Handles | 103.65 / 120.80 | 1.165 [1.097, 1.170] |
| Weak upgrade | 129.04 / 147.25 | 1.134 [1.125, 1.163] |
| Guard clone/drop | 18.07 / 18.10 | 0.997 [0.934, 1.142] |

Ratios are medians of process-pair ratios, not ratios of the displayed medians.
Control has ratio 1.036 [1.019, 1.165]. This remains an experimental progress
foundation with a net overhead regression, not a performance win. The intermediate
unpacked deletion prototype is retained as evidence, including its larger regression.
Defaults remain unchanged.

A 60-second load screen completes all 600,000 arrivals with zero final backlog and
clean root/reservation/resident-page/managed teardown. Operation p99.9/p99.99 is
68/492 µs; charged process peak is 29.8 MiB under CPUs 0/2, two-core quota, managed
64 MiB and process 512 MiB caps. This does not qualify endurance or G1/ZGC performance.

Evidence: `target/gc-implementation/root-release-{after,packed}/`,
`mechanisms-root-release/`, `mechanisms-root-protocol-combined/`,
`endurance-root-protocol-combined.json`, `pressure-root-protocol-combined/` and
`tests-root-protocol-combined-complete.log`. The first full-suite attempt exposed
the changed cache-page expectation; its failed log remains alongside the final run.

## Slot-root read view and preparation progress (2026-11-08)

Slot scanners use atomic page addresses and an `arc-swap` page-membership view;
they no longer take the writer mutex. Cursors retain one page, with resident-page
accounting surviving context destruction until the cursor releases it. Slot
acquisition now reserves address-zero storage before admission. Validated commit
publishes/shades under admission; cancellation drops invisible storage. Forced
native schedules finish three full collections while a preparation writer is
paused, under both publication policies and both slot stores. A weak upgrade
paused there rejects the target after intervening reclamation. Reservation
invisibility, stale generations, page retirement and unwind cleanup are checked.

All 380 GC tests and native GC/classloader/types/VM Clippy checks pass. Native VM
GC/reference tests cover the JDK 8/11/17/21/25 matrix. Four focused Miri cases cover
page/context retirement and reservation/owning-handle lifetimes; the final
reservation revision repeats cancellation under Miri. Actual WASI executes the
reservation case plus 21 root/context/reference tests, and the final revision
passes WASI Clippy for all targets and a bare-Wasm smoke build. Native-only lint
expectations are now conditional on the target. The initial WASI unwind probe
aborted as expected on its panic-abort target; only the explicit unwind subcase is
now conditional, while rejection and cancellation remain tested there.

The read-view-only experiment completes 840 processes with seven pairs, six
mechanisms, CPU 0 and 100,000 operations per process. Candidate operation-time
intervals include one for allocation, roots, handles, weak upgrade and guard
cloning. Root wall time rises about 1% [0.2%, 3.2%]. Adding reservations initially
raises root operation time 23%. Removing hot-path diagnostic counter updates
reduces that overhead in a subsequent 840-process comparison, but does not recover
the prior performance. Versus the read-view revision, candidate ratios are:

| Mechanism | Before / after ns | Paired after/before, 95% interval |
| --- | ---: | ---: |
| Allocation | 282.89 / 295.47 | 1.044 [1.028, 1.049] |
| Roots | 78.44 / 90.91 | 1.158 [1.134, 1.166] |
| Handles | 103.87 / 116.54 | 1.118 [1.112, 1.137] |
| Weak upgrade | 127.79 / 139.75 | 1.081 [1.073, 1.107] |
| Guard clone/drop | 17.96 / 17.91 | 0.999 [0.990, 1.003] |

All processes validate and frozen binaries remain unchanged. Control timing
shifts +13.2% [11.7%, 15.8%], limiting attribution of small changes. This is retained
as an experimental progress foundation with a measured cost, not a throughput win.
Default storage/publication choices remain unchanged. Root deletion, compatibility
tree roots and admitted metadata/queue waits still prevent the overall progress goal.

The final 60-second payload run finishes 600,000/600,000 arrivals, zero final backlog
and clean teardown including new resident-page and reservation assertions. Operation
p99.9/p99.99 is 68/494 µs, arrival p99.9/p99.99 is 434/1,072 µs, and charged process
peak is 28.7 MiB with CPUs 0/2, quota 2, managed 64 MiB and process 512 MiB caps.
This is neither a repeated endurance qualification nor an external-JVM comparison.

Evidence: `target/gc-implementation/root-read-view-{before,accounted}/`,
`mechanisms-root-read-view/`, `root-reservation-{after,no-counter}/`,
`mechanisms-root-reservation/`, `mechanisms-root-reservation-no-counter/`,
`endurance-root-reservation.json` and `pressure-root-reservation/`. Source archives,
raw samples and SVG/PNG graphs retain the costly prototype as well as the revision.

## Cooperative Wasm candidate policy retained (2026-11-08)

The `gc_compare` Wasm candidate disables adaptive workers, independent marking,
phase/relocation claims and stealing queues, and uses one metadata/region shard.
Counted publication, context roots/buffers, local allocation, prediction,
generation overlap, mark batching and guarded compaction remain enabled. This
configures the existing collector for threadless execution; no collector algorithm
or library default changes. Native comparison policy remains distributed.

`cooperative-policy-{before,after}/` freezes both WASI binaries after the retained
pending-transfer fix, isolating this policy change. All 120 Java processes pass
with unchanged inputs: five randomized pairs, allocation/roots/mixed workloads,
saturation and 1,000 scheduled arrivals/s, two-second measurements after one-second
warmup, CPUs 0/2 with two cores of quota, 64 MiB managed and 512 MiB process caps.
Every fixed-rate process finishes its offered work. Wasm Clippy passes.

Candidate saturation throughput ratios after/before are allocation 1.079
[1.074, 1.093], roots 1.118 [1.104, 1.154], and mixed 1.070 [1.065, 1.105].
Medians change from 79,630 to 86,124/s, 6,376 to 7,187/s and 71,306 to 76,896/s.
Unchanged baseline controls have throughput intervals including one. Root
saturation charged process peak falls about 5%; fixed-rate mixed peak increases
4.2% [2.5%, 7.5%]. Fixed-rate arrival-tail intervals are broad (only 2,000
completions/run), and the roots candidate still trails baseline throughput.
This supports experimental throughput tuning, not combined latency/memory or
G1/ZGC superiority qualification.

The browser candidate also drops independent phase/mark claims and stealing
queues; it already uses one metadata/region shard and no workers. All nine
Chromium trials (three forks per policy, ten seconds at 10,000 arrivals/s after
two-second warmup) finish all 100,000 requests with unchanged inputs and clean
content, reference, finalization and teardown checks. Candidate request-turn
p99.9 is 16–20 µs, baseline 20–21 µs and control 6 µs; arrival p99.9 is
4.7–5.0 ms across policies. Charged browser peak is 393–399 MiB under the same
512 MiB process cap. This is a current-policy screen, not a paired browser
revision comparison or an endurance qualification. Frozen Wasm/source evidence
and graphs are in `cooperative-browser-after/` and `browser-cooperative-policy/`.

`benchmarks/gc/revisions.py` now validates complete matching pairs, rejects failed,
duplicate/missing/mismatched-budget samples, suppresses censored or zero-resolution
latency ratios, and freezes its analysis sources. All 35 Python benchmark tests
pass, including six new comparison-integrity cases. Reports and SVG/PNG graphs
are in `target/gc-implementation/java-wasi-cooperative-policy/`,
`cooperative-policy-candidate-comparison/` and `cooperative-policy-baseline-comparison/`.

## Pending-reference transfer releases outer admission (2026-11-08)

Dequeuing now moves the queue's existing root into a stack removal guard. The
graph remains rooted through validated acquisition; success, error and unwinding
release the old registration. Queue waiting and owning-record allocation no
longer hold an outer publication admission. A forced pause after dequeue permits
three full collections and unrelated reclamation while preserving the owner and
child graph under both policies. Rejected-type cleanup covers all three root stores.

All 372 GC tests, native VM GC/reference checks, GC/classloader/types/VM Clippy,
two Miri queue-lifetime cases, seven actual WASI reference tests and bare-Wasm
smoke pass. `pending-transfer-{before,after}/` freezes matching benchmark fixtures;
the prior collector is reconstructed from `root-guard-final/` with only the fixture
overlaid and all other source hashes verified. All 700 processes in
`mechanisms-pending-transfer/` pass. Candidate empty poll changes from 21.39 to
11.74 ns, ratio 0.555 [0.541, 0.574]; populated transfer changes from 156.32 to
149.39 ns, ratio 0.959 [0.926, 0.962]. Control shifts 21%, so unrelated allocation
and weak timing changes are not attributed to this patch. Retention is supported
by the forced progress schedule; no sustained or external win is claimed.

## External comparisons refreshed through root-guard optimization (2026-11-08)

`root-guard-final/` freezes native/WASI JVMs, the browser fixture and payload load
drivers with full source archives. The Java screen uses five randomized process
pairs, two-second measurements after one-second warmup, allocation/roots workloads,
saturation/5,000 arrivals per second, CPUs 0 and 2, two cores of quota, 64 MiB
managed heaps and 512 MiB process caps. It includes interpreter and restricted-JIT
Ristretto modes plus HotSpot G1/ZGC on the same JDK 25 class library. All 120 native
and 40 WASI processes pass integrity checks; none overlaps builds or tests.

Native candidate interpreter medians are 171,056 allocation operations/s and
10,126 roots operations/s. G1 records 25,535,588 and 15,381,422; ZGC records
30,703,440 and 15,128,520. Candidate JIT medians are 155,273 and 9,613. Both candidate
modes fail all twelve short performance comparisons and each records 52 total
gate failures including inadequate extreme-tail sample counts. The whole-JVM gap
remains orders of magnitude; collector mechanism improvements do not establish
external superiority. Every native fixed-rate process completes its offered work.

WASI candidate medians are 79,553 allocation operations/s and 6,267 roots
operations/s, versus baseline 86,380 and 8,693. At 5,000 roots arrivals/s, all five
baseline trials and four candidate trials leave unfinished requests. Those tails
remain censored. Nine actual browser trials each complete 100,000/100,000 requests
with clean content/finalization/teardown. Candidate request-bearing-turn p99.9 is
16–20 µs, baseline 16–26 µs, control 6 µs; arrival p99.9 is about 4.8–4.9 ms across
all policies. These remain short collector-embedding trials, not browser JVM runs.

The separate sixty-second payload run completes 600,000 arrivals, with zero
backlog and clean managed/root/context/cache/retired teardown. Operation
p99.9/p99.99 are 57/504 µs; charged process peak is 32.4 MiB. All recorded inputs
remain unchanged. Browser graph export initially failed because the orchestration
script resolved the plotting venv's interpreter symlink to system Python. Graphs
were then generated from the unchanged saved data with the correct environment;
no measurement was rerun or discarded.

Evidence and SVG/PNG graphs: `target/gc-implementation/java-native-root-guard/`,
`java-wasi-root-guard/`, `browser-root-guard/`, `endurance-root-guard.json` and
`pressure-root-guard/`. Candidate-specific gate reports are retained separately.
These snapshots precede the subsequent pending-reference transfer change.
Hour/day endurance, no-global-blocking and external superiority remain unqualified.

## Root-guard duplicate barrier removed (2026-11-08)

Guard destruction suppresses an inner edge barrier only when its allocation ID
and collector match the owned registration. Root removal retains the snapshot
barrier, while replacement/foreign edges keep their ordinary drop behavior.
`into_rooted` moves both fields exactly once without running the new destructor;
an unwind test covers failed replacement transfers across all root stores and
both publication policies. The detached-buffer test now expects one root-deletion
record without a redundant coalesced edge record and still verifies reachability.

All 370 GC tests, GC/classloader/types/VM Clippy, native VM GC/reference checks,
three Miri transfer/unwind cases, fourteen actual WASI root/context tests and
bare-Wasm smoke pass. All 1,260 mechanism processes pass. Candidate guard clone/drop
changes from 27.49 to 18.16 ns, paired ratio 0.661 [0.655, 0.672]; temporary
allocation changes from 289.45 to 280.29 ns, ratio 0.962 [0.954, 0.971]. Owning
allocation, owning clones, roots, handles, unroot and barrier intervals include
one. Control ratio is 1.011 [1.0003, 1.037]. The new `guard_clones` fixture is
identical in both source archives; all other baseline source hashes are verified
against the prior identity-admission snapshot. Frozen sources, raw measurements
and graphs are under `target/gc-implementation/root-guard-barrier-{before,after}/`
and `mechanisms-root-guard-barrier/`. Existing load/external results below precede
this change; no sustained or G1/ZGC qualification is claimed.

## Identity lookup outside root admission retained (2026-11-08)

`from_handle_rooted` resolves the identity index before entering publication
admission. The result is only an address hint: the expected nonreused ID, payload
type and logical eligibility are checked again under admission before registration.
A forced schedule pauses after lookup under both policies; three full cycles
finish, and resumed reconstruction either retains the still-live graph or rejects
the expired address. Object-shard/root-installation waits remain admitted.

The revision passes 369 GC tests, GC/classloader/types/VM Clippy, native VM
GC/reference checks, Miri's expired/foreign/wrong-type handle matrix, three actual
WASI handle tests and bare-Wasm smoke. All 560 mechanism processes pass. Candidate
handle time changes from 105.7 to 102.9 ns, paired ratio 0.972 [0.970, 0.986];
allocation and root intervals include one. Frozen sources and evidence are in
`target/gc-implementation/identity-admission-after/` and
`mechanisms-identity-admission/`. The latest load check below precedes this small
change; no external or endurance qualification is claimed.

## Barrier working sets and optional-cache isolation retained (2026-11-08)

The completed-edge cache retains its two recent entries and adds four sets of four
exact records. After 64 extended-cache misses it bypasses that extension for 1,024
non-recent lookups, then probes again. All access uses a separate try-lock. A
held optional cache cannot block required-record draining or acknowledgement;
dirty notifications, epoch checks and rollover disabling are preserved. Native
context storage grows from 5,376 to 6,144 bytes, reported in diagnostics.

All 368 GC tests pass, including forced full collection while the optional cache
is held. GC/classloader/types/VM Clippy, native VM GC/reference checks, the held-cache
Miri test, eight actual WASI cache/epoch tests and bare-Wasm smoke pass. The benchmark
harness has 29 passing tests. New workloads cycle through 4/16/64 live targets;
the contention driver validates its configurable shared working set.

All 1,120 final mechanism processes pass. Candidate operation ratios are 0.963
[0.950, 0.979] for one edge, 0.338 [0.329, 0.342] for four, 0.629 [0.598, 0.814]
for sixteen and 1.011 [1.000, 1.017] for sixty-four (the unrounded lower bound is
0.99952). Allocation, roots and overwrite intervals include one. Control varies
about 7%. The initial fixed extension regressed sixty-four-edge time by about 8%;
its evidence is retained separately. These remain exploratory process-pair results.

All 120 final contention processes finish 1,024 cycles within their ten-second
deadlines, across both 16/64 shared targets, four producers, both generations and
three policies. Broad paired intervals do not establish a sustained reclamation
speedup. The 60-second final load check completes 600,000/600,000 arrivals with zero
backlog and empty managed/root/cache/retired teardown. Operation p99.9/p99.99 are
56/507 µs; charged process peak is 30.0 MiB. It does not qualify hour/day endurance.

Frozen sources, raw data and graphs are under `target/gc-implementation/`:
`barrier-working-set-before/`, `barrier-working-set-after/`,
`barrier-working-set-adaptive-after/`, `barrier-cache-isolation-candidate/`,
`mechanisms-barrier-working-set/`, `mechanisms-barrier-working-set-adaptive/`,
`mechanisms-barrier-cache-isolation/`, `contention-barrier-working-set16/`,
`contention-barrier-cache-isolation16/`, `contention-barrier-cache-isolation64/`
and `endurance-barrier-cache-isolation.json`. G1/ZGC and browser comparisons below
precede this change and remain unqualified.

## Weak-slot waits outside publication (2026-11-08)

Weak upgrade, clone and owner assignment release admission before blocking on a
slot, then retry the decision under fresh admission. New/clone slot allocation
also precedes admission; birth epochs are sampled afterward. Forced schedules
cover three waiting operations under both policies while unrelated young
reclamation finishes, and verify that a weak boundary crossed during the wait
still makes upgrade/clone fail. Native GC tests pass 364 cases after the related
owner-binding correction, with GC/classloader/types/VM Clippy and native VM
GC/reference checks passing.

Owner identity binding now follows descriptor publication and admission release.
The pending-owner count and temporary roots survive until binding and scope cleanup.
Reference enumeration checks that count before the slot try-lock, then rechecks
under the lock. A forced schedule completes three full cycles while binding is
blocked, reclaims unrelated allocations and preserves the referent's child graph
under both policies. The updated Loom reference model passes queue-ordering cases.
Four Miri weak tests and the construction/collection/unwind matrix pass, as do
twelve WASI reference unit tests, seven WASI integration tests and bare-Wasm smoke.
Held non-construction slots and stalled successful admissions remain
progress limitations.

The initial 700-process mechanism screen found a small weak-upgrade regression
from an out-of-line lock helper. Separating its cold wait removes the clear
regression in another 700 successful processes: candidate weak-upgrade ratio
0.991 [0.938, 1.036], roots 0.996 [0.981, 1.006], barriers 1.003 [0.958, 1.010].
These timings precede the final owner-binding move; they are not qualification.
Evidence is under `target/gc-implementation/weak-admission-after/`,
`weak-admission-inline-after/`, `mechanisms-weak-admission/` and
`mechanisms-weak-admission-inline/`.

The final owner-binding revision is frozen in `weak-publication-final/`. All 700
processes in `mechanisms-weak-publication-final/` pass. Candidate operation ratios
are allocation 0.996 [0.986, 1.009], weak upgrade 0.997 [0.985, 1.013], barrier
1.003 [0.984, 1.016] and roots 1.006 [1.004, 1.031]. The small root increase
coincides with a 9% control shift; no overall throughput win is claimed. Retention
is justified by the forced progress schedules, not an across-the-board speedup.
`endurance-weak-publication-final.json` completes 600,000/600,000 arrivals in 60
seconds with zero backlog, empty managed/root/cache/retired state, operation
p99.9 57 µs, p99.99 497 µs and 32.2 MiB charged process peak. Source/binary inputs
remain unchanged. These are short screens; G1/ZGC and hour/day gates remain open.

## Weak identities and strong-only metadata retained (2026-11-08)

Conditional edges now retain the original, nonreused target ID instead of a weak
metadata reference. A stale edge cannot bind to a replacement at the same address,
including when a new weak handle is constructed after address reuse. Registry
validation, per-slot decisions, phase eligibility and root admission remain in
place. Internal metadata references use `triomphe::Arc`; no payload lease or graph
root is inferred from metadata ownership. `Gc` layout is unchanged.

The change passes 361 native GC tests, GC/classloader/types/VM Clippy, native VM
GC/reference tests, eight Miri weak/storage tests, ten actual WASI weak/reference
tests and executed bare-Wasm smoke. All 1,680 paired mechanism processes pass.
Candidate weak upgrade time changes from 143.9 to 127.8 ns, paired ratio 0.881
[0.879, 0.898]. Allocation, roots and barriers show no clear change. Control varies
about 7%; these are isolated mechanism results, not whole-runtime qualification.

Forty native allocation-count processes pass across ephemeral and weak-retention
modes. Candidate requested bytes fall from 436.338 to 428.338 per ephemeral
allocation, with unchanged allocation count and no clear process-peak improvement.
Keeping cleared weak handles alive retains 126.631 rather than 342.631 counted
bytes per operation. These totals include all counted live allocations, not managed
live bytes. Eight actual WASI count runs verify teardown and show about 120 fewer
retained bytes per cleared weak handle; ephemeral requested bytes are unchanged
there. Native's eight-byte allocation saving does not carry over to Wasm alignment.

Frozen sources, raw reports and graphs are under
`target/gc-implementation/strong-metadata-before/`, `strong-metadata-after/`,
`allocations-strong-metadata/`, `weak-retention-strong-metadata/`,
`mechanisms-strong-metadata/` and `strong-metadata-wasi-counts/`.
The G1/ZGC, browser and hour/day qualification gates remain outstanding.

The separately frozen `strong-metadata-final/` load check, before the subsequent
weak-slot wait changes, completes 600,000 arrivals in 60 seconds with zero final
backlog and empty managed/root/cache/retired state. Operation p99.9 is 57 µs and
p99.99 is 496 µs; charged process peak is 29.7 MiB. See
`endurance-strong-metadata.json`. This one short trial does not establish an
endurance or memory-improvement result.

## Batched object identities retained (2026-11-08)

Each thread reserves 64 process-unique IDs at a time. Unused IDs are never returned,
partial terminal ranges remain usable and exhaustion cannot wrap or invalidate
another thread's reservation. The implementation passes 360 GC tests,
GC/classloader/types/VM Clippy, native VM GC/reference tests, four Miri cases,
three actual WASI identity tests and executed bare-Wasm smoke. The Python benchmark
suite passes 28 tests. ID ranges contain no heap/context ownership.

All 42 isolated ID processes pass, with seven pairs, 1/2/4 producers, four CPUs
and a 256 MiB cap. Four-producer wall ns/ID changes from 15.350 to 2.164, with paired
ratio 0.143 [0.101, 0.177]. One- and two-producer ratios also favor batching. This
includes timed output writes and is not a collector throughput measurement.
All 1,680 operation-screen processes pass: general allocation, roots and barriers
show no clear change; owning allocation has a small improvement. Control varies
about 9%, so small effects remain inconclusive. The change is retained for reduced
shared-counter contention, without a Java throughput superiority claim.

The 60-second final load check completes 600,000/600,000 arrivals, zero backlog,
empty managed/root/cache/retired state and 30.6 MiB process peak. It does not meet
hour/day qualification. Evidence is under `target/gc-implementation/identity-batch-before/`,
`identity-batch-after/`, `identity-batch-final/`, `identities-batched/`,
`mechanisms-identity-batch/` and `endurance-identity-batch.json`. The latest G1/ZGC
and browser comparisons below precede this ID-only change and remain unqualified.

## Compact-handle experiment rejected (2026-11-08)

Two prototypes derived heap-edge identity from a strong metadata reference. The
first reduced native `Gc` from 32 to 24 bytes but raised unroot cost by about 37%.
The second stored metadata in root registrations, removed the unroot lookup and
also reduced Wasm `Gc` from 24 to 12 bytes. It passed the 356-test GC suite,
GC/classloader Clippy, native VM GC/reference tests, actual WASI root/lifetime
tests and Miri lifetime/constructor checks. Its isolated fresh-root cost rose
about 10.5%. Neither prototype is retained.

The second prototype's five-pair Java screen has 40 successful processes with
unchanged inputs. Candidate root-workload saturation throughput ratio is 0.970
[0.952, 0.985] and process RSS ratio 1.061 [1.051, 1.072]. Allocation throughput
does not show a clear change; allocation saturation RSS also rises. Saturation
root tails worsen, although the sample count is insufficient for qualification.
This fails the combined improvement objective despite the smaller handle.
Original core files were restored. Two portable lifetime regressions and the
expanded operation/revision comparison harness remain. Archives, graphs and raw
evidence are under `metadata-edge-before/`, `metadata-edge-after/`,
`metadata-root-after/`, `metadata-root-native/`, `mechanisms-metadata-edge/`,
`mechanisms-metadata-root/` and `java-native-metadata-root/`.

## Refreshed native, Wasm and browser comparisons (2026-11-08)

The final edge-certification implementation is frozen in `barrier-final/` under
`target/gc-implementation/`. All 120 native Java processes and 40 WASI processes
complete without execution failures or changed inputs. Native candidate allocation
throughput is 172,043/s versus G1 25,557,413/s and ZGC 30,503,809/s; root workload
throughput is 9,963/s versus 15,456,268/s and 14,868,838/s. Enabling Ristretto's
restricted JIT does not close this gap. Both execution modes fail all twelve short
comparison performance gates; low sample counts add further qualification failures.
These are whole-runtime measurements, not isolated collector rankings.

WASI candidate saturation throughput is 81,034/s for allocation and 6,297/s for
roots, below the baseline's 87,457/s and 8,683/s. Some 5,000/s root trials in both
policies leave scheduled work unfinished; their latency is censored. Nine sandboxed
Chromium trials complete all 100,000 arrivals each. Candidate request-turn p99.9
is 20–25 µs versus baseline 20 µs, with arrival p99.9 around 4.8–4.9 ms. A separate
60-second native check completes all 600,000 requests, has no final backlog or
managed/root/cache/retired state, and peaks at 34.2 MiB process memory. None of these
short runs qualifies the hour/day endurance or no-global-blocking goals.

Reports, raw results, graphs and provenance are in `java-native-barrier/`,
`java-wasi-barrier/`, `browser-barrier-final/` and `endurance-barrier-final.json`.

## Read-only edge certification (2026-11-08)

The retained protocol passes 354 GC tests, including 25 production-state Loom
models, GC/classloader/types/VM Clippy, native VM GC/reference tests, four Miri
cases, eight actual WASI tests and bare-Wasm smoke. New forced schedules pause a
read-only check across three full cycles and complete a context-cache hit while
the object shard is held. Tests cover dirty notifications, object-epoch rollover,
phase ABA, numeric revision reuse and full/young reclamation with four producers.
Sources and binaries are frozen in `target/gc-implementation/barrier-certification/`
and the final routing-only revision in `barrier-dispatch/`. The latter passes the
354-test native suite and Clippy; its paired routing screen has 420 successful
processes with no clear barrier-time change.

The five-pair, four-producer contention screen completes all sixty processes and
all 1,024 cycles per process, with two CPUs and 256 MiB process caps. Candidate
completion-time ratios are 0.192 (full) and 0.138 (young), with exploratory paired
95% intervals wholly below one. The 1,260-process mechanism screen shows lower
counted barrier and candidate overwrite cost. Both screens retain raw data and
graphs. Small operation differences are inconclusive given control variation.
The preceding admitted-only revision missed three ten-second deadlines; those
censored results remain in `contention-barrier-progress/`. None of these screens
qualifies the G1/ZGC comparison, sustained load, or elimination of all global
blocking. See [implementation evidence](../benchmarks/gc/IMPLEMENTATION.md).

## Root ownership counter (2026-11-08)

Root registrations use a reference counter without weak ownership; metadata and
collector weak lifetimes keep the existing standard-library protocol. The retained
revision passes 341 GC tests, GC/classloader/types/VM Clippy, native VM/reference
tests, two Miri lifetime cases, thirteen actual WASI tests and bare-Wasm smoke.
Evidence is frozen under `target/gc-implementation/strong-root/`. The 1,260-process
`mechanisms-strong-root/` screen shows lower candidate root/handle/weak/allocation
cost with owning-clone timing essentially unchanged. Twenty allocation-count
processes in `allocations-strong-root/` show eight fewer requested bytes per
operation with unchanged allocation counts and process peak. These screens do not
meet the Java comparison or endurance qualification gates.

## Reference-free write barriers (2026-11-08)

Managed writes always run their unroot hook, then skip owner notification when the
type declares no references or the current value explicitly proves no relevant
edges/actions. Java primitive-array variants use this per-value check. Reference
variants remain conservative. The GC suite passes 341 tests; GC/classloader Clippy,
native VM GC/reference tests, six actual WASI publication tests and bare-Wasm smoke
pass. Miri passes the dynamic leaf-to-reference transition. Forced tests complete
scalar and empty-value writes with the legacy publication writer held, preserve
unroot hooks, and retain a young edge newly inserted into an old formerly empty
owner. A classloader test confirms that a primitive write during partial relocation
still causes recopying and preserves all updated values.

The extended `leaf_write` fixture is compiled against both the frozen preceding
source and the new implementation. Seven paired forks, nine workloads and 1,260
processes all pass with unchanged binaries. Candidate leaf-write operation cost
is 66.1 → 14.1 ns (paired ratio 0.216, 95% interval 0.213–0.217), with wall ratio
0.251 [0.248, 0.254]. The other operation intervals mostly include one; control
changes about 7%, so small changes are inconclusive. This is a collector mechanism
result, not a Java VM or G1/ZGC throughput claim. Evidence is under
`target/gc-implementation/leaf-barriers/`, `leaf-before-source/`,
`mechanisms-leaf-barriers/` and the `*leaf-barriers*` logs.

## Shared type information (2026-11-08)

The retained implementation passes 339 GC tests, all-target Clippy, native VM GC
and the installed JDK reference matrix. New tests distinguish plain/finalizable
allocations of the same Rust type, reject wrong-type handles, retain exactly-once
finalization after a caught native panic, and race four finalizer claimants. Both
tests pass Miri. The portable type/finalizer test executes under WASI; its panic
branch is skipped because these Wasm targets abort rather than unwind. Bare-Wasm
smoke passes. Logs are `tests-shared-type-final.log`, `clippy-shared-type-retained.log`,
`tests-shared-type-vm.log`, `miri-shared-type.log`, `shared-type-wasi-executed.log`
and `shared-type-bare-executed.log` under `target/gc-implementation`.

Frozen `shared-type/` binaries/source and `allocations-shared-type/` show 16 fewer
requested allocator bytes per object, unchanged allocation counts, and about 4%
lower candidate process peak in the 100,000-object fixture. All twenty processes
pass. Seven-pair timing comparisons across eight mechanisms also pass all 1,120
processes (`mechanisms-shared-type/`); allocation/root/identity timings show no clear
operation-time change. The retained revision has no deferred-root batching option:
both tested batching designs were removed after regressions.

## Allocation outside publication admission (2026-11-08)

Metadata `Arc` allocation now precedes registration admission. Typed identity and
weak acquisition keep an already-registered root on the stack, release admission,
then allocate its shared owner. The 337-test GC suite, all-target Clippy, native VM
GC/reference matrix, five actual WASI publication tests and bare-Wasm smoke pass.
Miri passes both new regressions: graph retention/cancellation across unrelated
cycles and paused metadata preparation with construction edges. These changes do
not remove admission from registry insertion or solve a stalled admitted publisher.

Seven paired forks across eight mechanisms (1,120 processes) all pass, with frozen
inputs unchanged. Candidate allocation/root timing is broadly unchanged; identity
conversion is 4.6% slower (paired 95% interval 4.0–5.9%). This is a progress tradeoff,
not a timing win. Evidence is under `target/gc-implementation/prepared-root/`,
`mechanisms-prepared-root/`, `tests-prepared-root-vm.log`,
`prepared-root-wasi-tests.log` and the `miri-prepared-root*` logs.

The corrected browser screen separately samples turns that execute request polls.
All fifteen trials again complete 100,000 measured requests and clean teardown.
Request-turn p99.9 is 16–25 µs for candidate, 20–21 µs for baseline and 6–10 µs for
observer control. Idle turns remain in the raw all-turn channel; maintenance-only
GC work is excluded from the request-turn percentile. Approximately 5 µs clock
increments limit resolution. Raw data, source/binary provenance and graphs are in
`target/gc-implementation/browser-request-turns/`. Qualification remains false.

## Compact metadata, root admission and browser screen (2026-11-08)

The retained core revision passes 335 GC tests and all-target GC Clippy with
warnings denied. Its two added regressions observe exactly one publication
admission across identity/weak validation and root insertion, and reject a token
from another collector. The native VM GC and installed reference matrix pass.
Actual WASI executions pass four publication regressions, seven root-slot tests
and three leaf tests. The legacy unwind fixture compiles with zero tests on WASI;
it is not counted as runtime coverage there. Bare-Wasm smoke passes.

Miri passes the constructor/unwind matrix after compact metadata and the owning/
edge/identity lifetime regression after admission reuse. The final split token
changes only which guards carry the collector identity; it passes the full native
suite and actual WASI publication regressions. Existing integer-address APIs still
emit Miri provenance warnings. No new ThreadSanitizer result is claimed here.

The new browser example passes its native lifecycle test and Clippy. Twenty-two
benchmark-tool tests and two pressure-tool tests pass. Real sandboxed Chromium
trials use bare Wasm, an isolated clock context, two CPUs and a two-core quota,
512 MiB process-tree memory, and a 16 MiB managed limit. Fifteen randomized trials
(five each for baseline, candidate and observer control) complete all 100,000
measured requests per ten-second run at 10,000/s. Warmup is two seconds. Every
trial verifies original payload values, weak clearing, exactly-once finalization
and zero final managed/root/retired/barrier counters. Frozen inputs remain unchanged.
This exercises the Rust collector in a browser, not the Java VM in a browser.

Operation and memory evidence, including rejected owner-slot experiments, is in
[the implementation report](../benchmarks/gc/IMPLEMENTATION.md). Logs and frozen
sources/binaries are under `target/gc-implementation`: `tests-root-admission.log`,
`tests-root-admission-vm.log`, `root-admission-wasi-*.log`, `bare-root-admission.log`,
`miri-compact-metadata-final.log`, `miri-shared-admission.log`, and `browser-screen/`.
These ignored artifacts must be preserved before cleaning. The browser screen is
short, lacks a million completions per fork, and does not establish calibrated
latency budgets, a long-run memory plateau, no-global-blocking or G1/ZGC superiority.

## Follow-up progress and ownership checks (2026-11-08)

The ownership follow-up snapshot passes 333 GC tests and GC all-target Clippy
with warnings denied. Native VM GC tests and the installed five-JDK reference
matrix pass. Bare-Wasm `gc_smoke` executes in Wasmtime 50.0.1. WASI executions
cover `leaf_progress`, `root_slots`, `reference_construction`, and
`clearing_progress`, including actual successful conditional-owner construction
on the aborting target. Miri passes all three leaf-progress tests and the
owning/edge/identity-reconstruction lifetime regression. Existing integer-address
APIs emit Miri provenance warnings; these checks do not establish strict-provenance
compatibility. Nineteen benchmark protocol tests and two pressure-runner protocol
tests pass.

Leaf tests hold payload write locks while old, young and three full collections
finish, verify that locked storage does not move, and preserve bounded element
destruction. Root conversion tests cover all three root-storage policies and both
publication policies, escaped contexts, weak upgrades, identity reconstruction and
last-owner collector teardown. The earlier construction/callback and allocation-owner
revisions have separate Miri and measurement evidence in
[the implementation report](../benchmarks/gc/IMPLEMENTATION.md).

Commands and logs are under `target/gc-implementation/owner-commands.json`,
`tests-owner-final.log`, `clippy-owner.log`, `tests-owner-vm.log`,
`miri-owner-{roots,leaf}.log`, `bare-owner.log`, and `wasi-owner-*.log`.
These ignored artifacts must be preserved before cleaning. No new ThreadSanitizer,
browser or long-duration qualification result is claimed by this section.

Results are grouped by implementation snapshot. The newest sections cover native async allocation and argument conversion, bounded preparation, logical recovery, distributed relocation, physical recovery, JIT allocation, worker pacing, and local caches; subsequent sections describe earlier implementation snapshots.

This records validation of the issue #940 implementation on 2026-11-06. It is evidence for the listed invariants and workloads, not JVM conformance certification or a hard pause-time guarantee. See [ARCHITECTURE.md](ARCHITECTURE.md) for the implementation contract.

## Sharded young and remembered indexes (2026-11-07)

Source `beae30ff8eb4e244224d3f8b9fd7cfa3b6ac50826705e813b8f79b3827a2633f`
extends `metadata_shards` to young membership and remembered owners. Separate
bounded cursors cover young cross-snapshot, remembered, sweep and relocation
passes. Promotion, mutation-version pruning and late publication preserve their
existing ordering. Forced tests hold unrelated generation shards during a mutation
barrier, bound sparse scans, and publish after both young/full sweep cursors with
one/sixteen shards and both marking policies. Coordinator and publication-gate
scope remain unchanged.

Verification passes 239 GC tests including the documentation test, 757 classloader
tests (14 ignored), six VM GC/reference integration tests (one subprocess helper
ignored), and 91 portable WASI tests. ThreadSanitizer passes all 19 recovery tests
in 413.20 seconds and 12 generational tests in 8.43 seconds. Clippy, formatting
and whitespace checks pass. See the
[command record and snapshot scope](validation/generational-shards-verification-2026-11-07.txt).

Eight `validation/generational-shards-*.json` files retain 48 paired runs against
preserved `efe204c…` binaries, using verified binary provenance. Settings match
the preceding native-async screen, with workers 1/2/4 for both workloads: two
reversed-order eight-second repeats, CPUs 0 and 2, compact/host storage, 16 MiB
recovery and 32 MiB mixed offered load at 10,000 operations/s. Local caches and
adaptive pacing are off; prediction, context roots/buffers, independent marking
and relocation, stealing and sixteen metadata shards are on. Diagnostics are on.
All runs exit successfully with empty stderr and zero teardown counters. Two
baseline offered-load runs end with backlogs of 19 (compact/1 worker) and 1
(host/4 workers); all candidate runs have zero backlog. No runs were discarded.

Means of recovery runs (operation quantiles in microseconds):

| Backend/workers | Allocations/s baseline → candidate | Change | p99.9 | p99.99 |
| --- | ---: | ---: | ---: | ---: |
| Compact / 1 | 386,543 → 394,228 | +2.0% | 15.5 → 15 | 759.5 → 767 |
| Compact / 2 | 380,286 → 384,067 | +1.0% | 14.5 → 15 | 657.5 → 679 |
| Compact / 4 | 372,975 → 377,667 | +1.3% | 26 → 25.5 | 619 → 641.5 |
| Host / 1 | 229,844 → 230,569 | +0.3% | 26.5 → 27 | 35.5 → 35.5 |
| Host / 2 | 229,940 → 232,082 | +0.9% | 27 → 27 | 42.5 → 42 |
| Host / 4 | 225,304 → 227,667 | +1.0% | 28 → 28 | 54 → 49 |

Offered-load arrival-to-completion quantiles in microseconds:

| Backend/workers | p99.9 baseline → candidate | p99.99 baseline → candidate |
| --- | ---: | ---: |
| Compact / 1 | 816 → 690.5 | 2280 → 1632 |
| Compact / 2 | 2344 → 458.5 | 6240 → 1529 |
| Compact / 4 | 608 → 486 | 1664 → 1027.5 |
| Host / 1 | 2288 → 2304 | 2816 → 2832 |
| Host / 2 | 2288 → 2352 | 2912 → 3168 |
| Host / 4 | 2336 → 2368 | 2864 → 2960 |

Compact recovery tails regress slightly despite higher throughput. Compact mixed
workload tails improve in this screen, while host tails are slightly worse. These
short runs do not qualify sustainable capacity, worker scaling or endurance; the
combined targets remain unmet. One metadata shard remains the default.

## Native async publication, argument conversion and logical recovery (2026-11-07)

Source snapshot `efe204c76750d0872091a7ac297ddfbae0ce5674083d7c01da833e713f984d34`
adds bounded preparation of existing Java payloads, native async object/array
publication and direct async conversion of fourteen standard Rust vector types.
Scalar and custom embedding conversions retain their synchronous compatibility
hook. Cancellation preserves source ownership and exactly-once destruction; a
truncation regression verifies destructor unwinding cannot leave an initialized
length that exposes or drops the removed suffix again.

The full GC suite exposed premature logical OOM under allocation competition.
Size-indexed observations now register before requesting/joining a cycle and
record sufficient headroom at the atomic refund, covering consumption before
notification or the first poll. Smaller refunds do not prolong a larger request.
The forced regressions, the 48,000-allocation native matrix, and its 354.34-second
ThreadSanitizer run pass. The physical-progress protocol remains distinct.

Verification passes 237 GC tests including the documentation test, 2,729 native
helper tests and seven registry-generation tests, 208 type-library tests, 896 VM
library tests and six VM GC integration tests (one subprocess helper ignored).
WASI checks execute the preparation/compaction/recovery regressions, all 34
Rust-value conversion tests and three actual VM GC integration tests. Miri passes
five preparation tests and two logical-observation tests. The bare-wasm collector
smoke executes, and the VM type-checks. Darwin/Windows Rust libraries type-check
with default features disabled; full-feature checks need unavailable cross C
toolchains/sysroots. No Darwin/Windows execution is claimed. Five-crate Clippy,
formatting and whitespace checks pass. See the
[command record and exact snapshot scope](validation/native-async-verification-2026-11-07.txt).

The current blocking-operation audit is in [PUBLICATION.md](PUBLICATION.md).
Publication and payload locks remain; these changes do not establish gate removal
or a hard latency bound. The endurance workflow now includes preparation, vector
argument conversion and exhausted-heap native allocation checks; it was not
remotely dispatched.

The paired collector/array-driver screen compares the preserved `37513ab…`
binaries with this candidate. Eight `validation/native-async-*.json` files retain
all 40 runs: two reversed-order eight-second repetitions, CPUs 0 and 2, compact and
host storage. Recovery uses workers 0/1/4, 16 MiB, 4 KiB payloads and 512 roots;
offered load uses workers 1/4, 32 MiB and 10,000 operations/s. Context roots,
32-record barriers, prediction, independent marking/relocation, eight-task marking,
sixteen metadata shards and stealing are enabled; local caches and adaptive pacing
are disabled. Diagnostics are enabled. Baseline binary hashes were checked against
saved provenance; its runner source hash is recorded separately. No builds or tests
ran concurrently. Every run exits successfully, all teardown checks are zero, and
all offered-load runs finish with zero backlog. This measures collector and Java
array paths; it does not measure the throughput of each migrated native intrinsic.

Means of recovery runs (operation quantiles in microseconds):

| Backend/workers | Allocations/s baseline → candidate | Change | p99.9 | p99.99 |
| --- | ---: | ---: | ---: | ---: |
| Compact / 0 | 486,796 → 495,188 | +1.7% | 20.5 → 19.5 | 1072 → 1056 |
| Compact / 1 | 423,492 → 429,213 | +1.4% | 12 → 11.5 | 635 → 651 |
| Compact / 4 | 403,450 → 410,170 | +1.7% | 20.5 → 19 | 534 → 540.5 |
| Host / 0 | 254,999 → 253,688 | −0.5% | 21 → 23 | 150 → 152.5 |
| Host / 1 | 243,220 → 246,865 | +1.5% | 20 → 16 | 33 → 31.5 |
| Host / 4 | 239,099 → 238,214 | −0.4% | 22.5 → 23 | 44.5 → 47 |

Offered-load arrival-to-completion quantiles in microseconds:

| Backend/workers | p99.9 baseline → candidate | p99.99 baseline → candidate |
| --- | ---: | ---: |
| Compact / 1 | 841.5 → 538 | 2928 → 1144 |
| Compact / 4 | 769 → 553 | 2264 → 1147 |
| Host / 1 | 2400 → 2320 | 3232 → 2896 |
| Host / 4 | 2288 → 2320 | 2816 → 2848 |

These short runs are a regression screen, not capacity or endurance qualification.
Some tails improve, others regress; the combined throughput/latency gates remain
unmet. No experimental policy default changes.

## Distributed relocation, physical competition and JIT allocation (2026-11-07)

Source snapshot `37513abdd3d1aef5bde2cfb1ad37574bf2016ce7f3900182568bc9a98e693f4a`
adds independent relocation queues and budgeted callback batches. Tests cover held
coordinators, abandoned worker queues, overflow, stealing, work budgets, actual
callback overlap, generation closure, pinning, and the 90% equal-size compaction
fixture under both policies. Completion retains an in-flight reservation across
all queue handoffs in a batch. The publication gate remains in use.

Pressure testing exposed premature physical OOM when competing allocations consumed
reclaimed storage before a waiter retried. Physical recovery now observes returned
ranges and shared backing refunds. Two consecutive cycles without physical progress
still fail; ongoing competition can extend recovery. A deterministic regression
steals each returned range, then stops reclamation to check bounded failure and
exact teardown. The native 48,000-allocation matrix and its 405.91-second
ThreadSanitizer run pass after the fix. JIT object, array, multidimensional-array,
and sentinel allocation now use async recovery through the existing runtime bridge.
The invoking compiled frame still waits; the executor worker can run other tasks.
Exhausted-heap tests preserve pending exceptions with zero inline full recoveries.

Verification passes 230 GC tests including the documentation test, 896 VM library
tests, six VM GC integration tests (one subprocess helper ignored), 81 portable
WASI tests, and Clippy for GC/classloader/VM tests and examples. Miri covers the
three relocation handoff/budget tests and the physical-competition regression;
ThreadSanitizer also passes all twelve compaction tests. See
[commands and intermediate-failure scope](validation/relocation-jit-verification-2026-11-07.txt).
These checks supplement the protocol argument; they do not establish gate removal.

The screen uses two reversed-order eight-second repetitions, workers 1/2/4, and
CPUs 0 and 2. Recovery uses 16 MiB, 4 KiB payloads and 512 retained roots; the
mixed offered-load workload uses 32 MiB and 10,000 operations/s. Both enable
context roots, 32-record buffers, prediction, independent eight-task marking,
sixteen metadata shards and stealing. Local allocation caches and adaptive pacing
are disabled to isolate relocation distribution. Diagnostics are enabled.
Six `validation/relocation-{recovery,endurance}-*.json` files retain all 36 runs,
commands, hashes, distributions, CPU and RSS. Every run exits successfully and
passes zero-object/range/queue/cache/retirement teardown checks. Offered-load runs
all finish with zero backlog; this fixed rate does not establish capacity.

Means of the two recovery runs (operation quantiles in microseconds):

| Workers | Throughput off → on, allocations/s | Change | p99.9 off → on | p99.99 off → on |
| --- | ---: | ---: | ---: | ---: |
| 1 | 409,907 → 400,588 | −2.3% | 15 → 15.5 | 306.5 → 687.5 |
| 2 | 401,508 → 392,541 | −2.2% | 15.5 → 15.5 | 305 → 608.5 |
| 4 | 382,887 → 375,690 | −1.9% | 25 → 25 | 305.5 → 576 |

Arrival-to-completion quantiles in microseconds:

| Backend/workers | p99.9 off → on | p99.99 off → on |
| --- | ---: | ---: |
| Compact / 1 | 421 → 679.5 | 1014.5 → 1552 |
| Compact / 2 | 895.5 → 991 | 2384 → 2608 |
| Compact / 4 | 662 → 607.5 | 2112 → 1808 |
| Host / 1 | 2256 → 2304 | 2784 → 2848 |
| Host / 2 | 2272 → 2272 | 2816 → 2896 |
| Host / 4 | 2368 → 2336 | 2880 → 2912 |

An initial one-callback-per-step implementation regressed recovery throughput by
20.7–24.1%. Its separate 36-run screen remains in `relocation-initial-*.json`, source
`dc95e9467d2699506db61bb606ad1a40a27cce68f4b4474b913b9afb1ad5c782`.
Batching removes most of that overhead, but recovery p99.99 still regresses and
mixed-workload latency gates remain unmet. Independent relocation stays disabled
by default. These short shared-host screens and means of per-run quantiles are
not pooled quantiles, confidence intervals, or one-hour/24-hour qualification.
The endurance workflow now separates baseline/candidate policies and includes the
combined options; it has not been dispatched from this workspace.

## Descriptor leases, adaptive workers and step progress (2026-11-07)

The integrated candidate has source SHA256
`51c8f03e7e9f3806fc3d9ebf973e5a01eb09f13774ef540fe1480d7d2741c0f0`.
Per-object storage leases now delay physical clearing/destruction after registry
retirement. Leased storage remains charged to the heap limit. Forced tests hold
borrowers across collection, reject weak upgrades and new storage leases, reclaim
unrelated objects, resume incremental clearing, wake idle workers, and retain
storage through collector teardown. Two Loom tests compile the production atomic
admission/retirement state machine with Loom atomics. The four previous buffered
publication models remain reduced models. These do not establish graph/reference
correctness without the global publication gate.

`adaptive_workers(true)` uses queued work, allocation debt, occupancy, and recovery
waiters with a CPU cap and hysteresis. Three deterministic policy tests cover
activation, pressure, decreasing demand, bounds, and contended sampling. The
48,000-allocation native matrix enables adaptive workers in context configurations
and passes under ThreadSanitizer. The policy remains disabled pending qualification.

An initial Miri lifetime run could not progress because interpreter overhead
consumed the fixture's soft wall-clock budget before its first unit. The fixture
now uses deterministic work budgets; all three lease tests pass under Miri.
The collector also now admits its first work unit before checking the deadline;
a dedicated final-candidate Miri regression verifies progress with a 1 µs target.
Exposed-provenance warnings remain; these runs are not strict-provenance proofs.
The lease Miri/ThreadSanitizer runs preceded the subsequent scheduling edits;
final-candidate suites cover their integration.

Final-candidate checks pass 225 GC tests (six Loom models), 757 classloader tests
(14 existing ignores), 205 types tests, 895 VM library tests, six GC integration
tests (one subprocess helper ignored), and 80 portable WASI tests. Bare wasm builds
and executes `gc_smoke` with Wasmtime 50.0.1. Clippy across the five affected crates'
tests/examples, formatting, and diff checks pass. See
[the verification record](validation/leases-pacing-verification-2026-11-07.txt)
for commands and scope. No one-hour/24-hour qualification has been completed.

The paired pacing screen contains 40 eight-second runs on CPUs 0 and 2: two
reversed-order repetitions per policy/backend/worker count. Recovery uses 16 MiB,
4 KiB payloads, 512 roots and 1/4 workers; mixed arrival workloads use 32 MiB and
10,000 offered operations/s. Both use context roots, 32-record buffers, prediction,
eight-task independent marking, sixteen metadata shards, and stealing; local
allocation caches are off. Diagnostics are enabled for both policies. The preserved
`0fed7c…` binary additionally measures core lease/deadline overhead separately from
adaptive pacing; baseline JSON identifies the binary's verified compilation source
and the current runner source separately. Ten `validation/adaptive-*.json` reports
retain commands, hashes, CPU/RSS, and raw distributions. All runs exit successfully
and pass zero-object/range/cache/task/retirement teardown checks.

Arithmetic means of recovery runs (operation quantiles in microseconds):

| Backend/workers | Core throughput vs preserved build | Adaptive throughput vs disabled | p99.9 off → on | p99.99 off → on |
| --- | ---: | ---: | ---: | ---: |
| Compact / 1 | −1.0% | −2.0% | 11.5 → 10.5 | 287 → 24.5 |
| Compact / 4 | −1.7% | +2.3% | 18.5 → 11 | 279.5 → 22.5 |
| Host / 1 | −1.3% | +3.4% | 15 → 17.5 | 32.5 → 50.5 |
| Host / 4 | −0.5% | +5.6% | 18.5 → 17.5 | 50 → 49.5 |

Compact recovery improves tails with adaptive pacing, with roughly 14–16% more
measured collector-step CPU. In the arrival workload all runs complete their
10,000/s offered load with zero final backlog, but compact arrival p99.9 means
worsen from 342.5 to 440 µs (one worker) and 496.5 to 1001.5 µs (four). Host arrival
p99.9 remains about 2.2 ms. These short runs miss the combined release gates;
adaptive pacing remains disabled. Means of per-run quantiles are not pooled
quantiles or confidence bounds, and fixed offered load does not establish capacity.

## Local allocation caches and descriptor slabs (2026-11-07)

Source snapshot `0fed7c26b365cd3d4fc17abbcda3246e2810539adc314c86fee6a9e05ebe7596`
adds optional context allocation ranges and logical-credit batching. Descriptor
slabs use a separate arena with the same backing budget as payload regions. An
intermediate implementation mixed pinned descriptors and payloads in regions and
failed the native pressure fixture; separating the arenas fixed that failure.
The VM logical-OOM fixture now uses an externally charged rooted host buffer to
isolate logical exhaustion from contiguous arena availability.

The full GC suite passes 217 tests, including four reduced Loom models; classloader
passes 757 with 14 existing ignores. The Java reference/pressure integration passes
three tests (one subprocess helper ignored), covering the supported library matrix
with caches enabled. Seven allocator regressions cover bounded retention, tiny
heaps, alignment, fallback, concurrent refill/reclamation, exact destruction, and
unwinding. The existing 48,000-allocation pressure matrix exercises caches in its
context configurations. Focused Miri runs pass escaped-root/detach and constructor
unwinding tests (exposed-provenance warning remains); ThreadSanitizer passes all
seven allocator tests. WASI passes 77 unit/context/compaction/cache tests. Bare wasm
builds; execution was not repeated for this snapshot. Focused Clippy across GC,
classloader, types, intrinsics, and VM tests/examples passes.

The cache benchmark uses the same release binary for enabled/disabled policies:
48 eight-second recovery runs, two repetitions in reversed policy order, CPUs 0
and 2, 16 MiB logical heap, 512 roots, one mutator, context roots, 32-record buffers,
prediction, eight-task independent marking, sixteen metadata shards, and stealing.
Raw distributions and environment/source/binary hashes are in the eight
`validation/local-allocation-{off,on}-{256,4096}-{compact,host}-2026-11-07.json`
reports. Every run exits successfully with zero remaining objects, occupied
ranges, context caches, logical cache credit, descriptor backing, and task/root
storage after teardown. Throughput changes below compare arithmetic means; tails
are means of per-run operation quantiles, in microseconds.

| Payload/backend | Workers | Throughput change | p99.9 off → on | p99.99 off → on |
| --- | ---: | ---: | ---: | ---: |
| 256 B compact | 0 | −6.8% | 22 → 4.5 | 30 → 14 |
| 256 B compact | 1 | +23.9% | 10.5 → 4 | 28 → 13 |
| 256 B compact | 4 | +23.0% | 13.5 → 4 | 34 → 14 |
| 256 B host | 0 | −29.0% | 22 → 28 | 29 → 37 |
| 256 B host | 1 | +6.1% | 11 → 10.5 | 24 → 25.5 |
| 256 B host | 4 | +3.5% | 15 → 15 | 32.5 → 34 |
| 4096 B compact | 0 | −15.0% | 19.5 → 6 | 646.5 → 3024 |
| 4096 B compact | 1 | −5.7% | 11 → 9.5 | 261.5 → 3264 |
| 4096 B compact | 4 | −3.3% | 18.5 → 8 | 251 → 3424 |
| 4096 B host | 0 | −10.0% | 22 → 30 | 141 → 169.5 |
| 4096 B host | 1 | −0.6% | 15 → 17 | 31.5 → 28.5 |
| 4096 B host | 4 | −0.4% | 19.5 → 19 | 43 → 41 |

Small compact allocations benefit with native workers, but cooperative throughput
and larger-payload tails regress. The cache policy remains off by default. These
short closed-loop screens do not qualify sustainable capacity, long-run memory
stability, or the plan's combined release gates.

## Asynchronous strings and runtime constructors (2026-11-07)

Runtime string conversion now awaits recovery for its payload, fields and final
publication. It constructs directly in the selected heap instead of staging a
complete output buffer. UTF-8 encoding/length scans and UTF-16 Latin-1 scans yield
between 64 KiB input chunks; initialization yields between 64 KiB output chunks.
The completed payload is an owning root while field construction or publication
waits. UTF-16 input preserves lone surrogates. Synchronous factory APIs remain.
String-array conversion likewise initializes managed storage directly and awaits
publication. Existing MUTF-8 decoding and host input preparation remain synchronous.

The 16 MiB VM regression keeps a 9 MiB byte array live and requests a 9 MiB string.
Before migration, conversion performs two inline full collection cycles. Afterward
it returns Java OOM through asynchronous recovery with zero added inline recoveries,
zero remaining waiters, and the original array intact, on both payload backends.
A separate factory test covers exhausted UTF-16 construction in each layout.

`Thread::object`, primordial-thread fields, module objects in both conversion
interfaces, built-in class-loader construction, and bootstrap lookup/argument
publication now use asynchronous recovery. A new test fills the managed heap with
a live payload before requesting an ordinary object. The previous infallible path
panics at the heap limit; the migrated path returns Java `OutOfMemoryError` while
preserving the live payload and budget. Existing host bootstrap argument buffers
can still require synchronous preparation. JIT helpers and other native/embedding
constructors remain migration work.

Five factory regressions cover UTF-8 scalar sequences split across scan chunks,
recovery while another executor task progresses, UTF-16 OOM, and cancellation at
every scan/initialization suspension. A held managed guard is rejected on the first
poll, before even encoding scans can suspend. The successful recovery fixture exercises
UTF-8 and UTF-16 in Java 8/9/11/16/17/25 layouts with both payload backends. Each
compact fixture verifies zero staged bytes and zero occupied ranges after release.
Existing representation tests verify empty, ASCII, Latin-1, BMP, supplementary,
NUL and lone-surrogate contents and the version-dependent hash/coder fields.

Two intrinsic test fixtures previously held managed guards across string allocation.
They now release those guards before awaiting allocation. The source-file launch
future is boxed once per launch to contain its larger composed state; the string
factory itself adds no boxed future per string.

[Before/after output and verification](validation/runtime-async-verification-2026-11-07.txt)
record the two regressions and passing suites. The final snapshot passes 205 types
library tests, six VM GC/reference tests (one subprocess fixture ignored), five
WASI string tests, native GC/classloader/types/intrinsics/VM Clippy, bare-wasm VM
compilation, formatting and diff checks. Eight compatibility comparisons also pass
against OpenJDK: interpreted/JIT lambda syntax, method references, and method-handle
constructor/constant cases. Within this batch, all 895 VM library and 2,728
intrinsic tests passed before the final early guard rejection; the VM library run
also precedes the last bootstrap-await migration. The focused VM/reference and
compatibility checks cover those subsequent changes. WASI VM compilation passes
before the guard-only change, which is covered by the final WASI factory tests.

Final implementation/test snapshot:
`cba7f01819983d2c023f46352e45efa60163fb222f7ee2a8977a1a07db383f04`.
The unchanged Java fixture hash is
`a647ac8d20caf02098bcb438b8809dd571e8dfcc70816350205ef01aa759b47a`.
These changes have no new throughput qualification; the context measurements below
retain their earlier source snapshot and do not measure these runtime constructors.

## Execution contexts, buffered barriers and pruning (2026-11-07)

Execution contexts now own root slots independently of OS threads. Their bounded,
optional barrier buffers carry active snapshot epochs, retain pending work through
cancellation and flushing, and participate in termination under the existing
publication gate. Root insertion and terminal weak/sweep barriers remain immediate.
[PUBLICATION.md](PUBLICATION.md) specifies ordering, ownership, lock order, and the
remaining requirements for removing that gate. Remembered-set pruning now checks
mutation versions and removes entries under the owner's metadata shard, without
acquiring the publication writer. A forced reader/pruner interleaving fails before
this change and passes afterward. All policies remain opt-in.

Miri also exposed an existing constructor-provenance defect: deriving a pointer
before an exclusive attachment borrow invalidated that pointer. Published pointers
now derive from the original whole-allocation pointer after attachment, preserving
the extent required for destruction. This fixes the observed Miri error; exposed
integer-pointer conversions remain and are not a strict-provenance qualification.

### Correctness evidence

[Verification output](validation/contexts-verification-2026-11-07.txt) records the
before failures and passing checks. Final snapshot:
`c2a76dcc100bc641d322abb3604e0054e9745055392cd216acf8732a30dd7489`.

- Native GC: 209 tests including four reduced Loom models and a doctest;
  classloader: 757 tests, fourteen ignored; VM GC/reference fixtures: six passed,
  one subprocess fixture ignored. The VM runs JDK 8/11/17/21/25 with both payload
  backends. The final context implementation also passed 895 VM library tests
  before the subsequent localized pruning change.
- Context tests cover nesting, foreign heaps, escaped roots, migration, suspended
  futures, cancellation, unwind, overflow, and final release. The concurrent
  pressure matrix covers 24 policy combinations and 48,000 allocations. Forced
  collector interleavings cover delayed barrier publication and dirty old owners
  during overlapping snapshots.
- Miri: nine context tests on the final snapshot; sixteen constructor tests before
  the pruning change. ThreadSanitizer: context/async pressure tests before pruning,
  then the pruning regression and context/generational/weak tests afterward.
- WASI: 111 tests before pruning and 87 relevant tests afterward. Bare-wasm smoke
  passes. Native and wasm GC Clippy pass, as do native classloader/intrinsics/types/
  VM Clippy and VM compile checks for both wasm targets with default features off.
  Formatting and diff checks pass.

The Loom tests are reduced models of buffer transfer, publication reservations,
claim closure, and pruning. They do not model the entire collector or establish a
protocol without the global gate.

### Repeated recovery screen

Each backend and worker count 0/1/4 has two ten-second runs for shared slots,
context roots with immediate barriers, and context roots with 32-record buffers.
Order reverses on the second repeat. The workload uses a 16 MiB heap, 4 KiB
allocations, 512 retained roots, one executor mutator, prediction, independent
claims of eight tasks, sixteen metadata shards, stealing, and diagnostics. All
runs use the same source/binary, CPUs 0 and 2, without concurrent builds or tests.
These are shared-host screens, without a process memory cap or an established
qualification noise tolerance. All policies include the pruning change; this
comparison does not isolate its performance effect.

Reports: compact [slots](validation/contexts-slots-compact-2026-11-07.json),
[contexts](validation/contexts-roots-compact-2026-11-07.json),
[buffers](validation/contexts-buffered-compact-2026-11-07.json); host-backed
[slots](validation/contexts-slots-host-2026-11-07.json),
[contexts](validation/contexts-roots-host-2026-11-07.json),
[buffers](validation/contexts-buffered-host-2026-11-07.json).
Cells list slots / contexts / buffered contexts. Throughput is the two-run mean in
thousands of allocations/s; operation p99.99 is the two-run range in microseconds.

| Payload | Workers | Throughput | Operation p99.99 |
| --- | ---: | --- | --- |
| compact | 0 | 513.2 / 505.3 / 512.7 | 662–678 / 678–697 / 661–693 |
| compact | 1 | 418.7 / 421.3 / 419.1 | 285–290 / 296–303 / 298–320 |
| compact | 4 | 402.0 / 391.5 / 384.3 | 294–301 / 306–314 / 297–307 |
| host | 0 | 245.0 / 245.8 / 246.4 | 145–150 / 142–163 / 142–160 |
| host | 1 | 236.3 / 236.8 / 238.2 | 32–33 / 33 / 32 |
| host | 4 | 232.0 / 230.1 / 233.1 | 37 / 37 / 42–48 |

These results do not meet the combined improvement targets. Buffered compact
throughput is about 4.4% below slots with four workers, and host four-worker tails
regress despite similar throughput. Cooperative recovery buffers no records in
this workload because mutations do not overlap its active marking windows; those
runs measure context overhead, not buffer effectiveness. Worker-backed buffered
runs process 909–40,158 records. Each context reports 5,184 bytes of fixed structure
on this host, excluding map/Arc overhead and separately reported root pages.

### Mixed workload and release

Single ten-second [compact](validation/contexts-endurance-compact-2026-11-07.json)
and [host](validation/contexts-endurance-host-2026-11-07.json) runs use buffered
contexts and the same combined policies, a 64 MiB heap and 10,000 offered operations/s.
Latencies are microseconds; this driver still has synchronous large constructors.

| Payload | Workers | Operation p99.9 / p99.99 | Arrival p99.9 / p99.99 | Inline recoveries | Peak RSS MiB |
| --- | ---: | --- | --- | ---: | ---: |
| compact | 0 | 42 / 1056 | 27392 / 34304 | 3 | 90.2 |
| compact | 1 | 59 / 656 | 1072 / 2816 | 0 | 64.5 |
| compact | 4 | 68 / 809 | 961 / 3200 | 0 | 20.7 |
| host | 0 | 56 / 2560 | 23296 / 26368 | 3 | 77.2 |
| host | 1 | 69 / 2784 | 2464 / 2976 | 0 | 60.3 |
| host | 4 | 76 / 2560 | 2432 / 3040 | 0 | 20.7 |

All complete the offered load with zero ending backlog. Four of six miss the
1 ms operation p99.99 gate. They process 40,354–289,310 buffered records and
118–8,853 bounded overflow flushes. Cooperative runs retain inline recovery;
physical RSS can exceed the logical heap budget. All 42 processes exit with empty
stderr and release every object, occupied payload range, root registration/page,
context, pending barrier, queued/in-flight task, and queue backing capacity.
Recovery runs have zero inline full-cycle recovery. Long endurance, complete
protocol verification, and default-policy qualification remain outstanding.

## Asynchronous `Object.clone` recovery (2026-11-07)

The Java clone intrinsic now awaits managed-headroom recovery before copying and
uses asynchronous value publication afterward. The source read guard ends before
publication can suspend. The source handle and copied reference fields remain
owning roots, and finalizable clones still register after publication. The guarded
copy itself, and preparation of an existing host buffer, remain synchronous; this
change does not provide bounded copying or a pause-free clone operation.

The existing 16 MiB VM pressure fixture attempts to clone a rooted 9 MiB array,
which cannot fit alongside its source. The strengthened test detects two inline
full-cycle recoveries before the fix. With the fix, both host-backed and compact
heaps deliver Java `OutOfMemoryError` with the original array preserved, record
asynchronous recovery, and add zero inline recoveries or remaining waiters.
Successful primitive-array, reference-array, and ordinary object clones additionally
check distinct identity/storage, copied scalar values, and shared reference fields
on JDK 8, 11, 17, 21 and 25. Existing reference ordering, resurrection, finalization
and queue-delivery assertions continue to pass.

Six VM GC/reference/process tests pass (one subprocess-only fixture ignored), as
do all three focused `java.lang.Object` intrinsic tests. Native GC/classloader/
intrinsics/types/VM Clippy tests/examples pass. VM compilation also passes on
WASI and bare wasm with default features disabled (bare wasm uses the configured
`wasm_js` randomness backend); these are compile checks, not Java execution on
wasm. Formatting and diff checks pass. The preceding GC and
classloader suites cover the unchanged queue/root implementation; no new
throughput or latency claim is made for cloning. The remaining synchronous
runtime/native constructors and synchronous embedding APIs remain migration work.

[Before/after test output](validation/clone-async-regression-2026-11-07.txt) records
the reproduced failure and passing suites. Final implementation/test snapshot:
`d4100af3d45d87ee540c071cb043f845a2382b99afbb5b3c727fab95a45598dd`.
The Java fixture is separately hashed as
`a647ac8d20caf02098bcb438b8809dd571e8dfcc70816350205ef01aa759b47a`,
since the performance runner’s source digest includes Rust/manifests/Python but
not Java fixtures. Measurements below retain their own earlier source snapshots.

## Root-page reuse after sharded sweeping (2026-11-07)

Root slots previously used exactly the metadata index’s address bucket function.
Sharded sweep order groups reusable descriptors by that bucket, and subsequent
root registration concentrated the rolling live set in the same root shard.
The one-empty-page cache stayed bounded but repeatedly created and retired pages.
Root selection now uses independent address mixing, with no new shared counter,
no change to slot generations or barriers, and the same cache bound.

A deterministic 512-live-root fixture reuses 8,192 aligned descriptor addresses in
metadata-shard order for four passes. Before the fix it creates 464 root pages and
fails the test’s 32-page ceiling; after the fix it passes on native and 32-bit WASI.
Final teardown releases every page. Native GC (192 tests plus one doctest),
classloader (759 tests including driver histograms; 14 ignored), and WASI (103
tests) pass, including the four-worker/four-mutator recovery matrix and all prior
slot-generation, cancellation, weak-reference and compaction fixtures. Bare-wasm
smoke, native/wasm GC Clippy, formatting and diff checks pass.

Source snapshot:
`437fa552c32f73824742395839aeeb15b6dfcee637857876592287cf18e8fc1a`.
The archived before binary uses `be8a609a…` from the queue-distribution batch below.
The comparison uses the same two CPUs, 16 MiB recovery workload and diagnostics
as that batch, with independent eight-task claims, stealing, sixteen metadata
shards, slot roots and prediction enabled. Each backend and worker count 0/1/4
has two ten-second runs, alternating before/after then after/before. No builds or
tests overlap measurement. Source/binary and runner hashes are recorded separately
for the archived binary. These are short repeated screens on shared cores.

### Repeated recovery comparison

Reports: compact [before](validation/rootmix-before-compact-2026-11-07.json) and
[after](validation/rootmix-after-compact-2026-11-07.json); host-backed
[before](validation/rootmix-before-host-2026-11-07.json) and
[after](validation/rootmix-after-host-2026-11-07.json). Cells show before → after.
Throughput is thousands of allocations/s, latencies are µs, and changes compare
two-run means. The page counts are pages registered in the slot backend, not all
system allocator activity.

| Payload | Workers | Root pages created | Throughput | Throughput change | Operation p99.99 | Poll p99.9 |
| --- | ---: | --- | --- | ---: | --- | --- |
| compact | 0 | 71024–72878 → 16 | 541.5–543.8 → 541.8–542.1 | -0.1% | 628–629 → 630–633 | 29–30 → 29–30 |
| compact | 1 | 23998–24691 → 16 | 453.6–457.8 → 455.8–459.2 | +0.4% | 233–235 → 217–220 | 41 → 40 |
| compact | 4 | 24624–25310 → 16 | 424.1–435.9 → 430.8–432.9 | +0.4% | 220–226 → 209–221 | 47–48 → 48 |
| host | 0 | 33655–33934 → 16 | 257.8–258.7 → 258.5–258.6 | +0.1% | 130–132 → 136–137 | 48–49 → 47–48 |
| host | 1 | 10891–11123 → 16 | 253.3–253.5 → 253.6–254.0 | +0.2% | 28 → 27–29 | 59–60 → 59–61 |
| host | 4 | 4207–4376 → 16 | 247.9–248.6 → 247.4–249.4 | +0.1% | 32–33 → 33–34 | 65 → 66–68 |

Each after run creates sixteen pages total, versus 4,207–72,878 before. Throughput
changes −0.1% to +0.4%. Compact operation p99.99 means improve 6.6% with one worker
and 3.6% with four, while cooperative compact and host-backed tails are flat or
worse. The change removes the measured page churn; it does not meet the broader
throughput/latency targets, and `RootStorage::Tree` remains the default.

### Mixed workload and teardown

Ten-second [compact](validation/rootmix-endurance-compact-2026-11-07.json) and
[host-backed](validation/rootmix-endurance-host-2026-11-07.json) screens use the
same combined policies, a 64 MiB heap, and 10,000 offered operations/s. Latencies
are µs. These are single runs, separate from the repeated recovery comparison.

| Payload | Workers | Operation p99.9 / p99.99 | Arrival p99.9 / p99.99 | Inline recoveries | Peak RSS MiB |
| --- | ---: | --- | --- | ---: | ---: |
| compact | 0 | 41 / 1023 | 25088 / 29696 | 3 | 88.4 |
| compact | 1 | 44 / 479 | 389 / 1020 | 0 | 60.4 |
| compact | 4 | 69 / 556 | 444 / 1104 | 0 | 20.8 |
| host | 0 | 54 / 2496 | 20992 / 24064 | 3 | 77.0 |
| host | 1 | 46 / 2592 | 2272 / 2752 | 0 | 20.7 |
| host | 4 | 72 / 2560 | 2272 / 2816 | 0 | 20.7 |

All mixed screens finish at about 10,000 operations/s with no ending backlog.
Four of six still miss the 1 ms operation p99.99 gate. Cooperative runs retain
three inline recoveries each, and RSS can exceed the logical heap budget. The
mixed driver still uses synchronous large-array constructors.

All 30 processes exit successfully with empty stderr, zero final live objects,
occupied payload ranges, root registrations/pages, queued tasks, in-flight work,
and retained queue backing capacity. All recovery runs have zero synchronous
full-cycle recovery, and worker-backed recovery has zero caller assists. The
archived-source runs and candidate runs match their respective binary hashes.
Miri, sanitizers, and long endurance qualification remain outstanding.

## Persistent tracing queues and bounded stealing (2026-11-06–07)

`work_stealing(true)` advances milestone 4 with persistent worker tracing queues,
lossless overflow to a shared injector, and bounded stealing. Each generation has
up to sixteen local queues, each holding at most 256 pending tasks, plus the
injector. Each claim probes a queue at most once and takes at most eight tasks from
one source within the remaining step budget. Cooperative wasm retains one queue.
The option defaults to false and works with either marking policy.

In-flight counters now belong to their generations, while closure still checks
both. Independent claims retain the claim reader until their reservation is
published; coordinated claims retain the coordinator. A contended/empty probe
cannot close marking: closure excludes publication and initial claims and checks
all closing-generation queues after the reservations. Continuations retain offset,
mutation version, and young-edge state when stolen. Departed workers leave queues
in the heap so other workers or cooperative callers can finish their work. No
queue guard spans another queue, metadata lookup, or a callback. Empty queue
backing storage is trimmed on `stop()` and all queues are cleared at shutdown.

### Checks

- Native GC: 191 tests plus one doctest passed. New fixtures cover bounded overflow,
  contended-source skipping, exact task delivery, stolen continuation state,
  a departed worker’s continuation, and queue storage release.
- The forced dequeue/reservation race now steals from another worker’s queue while
  metadata lookup is blocked. The claim writer cannot enter during the gap;
  after reservation, both generations remain unable to retire the snapshot until
  the blocked callback completes. Existing overlapping-generation batch tests also
  use persistent queues under both marking policies.
- Four native workers/four mutators completed 32,000 recovery allocations per
  matrix with both payload backends, root stores, prediction policies, and marking
  policies. Two complete GC suite runs passed during integration.
- Classloader: 757 unit/integration tests plus both driver histogram tests passed;
  14 existing tests remain ignored. WASI: 102 tests passed. Bare-wasm runtime smoke
  passed with combined experimental features and cooperative recovery.
- VM: three GC integration tests and three reference/process tests passed, with
  one subprocess-only fixture ignored. Java reference ordering, resurrection,
  exactly-once finalization and queue delivery ran on JDK 8/11/17/21/25 with four
  workers, stealing, independent eight-task claims and sixteen metadata shards.
- Native GC/classloader/types/VM Clippy and both wasm GC/smoke Clippy checks passed,
  as did formatting and diff checks.

No Miri, sanitizer, new model-checking harness, or multi-hour endurance run was
performed. Queue distribution retains the existing closure handshake; it does not
remove the global publication gate, distribute relocation, or introduce context
barrier buffering. The conservative cross-generation reclamation interlock remains.

### Measurement setup

Final source snapshot:
`be8a609ad8238a368d80572bcea65b5dc1ad8887e7439be3c34ea8893bdf2bdd`.
All processes share affinity to logical CPUs 0 and 2 (distinct physical cores),
including mutator and collector workers. The cores are not exclusive; the cgroup
CPU quota is unlimited. No builds or tests run during measurement.

The repeated recovery comparison uses a 16 MiB logical heap, one current-thread
executor, 512 retained roots, 4 KiB payloads and a yield every eight allocations.
Independent marking, eight-task claims, sixteen metadata shards and diagnostics
are enabled in both policies; prediction and slot roots are disabled. Policies
alternate off/on, then on/off, for two ten-second runs at each worker count
0/1/2/4 and payload backend. This isolates the stealing option in one binary; it
does not measure parallel mutator capacity. The preceding `af2dfe83…` binary is
also screened for five seconds at 0/1/4 workers under the same configuration.
Archived binary/source hashes are recorded separately from the current runner.

Combined-feature screens enable slot roots and prediction with stealing. Recovery
retains the 16 MiB fixture; mixed allocation/fragmentation uses a 64 MiB logical
heap and 10,000 offered operations/s. Mixed allocation still includes synchronous
large-array constructors. Short screens are diagnostic evidence, not qualification.

### Repeated recovery comparison

Reports: compact [single queue](validation/queues-off-compact-2026-11-06.json) and
[worker queues](validation/queues-on-compact-2026-11-06.json); host-backed
[single queue](validation/queues-off-host-2026-11-06.json) and
[worker queues](validation/queues-on-host-2026-11-06.json). Cells show off → on.
Throughput is thousands of allocations/s; latencies are µs. Ranges are per-run
values; changes compare the two-run means, not pooled histograms.

| Payload | Workers | Throughput | Throughput change | Operation p99.99 | Poll p99.9 |
| --- | ---: | --- | ---: | --- | --- |
| compact | 0 | 494.9–496.7 → 490.1–495.7 | -0.6% | 677–680 → 674–682 | 38–39 → 39–41 |
| compact | 1 | 406.6–421.7 → 404.2–421.4 | -0.3% | 208–208 → 210–218 | 57–58 → 58–61 |
| compact | 2 | 398.7–420.4 → 397.9–413.3 | -1.0% | 192–196 → 192–199 | 52–53 → 51–53 |
| compact | 4 | 373.1–389.2 → 382.2–383.3 | +0.4% | 208–213 → 202–205 | 58–73 → 56–62 |
| host | 0 | 233.6–238.8 → 235.8–238.9 | +0.5% | 159–160 → 156–159 | 63–84 → 64–89 |
| host | 1 | 226.2–233.2 → 229.6–230.9 | +0.2% | 230–249 → 243–246 | 77–123 → 76–120 |
| host | 2 | 229.3–236.7 → 233.2–233.4 | +0.1% | 250–263 → 257–258 | 75–80 → 78–83 |
| host | 4 | 230.0–230.0 → 223.6–227.5 | -1.9% | 249–263 → 263–275 | 77–80 → 81–90 |

Compact throughput changes −1.0% to +0.4%. Compact operation p99.99 improves 3.3%
with four workers, is approximately unchanged cooperatively, and worsens 0.8–2.9%
with one or two workers. Host throughput
changes −1.9% to +0.5%; host operation p99.99 worsens 5.1% with four workers.
Neither the combined target nor the worker-scaling exit gate is qualified. Short
runs on shared cores do not establish statistical significance; defaults remain
unchanged. The archived-source [compact](validation/queues-baseline-compact-2026-11-06.json)
and [host-backed](validation/queues-baseline-host-2026-11-06.json) screens are
separate five-second observations, not matched repeats.

### Combined-feature screens and teardown

Combined recovery: [compact](validation/queues-recovery-combined-compact-2026-11-06.json)
and [host-backed](validation/queues-recovery-combined-host-2026-11-06.json).
These are one ten-second run per worker count with all experimental policies
above enabled. Latencies are µs.

| Payload | Workers | Allocations/s | Operation p99.9 / p99.99 | Poll p99.9 | Root pages created |
| --- | ---: | ---: | --- | ---: | ---: |
| compact | 0 | 507,042 | 24 / 660 | 37 | 67,749 |
| compact | 1 | 432,026 | 11 / 192 | 46 | 23,377 |
| compact | 4 | 419,222 | 19 / 224 | 49 | 24,577 |
| host | 0 | 258,556 | 20 / 130 | 45 | 33,985 |
| host | 1 | 253,058 | 14 / 28 | 59 | 11,032 |
| host | 4 | 247,979 | 15 / 33 | 66 | 4,262 |

No recovery run executes a synchronous full-cycle recovery, and worker-backed
recovery runs execute no caller assists. Root pages remain bounded and are fully
released at teardown, but their allocation churn persists in this snapshot.

Mixed allocation/fragmentation: [compact](validation/queues-endurance-combined-compact-2026-11-06.json)
and [host-backed](validation/queues-endurance-combined-host-2026-11-06.json).

| Payload | Workers | Operation p99.9 / p99.99 | Arrival p99.9 / p99.99 | Inline recovery cycles | Peak RSS MiB |
| --- | ---: | --- | --- | ---: | ---: |
| compact | 0 | 40 / 1023 | 25344 / 31232 | 3 | 87.5 |
| compact | 1 | 49 / 552 | 434 / 1072 | 0 | 63.2 |
| compact | 4 | 71 / 646 | 477 / 1022 | 0 | 20.7 |
| host | 0 | 53 / 2368 | 21760 / 26368 | 3 | 76.9 |
| host | 1 | 46 / 2368 | 2208 / 2656 | 0 | 29.4 |
| host | 4 | 78 / 2368 | 2240 / 2816 | 0 | 20.7 |

All mixed screens complete about 10,000 operations/s with zero ending backlog.
Four of six miss the 1 ms operation p99.99 gate. Cooperative screens still execute
three inline full recoveries each; observed RSS can exceed the logical heap limit.
These results do not qualify ordinary synchronous native constructors or establish
sustainable offered-load capacity.

Across the repeated recovery comparison, process CPU is about 10 seconds with no
workers, 17.2–17.5 seconds for compact worker runs, and 12.0–12.3 seconds for host
worker runs. Peak RSS is 20.8–23.1 MiB. Four compact workers steal about 197,000–198,000
of 589,000–591,000 callbacks (including injector work), with 735–780 contended
queue probes, yet coordinator misses remain about 258,000–260,000. Distribution
alone leaves most phase coordination in place.

All 50 processes exit successfully with empty stderr, no live objects, occupied
payload ranges, or root registrations/pages after teardown. All 44 new-binary runs
also report zero final queued tasks, in-flight work and queue backing capacity.
The six archived-binary screens predate those new counters. The eight candidate
reports share the frozen source/binary pair, and both baseline reports identify
the archived source and binary separately. Measurements crossed local midnight
from November 6 to 7; filenames retain their start date.

## Sharded object and identity indexes (2026-11-06)

`metadata_shards(n)` advances milestone 4. Object-address trees and identity maps
can each use 1, 2, 4, 8, or 16 independent mutexes; direct and builder configurations
clamp/round to that range. One remains the default. Full cross-snapshot, sweep,
and compaction scans carry shard/address cursors, inspecting one entry or exhausted
shard per work unit. They release the shard before callbacks. Point operations
lock only the selected shard, and shutdown drains all shards before destruction.

The publication writer still serializes retirement against root/handle acquisition.
Insertion releases its identity shard before acquiring the object shard; retirement
uses object then identity shard under the existing coordinator/publication locks.
Publication barriers cover entries inserted behind a scan. Metadata ownership alone
does not grant safe access to a retired payload; existing in-flight reservations
and generation interlocks continue to protect task execution. Young/remembered
indexes, reference registries, and region allocation retain shared locks.

### Checks

- Native GC: 187 tests plus one doctest passed. New tests cover sparse bounded
  enumeration, distinct shard locks, configuration extremes, and publication after
  every sweep shard has been visited, followed by exactly-once reclamation.
- Classloader: 757 unit/integration tests plus both driver histogram tests passed;
  14 existing tests remain ignored.
- WASI: 98 tests passed. Bare-wasm runtime smoke passed with sixteen shards,
  independent/batched marking, slot roots, prediction, and cooperative recovery.
- Native GC/classloader/types/VM Clippy tests/examples and both wasm GC/smoke Clippy
  checks passed, as did formatting and diff checks.
- The four-worker/four-mutator recovery fixture uses sixteen shards with both
  marking policies, payload backends, root stores, and prediction off/on: 32,000
  allocations per matrix. Identity/type/foreign-heap validation, expired weak slots
  at reused addresses, generation overlap, late roots, reference ordering, and the
  equal-size fragmentation release gate also pass with sixteen shards.

Miri, sanitizers, and multi-hour endurance were not run. This adds no distributed
termination protocol or worker-local queues and does not remove the global gate.

### Measurement setup

Final source snapshot:
`af2dfe838031b0adcaf33c5a319325787ebe8dac20d8eecf2d1808eb0c90fa3d`.
The preceding `2494ac17…` implementation has fresh five-second
[compact](validation/shards-baseline-compact-2026-11-06.json) and
[host-backed](validation/shards-baseline-host-2026-11-06.json) baselines at 0/1/4
workers. All runs are pinned to logical CPUs 0 and 2, distinct physical cores on
this host, shared by the mutator and every worker. Affinity does not grow with
worker count. The cores are not exclusive and the cgroup CPU quota is unlimited.

The repeated comparison alternates one and sixteen shards, reversing order in the
second repeat. Each policy/backend/worker count (0/1/2/4) has two ten-second runs.
The recovery workload uses a 16 MiB logical heap, 512 retained roots, 4 KiB payloads,
one current-thread executor, and a yield every eight allocations. Independent
marking, eight-task claims, and diagnostics are enabled under both shard policies;
slot roots, prediction, and debt-triggered collection are disabled. This isolates
the shard option within the final binary; it does not measure multiple mutators or
isolate locking from tree size, layout, and traversal-order changes. Source/binary
hashes, CPU/RSS, affinity, complete configuration, raw histograms, and teardown
checks accompany each report. No builds or tests run during measurement.

### Repeated recovery comparison

Reports: compact [one shard](validation/shards-1-compact-2026-11-06.json) and
[sixteen shards](validation/shards-16-compact-2026-11-06.json); host-backed
[one shard](validation/shards-1-host-2026-11-06.json) and
[sixteen shards](validation/shards-16-host-2026-11-06.json). Table cells show
one → sixteen shards. Throughput is thousands of allocations/s; latency is µs.
Ranges are per-run values, and changes compare two-run means, not pooled histograms.

| Payload | Workers | Throughput | Throughput change | Operation p99.99 | Poll p99.9 |
| --- | ---: | --- | ---: | --- | --- |
| Compact | 0 | 487.9–500.4 → 506.1–507.4 | +2.5% | 678–715 → 651–659 | 37–40 → 37–38 |
| Compact | 1 | 422.6–428.1 → 428.7–429.4 | +0.9% | 167–169 → 198–212 | 54–55 → 52–53 |
| Compact | 2 | 397.6–400.4 → 400.6–404.5 | +0.9% | 179 → 197–201 | 50 → 49 |
| Compact | 4 | 383.1–393.3 → 394.0–400.7 | +2.4% | 189–198 → 196–208 | 54–57 → 51–55 |
| Host | 0 | 241.6–243.5 → 237.8–243.2 | −0.9% | 152–157 → 149–154 | 61–63 → 63–64 |
| Host | 1 | 235.1–240.0 → 233.9–234.9 | −1.3% | 225 → 236–240 | 75–76 → 76–77 |
| Host | 2 | 240.5–241.6 → 234.8–235.1 | −2.5% | 228–229 → 242–244 | 75–79 → 77–80 |
| Host | 4 | 234.6–235.4 → 225.4–229.5 | −3.2% | 232–234 → 248–253 | 78–82 → 96–122 |

Sixteen shards improve compact mean throughput 0.9–2.5%. Cooperative compact
operation p99.99 improves 6.0%, but worker-backed p99.99 worsens 4.4–22.0%.
Host throughput falls 0.9–3.2%; worker-backed operation p99.99 worsens 5.8–7.5%.
These results do not satisfy the combined performance target. One shard remains
the default, and the milestone exit gate remains unqualified. Two short repeats
on shared hardware also do not establish statistical significance.

Process CPU is about 10 seconds cooperatively, 17.2–17.5 seconds for compact runs
with workers, and 11.9–12.2 seconds for host-backed runs with workers. Peak RSS
is 20.8–22.0 MiB. Coordinator misses remain near the one-shard control, reaching
about 260,000 with four compact workers. Point-index sharding leaves serial
phase work, publication waits, and worker scheduling costs in place.

### Combined-feature screens

Single ten-second runs enable sixteen metadata shards, slot roots, prediction,
independent marking, eight-task claims, and diagnostics together, on the same
two CPUs. These check feature interaction; they do not isolate sharding or establish
endurance or sustainable capacity.

Recovery reports: [compact](validation/shards-recovery-combined-compact-2026-11-06.json)
and [host-backed](validation/shards-recovery-combined-host-2026-11-06.json), with the
same 16 MiB heap and allocation workload as above.

| Payload | Workers | Allocations/s | Operation p99.9 / p99.99 (µs) | Poll p99.9 (µs) | Early space wakes | Predictive requests |
| --- | ---: | ---: | --- | ---: | ---: | ---: |
| Compact | 0 | 520,997 | 23 / 643 | 37 | 0 | 1 |
| Compact | 1 | 413,873 | 15 / 254 | 60 | 1,250 | 36 |
| Compact | 4 | 399,980 | 25 / 250 | 59 | 1,239 | 62 |
| Host | 0 | 249,951 | 28 / 141 | 60 | 0 | 1 |
| Host | 1 | 237,423 | 26 / 167 | 169 | 2 | 746 |
| Host | 4 | 235,739 | 26 / 49 | 88 | 2 | 738 |

These six runs record zero synchronous recovery. Cooperative runs use bounded
assistance; worker-backed runs use none. Root pages show substantial churn:
21,652–68,242 pages created in compact recovery and 3,658–32,817 host-backed,
with all but sixteen released before shutdown and none retained afterward.
The preceding one-index combined runs created only sixteen pages. This comparison
does not isolate traversal order from allocator/layout changes, but it identifies
root-page reuse as a follow-up before changing defaults. The bounded cache prevents
unbounded retention; it does not eliminate page-allocation overhead. Poll p99.9 also
reaches 169 µs in the host one-worker screen, despite low operation percentiles.

Mixed allocation/fragmentation reports:
[compact](validation/shards-endurance-combined-compact-2026-11-06.json) and
[host-backed](validation/shards-endurance-combined-host-2026-11-06.json), with a
64 MiB heap, 10,000 offered operations/s, and synchronous large-array constructors.

| Payload | Workers | Operation p99.9 / p99.99 (µs) | Arrival p99.9 / p99.99 (µs) | Synchronous recoveries | Final backlog | Peak RSS (MiB) |
| --- | ---: | --- | --- | ---: | ---: | ---: |
| Compact | 0 | 45 / 1,072 | 26,368 / 32,000 | 3 | 0 | 86.2 |
| Compact | 1 | 46 / 514 | 426 / 1,104 | 0 | 0 | 64.1 |
| Compact | 4 | 84 / 1,104 | 966 / 1,440 | 0 | 0 | 20.6 |
| Host | 0 | 57 / 2,720 | 24,064 / 28,160 | 3 | 3 | 76.9 |
| Host | 1 | 60 / 2,464 | 2,272 / 3,008 | 0 | 0 | 20.6 |
| Host | 4 | 83 / 2,528 | 2,304 / 2,784 | 0 | 0 | 20.6 |

Completed throughput is approximately 10,000/s in all six mixed runs, but five
miss the sub-millisecond operation p99.99 target. Cooperative operation maxima
reach 32.8 ms compact and 28.9 ms host-backed. Scheduled-arrival latency records the
backlog during stalls; a nearly drained final backlog is not evidence of low tails.
RSS still exceeds the logical heap limit in some runs because that limit excludes
metadata and host allocator retention. Mixed runs create sixteen root pages each,
so the recovery workload's page churn is not universal.

All 50 runs across these ten reports exit successfully with zero live objects,
occupied region bytes, allocation waiters, owning/numeric roots, and root pages
after teardown. The eight final reports match the recorded source and binary
hashes; the two baselines retain their preceding snapshot. Neither sharding nor
the combined features qualify removal of the global publication gate, elimination
of synchronous recovery, or a default-policy change.

## Independent marking and recovery offers (2026-11-06)

`independent_marking(true)` advances milestone 4 by allowing bounded tracing-only
steps to claim, trace, and publish continuations without the coordinator or cycle
mutex. It remains disabled by default. Fresh work and continuations share a
per-generation queue lock, while phase state remains separately coordinated.
Object/identity indexes are still shared; this is not worker-local scheduling or
work stealing. Reference decisions, root enumeration, sweep, and relocation retain
their coordinated paths.

The claim read guard covers dequeue through in-flight reservation. Mark closure
takes the claim writer under the coordinator and publication writer, checks
in-flight work before the queues, and disables mark readiness before advancing the
phase. Callbacks run without the claim guard. A step keeps all its reservations
until it has returned continuations and merged its statistics, so completion cannot
precede unfinished work or its counters. Further claims within the same step use
the outstanding reservation to protect the epoch. All existing work/time limits,
scan locks, mutation versions, and per-task young-edge state remain in use.

A forced-interleaving test stops a claim after dequeue but before reservation.
The claim writer remains unavailable in that gap; after reservation, the in-flight
count prevents closure until the callback finishes. Another test holds both the
coordinator and cycle mutex while independent tracing completes, demonstrating
that the tracing path uses neither. Existing batch-reservation tests run with both
policies, including overlapping generations.

Testing also caught a scheduling error in the first implementation: a failed fast
claim advanced the generation hint, causing the fallback to revisit a busy full
cycle instead of closing an empty young cycle. Pending generations and root passes
now get coordinated turns, and failed fast claims preserve the hint. The existing
long-full-trace/young-progress regression failed before this fix and passes after.

### Recovery under competing allocations

The four-mutator stress test exposed intermittent `OutOfMemory` failures. The same
failure reproduced with independent marking disabled, at publication after other
allocators consumed reclaimed headroom. The old waiter lost the fact that space
had been offered when its registry entry was removed for notification. A later poll
could then see only cycle completion and spend the failure budget.

Space waiters now retain an atomic offer flag until observed or cancelled. Both
completion notifications and early space notifications can set it. An offer is
advisory, not a credit reservation: publication still reserves bytes atomically.
Losing offered or observed headroom does not consume a recovery cycle; two cycles
without such an opportunity still terminate an unsatisfiable request. Advisory
headroom checks recheck current capacity before returning. A deterministic test
loses three successive offers to a competing reservation and then succeeds; it
failed before the fix and passes after. This does not promise allocator fairness
or bounded waiting under continual competition.

[Before/after regression logs](validation/independent-regressions-2026-11-06.txt)
retain the development failures and focused reruns. They are separate from the
final source snapshots used for performance comparison.

### Checks

- Native GC: 183 tests plus one doctest passed.
- Classloader: 757 unit/integration tests and both driver histogram tests passed;
  14 existing tests remain ignored.
- WASI GC suites: 94 tests passed. Bare-wasm runtime smoke passed with independent
  marking, batched claims, slot roots, and clock-free prediction enabled.
- Native GC/classloader/types/VM Clippy tests/examples passed; GC tests/smoke passed
  Clippy on both wasm targets. Formatting and diff checks passed.
- Four mutators × 500 allocations, four workers, both marking policies, both payload
  backends, both root stores, and prediction off/on passed twice after the recovery
  fix (32,000 allocations per matrix). Unsatisfiable heaps, cancellation, worker
  failure, weak/final/phantom ordering, mutation barriers, and phase overlap also pass.

Miri, thread sanitizers, and multi-hour endurance were not run. The claim handshake
uses existing locks and global in-flight protection; publication-gate removal and
per-generation distributed termination remain open.

### Measurement setup

Final source snapshot:
`2494ac17424e4ccc897aadb9f318f7289f5b5f15eb78a6c1acee0b2247143dbf`.
The initial [compact](validation/independent-baseline-compact-2026-11-06.json) and
[host-backed](validation/independent-baseline-host-2026-11-06.json) five-second
baselines use the preceding `de258f73…` implementation and unrestricted affinity.
They are retained as context, not compared directly with the fixed-affinity runs.

The [compact](validation/independent-screen-compact-2026-11-06.json) and
[host-backed](validation/independent-screen-host-2026-11-06.json) five-second screens
use the final implementation on CPUs 0 and 2, two distinct physical cores on this
host. The repeated comparison uses those same two logical CPUs for every process,
including all workers and the mutator. CPU availability does not grow with worker
count. These CPUs are not exclusively reserved, and there is no additional cgroup
CPU quota. Background host activity can still affect results.

Each policy/backend/worker count (0/1/2/4) has two ten-second recovery runs. Policies
alternate within each worker count, reversing order in the second repeat. The
workload uses a 16 MiB heap, 512 retained roots, 4 KiB payloads, one current-thread
executor, and a yield every eight allocations. Diagnostics are enabled; prediction,
slot roots, and debt-triggered collection are disabled. Both policies claim one
task at a time, isolating independent steps from the earlier eight-task option.
Both contain the queue refactor and recovery-offer fix. Reports retain hashes,
configuration, affinity, raw histograms, CPU/RSS, and teardown checks. No builds or
tests run during measurement; ranges below are per-run values, not pooled histograms.

### Repeated recovery comparison

Raw reports: compact [coordinated](validation/independent-off-compact-2026-11-06.json)
and [independent](validation/independent-on-compact-2026-11-06.json); host-backed
[coordinated](validation/independent-off-host-2026-11-06.json) and
[independent](validation/independent-on-host-2026-11-06.json). Each paired table cell
shows coordinated → independent. Throughput is thousands of completed allocations
per second; latency is microseconds. Percentage changes compare the two-run means.

| Payload | Workers | Throughput | Throughput change | Operation p99.99 | Poll p99.9 |
| --- | ---: | --- | ---: | --- | --- |
| Compact | 0 | 483.1–497.9 → 488.9–497.7 | +0.6% | 724–756 → 718–725 | 38–40 → 39–40 |
| Compact | 1 | 416.5–421.6 → 418.6–419.1 | −0.1% | 191–193 → 183–184 | 53 → 53 |
| Compact | 2 | 391.4–414.9 → 395.2–395.6 | −1.9% | 196–200 → 192–196 | 53 → 51–53 |
| Compact | 4 | 383.2–384.7 → 380.3–386.6 | −0.1% | 218–222 → 209–212 | 56–57 → 58–61 |
| Host | 0 | 235.6–237.7 → 225.6–235.0 | −2.7% | 180–182 → 177–189 | 64–65 → 66–72 |
| Host | 1 | 235.9–236.9 → 232.4–233.6 | −1.4% | 226–242 → 215–248 | 73–74 → 75 |
| Host | 2 | 236.2–237.9 → 237.1–238.5 | +0.3% | 244–248 → 241–245 | 76–77 → 76–78 |
| Host | 4 | 231.8–233.5 → 231.0–237.4 | +0.7% | 239–254 → 232–242 | 81–104 → 77–91 |

Compact operation p99.99 means improve 2.0–4.4%, with throughput between −1.9%
and +0.6%. Host operation tails and throughput vary by worker count. Operation
p99.9 remains 15–33 µs across these recovery runs. Two short repeats on shared
hardware do not establish statistical significance or qualify the proposed
throughput/latency improvements. Independent marking remains disabled by default.

Process CPU is about 10 seconds cooperatively, 17.2–17.4 seconds for compact runs
with workers, and 11.8–12.0 seconds for host-backed runs with workers, under either
policy. Peak RSS is 20.7–22.1 MiB. Independent steps execute 9,440–11,984 times in
compact runs and 5,434–5,868 times in host runs. Claim-gate misses are 0–19 per run.
Coordinator misses remain close to the control, reaching about 254,000 compact and
138,000 host-backed with four workers. Removing the coordinator from tracing alone
does not remove contention in the other phases or improve worker scaling here.

### Combined-feature screens

These single ten-second runs enable independent marking, eight-task claims,
slot roots, prediction, and diagnostics together, on the same CPUs 0 and 2.
They check integration and expose remaining stalls; they are not an isolated
comparison or endurance qualification.

Recovery reports: [compact](validation/independent-recovery-combined-compact-2026-11-06.json)
and [host-backed](validation/independent-recovery-combined-host-2026-11-06.json).
The workload and 16 MiB heap match the recovery comparison above.

| Payload | Workers | Allocations/s | Operation p99.9 / p99.99 (µs) | Poll p99.9 (µs) | Early space wakes | Predictive requests |
| --- | ---: | ---: | --- | ---: | ---: | ---: |
| Compact | 0 | 522,161 | 23 / 690 | 37 | 0 | 1 |
| Compact | 1 | 411,303 | 15 / 183 | 52 | 1,237 | 77 |
| Compact | 4 | 397,169 | 25 / 204 | 60 | 1,229 | 226 |
| Host | 0 | 242,651 | 29 / 157 | 62 | 0 | 1 |
| Host | 1 | 238,758 | 24 / 33 | 72 | 174 | 751 |
| Host | 4 | 239,943 | 24 / 35 | 79 | 3 | 751 |

All six recovery runs record zero synchronous full-cycle recovery. Worker-backed
runs use zero caller assists; cooperative runs retain bounded assistance. Operation
and poll maxima still exceed their percentiles, reaching 3.80 ms and 6.18 ms
respectively, so the soft step budget remains no hard latency bound.

Mixed allocation/fragmentation reports:
[compact](validation/independent-endurance-combined-compact-2026-11-06.json) and
[host-backed](validation/independent-endurance-combined-host-2026-11-06.json).
These use a 64 MiB heap and 10,000 offered operations/s, including synchronous
large-array constructors. All six finish near 10,000 operations/s with zero final
backlog, but short-run recovery does not establish sustainable capacity.

| Payload | Workers | Operation p99.9 / p99.99 (µs) | Arrival p99.9 / p99.99 (µs) | Synchronous recoveries | Peak RSS (MiB) |
| --- | ---: | --- | --- | ---: | ---: |
| Compact | 0 | 44 / 1,104 | 26,624 / 31,744 | 3 | 101.3 |
| Compact | 1 | 51 / 686 | 883 / 2,944 | 0 | 70.6 |
| Compact | 4 | 97 / 712 | 534 / 1,184 | 0 | 20.6 |
| Host | 0 | 67 / 2,496 | 22,272 / 25,856 | 3 | 77.1 |
| Host | 1 | 62 / 2,592 | 2,336 / 2,848 | 0 | 58.6 |
| Host | 4 | 100 / 2,816 | 2,528 / 3,008 | 0 | 20.6 |

The sub-millisecond operation p99.99 target still fails for cooperative compact
and every host-backed mixed run. Arrival latency exposes backlog during the stalls,
even though the final backlog drains. Cooperative operation maxima reach 32.5 ms
compact and 26.7 ms host-backed. Physical RSS can exceed the logical heap limit;
metadata and allocator retention remain relevant to qualification.

All 56 runs across the 12 reports exit successfully with zero live objects,
occupied region bytes, allocation waiters, root registrations, and root pages
after teardown. The final ten reports match the recorded source/binary hashes;
the initial two baseline reports retain the preceding source snapshot. The global
publication gate, persistent worker queues, distributed termination, and remaining
synchronous allocation paths are still open plan items.

## Bounded tracing claims and local work counters (2026-11-06)

This advances milestone 4. Object marking, sweeping, freed bytes, promotions, and
reference examinations now accumulate within each step and merge at step exit and
before cycle completion. The public statistics API remains; an in-progress step's
counters may lag until that merge. Cycle completion publishes its counters before
releasing the completion ticket.

`mark_batch_size(1..=8)` selects the maximum tasks in a tracing claim; the default
remains one. The fixed-size local array shares the existing work/time budgets.
Before releasing the coordinator, every claimed task enters the global in-flight
count, including tasks not yet executed. Busy callbacks and soft time-limit yields
return remaining tasks with their original continuation state. Executed continuations
retain their mutation version and young-edge state. Relocation claims remain single
objects. This is not persistent worker-local scheduling, work stealing, or removal
of the global publication gate. Those plan items remain open.

Optional diagnostics count tracing claims (`mark_batches`), executed callbacks
(`trace_tasks`), and reserved tasks returned without execution (`trace_deferred`).
The latter includes all unexecuted tasks after a busy callback or time-limit yield.
The recovery/endurance drivers and `measure.py --mark-batch-size` record the policy
and counters; scheduled endurance adds an eight-task recovery comparison.

Validation covers:

- A blocked callback holds seven additional reserved tasks while another caller
  drains central work. Neither generation can finish or reclaim objects, including
  when a young cycle starts during the blocked full cycle. Both finish after release
  with the expected object counts.
- Continuation offsets survive `Busy`, later retries, and deferral, under step work
  limits of 1, 3, and 64 with both root stores. A slow callback forces the soft time
  limit, and all unexecuted tasks return without loss or duplicate completion.
- Direct configuration values of zero and `usize::MAX` clamp safely; completion
  snapshots include marking, promotion, reclamation, and freed-byte totals.
- Existing generation/barrier, reference-ordering, root-slot, parallel tracing and
  relocation, and concurrent async recovery fixtures exercise eight-task claims.
  Async recovery combines four workers and four mutators with both payload/root
  backends and prediction off/on. Bare wasm exercises batching with slot roots and
  clock-free prediction.

Native GC tests: 180 passed plus one doctest. Classloader unit/integration tests:
757 passed with 14 existing ignores; both driver histogram tests also passed. WASI suites: 93 passed. Bare-wasm runtime smoke and native
GC/classloader/types/VM Clippy checks passed, as did GC tests/smoke Clippy on both
wasm targets. Miri and thread sanitizers were not run.

### Initial comparison and regression investigation

The first tracing-batch snapshot was
`918f69c4a3f7a9806175b64ade1c1930361b79790b534825a20fe95b3a9591e5`.
Fresh pre-change [compact](validation/mark-batches-baseline-compact-2026-11-06.json)
and [host-backed](validation/mark-batches-baseline-host-2026-11-06.json) baselines
retain snapshot `9b019af46465317aa249c969b85ed8a8933998c1cfbb5822fa785973218babd7`.
The initial [five-second screen](validation/mark-batches-screen-2026-11-06.json)
and repeated [compact one-task](validation/mark-batches-1-compact-2026-11-06.json),
[compact eight-task](validation/mark-batches-8-compact-2026-11-06.json),
[host one-task](validation/mark-batches-1-host-2026-11-06.json), and
[host eight-task](validation/mark-batches-8-host-2026-11-06.json) reports are preserved.
Eight-task claims reduced native compact operation p99.99 by 12–14% but reduced
throughput 2.5–4.0%. These initial measurements did not qualify a default change.

The host-backed one-task build also lost throughput against the pre-change binary.
Interleaved ten-second [archived baseline](validation/mark-batches-control-old-host-2026-11-06.json)
and [new binary](validation/mark-batches-control-new-host-2026-11-06.json) controls
confirmed it: two-run mean throughput fell 5.4% cooperatively and 4.3% at one worker.
The archived binary hash matches the fresh baseline; control reports retain the
originating binary/source provenance and contain new process measurements.
Tracing time per allocation was nearly unchanged. A follow-up change selects the
host or region initialization loop once per chunk, removing the per-element backend
branch while retaining prefix ownership after every successful initializer. The
[follow-up screen](validation/mark-batches-initialize-screen-2026-11-06.json)
recovered host throughput. This does not isolate every compiler/layout effect; the
final comparison measures the complete resulting implementation.

### Final comparison

Final source snapshot:
`de258f73d6dac01bab553c21928cb588e74986e00e0631ceff8fa933a622c661`.
The tests above were rerun after separating the initialization loops, including
initializer error/unwind and async cancellation cleanup with both backends.

The repeated recovery comparison uses two ten-second runs per policy, payload
backend, and worker count (0, 1, 2, 4). It retains the 16 MiB heap, 512 roots, 4 KiB
payloads, current-thread executor, and yield every eight allocations. Diagnostics
are enabled; prediction and debt-triggered collection are disabled. One-task and
eight-task reports use the same binary, including local counters in both. Source
and binary hashes, raw histograms, process CPU/RSS, and teardown checks accompany
each report. No tests or builds ran during measurements. These are saturation
screens on an unisolated development host; fixed-CPU-quota qualification and long
endurance remain outstanding. Reported ranges are per-run values, not pooled
histograms.

Compact payloads: [one task](validation/mark-batches-final-1-compact-2026-11-06.json),
[eight tasks](validation/mark-batches-final-8-compact-2026-11-06.json).

| Workers | Tasks/claim | Completed/s | Operation p99.9 (µs) | Operation p99.99 (µs) | Poll p99.9 (µs) | Process CPU (s) |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 0 | 1 | 490,575–497,751 | 23–24 | 719–730 | 38–40 | 10.00 |
| 0 | 8 | 492,402–497,655 | 24–25 | 691–703 | 40–41 | 10.00 |
| 1 | 1 | 423,656–428,059 | 16–18 | 200–204 | 50–51 | 17.17–17.18 |
| 1 | 8 | 414,307–420,816 | 13–16 | 171–174 | 49–50 | 17.14–17.18 |
| 2 | 1 | 396,577–397,630 | 18–20 | 226–227 | 54–56 | 18.04–18.10 |
| 2 | 8 | 388,208–389,986 | 15–16 | 186–190 | 54–55 | 18.17–18.19 |
| 4 | 1 | 352,597–352,905 | 19 | 234–241 | 58–61 | 19.97–20.05 |
| 4 | 8 | 351,848–353,094 | 17–18 | 200–204 | 58–59 | 20.10–20.19 |

Host-backed payloads: [one task](validation/mark-batches-final-1-host-2026-11-06.json),
[eight tasks](validation/mark-batches-final-8-host-2026-11-06.json).

| Workers | Tasks/claim | Completed/s | Operation p99.9 (µs) | Operation p99.99 (µs) | Poll p99.9 (µs) | Process CPU (s) |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 0 | 1 | 237,246–240,179 | 29 | 173–176 | 62–66 | 10.00 |
| 0 | 8 | 234,369–237,987 | 29–32 | 154–157 | 64–70 | 10.00 |
| 1 | 1 | 228,958–240,719 | 24–27 | 250–275 | 73–78 | 11.75–11.81 |
| 1 | 8 | 229,764–230,149 | 28 | 254 | 75–84 | 11.78–11.80 |
| 2 | 1 | 226,356–238,378 | 23–28 | 265–294 | 73–81 | 11.96–12.00 |
| 2 | 8 | 226,833–230,545 | 27 | 252–266 | 76–79 | 12.01–12.04 |
| 4 | 1 | 214,053–217,939 | 24–25 | 330–335 | 82–83 | 12.39–12.45 |
| 4 | 8 | 223,229–229,705 | 21–22 | 260–269 | 77–78 | 12.28 |

With compact payloads, eight-task claims reduce mean native operation p99.99 by
about 15–17%. Throughput changes are +0.2%, -1.9%, -2.0%, and -0.1% at 0/1/2/4
workers. Host-backed changes are -1.1%, -2.1%, -1.6%, and +4.8%; operation p99.99
improves about 11%, 3%, 7%, and 20% respectively. Several throughput ranges span
5%, so these short runs do not establish reliable capacity gains. Poll and timer
tails vary; the raw reports retain both. Process CPU does not improve consistently.
The combined target remains unqualified, and one task remains the default.

Eight-task runs average 7.87–7.89 executed callbacks per tracing claim, reducing
claims per callback about 87%. Only 8–383 tasks per run were returned without
execution; they remain counted until returned. This amortizes tracing coordination
but retains shared indexes, the publication gate, and serial reclamation. Peak RSS
for these runs was 20.5–21.9 MiB.

The final one-task host throughput ranges overlap or exceed the archived controls:
237–240k/s cooperatively and 229–241k/s with one worker. The earlier 4–5% loss is not
reproduced after separating the initialization loops. Against the fresh pre-change
compact baseline, final one-task throughput is approximately 1.0%, 3.1%, and 0.5%
lower at 0/1/4 workers; this snapshot still has no qualified overall throughput gain.

### Combined features and remaining gates

Single ten-second recovery runs combine eight-task claims, slot roots, and
prediction with [compact](validation/mark-batches-final-recovery-combined-compact-2026-11-06.json)
and [host-backed](validation/mark-batches-final-recovery-combined-host-2026-11-06.json)
payloads at 0/1/4 workers. Native host-backed operation p99.99 is 34/33 µs, with
2/1 recovery waits at one/four workers. Compact native operation p99.99 is 200/207 µs,
with 1,836/1,253 waits. Cooperative recovery still needs caller-driven progress.

Mixed [compact](validation/mark-batches-final-endurance-combined-compact-2026-11-06.json)
and [host-backed](validation/mark-batches-final-endurance-combined-host-2026-11-06.json)
runs offer 10,000 operations/s for ten seconds with a 64 MiB heap. All complete
100,000 operations without backlog, prediction requests, or recovery waits; young
and periodic full collections keep occupancy below the predictive trigger. Compact
operation p99.99 is 928/1,104/1,016 µs at 0/1/4 workers, exceeding the 1 ms gate in
both native configurations. Host-backed values are 2,432/2,496/2,432 µs; the mixed
driver still performs synchronous large-array construction. These remain short
coexistence checks, not multi-hour endurance or latency qualification.

All 44 final runs finish with zero live objects, occupied region bytes, root
registrations/pages, and recovery waiters, with no inline allocation recovery.
All 114 runs across this batch's 22 reports have successful exit/teardown checks.
Earlier combined [compact recovery](validation/mark-batches-recovery-combined-compact-2026-11-06.json),
[host recovery](validation/mark-batches-recovery-combined-host-2026-11-06.json),
[compact mixed](validation/mark-batches-endurance-combined-compact-2026-11-06.json),
and [host mixed](validation/mark-batches-endurance-combined-host-2026-11-06.json)
reports retain the initial `918f69c4…` snapshot and are not pooled with final data.

Statistics aggregation is integrated, but persistent worker queues, stealing,
per-generation termination accounting, shared-index sharding, and replacement of
the publication gate remain open. None of these results establishes pause-free
collection or the plan's combined throughput/latency target.

## Reusable root slots and worker handoff (2026-11-06)

This batch advances the owning-root work in milestone 2. Select
`root_storage(RootStorage::Slots)` to use reusable pages for owning handles and
guards. The default remains `RootStorage::Tree`. Each of 16 registries has 64-slot
pages, selected by allocation address; registrations are independent of OS-thread
identity. Clones share their registration, suspended futures retain it, and dropping
or cancelling on another thread releases the same slot.

A location plus checked, nonzero 64-bit generation prevents stale removal from
deleting a new occupant. Encoding the tree variant in the unused zero generation
reduces the registration token from 24 to 16 bytes on the measured native host. Each generation enumerates slots with its own bounded cursor. Reuse
behind a cursor retains the existing insertion/deletion barriers, and the global
publication gate remains. Explicit numeric root IDs retain their tree API under
either policy. This is not registered execution-context storage or a buffered
barrier protocol; those milestone 2 items remain open.

Empty pages are retired outside registry locks, with at most one empty cached page
per registry. `stop()` trims those caches while preserving nonempty pages for escaped
roots. Sparse nonempty pages remain until their last registration leaves.
`root_statistics()` reports page capacity/bytes, cache occupancy, reuse, and page
retirement. Page-byte accounting excludes map nodes and owning-handle metadata;
process RSS includes those costs.

Testing exposed an idle-handoff race: after another worker clears a request flag
but before it publishes active state, a worker could observe no pending work and
block acquiring the coordinator. If a subsequent destructor held the coordinator,
that worker could not reach its normal backoff path. Idle handoff now uses a
nonblocking claim and the existing bounded backoff. A deterministic test held the
coordinator while waking an idle worker: it failed before the fix and passed after.
The serial destructor/backoff regression and allocation wakeup tests also pass.

| Check | Result |
| --- | --- |
| Native GC unit/integration tests | 176 passed, plus one doctest |
| Concurrent async recovery | Four mutators × 500 allocations, four workers, both root stores, both payload backends, prediction off/on |
| Classloader unit/integration tests | 757 passed, 14 existing ignores; both driver histogram tests passed |
| WASI GC suites | 91 passed |
| Bare-wasm smoke | Passed with root slots and clock-free prediction enabled; empty pages released at shutdown |
| Clippy | Native GC/classloader/types/VM tests/examples; GC tests/smoke on both wasm targets |

New coverage includes stale/duplicate removal, page-number wrap, bounded sparse
scanning, publication after each generation's slot scan, guard transfer/replacement,
explicit numeric/address compatibility APIs, cache retirement with escaped roots,
and suspended-future cancellation across threads. Constructor-unwind tests exercise
both root stores. Weak/final/phantom ordering and pending-root transfer now run with
both stores on native and WASI. Miri and thread sanitizers were not run.

### Performance comparison

The final release binary compares [tree](validation/root-tree-compact-2026-11-06.json)
and [slot](validation/root-slots-compact-2026-11-06.json) roots with compact payloads,
and [tree](validation/root-tree-host-2026-11-06.json) and
[slot](validation/root-slots-host-2026-11-06.json) roots with host-backed payloads.
Each report contains two 10-second runs per worker count: one current-thread
executor, 512 retained roots, 4 KiB payloads, a 16 MiB heap, a yield every eight
allocations, diagnostics enabled, and prediction/debt-triggered collection disabled.
These are saturation screens on an unisolated development host. No builds ran during
measurement; ranges are per-run values rather than pooled histograms.

Final source snapshot: `9b019af46465317aa249c969b85ed8a8933998c1cfbb5822fa785973218babd7`.
Reports include binary hashes, configuration, CPU limits, raw distributions, process
CPU, peak RSS, root-page creation/reuse, and teardown counts. A
[fresh pre-change baseline](validation/root-slots-baseline-2026-11-06.json) retains
source `52f566652aa699327802db9efbffbab67db6f73dedfd0d0cf0c9b5087c14f727`.
The final tree/slot comparison isolates the storage policy within the same binary;
the worker handoff fix is present in both.

Compact payloads:

| Workers | Roots | Completed/s | Operation p99.9 (µs) | Operation p99.99 (µs) | Poll p99.9 (µs) | Timer p99.9 (µs) | Process CPU (s) |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 0 | tree | 490,134–493,342 | 24–25 | 736–749 | 39–40 | 955–1,200 | 10.00 |
| 0 | slots | 505,796–508,019 | 25–26 | 741–757 | 38–40 | 905–950 | 10.00–10.01 |
| 1 | tree | 422,351–427,277 | 17–18 | 213–216 | 52–53 | 1,056–1,120 | 17.22–17.30 |
| 1 | slots | 424,553–440,801 | 15–17 | 231–245 | 51 | 1,088–1,200 | 17.42–17.44 |
| 4 | tree | 342,517–344,534 | 18–20 | 254–260 | 59–62 | 1,040–1,088 | 20.55–20.63 |
| 4 | slots | 339,668–348,570 | 17–18 | 275–288 | 58–65 | 1,056–1,072 | 20.84–20.88 |

Host-backed payloads:

| Workers | Roots | Completed/s | Operation p99.9 (µs) | Operation p99.99 (µs) | Poll p99.9 (µs) | Timer p99.9 (µs) | Process CPU (s) |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 0 | tree | 236,967–237,213 | 29–30 | 191–192 | 67–68 | 933–972 | 10.00 |
| 0 | slots | 237,601–240,312 | 29–30 | 188–191 | 70–72 | 817–995 | 10.00 |
| 1 | tree | 223,341–226,846 | 28 | 276–278 | 81–88 | 1,104–1,136 | 11.79–11.80 |
| 1 | slots | 226,773–229,593 | 28 | 301–324 | 82–106 | 1,136–1,248 | 11.77–11.78 |
| 4 | tree | 206,237–210,284 | 26–27 | 329–333 | 107 | 1,136–1,184 | 12.30–12.34 |
| 4 | slots | 210,456–210,751 | 26–27 | 345–351 | 85–88 | 1,040–1,088 | 12.25–12.34 |

Two-run throughput means improve about 3.1%, 1.9%, and 0.2% for compact payloads
at 0/1/4 workers; host-backed changes are about 0.8%, 1.4%, and 1.1%. These small
changes do not establish a reliable capacity gain without noise qualification.
Native operation p99.99 worsens roughly 10–11% with compact payloads and 5–13%
with host-backed payloads. Other tails vary by configuration. The combined target
is not met, and the prior one-worker throughput regression remains unresolved.
The tree policy stays the default.

Each slot run installed only 16 pages, reused slots for nearly every subsequent
allocation, and retained 16,512 bytes of empty pages before shutdown. Shutdown
released all pages. Slot-run peak RSS was 20.5–20.9 MiB. Every run finished with zero
live objects, occupied region bytes, root registrations, root pages, and recovery
waiters, and recorded zero inline allocation recovery. Shared publication/index
contention remains; these measurements do not qualify removing the global gate or
make a claim about a hard pause bound.

The [initial slot screen](validation/root-slots-screen-2026-11-06.json) and earlier
[compact tree](validation/root-tree-compact-screen-2026-11-06.json),
[compact slot](validation/root-slots-compact-screen-2026-11-06.json),
[host tree](validation/root-tree-host-screen-2026-11-06.json), and
[host slot](validation/root-slots-host-screen-2026-11-06.json) comparisons retain
their original snapshots. They preceded the smaller registration token and must
not be pooled with the final comparison.

### Prediction and mixed-workload coexistence

Additional single 10-second runs per worker count enable both slots and prediction
for [compact](validation/root-slots-recovery-predictive-compact-2026-11-06.json) and
[host-backed](validation/root-slots-recovery-predictive-host-2026-11-06.json) recovery.
Host-backed native operation p99.99 was 32–33 µs, with zero recovery waits at one
worker and one wait at four workers. Compact native operation p99.99 was 234–262 µs,
with recovery still required. These check the combined policies; they are not a
repeated comparison against a current tree-plus-prediction control.

The mixed driver also completed 100,000 operations per run at 10,000 offered
allocations/s on 64 MiB heaps with both policies enabled, for
[compact](validation/root-slots-endurance-predictive-compact-2026-11-06.json) and
[host-backed](validation/root-slots-endurance-predictive-host-2026-11-06.json) payloads.
Ordinary young/periodic-full cycles kept occupancy below the predictor's trigger;
there were no additional predicted requests or allocation-recovery waits. All runs
had clean object, region, root, and page teardown. Compact operation p99.99 was
562/674/1,040 µs at 0/1/4 workers, so the four-worker run exceeded the 1 ms gate.
Host-backed operation p99.99 was 2,400–2,560 µs; the driver still initializes large
arrays synchronously. These short checks do not replace multi-hour endurance or
latency qualification.


## Predictive full-collection scheduling (2026-11-06)

This batch adds the pressure model from milestone 3 in [PLAN.md](PLAN.md).
`ConfigurationBuilder::predictive_collection(true)` enables it on finite, nonzero
heaps. It remains disabled by default. The global publication gate, shared metadata
registries, and existing worker activation/pacing policy remain in use.

The predictor samples allocation rate and observes full-cycle survival, live bytes,
and time from request/start to the first reclaimed bytes. If nothing is reclaimed,
completion time supplies the latency observation. It requests a full collection
when occupancy reaches the predicted headroom threshold. Estimates rise quickly
and decay over samples/cycles. Burst and relocation allowances, plus an indexed
physical-fragmentation signal when backing space is tight, supplement the timing
estimate. The allowance is bounded; it grants neither logical credits nor physical
copying destinations. Existing bounded allocation recovery remains the fallback.

Allocation sampling uses nonblocking model/region locks and reads a clock only on
samples. Registered region storage and occupancy have cached totals; uninstalled
backing reservations and backing awaiting deallocation are excluded from free-space
estimates. The extent index supplies the largest hole without a region traversal.
Full requests coalesce, and a completed cycle requires fresh growth before another
automatic request, scaled by the estimated retained set. Bare wasm uses deterministic
occupancy/growth rules without a clock or fabricated timing. Prediction requests
work; cooperative hosts must still call `step()` or poll a recovery future.

| Check | Result |
| --- | --- |
| Native GC unit/integration tests | 163 passed, plus one doctest |
| Concurrent recovery | Four mutators × 500 allocations with four workers, prediction both off/on, both payload backends |
| Classloader unit/integration tests | 757 passed, 14 existing ignores; both driver histogram tests also passed |
| WASI GC suites | 80 passed, including prediction and bounded exhausted-live-set recovery |
| Bare-wasm smoke | Passed with prediction enabled and absent clock/rate/latency values asserted |
| Clippy | Native GC/classloader/types/VM tests/examples; GC tests and smoke example on both wasm targets |

The new tests cover early collection without inline assistance, rooted survivors,
request coalescing, high-survival cooldown, burst response, fragmented extents,
contention-deferred samples, extreme numeric inputs, clock-free policy, and bounded
async failure for an exhausted live set. Existing relocation, weak-reference,
generation, cancellation, and accounting suites still pass. A native-only test lint
expectation was scoped to native targets so the complete GC test set also passes
Clippy on wasm. Miri, thread sanitizers, and a new multi-hour endurance run were not run.

### Performance comparison

The same release binary was run with prediction [off](validation/predictive-off-2026-11-06.json)
and [on](validation/predictive-on-2026-11-06.json) for compact payloads, and with
prediction [off](validation/predictive-host-off-2026-11-06.json) and
[on](validation/predictive-host-on-2026-11-06.json) for host-backed payloads. Each
report contains two 10-second runs per worker count. Configuration matches the
recovery workload below: one current-thread executor, 512 retained roots, 4 KiB
payloads, a 16 MiB heap, and a yield every eight allocations. Ordinary allocation-debt
collection is disabled; diagnostics are enabled. No builds ran during measurement.

Source snapshot: `52f566652aa699327802db9efbffbab67db6f73dedfd0d0cf0c9b5087c14f727`.
The reports record the binary hash, host/CPU limits, full driver configuration,
raw histograms, process CPU, and peak RSS. These are development-host saturation
screens without CPU isolation or a qualified noise tolerance. Ranges describe
individual runs, not pooled percentiles.

Compact payloads:

| Workers | Prediction | Completed/s | Operation p99.9 (µs) | Operation p99.99 (µs) | Poll p99.9 (µs) | Timer p99.9 (µs) | Process CPU (s) |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 0 | off | 496,757–498,098 | 24 | 732–753 | 38 | 954–1,088 | 10.00 |
| 0 | on | 499,043–518,001 | 22–24 | 692–723 | 36–37 | 873–1,072 | 10.00 |
| 1 | off | 430,472–432,572 | 16 | 228 | 51 | 1,120–1,280 | 17.11–17.17 |
| 1 | on | 413,037–435,767 | 16–17 | 178–201 | 50–55 | 1,120–1,440 | 17.26–17.29 |
| 4 | off | 349,292–350,979 | 18–20 | 260–276 | 60–61 | 1,056–1,072 | 19.79–20.09 |
| 4 | on | 334,390–343,109 | 18–19 | 235–240 | 59–60 | 1,040–1,056 | 20.37–20.38 |

Host-backed payloads:

| Workers | Prediction | Completed/s | Operation p99.9 (µs) | Operation p99.99 (µs) | Poll p99.9 (µs) | Timer p99.9 (µs) | Process CPU (s) |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 0 | off | 232,559–232,862 | 30 | 193–194 | 69–70 | 946–1,008 | 10.00 |
| 0 | on | 231,674–233,162 | 30–31 | 194–197 | 71 | 1,002–1,018 | 10.00 |
| 1 | off | 227,484–232,382 | 20–24 | 265–273 | 82–87 | 1,104–1,168 | 11.75–11.76 |
| 1 | on | 228,349–229,640 | 20–22 | 32–33 | 78–82 | 951–1,168 | 12.06–12.10 |
| 4 | off | 212,838–219,775 | 23–24 | 327–338 | 93–113 | 1,104 | 12.26–12.33 |
| 4 | on | 211,103–211,612 | 19–20 | 33–34 | 88 | 812–893 | 12.63–12.71 |

For native host-backed runs, prediction removes all observed async allocation-recovery
waits (685–699 to zero with one worker; 640–661 to zero with four). Mean operation
p99.99 falls about 88% and 90%, respectively. Throughput means fall about 0.4% and
2.3%, and CPU rises about 3%. Compact native runs still need recovery: operation
p99.99 improves about 17% with one worker and 11% with four, while throughput means
fall about 1.7% and 3.3%. Timer and poll tails do not improve consistently.

The prior one-worker compact-heap throughput regression remains unresolved. In the
current off/on runs, aggregate publication-read wait time is 1.14–1.18 seconds per
10-second run. These counters demonstrate continuing blocking in shared publication;
they do not isolate every source of the regression. Prediction changes when work
starts, but does not remove the publication/index contention or ensure that sustained
reclamation keeps pace after the first bytes become available. Worker distribution,
local roots/metadata, and sustained-reclamation estimates remain follow-up work.

Cooperative recovery runs drive collection only when an allocation needs recovery.
Prediction therefore issues an initial early request without running it before that
boundary; it cannot independently create background progress. Their short-run
throughput differences are not evidence of a scheduling gain. Actual cooperative
hosts must continue driving their normal GC steps.

Every recovery run recorded zero inline allocation recovery, zero registered waiters
at workload completion, and zero live objects/occupied region bytes after teardown.
Peak RSS with prediction was 20.5–21.5 MiB across backends and worker counts. The
combined throughput/latency target is not met, so prediction and compaction remain
opt-in. The earlier five-second [off](validation/predictive-off-screen-2026-11-06.json)
and [on](validation/predictive-on-screen-2026-11-06.json) screens retain their separate
source snapshot and must not be pooled with this comparison.

Short mixed-workload checks with prediction enabled also passed for
[compact](validation/predictive-mixed-compact-2026-11-06.json) and
[host-backed](validation/predictive-mixed-host-2026-11-06.json) payloads: one 10-second
run per worker count, 64 MiB heap, and 10,000 offered allocations/s. Each completed
100,000 operations with clean teardown and no allocation-recovery waits. Ordinary
young/periodic-full scheduling kept occupancy below the predictive trigger, so
there were no extra predicted requests. This checks coexistence with the ordinary
policy; it does not establish prediction overhead against a matched control.
Compact operation p99.99 was 524–713 µs. Host-backed operation p99.99 was
2,400–2,720 µs, exceeding the existing 1 ms gate; the mixed driver still initializes
its large arrays synchronously. These short checks do not replace endurance or
latency qualification.


## Allocation headroom and worker contention (2026-11-06)

This batch advances the bounded-recovery and pacing work in [PLAN.md](PLAN.md).
Allocations waiting for logical bytes can resume during reclamation. Explicit full
collection retains its completion contract. At this snapshot the global publication
gate remained; predictive scheduling, local allocation caches, distributed collector
queues, and the concurrent reclamation redesign were still pending.

Implemented changes:

- Allocation waiters share outstanding full-cycle requests while retaining distinct
  registration IDs and wakers. An index ordered by requested bytes selects eligible
  waiters after each step. Notification removes both index entries before waking.
  It does not grant credit; publication reserves logical bytes atomically on the
  requesting task, with automatic refund on failure.
- Registration precedes rechecking headroom, cycle completion, and worker status.
  A lost race for advertised space re-registers against the same cycle and does
  not consume the two-completed-cycle exhaustion budget. Cancellation before the
  next poll drops unpublished data without retaining a logical reservation.
- Marking and clearing both use the descriptor's useful-slot budget. Clearing a
  leaf payload no longer consumes 32 units for a constant-size empty-edge clear.
  Reference containers still clear bounded batches, and time checks remain between
  work units. Cooperative recovery still performs at most one configured step per poll.
- Native workers back off after failed coordinator claims, doubling the delay up to
  the configured pacing limit (at most 1 ms) and halving it after successful claims.
  Productive work keeps its existing pressure-sensitive pacing. Pacing set to zero
  still yields without sleeping. Diagnostics report early allocation wakeups and
  worker backoffs separately.

The logical-byte invariant now includes credits reserved immediately before
publication as well as registered allocations. No reservation crosses an await
between acquisition and publication. The existing publication gate still protects
root/metadata registration and reclamation. Headroom notifications are advisory;
competing or newly arriving allocators can consume space before a waiter runs.
Physical backing and descriptor allocation failures retain bounded full-cycle
fallbacks. These are progress and accounting rules, not a hard latency guarantee.

Correctness evidence for this batch:

| Check | Result |
| --- | --- |
| Native GC unit/integration tests | 152 passed, plus one doctest |
| Recovery/accounting suite | 18 passed, including four concurrent allocators with four GC workers on both payload backends |
| Classloader unit/integration tests | 757 passed, 14 existing ignores; both workload histogram tests also passed |
| Types library | 200 passed |
| VM library and Java GC/reference integration binaries | 895 library tests and six integration tests passed; one existing helper ignore |
| Reflective array intrinsics | 140 passed |
| WASI GC suites | 69 passed, including nine portable recovery/accounting tests |
| WASI JVM fixtures | Two passed |
| Bare-wasm smoke | Passed, including clock-free async pressure recovery |
| Clippy | GC/classloader/types/VM native tests/examples; VM on both wasm targets |

New tests hold compaction unfinished while an allocation resumes or is cancelled,
share one cycle between separate allocation and full-collection waiters, race
multiple allocators for reclaimed space, verify bounded live-set exhaustion,
and confirm that leaf reclamation uses its work budget. A stalled serial destructor
forces coordinator contention and verifies native worker backoff. Publication
unwinding now drops the unpublished value and refunds logical credit; its regression
test covers both synchronous and async constructors. The existing parallel tracing,
relocation, generation, and reference-ordering tests still pass. Miri and thread
sanitizers were not run.

### Repeated recovery comparison

The [fresh pre-change baseline](validation/headroom-baseline-2026-11-06.json) and
[final candidate](validation/headroom-final-2026-11-06.json) each contain two
10-second runs per worker count. Both use the async recovery workload described
below: one current-thread executor, 512 retained roots, 4 KiB payloads, a compacting
16 MiB heap, a yield every eight allocations, and pressure-triggered collection.
Diagnostics are enabled. No builds ran during measurement. This is a development
host saturation screen without CPU isolation or a qualified noise tolerance.

Baseline source hash: `57d30a896ce6e4a21c42cf92095718da3ea4c597f35e9e37a894fd4e1826370e`.
Candidate source hash: `dd896bb28ddc2f3e48874e181a4bfa1d4856779dbc562bc39b46bd9dbcd7b430`.
The reports include binary hashes, complete driver configuration, raw distributions,
process CPU, and peak RSS. Ranges describe individual runs, not pooled histograms.

| Workers | Snapshot | Completed/s | Operation p99.9 (µs) | Operation p99.99 (µs) | Poll p99.9 (µs) | Timer p99.9 (µs) | Process CPU (s) |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 0 | Before | 486,309–492,151 | 10–11 | 2,368–2,432 | 28–31 | 1,072–1,152 | 10.00 |
| 0 | After | 491,128–491,983 | 24–25 | 742–757 | 39 | 947 | 10.00 |
| 1 | Before | 443,293–459,926 | 10–13 | 2,816–2,880 | 39–49 | 1,936–1,968 | 10.01 |
| 1 | After | 421,939–425,917 | 15 | 229–235 | 50 | 1,200–1,216 | 17.21–17.24 |
| 4 | Before | 326,020–328,339 | 11 | 5,504–5,568 | 41–43 | 1,776–1,792 | 25.33–25.47 |
| 4 | After | 345,916–345,995 | 19 | 269–270 | 59–60 | 1,152–1,200 | 20.28–20.33 |

Using the two-run means, four-worker throughput increases about 5.7%, operation
p99.99 falls about 95%, and process CPU falls about 20%. One-worker throughput
falls about 6.1%, with increased CPU as collection overlaps more allocation work.
Cooperative throughput is nearly unchanged. Operation p99.9 and executor-poll
p99.9 increase; improved p99.99 does not mean every latency measure improves.
The largest observed candidate allocation took 2.44 ms. Peak RSS fell from
24.9–36.9 MiB across baseline configurations to 20.8–21.8 MiB in the candidate.

All candidate runs ended with zero live objects, occupied region bytes, and
registered waiters, and recorded zero inline allocation recovery. The short-run
results do not meet the plan's combined throughput/latency target and do not
qualify removal of global coordination or a universal pause bound.

The [host-backed candidate screen](validation/headroom-host-final-2026-11-06.json)
uses the same source/configuration except `compact=false`, also with two 10-second
runs per worker count. It checks the default payload backend; no fresh host-backed
baseline was collected in this batch.

| Workers | Completed/s | Operation p99.99 (µs) | Poll p99.9 (µs) | Process CPU (s) |
| --- | ---: | ---: | ---: | ---: |
| 0 | 233,507–235,591 | 201–203 | 66–67 | 10.00 |
| 1 | 225,181–227,516 | 299–302 | 79–81 | 11.79–11.81 |
| 4 | 212,245–212,574 | 340–359 | 84–94 | 12.42–12.48 |

These runs also recorded zero inline recovery and clean teardown. Earlier
[headroom-only](validation/headroom-screen-2026-11-06.json),
[sweep-budget](validation/headroom-sweep-screen-2026-11-06.json), and
[backoff](validation/headroom-backoff-screen-2026-11-06.json) exploratory screens
retain their separate source snapshots. They exposed the wasted sweep budget and
contention cost, but their five-second distributions must not be pooled with the
final comparison.

## Async allocation recovery integration (2026-11-06)

This batch advances milestones 1 and 3 in [PLAN.md](PLAN.md). The interpreter's
`new`, `newarray`, `anewarray`, `multianewarray`, and reflective array constructors
now await recovery instead of completing a full cycle inline. Synchronous embedding
APIs and other runtime/native allocation paths remain available. The global
publication gate remains required; this is not a pause-free collector.

Implemented behavior:

- `Gc::try_new_once` returns failed, unpublished input without collecting.
  `Gc::try_new_async` retains that input across at most two full recovery cycles.
  Advisory headroom and physical storage recovery have separate bounded retries.
- `collect_full_async` uses completion tickets and cancellation-safe waker slots.
  Running native workers provide progress without caller assistance or busy polling.
  Without workers, each poll performs at most one configured recovery step. Worker
  shutdown wakes waiting futures; unwinding worker failure returns a collection
  error. Wakers run after collector locks and tracing context have been released.
- Async field/array initialization yields between 64 KiB batches (at least one
  element). Cancellation and initializer failure release only initialized values
  and their physical range. Primitive fills run in a synchronous inner loop so
  the async state machine does not impose per-element suspension checks.
- Worker pacing sleeps are bypassed while async completion waiters exist, including
  after sweep has reduced occupancy below the ordinary pressure threshold.
  Diagnostics separate inline recovery from awaited time and report async assists,
  collection calls, and registered waiters.

| Check | Result |
| --- | --- |
| Native GC unit/integration tests | 144 passed, plus one doctest |
| New recovery tests | 10 native; six portable tests also pass on WASI |
| Classloader unit/integration tests | 757 passed, 14 existing ignores |
| Shared bounded histogram tests | Passed in both workload examples |
| Types library | 200 passed, including executor progress during async allocation pressure |
| VM library | 895 passed, including async dispatch and yielding during large `newarray` initialization |
| Java GC/reference integration binaries | Six passed, one existing helper ignore; JDK 8/11/17/21/25 reference fixtures and native pressure/OOM fixtures |
| Reflective array intrinsics | 140 passed |
| WASI GC suites | 66 passed in Wasmtime 50.0.1 |
| WASI JVM fixtures | Two passed: reference-handler lifetime and finalization/resurrection/phantom ordering |
| Bare-wasm smoke | Passed, including async pressure recovery and chunked initialization without a host clock |
| Clippy | GC/classloader/types/VM tests and examples on native; VM on bare wasm and WASI |

Recovery tests force live-set exhaustion, cancellation before publication, physical
allocation retry before initializer side effects, destruction of partially initialized
host/region buffers, and owning edges retained by suspended futures. Native tests
force waiter migration between OS threads, waker replacement, cancellation, waiting
for the requested cycle, reentrant completion callbacks, worker shutdown, and worker
unwinding. Miri and thread sanitizers were not run for this batch.

### Recovery workload and measurements

`gc_recovery_latency` compares synchronous and async allocation on a current-thread
Tokio executor. Both modes replace a fixed ring of 512 owning roots with 4 KiB byte
arrays and explicitly yield every eight allocations. A companion timer records
lateness against 1 ms deadlines, retaining missed deadlines. The driver separately
records each allocation's completion latency and each workload-future poll duration.
Threshold-triggered collections are disabled to force allocation-pressure recovery.
Async waits still include the time needed to finish a cycle; timer delay and poll
time show whether unrelated executor work can progress during that wait.

```sh
cargo build --release -p ristretto_classloader --example gc_recovery_latency
python3 ristretto_gc/validation/measure.py --workload recovery --recovery-mode sync --seconds 10 --workers 0 1 4 --heap-mib 16 --repeats 2 --output /tmp/recovery-sync.json
python3 ristretto_gc/validation/measure.py --workload recovery --recovery-mode async --seconds 10 --workers 0 1 4 --heap-mib 16 --repeats 2 --output /tmp/recovery-async.json
```

The runner saves source/binary hashes, compiler/host/configuration details, raw
histograms, process CPU, and peak RSS. Diagnostics are enabled in both modes.
Worker count zero means the driver does not start native workers. The workflow
records both modes for cooperative, one-worker, and four-worker execution with
host-backed and compactable payloads. Shared CI records timings without enforcing
performance thresholds.

Two 10-second samples per mode and worker count used a compacting 16 MiB heap.
The [synchronous results](validation/recovery-sync-2026-11-06.json) and
[async results](validation/recovery-async-2026-11-06.json) share source hash
`57d30a896ce6e4a21c42cf92095718da3ea4c597f35e9e37a894fd4e1826370e`.
Ranges below are the two individual samples, not merged distributions. CPU is
process user plus system time, including teardown. These are saturation screens
on a development host without CPU isolation, not qualification or offered-load
response-time measurements. Timer latency also includes runtime timer precision.

| Workers | Mode | Completed/s | Operation p99.99 (µs) | Executor poll p99.9 (µs) | Timer delay p99.9 (µs) | Process CPU (s) |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 0 | Sync | 452,595–514,272 | 1,968–2,144 | 1,936–2,080 | 3,296–6,336 | 10.00 |
| 0 | Async | 440,076–467,153 | 2,528–2,656 | 34–37 | 1,056–1,168 | 10.00 |
| 1 | Sync | 420,074–456,477 | 2,656–2,720 | 2,592–2,656 | 4,064–7,872 | 11.46–11.47 |
| 1 | Async | 397,602–413,397 | 3,168–3,200 | 47–53 | 1,888–2,144 | 9.94–10.02 |
| 4 | Sync | 355,935–371,235 | 3,808–3,936 | 3,712–3,808 | 9,600–10,112 | 18.41–18.70 |
| 4 | Async | 253,700–259,234 | 6,400–7,488 | 53–59 | 2,816–3,008 | 24.90–25.05 |

Every async run recorded zero inline recovery calls, zero residual waiters, and
zero live objects/occupied region bytes after teardown. Async executor-poll maxima
were 0.6–3.1 ms, compared with 4.1–15.1 ms synchronously. Allocation completion is
still delayed by collection: the longest awaited recovery was 62.7 ms. Process peak
RSS ranged from 25.1 to 36.5 MiB across modes.

Executor responsiveness improves, but saturated throughput and allocation p99.99
do not meet the plan's combined target. Four workers consume more CPU while
completing fewer allocations asynchronously. Removing caller full-cycle assistance
also changes which threads perform collection. Predictive scheduling, shared-state
contention, and waking allocations before full-cycle completion remain areas for
further work; these results do not justify raising the default worker count.

A separate single 10-second screen per mode/worker count exercised the default
host-backed payload backend: [synchronous](validation/recovery-host-sync-2026-11-06.json)
and [async](validation/recovery-host-async-2026-11-06.json), with the same source hash
and workload settings except `compact=false`.

| Workers | Sync completed/s | Async completed/s | Sync poll p99.9 (µs) | Async poll p99.9 (µs) |
| --- | ---: | ---: | ---: | ---: |
| 0 | 247,282 | 216,727 | 1,376 | 62 |
| 1 | 236,043 | 217,221 | 1,888 | 68 |
| 4 | 196,368 | 179,956 | 2,944 | 116 |

Host-backed runs also recorded clean teardown and zero async inline recovery. They
show the same responsiveness/throughput tradeoff; a single sample does not establish
noise tolerance or qualify the default backend.

The [initial synchronous screen](validation/recovery-initial-sync-2026-11-06.json)
and [initial async screen](validation/recovery-initial-async-2026-11-06.json) preserve
a separate snapshot. They exposed a per-element async initialization penalty and
workers sleeping while an allocation awaited cycle completion. The final snapshot
moves filling into a synchronous inner loop and bypasses those pacing sleeps.
Do not combine the initial and final distributions or compare this forced-pressure
workload directly with the earlier mixed-size offered-load endurance workload.

## Throughput and latency plan: first integration batch

This batch implements parts of milestones 0, 1, 2, 3, and 7 in [PLAN.md](PLAN.md).
It does **not** remove the publication gate, add asynchronous VM allocation
recovery, or qualify a pause-free collector. Compaction remains opt-in. The earlier
sections below describe separate source snapshots and must not be mixed into the
new measurements.

Implemented changes:

- Optional timing distinguishes publication and payload lock contention, caller
  assistance, worker work/CPU, and synchronous full-cycle allocation recovery.
  Counters include root registrations, coordinator misses, and useful/busy tracing
  and relocation tasks. Region counters separate staging, copied, and recopied bytes
  and failed relocation destinations. Buffer flushes and retired storage are not
  reported because those protocols have not been introduced.
- Ordinary interpreter field/array allocation and reflective array construction
  initialize directly in the selected heap. Failure/unwind cleanup tracks the
  initialized prefix. A largest-free-range index narrows region selection; backing
  allocation/deallocation hold physical credits while running outside the registry
  lock. Heap-to-heap preparation transfers storage to the receiving heap.
- Root-guard conversion transfers an existing registration, including across guard
  clones. Sparse bitmap words replace one tree entry per dirty card. Repeated
  pending young-cycle requests avoid redundant statistics locking and worker wakes.
  The final scheduling correction requests periodic full cycles on young-cycle
  completion, with a regression test covering an idle allocator and coalesced requests.
- Relocation tracks dirty 1 KiB chunks. A 70 KiB mutation fixture copies 67 KiB,
  changes four chunks across bitmap-word boundaries, and then completes with only
  4 KiB of recopying. The equal-size fragmentation fixture still frees all eight
  theoretically reclaimable regions.

Correctness evidence collected for this batch:

| Check | Result |
| --- | --- |
| Native GC unit/integration tests | 134 passed, plus one doctest |
| Native classloader unit/integration tests | 756 passed, 14 existing ignores; the subsequently added indexed-write test also passed |
| Endurance histogram unit test | Passed, including fixed storage and percentile bounds |
| Native reflective-array intrinsics | 140 passed |
| Native types library | 199 passed |
| Native VM library | 894 passed |
| Java GC/reference integration binaries | 6 passed, one existing helper ignore |
| WASI GC unit, generational, weak-reference, compaction, and diagnostic tests | 60 passed in Wasmtime 50.0.1 |
| Bare-wasm `gc_smoke` | Passed in Wasmtime, including enabled clock-free diagnostics |
| Clippy | GC, classloader, types, VM tests/examples on native; VM on bare wasm and WASI |

Native tests cover initializer errors/unwinding, initialized-prefix destruction,
foreign-heap accounting, root-registration transfer/replacement, sparse-card
iteration, concurrent region allocation/release/trim, alignment, dirty-chunk
mutation, and existing generation/reference behavior. WASI uses aborting panics,
so it runs initializer-error cleanup but excludes the unwind-only assertion.

### Latest snapshot with fixed histogram storage

The [latest diagnostic screen](validation/fixed-histogram-2026-11-06.json)
includes the periodic-scheduling correction and fixed-size recorder described
below. Each row is one 10-second sample with a compacting 64 MiB heap and one
mutator; diagnostics are enabled. It is a screening result, not qualification.

| Workers | Offered/s | Completed/s, rounded | Operation p99.9 (µs) | Arrival p99.9 (µs) | Longest inline recovery (ms) |
| --- | ---: | ---: | ---: | ---: | ---: |
| 1 | 10,000 | 10,000 | 51 | 564 | 0 |
| 4 | 10,000 | 10,000 | 96 | 2,016 | 0 |
| 1 | 100,000 | 99,999 | 36 | 63,488 | 67.56 |
| 4 | 100,000 | 100,000 | 52 | 215,040 | 217.30 |
| 1 | 300,000 | 295,767 | 30 | 532,480 | 101.88 |
| 4 | 300,000 | 234,915 | 38 | 2,097,152 | 146.51 |

The lower service-time percentiles at heavier load do not indicate better response
latency: rare synchronous recovery and accumulating backlog dominate arrival
latency. At 300,000/s, final backlog was 42,325 operations with one worker and
644,487 with four. At 10,000/s the four-worker operation p99.99 was 1,264 µs,
failing the existing 1 ms gate. Publication locking and full-cycle allocation
recovery remain present. The plan's combined objective is unachieved.

Separate final screens used [cooperative collection](validation/cooperative-final-2026-11-06.json)
and [host-backed payloads](validation/host-final-2026-11-06.json), with diagnostics
disabled, at 10,000/s. Both sustained the offered load. Cooperative operation
p99.9/p99.99 was 56/524 µs; host-backed one-worker results were 65/2,752 µs.
All these final runs ended with zero live objects and occupied region bytes.
Their source hash is `a043fa27e5b5f3478182d1bcb4cfcd0dc97e695f4cff9fb27ff5b53d134cb2ce`.

The [preceding scheduling-only screen](validation/completion-scheduling-2026-11-06.json.gz)
and [host-payload screen](validation/host-payloads-2026-11-06.json) retain results
from before the fixed-size recorder. Do not merge their distributions with the
latest snapshot.

### Initial comparison results

The comparison uses compacting 64 MiB heaps, one mutator, and 10-second runs: two
repeats in the instrumented baseline and three in the candidate. Diagnostics were
disabled for this table. Ranges show individual runs, not confidence intervals.

| Workers | Offered operations/s | Baseline operation p99.9 (µs) | Candidate operation p99.9 (µs) | Candidate operation p99.99 (µs) |
| --- | ---: | ---: | ---: | ---: |
| 1 | 10,000 | 57–59 | 53–58 | 488–1,172 |
| 4 | 10,000 | 99–122 | 89–95 | 692–1,090 |
| 1 | 30,000 | 55–74 | 43–45 | 467–1,231 |
| 4 | 30,000 | 83–94 | 78–84 | 461–1,246 |

Both versions approximately sustained these offered loads. This does not demonstrate
a 10% increase in sustainable capacity. Candidate p99.9 improved most at 30,000/s
with one worker, but p99.99 exceeded 1 ms in some runs. Arrival p99.9 also varied
substantially, reaching 29.2 ms. The combined performance gate has **not passed**.

Staging fell from 209,215,744 bytes per 100,000 operations to 81,920 bytes for the
setup reference array. At 300,000 operations it fell from about 623 MB to the same
81,920 bytes. Relocation traffic increased in these snapshots, so staging savings
alone must not be reported as total copy savings. All completed comparison runs
ended with zero live objects and zero occupied region bytes.

Two enabled-diagnostic runs at four workers/10,000 operations/s observed no
allocation-recovery cycles. Publication-read waits still totaled 38.5–40.2 ms per
run, with individual waits up to 3.45 ms. Worker CPU during steps was 2.09–2.31 CPU
seconds per 10-second run. These results directly contradict any claim that GC
stalls have been eliminated. Enabled/disabled differences are within the noisy
spread of these samples; instrumentation overhead is not yet precisely isolated.

Raw reports preserve each source/binary hash:
[original pre-instrumentation samples](validation/original-2026-11-06.json),
[instrumented baseline, diagnostics disabled](validation/baseline-2026-11-06.json),
[baseline diagnostics](validation/diagnostics-2026-11-06.json),
[first candidate](validation/direct-allocation-2026-11-06.json), and
[candidate diagnostics](validation/direct-allocation-diagnostics-2026-11-06.json).
The original samples used the earlier arrival scheduling and should not be used
for arrival-latency comparisons. The first candidate measurements precede the
final foreign-heap relocation-cancellation and indexed-write regression fixes;
subsequent reports identify their own source snapshots.

A separate [capacity screen](validation/capacity-screen-2026-11-06.json.gz) ran two
10-second repeats at 60,000 and 100,000 operations/s with 1, 2, and 4 workers. All
runs approximately sustained the offered load and ended with at most four queued
operations, but arrival p99.9 reached 216 ms. This only screens these loads; it
neither finds maximum sustainable capacity nor establishes long-run backlog or
memory stability. No throughput percentage improvement is claimed.

The [pressure diagnostics](validation/pressure-diagnostics-2026-11-06.json.gz)
include the transfer/indexing fixes and precede the periodic-scheduling correction. At 100,000 operations/s, one and four workers
respectively performed 12 and 13 synchronous allocation-recovery cycles; the
longest took 82 and 187 ms. At an offered 300,000/s, completed throughput fell to
about 291,000/s with one worker and 208,000/s with four, ending with backlogs of
71,359 and 919,757 operations. The four-worker arrival p99.9 was 3.06 seconds.
All runs still ended with zero live objects and occupied region bytes after
teardown. Diagnostics were enabled, and these are single 10-second samples;
they establish overload and the existence of long inline recovery, not a precise
maximum sustainable rate.

An [ordinary-load screen after the transfer/indexing fixes](validation/final-screen-2026-11-06.json) ran two
10-second samples at 10,000 operations/s in cooperative mode and with one/four
native workers. Every run ended without backlog or retained objects/ranges. One
four-worker run measured operation p99.9 of 108 µs and p99.99 of 1.12 ms, failing
both existing latency gates. This confirms the need for longer, controlled-host
qualification; the earlier favorable samples cannot establish a release claim.

These measurements make milestone 3's asynchronous recovery and predictive
scheduling a priority. Root/barrier localization and worker coordination changes
remain prerequisites to the global-publication redesign; adding workers alone
cannot satisfy the combined objective.

### Measurement method and pending qualification

`validation/measure.py` runs a prebuilt driver and saves raw histogram counts,
source and binary hashes, revision, compiler, platform, CPU affinity,
cgroup limits, seed, process CPU, peak RSS, and context switches. The current driver
also records every collector configuration field and one mutator. Process CPU/RSS
include setup and teardown. The source hash covers tracked and untracked Rust,
manifest, lock, and Python files; it is checked again when the matrix ends.

Earlier reports used a dynamically allocated tree with one-microsecond buckets.
Under overload, the wide arrival distribution grew that tree substantially,
adding recorder CPU and RSS to the experiment. Those capacity screens are
exploratory and must not be used as clean collector-capacity measurements. The
largest raw reports are stored as lossless `.json.gz` archives.

The final recorder preallocates 4,096 counters (32 KiB per histogram), with no
allocation while recording. Buckets are exact to one microsecond through 1,024 µs,
then use 64 subdivisions per doubling; reported upper bounds are within 1.5625%
of the rounded input above that boundary. The two release thresholds retain
one-microsecond precision, and maximum latency remains separately recorded in
nanoseconds. Tests cover threshold boundaries, maximum supported durations,
conservation of sample counts, and fixed capacity across a broad latency range.
Do not merge distributions from these two recorder versions.

The driver retains the intended arrival schedule when work falls behind. Operation
latency measures service time; arrival latency includes backlog accumulated since
the scheduled arrival. Unfinished arrivals appear in `backlog_operations`, not in
the completed-request latency histogram. The existing `enforce` switch checks
operation percentiles only. Collection statistics include teardown, while the
explicit diagnostics snapshot precedes teardown. Peak live/committed samples are
sampled every 1,024 operations; RSS comes from the process high-water mark.

For the initial comparison, flag throughput changes below 5% and percentile changes
below 10% as noise. These screening tolerances are declared before inspecting the
candidate results; they do not establish a qualification host or excuse a baseline
whose repeated variation is larger. Short development-host runs are insufficient
for the plan's combined throughput/latency gate or one-hour/24-hour qualification.
The physical cap on regions is distinct from RSS; metadata and the operating
system allocator remain outside the managed-byte limit.

Reproduce a comparison after building and while no other validation/build is running:

```sh
cargo build --release -p ristretto_classloader --example gc_endurance
python3 ristretto_gc/validation/measure.py --seconds 60 --workers 1 4 \
  --rates 10000 30000 --repeats 3 --output /tmp/gc-comparison.json
python3 ristretto_gc/validation/measure.py --seconds 60 --workers 4 \
  --rates 10000 --repeats 3 --diagnostics --output /tmp/gc-diagnostics.json
```

The endurance workflow now uploads both the reproducibility report and a separate
30-second diagnostic sample. Hosted CI records timings without enforcing
performance thresholds.

## Compactable backend validation (2026-11-06)

The backend is opt-in (`Configuration::builder().compact_heap(true)`). Java payloads
move; descriptors and arbitrary Rust values remain pinned. Independent young/full
marking and parallel native tracing/relocation are enabled for both backing-storage
choices. Terminal phases between generations remain coordinated.

Correctness checks performed during this implementation:

- 125 native GC unit/integration tests and one doctest passed. Eight compaction tests
  cover mutation between copy chunks, alignment, exactly-once destruction, failed destinations,
  pinning, region release, overlapping snapshots, request completion, and real native
  overlap in both tracing and relocation. Concurrent old-to-young mutation now uses
  four collector workers as well as four mutators.
- The equal-size fragmentation fixture retains half of 128 allocations in 16 regions.
  Compaction releases eight regions, reaching the theoretical eight-region minimum;
  dropping the survivors releases all payload regions and logical managed bytes.
- Classloader, intrinsics, types and VM library suites were exercised. A module identity
  hash regression exposed a remaining address-based hash path; that path was migrated
  and its failing test rerun successfully. The types suite passed 198 tests and the VM
  suite 894. Existing ignored tests were retained.
- Java reference/finalization fixtures passed on JDK 8, 11, 17, 21 and 25 with both
  backends and four native workers, including a 16 MiB allocation-pressure/OOM fixture.
- WASI passed 52 portable GC tests and two actual JVM reference/finalization tests with
  compactable payloads. Bare wasm executed the smoke export, including region release
  and payload-content checks. Both wasm VM targets passed Clippy; native workspace
  Clippy included all targets.

The additional unreachable-Reference-owner regression ensures that clearing a
conditional referent does not resurrect an unreachable Reference merely to enqueue it.
Stable identity tests reject expired IDs, wrong types and foreign-heap handles.
The basic VM reclamation test now waits for collection completion rather than assuming
that an asynchronous request completes within a 100 ms sleep. Native tracing/relocation
overlap and worker termination also passed 20 consecutive regression runs.

## Sustained workload and release gates

`gc_endurance` retains an old Java reference array while replacing slots with mixed-size
primitive arrays (including periodic 1 MiB allocations). It checks identities and bytes,
occasionally holds a guard across a collector step, and verifies zero live objects and
zero occupied region bytes after teardown. Seed: `0x53a907b10042ef15`. Histograms use
one-microsecond buckets and bounded storage. Reported operation latency includes host
allocation/initialization, root publication, stores, validation, contention and cooperative
assistance; it is **not** an isolated stop-the-world pause metric. Peak heap samples are
sampled every 1,024 operations. Committed payload bytes exclude descriptors, metadata,
and temporary host buffers, and are not RSS.

```sh
cargo build --release -p ristretto_classloader --example gc_endurance
# seconds, workers (0 = cooperative), offered allocations/s, heap MiB, compact, enforce
target/release/examples/gc_endurance 3600 4 10000 64 true false
target/release/examples/gc_endurance 86400 4 10000 64 true false
# Use true in the last position only on a controlled latency qualification host.
```

The new `gc-endurance.yml` workflow runs a one-hour nightly matrix with 0, 1,
and 4 workers and both storage choices, preserving JSON results. Manual workflow runs
accept durations up to 18,000 seconds. The 24-hour command above is a separate release
qualification run; it has **not** been completed in this session.

Release requires repeated p99.9 operation latency below 100 microseconds and p99.99 below
1 millisecond at a sustainable offered rate on the declared qualification host, no leaked
objects/ranges or failed content/reference checks, at least 90% of theoretically reclaimable
regions in the equal-size fixture, and successful longer endurance/platform coverage.
The executable's optional latency assertion enforces the two percentile thresholds.
Shared CI records measurements without treating noisy timing as a correctness test.
Compaction must remain opt-in until these gates pass. This work does not certify JVM
conformance or a universal maximum pause, and it does not implement C4's virtual-memory
mechanisms, old-only collection, segmented large arrays, or concurrent reference decisions
between generations.

### Current short-run measurements

Linux x86_64 development host, Rust 1.99.1, release build, 64 MiB managed limit,
1 MiB regions, 4 KiB copy chunks, 25 microsecond step target, work budget 64 and
50 microsecond native worker pacing (bypassed above 75% occupancy). Other builds
and tests were stopped for these runs. Each row is 20 seconds at an offered
10,000 allocations/second; all compacting rows completed 200,000 operations.

| Mode | Operation p99.9 (µs) | Operation p99.99 (µs) | Maximum (ms) | Peak live MiB | Relocations |
| --- | ---: | ---: | ---: | ---: | ---: |
| Cooperative, compact | 59 | 901 | 2.426 | 12.84 | 87,996 |
| 1 worker, compact, run 1 | 54 | 523 | 1.897 | 23.96 | 83,915 |
| 1 worker, compact, run 2 | 55 | 617 | 3.098 | 46.76 | 94,540 |
| 4 workers, compact, run 1 | 89 | 552 | 1.596 | 12.94 | 135,334 |
| 4 workers, compact, run 2 | 89 | 677 | 1.318 | 13.39 | 133,465 |
| 4 workers, host-backed payloads | 92 | 242 | 1.701 | 11.93 | 0 |

All rows ended with zero live objects and zero occupied payload bytes. The
cooperative row uses identical cooperative scheduling; native pacing is irrelevant
there. These short samples meet the proposed percentile thresholds, while maxima
remain in milliseconds. They demonstrate operation latency for this workload,
not a universal pause bound or completed endurance qualification.

[Raw measurements](validation/compaction-2026-11-06.json) include the earlier,
unfavorable development runs. Before work-accounting and pacing changes, a
controlled cooperative run reached 91.35 ms maximum and the four-worker p99.9
was 425 µs. Earlier runs overlapping other validation also had larger outliers.
These different snapshots must not be mixed into a single performance distribution.

## Earlier noncompacting baseline correctness coverage

The collector tests cover unreachable allocations before the first cycle, cycles, independent live-byte/debt accounting, promotion, clean-old-generation skipping, old-to-young stores, concurrent root publication, four native mutators, bounded array scans and disposal, dirty-card races between scan slices, weak/final/phantom ordering, finalizable Reference owners, weak allocation identity, root identifier rollover, typed runtime handles, heap exhaustion, and escaped handles after shutdown.

Java integration tests exercise class libraries for JDK 8, 11, 17, 21, and 25. They check weak clearing, queue delivery, finalization once, resurrection, phantom processing after resurrection ends, and finalizable WeakReference subclasses. A 16 MiB heap also checks catchable `OutOfMemoryError` during array allocation and cloning. VM shutdown tests check that daemons and class roots are released while escaped values remain usable. Existing JIT helper tests check rooted values across explicit collection without broadening production JIT eligibility.

Native suites passed:

| Suite | Passed | Ignored |
| --- | ---: | ---: |
| `ristretto_gc` unit/integration tests | 115 | 0 |
| `ristretto_gc` doctest | 1 | 0 |
| `ristretto_classloader` unit/integration tests | 754 | 14 |
| `ristretto_intrinsics` library | 2,728 | 0 |
| `ristretto_types` library | 198 | 0 |
| `ristretto_vm` library | 894 | 0 |
| VM `gc` and `gc_references` integration binaries | 6 | 1 |

The ignored integration entry is a subprocess helper fixture; the classloader ignores are existing tests. These suite results were collected during implementation; focused GC, clone, and Java integration checks were repeated after their subsequent changes.

Useful native reproduction commands:

```sh
cargo test -p ristretto_gc
cargo test -p ristretto_classloader --lib --tests
cargo test -p ristretto_intrinsics -p ristretto_types --lib
cargo test -p ristretto_vm --lib --test gc --test gc_references
cargo clippy --workspace --tests --examples -- -D warnings
cargo fmt --all -- --check
```

The native Java fixture requires a JDK 25 compiler and the five supported class libraries. The test helpers locate/download these using the repository's existing runtime cache.

## Earlier baseline wasm execution

Both targets compile the VM. Validation used Wasmtime 50.0.1:

- **`wasm32-wasip2`:** 43 portable GC unit and generational/reference integration tests pass in Wasmtime. A scheduler unit test verifies that daemon futures survive host returns and drop at shutdown. Two actual JVM integration tests execute weak queue delivery across host calls and finalization/resurrection/phantom ordering using the JDK 25 class library. The test generates its Java fixture class in memory, without requiring a compiler inside WASI.
- **`wasm32-unknown-unknown`:** the exported `gc_smoke` function executes in Wasmtime without a WASI clock or thread imports. It covers allocation accounting, heap limits, weak clearing, and an old-to-young edge. This is collector runtime coverage; it is not a browser-hosted full JVM test.

With `wasmtime` on `PATH` and both Rust targets installed:

```sh
CARGO_TARGET_WASM32_WASIP2_RUNNER='wasmtime run' cargo test -p ristretto_gc --target wasm32-wasip2 --lib --test generational --test weak_references
cargo build -p ristretto_gc --example wasm_smoke --target wasm32-unknown-unknown
wasmtime run --invoke gc_smoke target/wasm32-unknown-unknown/debug/examples/wasm_smoke.wasm
cargo clippy -p ristretto_vm --target wasm32-unknown-unknown -- -D warnings
cargo clippy -p ristretto_vm --tests --target wasm32-wasip2 -- -D warnings
```

The WASI JVM tests additionally need filesystem access to the checkout and Java cache, a larger wasm stack, and the linked WASI HTTP imports. Adapt these example paths to the host:

```sh
CARGO_TARGET_WASM32_WASIP2_RUNNER='wasmtime run -W max-wasm-stack=8388608 -S http --dir=/path/to/ristretto --dir=/path/to/java-cache --dir=/tmp --env=HOME=/path/to/home --env=RISTRETTO_GC_JAVA_HOME=/path/to/java-cache/jdk25' \
  cargo test -p ristretto_vm --target wasm32-wasip2 --test gc_wasi
```

CI executes the bare-wasm smoke and retains the existing WASI and native platform matrix. Other native OS/CPU combinations were not executed locally. Thread sanitizers, Miri, a browser JVM run, and a JVM conformance suite were not run.

## Earlier noncompacting baseline latency measurements

Environment: Linux 7.0.0-38-generic, x86_64 Intel Core i7-13700T, 24 logical CPUs, approximately 30 GiB RAM, Rust 1.99.1. These are release builds on a general-purpose development host, without CPU isolation. Three sequential runs of each example are reported, including outliers. No previous-collector baseline was measured.

```sh
cargo build --release -p ristretto_gc -p ristretto_classloader --examples
target/release/examples/latency
target/release/examples/gc_latency
```

`latency` retains a 10,001-object old graph while allocating, rooting, and releasing 100,000 objects with a native collector worker. This timer includes allocation, root registration/release, lock contention, and scheduling; it does not isolate a stop-the-world pause. The workload completed 6–7 young cycles and 2 full cycles per run. A separate cooperative workload times roughly 66,680 steps while collecting batches of dead vector payloads.

All values below are microseconds:

| Native allocation/root/release | p50 | p99 | p99.9 | Maximum |
| --- | ---: | ---: | ---: | ---: |
| Run 1 | 2.581 | 8.009 | 24.539 | 192.471 |
| Run 2 | 2.522 | 6.894 | 16.969 | 173.062 |
| Run 3 | 2.572 | 7.024 | 15.107 | 325.142 |

| Cooperative collection step | p50 | p99 | p99.9 | Maximum |
| --- | ---: | ---: | ---: | ---: |
| Run 1 | 0.839 | 1.442 | 5.401 | 32.052 |
| Run 2 | 0.759 | 1.189 | 5.103 | 337.688 |
| Run 3 | 0.726 | 1.077 | 4.176 | 322.147 |

`gc_latency` uses the actual Java array representation with 1,000,000 reference slots, 10,000 point stores, periodic young collections, and full collection/disposal of the array and its children. Each run records 7,941 young steps and 10,314 final collection/disposal steps. It does not include interpreter execution.

| Million-slot Java array workload | Run 1 p99.9 / max | Run 2 p99.9 / max | Run 3 p99.9 / max |
| --- | ---: | ---: | ---: |
| Point store | 0.309 / 4.619 | 0.493 / 2.558 | 0.158 / 0.794 |
| Young collection step | 22.886 / 46.201 | 17.935 / 23.216 | 17.732 / 61.604 |
| Full collection/disposal step | 2.655 / 1,353.540 | 2.831 / 1,493.129 | 3.564 / 1,468.951 |

The tables show three runs after the final guard-tracking and reference-processing fixes. An earlier three-run validation on the same host observed a 2,987.011 µs native allocation/root/release maximum and a 1,729.574 µs large-array collection/disposal maximum. Those outliers are retained here as additional evidence against interpreting the soft target as a maximum pause bound.

These workloads demonstrate low ordinary tail latency and generational efficiency, with millisecond outliers. They do not establish an ultra-low maximum pause across workloads or platforms. The work budget cannot bound host allocator calls, final backing-store deallocation, OS scheduling, or arbitrary custom Rust callbacks. Explicit full collection and heap-pressure recovery remain synchronous. No cause is assigned to individual outliers without profiling.

## Cost-aware evacuation and bounded reserve — 2026-11-07

Final source `1383ac329b0137705a87b2e5e4abee419b016e60b232b37689851293ff162a1e`
adds opt-in full-cycle selection of at most four sparse sources, sampled hot/pin
feedback, and an optional protected destination reserve shared by payload and
descriptor arenas. The collector builds its cost index incrementally; ordinary
allocation/free no longer updates a second sorted tree. Role checks restart stale
partial copies and prevent destinations from becoming sources in the same selection.
Backing credits refund after actual deallocation, including retained cache tails.
Empty oversized backing retires before small copies can strand it. Pure movement
of live data cannot indefinitely reset physical recovery's failure allowance.

The [verification record](validation/evacuation-verification-2026-11-07.txt) records
native, Java reference, WASI, bare-wasm, Miri, and ThreadSanitizer checks and the
failures fixed during development. The full native suite passes 251 tests including
doctests; the final selector's region tests pass under Miri and ThreadSanitizer.
Equal-size fragmentation still releases all eight theoretically reclaimable regions
from sixteen. Low-headroom, pin/hot coexistence, cross-cycle partial copies,
overalignment, cancellation, and exactly-once destructor fixtures pass. The VM's
16 MiB capacity-edge clone fixture uses zero reserve: reserving 1 MiB makes its
initial 9 MiB source exceed ordinary backing capacity. The preceding JDK reference
matrix continues to exercise the 1 MiB reserve.

Two reversed-order six-second repeats use CPUs 0 and 2, workers 0/1/2/4, context
roots, 32 barrier records, prediction, independent marking/relocation, batch size 8,
16 metadata shards, and work stealing. Local allocation caches and adaptive workers
are off. Recovery uses a 16 MiB heap and 4 KiB payloads; mixed allocation/mutation/
pinning uses 32 MiB and 10,000 scheduled arrivals/s. Diagnostics are enabled.
The preserved `beae30ff` baseline's binary hashes are verified; the new runner hash
is recorded separately. There were no concurrent builds, tests, or source edits.

The initial implementation (`436fa39e…`) has 48 retained results in
`validation/evacuation-initial-*.json`. Its native recovery throughput fell 5.2–7.5%
even with selection disabled because diagnostics allocated per-payload history.
Enabling selection plus a 1 MiB reserve produced operation p99.99 of 3.09–3.23 ms.
The revised implementation allocates history only when selection is enabled and
bounds source exclusion while building the index on the collector.

The revised 48 runs are in `validation/evacuation-bounded-*.json`. Values below are
means of two run quantiles; latency units are microseconds. “On” includes a 1 MiB
reserve. These are short screens, not qualification distributions.

| Workers | Recovery allocations/s baseline / off / on | Operation p99.99 baseline / off / on | Mixed arrival p99.9 baseline / off / on |
| --- | --- | --- | --- |
| 0, cooperative | 497,651 / 500,977 / 480,722 | 1,072 / 1,072 / 1,152 | 36,608 / 37,632 / 34,944 |
| 1 | 424,752 / 431,507 / 415,999 | 654.5 / 676 / 2,768 | 564.5 / 563.5 / 414 |
| 2 | 423,770 / 418,487 / 408,907 | 569.5 / 584.5 / 2,768 | 718 / 472 / 494.5 |
| 4 | 407,913 / 412,642 / 387,724 | 533 / 552 / 2,864 | 347.5 / 615 / 446.5 |

An additional twelve paired native recovery runs in
`validation/evacuation-unreserved-*.json` isolate selection with zero reserve.
At 1/2/4 workers, off→on throughput is 432,924→418,920 / 429,293→411,511 /
419,503→398,670 allocations/s, and p99.99 is 678→2,544 / 579→2,416 /
544.5→2,512 µs. Reducing ordinary capacity is therefore only part of the observed
recovery-tail cost. Physical allocation waits still await full-cycle completion;
the measurements do not prove that changing that wait alone will meet the gates.

All 108 runs exit successfully with empty stderr, zero final offered-load backlog,
and zero live/occupied/shared backing after teardown. Shared peak backing stays
within the configured cap. Revised process RSS peaks at 57,794,560 bytes, including
metadata and driver storage outside the arena cap. Revised mixed/pressure runs
record up to 27 hot-region skips and two resolved pin-deferral samples per run.
Total copied bytes divided by gross evacuated source bytes ranges from 0.118 to
1.380 where the denominator is nonzero; it includes young packing and is **not**
net memory saved or an isolated evacuation-copy efficiency measurement. Destination
failures, recopy volume, committed peaks, reserve usage, selection capacity, and
pin deferral are preserved in each result. Pin deferral includes scheduling and
cooldown and only counts resolved samples; exact guard-hold timing remains pending.

The throughput/latency targets and one-hour/24-hour gates remain unmet. Selection
and the reserve remain disabled/zero by default. The publication gate, payload
locks, phase coordinator, and generation interlocks remain in place.

## Early physical recovery and managed guard timing — 2026-11-07

Source `e0cb1ecef2eca75d888073518fc52c2f365ab2c24427da8593b3aec92f44f22a`
adds bounded layout-aware physical wakeups, completed-cycle retry accounting, and
actual managed guard-hold samples when diagnostics and cost-aware compaction are
both enabled. Offers grant no range ownership; stolen offers retry safely, and
cooperative retries still advance collection. Mapped guards and projection
unwinding preserve one sample. Bare wasm records counts without host-clock calls.
The [verification record](validation/physical-early-verification-2026-11-07.txt)
records 256 passing GC tests, classloader/VM integration, WASI/bare wasm, Miri,
Clippy and focused ThreadSanitizer checks. Full TSan stress stopped at an overly
strict prediction-request assertion, with no race reported; that assertion was
corrected and the native suite passed, but the corrected full sanitizer rerun
remains pending.

`validation/physical-early-{recovery,endurance}-*.json` preserves 64 paired runs
against the hash-verified `1383ac` binaries. Each combination has two reversed-order
six-second repeats on CPUs 0 and 2, using the preceding section's configuration
and budgets. Both binaries run with cost selection off and on; on includes the
1 MiB reserve. No builds, tests or source edits ran during measurement. Values
below are means of run quantiles, in microseconds, with diagnostics enabled.

| Workers | Recovery allocations/s baseline on / candidate on | Operation p99.99 baseline on / candidate on | Mixed arrival p99.9 baseline on / candidate on |
| --- | --- | --- | --- |
| 0, cooperative | 466,034 / 431,919 | 1,192 / 1,232 | 32,384 / 33,408 |
| 1 | 389,441 / 381,031 | 2,880 / 196.5 | 620 / 744.5 |
| 2 | 388,527 / 368,359 | 2,864 / 149.5 | 712.5 / 697 |
| 4 | 365,857 / 347,476 | 2,944 / 235 | 505 / 453 |

Native recovery recorded 953–1,147 early physical wakeups across each worker
count's two on-policy runs. Native recovery tails improve substantially, while
throughput falls 2.2–5.2%; cooperative throughput falls 7.3%. With cost selection
off, throughput changes −1.2% to +1.4% and native p99.99 changes 704→722.5,
590→669 and 573→639 µs. These mixed results do not meet the combined improvement
targets. Mixed arrivals still miss the 100 µs p99.9 gate.

Sixteen additional paired runs in `validation/physical-early-overhead-*.json`
compare this candidate's diagnostics disabled/enabled, retaining the on-policy
configuration. Throughput is 464,872/437,267 (cooperative), 412,455/390,314,
403,672/374,439 and 381,252/361,874 allocations/s at 1/2/4 workers. All diagnostics,
including guard timing, cost 5.1–7.2% in this screen; it does not isolate guard
timing alone. Native p99.99 remains 157–222 µs with diagnostics and 167.5–208.5 µs
without. The mixed on-policy runs record 3,094,007 guard samples, with a maximum
observed hold of 634,535 ns. These include managed tracing reads and scheduling
inside a hold; raw relocation locks are excluded. They are not global pauses.

All 80 runs exit successfully with empty stderr, zero final live/occupied storage,
and zero roots, caches, retired objects, waiters, work and active guards after
teardown. Every mixed run finishes with zero offered backlog. Shared peak backing
stays within each configured cap. Ordinary guard sampling remains disabled with
diagnostics off. Cost selection remains false and the reserve zero by default;
no hour-long or 24-hour qualification or publication-gate removal is established.

## Construction roots and publication-free drains — 2026-11-07

Source `3672fe96cb7fb491372820a1cc1d194fcb55c4e8e9d9928e838ab4d00f33baea`
transfers roots released by ordinary construction hooks to a stack-scoped owner.
Hooks and system allocation no longer retain publication admission for strong-edge
construction. Existing registrations transfer directly; explicit root removals
preserve replacement roots. Conditional-reference owner assignment deliberately
retains its reader through metadata registration, preserving weak decisions and
queue delivery. Standalone context drains now use their pending reservations
without publication admission; closure checks pending before queue emptiness.

The [verification record](validation/construction-verification-2026-11-07.txt)
records the full native suite and final focused checks, 757 classloader tests,
896 VM library tests, VM GC/reference integration, WASI/bare wasm and Miri. The
48,000-allocation ThreadSanitizer recovery matrix passes in 959.32 seconds, followed
by compaction, construction, contexts and diagnostics. The final conditional-owner
safeguard has separate library, Java reference, WASI, Miri and TSan checks. Clippy,
format and diff checks pass. The five new admission models describe a proposed
protocol only. Loom rejected the initial late count-check ordering; the corrected
models pass, but the runtime still uses the global gate at phase boundaries.

`validation/construction-{recovery,endurance}-*.json` preserves 64 runs against
hash-verified `e0cb1e` binaries, using the previous section's CPU, heap, load and
feature configuration. There are two reversed-order six-second repeats per case;
no source edits, builds or tests overlapped measurement. “On” means cost-aware
relocation plus the 1 MiB reserve; diagnostics remain enabled. Latencies below are
means of run quantiles in microseconds.

| Workers | Recovery allocations/s baseline off / candidate off | Operation p99.99 baseline off / candidate off | Recovery allocations/s baseline on / candidate on | Operation p99.99 baseline on / candidate on |
| --- | --- | --- | --- | --- |
| 0 | 452,187 / 457,175 | 1,200 / 1,168 | 413,699 / 419,960 | 1,312 / 1,296 |
| 1 | 396,862 / 404,968 | 700.5 / 758 | 370,350 / 391,822 | 182 / 172.5 |
| 2 | 389,240 / 406,077 | 599.5 / 664.5 | 363,054 / 385,384 | 231 / 136 |
| 4 | 380,413 / 390,267 | 558 / 660.5 | 339,369 / 359,110 | 218 / 230 |

Native throughput improves 2.0–4.3% off and 5.8–6.2% on. Off-policy p99.99 worsens
8.2–18.4%, and the on-policy four-worker tail worsens 5.5%. Mixed arrival p99.9
is also mixed: off-policy baseline/candidate is 606/722, 674/704.5 and 1,354/666 µs
at 1/2/4 workers; on-policy is 716.5/466.5, 589.5/718 and 507.5/525.5 µs. Cooperative
mixed p99.9 remains 34–40 ms. The combined throughput/latency targets remain unmet.

Every run exits successfully with empty stderr, zero final live/occupied backing,
roots, caches, retired storage, queued/in-flight work, waiters and active guards.
Mixed workloads finish at 10,000 operations/s with zero final backlog. Peak shared
backing stays within its cap; candidate process RSS peaks at 55,365,632 bytes.
This is another short screen, not hour-long/24-hour qualification. No experimental
configuration default changes and no complete publication-gate removal are claimed.

### Legacy constructor ownership correction (2026-11-07)

Source `073212de43d6437884cef0aa0b6c857b5b8bc5aaa97843a1443b59ffe4787e63`.
All four legacy constructors now own initialized storage and an uncommitted logical
reservation through attachment and metadata preparation. Partial metadata unwind
drops its owned allocation; successful publication commits accounting afterward.
The 48-case matrix covers three root policies, four hook outcomes, rooted/unrooted
construction, and optional finalization. GC, classloader, VM, WASI, Miri, Clippy,
formatting and diff checks pass; details are in
[legacy-construction-verification-2026-11-07.txt](validation/legacy-construction-verification-2026-11-07.txt).
No performance improvement is claimed for this ownership correction.

### Frozen reference boundaries and logical handle retirement (2026-11-07)

Source `da43cd96dc2ec8df2ab94fac0aaf4cbda950b1af8ca29f475f9277a8ab99b114`.
A forced regression reproduced a Final upgrade reversing an earlier logical Weak
clearing decision. Existing mark words now distinguish strong reachability from
final/phantom retention. New references created after a boundary defer to the next
cycle; cloning preserves the original decision. Bounded rollover scans prevent
mark and slot-birth epoch aliasing. A second regression reproduced numeric-handle
resurrection of an untraced graph after strong closure. Such reconstruction now
fails; retaining references can upgrade before sweep, and dead-edge barriers cannot
revive graphs after sweep begins.

Forced cases cover young/full cycles, all root policies, soft/weak/phantom decisions,
retained descendants, fresh references to resurrected targets, exactly-once queue
delivery, and counter rollover. Full GC, VM library and five-JDK reference tests,
WASI, bare wasm, focused Miri and ThreadSanitizer checks pass. See
[weak-boundary-verification-2026-11-07.txt](validation/weak-boundary-verification-2026-11-07.txt)
for scopes and pre-correction reproduction logs. This frozen snapshot still uses
the global gate and makes no new performance claim.

### Experimental counted publication (2026-11-07)

Source `39ac111d3f1dd23a0f5502bbd81845342dc72ca8503b325d6d5a49129c9eb0bb`
integrates the production revision/phase clock, counted admission, per-slot reference
decisions and expected-identity reconstruction. The legacy gate remains selectable
and enabled by default. [The verification record](validation/publication-verification-2026-11-07.txt)
includes the full GC suite, both 48,000-allocation policies, ten shared-clock Loom
models, classloader tests, both policies across five supported JDKs, focused Miri,
WASI/bare wasm, and ThreadSanitizer. The counted 48,000-allocation TSan matrix passes
in 887.95 seconds; the final focused sanitizer suite includes the subsequent
defensive slot-lock poison recovery. A forced test holds the legacy writer locked
through unrelated allocation, weak operations, collection and teardown.

`validation/publication-{recovery,endurance}-{baseline,gated,counted}-{host,compact}-2026-11-07.json`
preserves 96 runs: two reversed-order six-second repeats at 0/1/2/4 workers, with
CPU affinity 0,2. The hash-verified baseline is construction snapshot `3672fe`;
gated and counted use the same candidate binaries. Configuration retains context
roots, 32-record buffers, prediction, independent marking with batches of eight,
16 metadata shards, stealing, independent relocation and diagnostics. Local caches
and adaptive workers are disabled. Compact storage also enables cost selection and
a 1 MiB reserve. Recovery uses a 16 MiB heap, 4 KiB payloads and 512 retained roots;
mixed endurance uses 32 MiB at 10,000 offered operations/s. No builds, tests or
source edits overlapped measurement. Values below are means of run quantiles,
not quantiles computed from a merged histogram; latencies are microseconds.

| Workers | Host recovery allocations/s baseline / gated / counted | Host operation p99.99 baseline / gated / counted | Compact recovery allocations/s baseline / gated / counted | Compact operation p99.99 baseline / gated / counted |
| --- | --- | --- | --- | --- |
| 0 | 323,622 / 323,774 / 325,366 | 152 / 151.5 / 152 | 424,060 / 430,129 / 425,473 | 1,280 / 1,256 / 1,296 |
| 1 | 304,340 / 307,537 / 316,303 | 30 / 30 / 30 | 396,047 / 384,582 / 416,301 | 153 / 179 / 216 |
| 2 | 302,742 / 304,230 / 315,779 | 33 / 32.5 / 31 | 382,937 / 381,706 / 400,270 | 150.5 / 138 / 194 |
| 4 | 299,708 / 298,753 / 314,552 | 36.5 / 37 / 35 | 372,119 / 371,371 / 391,134 | 209 / 195.5 / 232 |

Against the earlier baseline, counted native recovery throughput improves 3.9–5.0%
with host storage and 4.5–5.1% with compact storage. Compact p99.99 worsens
11.0–41.2%. Within the candidate, counted throughput improves 2.9–5.3% host and
4.9–8.2% compact over the gate; compact tails worsen 18.7–40.6%. These short
comparisons do not establish the combined improvement target or baseline noise.

Mixed arrival p99.9 at 1/2/4 workers is 1,480/1,496/1,536 for the host baseline,
1,880/1,872/1,544 gated and 1,600/1,496/1,512 counted. Compact values are
543.5/786.5/651 baseline, 459/761/616.5 gated and 513/493.5/643 counted.
Cooperative mixed p99.9 spans 19–35 ms. All configurations miss the 100 µs gate.
All runs exit successfully with empty stderr and zero final live/occupied storage,
roots, caches, pending records, retired storage, queued/in-flight work and active
guards/admissions. Mixed runs finish with zero backlog except one four-worker
compact baseline run with one outstanding operation; peak shared backing stays
within its cap. Counted runs record zero publication-lock waits. Endurance diagnostics
sample active work before shutdown; the `roots.final_*` fields record teardown.

A stalled counted admission can still delay phase closure. Generation terminal
interlocks and guarded payload access remain; no universal stall bound, old-only
collection, sustained-capacity result or hour-long/24-hour qualification is claimed.

### Exact allocation identity in tracing queues (2026-11-07)

Source `6eb5041a995a3e36d2fadf82c0120a103d8e7c8fc8c9f28409df7d21973c5820`
retains exact allocation metadata in queued tracing and continuations. Claiming
needs no address lookup; a retired allocation cannot lease a replacement's storage.
Remembered pruning validates identity alongside mutation version. A forced reuse
fixture verifies stale callback rejection, preservation of the replacement's
remembered entry and exactly-once destruction under both publication policies.
The dequeue/reservation fixture still pauses the actual claim before reservation.

The [verification record](validation/identity-queues-verification-2026-11-07.txt)
records the complete GC suite, including both 48,000-allocation stresses, focused
TSan library/compaction/generation suites, Miri's retired-task fixture, WASI library
and generation tests, Clippy, formatting and whitespace checks. Fresh queue entries
now store one metadata `Arc` rather than a type-erased trace descriptor. Metadata
ownership alone does not retain retired storage. No throughput gain or generation
independence is claimed for this prerequisite.

### Conditional-reference owner identity (2026-11-07)

Source `ae4326d9154726940cfb85f773f7ea6b04f2623db5d7a7af75b56a4bad5fae97`
fixes a reproduced stale-owner bug: a surviving conditional slot could enqueue an
unrelated replacement at its former owner's address. Slots now retain owner identity;
constructor registration binds pending slots before releasing construction admission.
Failed attachment remains unbound. Liveness and queue insertion reject expired
identities, including for final and retained-phantom edges.

The [verification record](validation/reference-owner-verification-2026-11-07.txt)
includes the failing reproducer, complete GC suite, supported-JDK reference matrix,
focused TSan, Miri, WASI, Clippy and formatting. A fixture correction limits intentional
panic/unwind checks to native targets; WASI validates successful attachment because
its panics abort. Both publication policies and all five reference kinds are covered.
The generation policy and defaults remain unchanged; no performance gain is claimed.

### Reachability strength across generations (2026-11-07)

Source `8fac978c452b31c102b67274fff103f25b0649c1ea0232299330584c946dd453`
fixes a reproduced overlapping-cycle bug: young final retention could become strong
reachability in the full snapshot, incorrectly preserving an old weak referent.
Tracing, cross-snapshot roots and promotion now preserve retention strength, clamped
to the receiving domain's boundary. A stronger discovery before closure queues a
fresh traversal, including earlier children; weaker work cannot downgrade it.
Root/barrier/allocation publication establishes its own strength independently of
an enclosing callback.

The [verification record](validation/cross-strength-verification-2026-11-07.txt)
includes the failing schedule, full GC suite and both 48,000-allocation policies,
classloader tests, the five-JDK reference matrix, focused TSan, six Miri reference
fixtures, WASI and bare wasm, Clippy and formatting. Thirteen reduced models compile
the production publication clock and mark word with Loom atomics. The new fixture
also roots the retained parent after its first scan and requires its earlier child
to be retraced strongly. These models supplement the forced schedules; they do not
prove the entire graph/reference algorithm. Generation interlocks remain unchanged
in this snapshot, and no performance improvement is claimed.


## Young progress during full reclamation (2026-11-07)

Source `1cc36154bb9ed5ed62ae2b6f69f12b88075cfe2fc796d9634f981a26bf5bc913`
adds opt-in `overlapping_generations`, default false. Full stage-5 sweeping/copying
now gives young cycles turns and uses per-domain closure reservations. New births
stay young and are excluded from full copying; the traced full snapshot protects
copy callbacks' outgoing graphs. Strong/reference interlocks remain. Forced tests
cover held clearing, held copying after root removal, post-trace birth reclamation,
no premature promotion, dead old Reference owners, persistent-copy fairness and
shared evacuation ownership. See `generation-overlap-verification-2026-11-07.txt`
for full native GC, supported-JDK references, Miri, TSan, WASI and bare-wasm checks.
The TSan 48,000-allocation stress passed in 1,059.63 seconds.

Eight `generation-overlap-{recovery,endurance}-{off,on}-{host,compact}-2026-11-07.json`
reports preserve 64 paired runs: two six-second repeats, reversed policy order,
workers 0/1/2/4 on CPUs 0 and 2. Both policies use the same frozen binary, counted
publication, context roots, 32-record buffers, prediction, independent marking and
copying, batch 8, sixteen metadata shards and stealing. Recovery uses 16 MiB,
4 KiB payloads and async allocation; mixed endurance uses 32 MiB and 10,000 offered
operations/s. Compaction uses cost selection and a 1 MiB reserve. Diagnostics are
on, local caches/adaptive workers off. These are short screens, not qualification.
Process caps were not added to this already-frozen comparison.

Mean recovery allocations/s and operation p99.99 (off → on):

| Backing | Workers | Allocations/s | p99.99 µs |
| --- | ---: | ---: | ---: |
| Host | 0 | 309,717 → 305,590 | 142.5 → 144 |
| Host | 1 | 296,523 → 298,538 | 33.5 → 33.5 |
| Host | 2 | 294,569 → 294,377 | 35.5 → 35 |
| Host | 4 | 292,279 → 289,517 | 37.5 → 37.5 |
| Compact | 0 | 424,502 → 465,392 | 1,264 → 594.5 |
| Compact | 1 | 420,653 → 443,323 | 234.5 → 44 |
| Compact | 2 | 414,968 → 435,154 | 259 → 47 |
| Compact | 4 | 402,500 → 410,438 | 231.5 → 63 |

Worker-backed mixed arrival p99.9 means (1/2/4 workers) are host
1,928/2,192/1,920 → 1,536/1,928/1,552 µs and compact
852/650/406.5 → 598/525/692.5 µs. Cooperative arrival tails remain tens of
milliseconds. All 64 runs succeeded without stderr, final backlog or leaked
managed/region/root/retired/work storage; shared peak backing stayed within the
managed cap. Active-run diagnostic samples remain distinct from teardown checks.
The mixed latency gates still fail. No default changed.

The saved binaries in `/tmp/ristretto-gc-generation-overlap-baseline` have build
provenance embedded in every report, including an actual complete tracked and
untracked non-ignored source archive. Its SHA-256 is
`fc61317086660c2f076fcc0ae4f6fff3aaf043495c06f6bb43d392b16701dc1c`.
This archive includes Java fixtures and documentation, unlike the implementation
hash alone. Long endurance and the remaining milestones are still outstanding.


## Qualification capture and declared capacity checks (2026-11-07)

[validation/QUALIFICATION.md](validation/QUALIFICATION.md) documents the new
source/binary freezer, per-child address-space cap, RSS sampling, bounded heap and
backlog histories, and the criteria declared in `validation/qualification.json`.
The five-percent baseline p99.99 noise tolerance is fixed before qualification;
shared CPUs may fail that stability requirement. The process cap is virtual address
space, not an RSS cap. The source hash now covers Java/workflow inputs and the
qualification policy. Full source archives include the actual files, not just a
hash of a dirty working tree. The earlier overlap archive is preserved durably as
`validation/snapshots/1cc36154-source.tar.gz` with the hash recorded above.

The endurance driver now selects synchronous embedding or asynchronous VM
construction/publication while retaining the same arrival schedule, sizes,
mutation, pinning and seed. Native async mode asserts no synchronous allocation
recovery. It records per-completion peak backlog and bounded time-series samples;
disabled diagnostic fields are null instead of being mistaken for measured zeros.
The sample vector's capacity and effective interval are reported. Capacity analysis
checks completion rate, backlog growth/bursts, logical/retired/arena memory trends,
RSS trends, source/resource evidence and teardown, separately from latency gates.
It requires three sufficiently long repetitions and does not count duplicate input
reports as additional evidence. CI preserves frozen sources, binaries and provenance.

Four tooling regressions pass: actual child address-space exhaustion, timeout and
reaping, deterministic archives with dirty/untracked Java and broken symlinks, and
rejection of growing backlog or missing retirement evidence. A two-second async
smoke with zero and one worker, compact regions, both distributed policies,
counted publication and overlap passed under the 2 GiB process cap with zero final
managed objects/ranges. It ran alongside correctness checks and is functional
evidence only. No controlled capacity, one-hour or 24-hour result is implied.

## Distributed region registries and phase units (2026-11-07)

Frozen implementation source `da1d5c71205f940f67227e5f35df36cfeb4fd7edb1a1541e1df799cc67ffce04`
adds selectable region registries and independent root/reference/sweep units.
[Verification record](validation/distributed-verification-2026-11-07.txt) preserves
individual test results. The [source archive](validation/snapshots/da1d5c71-source.tar.gz)
contains 2,326 files and has SHA-256
`705a7a67869426960952f47aaef7c19d8351eba0cb8164aa9ba095ff00356cf1`.

The GC suite passed with 116 library tests before the additional root-gap fixture;
all four phase fixtures subsequently passed natively, under Miri and TSan. The
integrated 48,000-allocation TSan stress passed in 1,970.82 seconds. Classloader
verification passed 757 tests (14 ignored), VM library/GC verification passed
896 + 3 tests, and the supported-JDK reference driver passed 3 tests (1 ignored).
WASI passed 94 library, 11 compaction, 10 generation and 6 weak-reference tests.
Bare wasm `gc_smoke`, workspace Clippy, final GC/classloader Clippy and formatting
checks passed. Measurement tooling's four regressions also passed.

The performance runner uses an isolated copy reconstructed from that archive;
its implementation hash was verified before measurement. Subsequent old-only
collector development therefore cannot change these binary or runner inputs.
All 96 paired six-second control/region-only/distributed screens completed after
builds and correctness jobs finished. Each process used affinity CPUs 0 and 2 and
a 2 GiB `RLIMIT_AS` cap; every run exited cleanly and released all managed objects
and occupied ranges at teardown. The first timing attempt overlapped a subsequent
build and was invalidated; its partial output remains in
[distributed-interrupted-2026-11-07](validation/distributed-interrupted-2026-11-07/).
Only the complete rerun below is used for comparisons.

[Distribution summary](validation/distribution-summary-2026-11-07.json) binds all
12 raw report files by hash and records CPU, peak RSS, coordinator misses and means
of both repetitions. All three policies already enable counted publication,
generation overlap, context roots, buffered barriers, independent marking/relocation
and prediction. The control uses one region registry and coordinated phase scans;
“region shards” uses sixteen registries; “distributed” also enables independent
phase units. This isolates distribution from the later old-only candidate.
Recovery is asynchronous; worker zero is cooperative and has a separate contract.
The table reports arithmetic means of per-run throughput and quantiles, not pooled
latency distributions.

| Backend / workers | Control alloc/s | Region shards alloc/s | Distributed alloc/s | Control / shards / distributed p99.99 (us) |
| --- | ---: | ---: | ---: | ---: |
| host / 0 | 320,227 | 317,159 | 320,923 | 129.5 / 131 / 136 |
| host / 1 | 309,869 | 314,023 | 305,940 | 30.5 / 29.5 / 29.5 |
| host / 2 | 311,015 | 314,459 | 306,928 | 31.5 / 32 / 32.5 |
| host / 4 | 313,059 | 312,059 | 309,712 | 34.5 / 34.5 / 35.5 |
| compact / 0 | 496,306 | 490,588 | 489,070 | 560.5 / 558 / 566.5 |
| compact / 1 | 467,248 | 466,389 | 465,123 | 44 / 32 / 34 |
| compact / 2 | 447,993 | 451,737 | 454,565 | 53.5 / 61.5 / 54 |
| compact / 4 | 438,423 | 445,849 | 435,680 | 58.5 / 63 / 76 |

The two-worker compact recovery screen improves throughput 1.5% with the distributed
policy while p99.99 is nearly unchanged. Four-worker compact p99.99 regresses
from 58.5 to 76 microseconds; host throughput falls about 1–1.3% with native workers.
These mixed short results do not meet the throughput improvement target.

Synchronous mixed endurance at 10,000 offered operations/second remains a separate
compatibility screen. Host-backed mean p99.99 stays roughly 2.37–2.38 ms with native
workers, above the one-millisecond gate. Native compact mean p99.99 ranges from
474 to 515.5 microseconds for the distributed policy; worker-zero mean is 1,015.5
microseconds. These six-second runs do not establish capacity stability, recorder
overhead, one-hour endurance or the 24-hour gate. No defaults have changed and no
complete milestone has been performance-qualified.

### Conservative old-only passes and reference-free buffers (2026-11-07)

The integrated candidate is frozen at implementation SHA-256
`89728d07d3961749e938bb6b99b0a687069d535e4645d884950019549f65a686`.
The reconstructible archive is
[89728d07-source.tar.gz](validation/snapshots/89728d07-source.tar.gz), SHA-256
`35ec1be6185de3a78837f801be88057f5d0752fd56347017abd139fd95096fc1`
(2,339 files). Independent binary hashes and compiler information are saved in
[endurance provenance](validation/old-only-gc_endurance-2026-11-07.provenance.json)
and [recovery provenance](validation/old-only-gc_recovery_latency-2026-11-07.provenance.json).
The source digest covers implementation, tests, tools and the qualification policy;
the archive additionally preserves documentation and prior evidence.

`old_collection_interval(N)` schedules conservative old passes between precise full
intervals. A bounded initial-young handoff feeds old-only tracing queues; promotion
publishes old liveness before age and queues work afterward. Young collections can
complete during unrelated old tracing, sweeping and copying. Per-object scan locks
protect conflicting remembered graphs. Conditional old referents remain retained
until precise full processing; old passes never satisfy full-completion tickets.
See [the protocol](PUBLICATION.md#old-only-handoff-and-precise-fallback).

Forced pressure testing exposed conservative young roots being inherited by
successive precise full recoveries, causing premature OOM. The corrected full
handoff drains active young work before starting and defers new young starts
through full reference closure. A separate correction prevents remembered scans
from borrowing logically retired owners: a storage lease alone cannot protect
their dead outgoing graph.

Primitive `Vec`/`HeapBuffer` traversal now skips reference-free elements through
`Trace::may_contain_gc_references()`. Clearing is constant work only for
reference-free elements that need no destructor; destructor-bearing elements still
clear in bounded batches. Before this correction, the sanitizer pressure fixture
exceeded its existing allocation-wait deadline while traversing empty byte edges.
With the correction, the same 48,000-allocation stress passes in 17.93 seconds with
the ten-second waiter deadline unchanged. This correctness-run duration is not a
Java throughput comparison; Java primitive arrays already used leaf tracing.

[Verification details](validation/old-only-verification-2026-11-07.txt) record:

- Full native GC tests: 125 library tests plus all integration and documentation
  suites. The subsequently expanded admission suite passes all 18 Loom models.
- Classloader: 757 passed, 14 ignored. VM: 896 library and 3 GC tests passed.
  The supported-JDK reference driver passes 3 tests, with 1 ignored.
- ThreadSanitizer: 125 library, 13 compaction, 13 generational, 11 tracing and
  6 weak-reference tests, plus the 48,000-allocation stress across 24 policies.
- Miri: all 8 new old-generation schedules pass in 285.06 seconds, with the
  existing integer-to-pointer/exposed-provenance caveat.
- WASI: 100 library, 11 compaction, 11 generational and 6 weak-reference tests.
  Native-only tracing fixtures are excluded there. Bare wasm builds and invokes
  `gc_smoke` successfully with old-only scheduling enabled.
- Workspace Clippy with warnings denied, formatting and diff checks pass.
  Nine qualification-tool regressions pass. Four one-second runner smoke checks
  match their requested policies, resources and modes; they do not qualify timing.

The repeated blocking audit finds no counted-policy dependency on the legacy
writer. Forced collector tests hold that writer through ordinary publication,
weak operations, full collection and old-only collection. The eighteen reduced
models include late context attachment, detached buffered records, two-generation
retention, promotion and scan conflicts. They supplement the full-collector tests;
they are not an exhaustive model of arbitrary user callbacks.

The predeclared [qualification policy](validation/qualification.json) and
[resumable runner](validation/QUALIFICATION.md) now bind every job to its source,
binary, actual configuration, host and process cap. The sequence records recovery,
instrumentation overhead, capacity, three paired one-hour runs and a separate
24-hour candidate run. Long runs require a common capacity rate passing both
policies' stability and latency gates. Failed/interrupted runs remain evidence;
missing or noisy observations cannot pass.

The integrated sequence started after the separate frozen distributed-phase
screens completed. Results are in [qualification-old-only-2026-11-07](validation/qualification-old-only-2026-11-07/).
No build or correctness job overlaps those timings. Its manifests, raw reports,
`driver.log` and `assessment.json` show actual progress; queued work is not a
completed gate. Old-only scheduling, counted publication, independent phases,
sharded regions and compaction retain conservative defaults. Individual payload
locks, conflicting scans, synchronous embedding recovery and exhaustion waits
remain documented limitations.

## Isolated catalog/phase experiments (2026-11-14)

The [experiment README](validation/catalog/README.md) contains commands and scope
for the frozen numeric descriptor prototype: 11 native fixtures, eight models,
eight WASI fixtures, eight Miri fixtures and eight Tree Borrows fixtures pass.
The retained tree alternative passes three native fixtures and three models.
Clippy and bare-Wasm compilation pass. Models use the exact transition source with
a linearizable owning-link adapter; payloads and ArcSwap internals are excluded.

Three broken source variants fail their targeted tests. The initial tree
result-recovery failure is also retained. Sixty isolated allocation-count processes
pass with identical repeated counts and unchanged source/binary hashes. These are
protocol and allocation-design results, not production integration or qualification.
See the [frozen report](../target/gc-implementation/catalog-candidate-REPORT.md).

The pooled follow-up adds 13 native catalog fixtures, eight protocol models, ten
WASI/Miri/Tree Borrows fixtures, and ASan/TSan runs. The pool separately has six
native fixtures, four models, five Miri/Tree Borrows fixtures and three WASI fixtures.
Stock and isolated corrected-checker runs are retained, along with five deliberate
protocol/pool mutations. A shared-target reproduction attempt was rejected when a
negative control unexpectedly passed; the final runner isolates every variant's
build directory and validates both negative failures. See
[the runner](../benchmarks/gc/catalog_model_check.py) and
[follow-up evidence](../target/gc-implementation/catalog-pooled-REPORT.md).

Ninety allocation-count processes pass and reproduce the original control counts.
Graphs retain the additional backing cost. No GC timing, real-root/payload ownership,
byte-budget, pause or endurance qualification follows from these numeric checks.

## Descriptor-owned allocation charges (2026-11-14)

The [frozen evidence](../target/gc-implementation/allocation-credit-REPORT.md)
records a reproducible accounting failure and its runtime repair. Constructor
unwind after publication previously reduced a live descriptor's charge from 16
bytes to zero. Reservations now transfer into metadata before visibility, and
payload destruction refunds them exactly once after backing is released.

All 24 check commands pass: native GC 508 default / 532 combined-feature tests
(each retains one documented stock-Loom exclusion), classloader 775 with 14
existing ignores, VM four GC/reference integrations and 19 compiled-frame context
tests, actual WASI GC 253 / 264 and nine classloader ownership tests. Miri and Tree
Borrows each pass six existing constructor fixtures and five new charge fixtures.
ASan and TSan each pass 15 construction/ownership fixtures, five charge fixtures
and 26 publication fixtures. Native/WASI Clippy, bare-Wasm compilation, formatting
and diff checks pass. Existing Miri exposed-provenance warnings remain visible.

The 1,008-process timing screen uses seven randomized pairs, six mechanisms,
three policies, diagnostics off/on and native/WASI. Native allocation operation
time stays near baseline, while collection-inclusive wall time rises 0.6–2.4%
for allocation and 1.2–1.8% for managed-cell allocation. WASI allocation operation
time falls 1.4–1.9%, alongside shifts in controls, including slower overwrites and
nested writes. The report retains every result and interval; these changes do not
establish whole-JVM throughput, scheduled latency or global pauses.

Forty-eight separate counter processes pass. Four of eight condition groups have
identical counts; others vary by 2–34 extra temporary requests per 10,000 objects.
All extra bytes are released during collection, and retained requested bytes match
in all groups. Requested bytes are not RSS. The original failing executable,
frozen sources, raw data, binaries and PNG/SVG graphs are preserved. Experimental
policy defaults and all remaining catalog, graph-progress, JIT, pause, comparison,
capacity and endurance gates remain unchanged.

## Guarded refunds and borrowed reclamation (2026-11-14)

The [frozen report](../target/gc-implementation/refund-guard-REPORT.md) preserves two
additional unwind failures: a diagnostic subscriber panic after payload destruction,
and a diagnostic panic during finalization of private metadata. Both left 16 bytes
charged after payload release. The first borrowed-refund candidate passed 22 check
commands but failed the additional finalizer case; it was rejected before timing.
Scoped payload and refund guards repair both paths. Reclamation also validates and
borrows its collector instead of upgrading a weak reference for each refund.

The final 22 commands pass: native GC 512 / 536 with the same one documented
stock-Loom exclusion, classloader 775 with 14 existing ignores, VM four integrations
and 19 context tests, WASI GC 254 / 265 and nine ownership tests. Miri and Tree
Borrows each pass all nine affected fixtures; WASI executes the four non-unwinding
cases. ASan and TSan each pass nine charge, 15 construction/ownership and 26
publication fixtures. Native/WASI Clippy, bare-Wasm compilation, format and diff
checks pass. Existing exposed-provenance warnings remain in Miri logs. The preceding
six constructor Miri fixtures remain archived separately; they are not counted as
rerun in this matrix.

The final comparison has 1,008 timing processes with identical fixture source and
fresh before/after builds. Baseline mechanism/counter binaries reproduce the archived
descriptor-owned-charge revision byte for byte. Diagnostics-off native wall time
including collection decreases 0.2–1.4% for allocation and 0.8–1.0% for managed-cell
allocation. Operation medians are mixed, including 0.5–1.8% higher native allocation
times. WASI managed-cell operation time improves 1.6–2.0%, but leaf-write control
time regresses 9–16%. Other controls shift in both directions. The report retains
this unresolved regression; it establishes neither broad throughput improvement
nor recovery to the original pre-charge-transfer baseline.

All 48 separate counter processes pass. Five of eight groups reproduce every count;
three show small variable temporary-request differences. Those requests/bytes are
released during collection, and retained requested bytes match in every group.
Requested bytes are not RSS. All correctness, builds and counters finish before
timing. Raw data, rejected sources, exact binaries and PNG/SVG graphs are preserved.
Canonical publication, initial roots, callback/graph progress, JIT, observed pauses,
G1/ZGC qualification, capacity and endurance remain open. No defaults or gates change.

## WASI leaf-write code-placement diagnosis (2026-11-14)

A [336-process diagnostic follow-up](../target/gc-implementation/wasi-leaf-refund-diagnosis/REPORT.md)
reuses the exact refund-guard binaries. At one million operations per process, the
default-engine leaf-write regression persists at 8–11% with diagnostics off. A
separate Wasmtime experiment aligning all compiled functions to 64 bytes gives
changes of −1.6%, −0.8% and +0.5% across the three policies. The latter two paired
intervals cross one; the first ends narrowly below one at 0.999917. Root controls,
diagnostics-on results and every raw process remain recorded, including slower
diagnostic root operations in the alignment experiment.

Static inspection finds only 163 changed integer constants in the shared benchmark
function's 13,498 WAT lines, alongside changed native code placement. The alignment
intervention supports layout sensitivity without identifying a specific hardware
cause. Hardware performance counters are unavailable on this host. The
[PNG/SVG graph](../target/gc-implementation/wasi-leaf-refund-diagnosis/comparison.svg)
compares the two environments. This is attribution evidence, not a runtime fix:
the default-engine regression, existing qualification requirements and benchmark
defaults remain unchanged.

## Construction root batches (2026-11-14)

Nested construction scopes now move registration batches instead of cloning each
token. An empty enclosing batch takes the existing vector backing. Detached cleanup
skips ambient construction adoption while retaining normal deletion admission,
barriers and exact registration validation. A preserved counterexample rejects
delegating this new helper operation to ordinary root removal; it is not a previously
reachable production helper failure. The [report](../target/gc-implementation/root-batch-REPORT.md)
contains the frozen before/after inputs, raw data, graphs and scope limits.

All 23 scoped check commands pass: native GC 516 / 540 (one existing stock-Loom
exclusion each), classloader 775 with 14 existing ignores, four VM integrations and
19 context tests, WASI GC 257 / 268 and nine ownership tests. Both Miri modes pass
four new and six existing construction fixtures; existing exposed-provenance
warnings remain. ASan and TSan each pass four new and 15 existing ownership fixtures.
Clippy, bare-Wasm compilation, 78 Python harness tests, formatting and diff checks pass.

Seventy-two separate counter processes show about eight fewer global-allocator
requests per nested-constructor operation, with equal retained requested bytes in
all twelve groups. Guest allocator requests on WASI are not host OS allocations
or RSS. The 1,176-process timing screen improves nested-constructor operation time
by 8.9–16.3% native and 10.1–13.0% WASI. Collection-inclusive improvements are
5.0–10.0% native and 6.7–12.4% WASI. Controls remain mixed: WASI leaf writes,
nested writes and some overwrites regress. All policies and diagnostics-on results
remain recorded; these results do not qualify whole-JVM throughput or pauses.

A separate audit reproduced a destroyed unpublished parent's registration being
adopted by an enclosing constructor after a metadata-hook panic. It fails on both
the prior runtime and this batch revision. The isolated initial-root repair passes
its focused reproducer but is not integrated or broadly qualified here. This
known cancellation gap, canonical publication, callback/graph progress, JIT,
pause telemetry, G1/ZGC, capacity and endurance gates remain open.

## Ordered initial-root cleanup (2026-11-14)

The follow-up repairs the initial-parent cancellation gap above. Rooted constructors
finish private metadata hooks before installing their parent root, then retain a
metadata owner across borrowed registration. Reverse local drop order removes that
root before private payload destruction. Success transfers the exact registration
into the existing client guard. Child construction roots retain their independent
lifetime and enclosing-scope adoption behavior.

The first isolated repair removed the leaked root but failed an added destructor-order
witness: payload destruction still observed the initial registration. That exact
failing executable, original leak reproductions, final source, logs and graphs are
preserved in the [report](../target/gc-implementation/initial-root-REPORT.md).
The final fixtures cover three rooted constructors, gated/counted publication and
tree/slot/context roots; failures in trace-work, heap-owner and relocation hooks
retain children while releasing the failed parent. Explicit cancellation also runs
on WASI without relying on panic unwinding.

All 26 declared commands pass: native GC 520 / 544 (one documented stock-Loom
exclusion each), classloader 775 with 14 existing ignores, four VM integration and
19 context tests, WASI GC 258 / 269 and nine ownership tests. Miri and Tree Borrows
each pass five construction-scope, three initial-root and six existing constructor
fixtures. Existing exposed-provenance warnings remain in the logs. ASan and TSan
each pass the eight scope/initial-root and 15 existing ownership fixtures. Native/WASI
Clippy, bare-Wasm compilation, formatting and diff checks pass.

Seventy-two allocation-count processes pass, with unchanged retained and net
requested bytes in all twelve groups. There is no extra successful-construction
runtime allocation; small temporary count variation remains recorded. These are
requested-byte counters, not RSS, and WASI counts describe the guest allocator.

The 1,176-process paired screen reuses the exact root-batch baseline binaries and
builds the candidate from its frozen source. Allocation-operation medians fall
1.1–3.2% native (the candidate-policy interval includes no change) and 2.6–6.2% WASI.
Native collection-inclusive times are mixed. WASI allocation/managed-cell wall
times improve, but candidate-policy construction wall time regresses 4.31%
[3.73%, 4.68%] despite a 4.40% operation-time improvement. WASI baseline overwrites
regress 2.31%; native counted roots and candidate nested writes also regress.
The [operation](../target/gc-implementation/initial-root-operation.svg) and
[collection-inclusive](../target/gc-implementation/initial-root-wall.svg) figures
retain every control; exact intervals, diagnostics-on results and raw processes
remain in the report. No cause is inferred from earlier tests of different binaries.

This correctness repair is retained independently of the mixed cost results.
Neither metadata ownership nor the private root guard is a shared graph-publication
record. Admission, callback/graph progress, JIT coverage, pause telemetry, G1/ZGC,
capacity and hour/day qualification remain open. No defaults or release gates change.

## Generation-tagged owning-link experiment (2026-11-14)

The isolated [tagged pool](validation/catalog/catalog-tagged-pool-prototype/README.md)
combines generation and reference state in one atomic word. Delayed link tokens can
acquire only their exact live generation; initialization/destruction exclude readers
and reuse. Owning loads and clones make one acquisition attempt, returning contention
instead of retrying internally. Link replacement returns rejected ownership and
cannot run the replaced value's destructor while its expected owner remains held.

Seven native, three WASI, seven Miri and seven Tree Borrows fixtures pass, along with
seven fixtures under each of ASan and TSan, native Clippy and bare-Wasm compilation.
Five models import the exact state transitions; stock and corrected Loom pass.
Four mutations fail: missing generation validation, early reuse, missing final-owner
acquire and missing initialization acquire. The repository
`benchmarks/gc/tagged_pool_model_check.py` reproduces the corrected-checker pass and
all four faults with separate build directories. Its result checks reject compile
failures, missing models and unexpectedly passing faults; all 82 Python harness
tests pass.

The [report](../target/gc-implementation/tagged-pool-REPORT.md) and frozen evidence
retain the first runner's expected-diagnostic mismatch and initial Clippy failure.
The corrected checker uses the previously recorded unmerged Loom RMW patch without
changing production dependencies. These models cover state atomics, not pointer
provenance, aggregate pool ownership or the full link operation; native/Miri tests
exercise those schedules separately.

The collector and numeric catalog do not import this package. Atomic-attempt bounds
are not measured wall-time or arbitrary-destructor bounds. Representation cost,
participant/retained-state limits, reserved progress capacity, heap-byte accounting,
real metadata/initial graphs and phase integration remain open. No throughput,
latency, pause, peer-performance or endurance gate is established.

## Tagged catalog integration (2026-11-14)

The isolated [tagged catalog](validation/catalog/catalog-tagged-prototype/README.md)
replaces controller, membership and optional discovery links with fallible tagged
owners, and moves cells into fixed backing. An unresolved installed request returns
Pending and must be retained; Retry requires evidence that its expected immutable
controller was superseded. Failed discovery remains contention rather than an empty
slot. Certification still precedes releasing the Busy controller.

The first adapter lost this distinction and stalled a native contention fixture.
A deterministic injected-acquisition failure reproduces the lost completion result.
Both failures are preserved. Other fixtures cover failure before installation,
after applying the effect, during discovery and after certification before release.

The final 17-command matrix passes: pool 9 native, 4 WASI, 9 Miri, 9 Tree Borrows,
9 ASan and 9 TSan fixtures plus five stock models; catalog 18 native, 15 WASI,
15 Miri, 15 Tree Borrows, 18 ASan and 18 TSan fixtures; native Clippy and bare-Wasm
compilation for both. Byte-identical pool code reuses its completed checks after
the catalog fixture revision. The repository checker completes 12 protocol models
and five pool models using the recorded isolated Loom RMW repair. All nine faults
fail for executed invariant violations. All 86 Python harness tests pass.

The evidence also retains three validation limitations and their follow-ups:

- An original final-release acquire fence triggers a TSan read-before-reuse report.
  TSan does not model fences. The revised pool uses an acquire load while BUSY is
  exclusively held by the final owner, with the same implementation in ordinary,
  modeled and sanitizer builds. This is not reported as a proven memory-model bug.
- An expanded failure/race model times out at 900 seconds. The final suite separates
  failure handoff from the existing concurrent-certification model; it does not
  claim that the combined exploration finished.
- A stale-CAS fault initially passes because its test stop precedes the completion
  check. Moving that stop after observing an uncertified effect exercises the late
  CAS directly; the unmodified implementation passes and the broken variant fails.

The [cost fixture](validation/catalog/catalog-tagged-cost-prototype/README.md) runs
48 counting processes and 112 paired timing processes on frozen native/WASI binaries.
Counting and timing builds are separate, and validation does not overlap timing.
Every counting run uses 65,536 lifecycles, exceeding pool wraparound; timings use
500,000 lifecycles, seven paired repetitions per target/resident count, CPU 0 and
seed 42. Loop time includes preparation retries; process wall time includes setup,
teardown and runtime startup. These are numeric catalog lifecycles, not GC latency.

The loop issues zero system allocation requests versus one baseline cell request
(56 bytes native / 32 WASI). Median paired lifecycle time falls 51–53% native and
24–25% WASI. Setup backing at 16,384 residents grows from 2,644,200 to 3,696,792
requested bytes native and from 1,917,044 to 3,434,580 WASI. The tagged fixture
returns all requested backing after teardown; the baseline retains a 128-byte
native / 64-byte WASI ArcSwap thread-local cache. Requested bytes are not RSS.
Preparation retries range from 0 to 8,064 per 65,536 lifecycles at the tested sizes.

The [report](../target/gc-implementation/catalog-tagged-REPORT.md) contains paired
intervals and raw provenance; the [loop](../target/gc-implementation/catalog-tagged-v3-cost-operation.svg),
[wall](../target/gc-implementation/catalog-tagged-v3-cost-wall.svg) and
[backing](../target/gc-implementation/catalog-tagged-v3-cost-backing.svg) figures
show the cost tradeoff. These checks abstract real metadata/graph ownership and
the complete phase rules. Participant/retained-state limits, charged progress
reserves, reliable slot selection and arbitrary-destructor handling remain open.
The collector does not import these packages. Admission, callback progress, JIT
coverage, pause telemetry, G1/ZGC, capacity and hour/day qualification remain open.

## Resumable private catalog preparation (2026-11-14)

The tagged numeric prototype now retains successful private control allocations
across polls. A 32-slot fixture with six reusable free slots reproduces 128 failed
preparations in the old path: each failure discarded two successful allocations
before the third slot was found. The new `begin_*`/`Preparation::poll` API retains
that work and completes. Another fixture reaches three isolated free slots in 31
one-slot polls. These checks assume quiescent selection and reusable generations.

The original expected controller remains fixed; phase changes make the eventual
attempt retry rather than install an obsolete proposal. Failed final-record
allocation retains both prepared outcomes, cancellation returns every private
slot, and completed/foreign polls cannot consume another result. Existing
convenience calls make at most 24 probes and can abandon partial work; callers
requiring progress must retain `Preparation` between polls.

Eight catalog validation commands pass: 25 native, 22 WASI, 22 Miri, 22 Tree
Borrows, 25 ASan and 25 TSan fixtures, Clippy and bare-Wasm compilation. Pool code
is unchanged. The corrected checker completes 14 protocol models and five pool
models; all 11 deliberately broken variants fail at executed invariant checks.
The extra faults discard prepared outcomes or refresh the expected controller.
The model simulates selected control-allocation failure, not pool capacity or
byte accounting. All 86 Python harness tests pass.

The evidence retains the first reproduction fixture's lifetime-capture compile
error separately from the corrected old-runtime failure. It also preserves an
isolated checker-driver import error: a stale artifact-directory helper caused a
mutation mismatch before execution. The fixed driver selects repository helpers
explicitly and uses a fresh output/build tree.

All 48 new counting processes reproduce the previous revision's allocation,
backing and retry counts. A separate 224-process screen reuses the exact old
binaries and compares both catalog modes across native/WASI, four resident sizes
and seven paired repetitions. Each process performs 500,000 lifecycles. Counting,
validation, builds and source edits do not overlap timing. CPU 0, seed 42 and
fixed invocation names match the recorded method.

The tagged lifecycle costs 8.6–10.0% more native and 15.1–16.1% more WASI. The
unchanged pooled control has mostly intervals spanning zero; its 64-resident WASI
process-wall median rises 2.4% with a positive interval. Both controls and all
paired intervals remain in the [report](../target/gc-implementation/catalog-resume-REPORT.md),
[loop figure](../target/gc-implementation/catalog-resume-operation.svg) and
[wall figure](../target/gc-implementation/catalog-resume-wall.svg). The progress
repair is retained with its measured cost; no throughput improvement is claimed.

Participant/retained-state limits, charged progress reserves, graph/phase ownership
and migration of every admitted operation remain open. This package is still
isolated from the collector; no pause, G1/ZGC, sustainable-capacity or endurance
gate changes.

## Bounded catalog participants (2026-11-14)

The isolated tagged facade enforces separate mutator/helper limits and confines
owning snapshots, cells and requests to one non-cloneable participant state. Copied
keys reject foreign catalogs and reused slots. State destruction precedes quota
release. Derived capacities leave slack above the conservative retained-role counts.
Private cell creation preserves membership allocation across a cell-slot miss.
Forced fixtures continue with an active reserved helper while every ordinary
participant is stopped in private, installed or helping work.

The final eight-command matrix passes 39 native, 35 WASI, 36 Miri/Tree Borrows and
39 ASan/TSan tests, plus Clippy and bare-Wasm compilation. The corrected checker
completes 16 catalog models and five pool models; all 16 fault controls fail with
matching executed witnesses. All 86 Python harness tests pass. The model adapter
excludes pointer provenance, participant capacity and byte budgets. The final matrix
changes only the concurrent helper-stop fixture from the modeled source; runtime,
model and selected negative-control sources are identical.

Failed fixture schedules are preserved: the first candidate could return on owning
acquisition contention before its barrier; Miri detected the deadlock. A second
candidate sequenced acquisition but could still return before the effect stop on
contention. The final fixture sequences acquisition and stepping to each stop,
then tests progress while all ordinary participants remain stopped. Runtime helping
is unchanged by these fixture corrections.

A matched-capacity screen retains a **37–38% native / 98–102% WASI lifecycle-time
regression** for the bounded facade. There are 48 separate counter processes and
112 paired timing processes. Both modes issue zero loop allocations and refund all
requested backing at teardown; the facade adds one setup allocation of 160 bytes
native / 96 WASI. Counter bytes exclude inline participant state, allocator overhead,
RSS and Wasmtime host memory. The compared APIs perform different validation and
polling work; timings do not isolate a single mechanism.

See the [complete report and figures](../target/gc-implementation/catalog-quota-REPORT.md),
[final checks](../target/gc-implementation/catalog-quota-v3-check-results.json),
[model/fault evidence](../target/gc-implementation/catalog-quota-models/result.json),
and [fixture-only reuse check](../target/gc-implementation/catalog-quota-model-check-reuse.json).
No collector admission, default feature or qualification gate changes. Byte budgets,
finite exhaustion, fairness under arbitrary contention, real graph/reference-phase
integration and the remaining native/Wasm qualification remain open.

## Retained participant polls (2026-11-14)

The numeric facade polls pending state in place and uses the core preparation's
owning membership observation on success. Copied precondition diagnosis runs only
after a failed attempt; a still-valid command remains pending. The eight-command
matrix passes 42 native, 38 WASI, 39 Miri/Tree Borrows and 42 ASan/TSan tests, Clippy
and bare-Wasm. All 86 Python harness tests pass. Seven focused broken variants fail
with matching witnesses, including discarded Pending requests and falsely rejected
valid commands. Core transition/pool/model sources are byte-identical to the prior
revision: its 16 catalog/five pool models and 11 other fault controls are reused,
not claimed to model the changed facade. There are 18 distinct fault controls total.

48 new counter processes reproduce all prior stage allocations, backing and retries
exactly. 224 new timing processes compare the saved baseline with this candidate,
including the unchanged raw API as a control. Facade lifecycle time falls
**19–20% native / about 26% WASI**; all corresponding intervals are below zero.
The control retains small native differences and several 1–2% WASI improvements.
The facade still has higher absolute cost than the raw API. The combined changes
are not individually attributed, and these are process-level cost screens, not
operation tail latency or GC qualification.

[Report and graphs](../target/gc-implementation/catalog-poll-REPORT.md),
[checks](../target/gc-implementation/catalog-poll-check-results.json),
[fault controls](../target/gc-implementation/catalog-poll-faults/result.json),
[unchanged model-source evidence](../target/gc-implementation/catalog-poll-model-reuse.json).
Backing/participant byte charges, finite exhaustion, contention fairness and real
metadata/graph/reference-phase integration remain open, along with the existing
JIT, telemetry, G1/ZGC and endurance gates. Collector defaults remain unchanged.

## Charged catalog backing (2026-11-14)

The numeric facade can reserve requested backing plus inline participant storage
from an externally supplied Budget before allocation. `CatalogHandle` exposes no
Arc/Weak owner; final teardown uses `Arc::into_inner` to release the outer allocation
before dropping the catalog and refunding credits. Stopped participants retain the
whole charge. Exact-fit, failed and competing reservations and concurrent last
handle drops are covered. The convenience constructor creates a private unlimited
Budget; configured shared limits require `new_in`.

The 14-command matrix passes 47 native, 40 WASI, 44 Miri/Tree Borrows, 47 ASan/TSan
tests, Clippy, bare-Wasm and separate accounting audits under all six execution
modes. Native/WASI/sanitizer audits cover 12 configurations each; Miri/Tree cover six
smaller cases. Actual allocation bytes match the frozen toolchain Arc-layout formula,
and refund follows the final backing free. Ordinary-Arc early refund and undercharged
Arc-header variants fail with expected native/WASI witnesses. The reusable audit
runner also rejects incomplete/incorrect results. All 90 Python harness tests pass.
The corrected checker re-executes 16 catalog models, five pool models and 18 earlier
faults; with the two new accounting faults there are 20 distinct mutations. Models
do not cover the new budget or deallocation order; the allocation/memory-safety
matrix supplies that evidence. A recorded cargo-fmt-only comparison connects the
matrix source to the final normalized pool source.

48 new counter processes retain zero loop allocations and unchanged retries. The
private-budget convenience adds one setup allocation and 48 bytes native / 24 bytes
WASI, all returned at teardown. 224 paired timing processes retain mostly neutral
native facade results, with **+0.33%** loop time at 16,384 residents, and a **9–10%**
WASI facade decrease alongside an **approximately 5% WASI raw-control regression**.
No causal mechanism or GC improvement is inferred from those opposite changes.

[Report/graphs](../target/gc-implementation/catalog-charge-REPORT.md),
[14 checks](../target/gc-implementation/catalog-charge-check-results.json),
[model results](../target/gc-implementation/catalog-charge-models/result.json),
[accounting fault controls](../target/gc-implementation/catalog-charge-final-audit/result.json),
[toolchain/layout record](../target/gc-implementation/catalog-charge-toolchain-layout.json).
This is a declared logical footprint, excluding the external Budget, allocator
bookkeeping, RSS and Wasmtime host memory. It is not integrated heap accounting.
Finite exhaustion, contention fairness, transitive graphs, real reference phases,
admission migration, JIT, telemetry, peer comparisons and endurance remain open.


### Finite catalog generation exhaustion (2026-11-14)

A frozen red configuration reduces the numeric pool generation ceiling to three:
valid phase advances then remain Pending for 1,000 polls. The fix adds checked,
immutable generation limits (default the original 40-bit width), detailed selected
allocation failures and terminal facade rejection with private-state cleanup.
The selected slot may be permanently exhausted while other slots remain reusable;
a sticky observation is conservative failure diagnosis. Existing final-generation
owners and installed requests remain valid. There is no pool rotation or unbounded
service-life/fairness claim.

The 22-command matrix passes catalog tests: 52 native, 45 WASI, 49 Miri/Tree,
52 ASan/TSan; pool tests: 11 native, 6 WASI, 11 under each memory tool. Both packages
pass Clippy/bare-Wasm checks. Allocation-order audits also pass on all six runtime/
memory-tool targets. Six exact pool models and 16 catalog transition models pass
with the recorded corrected Loom checker. All 21 model/runtime fault variants fail
with matching witnesses, including ignored generation limits, lost exhaustion and
rejected installed work. The latter two also fail on WASI. Two accounting faults
fail on both targets, for 23 distinct variants. All 90 harness tests pass.

48 new counter processes reproduce previous loop counts/retries with zero loop
allocations. Facade backing increases by 56 bytes native / 44 WASI; raw backing
by 48/36, with the full increment returned during teardown. 224 paired timing
processes retain a +2.87% native facade regression at 16,384 records (95% CI
+2.53 to +3.77); its other native intervals cross zero. WASI facade results are
mixed, including -2.04% at 1,024 records. Raw API results also remain reported;
it executes the changed pool and is not an unchanged control. These are numeric
catalog cost screens, not GC latency or throughput qualification.

[Report/graphs](../target/gc-implementation/catalog-exhaust-REPORT.md),
[checks](../target/gc-implementation/catalog-exhaust-check-results.json),
[models/faults](../target/gc-implementation/catalog-exhaust-models/result.json),
[WASI witnesses](../target/gc-implementation/catalog-exhaust-wasi-faults-v2/result.json),
[accounting](../target/gc-implementation/catalog-exhaust-final-audit/result.json).
The first WASI negative run lacked captured panic text and was rejected; --nocapture
reruns preserve the witnesses. A count-edit error in a harness fixture is also
retained with its correction. Neither required changing the runtime candidate.

Real metadata, initial roots, transitive graphs, six-floor reference semantics,
admission migration, stalled callbacks, JIT, complete pause telemetry, G1/ZGC
comparisons and sustained-load qualification remain open. Defaults are unchanged.


### Generation budgets at setup (2026-11-14)

Pool setup encodes a budget L as initial generation MAX_GENERATION − L. Each claim
then checks the constant maximum, eliminating the per-pool ceiling field/read.
Exactly L successful allocations remain possible; final owners and installed work
retain their previous validity. Default tokens still begin at generation 1. Small
validation budgets occupy the final L private token values, with no wrapping or
reuse of exhausted storage.

The final 22-check matrix passes: catalog 52 native, 45 WASI, 49 Miri/Tree and
52 ASan/TSan; pool 12 native, 7 WASI and 12 under each memory tool. Both packages
pass Clippy/bare-Wasm checks, and allocation/order audits pass on six targets/tools.
Six exact pool models, five pool faults and two facade faults execute on final
source; the facade faults also fail on WASI. Two accounting faults fail on both
targets, for nine current distinct variants. The 16 unchanged abstract catalog
models are reused with a recorded source/type/constant comparison; historical
native faults are not counted as new execution evidence. All 90 harness tests pass.

The first matrix’s pool Clippy rejected an unused test-only constructor. Removing
that helper and calling the actual constructor in models changes no non-test
runtime code. Eight pool checks and eight focused model/fault commands rerun on
the final source. The 14 passing catalog/audit commands and cost builds remain
bound to their original frozen source; the precise cfg/source comparison records
why their executed runtime code is unchanged.

48 counters preserve loop counts/retries and zero loop allocations; setup and
teardown shrink by 24 bytes on both targets. 224 paired timing processes show
native facade improvements of 2.97–4.11% and raw improvements of 3.26–3.69%, with
all native 95% intervals below zero. WASI facade changes are +0.26/+1.02/+1.11/+1.37%
for 1/64/1,024/16,384 residents; the first three intervals exclude zero, while the
last crosses it. Raw WASI also has mixed changes. Earlier evidence is immutable;
no cross-screen ratio composition or cause attribution is made.

[Report/graphs](../target/gc-implementation/catalog-limit-REPORT.md),
[final checks](../target/gc-implementation/catalog-limit-final-check-results.json),
[pool/facade checks](../target/gc-implementation/catalog-limit-final-models/result.json),
[catalog model reuse](../target/gc-implementation/catalog-limit-model-reuse.json),
[source/cfg comparison](../target/gc-implementation/catalog-limit-cfg-equivalence.json).
Production graph/reference integration, admitted caller migration, JIT, pause
telemetry, G1/ZGC comparisons and sustained qualification remain open.


### Canonical retention and traversal work (2026-11-14)

The tagged catalog stores both domains' epoch, retention and traversal ticket in
immutable membership. The controller commits membership and pending counts together.
Six floors clamp discovery to Strong (0), Final (1/2), Phantom (3/4), then refuse it
at retirement (5). Stronger discovery replaces the ticket; exact acknowledgement
retains the mark and clears only the current obligation. Retirement requires white
membership in the named domain and no tickets in either domain. Both numeric
domains currently cover all records; young/old exclusions are not implemented.

The 14-command matrix passes 60 native, 53 WASI, 57 Miri, 57 Tree Borrows and 60
ASan/TSan fixtures, Clippy, bare-Wasm compilation and six ordinary allocation audits.
Seven supplemental checks validate the final accounting example with --trace on
all six execution modes. It completes 12 configurations per native/WASI/sanitizer
process (six per Miri mode), with no lifecycle system allocation/free or premature
credit refund. Only that example differs from the initial frozen matrix source.

20 catalog models and six pool models pass with the recorded corrected Loom.
Four new schedules cover closure/discovery, stale acknowledgement, old-epoch help
and independent domains. The old-epoch model has max_branches=10000 and unrestricted
preemptions. All 25 protocol/pool faults fail with matching witnesses; four new
tracing faults and two existing exhaustion faults also fail on WASI. Two accounting
faults fail on both targets, giving 27 distinct negative variants. All 90 Python
harness tests pass. The initial four-floor expectation failure is retained.

48 counter runs reproduce unchanged loop counts. Membership backing grows 24 bytes
per slot in these builds; participant storage remains 368 bytes native / 320 WASI.
224 randomized paired timing processes retain increased lifecycle cost: facade
+3.73–6.07% native and +3.29–4.49% WASI, with the one-record WASI interval crossing
zero. Raw API regressions are also retained. The unchanged numeric workload does
not exercise actual payload tracing, and these are not GC qualification results.

[Report and figures](../target/gc-implementation/catalog-trace-REPORT.md),
[matrix](../target/gc-implementation/catalog-trace-check-results.json),
[tracing audits](../target/gc-implementation/catalog-trace-activity-check-results.json),
[models and faults](../target/gc-implementation/catalog-trace-models/result.json).
Production admission, initial graph ownership, birth/generation exemptions,
conditional-reference decisions and stalled callbacks remain open.


### Sealed publication ownership (2026-11-14)

Production constructors now carry metadata, initial parent registration and sealed
construction protection in one owned record. Five new native/four WASI fixtures
retain private graphs through repeated collection, complete or cancel after the
origin scope/context ends, move work to a native helper, bind exact conditional
owners under unrelated TLS, and check parent/child destruction order. The destructor
fixture checks child liveness before dereferencing and records completion separately
so caught destructor panics cannot make it pass.

All 30 commands pass: GC totals are 525 native / 549 with optional features and
262 WASI / 273 with features, plus 775 classloader, four VM GC and 19 VM context
checks. Construction, initial-root and new handoff fixtures pass both Miri modes
and ASan/TSan. Native/WASI Clippy, bare-Wasm, formatting and diff checks pass.
Existing exposed-provenance warnings remain in the raw Miri logs.

Four executed faults each fail on native and WASI: releasing roots before sealing,
adopting helper TLS, skipping conditional-owner identity binding and dropping
private metadata before the parent registration. All 72 counter processes and
1,176 paired timing processes pass integrity checks. The baseline reuses exact
ordered-initial-root binaries with source equality verified; the candidate has
unchanged benchmark fixtures. Every policy, diagnostics mode, control and cost
regression remains in the [report/graphs](../target/gc-implementation/publication-owner-REPORT.md).
Allocation time rises 3.14–4.18% native / 5.75–7.73% WASI; nested construction uses
more allocator requests but fewer requested bytes. Net/retained requested bytes
match in all groups. This stage does not lower allocation overhead.

The record's heap borrow excludes final collector destruction, not publication
admission. Shared discoverability, replay-safe commit, collector-owned shutdown
ordering, real canonical root/reference state, arbitrary callback progress and
qualification remain open. No policy default changes.

### Rejected inline construction capture (2026-11-14)

This candidate stores four captured registrations inline and bypasses construction
TLS for types declaring no references. Both comparison revisions rebuild an expanded
fixture with a new explicit-leaf workload; the original allocation payload retains
its tracing capability. Baseline runtime/dependencies match the completed sealed
publication revision. Exactly three benchmark-related baseline changes are recorded.

All 31 commands pass: 526 native GC tests / 550 with features, 263 WASI / 274 with
features, 775 classloader, four VM GC, 19 VM context and nine WASI ownership tests.
Both Miri modes pass six capture, six construction, three initial-root and five
publication-transfer fixtures. ASan/TSan pass the focused roots, capture, initial-root
and transfer groups. Native/WASI Clippy, bare-Wasm compilation, 90 Python benchmark
tests, format and diff checks pass. Existing exposed-provenance warnings are retained.
Five fault variants each fail with executed witnesses on native/WASI, including
an inactive leaf scope incorrectly restoring TLS and losing its enclosing scope.

All 72 counters and 1,344 paired timing processes pass. **Reject the inline
representation:** ordinary allocation operation time increases 6.51–8.71% native
and 6.95–9.50% WASI; nested construction increases 4.37–5.32% / 2.71–4.20%.
All these intervals exclude zero. About four fewer allocator requests per nested
operation do not compensate for these measured regressions. Net requested bytes
and post-collection retained ranges match in all twelve count groups. The publication
type grows from 128 to 248 bytes native and 80 to 184 WASI; these diagnostic layouts
are not measured stack or RSS peaks. The measurements do not isolate causation.

The [complete report and graphs](../target/gc-implementation/capture-inline-REPORT.md)
retain diagnostics, root/write controls, all intervals, actual counter ranges,
failures, source/binary hashes and the predeclaration. A follow-up restores vector
storage and tests TLS bypass separately. This rejected cost experiment establishes
no pause, contention, resource-budget, JIT, comparison or endurance qualification.

### Optional reference-free construction scopes (2026-11-14)

This follow-up restores vector capture and represents constructor scope entry as
an Option. Explicitly reference-free types leave enclosing TLS untouched and create
empty detached protection without RefCell extraction; hooks still execute. There
is no new unsafe code or atomic protocol. Native/WASI diagnostic type sizes match
the pre-inline baseline: publication records remain 128/80 bytes.

All 25 commands pass, including broad native/WASI GC, classloader and selected VM
coverage, native/WASI Clippy, bare-Wasm, 90 Python tests and focused ASan/TSan groups.
Both Miri modes run all six capture fixtures, including varied batch sizes and an
absent leaf scope inside an enclosing constructor. Older broad Miri constructor,
initial-root and publication-owner runs remain historical evidence, not reruns on
this candidate. Their owner, metadata and credit code is unchanged. Existing
exposed-provenance warnings are retained. Five faults fail with executed witnesses
on native and WASI, including an absent scope incorrectly clearing TLS.

All 72 counter and 1,344 timing processes pass. Baseline uses exact rebuilt pre-inline
binaries with the expanded leaf fixture, and every paired process is rerun. Explicit-
leaf operation time falls 1.49–3.04% native (candidate-policy interval includes zero)
and 1.57–5.61% WASI (all three intervals below zero). Native nested construction has
lower collection-inclusive time for all three policies. Retained regressions include
WASI baseline allocation +3.61% and WASI leaf-write controls +5.79–7.41%; two leaf-write
intervals exclude zero. Root and other controls, wide intervals and diagnostics-on
results remain in the [report and graphs](../target/gc-implementation/leaf-scope-REPORT.md).

Allocator differences are small variations, not demonstrated request savings. Net
requested bytes and retained-after-collection ranges match across all twelve groups.
Type layouts do not measure stack/RSS peaks, and operation times are not scheduled
latency or pauses. Retain this candidate for integration without closing any full
qualification gate. Shared commit, stalled callbacks and bounded cleanup remain open.

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

[Report](../target/gc-implementation/shared-publication-REPORT.md) ·
[Verification](../target/gc-implementation/shared-publication-verification.json).

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

[Report](../target/gc-implementation/shared-view-REPORT.md) ·
[Verification](../target/gc-implementation/shared-view-verification.json).

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

[Report](../target/gc-implementation/commit-prefix-REPORT.md) ·
[Verification](../target/gc-implementation/commit-prefix-verification.json).

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

[Report](../target/gc-implementation/erased-publication-REPORT.md) ·
[Verification](../target/gc-implementation/erased-publication-verification.json).

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

[Report](../target/gc-implementation/announced-storage-REPORT.md) ·
[Verification](../target/gc-implementation/announced-storage-verification.json).

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

[Report](../target/gc-implementation/catalog-domain-REPORT.md) ·
[Backing graph](../target/gc-implementation/catalog-domain-cost-v2/backing-change.svg) ·
[Verification](../target/gc-implementation/catalog-domain-verification.json).

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

[Report](../target/gc-implementation/record-admission-REPORT.md) ·
[Verification](../target/gc-implementation/record-admission-verification.json).

## Bounded opaque-record requests (2026-11-14)

The isolated catalog facade owns non-cloneable records across transient preparation
failures. Guarded commands cannot select numeric insertion; private dispatch keeps raw
owners within participant quotas. Cancellation/terminal failure drops private records
before returning quota. A forced native destructor stall leaves its quota charged while
a reserved helper publishes an unrelated synthetic record.

A large-record fixture exposed unit-record layouts in the generic backing calculator.
The fix charges actual Cell/Control layouts and generic participant storage. All72
native/WASI record audit rows match requested backing and report zero lifecycle
allocations, nine setup requests and one refund after backing release; all24 numeric
rows remain unchanged. External payload/header/graph storage remains unaccounted here.

Thirteen matrix commands pass (native88, WASI79, both Miri modes84, sanitizers88,
Clippy/bare/audits/format), as do31 corrected catalog models. The latter exclude the
facade and quota protocol. Four faults fail in eight native/WASI processes; three
compiler checks reject command bypass, numeric registration and cloned participants.
Exact candidate integration and four main checks pass.

See [the report](../target/gc-implementation/bounded-record-REPORT.md) and
[allocator backing graph](../target/gc-implementation/bounded-record-backing.svg).
This remains an isolated catalog; actual prepared-payload identity/graph integration,
roots/references, callbacks and all GC/JIT/external/endurance gates remain open. No
new timing claim or production policy change is made.

## Prepared payload record identity (2026-11-14)

PreparedRecord owns a unique token and the exact private descriptor storage. Entry
admits the matching heap's graph handle before one-way binding; repeated helpers of
that record succeed and every other token is rejected. Discarding an unbound token
permits a new private candidate, while binding survives record/guard destruction.
Exhaustion precedes announcement, so failure preserves restoration eligibility.
RecordGuard borrows its record and heap and accesses no private result lock.

The 13-command v2 matrix passes: native 567/all-features 591 (one existing ignored
fixture), WASI 282/all-features 293, both Clippy targets, bare-Wasm, 26 shared fixtures
under each Miri mode and sanitizer, format and frozen-source verification. Six
corrected atomic models pass; three atomic mutations fail. Five source mutations fail
on native and WASI. Three compiler checks reject escaping record/heap borrows and
record cloning. Main integration exactly matches 2143 candidate sources before docs;
four main checks pass. V3 uses the existing portable-atomic counter type and repeats
full native/WASI and compile checks plus all six new fixtures under Miri/Tree and
sanitizers; this supplements V2's 26-fixture memory matrix. Retained v1 evidence
includes a lint-stopped matrix and an
invalid negative that failed at compilation; neither is counted as a passing check.

See [the report](../target/gc-implementation/prepared-record-REPORT.md). The binding
is not collector membership or a published cleanup latch. Final private closure still
cancels its payload. Physical header accounting, actual catalog integration, roots,
references and full GC/JIT/external/endurance qualification remain open. No new timing
or no-global-blocking claim is made; production defaults are unchanged.

## Canonical root ownership (2026-11-14)

The isolated catalog couples root status, target publication/shading, both pending
counts and scan cursors under one controller. Unique root Drop posts durable removal;
helpers preserve deletion work before exact slot release. A held revoked token still
blocks slot reuse. Bounded RootHandle retains its own quota and CatalogHandle through
target destruction, with credit refunded only after backing is freed. A stopped last
root destructor retains charge while a reserved participant retires an unrelated
synthetic record. Actual collector graph access and helper scheduling remain open.

All 15 matrix commands pass (native 109/WASI 99, both Miri modes105, sanitizers109,
Clippy/bare/six allocator audits/format), as do37 corrected catalog models. These models
exclude facade quotas and pool pointer protection. Seven faults fail in14 native/WASI
processes and five model variants; five compiler rejections preserve the bounded API.
All 32 candidate files match before documentation changes; four integrated checks pass.

The288 before/after accounting rows report zero lifecycle system requests and exact
refund-after-free. Root-enabled setup uses 12 requests; zero-root setup 9. Larger
controllers, root backing and conservative retained-target reserves increase logical
charge; the [graph](../target/gc-implementation/canonical-root-backing.svg) shows that
cost. This is an allocator audit, not throughput, latency or RSS evidence. See
[the report](../target/gc-implementation/canonical-root-REPORT.md) and
[quota/authority notes](validation/catalog/catalog-tagged-prototype/ROOTS.md).
Real roots/reference/traversal authority, admitted callers, callbacks and all
GC/JIT/peer/capacity/endurance qualification remain open. Defaults are unchanged.


## Canonical conditional references (2026-11-14)

The isolated catalog couples conditional decisions, owner publication, construction
custody and exactly-once notification transfer with phase work. Embedded reserved
roots belong to the exact reference object, so late announcements cannot occupy a
replacement's capacity. Non-cloneable reference/construction handles retain their
quota and catalog charge through final payload destruction. Helpers finish stopped
origins, and cancellation preserves an irreversible commit.

All 18 matrix commands and four main-tree checks pass: native/release 175,WASI 161,
full Miri/Tree 170 and both sanitizers 175, Clippy/bare, eight allocator executions
and format. The 48 corrected models are reused only for identical transition inputs;
they exclude facade quotas, real pointer protection, payload destruction and runtime
integration. 36 runtime faults, two model faults and nine external compiler failures
are caught, with a positive API control. All 528 accounting rows pass; reference
lifecycles make zero system requests but add fixed backing and inline storage.

These are isolated protocol and ownership results. The collector still uses its
existing runtime path. Graph/cleanup/root-result integration, admitted operations,
callbacks, JIT and all whole-GC qualification gates remain open. Defaults are unchanged.

[Protocol](validation/catalog/catalog-tagged-prototype/REFERENCES.md) ·
[Report and cost graph](../target/gc-implementation/canonical-reference-REPORT.md) ·
[Verification](../target/gc-implementation/canonical-reference-verification.json).
