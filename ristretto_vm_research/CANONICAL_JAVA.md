# Restricted canonical Java execution tier

`experimental-canonical-runtime` exposes `ristretto_vm_research::canonical::Program`.
This is an explicit, restricted execution tier. The ordinary VM, its `Value`
representation, interpreter, intrinsic registry and JIT retain their existing GC.
`experimental-canonical-jit` additionally provides `Program::compile()` and an
explicit `CompiledProgram::run` using a private canonical root-token ABI. The
ordinary VM's separate experimental compiled dispatcher still uses legacy roots.

`Program::from_classfiles` consumes ordinary javac classfiles, uses the existing
`Class`, `Method` and `Instruction` decoding, checks the entire closed class set,
and runs the existing strict bytecode verifier with a closed type context. It
rejects unavailable capabilities before a managed heap is created. `Program::run`
executes an admitted static `(I)I` method on a dedicated canonical heap. There is
no checksum-specific dispatch or substitution for Java method bodies.

After those checks succeed, the closed program links only constant-pool class,
field and call references used by admitted instructions. Per-owner metadata maps
retain class indices, typed field slots and method metadata; they contain no Java
values, roots or payload pointers. Missing or wrong-kind lookup entries reject.
Both execution tiers use those links and retain the existing receiver, argument,
return and dispatch-kind checks. Compiled dispatch uses borrowed method name and
descriptor keys, including overloads. Admission owns the extra metadata cost;
frame allocation, root publication, barriers and helper scheduling are unchanged.
Allocation diagnostics and any timing comparisons must use separately bound
artifacts and equivalent workloads; this representation alone is no performance claim.

The supported scope is direct subclasses of `java/lang/Object`, up to
eight integral fields (`boolean`, `byte`, `char`, `short`, or `int`) and eight reference fields per instance, ordinary constructors,
static calls, primitive numeric locals/operand stacks and conversions, integer and
selected numeric arithmetic/comparisons, branches, reference identity/null
comparisons and returns. One-dimensional arrays of all eight primitive kinds are
admitted, including array references in fields, parameters and return values.
Both canonical tiers also admit one-dimensional reference arrays of Object or a
closed class, with catalog-owned element edges and shared runtime helpers.
Long/double values occupy two local/stack slots; `pop2`/`dup2` remain rejected. The Object no-argument constructor
has its standard empty-body behavior. Field/method type checks and access checks
apply. Private nestmate accesses are not admitted. The implemented opcode list in
`admit_method` is the capability boundary.

Multidimensional arrays, `long`, `float`, and `double` object fields,
integer division/remainder, inheritance, interfaces, volatile and instance-final
fields, finalizers, weak/conditional references,
native calls, monitors, virtual/interface/dynamic invocation, Java exception
handlers and JIT/native/legacy-reference interop are unsupported. Runtime faults
are explicit Rust errors, not allocated Java Throwables. This is not a complete
Java VM or a replacement backend selection for ordinary workloads.

Both tiers additionally admit heap-owned static integral and reference storage
and logical-owner class initialization, including restricted static-final writes.
The request-only native driver executes initializers as native activations using
the registry's prepare/finish lease and prepared field helpers. See
[static state and initialization](CANONICAL_STATIC_STATE.md) and
[compiled static execution](CANONICAL_COMPILED_STATICS.md) for ownership,
failure, measurement and cancellation boundaries.

The native interpreter, native JIT and WASI interpreter use the same integer slots.
Stores keep the low bit for `boolean`, sign-extend `byte` and `short`, and
zero-extend `char`. The field oracle compares 116 results with HotSpot, including
raw integer stores and class-file constants. Regenerate its class and TSV with JDK 25
from the repository root.

```sh
java -Xverify:all benchmarks/gc/canonical-integral-field-oracle/IntegralFieldOracle.java ristretto_vm/tests/fixtures/canonical_bytecode
```

## Ownership and scheduling

Private payloads contain object class/primitive metadata, primitive-array metadata
and fixed boxed atomic elements, immutable reference-array header/page metadata,
or hidden static-storage class/int metadata.
They have no managed references or custom callback; reference-array elements and
internal page links are catalog-owned edges. Its Trace implementation declares
reference-free contents and charges the exact requested array backing bytes via
`external_size`; existing canonical admission checks still run. Array allocation
prepares and zeroes backing before incremental collector registration. The
`array_bytes` caller limit bounds one requested backing allocation, excluding
headers and allocator overhead. Preparation and backing destruction are not
bounded collector service operations or pause-qualified work. Class metadata lives outside the managed heap. Every Java reference field
exists solely in one catalog edge position. `putfield` completes the coupled
deletion/insertion write barrier before advancing, and `getfield` acquires an
independent logical root before returning a child.

Operand-stack values, local variables and call arguments own typed roots. Copies
share one logical root permit; child reads use an independent permit. Source and
target roots remain owned while a request is pending. Async method frames retain
their locals across helper work and host yields. Dropping a parked future drops
requests and all frame roots using the canonical cancellation protocol.

Each object publishes as many catalog edge slots as its verified class declares
reference fields. Primitive arrays publish zero slots. A reference-array header
or page publishes one slot for each child or element it addresses. Catalog
reservation and traversal use the configured maximum stride.

There is one mutator and one reserved helper per attached Java thread. Each pending request drives
one helper poll. The first pending result yields to the host; subsequent resumes
of that request execute at most 16 request/helper pairs. This is a per-request
work bound: an outer Java/array-construction future can finish a request, perform
other work and begin another request in the same executor poll. It is not a global
executor-work or wall-clock bound. This preserves the initial cancellation point
while amortizing later executor handoffs. Bytecode
execution drives helper work and yields every 64 instructions. Compiled execution
retains its resume and completed-request suspension points. The host chooses task
scheduling. Instruction/frame/request-poll/
cleanup-poll limits are caller policy and return errors, including when root or
object quotas cannot make progress. They are not wall-time or fairness bounds.
Cancellation cleanup at runtime destruction and the host allocator remain outside
any pause qualification.

Normal completion and execution errors drop frame roots and drive payload cleanup
before runtime teardown. A successful report requires zero payload bytes after
cleanup and zero catalog credit after runtime drop. Catalog reservation uses the
canonical checked census. Payload heap accounting uses the existing canonical
allocation path. Program metadata, Rust async-frame/stack allocations, allocator
overhead and RSS remain separate; this is not a whole-process bounded-memory claim.

## Reusing a heap across Java calls

`Program::with_heap(configuration, max_payload_bytes, cleanup_limit, async |heap| { ... })`
creates one collector and catalog for the entire workload. `heap.attach(limits)`
registers an interpreter context. On native, `compiled.attach(heap, limits)`
registers compiled code from that exact admitted program. Both contexts expose
`invoke(class, method, argument)`. Their Java frames release their roots on
return, error or cancellation. The contexts retain their mutator and helper
registrations between invocations. Compiled contexts also retain their frame
table. A new invocation starts with fresh instruction or native-slice limits.

Background work continues across calls. An invocation's release counter covers
the objects its helper released, including garbage from other calls. An idle
context can observe the catalog's completed primary-cycle bounds with
`primary_cycles()`. The callback can inspect current managed payload bytes.
The heap verifies detachment, drains the remaining payloads and refunds catalog
credit at workload completion. The caller controls the payload-byte ceiling,
catalog capacities, and final cleanup-poll budget separately.

The shared-heap tests park a Java invocation, complete eight more entries on a
second mutator, verify collection advances, then resume the first invocation
and check its checksum. The fixture uses 48 slots for 194 allocations. Native
tests also execute interpreters on separate OS threads and exercise a parked
native compiled invocation alongside interpreted entries. WASI schedules the
interpreters cooperatively. The prior timing runs use per-invocation heaps.
[Shared-heap tests](tests/canonical_heap.rs).

## Shared Java correctness fixture

`tests/fixtures/canonical/CanonicalGraphWorkload.java` is ordinary Java, compiled
with `javac --release 17 -g`. `run(int)` creates two Nodes per round, establishes a
cycle, invokes a checksum method, independently retains a child across unlinking,
checks child/backedge identity, and restores a cycle through another field. The
result is `rounds * rounds` under Java int arithmetic. Native and WASI use the
same embedded `.class` bytes. The compiled-dispatch track uses the same source
through the ordinary VM; canonical and compiled ownership paths remain separate.

Targeted checks:

```sh
cargo test -p ristretto_vm_research --no-default-features \
  --features experimental-canonical-runtime --test canonical_java -- --test-threads=1
cargo test -p ristretto_vm_research --no-default-features \
  --features experimental-canonical-runtime --lib canonical::tests -- --test-threads=1
cargo run -p ristretto_vm_research --no-default-features \
  --features experimental-canonical-runtime --example canonical_java
cargo run -p ristretto_vm_research --no-default-features \
  --features experimental-canonical-jit --example canonical_java -- --compiled
```

The integration suite checks real cycle/call/identity execution, complete cleanup,
quota/limit failure, unsupported layouts/opcodes and malformed stack types. The
private caller test cancels parked Java futures at several prefixes and checks
complete payload and catalog cleanup. The example executes eight rounds as a
correctness smoke, with no timing. WASI builds use `--target wasm32-wasip2` and
Wasmtime with `-W max-wasm-stack=8388608`.

The full VM unit-test artifact also imports WASI HTTP through unrelated test code;
running the filtered cancellation test requires Wasmtime `-S http=y`. The canonical
test itself uses no networking. The external integration test and smoke do not
require that additional import implementation.

## Explicit compiled canonical execution

The native-only compiled tier uses the intersection of Program admission and the
JIT's request-only capability gate. Every method, including constructors and
reference-returning methods, uses an owning Activation. Compilation binds exactly
two private callbacks: canonical frame publication and pending-error lookup.
Generated code cannot call legacy allocation/reference helpers or reinterpret
legacy `RuntimeContext`. The generated frame returns before allocation, field or
Java-call requests are handled. Both tiers use the same canonical allocation,
read-field, write-field, array preparation and typed array-access methods.

Reference tokens are opaque positive integers, never payload addresses. A private
process-wide source leases nonoverlapping ranges from the positive `i64` domain.
Every native frame owns a bounded snapshot of `Rc<Root<Payload>>` aliases. The
table stores only Weak entries and interns independently rooted
aliases using `Root::same_object`, so compiled reference comparisons preserve
identity. Complete replacement owners are acquired in preallocated scratch before
old owners are dropped. Foreign, expired or over-quota tokens reject without
changing the prior snapshot. A Box keeps each typed callback context at a stable
address; the native pointer is supplied for one resume and never points into a
movable async frame.

Dropping a frame directly drops its root owners without borrowing the table.
Cancellation therefore releases owners even while another caller has borrowed the
table. Frame/table/scratch capacities are explicit. Publication performs no vector
growth; entries may reuse physical slots but never token values. Metadata, Rc
backing, snapshots, activation storage and generated code remain host allocations
outside the canonical catalog/payload reservation; this is not whole-process
allocation or RSS accounting.

The table retains one exclusive lease, normally up to 4096 IDs. Issuance is local;
refill makes one weak CAS attempt and yields on contention with no table/frame
borrow held. Incoming argument/result roots remain owned until the destination
frame retains them. `request_polls` also bounds refill attempts per exported
reference. Native publication never issues IDs or reserves a range. Only exact
live-entry lookup authorizes a token; frame registry identity remains independent.

The former per-invocation 32-bit object ceiling is removed. The process-wide
positive `i64` domain remains finite and exhausts explicitly at `2^63 - 1`; IDs
never wrap or recycle. Unused IDs in a dropped lease are permanently discarded,
and no list of historical ranges is retained. Separate loaded copies of the
allocator are separate identity domains and must not exchange token tables.
These are operation-count/ownership guarantees, not a wall-clock latency bound.
See [token ranges](CANONICAL_TOKEN_RANGES.md) for the reservation protocol.

`CompiledLimits` separately limits native slices, frame depth, request polls and
cleanup polls, plus the per-array backing byte limit. Each resume receives one unit of request/backedge fuel. This is
not the interpreter's exact instruction limit. The report marks instruction count
unavailable for compiled execution and separately records actual generated
entries/resumes/backedges and requested native slices. A prepared program permits
one active invocation, making its profile deltas unambiguous. Compiled contexts
use Rc/RefCell and remain local to their executor thread. Dedicated runs own a
heap per invocation. An attached compiled context shares the workload's heap
with other contexts as described above.

Native compiled checks run `--features experimental-canonical-jit --test
canonical_compiled` and `--lib canonical::`. They include the same graph workload,
an ordinary Java reference-return/unlink fixture, quota/fuel failures, token
provenance, transactional publication, callback borrow conflicts, and cancellation.
WASI retains interpreted execution; the frame-token ownership tests also run there.

The shared retained-ring fixture adds 21 correctness cases per execution path.
It keeps a doubly linked graph and variable-length byte arrays alive across
iterations, leaves removed nodes as cycles, and uses an array alias after
unlinking. The larger step counts require slot reuse before method completion.
HotSpot results also match an independent sequence model. Run
`--test canonical_ring` with the same native/WASI feature configurations; see
[the workload contract](../benchmarks/gc/canonical-ring-oracle/README.md).
A separate 18-process G1/ZGC check observes allocation above minimum payload
demand in every fork and verifies independent checksums. Those aggregate counts
include headers and launcher overhead; they establish allocation exposure, not
exact survival of every object, measured-window collections or performance.

No native pause, throughput, latency, browser-turn, G1/ZGC, sustained-load or
endurance improvement is claimed. General VM value/layout and ordinary VM backend
integration remain required before this tier represents general collection behavior.

`CompiledProgram::method_profiles()` is an opt-in diagnostic snapshot of every
compiled method, including those with zero executions. Rows are sorted by Java
class/name/descriptor and contain actual native entries, continuation resumes and
loop backedges. Identities borrow prepared-program metadata; only the returned
vector allocates. Observation rejects while an invocation is active. The counters
cover the prepared program's lifetime, so a caller must bind before/after snapshots
to an independently validated completed invocation and the exact workload. Partial
execution from cancelled invocations remains in lifetime counters. These snapshots
establish method coverage, not timing, GC exposure or global pause coverage.

## Primitive-array correctness fixture

`CanonicalPrimitiveArrayWorkload` is ordinary `javac --release 17 -g` Java.
The same admitted bytes exercise all eight kinds, zero initialization, byte/short
sign extension, unsigned chars, numeric stores/loads, array identity/return
handoff, and a Holder field cleared before returning its array. All array
accesses validate the actual payload kind; `baload`/`bastore` accept byte or
boolean, and boolean stores use the low bit. Null receiver checks precede bounds
checks. Negative lengths and bounds failures return explicit Rust errors naming
the Java fault; this tier does not allocate Throwable objects.

The tab-separated oracle under `tests/fixtures/canonical` derives from the
independent HotSpot `-Xverify:all` oracle frozen at
`ristretto-gc-evidence/jit-runtime-v1/primitive-array-fixtures`: 127 successful
rows and 20 admitted Java faults. The separate primitive cast-mismatch fixture
is now admitted and reports ClassCastException at execution. `--test canonical_arrays` checks these cases against both canonical
execution paths on native and the interpreter on WASI. Direct lifecycle checks
cover exact external-size charge, floating-point raw bits, wrong-kind rejection,
failed writes preserving elements, and cancellation of pending array registration
for every primitive kind. This remains finite correctness evidence.

`CanonicalNumericEdgeWorkload` contributes 114 additional HotSpot oracle rows
covering NaNs, infinities, signed zero, conversion boundaries, floating-point
comparison branches, numeric array returns, mixed int/long/double/reference
arguments, and constructor slot placement. Float/double to integer conversion
uses JVM saturation and NaN-to-zero behavior. The canonical return boundary also
normalizes byte/char/short/boolean results according to
[JVMS ireturn](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.ireturn);
verification alone does not imply a narrow result is already normalized.

The narrow-return fixture under `tests/fixtures/canonical_bytecode` is deliberately
patched valid bytecode, not an unchanged javac output. Its checked-in Java template
and reproducible patch script replace four helper bodies with `iload_0; ireturn`
for B/C/S/Z descriptors. HotSpot `-Xverify:all` supplies 40 additional oracle rows,
including inputs whose high bits must be discarded. Together the three oracle
tables cover 301 admitted cases per execution tier, with a separate executed
cast-mismatch case and unsupported pop2/dup2 admission checks.

## Explicit service recording

`experimental-canonical-telemetry` exposes `canonical_telemetry::Telemetry` and
`Program::run_recorded(..., &Telemetry)`; the native compiled tier provides the
same entry with `CompiledLimits`. Ordinary `run` has no recorder. Callers create
and bind a fixed-capacity recorder before execution and export it after the run
and all guards have ended. Binding digests are declared inputs: a separate harness
must verify the actual source, executable, bytecode and complete configuration.

Calling `run_recorded` explicitly selects the entry-method body as the
`entry_body_v1` measurement window. It starts after dedicated heap/setup and ends
when the selected call returns, before cleanup. Cleanup helper events may occur
afterward. This is not an inferred warmup/body/report phase for arbitrary Java.
The recorded entry must itself represent the intended finite body.

A request lifetime spans preparation, incremental polls and host yields. Separate
synchronous spans cover primitive-array allocation/zero initialization,
mutator-request preparation, individual request polls, and helper polls. Each
synchronous span ends before yielding. Request IDs connect Pending polls, helper
service and uninstrumented gaps; gaps can include scheduler delay and other work,
so they are not pure collector time. Successful allocation counts include objects
and arrays; primitive array element operations have separate logical counters.
Cleanup samples charged managed bytes through the atomic `managed_bytes()` accessor
instead of locking statistics. Dedicated invocation ownership is still required
before zero bytes can be treated as cleanup evidence.

Overflow, abandoned guards, missing or failed body windows, and invalid span
relationships make the instrumented capture incomplete. Failed/cancelled runs do
not establish successful workload results. Root-owner release, generated native
execution/frame publication, cancellation/teardown and global actor state remain
explicit missing sources. Instrumented completeness is never complete global
pause coverage. Recording adds clock/metadata work; the output establishes no
unperturbed latency, throughput or zero-stop-the-world claim.

`--features experimental-canonical-jit,experimental-canonical-telemetry --test
canonical_service` checks actual Java service counts, array preparation coverage,
Pending/gap linkage, the body/cleanup boundary, failed preparation, overflow and
cancellation. Test-only declared bindings are not authenticated timing artifacts;
the independent capture harness supplies and verifies real bindings.

Recorded runs also bracket each measurement clock boundary with two passive
primary-cycle observations. Each observation may conservatively bound an instant
within its call. The window therefore guarantees at least
`max(0, end_before.lower - start_after.upper)` completed primary full epochs and
at most `end_after.upper - start_before.lower`. Missing acquisitions, impossible
monotone ordering, cancellation or multiple dedicated-heap windows cannot produce
an exact count. The raw four observations and measurement event identity remain
available for independent validation. Secondary-domain initialization, reclaimed
objects and callbacks are different quantities. Ordinary runs do not observe
cycles. Observer calls do not help collection, but their temporary-owner release
can run destruction; recorded timing includes this instrumentation and establishes
no unperturbed latency or pause bound.

## Reference-array layout and admission

The interpreter and request-only native tier support `anewarray`, `aaload`,
`aastore` and `arraylength` for
one-dimensional Object[] and arrays of admitted direct classes. Anewarray component
metadata is linked after full admission and strict verification; runtime execution
uses that immutable component mapping. Null elements are allowed. An array of a
closed class accepts only that exact class because inheritance stays unsupported;
Object[] accepts every public Java object/primitive-array/reference-array. Closed
class arrays are covariant to Object[]. All Java arrays are assignable to Object.
Declared nested array components/descriptors and multianewarray remain rejected;
placing an array value into Object[] does not admit those descriptors. Array stores
check null receiver, index bounds, then actual component assignability; faults remain
explicit Rust errors rather than allocated Java Throwable instances.

Reference arrays require eight catalog edges per physical object. The Java array
header doubles as the root of a dense radix-eight tree: up to eight elements use its
direct edges; length 2,048 uses 292 hidden pages plus the header (256 + 32 + 4 + 1).
Each page contains only immutable height metadata. Every tree link and Java element
is a canonical catalog edge. No owned vector of element Roots substitutes for the
heap graph. An access takes at most the tree height in canonical edge requests
(four at length 2,048; at most eleven for an i32 length). Each request retains its
own bounded poll/yield/cancellation protocol; this is not a wall-clock guarantee.

Construction checks negative length and all node/byte arithmetic before publishing
anything. It rejects a tree larger than configured physical object slots, a degree
other than eight, or fewer than height+1 temporary root permits. Per-array requested
payload charge is the exact canonical Gc allocation layout of Payload times the
physical node count, with zero external page backing; it is checked against the
same `array_bytes` limit that bounds primitive-array backing bytes. This per-array
bound is separate from total heap credit and the precharged catalog reservation.
Nominal capacity is not free capacity: existing live objects/roots can still cause
an actual incremental admission to fail. Every payload still passes ordinary
canonical heap/root/slot admission.

Eager depth-first construction uses at most eleven fixed temporary frames. A child
is attached through a canonical write before descent. No array escapes until all
pages are present; cancellation or failure drops the bounded construction roots,
leaving the published prefix to ordinary canonical reclamation. Element traversal
owns only its bounded temporary path roots. Hidden pages fail Java-reference type
checks, array-element export, array stores and arraylength; they cannot be supplied
as legal Java values. Caller frames and other live roots remain separately bounded
and can exhaust the remaining root quota.

`Report.allocations` counts completed successful physical allocation requests;
`internal_allocations` is its hidden-page subset. Java allocations therefore equal
allocations minus internal_allocations. `reference_array_allocations` counts fully
constructed Java reference-array headers. Existing `array_allocations` continues
to count primitive arrays, matching PrimitiveArrayPreparation recording.
`released_objects` counts total reclaimed physical catalog slots and has no
release-by-kind classification or GC-cycle meaning. ChildRead/EdgeWrite counters
include page traversal and construction; array_reads/array_writes count successful
Java array operations (array_reads also includes arraylength, as before).
Representation metadata and pages must remain included in physical resources.
On cancellation, a pending allocation can have installed a physical slot before
returning Ready and incrementing that counter; reclamation may therefore include
that additional pending slot. A cancelled invocation returns no successful Report,
and its abandoned recording remains incomplete. Cleanup checks use managed bytes
and catalog credit, not equality with an incomplete allocation counter.

The shared Java fixture and independent checksum/fault model are described in
[the reference-array contract](../benchmarks/gc/canonical-reference-array-oracle/README.md).
It does not yet admit full GcBench: static/class-initializer roots, final/volatile
semantics, required general calls and production JIT eligibility remain
separate work. Defaults and the ordinary VM backend are unchanged.

## Closed-layout casts

`checkcast` targets are linked after admission and strict bytecode verification.
Targets may be Object, an admitted direct class, a one-dimensional primitive array,
or an array of Object/an admitted direct class. Class references must be public
or in the referring class's package within the single closed loader namespace;
module graphs and external loader constraints remain outside this tier. The same
cold check applies to New and Anewarray. Inaccessible targets reject before
execution rather than deferring an unsupported linking error. Null succeeds for
admitted targets. Object accepts every public payload kind; closed classes match exactly; primitive array kinds match
exactly; closed reference-array components are covariant to Object[]. The actual
component metadata is unchanged, so a covariantly cast Node[] still rejects an
Other element. Interfaces, unadmitted classes, multidimensional targets and
`instanceof` remain rejected before execution, including for statically null inputs.

The shared helper moves the original owning reference through the type check.
Successful casts preserve identity without new payloads, root permits or catalog
edges. It validates the actual public payload kind before accepting Object, so
foreign roots and hidden pages fail closed. A failed valid Java cast reports
`TypeFault("ClassCastException")`; throwable objects and handlers remain unsupported.
The request-only native driver accepts exactly one owning token/null argument and
only the CheckCast request kind, then returns the same reference through the common
frame handoff. No ordinary runtime context crosses this boundary.

`CanonicalCastWorkload` supplies 85 javac/HotSpot cases covering eight primitive
array kinds, null, classes, covariant arrays, identity, field/call/return handoffs,
and cast/store/bounds/null faults. Separate public/package-private class variants
check casts and allocations across package boundaries. These semantics follow
[JVMS checkcast](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.checkcast)
and [class access control](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-5.html#jvms-5.4.4)
within the admitted scope. The independent scalar/fault oracle is under
`benchmarks/gc/canonical-cast-oracle`. The earlier primitive cast-mismatch fixture
is now an executed failure case. These are finite correctness checks, not a
broader Java capability or performance qualification.
