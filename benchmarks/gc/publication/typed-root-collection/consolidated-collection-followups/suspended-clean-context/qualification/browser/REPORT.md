# Browser collector screen

Fixed scheduled arrivals; completed-request tails are censored when requests remain unfinished. These are actual Chromium main-thread measurements of the Rust collector, not a browser JVM or a ZGC/G1 comparison. Wasm linear memory does not shrink when objects are reclaimed. Qualification remains false. Request-turn work includes all polls and the GC step in turns with at least one request poll; maintenance-only and idle turns remain in the separate raw all-turn histogram. Clock resolution is recorded per fork and limits interpretation of small durations.

| Policy | Fork | Completed/offered | Arrival p99.9 µs | Request-turn work p99.9 µs | Heartbeat max µs | Peak cgroup memory MiB | Unfinished |
|---|---:|---:|---:|---:|---:|---:|---:|
| baseline | 0 | 4000/4000 | 5216 | 75 | 6560.0 | 416.57 | 0 |
| candidate | 0 | 4000/4000 | 5280 | 50 | 9740.0 | 389.89 | 0 |
| control | 0 | 4000/4000 | 5120 | 25 | 7050.0 | 394.56 | 0 |
| candidate | 1 | 4000/4000 | 5312 | 50 | 6795.0 | 394.09 | 0 |
| baseline | 1 | 4000/4000 | 5504 | 76 | 6150.0 | 393.57 | 0 |
| control | 1 | 4000/4000 | 5056 | 11 | 6770.0 | 391.51 | 0 |
| candidate | 2 | 4000/4000 | 5056 | 66 | 7130.0 | 394.30 | 0 |
| control | 2 | 4000/4000 | 5184 | 11 | 6430.0 | 393.54 | 0 |
| baseline | 2 | 4000/4000 | 5152 | 60 | 5920.0 | 393.61 | 0 |
