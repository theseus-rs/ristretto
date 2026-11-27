# Ordinary VM array projection progress

Typed primitive-array views and fully rooted object-array views now carry tracing coverage in the default build. The existing `experimental-trace-projections` feature names remain compatibility aliases. Collector configuration and backend selection are unchanged.

A parked host writer previously made the ordinary collector repeatedly return `TraceProgress::Busy`, even when the real VM API exposed a graph-free primitive slice or a fully rooted object-array slice. That prevented mark closure and therefore unrelated reclamation. The explicit `try_map_trace_empty` and `try_map_trace_rooted` contracts now apply without an opt-in feature. Ordinary unrestricted writers, unknown custom tracing and partial rooting remain conservative.

## Coverage and lifetime

Primitive `Value::as_*_vec_mut` and `as_bytes_mut` calls select a primitive `Reference` variant. Their fixed-element slices cannot change that variant or introduce managed edges. Object-array `Value::as_class_vec_mut` first roots every current `Value`. Safe construction, cloning and extraction preserve owning strong handles, and the exposed slice cannot replace the containing `Reference` or safely invoke attachment/demotion hooks. Conditional target objects remain separately rooted allocations and receive normal tracing and reference processing. The array class's `Arc` remains owned by its existing heap-owner metadata independently of the array's element scan.

Coverage is published only after mutation preparation and the successful projection callback complete. Partial rooting, a failed projection and callback unwind publish no certificate. Nested mappings retain the original parent guard and narrow the exposure. Dropping or cancelling that guard revokes coverage before demotion, mutation barriers and unlock. Replacement and removal of rooted elements retain the ordinary publication/deletion barriers.

Each collector scan uses a fresh atomic read-modify-write certificate check. A stale true value from an earlier guard cannot authorize a later uncovered writer. A successful check establishes complete graph coverage without borrowing the locked payload. Relocation still requires its existing exclusive payload access; the certificate does not permit moving pinned storage or skipping required destruction.

## Scope of the checks

The default-feature `default_projection_progress` tests use real VM `Value` APIs. A native test fills a heap with exactly two byte-array charges, parks one mutator with a typed host guard, and requires another mutator's asynchronous allocation to reclaim the unrelated dropped array before the guard is released. It checks reclaimed bytes rather than requiring whole-cycle completion: allocation recovery deliberately returns as soon as enough space is available. A cooperative companion exercises the held-guard/headroom behavior on WASI. Object-array tests cover reachable children, replacement, clearing and future cancellation.

`partial_projection_progress` pauses after the first of two child roots is restored. Mark closure must remain blocked until both roots and coverage are established; it must then progress while the guard remains parked. The existing projection suites retain failed/unrestricted/nested/unwind, conditional-target and cross-heap ownership cases. Certificate models retain the explicitly ignored upstream Loom atomicity regression; an ignored model is not a passing model.

The separate `stalled_callback_progress` tests confirm that the already-detached finalizer, clearing and destructor paths allow a second mutator to start a later collection, reclaim another object and allocate while the earlier callback remains parked. The stalled object's storage stays charged until its own callback finishes. Native callback tests do not claim WASI thread coverage. Channel deadlines bound failure paths; they are not latency measurements.

## Remaining limits

The ordinary collector still has a publication gate and conservative dependencies on unknown outgoing references. A thread paused inside publication admission can delay a phase change, and an arbitrary `Trace` or incomplete root callback cannot safely be treated as complete coverage. This change removes the demonstrated typed-projection dependency; it does not establish zero global blocking, a pause bound, arbitrary callback qualification, or benchmark performance.
