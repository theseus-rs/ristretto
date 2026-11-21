# Canonical static field storage

The restricted tier admits static `int` fields and references to closed admitted
classes. Each class with static fields owns one hidden catalog object and one
root permit on its Java heap. Its reference fields use ordinary catalog child
loads and coupled writes, so replacing a static reference releases the old graph
when no other root or edge retains it. Hidden backing is tagged and rejected by
Java receiver/type checks. It never becomes a Java value.

`ConstantValue` integer values are installed before the class is prepared.
Ordinary static integers use nontearing atomics. Volatile statics, non-int scalar
statics, array statics, unsupported inheritance and instance final fields remain
rejected. Final static writes are accepted only in the declaring `<clinit>` while that
context owns initialization. The class state is published with release/acquire
ordering after the initializer completes. Static calls, `new`, static field access,
and heap entry all gate on the declaring class. The separate synchronous scalar
API rejects classes needing initialization; use the heap entry API for those.

A heap registry owns static roots across entry returns. Each attached context has
a nonreused logical owner identity. Storage preparation uses a single-use lease;
cancellation or embedding failure poisons that class and wakes its class-local
waiters. This is embedding cancellation, not Java interruption semantics. A
program with no static fields or initializer creates no registry or owner state.

`allocations` counts completed physical registrations. `internal_allocations`
and `static_storage_allocations` identify the hidden-storage subset. Entry
preparation counters are separately reported in `entry_initialization` and remain
included in the full counters. Root/object capacity is checked before heap
creation; the normal payload budget charges each hidden object. Additional live
Java values still require their own available roots and object slots.

Dedicated runs close static state before draining. Shared heaps retain it until
all contexts detach and the outer workload finishes; static roots drop before the
catalog on outer-future cancellation. Successful teardown requires zero payload
bytes and full catalog refund. No default backend, timing or pause claim changes.

Run `tests/fixtures/canonical/check_static_fields_fixture.sh` with a JDK 17+
`javac`/`java` on PATH to reproduce the classfiles and the independent HotSpot
sequence. The peer oracle is never part of the admitted class set.

## Class initialization and native-driver handoff

Initialization is a class-local state machine: uninitialized, owned by one logical
Java context, initialized, Java-failed, or embedding-aborted. Only recursion by
the same owner may read an initializing class. Other contexts register a waiter
before rechecking the phase, preserving completion/cancellation wakeups. Waiting
on one class does not hold a collector lock or block unrelated classes and GC.

The owner lease is installed before the fallible static backing request. Its
single-use finish operation checks registry, class and owner identity. Dropping
an unfinished lease marks that class embedding-aborted and wakes its waiters;
dropping a waiter does not change the owner's authority. Registry roots persist
until heap teardown even for failed classes.

The supported initializer Java fault is a null field receiver, wrapped once in
`ExceptionInInitializerError`. An existing initialization Error is propagated
without another wrapper. Subsequent use of a Java-failed class reports
`NoClassDefFoundError`. Host quota/limit failures and future cancellation are
embedding failures, not modeled Java interruption or OutOfMemoryError delivery.
Handlers and unsupported exception classes remain excluded.

Entry initializer visits count against the same instruction budget as the body,
and its work is reported as a subset in `entry_initialization`. Initializers
triggered from inside a body remain in that body's full counts. The future native
driver must call `prepare_initialization`, drive exactly the returned lease's
`class()` / `initializer()`, and pass that lease and outcome to
`finish_initialization`. Prepared static helpers check terminal or same-owner
state and never select a different initializer themselves.

The initialization fixture script reproduces the 13-row HotSpot oracle. Native
and WASI tests also park an initializer, retain a new object's sole owning value
at a pending `putstatic`, run unrelated collection work, and verify normal or
cancelled handoff. Native additionally runs the owner and waiter on independent
OS-thread executors. These are finite correctness tests, not pause measurements.
