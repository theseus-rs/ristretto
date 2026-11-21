# Closed-layout cast correctness

`CanonicalCastWorkload` and its Node/Other classes are unchanged javac
`--release 17 -g` bytecode. The peer-only reflective launcher remains outside the
canonical Program class set. HotSpot `-Xverify:all` supplies 85 finite rows; the
independent Python scalar/fault model verifies each result.

The cases cover all eight primitive-array cast targets, direct Node casts,
Object[] and Node[] covariance, null, identity, returned/field-unlinked roots and
actual array component preservation after casts. Rejected stores still distinguish
bounds before ArrayStoreException and null before both. ClassCastException paths
cover class, primitive-kind, object-array and closed-array mismatches.

Separate javac classes prove that multidimensional targets, interfaces, missing
classes and instanceof remain admission rejections. A null input does not bypass
closed-target admission. Object upcasts may be omitted by javac; direct ownership
tests additionally cover the broad Object target and hidden/foreign-root rejection.

These cases extend the restricted canonical Java tier. Static roots, class
initialization, final/volatile field semantics, general dispatch and exception
handlers remain separate work; this is not a full GcBench or performance result.

`verify_access.py` clears only ACC_PUBLIC in a separate target class variant,
then verifies 16 HotSpot rows across public/package-private targets and same/other
packages. The canonical tests perform the identical class-flag change and reject
inaccessible cross-package Class constants before execution; they do not execute
unsupported linking errors. The same guard covers new and anewarray as well as
class/array casts. All supplied Program classes share one closed loader namespace.
