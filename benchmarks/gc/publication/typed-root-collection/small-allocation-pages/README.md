# Complete collection with small allocation pages

The allocator packs up to 64 small managed values into a size-class page. Slot return follows payload destruction. The last strong owner releases the backing and its charge against the shared arena limit. The allocation cache holds a weak reference. Contended, unsupported and over-budget requests retain the exact-size global allocation path.

Two comparisons used the same 4,095-node fixture. Each process completed 16 full collections. In the mixed case each collection traced the live tree and reclaimed 1,023 unreachable scalars. The timer included reclamation. Each process checked the live checksum, cycle and object counts, refunded bytes and zero managed storage at teardown.

| Mixed heap | Speedup over the immediate parent | Speedup over f1daebeb |
|---|---:|---:|
| Native, cooperative | 1.31–1.33× | 4.96–5.11× |
| Native, two workers | 1.16–1.18× | 3.57–3.77× |
| WASI, cooperative | 1.28–1.31× | 5.01–5.20× |

Ranges span paired medians for both publication policies and diagnostics settings. The incremental comparison used 12 shuffled process pairs per condition on the published parent and candidate. The historical comparison used 24 pairs per condition and held the fixture, compiler, Cargo manifests and lockfile constant. Collector source hashes were checked against the published revisions. Both comparisons cover tree, wide and mixed heaps.

Across the historical comparison, 30 of 36 medians reached 5×. The weakest native median was 3.57× with a 3.47× lower endpoint of the paired 95% interval. On WASI the weakest median was 5.01× and the weakest lower endpoint was 4.94×. The plots show the historical comparison. [Native graph](native.svg) · [WASI graph](wasm.svg) · [Historical process results](processes.csv) · [Incremental process results](incremental-processes.csv) · [Per-condition intervals](summary.json) · [Builds, schedules and host](provenance.json)
