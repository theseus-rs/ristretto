# Browser collector screen

Fixed scheduled arrivals; completed-request tails are censored when requests remain unfinished. These are actual Chromium main-thread measurements of the Rust collector, not a browser JVM or a ZGC/G1 comparison. Wasm linear memory does not shrink when objects are reclaimed. Qualification remains false. Request-turn work includes all polls and the GC step in turns with at least one request poll; maintenance-only and idle turns remain in the separate raw all-turn histogram. Clock resolution is recorded per fork and limits interpretation of small durations.

| Policy | Fork | Completed/offered | Arrival p99.9 µs | Request-turn work p99.9 µs | Heartbeat max µs | Peak cgroup memory MiB | Unfinished |
|---|---:|---:|---:|---:|---:|---:|---:|
| candidate | 0 | 4000/4000 | 5248 | 66 | 8510.0 | 413.24 | 0 |
| baseline | 0 | 4000/4000 | 5536 | 71 | 7180.0 | 392.47 | 0 |
| control | 0 | 4000/4000 | 5024 | 20 | 5995.0 | 391.15 | 0 |
| candidate | 1 | 4000/4000 | 5312 | 56 | 7305.0 | 390.47 | 0 |
| baseline | 1 | 4000/4000 | 5152 | 66 | 9075.0 | 390.06 | 0 |
| control | 1 | 4000/4000 | 5152 | 26 | 6655.0 | 391.00 | 0 |
| baseline | 2 | 4000/4000 | 5120 | 60 | 7305.0 | 389.50 | 0 |
| candidate | 2 | 4000/4000 | 5408 | 71 | 6030.0 | 391.68 | 0 |
| control | 2 | 4000/4000 | 4992 | 26 | 7035.0 | 388.03 | 0 |
