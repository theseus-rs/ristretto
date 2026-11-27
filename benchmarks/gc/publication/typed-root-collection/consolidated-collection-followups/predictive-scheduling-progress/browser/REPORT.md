# Browser collector screen

Fixed scheduled arrivals; completed-request tails are censored when requests remain unfinished. These are actual Chromium main-thread measurements of the Rust collector, not a browser JVM or a ZGC/G1 comparison. Wasm linear memory does not shrink when objects are reclaimed. Qualification remains false. Request-turn work includes all polls and the GC step in turns with at least one request poll; maintenance-only and idle turns remain in the separate raw all-turn histogram. Clock resolution is recorded per fork and limits interpretation of small durations.

| Policy | Fork | Completed/offered | Arrival p99.9 µs | Request-turn work p99.9 µs | Heartbeat max µs | Peak cgroup memory MiB | Unfinished |
|---|---:|---:|---:|---:|---:|---:|---:|
| candidate | 0 | 4000/4000 | 4672 | 40 | 5965.0 | 420.90 | 0 |
| control | 0 | 4000/4000 | 4768 | 15 | 5770.0 | 396.98 | 0 |
| baseline | 0 | 4000/4000 | 6080 | 46 | 7940.0 | 400.82 | 0 |
| control | 1 | 4000/4000 | 5056 | 15 | 5600.0 | 397.59 | 0 |
| baseline | 1 | 4000/4000 | 5152 | 40 | 7150.0 | 399.63 | 0 |
| candidate | 1 | 4000/4000 | 4896 | 60 | 6400.0 | 399.48 | 0 |
| candidate | 2 | 4000/4000 | 4704 | 46 | 5530.0 | 395.92 | 0 |
| control | 2 | 4000/4000 | 4768 | 16 | 5535.0 | 398.25 | 0 |
| baseline | 2 | 4000/4000 | 5120 | 56 | 5760.0 | 397.31 | 0 |
