# Browser collector screen

Fixed scheduled arrivals; completed-request tails are censored when requests remain unfinished. These are actual Chromium main-thread measurements of the Rust collector, not a browser JVM or a ZGC/G1 comparison. Wasm linear memory does not shrink when objects are reclaimed. Qualification remains false. Request-turn work includes all polls and the GC step in turns with at least one request poll; maintenance-only and idle turns remain in the separate raw all-turn histogram. Clock resolution is recorded per fork and limits interpretation of small durations.

| Policy | Fork | Completed/offered | Arrival p99.9 µs | Request-turn work p99.9 µs | Heartbeat max µs | Peak cgroup memory MiB | Unfinished |
|---|---:|---:|---:|---:|---:|---:|---:|
| candidate | 0 | 4000/4000 | 5056 | 46 | 9915.0 | 397.99 | 0 |
| baseline | 0 | 4000/4000 | 5056 | 46 | 5960.0 | 396.24 | 0 |
| control | 0 | 4000/4000 | 5056 | 26 | 7035.0 | 396.77 | 0 |
| baseline | 1 | 4000/4000 | 4992 | 46 | 6705.0 | 399.53 | 0 |
| candidate | 1 | 4000/4000 | 5952 | 55 | 10000.0 | 396.41 | 0 |
| control | 1 | 4000/4000 | 4928 | 11 | 6225.0 | 397.63 | 0 |
| baseline | 2 | 4000/4000 | 4768 | 45 | 6655.0 | 398.05 | 0 |
| control | 2 | 4000/4000 | 5120 | 15 | 6620.0 | 398.75 | 0 |
| candidate | 2 | 3999/4000 | 5088 | 55 | 5975.0 | 397.27 | 1 |
