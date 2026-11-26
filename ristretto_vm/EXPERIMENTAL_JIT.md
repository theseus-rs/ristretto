# Explicit experimental reference-method preparation

The `experimental-reference-dispatch` Cargo feature exposes
`VM::prepare_experimental_jit(class_name, method_name, descriptor)`. It prepares
one method synchronously for subsequent ordinary VM dispatch. Configuration must
enable the native JIT and set `batch_compilation(false)`. Unsupported targets,
interpreter mode, intrinsic interception, unsupported capabilities and compiler
failures return errors. Preparation is intended before workload invocation; it
does not transfer an already executing interpreter frame into compiled code.

The capability gate supports the existing production primitive subset plus
reference locals/returns/comparisons, object allocation, field access, and
static/special calls. It is independent of class names. Arrays, monitor bytecodes,
synchronized methods, exception tables, explicit throws, virtual/interface calls
and dynamic calls remain excluded from this preparation slice. Callees remain
subject to ordinary dispatch unless explicitly prepared themselves. Runtime
exceptions still propagate through the existing owning throwable bridge to
interpreted handlers.

Production `can_compile` and all defaults remain unchanged. The feature does not
automatically prepare any method. With `execution-profile`, an explicit
`experimental_preparation` field marks these requests while preserving production
rejection reasons. Compilation success is not proof of execution: interval
compiled entries and taken backedges must be checked after actual invocation.
Background compilation is disallowed during preparation to prevent pending
production requests from replacing prepared code. Cached successful functions
also survive concurrent negative cache insertion.

The integration test `experimental_jit` recompiles the shared
`tests/fixtures/canonical/CanonicalGraphWorkload.java` with javac, verifies the
result matches the frozen shared class files, and compares HotSpot and Ristretto
results. It executes the graph under the interpreter, mixed compiled/interpreted
calls, and all prepared graph methods. It collects between native slices and
checks collection, migration, resumption, cancellation and exception ownership
using `ExperimentalJitLifecycle.java`. Infinite-loop lifetime checks run in a
bounded subprocess. Optional `RISTRETTO_JIT_EVIDENCE` writes diagnostic intervals
outside Cargo storage; these are correctness evidence, not performance samples.

```sh
CARGO_BUILD_JOBS=1 cargo test -p ristretto_vm --no-default-features \
  --features execution-profile,experimental-reference-dispatch \
  --test experimental_jit
```

This uses the VM's legacy `Gc<RwLock<Reference>>` heap and existing owning frame
contexts. The restricted canonical execution tier uses a separate root and
payload representation. Sharing Java bytecode and arithmetic results does not
make their object layouts interoperable. Canonical allocation, field access,
call arguments/results and native continuation root publication need a common
handle boundary before canonical compiled execution is supported. The full
`GcBench` measurement loop, arrays and required coverage manifest remain open;
this test does not qualify J0–J5 or collector performance/pause/endurance gates.
