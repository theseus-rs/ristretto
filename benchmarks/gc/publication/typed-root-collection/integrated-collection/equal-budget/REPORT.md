# GC workload comparison

Exploratory end-to-end runtime measurements, not isolated collector rankings or completed qualification.
HotSpot uses its default JIT unless the runtime name ends in `-int`; native Ristretto names ending in `-jit` enable its JIT, other Ristretto rows interpret. Wasm is a separate target.
Heap limits constrain different layouts; process peak RSS and CPU include startup, warmup and result output.
Rate 0 is a saturation screen, not proof of sustainable capacity. Positive rates include scheduled-arrival delay; unfinished requests remain visible.
Values below are medians across processes; raw histograms and per-run ranges are retained. Low sample counts do not establish extreme-tail behavior.

| Runtime | Workload | Offered ops/s | Runs | Completed ops/s | Service p99.9 µs | Arrival p99.9 µs | Arrival p99.99 µs | Unfinished | RSS MiB | Tail N ≥ 1M/run | Runs observing GC* |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---|---:|
| g1 | allocation | 0 | 5 | 16,730,821.0 | 1.0 | 1.0 | 2.0 | 0.0 | 80.7 | yes | 5 |
| g1 | control | 0 | 5 | 29,697,713.2 | 1.0 | 1.0 | 1.0 | 0.0 | 40.8 | yes | 0 |
| g1 | mixed | 0 | 5 | 8,249,821.9 | 2.0 | 2.0 | 66.0 | 0.0 | 94.0 | yes | 5 |
| g1 | roots | 0 | 5 | 14,211,809.5 | 1.0 | 1.0 | 5.0 | 0.0 | 81.5 | yes | 5 |
| g1-int | allocation | 0 | 5 | 3,314,297.8 | 1.0 | 1.0 | 4.0 | 0.0 | 75.3 | yes | 5 |
| g1-int | control | 0 | 5 | 4,346,392.0 | 1.0 | 1.0 | 1.0 | 0.0 | 35.9 | yes | 0 |
| g1-int | mixed | 0 | 5 | 2,132,613.0 | 2.0 | 2.0 | 71.0 | 0.0 | 88.1 | yes | 5 |
| g1-int | roots | 0 | 5 | 1,312,180.3 | 3.0 | 3.0 | 14.0 | 0.0 | 74.8 | yes | 5 |
| ristretto-candidate | allocation | 0 | 5 | 97,266.9 | 110.0 | 110.0 | 210.0 | 0.0 | 224.4 | no | 5 |
| ristretto-candidate | control | 0 | 5 | 399,805.0 | 2.0 | 2.0 | 4.0 | 0.0 | 65.2 | no | 5 |
| ristretto-candidate | mixed | 0 | 5 | 58,899.7 | 113.0 | 113.0 | 11,520.0 | 0.0 | 163.8 | no | 5 |
| ristretto-candidate | roots | 0 | 5 | 7,805.1 | 1,712.0 | 1,712.0 | 35,328.0 | 0.0 | 332.7 | no | 5 |
| ristretto-candidate-jit | allocation | 0 | 5 | 87,343.3 | 111.0 | 111.0 | 210.0 | 0.0 | 229.7 | no | 5 |
| ristretto-candidate-jit | control | 0 | 5 | 316,904.3 | 2.0 | 2.0 | 5.0 | 0.0 | 69.8 | no | 5 |
| ristretto-candidate-jit | mixed | 0 | 5 | 43,520.9 | 56.0 | 56.0 | 15,872.0 | 0.0 | 167.2 | no | 5 |
| ristretto-candidate-jit | roots | 0 | 5 | 7,401.7 | 2,208.0 | 2,208.0 | 36,352.0 | 0.0 | 332.0 | no | 5 |
| wasm-candidate | allocation | 0 | 5 | 62,716.4 | 113.0 | 113.0 | 213.0 | 0.0 | 94.5 | no | 5 |
| wasm-candidate | control | 0 | 5 | 131,571.9 | 5.0 | 5.0 | 10.0 | 0.0 | 74.4 | no | 5 |
| wasm-candidate | mixed | 0 | 5 | 38,763.3 | 129.0 | 129.0 | 443.0 | 0.0 | 97.6 | no | 5 |
| wasm-candidate | roots | 0 | 5 | 7,243.5 | 1,808.0 | 1,808.0 | 1,936.0 | 0.0 | 249.6 | no | 5 |
| zgc | allocation | 0 | 5 | 25,741,739.2 | 1.0 | 1.0 | 3.0 | 0.0 | 107.3 | yes | 5 |
| zgc | control | 0 | 5 | 31,938,638.4 | 1.0 | 1.0 | 1.0 | 0.0 | 42.0 | yes | 0 |
| zgc | mixed | 0 | 5 | 11,196,263.0 | 2.0 | 2.0 | 39.0 | 0.0 | 107.9 | yes | 5 |
| zgc | roots | 0 | 5 | 13,969,653.2 | 1.0 | 1.0 | 5.0 | 0.0 | 106.2 | yes | 5 |
| zgc-int | allocation | 0 | 5 | 3,286,899.3 | 1.0 | 1.0 | 3.0 | 0.0 | 98.9 | yes | 5 |
| zgc-int | control | 0 | 5 | 3,763,846.4 | 1.0 | 1.0 | 1.0 | 0.0 | 36.6 | yes | 0 |
| zgc-int | mixed | 0 | 5 | 2,062,837.1 | 3.0 | 3.0 | 76.0 | 0.0 | 101.0 | yes | 5 |
| zgc-int | roots | 0 | 5 | 1,249,344.3 | 4.0 | 4.0 | 22.0 | 0.0 | 99.3 | yes | 5 |

*GC observation covers the whole process, including bootstrap/warmup/output; it does not establish measured-window collection exposure. Missing activity cannot support collector-performance claims.
Failed/invalid/timed-out runs: 0. Input integrity: True. Schedule complete: True.
No winner or release gate is inferred from this report. Inspect failures, backlog, CPU/RSS limits, repetitions and longer runs before making a claim.
