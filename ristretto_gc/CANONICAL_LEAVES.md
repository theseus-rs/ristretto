# Private leaf ownership in the canonical catalog

This historical experiment can be reproduced from the [archived source](https://github.com/theseus-rs/ristretto/blob/0497123e05056546bc526ecc49c081f934f38d64/ristretto_gc). The current Java preview uses the [unified JVM heap](../GC_PREVIEW.md).

The optional `experimental-canonical-leaves` feature connects private GC allocations
to the catalog experiment and provides internal typed reads through canonical roots.
Public `Gc` constructors and collection policy do not select it. It exposes no
public allocation/result API and does not qualify whole-GC pauses, performance or
sustained-load behavior.

The subsequent [allocation transaction](CANONICAL_ALLOCATION.md) composes real
private preparation, root publication and a unique internal typed result. It adds
durable cancellation between record creation and root creation while retaining the
ownership and read requirements below. The internal
[maintenance policy](CANONICAL_MAINTENANCE.md) now cleans abandoned preparations,
requested roots and unreachable leaf storage through complete epoch-bound sweeps.
The [internal runtime owner](CANONICAL_RUNTIME.md) then binds allocation, helpers and
reads to one heap/catalog, reserving owning-wrapper storage at setup. The
[automatic request API](CANONICAL_RETRY.md) subsequently adds bounded slot rotation
and preserves unique payloads while waiting for root quota or discovery space.

A private `Publication` can transfer its exact metadata into one non-cloneable
`LeafRecord` only when its immutable tracing pointer has no step function, its
metadata has neither a finalizer nor a `HeapOwner`, and it owns neither an initial
parent registration nor captured construction protection. The tracing capability
comes from the existing unsafe `Trace` contract. An opaque tracing hook is rejected
even when the current value happens to have no edges. Rejection returns the original
publication with every owner intact.

Successful transfer disables the discarded private `Gc` edge's weak heap link
before dropping that handle. It never calls legacy registration or inserts the
payload into legacy object/identity indexes. Metadata owns the initialized
allocation and existing logical charge. A weak heap link avoids a collector cycle;
a surviving record remains an owner of its own reference-free storage.

`Record::enter` checks the exact heap and returns a guard borrowing both the record
and that heap. The catalog retains this guard through its canonical effects and
publication certificate. Borrowing this sealed record excludes its destructor:
there is no separate index that can retire it, and no independent metadata owner
or storage lease escapes. Typed references remain within their root/heap view
lifetimes. Thus this particular admission needs
no retrying storage-lease acquisition. General metadata views, graphs and public
`Gc` handles cannot use this argument.

## Typed reads through canonical roots

The bounded `RootHandle::borrow_live_record` borrows its exact target record after
observing Live. It acquires no owning pool share or membership snapshot. That
observation retains storage through the root token but is not a publication or
runtime access certificate.

The leaf adapter's `borrow_root<T>` also enters the matching heap/record, reads the
publication latch with acquire ordering, and validates the descriptor's immutable
TypeId. It then derives `&T` from the original pointer, preserving provenance.
`LeafView` retains the root, record admission and heap borrows; its `Deref` exposes
only a reference tied to the view. It provides no mutable access or relocation.

Explicit revocation can follow a successful Live observation. A read admitted
before revocation may complete afterward, retaining the same storage through its
borrowed token. Reads observing Retired fail. In the initial-publication race, a
later removal must first help the earlier publication through certification, so a
Live observation followed by a successful certificate read can be admitted before
that removal. An existing view does not create a new root or resurrect membership.
The finite fixtures establish these schedules, not arbitrary fairness.

No extra independent pooled owner is retained by a view; all its readers borrow
one existing token and its root permit. In particular, the read path avoids an
owning membership load outside the participant budget. General graph access and
future relocation still need their own admission/lifetime protocol.

Six access fixtures cover typed values, exact type/heap/certificate rejection,
revocation while an old view survives retirement and slot reuse, heap shutdown,
helped publication before the origin returns, and native pauses both during
admission and after view creation while a reserved helper reclaims other values.
The missing-certificate fixture deliberately synthesizes an intermediate state;
it is not a forced pause inside the catalog's root effect.

All six pass strict-provenance Miri, and a deliberately integer-reconstructed
payload pointer fails that check. Four access-check mutations fail on native and
WASI. Compiler controls reject escaping root/heap/value borrows, early root/heap
destruction, and a record reference escaping its owning root.

## Cleanup and qualification scope

A private cancellation or the final release of a retired cell invokes descriptor
cleanup. Actual payload destruction/free precedes logical credit refund. A stopped
exact cell owner retains only its own payload. A stopped participant destructor
also retains its permit and catalog charge until its owned state is destroyed.
Leaf certification does not bound arbitrary Rust `Drop`; threadless Wasm needs a
separate bounded callback contract before production polling can be qualified.

Sixteen native fixtures cover independent eligibility checks, exact rejection,
private cancellation at creation prefixes, discovery collisions, foreign-heap
retry, helped publication, retirement, slot reuse, heap shutdown and destructor
panic. Native schedules stop actual admission and payload destruction while a
reserved helper reclaims unrelated canonical and legacy allocations. Thirteen
portable fixtures run on WASI, including a nested helper that completes publication
and reclaims another allocation before the original admission returns.

The scoped 16-fixture Miri, Tree Borrows, ASan and TSan runs use only this new
feature. Full-feature native/WASI suites and lint/build checks are separate;
all-feature Miri attempts were stopped and remain recorded as incomplete. Eight
fault variants fail their intended assertions on both native and WASI release
builds. Compiler controls reject escaped guards, record cloning, private metadata
or certificate access, and duplicate consumption. Corrected catalog models are
reused only for their unchanged transition sources, not this concrete adapter.

```sh
cargo test --locked -p ristretto_gc --features experimental-canonical-leaves --lib canonical_leaf
cargo +nightly miri test --locked -p ristretto_gc --features experimental-canonical-leaves --lib canonical_leaf
```

Set `MIRIFLAGS=-Zmiri-disable-isolation` for the recorded Miri configuration; add
`-Zmiri-tree-borrows` for Tree Borrows. WASI needs the configured Wasmtime runner and
`--target wasm32-wasip2`. The catalog dependency is currently an in-repository
experiment; publishing its package layout is not settled.

Public initial allocation/result transfer, graph tracing/cleanup, scheduling,
physical/runtime accounting beyond the owning-wrapper census, real allocation policy,
admitted callers, JIT and full qualification remain open. See
[validation](VALIDATION.md#concrete-canonical-leaf-ownership-2026-11-14) and the
[catalog protocol](https://github.com/theseus-rs/ristretto/blob/0497123e05056546bc526ecc49c081f934f38d64/ristretto_gc/validation/catalog/catalog-tagged-prototype/README.md).

The subsequent [typed-access validation](VALIDATION.md#canonical-leaf-root-access-2026-11-14)
uses a fresh frozen workspace and build directory, with explicit fixture counts.
It extends the original leaf stage without changing public constructors or defaults.
