# Wide scalar call oracle

Use a Java 17 JDK to regenerate the compiled wrapper and its observed results.
The wrapper calls the checked wide scalar fixture from the interpreter reviews.

```sh
fixtures=ristretto_vm/tests/fixtures/canonical
out=$(mktemp -d)
javac --release 17 -g -d "$out" "$fixtures/CanonicalWideScalar.java" "$fixtures/CanonicalWideCalls.java" benchmarks/gc/canonical-scalar-call-oracle/WideCallsOracle.java
cmp "$out/CanonicalWideCalls.class" "$fixtures/CanonicalWideCalls.class"
java -Xverify:all -cp "$out" WideCallsOracle | diff - "$fixtures/wide-calls-oracle.tsv"
```
