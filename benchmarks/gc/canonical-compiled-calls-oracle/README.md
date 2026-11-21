# Native call fixture oracle

`CanonicalCompiledCalls.java` is shared by interpreter recursion and native call
tests. The oracle driver is separate because main, strings and console output are
not admitted runtime capabilities.

```sh
javac --release 17 -g -d /tmp/canonical-call-classes \
  ristretto_vm/tests/fixtures/canonical/CanonicalCompiledCalls.java \
  benchmarks/gc/canonical-compiled-calls-oracle/CanonicalCompiledCallsOracle.java
java -Xverify:all -cp /tmp/canonical-call-classes CanonicalCompiledCallsOracle
```

Compare stdout byte-for-byte with
`ristretto_vm/tests/fixtures/canonical/compiled-calls-oracle.tsv`. The ten rows are
semantic checks, not timing or capacity evidence.
