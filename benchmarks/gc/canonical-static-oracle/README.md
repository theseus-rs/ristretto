# Static initialization semantic oracle

`CanonicalStaticOracle` is a peer-only reflection/error wrapper and is never in
the admitted Program class set. Each row is generated in a fresh HotSpot process,
matching the canonical runner's current distinct Java world per invocation.
The fixtures cover static roots/marks at both 9 and 2,048 elements, reference
rewrites, recursive initialization and trigger/error ordering. They contain no
timing, warmup, measurement API, explicit GC or allocation-count sampling.

From the source root, regenerate with the recorded JDK:

```sh
javac --release 17 -g ristretto_vm/tests/fixtures/canonical/CanonicalStaticRootsSmall.java ristretto_vm/tests/fixtures/canonical/CanonicalStaticRootsFull.java ristretto_vm/tests/fixtures/canonical/CanonicalStaticProtocol.java benchmarks/gc/canonical-static-oracle/CanonicalStaticOracle.java
python3 benchmarks/gc/canonical-static-oracle/oracle.py /path/to/java
```

The script invokes `-Xverify:all` for each case and writes the 16-row TSV used on
native and WASI. The Rust suite separately patches a verified initializer to use
`getstatic SLOTS` in place of javac's inlined literal, testing ConstantValue before
recursive reads for final and non-final static ints; those extra cases are
bytecode correctness tests, not rows claimed to come from this javac oracle.
