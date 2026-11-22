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
JDK on PATH to compile the Java fixtures and execute the cycle oracle.
