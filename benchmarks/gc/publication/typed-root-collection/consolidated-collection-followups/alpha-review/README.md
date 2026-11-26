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
