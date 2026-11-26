# One-minute compacting allocation trials

Collector `81594aad` completed 600,000 scheduled array-element writes in each of three 60-second processes. Each process finished with zero backlog, zero live objects and zero occupied managed bytes. All three met the predeclared operation limits of p99.9 below 100 µs and p99.99 below 1,000 µs. Latency measured from the scheduled arrival was higher.

| Trial | Operation p99.9 (µs) | Operation p99.99 (µs) | Scheduled p99.9 (µs) | Scheduled p99.99 (µs) | Peak cgroup memory (MiB) |
| --- | ---: | ---: | ---: | ---: | ---: |
| 1 | 77 | 784 | 2784 | 3424 | 34.8 |
| 2 | 49 | 612 | 606 | 2400 | 37.3 |
| 3 | 45 | 573 | 2016 | 3296 | 35.7 |

The screen used `gc_endurance` with a 64 MiB managed heap, a 512 MiB cgroup, two CPUs, two workers, async allocation, cost-aware compaction, 16 region shards, counted publication, context roots, 32 barrier records and local allocation buffers. The fixed arrival rate was 10,000 operations/s. Longer-duration, whole-JVM and global-pause qualification remain open.

[Measurements, build hashes, per-second capacity samples and histograms](measurements.json.gz) · [Build and reproduction instructions](../../../README.md#collector-endurance-and-recovery)
