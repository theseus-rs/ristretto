# Browser collector screen

Fixed scheduled arrivals; completed-request tails are censored when requests remain unfinished. These are actual Chromium main-thread measurements of the Rust collector, not a browser JVM or a ZGC/G1 comparison. Wasm linear memory does not shrink when objects are reclaimed. Qualification remains false. Request-turn work includes all polls and the GC step in turns with at least one request poll; maintenance-only and idle turns remain in the separate raw all-turn histogram. Clock resolution is recorded per fork and limits interpretation of small durations.

| Policy | Fork | Completed/offered | Arrival p99.9 µs | Request-turn work p99.9 µs | Heartbeat max µs | Peak cgroup memory MiB | Unfinished |
|---|---:|---:|---:|---:|---:|---:|---:|
| control | 0 | 4000/4000 | 5376 | 16 | 7195.0 | 392.39 | 0 |
| baseline | 0 | 4000/4000 | 5792 | 70 | 10625.0 | 389.64 | 0 |
| candidate | 0 | 4000/4000 | 5120 | 46 | 8070.0 | 390.20 | 0 |
| baseline | 1 | 4000/4000 | 5440 | 60 | 8550.0 | 390.00 | 0 |
| control | 1 | 4000/4000 | 5152 | 20 | 6555.0 | 390.88 | 0 |
| candidate | 1 | 4000/4000 | 5376 | 60 | 8050.0 | 390.29 | 0 |
| baseline | 2 | 4000/4000 | 5248 | 60 | 6860.0 | 391.65 | 0 |
| candidate | 2 | 4000/4000 | 5440 | 75 | 6180.0 | 391.40 | 0 |
| control | 2 | 4000/4000 | 5248 | 26 | 10305.0 | 391.29 | 0 |
