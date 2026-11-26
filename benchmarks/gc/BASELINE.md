# Initial GC comparison screen — 2026-11-07

128 processes completed: 96 native and 32 WASI. All workload checksums and retained-content audits passed; both recorded schedules completed and input-integrity checks passed. This does not establish GC correctness for other workloads or complete performance qualification.

Each process used one second of warmup and two seconds of measurement, with two independent processes per configuration. The workload ran at saturation and at 10,000 offered operations/second. Configured heap: 64 MiB; CPU affinity: Linux CPUs 0 and 2. Native Ristretto used two GC workers; WASI used cooperative execution. No per-process cgroup physical-memory cap was imposed.

OpenJDK Corretto 25.0.5, Rust 1.99.1 and Wasmtime 50.0.1 were used on a shared development host. Directory dates use the local America/Denver date; metadata records UTC on November 8. Full source archives, bytecode, binaries, versions, flags and hashes are in the evidence below.

## Saturation observations

These values are medians of per-process completed operations/second, rounded to whole operations. A root-churn operation allocates and traverses 16 nodes; an array operation allocates one escaping array. The table measures the entire runtime. Saturation is not a sustained-capacity result.

| Target / runtime | Array allocation ops/s | Root churn ops/s | Mixed arrays ops/s |
| --- | ---: | ---: | ---: |
| g1 | 25,383,360 | 13,848,914 | 9,869,189 |
| g1-int | 3,935,279 | 1,377,826 | 2,973,677 |
| ristretto-baseline | 148,440 | 10,099 | 135,447 |
| ristretto-candidate | 129,081 | 9,062 | 134,183 |
| zgc | 30,581,409 | 14,616,621 | 12,679,544 |
| zgc-int | 3,847,731 | 1,350,154 | 3,037,883 |
| wasm-baseline | 72,748 | 7,330 | 62,143 |
| wasm-candidate | 61,698 | 5,353 | 52,054 |

HotSpot uses its default JIT except for `-int` rows. Ristretto uses its interpreter in all rows. The observed gap cannot be attributed solely to collector algorithms. The combined experimental policy does not improve throughput consistently; its default-enablement gates remain unmet.

## Fixed-rate overload

At 10,000 offered root-churn operations/second, both Ristretto native policies and both WASI policies left unfinished requests. Their completed-request latency distributions are censored and cannot pass a latency gate. The median unfinished counts were:

| Runtime | Unfinished requests out of 20,000 offered | Arrival p99.9, median of run quantiles (µs) |
| --- | ---: | ---: |
| ristretto-baseline | 963 | 119,808 |
| ristretto-candidate | 815 | 125,440 |
| wasm-baseline | 4,925 | 514,048 |
| wasm-candidate | 8,731 | 892,928 |

The native G1/ZGC rows completed all requests at this offered rate in these short runs. That is not proof of their long-run capacity. Lower-load tail measurements, more independent forks, and pressure/contended-reclamation fixtures are required before any winner or release decision.

## Reproducible evidence and graphs

- [Native report](../../target/gc-comparison-2026-11-07/REPORT.md), [raw measurements](../../target/gc-comparison-2026-11-07/raw.jsonl), [provenance](../../target/gc-comparison-2026-11-07/metadata.json).
- [WASI report](../../target/gc-comparison-wasi-2026-11-07/REPORT.md), [raw measurements](../../target/gc-comparison-wasi-2026-11-07/raw.jsonl), [provenance](../../target/gc-comparison-wasi-2026-11-07/metadata.json).
- Native: [throughput](../../target/gc-comparison-2026-11-07/graphs/native-throughput.svg), [p99.9](../../target/gc-comparison-2026-11-07/graphs/native-latency.svg), [p99.99](../../target/gc-comparison-2026-11-07/graphs/native-latency-p9999.svg), [memory](../../target/gc-comparison-2026-11-07/graphs/native-memory.svg), [backlog/tradeoffs](../../target/gc-comparison-2026-11-07/graphs/native-load.svg).
- WASI: [throughput](../../target/gc-comparison-wasi-2026-11-07/graphs/wasm-throughput.svg), [p99.9](../../target/gc-comparison-wasi-2026-11-07/graphs/wasm-latency.svg), [p99.99](../../target/gc-comparison-wasi-2026-11-07/graphs/wasm-latency-p9999.svg), [memory](../../target/gc-comparison-wasi-2026-11-07/graphs/wasm-memory.svg), [backlog/tradeoffs](../../target/gc-comparison-wasi-2026-11-07/graphs/wasm-load.svg).

PNG versions accompany all SVG figures. Graph provenance separately records the renderer and input hashes. Evidence is local under ignored `target/`; archive those directories before cleaning the workspace. The frozen workload/VM binaries were unchanged during timing. Subsequent renderer layout and launcher environment-sanitization edits did not change these workloads or measurements.

## Next experiments

Use [the plan](../../ristretto_gc/PLAN.md#next-delivery-sequence-overhead-progress-and-external-qualification) to isolate allocation/root/barrier costs, test progress under forced contention, and qualify individual features before combining them. Add per-window GC exposure, recorder-overhead measurements, enforced RSS/CPU budgets, calibrated capacity search and independent-run confidence intervals. Native and Wasm need separate acceptance envelopes. One-hour and 24-hour qualification has not been performed by this screen.
