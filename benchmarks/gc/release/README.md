# Build a GC preview source archive

From a Git checkout with Python 3.11 or newer, name the commit to release and a
new output directory. The tool exports the tracked tree with Git file modes and
a fixed gzip header. It writes the source archive and its SHA-256 side by side.

```sh
python3 benchmarks/gc/release/source_archive.py HEAD /tmp/gc-preview-a
python3 benchmarks/gc/release/source_archive.py HEAD /tmp/gc-preview-b
diff -qr /tmp/gc-preview-a /tmp/gc-preview-b
(cd /tmp/gc-preview-a && sha256sum -c SHA256SUMS)
```

Extract into an empty directory. Run
`ristretto_vm/tests/fixtures/canonical/check_graph_fixture.sh` with a Java 17
JDK on PATH to regenerate the checked classfiles and execute the cycle oracle.


## Check coordinated packaged features

The external consumer checker packages the publishable workspace members and extracts the actual
`.crate` files for each selected crate and its workspace dependencies. It patches
those normalized archives together at their package versions, then builds a
separate consumer with default features disabled and the requested features
explicitly enabled. A bare package name requests its base library with default
features disabled. Each external build uses the normalized archives from the
coordinated release.

```sh
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR="$PWD/../preview-package-target"
python3 .github/scripts/check-packaged-features.py \
  ristretto_gc \
  ristretto_vm/execution-profile ristretto_vm/experimental-reference-dispatch \
  ristretto_jit/experimental-reference-dispatch
rustup target add wasm32-wasip2
python3 .github/scripts/check-packaged-features.py --target wasm32-wasip2 \
  ristretto_vm/execution-profile
```

The native consumer builds the VM's reference-dispatch feature. The WASI
consumer depends only on the VM and reaches the collector transitively.
Both check the extracted GC and VM packages for retired heap and interpreter sources. The
[source preview](../../../GC_PREVIEW.md) exercises both execution tiers.
