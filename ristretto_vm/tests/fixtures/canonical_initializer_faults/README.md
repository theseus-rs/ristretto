# Initializer fault oracle

`sh check.sh` uses JDK 17+ to reproduce every checked-in classfile and the
16-row HotSpot oracle (`-Xverify:all`). Only `CanonicalInitializerFaults` and
its nested classes are admitted; the reflection-based oracle stays outside the
Program. Each class fails once in its initializer and then remains erroneous.
Nested initialization verifies that an existing Java Error is not wrapped again.
Embedding instruction limits and cancellation are separate Rust tests and have
no claimed Java interruption or OutOfMemoryError semantics.
