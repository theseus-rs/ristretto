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
rejected. Final static writes are rejected outside the declaring initializer;
this slice still rejects every `<clinit>`, so only constant final values are
usable. The next slice enables initializer execution and its cross-context tests.

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
