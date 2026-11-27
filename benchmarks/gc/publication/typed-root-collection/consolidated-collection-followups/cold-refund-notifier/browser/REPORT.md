# Browser collector screen

Fixed scheduled arrivals; completed-request tails are censored when requests remain unfinished. These are actual Chromium main-thread measurements of the Rust collector, not a browser JVM or a ZGC/G1 comparison. Wasm linear memory does not shrink when objects are reclaimed. Qualification remains false. Request-turn work includes all polls and the GC step in turns with at least one request poll; maintenance-only and idle turns remain in the separate raw all-turn histogram. Clock resolution is recorded per fork and limits interpretation of small durations.

| Policy | Fork | Completed/offered | Arrival p99.9 µs | Request-turn work p99.9 µs | Heartbeat max µs | Peak cgroup memory MiB | Unfinished |
|---|---:|---:|---:|---:|---:|---:|---:|
| candidate | 0 | 4000/4000 | 4992 | 45 | 6820.0 | 418.38 | 0 |
| baseline | 0 | 4000/4000 | 4992 | 46 | 6715.0 | 397.50 | 0 |
| control | 0 | 4000/4000 | 5120 | 20 | 5720.0 | 398.39 | 0 |
| control | 1 | 4000/4000 | 4928 | 11 | 6775.0 | 396.10 | 0 |
| candidate | 1 | 4000/4000 | 5056 | 66 | 6240.0 | 396.81 | 0 |
| baseline | 1 | 3994/4000 | 5024 | 46 | 6240.0 | 399.24 | 6 |
| baseline | 2 | 4000/4000 | 5184 | 46 | 8455.0 | 396.25 | 0 |
| candidate | 2 | 4000/4000 | 5248 | 46 | 6595.0 | 396.92 | 0 |
| control | 2 | 4000/4000 | 4800 | 20 | 7090.0 | 398.29 | 0 |
