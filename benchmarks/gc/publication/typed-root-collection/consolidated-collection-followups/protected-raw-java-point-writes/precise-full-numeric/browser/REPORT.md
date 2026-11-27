# Browser collector screen

Fixed scheduled arrivals; completed-request tails are censored when requests remain unfinished. These are actual Chromium main-thread measurements of the Rust collector, not a browser JVM or a ZGC/G1 comparison. Wasm linear memory does not shrink when objects are reclaimed. Qualification remains false. Request-turn work includes all polls and the GC step in turns with at least one request poll; maintenance-only and idle turns remain in the separate raw all-turn histogram. Clock resolution is recorded per fork and limits interpretation of small durations.

| Policy | Fork | Completed/offered | Arrival p99.9 µs | Request-turn work p99.9 µs | Heartbeat max µs | Peak cgroup memory MiB | Unfinished |
|---|---:|---:|---:|---:|---:|---:|---:|
| baseline | 0 | 4000/4000 | 5152 | 66 | 8520.0 | 394.22 | 0 |
| control | 0 | 4000/4000 | 5088 | 16 | 7380.0 | 390.71 | 0 |
| candidate | 0 | 4000/4000 | 5248 | 55 | 5870.0 | 392.43 | 0 |
| control | 1 | 4000/4000 | 5440 | 21 | 6435.0 | 397.25 | 0 |
| candidate | 1 | 4000/4000 | 5408 | 66 | 8215.0 | 391.79 | 0 |
| baseline | 1 | 4000/4000 | 5440 | 75 | 6140.0 | 389.38 | 0 |
| control | 2 | 4000/4000 | 5440 | 16 | 6760.0 | 390.85 | 0 |
| candidate | 2 | 4000/4000 | 5344 | 66 | 7555.0 | 391.50 | 0 |
| baseline | 2 | 4000/4000 | 5344 | 66 | 8480.0 | 390.46 | 0 |
