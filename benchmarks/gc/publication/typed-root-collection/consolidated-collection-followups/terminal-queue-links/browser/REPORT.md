# Browser collector screen

Fixed scheduled arrivals; completed-request tails are censored when requests remain unfinished. These are actual Chromium main-thread measurements of the Rust collector, not a browser JVM or a ZGC/G1 comparison. Wasm linear memory does not shrink when objects are reclaimed. Qualification remains false. Request-turn work includes all polls and the GC step in turns with at least one request poll; maintenance-only and idle turns remain in the separate raw all-turn histogram. Clock resolution is recorded per fork and limits interpretation of small durations.

| Policy | Fork | Completed/offered | Arrival p99.9 µs | Request-turn work p99.9 µs | Heartbeat max µs | Peak cgroup memory MiB | Unfinished |
|---|---:|---:|---:|---:|---:|---:|---:|
| baseline | 0 | 4000/4000 | 4928 | 51 | 7020.0 | 399.14 | 0 |
| control | 0 | 4000/4000 | 5152 | 20 | 6385.0 | 395.89 | 0 |
| candidate | 0 | 4000/4000 | 5216 | 46 | 6575.0 | 393.46 | 0 |
| control | 1 | 4000/4000 | 5056 | 15 | 6645.0 | 395.16 | 0 |
| candidate | 1 | 4000/4000 | 5280 | 46 | 5680.0 | 397.21 | 0 |
| baseline | 1 | 4000/4000 | 4736 | 46 | 7005.0 | 398.21 | 0 |
| candidate | 2 | 4000/4000 | 4640 | 51 | 6010.0 | 398.20 | 0 |
| baseline | 2 | 3992/4000 | 5088 | 40 | 6355.0 | 398.66 | 8 |
| control | 2 | 4000/4000 | 4864 | 16 | 5980.0 | 397.16 | 0 |
