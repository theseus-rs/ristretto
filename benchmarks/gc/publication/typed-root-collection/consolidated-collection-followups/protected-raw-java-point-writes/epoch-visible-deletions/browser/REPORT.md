# Browser collector screen

Fixed scheduled arrivals; completed-request tails are censored when requests remain unfinished. These are actual Chromium main-thread measurements of the Rust collector, not a browser JVM or a ZGC/G1 comparison. Wasm linear memory does not shrink when objects are reclaimed. Qualification remains false. Request-turn work includes all polls and the GC step in turns with at least one request poll; maintenance-only and idle turns remain in the separate raw all-turn histogram. Clock resolution is recorded per fork and limits interpretation of small durations.

| Policy | Fork | Completed/offered | Arrival p99.9 µs | Request-turn work p99.9 µs | Heartbeat max µs | Peak cgroup memory MiB | Unfinished |
|---|---:|---:|---:|---:|---:|---:|---:|
| baseline | 0 | 3998/4000 | 5216 | 61 | 8005.0 | 394.88 | 2 |
| candidate | 0 | 4000/4000 | 8256 | 2208 | 10610.0 | 390.26 | 0 |
| control | 0 | 4000/4000 | 5440 | 26 | 5720.0 | 392.17 | 0 |
| candidate | 1 | 4000/4000 | 5024 | 60 | 8860.0 | 395.05 | 0 |
| baseline | 1 | 4000/4000 | 5344 | 71 | 7600.0 | 388.49 | 0 |
| control | 1 | 4000/4000 | 5376 | 16 | 6465.0 | 390.65 | 0 |
| baseline | 2 | 4000/4000 | 5376 | 65 | 5835.0 | 390.24 | 0 |
| candidate | 2 | 4000/4000 | 5312 | 70 | 6865.0 | 389.37 | 0 |
| control | 2 | 4000/4000 | 5440 | 16 | 8125.0 | 392.09 | 0 |
