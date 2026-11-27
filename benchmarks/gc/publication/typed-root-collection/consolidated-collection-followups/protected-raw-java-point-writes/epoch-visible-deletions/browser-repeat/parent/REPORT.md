# Browser collector screen

Fixed scheduled arrivals; completed-request tails are censored when requests remain unfinished. These are actual Chromium main-thread measurements of the Rust collector, not a browser JVM or a ZGC/G1 comparison. Wasm linear memory does not shrink when objects are reclaimed. Qualification remains false. Request-turn work includes all polls and the GC step in turns with at least one request poll; maintenance-only and idle turns remain in the separate raw all-turn histogram. Clock resolution is recorded per fork and limits interpretation of small durations.

| Policy | Fork | Completed/offered | Arrival p99.9 µs | Request-turn work p99.9 µs | Heartbeat max µs | Peak cgroup memory MiB | Unfinished |
|---|---:|---:|---:|---:|---:|---:|---:|
| candidate | 0 | 4000/4000 | 5216 | 51 | 8165.0 | 396.92 | 0 |
| control | 0 | 4000/4000 | 5312 | 15 | 7790.0 | 395.56 | 0 |
| baseline | 0 | 4000/4000 | 5472 | 46 | 7415.0 | 396.65 | 0 |
| baseline | 1 | 4000/4000 | 4960 | 71 | 7330.0 | 390.51 | 0 |
| candidate | 1 | 4000/4000 | 5216 | 65 | 10020.0 | 390.16 | 0 |
| control | 1 | 4000/4000 | 5248 | 16 | 7905.0 | 390.79 | 0 |
| candidate | 2 | 4000/4000 | 5440 | 66 | 9435.0 | 389.93 | 0 |
| baseline | 2 | 4000/4000 | 5120 | 55 | 7860.0 | 395.07 | 0 |
| control | 2 | 4000/4000 | 4864 | 11 | 6630.0 | 395.47 | 0 |
