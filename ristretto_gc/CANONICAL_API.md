# Experimental canonical caller API

This historical experiment can be reproduced from the [archived source](https://github.com/theseus-rs/ristretto/blob/0497123e05056546bc526ecc49c081f934f38d64/ristretto_gc). The current Java preview uses the [unified JVM heap](../GC_PREVIEW.md).

Enable `experimental-canonical-runtime` to use `ristretto_gc::canonical`. The feature
exposes the existing graph runtime through transparent wrappers; it does not select
the VM collector or change ordinary Gc allocation. The current layout reserves
1–8 catalog edge positions per object, admits reference-free payloads and rejects
conditional-reference layouts, arbitrary managed Trace graphs and finalizers.

`Configuration::charged_bytes` computes the complete existing catalog/runtime
reservation with checked arithmetic. Construct `Runtime` with a matching collector
and `Arc<Budget>`, then register mutators and reserved helpers. Catalog credit and
payload heap credit are separate. No extra wrapper allocation or retained owner is
added by the API. Native/WASI size/alignment fixtures check the charged representation
of every transparent wrapper. Caller variables, stacks, supplied collector/budget
owners, payloads, allocator overhead and RSS remain separately accounted.

## Ownership and incremental work

`Mutator::allocate`, `write` and `child` return requests borrowing that mutator
exclusively. Poll returns Pending or one completed result; it does not retry in a
loop, spawn a task, consult a clock or yield to a host. Callers schedule helper polls
and host yields, including on bare Wasm. Full object/root quotas can leave work
pending; callers choose cancellation and exhaustion policy. Dropping a request
preserves the existing cancellation/committed-effect protocol without collecting
in Drop. A repeated poll after completion returns Finished.

`Root<T>` copies share one logical root without a T: Clone bound. A child read obtains
an independent logical root. `same_object` compares catalog-scoped record identity,
including separate roots of the same payload. It exposes no payload pointer or raw
token. `Runtime::borrow` returns a shared View tied to the exact matching runtime
and root. Rust rejects escaping that view or dropping either owner while borrowed.
No mutable payload view, raw catalog or managed edge escapes. Graph mutation uses
only coupled write operations; a full-quota read can observe a later null replacement.

Mutators/helpers may outlive their initial runtime while their collector borrow
remains valid. Roots may retain storage after runtime/collector shutdown, but a
replacement runtime cannot read them. The [shared-root correction](CANONICAL_SHARED_ROOTS.md)
frees final Arc backing before releasing the unique root and refunding catalog credit.
Debug output is opaque and invokes neither payload formatting nor collector polls.

Helper polling bounds protocol work, not arbitrary payload Drop or host allocator
delay. Preparation hooks, allocation and destruction remain outside that bound.
The API makes no pause, fairness, capacity, latency or sustained-load guarantee.

## Executed examples and evidence

[The clock-free example](https://github.com/theseus-rs/ristretto/blob/0497123e05056546bc526ecc49c081f934f38d64/ristretto_gc/examples/canonical_client_smoke.rs) executes a cyclic graph,
independent/copy-root retention and complete payload/credit cleanup. It ran both as
a native shared library and as a bare wasm32-unknown-unknown module through Wasmtime;
the exported `canonical_client_smoke` returns 1 on success and traps on failure.
[External-client fixtures](https://github.com/theseus-rs/ristretto/blob/0497123e05056546bc526ecc49c081f934f38d64/ristretto_gc/tests/canonical_api.rs) also execute on native and WASI.

The [report](../target/gc-implementation/canonical-client-REPORT.md) records 686 native
GC tests (one existing ignore), 379 WASI tests, Clippy, formatting, bare-Wasm checks,
six external fixtures under each of four memory tools, one positive/four negative
compiler controls and two smoke executions. Existing catalog/pool model evidence is
reused only through unchanged inputs. No new performance improvement is claimed.

General VM value/array layouts and execution callers, dynamic graph layouts,
root-creation allocation, arbitrary callbacks/destruction, backend/default selection,
compaction, J0–J5, native pause capture, G1/ZGC comparisons and sustainable/hour/day
qualification remain open. The graph runtime is now callable outside the GC crate;
this is a prerequisite for VM integration, not its completion.
