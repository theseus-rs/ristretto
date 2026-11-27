# Browser collector screen

Fixed scheduled arrivals; completed-request tails are censored when requests remain unfinished. These are actual Chromium main-thread measurements of the Rust collector, not a browser JVM or a ZGC/G1 comparison. Wasm linear memory does not shrink when objects are reclaimed. Qualification remains false. Request-turn work includes all polls and the GC step in turns with at least one request poll; maintenance-only and idle turns remain in the separate raw all-turn histogram. Clock resolution is recorded per fork and limits interpretation of small durations.

| Policy | Fork | Completed/offered | Arrival p99.9 µs | Request-turn work p99.9 µs | Heartbeat max µs | Peak cgroup memory MiB | Unfinished |
|---|---:|---:|---:|---:|---:|---:|---:|
| candidate | 0 | 4000/4000 | 5344 | 66 | 8075.0 | 398.03 | 0 |
| control | 0 | 4000/4000 | 5216 | 16 | 7855.0 | 391.83 | 0 |
| baseline | 0 | 4000/4000 | 5184 | 75 | 7445.0 | 389.34 | 0 |
| candidate | 1 | 4000/4000 | 5184 | 55 | 10555.0 | 393.66 | 0 |
| baseline | 1 | 3995/4000 | 5408 | 76 | 6090.0 | 392.55 | 5 |
| control | 1 | 3998/4000 | 5120 | 26 | 7685.0 | 392.96 | 2 |
| control | 2 | 4000/4000 | 4896 | 20 | 6735.0 | 394.09 | 0 |
| baseline | 2 | 4000/4000 | 5248 | 60 | 8295.0 | 393.72 | 0 |
| candidate | 2 | 4000/4000 | 5408 | 51 | 6775.0 | 392.52 | 0 |
