# Canonical static state and initialization

The canonical interpreter supports static integral and reference storage and class
initialization on both dedicated invocations and a long-lived JavaHeap. This is a
foundation for GcBench's roots/marks/SLOTS layout, not full GcBench admission.
Ordinary VM defaults and legacy static-value storage are outside this path.

## Heap authority and lifetime

Each prepared class with static fields has one hidden canonical storage payload,
with eight shared slots for boolean, byte, char, short and int fields, and up to eight catalog-owned reference edges. Immutable
per-record degree equals that class's reference-field count; global degree stays
at most eight. The payload has no Root, legacy Gc or owned reference collection.
The heap registry owns its one canonical Root in a set-once slot. Static reads
obtain ordinary owning children, and reference overwrites complete ordinary
catalog write barriers. Hidden storage never becomes a Java value/frame token.

JavaHeap owns the registry and its immutable Program namespace. Attached contexts
share state across entries; detaching one context does not release statics. A
Program::run creates a new heap/registry per invocation. No-static/no-initializer
programs keep an empty handle and allocate no registry or hidden managed storage.
Fieldless classes with an initializer need metadata, but no hidden payload.

Hidden storage uses existing payload, object and root admission. Conservative
preflight counts storage-bearing classes against root/object limits; temporary
object/child/traversal roots and simultaneous live values still require additional
capacity. Class-state/notification/owner metadata is host memory bounded by the
admitted class set and attached contexts, outside managed/catalog byte accounting.
This is not a whole-process allocator/RSS bound.

## Logical initialization owners

Each attached context receives a distinct checked nonwrapping owner ID within its
registry. IDs never recycle; terminal phase tags are disjoint from owner IDs.
Authority is registry identity plus owner identity, not an OS-thread ID or a
reusable address. A live lease retains its registry/state identity. Exhaustion is
an explicit attachment error. Contexts reuse their identity across entries and
Rust's exclusive invocation borrow prevents concurrent use of one context.

Each class has an atomic phase/owner, a set-once backing Root and a class-local
notification. Only Uninitialized -> Initializing(owner) claims initialization.
Only that owner may complete it. Initialized, JavaFailed and EmbeddingAborted are
terminal; there is no reset/replay or owner-ID reuse.

1. Same-owner recursive initialization returns normally after prepared defaults
   and narrowed integral ConstantValue values are installed. Other owners cannot observe a
   partially initialized class through an early-success path.
2. A competing context registers its notification before rechecking phase, drives
   a helper step, then awaits that class notification with all frame/argument roots
   intact. It holds no registry/table lock across Java execution, GC work or await.
3. An owning lease is installed before fallible backing preparation. The owner
   executes the exact initializer under normal instruction/frame/request limits.
4. Success publishes Initialized with release ordering before waking waiters.
   Known admitted Java Exceptions become ExceptionInInitializerError once;
   existing initialization Error categories propagate without another wrapper.
   JavaFailed subsequently reports NoClassDefFoundError, without replay.
5. Host cancellation/quota/boundary failure publishes EmbeddingAborted and wakes
   waiters. This is explicit embedding behavior, not Java interruption semantics.
   Cancelling only a waiter removes that wait and does not abort the owner.

New/getstatic/putstatic/invokestatic gate the resolved declaring class. Array
creation and casts do not initialize the component class. Final static writes
require the declaring initializer and its active owner. Interfaces, non-Object
superclasses, loader/module graphs, Throwable objects/handlers and class unloading
remain unsupported. Long, float and double static fields, volatile fields and instance-final fields
reject; GcBench's volatile sink and final Node fields therefore still reject.

The admitted ordering follows [JVMS initialization](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-5.html#jvms-5.5)
and [putstatic](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.putstatic).
No complete Java cause-object/stack-trace model is claimed. Cross-owner cyclic
initialization can deadlock as in Java; it is not silently treated as recursion.
Unrelated classes and GC can continue, and host cancellation has an explicit
poison/wake path. Notification queues, wake callbacks and host scheduling have no
wall-time bound or GC pause qualification here.

## Accounting, measurement and teardown

Completed physical registrations include hidden storage; internal_allocations
includes it, and static_storage_allocations identifies that subset. Primitive and
reference-array counters retain their meanings. Only the actual initializer owner
records its initializer calls/allocations. A waiter records its own helper work
and wait attempts; wait attempts are not measured durations or actual park counts.
A helper can reclaim objects from any invocation, so a shared invocation's release
count need not equal its allocation count. Failed/cancelled prefixes can include
submitted physical work beyond completed-allocation counters.

Entry initialization precedes the dedicated entry_body_v1 measurement. Its
InitializationWork is a subset of full invocation work, including native slices
and an explicit instruction-counter availability flag. Later lazy initialization
inside the body stays inside it. Shared contexts restart per-entry counters but
preserve class state. Existing fine-attribution schemas requiring every request
inside the body must reject or explicitly represent initialization prefixes.

After all contexts detach, the heap releases static ownership before final drain.
Static roots also precede Runtime in field-drop order on outer-future cancellation.
Successful HeapCompletion requires zero managed bytes and complete catalog credit
refund. Root teardown, allocator/destructor work and notification callbacks remain
outside any latency/global-pause qualification. Native OS threads may each attach
a local context to the same heap; suspended-task migration is not qualified. WASI
checks cover cooperative contexts, not an unimplemented WASI OS-thread facility.

## Request-only native handoff

The request-only native driver also executes admitted static-bearing Programs. It uses
prepare_initialization(class), executes only the returned lease's class()/initializer(),
then calls finish_initialization with the same lease/outcome. The shared prepare
method owns all owner/wait decisions; native code does not inspect phase or infer
recursion. Dropping the lease aborts unfinished ownership and wakes waiters.

read_prepared_static/write_prepared_static require Initialized or the caller's
active owner phase and never run an initializer. Native field dispatch uses
these after its native initializer gate, preventing an invisible interpreter
fallback. The interpreter's read_static/write_static first initialize and then
call the same prepared helpers. No legacy runtime context/static-value table is
interchangeable with this registry.

Initializer frames and native slices consume the ordinary compiled limits. Entry
initialization remains before the recorded body and is reported with its own
native-slice subset and unavailable exact instruction count. Parked callers keep
owning frame/argument roots across initialization waits. The existing single-active
guard per prepared compilation remains. See [compiled static execution](CANONICAL_COMPILED_STATICS.md)
for the handoff, finite tests and unchanged unsupported behavior.
