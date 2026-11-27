# Complete collection after root deletion and allocation refund changes

All 36 paired median speedups cleared 3× against f1daebeb. This fourth campaign includes typed and paged root removal, independent allocator refund delivery and physical retirement accounting. It used 24 randomized fresh process pairs per condition, seed 9167, and the same 4,095-node fixture as the earlier campaigns.

| Target | Conditions | Fresh processes | Smallest paired median speedup | 95% interval in that condition |
|---|---:|---:|---:|---:|
| Native | 24 | 1,152 | 3.043× | 2.978–3.111× |
| WASI | 12 | 576 | 3.861× | 3.751–3.899× |

The native minimum occurred on the mixed workload with two workers, gated publication and diagnostics disabled. Its interval includes results below 3×. The WASI minimum occurred on the cooperative mixed workload with gated publication and diagnostics enabled. Intervals use paired process bootstraps and are unadjusted for multiple comparisons. The preceding integrated campaign measured minima of 3.282× native and 3.876× WASI. [Prior campaign](../integrated-collection/README.md)

Each process timed 16 full collections including reclamation. The fixture verified the live graph checksum, exact swept count and refunded bytes for each process. The mixed workload created 1,023 new garbage nodes per cycle before starting its timer. Native ran on CPUs 4, 6 and 8 with zero or two workers. WASI ran on CPU 10 with cooperative steps. [Native graph](native.svg) · [WASI graph](wasm.svg) · [All process measurements](processes.csv) · [Paired estimates](summary.json)

The collector source at 0d4b3f43 and control at f1daebeb match the source hashes from the measured binaries. Both builds used the same fixture, manifest adjustments, dependencies and profile. The manifest patch applies to 0d4b3f43 with `git apply --unidiff-zero`. Collector benchmark time covers the collection and reclamation work. The separate equal-budget VM screen measures application throughput and latency with G1 and ZGC. [Build provenance](provenance.json) · [Shared manifests](shared-manifests.patch) · [Equal-budget VM screen](../integrated-collection/equal-budget/REPORT.md)
