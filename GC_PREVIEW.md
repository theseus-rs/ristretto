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
The example also passes two linked VM-owned nodes to Java in the selected
execution tier. Java moves one edge from `next` to `other`. The VM checks both
ends of the resulting cycle against the original weak identities, drops the
roots and runs a full collection. Both witnesses must clear and the managed
charge must return to its pre-allocation baseline. For this check the VM stops its background GC worker and drives
collection explicitly. These are functional checks.
[Java fixture](ristretto_vm/tests/fixtures/canonical/CanonicalGraphWorkload.java) ·
[Rust example](ristretto_vm/examples/gc_preview.rs) ·
[Archive results and broken-edge control](benchmarks/gc/release/shared-heap-cycle.json)

Three fresh Chromium workers exercised Java collections on the production WASIp2 playground engine. Each completed 13 guest collections, reclaimed 10,473,075 managed bytes and returned checksum 799864. The measured component contains the same workspace collector as the JVM. [Browser Java workload, worker recordings and artifact hashes](benchmarks/gc/publication/typed-root-collection/consolidated-collection-followups/protected-raw-java-point-writes/epoch-visible-deletions/browser-jvm/README.md).

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

The workspace check covers 21 graphs, including the release `java` launcher
with default features disabled on bare wasm32, WASIp2 and bare wasm64. Each
release job verifies its own launcher target and Cargo features before packaging.
All three Wasm launchers built using the release workflow’s dry-run profile.
On WASIp2 the released launcher entry point executed eight rounds of the Java
graph fixture and returned 64. The bare wasm32 and wasm64 results cover build
and dependency identity. [Launcher targets, build hashes and fixture](benchmarks/gc/release/launcher-artifacts.json).

CI also builds reproducible source archives, runs each tier from the extracted
archive, and builds normalized Cargo archives from an external consumer.
Both native and WASI consumer graphs must resolve every collector dependency
to the exact unpacked `ristretto_gc` release archive. The verifier checks the
workspace and the unpacked GC and VM crates for the retired catalog, pool and
restricted interpreter. It also checks the public class loader independently on
native and WASI, so a direct VM dependency cannot mask a missing collector in
the crate that owns Java objects and arrays.
[Archive and package instructions](benchmarks/gc/release/README.md)
