# Catalog with fallible tagged owning links

Current scan batching (2026-11-15): the original single-slot scan remains unchanged.
A separate `ScanBatch` operation examines at most eight slots and stops at the first
root/target effect. The internal leaf helper selects it. Cursor and effects retain
the same controller/certificate ordering and storage bounds; failed observation
never advances the cursor. Current validation includes 59 catalog and six pool
models plus 32 fault controls. The measured lifecycle improvement is 18–21%, with
unchanged canonical allocator activity and no full-GC qualification. See
[the batching protocol](../../../CANONICAL_ROOT_BATCH.md).

This catalog experiment uses the generation-tagged pool for controller,
membership and optional discovery links, including cell storage. The optional
[private leaf adapter](../../../CANONICAL_LEAVES.md) imports it for concrete
allocation ownership; public GC allocation policy does not select it. It retains
the exact-owner comparisons and completion-before-release protocol of the pooled
catalog.

Every owning load or clone makes one acquisition attempt. Discovery returns
`Result<Option<_>, Contended>`: a failed acquisition cannot be interpreted as an empty
slot. A controller snapshot also returns no phase while a Busy operation remains.
Effect ownership is prepared before the immutable commit decision. A failed strong
CAS against the exact expected membership proves this controller's effect already
happened: only canonical controllers mutate it, and old owning versions are never
reinstalled. Certification therefore needs no fallible owning reread. Controller
release can still fail after certification and remains helpable.

`commit` returns `Pending` when this request may still commit. The caller must retain
and poll that same attempt, or try to cancel it. It may prepare a replacement only
after `Retry` proves that another immutable controller superseded its expected
version, or after a terminal outcome. Cancellation can lose to the irreversible
commit decision. A committed result can precede release of Busy; any helper can
complete that final replacement using already prepared storage.

The first adapter confused Pending with Retry. A native contention test stalled,
and a deterministic fault fixture reproduced loss of the installed request's
completion result. The repaired adapter retains requests across transient failures.
Fixtures also cover failed discovery, failure before installation, failure after
an effect CAS and failure to acquire the final controller after certification.

The earlier trace-floor matrix passed 60 native tests, 53 WASI tests, 57 in each Miri mode,
60 ASan/TSan tests, Clippy and bare-Wasm compilation. The protocol models import the
same transition source through a linearizable owning-link adapter. That adapter
models selected acquisition and control-allocation failures, but excludes pointer provenance, pool capacity
and real metadata/graph ownership. The pool has separate exact-atomic models and
native/Miri/sanitizer coverage. The checker runner requires executed invariant
failures in deliberately broken variants; a compiler failure is not a passing
negative control.

That earlier corrected-checker run completed 20 protocol models and six pool models,
and rejects 25 protocol/ownership fault variants. The separate allocation audit
adds two accounting faults, each executed on native and WASI. The earlier integration's combined failure/race model timed
out at 900 seconds; the retained suite separates failure handoff from the existing
concurrent certification model. An initially ineffective stale-CAS fault test is
also retained: moving its stop after the completion check exposes the intended
race. See the [complete report](../../../../target/gc-implementation/catalog-tagged-REPORT.md).

Private control preparation now has a resumable `Preparation` returned by the
`begin_*` methods. Each `poll` attempts one selected slot and retains partial
successes. Its original expected controller cannot be refreshed after a phase
change. Dropping it cancels private work; after it produces an `Attempt`, normal
commit/cancel semantics apply. Polling a finished preparation or a foreign catalog
returns an error without consuming another request.

This fixes a quiescent fragmented-slot livelock: the old path repeatedly discarded
two successful allocations before finding the third, despite having enough free
slots. Effect-bearing requests now reserve one successor containing both
possible phases and one controller containing the effects. The successor's
decision remains unset while the effect controller is private or unfinished.
Helpers publish the irreversible completion decision after applying and
certifying the effects. A helper can then install that same successor even
if the first helper stops. Effect-free requests still reserve one slot.
The fragmented 32-slot fixture reaches both available slots in 31 polls. This is a quiescent selection check, not a contention or
participant-capacity proof. Convenience methods make at most 24 probes and can
abandon partial work; progress-sensitive callers must retain `Preparation`.

The [resumption report](../../../../target/gc-implementation/catalog-resume-REPORT.md)
preserves the red fixture and costs. Allocator/backing counts stay unchanged, but
the paired screen increases lifecycle time by 9–10% native and 15–16% WASI.

```sh
cargo test --locked --manifest-path ristretto_gc/validation/catalog/catalog-tagged-prototype/Cargo.toml --lib
python3 benchmarks/gc/tagged_catalog_model_check.py --output target/tagged-catalog-models
```

That checked-in runner now executes the current 55 catalog and six pool models and
28 fault controls, including all 25 historical controls. It preflights exact source
anchors, isolates copied workspace/build roots, and records each executed binary.
`--preflight-only` checks anchors without executing a model; its output is not a
proof result. The [current runner report](../../../../target/gc-implementation/canonical-checker-REPORT.md)
records the successful 30-case execution and 15 Python evidence-boundary tests.


The raw catalog API remains an unbounded-ownership experiment. The optional
`bounded::BoundedCatalog` facade enforces separate mutator/helper limits and hides
all owning snapshots, cells and requests behind non-cloneable participants. Only
copied keys and numeric observations escape; keys include a non-reused catalog ID.
All observations require an idle mutable participant. A participant retains one
creation/preparation/attempt, and its state is destroyed before its quota is returned.
Dropping a request asks for cancellation; a prior commit decision remains helpable.

For K registered participants and N registry slots, the facade rounds 8K+8 controls,
N+8K+8 memberships and N+2K+2 cells up to power-of-two capacities. Conservative ownership counts are at most 7K+3 controls, N+6K+3 memberships
and N+2K+1 cells. The private state contributes up to four controls; temporary help
can retain three more. Registry/request/helper cell owners and effect memberships
are included. Stalled destruction retains the registration until its owned state
is destroyed. These slot bounds assume all access goes through this facade.

`Command::Create` uses resumable private cell creation. A selected cell-slot miss
retains its exact private membership, rather than repeatedly discarding an earlier
allocation. Each creation poll probes at most one backing slot or attempts one
registry announcement. Losing announcements release their private backing.

Fixtures retain all ordinary participants in private requests, before announcement,
and at installed/helper boundaries while a separately registered helper completes
work, retires records, reuses registry slots and advances numeric phases. They also
check key isolation, quota release order, cancellation and failed acquisitions.
This is scoped numeric progress evidence: at least one helper must run. Shared-cursor
fairness and exhausted generations remain open. The subsequent configured
byte budget below is a logical footprint budget, not an RSS limit.
The [participant report](../../../../target/gc-implementation/catalog-quota-REPORT.md)
records failures and the matched-capacity cost screen.

There is no production byte-budget integration or complete collector progress proof. Value destruction and pool teardown remain
outside the atomic-attempt bound. The six-floor numeric phase machine does not
implement the collector's full reference semantics. Metadata, initial roots,
construction graphs and all admitted operations still require integration.


The facade now polls pending state in place and lets successful preparation use
its existing owning membership observation. Copied failure diagnosis runs only
after an attempt fails; valid commands remain pending. The paired
[poll-cost screen](../../../../target/gc-implementation/catalog-poll-REPORT.md)
reduces facade lifecycle time by 19–20% native and about 26% WASI, with every
allocation/backing count unchanged. This recovers part of the initial facade cost;
it remains slower than the raw API and is not integrated GC qualification.


Configured `BoundedCatalog::new_in(capacities, &budget)` reserves requested backing
plus inline participant storage before allocating. An opaque `CatalogHandle`
exposes no Arc/Weak owner; every final drop uses `Arc::into_inner` so the outer
allocation is freed before catalog fields drop and credits are returned. A stopped
participant retains the charge. The convenience `new` creates a private unlimited
Budget; use `new_in` to enforce a shared declared limit.

Arc layout is checked against the frozen toolchains and actual native/WASI allocator
requests. The externally supplied Budget object, allocator bookkeeping, RSS, host
runtime memory and application containers are outside that charge. The 14-command
matrix includes deallocation-order audits under native/WASI, Miri/Tree and both
sanitizers. The [charged-backing report](../../../../target/gc-implementation/catalog-charge-REPORT.md)
records both deliberately broken accounting variants, all 90 harness tests, setup
costs, and the mixed timing screen. Run the separate audit with:

```sh
python3 benchmarks/gc/catalog_accounting_check.py --output target/catalog-accounting --wasmtime /path/to/wasmtime
```

Finite generation exhaustion, contention fairness, real metadata/graph/reference
semantics and production accounting integration remain required.


## Generation exhaustion

`Capacities::with_generation_limit` checks an immutable per-slot allocation budget; the
default remains the full 40-bit generation width. Preparation and creation return
`GenerationExhausted`, and the bounded participant drops private state and rejects
the request. Ordinary contention remains Pending. Existing owners stay valid;
installed requests finish without another allocation. A selected-slot exhaustion
observation is not a whole-pool exhaustion certificate. Failure-only diagnosis can
conservatively reject while other slots remain reusable.

The [exhaustion report](../../../../target/gc-implementation/catalog-exhaust-REPORT.md)
retains the red driver, 22-command matrix, six pool/16 catalog models, all 23 distinct
negative variants, the byte-cost increment and mixed paired timings. Indefinite
service, fairness, real reference/graph integration and GC qualification remain open.


The setup-budget follow-up encodes that limit in the initial private token value,
removing the pool field and allocation-path read. Default generation numbering
and rejection/completion behavior remain unchanged. Actual backing is 24 bytes
smaller per catalog on both targets. The [report](../../../../target/gc-implementation/catalog-limit-REPORT.md)
records the final matrix, scoped model reuse, native speedups and WASI regressions.


Canonical tracing uses `Membership::Tracing` with two `TraceMark` values. Each holds
epoch, optional Strong/Final/Phantom retention and an exact traversal ticket. The
controller couples those values with pending counts. `Shade` clamps to the receive
floor, replaces weaker work without double counting and rejects retirement-floor
discovery. `TraceAcknowledge` accepts only the current epoch/ticket and preserves
retention. `TraceRetire` requires white membership in its domain and no outstanding
work in either domain. Generic numeric retirement cannot delete tracing membership.

`TraceInsert` is white numeric publication; it is not real allocation publication.
`AlreadyMarked` reports a copied observation, not a new root or a payload-access
certificate. Both domains currently cover all records. Birth exemptions, real
young/old membership, conditional-slot decisions, graph traversal and callbacks
remain outside the experiment. See the [current report](../../../../target/gc-implementation/catalog-trace-REPORT.md)
for retained storage/timing regressions and the expanded safety/model evidence.
The accounting example accepts `--trace` to audit the numeric tracing lifecycle.

Effect ownership is now prepared before the immutable commit decision. An injected
acquisition failure leaves the request cancellable. The effect's exact-version CAS
and its certificate need no additional fallible acquisition; a failed strong CAS
proves the effect already happened under the sole canonical writer protocol.
Controller release may still need another helper after the certificate exists.
The 22-model suite includes a forced stale effect CAS after later membership work.
This is a prerequisite for a graph guard spanning commit, not an implemented graph
admission protocol. See the [report](../../../../target/gc-implementation/commit-prefix-REPORT.md).


The current domain extension adds `DomainMode` and `TraceState`. `Publish` computes
both domain birth/retention/ticket effects together; `Promote` couples generation
with old handoff work and retains outstanding young work. `Start` increments an idle
domain's epoch, `Finish` closes an empty retiring domain, and `Advance` no longer
silently restarts. The later `FinishEpoch` command and `begin_finish_epoch` factory
bind a completed policy sweep to its exact epoch across preparation/retries. The
internal [leaf maintenance policy](../../../CANONICAL_MAINTENANCE.md) uses this
bound operation; trusted raw callers retain responsibility for unbound Finish. The compatibility constructor initially activates both numeric
domains; this is not production policy selection. `reference_live` answers only the
reachability/exclusion part of reference eligibility. Root enumeration, conditional
reference decisions, real payloads and graph protection remain unimplemented.

The [domain report](../../../../target/gc-implementation/catalog-domain-REPORT.md)
records the current 74 native/67 WASI/71 Miri fixtures, 27 corrected models, nine
mutations and backing costs. The new state adds 512 KiB at 16,384 residents while
retaining zero lifecycle system requests. Earlier matrix counts above describe
their corresponding historical evidence.


`Catalog<R>` now retains opaque records with a borrowed context and GAT guard.
`snapshot_in`/`commit_in` acquire a publication guard before each helper's decision
attempt and exact effect, and publish a cleanup latch before certification. Closed
private admission cancels undecided requests; certified controller release needs no
record access. Existing numeric methods are wrappers on `Catalog<()>`; its legacy
Insert/TraceInsert paths are unavailable for other record types. Preflight creation
failure returns the record through `RecordFailure<R>`. A later announcement collision
still cancels its private creation; concrete token retry policy remains required.

The [record report](../../../../target/gc-implementation/record-admission-REPORT.md)
records native81/WASI73, scoped Miri/sanitizer coverage, 31 corrected models, six faults
and three compiler checks. Its guard fixture is synthetic; no real collector uses
this interface. Bounded participants currently remain unit-record-only. Concrete
heap/identity/graph validation and payload uniqueness are runtime obligations.

## Guarded bounded records

BoundedCatalog<R>::with_records_in and register_record_{mutator,helper} provide
non-cloneable Participant<R, GuardedCommand> owners. submit_record transfers R into
resumable creation; Busy returns it unchanged. Pending retains it. Cancellation and
terminal rejection, including a discovery-slot collision, destroy private ownership.
Actual underlying payload uniqueness remains the concrete record's responsibility.
Numeric commands and registration remain unit-record-only. poll_in/phase_in require
the matching borrowed record context; copied observations expose no owning record.

Footprint::for_records charges actual generic pool layouts and inline participant
storage. It excludes external allocations owned by R. The record_accounting example
checks small, large and over-aligned records on native and WASI, including zero
lifecycle allocations and refund after actual free. Unit-record tables are unchanged.
The current thirteen-command matrix passes native88/WASI79, both Miri modes84,
ASan/TSan88, Clippy/bare/audits/format;31 catalog models pass, excluding facade quotas.
Four source faults fail on both targets; three external compiler checks reject bypass
and cloned quotas. See the [report](../../../../target/gc-implementation/bounded-record-REPORT.md).
Actual GC graph/binding/root/reference integration and qualification remain open.

## Canonical root tokens

Optional root capacity adds unique RootHandle ownership, durable Drop requests,
paired publication/removal barriers and per-domain scan cursors. The bounded facade
reserves root quota before private creation; Ready results require take_root or
cancellation. RootCommand is separate from numeric compatibility commands. Retired
slots cannot be reused until token Drop requests removal. Catalog charge survives
all escaped bounded handles and their target destruction.

See [ROOTS.md](ROOTS.md) for protocol, retained-owner bounds and runtime limits, and
[the report](../../../../target/gc-implementation/canonical-root-REPORT.md) for the
fifteen-command matrix,37 models, faults, compiler checks and allocator graph. Current
results are109 native/99 WASI/105 per Miri mode/109 per sanitizer. Root-enabled audits use
12 setup requests and0 lifecycle requests, with exact refund-after-free. Root backing
and larger control layouts increase memory; this stage has no timing claim. Earlier
counts above describe their historical snapshots. Actual GC integration and full
qualification remain open.


## Canonical conditional references

The experiment now has exact conditional-reference versions, owner-bound scans,
construction lifetime roots and notification custody. The bounded facade reserves
separate reference/construction quotas and retains catalog charge through token and
payload destruction. Helpers finish stopped scans and installed operations; exact
root/reference CAS prevents late replay after consumption and reuse. See
[REFERENCES.md](REFERENCES.md) and the [report](../../../../target/gc-implementation/canonical-reference-REPORT.md).

The current 18-command matrix passes native 175/WASI 161, full Miri/Tree 170 and both
sanitizers 175. The 48 transition models have explicit pointer/facade exclusions;
36 runtime fault processes, two model faults and nine compiler rejections pass their
expected checks. All 528 allocator rows match backing/refund accounting; reference
lifecycles make zero system requests with increased setup/backing costs. These are
isolated ownership/protocol results. Actual GC integration, performance, pause and
endurance gates remain open.


## Supplemental runtime storage reservation

`with_records_and_storage_in` accepts checked caller-declared inline storage in
addition to the base Footprint. Existing constructors pass zero. The existing
Charge retains the sum until final backing free; no catalog field or per-poll
reservation is added. `reserved_storage_bytes` reports the supplement and
`charged_bytes` reports the sum. The caller must enforce wrapper count/lifetime
bounds; [the internal leaf runtime](../../../CANONICAL_RUNTIME.md) does so through
its private catalog and participant/root quotas. `RootHandle::belongs_to` compares
exact catalog identity without acquiring an owner or granting payload access.

The subsequent [automatic rooted transaction](../../../CANONICAL_RETRY.md) retains
records through cell/root discovery collisions and full/contended root quota.
`poll_rotating` keeps private backing and seals its location after announcement;
reserved reference roots reject rotation. Explicit polls remain terminal on collision.
The current scoped run passes 188 native / 173 WASI tests and 55 corrected models;
96 forced-collision allocator rows add no lifecycle backing requests. Participant
hints add 16 native / 8 WASI bytes per quota. These are correctness/storage checks,
not a public GC integration, scheduling guarantee or performance qualification.
