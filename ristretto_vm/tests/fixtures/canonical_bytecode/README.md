# Narrow-return fixture

Compile `CanonicalNarrowReturnWorkload.java` with `javac --release 17 -g` into a
temporary directory. Run
`benchmarks/gc/canonical-array-oracle/patch_narrow_return_fixture.py` with that
original classfile and this directory's classfile as separate input/output paths.
The script replaces only the four helper `Code` attributes with `iload_0; ireturn`,
keeping their `(I)B`, `(I)C`, `(I)S`, and `(I)Z` descriptors. Caller bodies remain
ordinary javac output. This is intentional valid-bytecode coverage beyond the
explicit casts emitted by javac; the checked-in classfile is not unmodified
javac output.

`CanonicalNarrowReturnOracle.java` is a separate HotSpot peer driver. With the
patched classfile on its classpath, `java -Xverify:all` verifies all classes and
checks 40 cases against independent Java casts or boolean low-bit expectations.
Never include the peer driver in the canonical Program's class set.

The JVM requires this conversion at return, as described in
[JVMS ireturn](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.ireturn).
