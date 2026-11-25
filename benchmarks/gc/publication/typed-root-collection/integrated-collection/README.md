# Complete collection after typed-root transfer

All 36 conditions cleared the 3× collection target against the f1daebeb control. Each condition used 24 independently launched, randomized process pairs. The table gives the smallest paired median speedup on each target and the smallest lower bound among its per-condition 95% bootstrap intervals. Intervals are unadjusted for multiple comparisons.

| Target | Conditions | Measured processes | Smallest median speedup | Smallest lower bound |
|---|---:|---:|---:|---:|
| Native, cooperative or two workers | 24 | 1,152 | 3.28× | 3.11× |
| WASI, cooperative | 12 | 576 | 3.88× | 3.83× |

Each process timed 16 complete collections over a 4,095-node tree, wide graph, or tree with 1,023 new garbage allocations per cycle. Garbage creation and warmup were outside the timed region. The fixture checked the live checksum and returned byte budget for reclaimed garbage. Both publication policies ran with diagnostics on and off. Figures show elapsed time per node per cycle. [Native](native.svg) · [WASI](wasm.svg) · [Process measurements](processes.csv) · [Paired statistics](summary.json)

The merge at 0abf26fb contains the exact tested collector source. Both arms used identical workspace manifests, fixture, profile, toolchain and resolved collector dependencies. The manifest patch records the differences between the public merge and the integrated workspace used to build both arms. The before arm replaced only the collector source with f1daebeb. The stored source hashes cover each change, including the candidate's new typed-attempt module. [Build provenance](provenance.json) · [Shared manifest differences](shared-manifests.patch)

## Whole-JVM screen

A separate screen ran the integrated Java VM, the WASI VM, and HotSpot with G1 and ZGC. It used two pinned CPUs, a configured 64 MiB heap, one second of warmup, three seconds of measurement and three forks per condition. All 60 processes completed. Median saturation throughput follows.

| Workload | Ristretto interpreter | Ristretto JIT enabled | Ristretto WASI | G1 | ZGC |
|---|---:|---:|---:|---:|---:|
| Allocation, ops/s | 92,201 | 89,046 | 45,150 | 18,391,369 | 20,289,724 |
| Roots, ops/s | 8,058 | 7,781 | 7,288 | 12,469,745 | 14,273,794 |

At 1,000 scheduled allocation operations/s, all runtimes sustained the offered rate. Median arrival p99.9 was 43 µs for the native Ristretto interpreter, 3 µs for G1 and 3,232 µs for ZGC. The short runs had too few completions at that rate to support an extreme-tail claim. The v2 qualifier rejected this screen. It used CPU affinity without enforced CPU and RSS ceilings, three forks and no predeclared acceptance policy. It leaves the sustained-load and zero-pause release gates open. [Detailed comparison](jvm/REPORT.md) · [Gate results](jvm/gates-v2.json)

[Native throughput](jvm/native-throughput.svg) · [Native latency](jvm/native-latency.svg) · [WASI throughput](jvm/wasm-throughput.svg) · [WASI latency](jvm/wasm-latency.svg) · [Process measurements](jvm/processes.csv) · [Grouped measurements](jvm/summary.json) · [JVM build and run provenance](jvm/provenance.json)
