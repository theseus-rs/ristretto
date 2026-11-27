# Browser collector screen

Fixed scheduled arrivals; completed-request tails are censored when requests remain unfinished. These are actual Chromium main-thread measurements of the Rust collector, not a browser JVM or a ZGC/G1 comparison. Wasm linear memory does not shrink when objects are reclaimed. Qualification remains false. Request-turn work includes all polls and the GC step in turns with at least one request poll; maintenance-only and idle turns remain in the separate raw all-turn histogram. Clock resolution is recorded per fork and limits interpretation of small durations.

| Policy | Fork | Completed/offered | Arrival p99.9 µs | Request-turn work p99.9 µs | Heartbeat max µs | Peak cgroup memory MiB | Unfinished |
|---|---:|---:|---:|---:|---:|---:|---:|
| baseline | 0 | 4000/4000 | 4608 | 160 | 5500.0 | 400.27 | 0 |
| control | 0 | 4000/4000 | 4928 | 20 | 7185.0 | 400.63 | 0 |
| candidate | 0 | 3996/4000 | 5088 | 46 | 5550.0 | 397.49 | 4 |
| control | 1 | 4000/4000 | 5088 | 16 | 6110.0 | 396.89 | 0 |
| candidate | 1 | 4000/4000 | 5120 | 45 | 5595.0 | 396.62 | 0 |
| baseline | 1 | 4000/4000 | 4960 | 40 | 6445.0 | 398.86 | 0 |
| baseline | 2 | 4000/4000 | 4832 | 36 | 6685.0 | 397.63 | 0 |
| control | 2 | 4000/4000 | 4768 | 20 | 6930.0 | 398.70 | 0 |
| candidate | 2 | 4000/4000 | 5152 | 40 | 5515.0 | 395.93 | 0 |
