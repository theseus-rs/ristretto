# Java GC source preview

The native JVM, WASI interpreter, browser engine and GC preview all use
`ristretto_gc::GarbageCollector`. The native compiled tier uses the same JVM
heap and reference barriers as the interpreter. Java instance fields, arrays,
typed roots and soft, weak, final and phantom references all use this collector.
The catalog and pool are available in the historical revision linked below.

## Run the preview

Use the Rust toolchain in `rust-toolchain.toml`, Python 3.11 or later, a native
linker and a JDK 25 installation. Set `JAVA_HOME` to that JDK. The archive
includes both Java classfiles for the graph fixture.

```sh
sha256sum --check SHA256SUMS
tar -xzf ristretto-gc-preview-*.tar.gz
cd ristretto-gc-preview-*/
export CARGO_TARGET_DIR="$PWD/../preview-target"
python3 benchmarks/gc/release/verify_preview.py native-interpreted
python3 benchmarks/gc/release/verify_preview.py native-compiled
rustup target add wasm32-wasip2
python3 benchmarks/gc/release/verify_preview.py wasi-interpreted
```

The compiled preview runs on native x86-64 Linux and explicitly prepares three
methods. The WASI verifier uses Wasmtime 50.0.1 on PATH, or the executable set
in `WASMTIME`, and grants the guest access to the JDK and fixture directories.

Each tier invokes the same Java graph and checks its result of **64**. The
method counters are scoped to that invocation. The interpreter records
25 calls across the entry method, node constructor and checksum method. The
compiled tier must record all 25 calls and eight taken backward branches.
The example also builds a separate two-node cycle through ordinary Java
fields, drops its roots, runs a full collection, and checks that both weak
witnesses clear and the full managed charge returns to its pre-allocation
baseline. For this check the VM stops its background GC worker and drives
collection explicitly. These are functional checks.
[Java fixture](ristretto_vm/tests/fixtures/canonical/CanonicalGraphWorkload.java) ·
[Rust example](ristretto_vm/examples/gc_preview.rs)

## Migrate from the restricted research preview

Source users of the earlier bounded catalog, pool, restricted Java session,
`CanonicalPayload` or `Trace::canonical_payload`
can find the final implementation at commit
[0497123e](https://github.com/theseus-rs/ristretto/tree/0497123e).
Use `ristretto_vm::VM` for Java execution and `Gc` with
`GarbageCollector` for Rust-managed objects. The slot capacities, bounded
poll API and catalog-specific accounting belonged to the retired runtime.
For the current JVM's progress and performance gates see the
[collector plan](ristretto_gc/PLAN.md).

## Verify the release

From a clean checkout, check the workspace and all shipping dependency graphs,
then package the collector.

```sh
python3 benchmarks/gc/release/verify_alpha_heap.py
cargo package --locked -p ristretto_gc
```

CI also builds reproducible source archives, runs each tier from the extracted
archive, and builds normalized Cargo archives from an external consumer.
Both native and WASI consumer graphs must resolve every collector dependency
to the exact unpacked `ristretto_gc` release archive. The verifier checks the
workspace and the unpacked collector for retired catalog and pool sources,
including when a consumer reaches the collector transitively.
[Archive and package instructions](benchmarks/gc/release/README.md)
