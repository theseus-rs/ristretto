# Ristretto GC

[![Documentation](https://docs.rs/ristretto_gc/badge.svg)](https://docs.rs/ristretto_gc)
[![Code Coverage](https://codecov.io/gh/theseus-rs/ristretto/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/ristretto)
[![Latest version](https://img.shields.io/crates/v/ristretto_gc.svg)](https://crates.io/crates/ristretto_gc)
[![License](https://img.shields.io/crates/l/ristretto_gc)](https://github.com/theseus-rs/ristretto#license)
[![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)

A generational collector with stable descriptors and optional compaction of Java fields and arrays. Native hosts run concurrent workers with parallel tracing and relocation. `wasm32-unknown-unknown` and `wasm32-wasip2` run the same incremental state machine cooperatively, without requiring OS threads or shared wasm memory.

The interpreter, native JIT and browser engine share `ristretto_gc::GarbageCollector`. Source users of the retired catalog and pool can use `Gc` with the JVM collector. [Java preview and migration guide](https://github.com/theseus-rs/ristretto/blob/dcdc7dbed3eecd3b90525b83f815bf87947d9d06/GC_PREVIEW.md).

The implementation targets low ordinary collection latency through bounded root, marking, reference-processing, and reclamation work. `max_pause_time_us` is a soft scheduling target, not a real-time guarantee. Synchronous allocation recovery, explicit full collection, host allocation/deallocation, and custom Rust tracing/finalization can take longer.

See [the architecture and invariants](ARCHITECTURE.md) and [validation and measurements](VALIDATION.md).

Implemented checkpoints and remaining work are tracked in the [throughput and latency improvement plan](PLAN.md).
The [GC comparison suite](../benchmarks/gc/README.md) runs shared Java workloads
against HotSpot ZGC and G1 and produces throughput, latency, memory and backlog
graphs, with separate native and WASI results.

## Configuration

```rust
use ristretto_gc::{Configuration, GarbageCollector};

let gc = GarbageCollector::with_config(Configuration::builder()
    .max_heap_bytes(512 * 1024 * 1024)
    .allocation_threshold(8 * 1024 * 1024)
    .compact_heap(true) // opt-in until endurance and latency release gates pass
    .threads(4)
    .region_layout(1024 * 1024, 4096)
    .promotion_age(2)
    .full_collection_interval(16)
    .incremental_step_size(128)
    .max_pause_time_us(100)
    .build());
gc.start();
```

The default managed limit is 512 MiB on native targets and 256 MiB on wasm. Managed allocation sizes include instance fields and array backing storage. They exclude collector metadata, class metadata, and temporary host buffers. `usize::MAX` disables the configured limit. Live bytes and allocation debt are separate statistics.

`threads(0)` selects one native worker; positive values select that many workers. `parallel_threshold` remains a compatibility setting. Young and full cycles have independent scheduling and can overlap their marking phases. Reference decisions and generation reclamation are coordinated. Compaction is disabled by default; enabling it moves guarded Java payloads while descriptors stay stable.

Ristretto uses fallible allocation, attempts full collection on heap pressure, and throws a preallocated Java `OutOfMemoryError` when the managed request cannot be satisfied. Array capacity overflow is checked before backing storage allocation. Host code should use `Gc::try_new` or `Gc::try_new_async`; the legacy infallible constructors panic on managed budget exhaustion.

Optional `.diagnostics(true)` enables attributed lock-wait and allocation-recovery
measurements through `gc.diagnostics()`. It also records caller/worker work and,
on Unix, worker CPU time. Leave it disabled for ordinary performance measurements
and compare separate enabled runs to assess overhead. Bare wasm records counts
without requiring a clock. Inline recovery and awaited recovery (including time
suspended) have separate counters; async assists and registered waiters are also
reported. The legacy `total_pause_time_us` statistic measures
caller assistance, not stop-the-world pauses.

For experimental pressure prediction, set `predictive_collection(true)` on a
finite heap. Sampled allocation rate, reclamation latency, survival, and region
fragmentation determine when to request a full cycle. `scheduling_statistics()`
reports the estimates. Bare wasm uses an occupancy policy without a host clock.
Prediction is disabled by default; allocation-failure recovery remains available,
and early collection does not guarantee lower latency or higher throughput.

For experimental owning-root storage, select
`.root_storage(ristretto_gc::RootStorage::Slots)`. Reusable sharded pages replace
per-registration tree entries for owning handles; guards and suspended futures
retain their existing rooting guarantees. Numeric root IDs keep the compatibility
tree. `root_statistics()` reports page retention and reuse. The default remains
`RootStorage::Tree`; slot pages still use the publication and deletion barriers.
Root address mixing differs from metadata sharding to avoid page churn when
sweep order groups reused descriptors. Cached empty pages remain bounded to one
per root shard.

For experimental tracing batches, set `.mark_batch_size(8)`. Values are clamped to
1..=8; the default is one task per claim. Tasks share the step work/time budgets and
remain visible to termination checks until returned. Larger batches can reduce
coordinator acquisitions but may reserve work another worker could execute.
Diagnostics include batch and deferred-task counts. Object/reference statistics
merge at step boundaries and before cycle completion.

For experimental independent task claims, set `.independent_marking(true)`.
Tracing-only steps then share a short claim read lock instead of acquiring the
coordinator. Phase transitions exclude new claims and wait for every reservation,
including unfinished statistics publication. Pending generations retain scheduling
priority. `independent_mark_steps` reports accepted steps; the default is false.

For experimental publication without the heap-wide reader/writer gate, set
`.concurrent_publication(true)`. Publishers use counted admission; the coordinator
publishes phase changes with an atomic compare/exchange. Reference decisions use
individual slot locks and sweep uses object shards. A stalled admission can delay
phase closure while other publishers continue. Numeric handles cannot resurrect
logically dead graphs. Diagnostics include active admissions, phase transitions
and retries. The default remains false pending performance/endurance qualification;
see [PUBLICATION.md](PUBLICATION.md) for ordering, construction limits and tests.

`.overlapping_generations(true)` experimentally allows young cycles during full
sweeping and relocation, with either publication policy. New allocations after full
tracing closes remain young and are excluded from that full cycle's relocation.
Existing full-snapshot graphs protect callbacks while young garbage is reclaimed;
copy queues give young phases coordinator turns. Strong tracing and reference
decisions retain their generation interlocks. This is not old-only tracing, and
the option stays disabled pending qualification.

For experimental metadata sharding, set `.metadata_shards(16)`. Object-address,
identity, young-allocation and remembered-owner lookups then lock one of sixteen
indexes. Counts round up to a power of two between 1 and 16; the default remains
one. Full-heap and generation-index enumeration retain bounded shard/address cursors. Publication barriers, identity checks, and the selected
publication protocol remain in use. See [VALIDATION.md](VALIDATION.md) for measurements.

For experimental tracing work distribution, set `.work_stealing(true)`. Each native
worker uses a persistent queue; idle workers and cooperative callers can steal
bounded batches. Local overflow stays in a shared injector, and all queues and
reserved tasks participate in termination. The option works with either marking
policy and defaults to false; wasm retains one queue. Diagnostics expose stolen
tasks, queue contention, per-generation work and retained task capacity. `stop()`
releases empty queue backing allocations. Relocation has a separate optional policy.

## Embedding

`Gc::clone` and `GcRootGuard::clone_gc` return owning handles. Cloning a borrowed
heap field registers a root; cloning an owning handle or a root guard shares its
existing registration. A clone can outlive its source guard, parent object,
execution context and host collector handle. `rooted_clone` is an explicit alias
for the same ownership behavior. Raw, expired and logically retired edges cannot
be cloned into owning handles; use `ObjectId` or `GcWeak` for non-owning identities.
`Gc::as_root` returns an error for a different or unassociated collector.
`GarbageCollector::create_root_guard` rejects that mismatch by panicking before
registration; a guard must register in the allocation's own heap.

`Trace` requires an `unsafe impl`: every demoted strong field must be traced and
regain ownership in `root` before it can move. Its unsafe `unroot` and `attach`
hooks receive a `TraceOwner` that must be forwarded to embedded fields and cells.
Use managed `cell::RwLock` guards for mutation. They demote same-heap handles on
release, keep foreign-heap strong handles owning, and detach/rebind nested cells
when their values move. Explicit `Gc::unroot` is unsafe and requires independent
protection throughout every payload access.

Plain Rust locks, hash-set elements and hash-map keys keep independently owning
handles; their mutation cannot provide the managed barrier and mutable key access
cannot preserve hashing invariants automatically. Hash-map values do forward the
managed ownership hooks. Cycles through conservatively owning fields or between
separate heaps can remain retained. Conditional references keep their weak/final
semantics; cross-heap Java reference queue delivery is not established.
`remove_root_by_id` releases only
manual roots returned by `add_root`. Removing every registration for an allocation
with `remove_root` is unsafe and requires control over all affected handle lifetimes.

`HeapBuffer::try_with_fn_in` initializes payloads directly in the collector's
selected storage. `Object::new_in` and collector-aware array constructors use this
path. Initializer failures clean up only the initialized prefix; legacy host-backed
constructors remain available. These synchronous constructors can recover allocation space inline.

`HeapBuffer::try_with_fn_in_async` yields between 64 KiB initialization batches
(at least one element) and awaits storage recovery before running the initializer.
Cancellation drops the initialized prefix and releases its physical range.
`Object::new_in_async` and the interpreter/reflective array constructors use this
path. Async publication also chunks preparation of existing host buffers; legacy
custom preparation hooks can still perform synchronous work.

`Gc::try_new_once` returns the unpublished value on failure without collecting.
`Gc::try_new_async` retains its prepared value until it can atomically reserve
logical bytes. It can resume during sweeping, before the rest of the recovery
cycle completes. If two recovery cycles finish without observing or being offered
sufficient space, it returns an error. A race with another allocator for offered
headroom does not consume that cycle budget, including when the offer is lost
before the waiter is polled or notified. A size-indexed observation installed before
the cycle request records headroom at each refund. Physical backing retries track
reclamation separately; descriptor allocation retains bounded full-cycle retries.
Failed publication refunds its logical credit.
`collect_full_async` suspends on a completion notification while
native workers run; without workers each poll performs at most one configured GC
step and yields. Cancelling the waiter does not cancel the collection request.
Construction edges must keep owning roots across suspension. No managed guard may
span recovery, and no publication lock spans an await. Advisory headroom checks
recheck current capacity; they do not reserve space. Continual allocation competition
does not promise bounded waiting.
Recovery waiters share outstanding cycle requests while retaining separate wakers;
explicit full collection still waits for a cycle requested for that call.

The interpreter's ordinary allocation instructions, reflective arrays, and
`Object.clone()` use async recovery. Cloning still copies its payload under the
source guard before asynchronous publication. Native Java allocation helpers and
standard vector arguments also await recovery. Synchronous embedding and custom
conversion paths can still collect inline. The default publication gate, OS allocation, locks, or
custom callbacks can exceed the work budget's soft timing target.

Keep Rust locals and suspended futures alive with `GcRootGuard`, `into_rooted`, or `rooted_clone`. `Value::clone` creates or shares an owning root. Heap fields must become traced edges; Ristretto's managed `sync::RwLock` performs this conversion and publishes mutation barriers before releasing a write guard.

Custom `Trace` implementations must visit every outgoing managed edge, participate in mutation barriers, and implement bounded `trace_step`/`clear_step` when they own large reference containers. Unmanaged locks cannot supply those write barriers automatically. JIT reconstruction validates a non-reused object ID, allocation type and logical liveness under the selected publication protocol; unsafe raw-pointer APIs retain their documented lifetime contracts.

Cooperative embedders call `step()` regularly outside managed guards. The VM does this at allocation and interpreter yield boundaries. Bare wasm relies on the work budget and does not call an unavailable host clock. `collect_full()` is synchronous on every platform and must not be called while holding managed guards.

`stop()` stops the workers and collects unreachable allocations; escaped owning handles remain valid and keep the collector alive. VM shutdown cancels daemon tasks and releases class/static roots. The last collector owner destroys the remaining heap.

### Task contexts and buffered mutation records

`RootStorage::Contexts` selects task-owned root slots. Register a context with
`gc.register_context()`, use `context.with(|| ...)` for synchronous work, and
`context.scope(future).await` for asynchronous work. Java method execution installs
its configured context per poll. Suspension and cancellation do not require the
original OS thread; escaped owning handles retain their registrations.

`barrier_buffer_size(0..=64)` selects immediate barriers (the default) or bounded
context buffers during strong tracing. Cancellation transfers pending records to
the collector. Overflow has a bounded flush and immediate fallback, and terminal
phases use immediate barriers. Diagnostics expose context/buffer storage and
pending, flushed, and overflow counts. Root page statistics include detached
contexts' cumulative allocation/release counts. These options remain experimental.
See [PUBLICATION.md](PUBLICATION.md) for the default gate and experimental counted publication protocol.

Runtime UTF-8/UTF-16 string construction now awaits recovery and writes directly to
managed payloads, yielding during encoding scans and initialization. Runtime object,
module, thread, and bootstrap construction also use asynchronous publication in the
migrated paths; `Thread::object` reports Java OOM on exhaustion. Synchronous factory
and embedding APIs remain available. See [ARCHITECTURE.md](ARCHITECTURE.md) for the
remaining synchronous paths and [VALIDATION.md](VALIDATION.md) for test scope.

For experimental small-allocation caches, set `.local_allocation_buffers(true)`
and use execution-context scopes (the VM installs these automatically). Each
context can retain two 16 KiB ranges and 4 KiB of unused, charged logical credit.
Allocations over 1 KiB or aligned beyond 64 bytes bypass the caches. Separate
payload and descriptor arenas share the physical backing cap; global descriptor
fallback and metadata are outside that cap. Context-handle detach, collection,
and shutdown release unused caches. Diagnostics report cache retention and live
context activity. This option defaults to false.

For experimental native worker pacing, set `.adaptive_workers(true)`. The policy
uses queued work and allocation debt, caps activation by available CPUs, and reduces
workers with hysteresis. It does not change tracing/retirement rules. Diagnostics
report the target, delay, and throttled turns. It defaults to false and is ignored
on cooperative wasm. Soft deadlines always allow a step's first work unit so
phase setup or preemption cannot prevent collection progress indefinitely.

For experimental relocation distribution, set `.independent_relocation(true)`.
Workers then batch copy chunks within the step work/time budget without the phase
coordinator; enabling
`.work_stealing(true)` also gives them persistent local relocation queues. Enumeration
and phase completion remain coordinated. Queued tasks and in-flight claims prevent
premature completion, and storage leases protect callbacks. Diagnostics report
queue retention, independent steps, stealing, and contention. `stop()` releases
empty queues. The option defaults to false pending combined performance qualification.

Async publication of existing Java payloads now stages at most 64 KiB per batch,
yielding between batches. Direct managed constructors avoid staging. Cancellation
retains exactly-once ownership, and async publication rejects held managed guards.
Native helpers and standard Rust vector arguments used by VM calls await recovery;
custom embedding conversions/preparation keep their synchronous defaults unless
explicitly overridden. Arbitrary callbacks, host operations and guarded copies can still block;
publication policy is selected independently.

For experimental whole-region evacuation during full collections, combine
`.compact_heap(true)` and `.cost_aware_relocation(true)`. Selection favors at most four sparse,
cold regions per full cycle and defers recently pinned regions for one selection. An optional
`.relocation_reserve_bytes(1024 * 1024)` protects destination backing inside the
shared payload/descriptor cap, clamped to half the heap limit. Choose enough room
for a backing allocation; a reserve alone cannot ensure evacuation. Ordinary
allocation has correspondingly less backing capacity. Young collections retain
packing into earlier regions. Both options default to off/zero.

Region statistics include shared current/peak committed bytes, reserve usage,
selection skips, destination failures, copied/recopied bytes, and gross source
backing released. Gross release is not net memory savings: destination backing
remains charged. With this policy enabled, diagnostic pin deferral measures a failed copy attempt through
the next non-busy callback, including scheduling, and only reports resolved samples.
This policy retains payload locks, the selected publication protocol, and generation interlocks.

Cost-aware heaps with diagnostics also report `payload_guard_holds` and
`active_payload_guards`. Hold timing follows mapped guards through unlock, includes
managed tracing guards, and excludes lock-wait time and raw relocation copy locks.
It records unfinished guards separately from completed durations. Direct async
payload allocation can now resume on an advisory physical-space offer before the
requested cycle finishes; the allocator still reserves the range on retry. Generic
preparation hooks retain completion-based physical recovery.


Experimental `region_shards(1..=16)` distributes payload and descriptor allocation
registries while preserving one shared backing cap and one evacuation-source
quota. `independent_phases(true)` lets bounded root, reference and sweep work run
without the global phase coordinator; per-domain cursor locks and publication
admission preserve phase ordering. Clearing a retired allocation does not hold
unrelated ready storage behind that callback. These policies remain opt-in pending
performance/endurance qualification; custom callbacks and individual payload or
registry locks still have no universal time bound.

`old_collection_interval(N)` is an experimental old-only policy (zero by default).
It performs an initial young-to-old handoff, then traces/sweeps old allocations while
unrelated young collection continues. Conditional old referents and cross-generation
cycles await the precise full collector. Precise full intervals take priority;
`collect_full()` and async recovery retain their completion semantics. An old pass
can be requested with `collect_old()`. The policy enables independent phase units
and generation overlap. See [ARCHITECTURE.md](ARCHITECTURE.md#experimental-old-only-passes)
for retained snapshot memory, conflicting callback scans and the precise-full handoff.

`Configuration` adds `old_collection_interval`; `CollectionKind` and `Statistics`
include old-pass classification/counters. Update exhaustive matches and explicit
struct literals when adopting this API. `Trace::may_contain_gc_references()`
defaults to true, preserving custom tracing hooks. Primitive buffers use it to skip
reference-free traversal; destructor-bearing elements retain bounded clearing.
