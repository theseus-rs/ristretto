# Root storage under scheduled allocation

At `3d882623`, five fresh processes per policy ran the same single-mutator
Java payload fixture for eight seconds. Each process scheduled 24,000
operations, alternating allocation, point writes into an old reference array,
pinning and fragmentation. The only policy selection changed was root storage.
Context registration and use followed the fixture's normal configuration.
Native used four collection workers on two assigned CPUs. WASI used cooperative
collection on one assigned CPU. Both used a 64 MiB noncompacting heap, one
metadata shard and uncounted publication. All 30 processes reclaimed all live
objects and occupied bytes at teardown.

| Target | Root storage | Completed | Operation p99.99 | Scheduled arrival p99.99 |
|---|---|---:|---:|---:|
| Native | Tree | 24,000 | 3,008 µs | 8,320 µs |
| Native | Slots | 24,000 | 2,880 µs | 3,936 µs |
| Native | Contexts | 24,000 | 2,016 µs | 2,912 µs |
| WASI | Tree | 23,999 | 4,480 µs | 5,568 µs |
| WASI | Slots | 23,998 | 4,224 µs | 5,696 µs |
| WASI | Contexts | 24,000 | 4,416 µs | 8,448 µs |

Each entry is a median of five process values. Native context roots reduced
the scheduled tail, while the WASI context median was higher than the tree
median. Each process contributes only about two or three operations in the
upper 0.01%. The 1 ms operation p99.99 target remains open on both targets.
This screen supports retaining root-policy selection pending sustained tests.

Build the locked release `gc_endurance` example in `ristretto_classloader` for
native or `wasm32-wasip2`. Each raw record includes the full ordered argument
list and reported configuration, counters, histograms and backlog. The order
of the three arms was shuffled within each repetition.

[Summary](summary.csv) · [Native processes](native.jsonl.gz) · [WASI processes](wasm.jsonl.gz) · [Artifact identities](builds.json)
