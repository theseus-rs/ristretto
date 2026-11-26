# Collection and allocation layout after the alpha review

The collector at `8e277a39` met the 10× full-cycle gate in all 36 fixture
conditions against the frozen `f1daebeb` binaries. The minimum lower endpoint
of the paired 95% bootstrap intervals was **10.78× on native** and **10.62× on
WASI**. Each condition used 24 randomized pairs of fresh processes, 4,095 nodes
and 16 timed cycles. Every process passed the harness's mark and reclamation
checks. The fixture selects slots, 16 metadata shards, independent marking and
work stealing in both arms. Stock launcher settings need a separate measurement.

Workspace membership changed when the catalog moved. Isolated rebuilds
reproduced both frozen baseline binary hashes exactly. The compiler-reported
dependency targets, versions, profiles and features matched within both target
pairs. The runner also compares the registry lock entries, configuration files,
toolchain, workspace settings and selected fixture.

[36 conditions and intervals](collection.csv) · [Native process records](native.jsonl.gz) · [WASI process records](wasm.jsonl.gz) · [Build identities](provenance.json)

## Descriptor layout

Rust 1.100.0 reported these x86-64 sizes at `af35ecf7` and `8e277a39`
using `cargo rustc -p ristretto_gc --lib -- -Zprint-type-sizes` with the
compiler's unstable diagnostics enabled.

| Type | Before | After |
|---|---:|---:|
| ObjectMetadata | 232 bytes | 208 bytes |
| Reference-counted descriptor entry | 272 bytes | 248 bytes |

The 24 bytes include grouping the small state fields, sharing the zero-sized
allocation adjustment by type, and replacing the destructor mutex with an
exclusively claimed payload slot. A retained descriptor can outlive payload
destruction. The charge is released after backing return. Per-object RSS and
total allocation throughput require separate measurements.

## Returning a stale descriptor under contention

The native regression retains the descriptor of a collected object, verifies
zero charged managed bytes, and parks the allocation registry before releasing
the descriptor on another thread. The release must complete within 250 ms.
On top of `2e9795b8`, the ordinary allocator passed 16 of 16 fresh processes.
An experiment that placed payload and descriptor in one regional reservation
passed 0 of 16. Returning the last descriptor tried to lock the parked registry.
The process order was shuffled. This tests a particular parked-owner schedule.

Run the test with
`cargo test --locked -p ristretto_gc --lib retained_descriptor_release_finishes_with_its_allocation_registry_parked`.
To reproduce the rejected implementation, apply the decompressed patch to
`2e9795b8` with the regression test present. Its changes are research evidence.

[Trial order and outcomes](paired-region-outcomes.json) · [Regional experiment patch](paired-region-experiment.patch.gz)
