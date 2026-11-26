# Canonical runtime adapter

This historical experiment can be reproduced from the [archived source](https://github.com/theseus-rs/ristretto/blob/0497123e05056546bc526ecc49c081f934f38d64/ristretto_gc). The current Java preview uses the [unified JVM heap](../GC_PREVIEW.md).

The `experimental-canonical-runtime` feature is opt-in. Ordinary `Gc` allocation
continues to use the ordinary collector. The typed adapter registers only fresh,
certified reference-free payloads in its catalog; ordinary descriptors and
finalizer registrations have no conversion into this backend.

The ordinary collector requires an audited `unsafe impl Trace`. That alone
does not grant canonical admission: a custom payload must also return a type-invariant
`CanonicalPayload<Self>` certificate, report no managed references, and report
no heap owner. The certificate covers all values and interior mutations of the
type. It excludes required tracing, attachment, relocation and Java finalization
callbacks. Rust destructors remain supported and their execution is not bounded
by the protocol's poll-work limit. Owned external backing is charged with the
inline payload before preparation succeeds.

The adapter checks exact heap, runtime and payload type before borrowing. A view
borrows both its logical root and the runtime; it does not expose a transferable
storage lease. Root copies share one original permit. Independent roots consume
independent permits. The last alias releases its outer allocation before the
catalog token can refund that backing reservation.

Catalog and wrapper storage are reserved separately from payload bytes. Caller
alias variables, host stacks, allocator overhead and process RSS are not included.
A pending allocation exclusively borrows its mutator. Cancellation posts intent
and releases its owners; it does not loop to finish collection in Drop.

Run the typed-root, quota, cancellation-prefix and cleanup checks with:

```sh
cargo test --locked -p ristretto_gc --features experimental-canonical-runtime --lib canonical_adapter
```

This slice makes no global pause, throughput, callback-latency or default-backend
claim. The public API is available as `ristretto_gc::canonical`.


Helpers advance one protocol request or scan at most eight consecutive empty
positions per poll, stopping at an occupied or contended position. A failed
object discovery invalidates the local complete-sweep certificate. Epoch finish
requests carry the exact epoch; they cannot finish a later unswept epoch.
Dropping a root permits incremental retirement and slot reuse. A retained typed
view keeps its payload alive while unrelated payloads can be reclaimed.


Graph construction admits a fixed maximum degree of 1–8, with an immutable
per-payload usable count from zero through that maximum. Each successful result
owns its complete source-edge inventory; cancellation at an intermediate stage
releases the root and orphan edge slots through the same helper policy. Index and
capacity arithmetic is checked before preparation. Edge replacement retains source and target roots through completion and
couples deletion/insertion barriers with the membership change. Foreign roots
and out-of-shape positions reject before submission; stale edge identities
cannot mutate reused slots.


A child read allocates an independent logical root, then revalidates and activates
it against the exact edge before checking the payload type and returning it.
Replacing the edge between root preparation and activation forces a fresh
observation. Null results reserve no root. A full root quota remains pending;
a later null replacement can still complete that read. Dropped reads cancel the
request and release any private child root without driving an implicit loop.


The public surface preserves the preview's `Configuration`, `Budget`, `Runtime`,
`Mutator`, `Helper`, `Root`, `View`, allocation/read/write requests and error types.
`allocate_with_edges` retains each payload's usable edge count. `same_object`
compares exact catalog-scoped identities, including independently acquired roots.
`primary_cycles` returns conservative numeric bounds from one idle-participant
observation; these are completed primary full epochs, not reclaimed-object or
callback counts. The observation does not help and may fail under contention.

Two nodes can retain each other solely through catalog edges. Once their last
external roots disappear, helper polls reclaim both payloads and permit reuse at
finite capacity. Payload bytes are observable through `managed_bytes()`; ordinary
collector scheduling statistics keep their upstream meaning. Catalog budget
credit remains charged while any runtime, participant or escaped root retains it.

```sh
cargo test --locked -p ristretto_gc --features experimental-canonical-runtime --test canonical_api --test canonical_observation
```

The same tests execute on `wasm32-wasip2` with the documented Wasmtime runner.
No Java VM backend is changed by these Rust API slices. Conditional references,
arbitrary traced payload graphs and conversion of ordinary finalizer descriptors
remain unsupported. General Rust destructors and host allocation are outside
bounded protocol-work claims.

## Private leaf diagnostics

The `experimental-canonical-leaves` feature also retains the private
`gc::publication_owner::canonical_leaf` diagnostic backend. Its implementation
lives in `canonical_diagnostics/`; it is not exported as `canonical` and cannot
grant the public adapter payload authority. Both backends use the bundled catalog
and pool. The following retained notes describe the historical diagnostic
layout and evidence, not new measurements or public admission guarantees.

### Heap-bound canonical leaf runtime

`LeafRuntime` composes the internal allocation and maintenance APIs under one
collector/catalog identity. It borrows its collector and privately owns the catalog
handle. It creates non-cloneable registered mutators and helpers using that exact
collector. A mutator's `begin` returns the existing exclusive-borrow allocation
request. Runtime reads require the root's exact catalog identity before checking
heap admission, publication and type. Two runtimes sharing a collector reject each
other's roots. No additional owning pooled observation or strong collector owner
is introduced. Public `Gc` constructors and defaults remain unchanged.

Participants may outlive the initial runtime owner while their collector borrow
remains valid. They can finish allocation or maintenance without that initial owner.
Typed reads through this API still require its runtime owner. A root may retain
reference-free storage after both runtime and collector shutdown; it cannot be read
through an unrelated replacement runtime. Existing leases retain catalog backing
and its charge through these lifetimes and through actual payload destruction.

The [shared-root follow-up](CANONICAL_SHARED_ROOTS.md) now permits typed aliases.
It adds private backing before requests poll and charges live and pending owners;
the original layout figures below describe the earlier unique-wrapper stage.
The corrected final-alias teardown frees that backing through `Arc::into_inner`
before dropping the unique root and refunding catalog credit; a populated final-owner
audit covers both leaf and graph roots. Ordinary Arc teardown in the preceding
shared-root version did not preserve that order.

## Storage reservation

The base catalog Footprint reserves its requested backing, participant storage and
root handles. `with_records_and_storage_in` adds a caller-declared inline reservation
at setup using checked addition. Overflow or insufficient budget fails before
catalog allocation. Existing constructors supply zero. `footprint()` remains the
base catalog census; `charged_bytes()` includes the supplement, and
`reserved_storage_bytes()` reports their difference. The caller of this low-level
factory must enforce the count/lifetime bounds of its declared supplemental storage.

The runtime enforces those bounds with its private catalog and existing participant
and root quotas. Its supplement covers one initial runtime object, each helper's
storage beyond its already charged Worker, each mutator's storage beyond Worker,
one simultaneous allocation request per mutator, and any typed-root storage beyond
the already reserved root handle. It uses actual target layouts and checked
arithmetic. The original typed-root wrapper added zero bytes to its handle. Allocation
requests and typed roots store PhantomData rather than inline T values.

The existing Charge retains the supplement until final catalog backing is freed.
No extra budget owner, atomic reservation, allocation or refund operation is added
to each allocation or maintenance poll. Tests keep the full charge through retained
participants/roots and blocked payload destruction, including after the original
runtime owner leaves. A portable payload destructor checks the charge before it
finishes. Quota failures create no additional wrapper owner.

For the recorded production configuration (Rust 1.99.1, no cfg(test)), the additional
reservation is `16 + 32*U + 208*H` bytes on native 64-bit and `8 + 20*U + 136*H` on
WASI 32-bit, where U and H are declared mutator/helper capacities. With seven
mutators and two helpers, this is 656 / 420 bytes. These values describe the tested
layouts, not a stable Rust ABI. The implementation calculates sizes on its target.
A temporary print-only harness exposes private layouts for this audit without
adding a public diagnostic API to the candidate.

The 672 existing catalog accounting rows are unchanged. Another 72 rows exercise
supplements of 0, 128 and 4096 bytes, verifying unchanged requested backing, no new
system allocation during each numeric trace cycle, and refund after final free.
The 48 production-layout rows separately describe actual owning wrapper sizes.
These measurements do not include concrete payload preparation or execution time.

## Remaining integration

This census covers bounded owning protocol state. Borrowed views/pointer copies
add no independent pooled ownership and remain caller storage. The supplied
collector and budget owners, caller boxes/stacks, payloads, allocator metadata and
RSS need their own accounting. A logical reservation is not a physical-memory cap.

The subsequent [automatic allocation API](CANONICAL_RETRY.md) retains the unique
payload through slot collisions and full/contended root quota. Explicit `begin`
keeps its collision/error semantics; `begin_any` advances one candidate per poll.
The current participant hints add 16 native / 8 WASI bytes per quota to the base
Footprint; the wrapper supplement reported above is unchanged. The 672 unchanged
rows above describe the earlier runtime-owner stage, before those hints were added.
The runtime does not add scheduling, wakeups or collection loops in Drop. Arbitrary
payload Drop, preparation hooks and host allocation can still take unbounded time.
Graph/conditional-reference integration, callback capabilities, admitted callers,
public backend/package selection, scheduling, JIT and
native/Wasm performance and sustained qualification remain open.
