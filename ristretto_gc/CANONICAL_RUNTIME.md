# Canonical runtime adapter

The `experimental-canonical-runtime` feature is opt-in. Ordinary `Gc` allocation
continues to use the ordinary collector. The typed adapter registers only fresh,
certified reference-free payloads in its catalog; ordinary descriptors and
finalizer registrations have no conversion into this backend.

`Trace` remains safe. A custom payload must return an unsafe type-invariant
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
