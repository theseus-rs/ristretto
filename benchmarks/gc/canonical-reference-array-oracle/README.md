# Reference-array correctness fixture

`CanonicalReferenceArrayWorkload` is unchanged javac `--release 17 -g` bytecode.
Its closed admitted set contains that class plus Node, Other and Holder. The peer
oracle launcher stays outside that set. HotSpot `-Xverify:all` emits 34 finite TSV
rows; `verify_oracle.py` independently reconstructs the checksums/fault classes.

`nodes(2048)` allocates a real 2,048-element Node array, fills and reads every
position, and gives every node a reference back to the array. It checks alias
identity after a covariant call and reads the last element after clearing its
array edge. `mixed(2048)` stores primitive arrays and ordinary objects in Object[],
checks every index, and creates an array self-cycle. `returned` exercises array
roots across constructor, field clearing, calls, returns, and an Object[] element.
Boundary lengths 0, 1, 8, 9 and 65 cross internal tree levels. Fault rows distinguish
negative length, null, bounds, and covariant-store failure, including invalid index
and wrong value together and a null array with an invalid index.

The VM's eight-way dense representation uses one Java header and 292 internal
pages for length 2,048. All tree and element edges are catalog-owned; pages are
reference-free metadata and never legal Java values. These representation objects
are physical allocations and must be included in heap/catalog resource reporting.
The typed array constructor and element operations remain incremental/cooperative;
this does not establish a wall-time, pause or throughput bound.

Separate valid javac classes in `canonical_rejected` exercise array components
that would introduce multidimensional descriptors and the multianewarray opcode.
They remain admission-rejection fixtures, never part of the admitted class set.
These finite checks do not admit the full GcBench or qualify its workload gates.
