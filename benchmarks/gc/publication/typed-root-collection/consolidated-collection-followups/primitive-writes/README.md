# Finishing writes to Java primitive arrays

At `3484c575`, an ordinary managed write guard can finish a Java primitive
array write under its existing exclusive lock without opening a temporary
root scope. All eight primitive array variants have a no-op demotion hook and
no graph notification. HeapBuffer invalidates relocation copies on mutable
access. The guard still revokes trace coverage and releases its payload pin.
Object and reference-array writes retain their demotion and publication paths.

## Mutator cost

Median elapsed nanoseconds per operation on one native CPU, using the
baseline settings of the `mutation_ownership` fixture. Each row has 24 fresh
process pairs, 10,000 warmup and 100,000 timed operations per process.
The last column reports the paired median candidate/parent ratio and its
individual bootstrap 95% interval.

| Operation | Parent ns | Candidate ns | Ratio [95% interval] |
|---|---:|---:|---:|
| Primitive array, whole writer | 34.00 | 22.56 | 0.657 [0.643, 0.675] |
| Primitive array, typed slice | 32.57 | 20.80 | 0.624 [0.614, 0.644] |
| Primitive array, byte slice | 32.59 | 20.74 | 0.632 [0.620, 0.650] |
| Primitive load | 20.30 | 20.78 | 1.009 [0.969, 1.052] |
| Reference field store | 194.55 | 194.46 | 1.005 [0.984, 1.029] |

The compact heap with counted publication and context roots also reduced all
three primitive write times. Both controls included parity in each configuration.
All 480 processes checked the contents and zero remaining managed bytes and
objects. Each runtime used the same fixture, CPU and operation count. JVM
application throughput and scheduled latency require their own comparison.

Build `cargo build --locked --release -p ristretto_classloader --example
mutation_ownership` in separate target directories at `7acfad10` and
`3484c575`. A stock fixture invocation is `mutation_ownership primitive-whole
100000 64 false false false`. Replace the last three arguments with `true`
for the compact/counted/context arm. All mode names and measurements are in
the archived records.

[Mutation intervals](mutation.csv) · [Per-process measurements](mutations.jsonl.gz)

## Collection and snapshot safety

At `3d882623`, every one of the 36 fixed complete-collection comparisons
cleared 10× against `f1daebeb`. The weakest individual speedup lower bounds
were **10.30× native** and **12.21× WASI**. Each condition used 24 independent
process pairs, 4,095 nodes and 16 timed cycles. All 1,728 processes passed
their semantic checks. The harness measures every collection phase and its
waits under the selected GC policies. It excludes allocation and Java execution.

The forced snapshot test starts full marking with the outgoing child still
white, drops the last edge under a managed writer, checks the child survives
the active ticket and observes its reclamation in the next collection. All
100 fresh native processes exercised both publication policies. The same test
passed on WASI and under Miri. Classloader tests also switch between reference
and primitive array variants and check relocation dirtiness during primitive
writes.

[Collection intervals](collection.csv) · [Native records](native.jsonl.gz) · [WASI records](wasm.jsonl.gz) · [Build identities and settings](provenance.json) · [Snapshot trials](white-edge-processes.json)
