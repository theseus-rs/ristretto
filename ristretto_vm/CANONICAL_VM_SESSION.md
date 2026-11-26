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
Admission, exact-program checks, frame roots, initialization and native
entry/backedge counters use the existing paths. WASI uses the interpreter.
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

## Application native methods

`VM::bind_canonical_native` loads and links a declared static native `(I)I`
application method, its target, and a caller-supplied set of companion classes.
The VM verifies the target before installing the binding. Ordinary native dispatch
compares the actual declaring class and method identities. A VM accepts one binding.

Each invocation passes an integer to the canonical interpreter and returns an
integer. Its catalog has 128 object slots, 128 root permits, one mutator and one
helper, with up to eight edges per object. The interpreter uses `Limits::default()`.
The VM heap supplies the payload budget. The per-call lifetime admits classes
without static fields or class initializers. Managed references stay within
the execution tier that owns them.

On native hosts a Tokio blocking-pool worker owns the local execution future.
Dropping the Java call signals that worker to drop the future and its roots at
its next suspension. WASI executes the future on the VM's local scheduler.
Known Java null, array and cast faults reach the caller as the corresponding
Java throwable. Heap exhaustion uses the VM's ordinary out-of-memory path.
A native caller receives a RuntimeException for an embedding limit or protocol
failure. The host controls scheduling and worker-pool capacity.

The session tests exercise both native tiers, same-collector ownership,
ordinary Java arrays, cancellation, and shared payload limits. Portable session
and drain tests run on WASI.
