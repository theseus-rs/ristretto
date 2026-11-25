# GC workload comparison

Exploratory end-to-end runtime measurements, not isolated collector rankings or completed qualification.
HotSpot uses its default JIT unless the runtime name ends in `-int`; native Ristretto names ending in `-jit` enable its JIT, other Ristretto rows interpret. Wasm is a separate target.
Heap limits constrain different layouts; process peak RSS and CPU include startup, warmup and result output.
Rate 0 is a saturation screen, not proof of sustainable capacity. Positive rates include scheduled-arrival delay; unfinished requests remain visible.
Values below are medians across processes; raw histograms and per-run ranges are retained. Low sample counts do not establish extreme-tail behavior.

| Runtime | Workload | Offered ops/s | Runs | Completed ops/s | Service p99.9 µs | Arrival p99.9 µs | Arrival p99.99 µs | Unfinished | RSS MiB | Tail N ≥ 1M/run | Runs observing GC* |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---|---:|
| g1 | allocation | 0 | 3 | 18,391,369.2 | 1.0 | 1.0 | 1.0 | 0.0 | 80.0 | yes | 3 |
| g1 | allocation | 1000 | 3 | 1,000.0 | 1.0 | 3.0 | 22.0 | 0.0 | 80.0 | no | 3 |
| g1 | roots | 0 | 3 | 12,469,744.5 | 1.0 | 1.0 | 4.0 | 0.0 | 81.5 | yes | 3 |
| g1 | roots | 1000 | 3 | 1,000.0 | 6.0 | 9.0 | 275.0 | 0.0 | 81.1 | no | 3 |
| ristretto-candidate | allocation | 0 | 3 | 92,200.6 | 111.0 | 111.0 | 126.0 | 0.0 | 224.8 | no | 3 |
| ristretto-candidate | allocation | 1000 | 3 | 1,000.0 | 39.0 | 43.0 | 59.0 | 0.0 | 198.4 | no | 3 |
| ristretto-candidate | roots | 0 | 3 | 8,057.9 | 1,808.0 | 1,808.0 | 34,816.0 | 0.0 | 333.9 | no | 3 |
| ristretto-candidate | roots | 1000 | 3 | 1,000.0 | 195.0 | 412.0 | 794.0 | 0.0 | 299.4 | no | 3 |
| ristretto-candidate-jit | allocation | 0 | 3 | 89,045.7 | 110.0 | 110.0 | 124.0 | 0.0 | 227.8 | no | 3 |
| ristretto-candidate-jit | allocation | 1000 | 3 | 1,000.0 | 42.0 | 3,136.0 | 3,584.0 | 0.0 | 198.9 | no | 3 |
| ristretto-candidate-jit | roots | 0 | 3 | 7,780.8 | 1,776.0 | 1,776.0 | 35,840.0 | 0.0 | 334.8 | no | 3 |
| ristretto-candidate-jit | roots | 1000 | 3 | 1,000.0 | 170.0 | 406.0 | 644.0 | 0.0 | 303.2 | no | 3 |
| wasm-candidate | allocation | 0 | 3 | 45,149.5 | 125.0 | 125.0 | 227.0 | 0.0 | 100.2 | no | 3 |
| wasm-candidate | allocation | 1000 | 3 | 1,000.0 | 35.0 | 45.0 | 61.0 | 0.0 | 96.9 | no | 3 |
| wasm-candidate | roots | 0 | 3 | 7,288.3 | 1,808.0 | 1,808.0 | 1,952.0 | 0.0 | 251.1 | no | 3 |
| wasm-candidate | roots | 1000 | 3 | 1,000.0 | 1,808.0 | 69,632.0 | 69,632.0 | 0.0 | 215.6 | no | 3 |
| zgc | allocation | 0 | 3 | 20,289,724.4 | 1.0 | 1.0 | 7.0 | 0.0 | 107.0 | yes | 3 |
| zgc | allocation | 1000 | 3 | 1,000.0 | 8.0 | 3,232.0 | 3,872.0 | 0.0 | 106.8 | no | 3 |
| zgc | roots | 0 | 3 | 14,273,794.2 | 1.0 | 1.0 | 4.0 | 0.0 | 103.7 | yes | 3 |
| zgc | roots | 1000 | 3 | 1,000.0 | 9.0 | 955.0 | 3,328.0 | 0.0 | 101.7 | no | 3 |

*GC observation covers the whole process, including bootstrap/warmup/output; it does not establish measured-window collection exposure. Missing activity cannot support collector-performance claims.
Failed/invalid/timed-out runs: 0. Input integrity: True. Schedule complete: True.
No winner or release gate is inferred from this report. Inspect failures, backlog, CPU/RSS limits, repetitions and longer runs before making a claim.
