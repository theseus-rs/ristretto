# Browser collector screen

Fixed scheduled arrivals; completed-request tails are censored when requests remain unfinished. These are actual Chromium main-thread measurements of the Rust collector, not a browser JVM or a ZGC/G1 comparison. Wasm linear memory does not shrink when objects are reclaimed. Qualification remains false. Request-turn work includes all polls and the GC step in turns with at least one request poll; maintenance-only and idle turns remain in the separate raw all-turn histogram. Clock resolution is recorded per fork and limits interpretation of small durations.

| Policy | Fork | Completed/offered | Arrival p99.9 µs | Request-turn work p99.9 µs | Heartbeat max µs | Peak cgroup memory MiB | Unfinished |
|---|---:|---:|---:|---:|---:|---:|---:|
| control | 0 | 4000/4000 | 5184 | 16 | 8210.0 | 414.61 | 0 |
| baseline | 0 | 4000/4000 | 5216 | 140 | 7480.0 | 395.86 | 0 |
| candidate | 0 | 4000/4000 | 5344 | 1408 | 8720.0 | 394.59 | 0 |
| candidate | 1 | 4000/4000 | 5152 | 60 | 9890.0 | 391.75 | 0 |
| control | 1 | 4000/4000 | 4960 | 20 | 5735.0 | 396.82 | 0 |
| baseline | 1 | 4000/4000 | 5024 | 96 | 7590.0 | 395.03 | 0 |
| candidate | 2 | 3997/4000 | 8768 | 71 | 11505.0 | 394.91 | 3 |
| control | 2 | 3998/4000 | 5216 | 11 | 6410.0 | 396.45 | 2 |
| baseline | 2 | 4000/4000 | 4736 | 46 | 5960.0 | 400.05 | 0 |
