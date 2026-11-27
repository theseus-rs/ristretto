# Browser collector screen

Fixed scheduled arrivals; completed-request tails are censored when requests remain unfinished. These are actual Chromium main-thread measurements of the Rust collector, not a browser JVM or a ZGC/G1 comparison. Wasm linear memory does not shrink when objects are reclaimed. Qualification remains false. Request-turn work includes all polls and the GC step in turns with at least one request poll; maintenance-only and idle turns remain in the separate raw all-turn histogram. Clock resolution is recorded per fork and limits interpretation of small durations.

| Policy | Fork | Completed/offered | Arrival p99.9 µs | Request-turn work p99.9 µs | Heartbeat max µs | Peak cgroup memory MiB | Unfinished |
|---|---:|---:|---:|---:|---:|---:|---:|
| candidate | 0 | 4000/4000 | 5056 | 46 | 8545.0 | 393.09 | 0 |
| control | 0 | 4000/4000 | 5216 | 20 | 5635.0 | 396.41 | 0 |
| baseline | 0 | 4000/4000 | 5408 | 56 | 9870.0 | 392.13 | 0 |
| baseline | 1 | 4000/4000 | 5312 | 66 | 8240.0 | 393.13 | 0 |
| candidate | 1 | 4000/4000 | 5248 | 40 | 5935.0 | 394.53 | 0 |
| control | 1 | 4000/4000 | 5376 | 15 | 6080.0 | 395.71 | 0 |
| candidate | 2 | 4000/4000 | 5248 | 55 | 7800.0 | 394.68 | 0 |
| baseline | 2 | 4000/4000 | 4992 | 55 | 5990.0 | 397.26 | 0 |
| control | 2 | 4000/4000 | 4992 | 20 | 9165.0 | 395.48 | 0 |
