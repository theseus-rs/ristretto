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
claim. Graph operations are a separate dependent slice.


Helpers advance one protocol request or scan at most eight consecutive empty
positions per poll, stopping at an occupied or contended position. A failed
object discovery invalidates the local complete-sweep certificate. Epoch finish
requests carry the exact epoch; they cannot finish a later unswept epoch.
Dropping a root permits incremental retirement and slot reuse. A retained typed
view keeps its payload alive while unrelated payloads can be reclaimed.
