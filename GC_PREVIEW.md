# Restricted Java GC research preview

The alpha JVM uses the ordinary Ristretto collector for every Java allocation on
native, WASI and browser targets. The canonical catalog and bounded pool are a
separate research implementation in `ristretto_gc_catalog`. Its restricted
interpreter, compiled tier and WASI examples live in the workspace-only
`ristretto_vm_research` crate. Run
`python3 benchmarks/gc/release/verify_alpha_heap.py` to check the dependency
graphs of the shipping JVM, launcher and browser engine.

## Build from the archive

Verify the release attachment before extraction, then enter its source directory:

```sh
sha256sum --check SHA256SUMS
tar -xzf ristretto-gc-preview-*.tar.gz
cd ristretto-gc-preview-*/
rustup toolchain install 1.100.0 --no-self-update
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR="$PWD/../preview-native-target"
python3 benchmarks/gc/release/verify_preview.py native-interpreted
python3 benchmarks/gc/release/verify_preview.py native-compiled
```

Use Python 3.11 or newer and a native C compiler/linker on the supported
x86-64 Linux host. Cargo downloads
the versions in `Cargo.lock` on the first build. The archive already includes the
Java 17 classfiles, so running this example does not require `javac`. The native
compiled mode builds its admitted Java methods before executing them.

For WASI, install Wasmtime 50.0.1 and put `wasmtime` on `PATH`:

```sh
rustup target add --toolchain 1.100.0 wasm32-wasip2
export CARGO_TARGET_DIR="$PWD/../preview-wasi-target"
export CARGO_TARGET_WASM32_WASIP2_RUNNER='wasmtime run -W max-wasm-stack=8388608 -S http'
python3 benchmarks/gc/release/verify_preview.py wasi-interpreted
```

The verifier runs the following commands with a complete build/run watchdog and
checks the report's top-level fields, keeping nested initialization counts apart:

```sh
cargo run --locked -p ristretto_vm_research --no-default-features --features experimental-canonical-runtime --example canonical_java
cargo run --locked -p ristretto_vm_research --no-default-features --features experimental-canonical-jit --example canonical_java -- --compiled
cargo run --locked -p ristretto_vm_research --no-default-features --features experimental-canonical-runtime --example canonical_java --target wasm32-wasip2
```

Each run must return **64**, report **16 allocations and 16 released objects**,
and finish with **zero payload bytes and zero catalog bytes**. Compiled execution
must additionally report **25 compiled entries and 8 backedges**. The example uses
four object slots, twelve root slots, two reference fields per object, one mutator
and one helper. It creates cycles and retains children across unlinking. These are
functional checks; released slots are not collection-cycle or pause counts.
[Java source](ristretto_vm/tests/fixtures/canonical/CanonicalGraphWorkload.java) ·
[Rust example](ristretto_vm/examples/canonical_java.rs)

## Supported scope and limits

Admission verifies the complete closed class set before creating a managed heap.
Classes directly extend `Object` and resolve their referenced classes from that
supplied set. The admitted interpreter and native compiled tier cover:

- Integer/scalar arithmetic, conversions, branches and bounded loops; constructors
  and static calls in the admitted set.
- Instance integer fields and graph references, identity comparisons, child
  retention across unlinking, and cyclic reclamation.
- One-dimensional primitive and reference arrays, their checked accesses, and
  checked casts within the admitted type system.
- Shared static integer/reference storage and bounded class initialization,
  including owner/waiter ordering and cancellation cleanup.

The heap example enters a static `(I)I` method. Separate scalar entry APIs expose
supported primitive results. This is an explicit admission API; ordinary
unrestricted bytecode dispatch is outside the preview. Unsupported bytecodes and
layouts reject. Runtime faults return Rust errors rather than providing general
Java exception execution. The limited initializer failure behavior and detailed
storage restrictions are documented with
[heap ownership](ristretto_vm_research/CANONICAL_HEAP.md),
[static fields and initialization](ristretto_vm_research/CANONICAL_STATICS.md), and the
[GC ownership contract](ristretto_gc/CANONICAL_RUNTIME.md).

General inheritance/interfaces, virtual/interface/dynamic calls, monitors, native
calls, exception handlers, finalizers, weak/conditional references, and
multidimensional arrays are outside this preview. WASI runs the interpreter;
there is no WASI native-code compilation claim.

Callers choose object/root/mutator/helper/edge capacities and work limits.
`Program::run` uses a dedicated heap. `Program::with_heap` permits multiple calls
on one heap with a separate payload-byte ceiling and final drain limit. Catalog
reservation, managed payloads, program metadata, stacks, executable pages,
allocator overhead and whole-process RSS have different scopes. Bounded polls
count operations; host scheduling, payload preparation/zeroing, allocation and
destruction do not acquire a wall-clock bound from those limits.

A stalled ordinary constructor can block reclamation in the default collector.
This preview does not resolve that ordinary-publication limitation or establish
zero global stalls, complete pause coverage, throughput/tail superiority, or
sustained one-hour/24-hour qualification. Any comparison data must retain its own
source, workload, resource and lifecycle scopes. The
[ring comparison](benchmarks/gc/ring/README.md) links the measured revision's
results, raw summaries and graphs with those limits. Those finite comparisons
do not replace sustained capacity, scheduled-tail or endurance qualification.

## Reproduce the GC package check

Cargo can stage both versioned crates together before a registry release. It
resolves the optional research dependency against the staged catalog package and
verifies each extracted crate. From the workspace root run:

```sh
cargo package --locked -p ristretto_gc_catalog -p ristretto_gc
```

Independent registry publication orders `ristretto_gc_catalog` before
`ristretto_gc` because Cargo resolves optional dependencies when packaging.
The regular JVM dependency check above also covers all launcher features.

## Reproduce source identity and validation

In a Git checkout of the reviewed commit:

```sh
python3 benchmarks/gc/release/source_archive.py HEAD ../preview-source-a
python3 benchmarks/gc/release/source_archive.py HEAD ../preview-source-b
cmp ../preview-source-a/ristretto-gc-preview-*.tar.gz ../preview-source-b/ristretto-gc-preview-*.tar.gz
cmp ../preview-source-a/SHA256SUMS ../preview-source-b/SHA256SUMS
```

The archive comes from tracked bytes and executable modes at that commit. File
permissions are fixed to 0644/0755 independently of the caller's Git tar mask.
Repeat builds must
produce identical compressed bytes, which also binds the tar contents and modes.
CI extracts that archive and runs all three functional tiers with a fresh target
per source tree. Its result receipts are functional evidence, not measurements.
The release's named-suite reconciliation separately records retained, added and
missing cases relative to the earlier frozen archive; historical test totals
cannot be substituted for results from this curated source.
