# Admitted Java sessions on an existing VM

With `experimental-canonical-runtime`, an admitted `Program` can execute against
an existing `VM::garbage_collector()` through `Program::with_vm`. It reserves a
separate bounded catalog and static registry. Payload reservations use the VM's
existing global managed-byte limit, shared with ordinary objects and other
canonical sessions. Catalog credit remains separately bounded by `Configuration`.

```rust,ignore
let completion = program.with_vm(
    &vm,
    configuration,
    cleanup_poll_limit,
    async |heap| {
        let mut thread = heap.attach(Limits::default())?;
        let first = thread.invoke("CanonicalGraphWorkload", "run", 3).await?;
        let second = thread.invoke("CanonicalGraphWorkload", "run", 4).await?;
        Ok((first, second))
    },
).await?;
```

For native execution with `experimental-canonical-jit`, prepare
`let compiled = program.compile()?` before the session, then attach through
`compiled.attach(heap, CompiledLimits::default())?` inside the same closure.
Admission, exact-program checks, frame roots, initialization use the existing paths. The public `Invocation` retains its existing fields;
the compiled session control rejects a zero native-slice budget, and the
existing dedicated compiled regressions verify entry/backedge counters. WASI uses the interpreter.
`Program::with_collector` provides the same session boundary for direct collector
embeddings. `Program::with_heap` remains available for a dedicated collector and
also verifies zero total managed bytes after successful cleanup.

Cleanup closes only this session's static registry after all attached contexts
have dropped. An exclusive `Runtime::drain` guard rejects existing participants
and prevents new registration. Each poll performs one existing helper visit and
one physical object-cell occupancy observation. A full empty pass certifies no
retained payload cells in this catalog. Occupancy includes private pooled cells,
retired records and destruction in progress. Pre-pool private records drop before
their mutator registration; helpers cannot create payload cells. Existing roots
can delay completion but cannot repopulate a previously empty cell during the
exclusive drain. No new allocation counter, clock or callback is introduced.

The cleanup poll limit includes these census steps. Cleanup yields after every
64 polls; empty catalogs also require one incremental pass when the collector
contains other payloads. A zero global byte count is a sufficient empty-session
shortcut because every VM payload has a positive reservation; this preserves
the dedicated scalar-only API's zero cleanup work. Global nonzero never causes
waiting on another owner's objects. No unbudgeted full-pool scan runs inside a
helper poll or a host turn. Poll work and capacity are finite;
payload destructors, catalog destruction and the host allocator have no wall-time
bound. Dropping an invocation releases its frames; callers can then await bounded
session cleanup. Dropping the whole session cancels its owners directly, without
polling a collector loop in `Drop`. It does not stop the VM collector or release
ordinary VM roots. Successful session completion checks its own catalog refund.

`JavaHeap::managed_bytes()` is explicitly collector-wide telemetry. It is not
cleanup authority: an ordinary rooted value or a second session can remain live
after this session completes.

The remaining launcher boundary is explicit class loading/admission and calling
this API on the launched VM. The ordinary CLI does not automatically dispatch
arbitrary methods into this tier. There is no fallback from rejected bytecode,
ordinary/canonical reference conversion, shared static registry across sessions,
or unrestricted Java support. `tests/canonical_vm_session.rs` exercises the actual
VM method, both native tiers, same-collector owners, ordinary Java arrays,
cancellation and shared payload limits; portable session/drain tests run on WASI.
