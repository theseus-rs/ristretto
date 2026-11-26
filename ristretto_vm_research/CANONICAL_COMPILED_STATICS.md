# Canonical compiled static state

The experimental request-only native tier uses the same canonical static registry
as the interpreter. This extends the admitted static int/reference layouts and
closed class initialization protocol; ordinary VM/JIT defaults are unchanged.
Volatile fields, unsupported static scalars, arbitrary inheritance, Java exception
objects/handlers and external direct calls to `<clinit>` remain rejected.

For New, getstatic, putstatic and static calls, the driver asks the registry to
prepare the resolved declaring class. A returned single-use lease identifies the
exact initializer. The driver executes it as a native activation and passes its
actual result to the registry's finish operation. It does not invoke the
interpreter as a fallback. The registry owns same-logical-owner recursion, other
owner waiting, Java error propagation and embedding-abort poisoning. Casts and
reference-array allocation do not initialize their component class.

Static field access uses prepared helpers that cannot initialize by themselves.
Final writes retain the declaring-initializer and active-owner checks. Static
backing roots never become native tokens; only public Java values read from their
catalog edges enter the native frame table. Putstatic retains an owning decoded
value and the parked caller frame while initialization waits. Returned values
use the existing owning caller result handoff. No frame-table borrow spans an
await. Cancellation drops frame owners independently of table mutation, drops
the initialization lease, and wakes waiting owners through the shared registry.

Initializer frames, requests and native slices obey the same limits as other
compiled Java calls. Native execution reports unavailable exact instruction
counts and zero interpreted visits. Prepared method profiles include actual
initializer entries/resumes/backedges. The existing single-active-run guard for
each CompiledProgram remains; distinct compilations of the same admitted Program
may attach independent local contexts to one JavaHeap. This is not a Send or
general parallel native execution API.

Dedicated runs initialize the entry class before entry_body_v1 and report that
work separately. Lazy initialization triggered inside the body remains inside
the body window. Failed or cancelled entry initialization has no successful body
window. Dedicated statics close before heap cleanup. Shared-heap statics persist
across entries/context detach and close only with that heap's registry.

Finite checks bind existing javac/HotSpot static and shared-static fixtures. They
cover actual native initialization, mixed-tier persistent state, class wait
wakeup with live caller references, unrelated class/collector progress, Java
failure and cancellation without replay, cancelled waiters, cross-owner cycles,
frame/slice limits and teardown. These checks are not timing, throughput, pause,
allocation-count or endurance evidence.
