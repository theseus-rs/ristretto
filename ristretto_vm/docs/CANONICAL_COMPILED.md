# Restricted canonical native execution

The JVM and its compiled tier now use the same collector. The commands below apply to the archived research runtime. [Current GC preview](../../GC_PREVIEW.md) · [Archived implementation](https://github.com/theseus-rs/ristretto/tree/0497123e05056546bc526ecc49c081f934f38d64/ristretto_vm_research).

With `experimental-canonical-jit`, an admitted `canonical::Program` can prepare a
`CompiledProgram`. Preparation checks every method against the request-only JIT
capability. The ordinary VM dispatch policy is unchanged. Native compilation is
unavailable on WASI; the interpreter and token-table compatibility checks remain.

This consumer slice supports scalar/reference control flow and admitted static or
constructor calls, object allocation and instance fields. A native caller returns before invoking its callee. An explicit,
bounded vector owns the Java activation frames; Java recursion does not recursively
poll Rust futures. Each frame's typed snapshot owns its canonical references, and
return handoff acquires the returned owner before retiring the callee snapshot.
Call, return and loop boundaries yield to the host. Dropping the invocation drops
all snapshots and resets the exclusive prepared-program run lease.

`CompiledLimits` separates native slices, frame depth, request polls and cleanup
polls from interpreted instruction counting. Reports count actual native entries,
resumes and single-unit backedge yields. `instruction_count_available` is false;
zero `instructions` is not a claim of zero Java work. Limits are work bounds, not
wall-clock pause bounds. `run` owns a dedicated heap and completes cleanup before
returning a successful report or runtime error.

The compiler protocol can be broader than this consumer. The effective capability
is the intersection of verifier, canonical admission and compiler support. No
legacy context, interpreter fallback or payload pointer is linked into generated
code. Object/field requests use the same typed Execution helpers as the interpreter.
Static access and initialization use the same registry-bound prepare/finish and
prepared field helpers as interpreted execution. Initializer activations live on
the explicit bounded frame stack. Each owns its initialization lease and the
suspended action; static-store values remain owned across preparation and waits.
Errors unwind leases from the inner initializer outward, and cancellation poisons
only unfinished leases owned by that logical Java context. No interpreter execution
is called by native initialization. Checked casts use the shared closed-layout target
and preserve null/identity; 85 native HotSpot rows cover object and array casts and
fault ordering. Unsupported interfaces, multidimensional targets and instanceof
remain excluded at admission. Reference-array requests use the shared
paged layout, bounds/component checks and owning return handoff; hidden pages never
become Java references. The 34-row native oracle includes full 2,048-element arrays,
covariance, mixed references, cycles and null/bounds/store fault ordering. Primitive arrays use
the shared typed allocation, length, load and store helpers. The native oracle suite
covers all eight kinds, wide arguments, array returns, saturation, narrowing and
null/bounds/negative-length faults (301 HotSpot rows).

Runnable proof:

```sh
cargo test --locked -p ristretto_vm --no-default-features \
  --features experimental-canonical-jit --test canonical_compiled
```

The Java fixture is compiled with `javac --release 17 -g`. A separate ordinary Java
oracle uses `-Xverify:all` and emits ten rows in `compiled-calls-oracle.tsv`; its
main/string/output helpers are excluded from the admitted classes. The tests check
HotSpot and interpreter parity, real native entry/backedges, frame/slice errors,
cancellation and 1,025 live Java frames on the ordinary executor stack.

`compiled.attach(&heap, limits)` creates a reusable `CompiledThread` for exactly
the same admitted Program. `invoke(class, method, argument).await` resets call-local
limits and counters; static roots persist until the heap closes. Separate local
contexts can await the same initializer while an unrelated context allocates and
collects. A foreign Program is rejected before participant registration. These
native contexts retain their local `Rc` frame tables and are not `Send`.

Dedicated `run` closes its static registry before cleanup. A fieldless initializer
still executes natively. Initialization work is included in the native entry/slice
counters; the instruction count (including its initialization subset) is unavailable
when `Report::instruction_count_available` is false. Shared `Invocation` retains its
existing result/allocation/helper contract. The shared initializer outcome authority
preserves Java failure state across repeated entries.

The native static tests replay the persistent Java oracle, nested initialization,
first/subsequent failure, same-owner cycles, waiting writes during unrelated GC,
owner/waiter cancellation, final-write admission, typed arrays and complete heap
teardown. Separate tests cover foreign-heap attachment, limits and context reuse.
