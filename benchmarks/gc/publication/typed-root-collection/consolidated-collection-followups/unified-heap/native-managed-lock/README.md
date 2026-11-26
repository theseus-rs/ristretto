# Native managed lock comparison

Replacing the managed payload's native lock with `parking_lot::RwLock` made a
primitive array read take **13.7% longer** in the initial screen and **18.2%
longer** in the repeat. The unified JVM keeps its native managed lock and its
existing allocation-free, typed projections.

| Operation | Initial elapsed ratio, 16 pairs | Repeat elapsed ratio, 24 pairs |
|---|---:|---:|
| Primitive array load | 1.137 [1.093, 1.197] | 1.182 [1.144, 1.217] |
| Primitive array write projection | 1.007 [0.859, 1.035] | 1.048 [1.019, 1.065] |
| Reference array load | 0.985 [0.946, 1.140] | 1.026 [1.012, 1.051] |
| Object field replacement | 1.061 [0.945, 1.119] | 0.997 [0.988, 1.012] |

Each ratio divides candidate elapsed time by the ordinary lock's elapsed time.
Brackets give individual paired bootstrap 95% intervals. Each pair used two
fresh release processes on CPU 2 in randomized order. The initial run used
200,000 operations per process and the repeat used 1,000,000. Both used length
8, with counted publication, compaction and context roots disabled. All 320
processes passed their final contents and reclamation checks. These measure
uncontended managed field/array operations.

To reproduce the candidate, decompress and apply the [lock patch](candidate.patch.gz) to
`1f669afe`. For each build run
`cargo build --locked --release --no-default-features -p ristretto_classloader --example mutation_ownership`
in a separate target directory. Invoke `mutation_ownership MODE N 8 false false false`
for each mode in the [observations](repeat.jsonl.gz). The candidate maps native
guards to the full payload and preserves thread-local guard ownership even when
a dependency enables `parking_lot/send_guard`. The write hook runs before the
parent guard unlocks. [Initial observations](initial.jsonl.gz) ·
[Estimates and executable hashes](ratios.json).
