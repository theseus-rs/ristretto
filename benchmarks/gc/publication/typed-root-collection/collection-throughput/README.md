# Complete collection throughput

Every measured condition reached a median speedup of at least 3× over the f1daebeb collector in two separate campaigns. Each campaign ran 24 matched process pairs per condition across 24 native and 12 WASI conditions, totaling 3,456 validated processes. The table gives the range of median speedups across both publication policies and diagnostics settings on the final build.

| Execution | Live tree | Live wide graph | Tree with fresh garbage |
|---|---:|---:|---:|
| Native, cooperative | 6.24–6.55× | 5.94–6.27× | 3.78–3.92× |
| Native, two background workers | 4.63–4.89× | 6.05–6.75× | 3.03–3.19× |
| WASI, cooperative | 5.66–6.09× | 5.58–5.99× | 3.77–3.96× |

Each process ran 16 full collections on a 4,095-node graph after three warmups. Each mixed cycle added 1,023 unreachable scalars before timing and waited for their sweep and byte refunds before stopping the clock. Both builds used the same fixture from 47f4a4c6. The driver checked marked and swept objects, reclaimed bytes, completed cycles, graph checksum, worker count and teardown. Construction, validation and teardown are outside the elapsed time.

The initial build recorded minimum median speedups of 3.06× on native and 3.85× on WASI. Its lowest per-condition 95% bootstrap bounds were 3.03× and 3.74×. The final build preallocated the bounded sweep queue before entering collection. Its lowest medians were 3.03× and 3.77× and its lowest bounds were 2.95× and 3.71×. Intervals are exploratory and unadjusted for multiple comparisons. Application throughput, scheduled tail latency, GC-induced pauses and comparisons with G1 or ZGC require their own measurements.

The figures show candidate/control elapsed time per node and cycle. The reciprocal gives the speedup. [Native figure](native.svg) · [WASI figure](wasm.svg)

Each CSV row describes one timed process in randomized order. The provenance records CPU affinity, runner, fixture, toolchain and binary hashes for both campaigns. [Final processes](processes.csv) · [Initial processes](initial.csv) · [Paired medians and intervals](summary.json) · [Build provenance](provenance.json)
