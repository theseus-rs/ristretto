# Retained-ring comparison workload

`CanonicalRingWorkload` is shared, ordinary javac bytecode for the restricted
canonical interpreter, native compiled tier and HotSpot. It maintains a doubly
linked ring of 8, 32 or 128 nodes. Each node owns a variable-length byte array.
Every iteration replaces a node, updates both neighbor links and leaves the removed
node as a self-cycle. Its array remains locally rooted through a checksum call
after unlinking. A final traversal checks both directions and consumes each live
payload. The result depends on all removed and retained generations using Java
integer overflow and signed byte semantics.

The variants use minimum payloads of 64, 1,024 and 4,096 bytes. Object creation is
exactly `2 * (retained + steps)` for nonnegative steps, including one array per
node. The negative-input case allocates no objects. VM correctness cases run with
`4 * retained + 8` object slots and require complete reclamation. The 257-step
cases allocate more objects than the catalog can hold simultaneously, so they
exercise slot reuse while the retained ring remains live. Released object counts
are not collection counts.

The peer oracle stays outside the admitted class set. Its default mode emits
21 finite results. `verify_oracle.py` compares those with an independent Python
model that maintains only a sequence of live generation numbers. The generated
classfiles use `javac --release 17 -g`; shared Java source has no timing calls.

The separate `--allocation-evidence <variant> <steps> <repetitions>` mode warms
execution and measures the current Java thread's actual allocated bytes through
`ThreadMXBean`. Run it on a quiet host and preserve the JVM configuration, source,
classfile and executable identities. Its counter includes peer launcher overhead;
it is an exposure check, not an exact object-layout measurement. A positive
counter alone does not establish that all intended allocations survive compiler
optimization: compare it with the workload's minimum payload-byte demand before
using the workload for performance claims.

No performance, pause, whole-process memory or endurance qualification follows
from the correctness fixture. Future comparisons must include deferred collection
and cleanup costs, match workload and offered load, verify collector exposure and
preserve all samples. This entry currently owns a dedicated heap per invocation;
body-only service diagnostics exclude heap setup and final cleanup and cannot
stand alone as end-to-end throughput measurements.
