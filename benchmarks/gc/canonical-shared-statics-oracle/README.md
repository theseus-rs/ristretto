# Shared-world static semantics

The admitted `CanonicalSharedStatics` fixture is separate from the peer-only
reflection/error wrapper. Run the wrapper once per fresh Java world: its 14 rows
share one class state, matching alternating contexts on one canonical JavaHeap.
The oracle compares result values and error classes. It does not claim Java
Throwable cause-object or stack-trace equivalence.

From the source root, using the recorded JDK:

```sh
javac --release 17 -g ristretto_vm/tests/fixtures/canonical/CanonicalSharedStatics.java benchmarks/gc/canonical-shared-statics-oracle/CanonicalSharedStaticsOracle.java
java -Xverify:all -cp ristretto_vm/tests/fixtures/canonical:benchmarks/gc/canonical-shared-statics-oracle CanonicalSharedStaticsOracle
```

The frozen TSV is embedded in native and WASI checks. Separate bounded scheduler
checks park an initializer and its waiter, advance unrelated Java/GC work, then
exercise success, failure and cancellation wake paths. Native checks additionally
use independent OS threads/executors; WASI checks are cooperative. These are finite
correctness checks, with no benchmark timing, allocation sampling or pause claim.
