# One-minute compacting allocation trials

Collector `81594aad` completed 600,000 scheduled array-element writes in each of three 60-second processes. Each process finished with zero backlog, zero live objects and zero occupied managed bytes. All three met the predeclared operation limits of p99.9 below 100 µs and p99.99 below 1,000 µs. Latency measured from the scheduled arrival was higher.

| Trial | Operation p99.9 (µs) | Operation p99.99 (µs) | Scheduled p99.9 (µs) | Scheduled p99.99 (µs) | Peak cgroup memory (MiB) |
| --- | ---: | ---: | ---: | ---: | ---: |
| 1 | 77 | 784 | 2784 | 3424 | 34.8 |
| 2 | 49 | 612 | 606 | 2400 | 37.3 |
| 3 | 45 | 573 | 2016 | 3296 | 35.7 |

The screen used `gc_endurance` with a 64 MiB managed heap, a 512 MiB cgroup, two CPUs, two workers, async allocation, cost-aware compaction, 16 region shards, counted publication, context roots, 32 barrier records and local allocation buffers. The fixed arrival rate was 10,000 operations/s. Longer-duration, whole-JVM and global-pause qualification remain open.

[Measurements, build hashes, per-second capacity samples and histograms](measurements.json.gz) · [Build and reproduction instructions](../../../README.md#collector-endurance-and-recovery)

A separate repeated hard-cap test at the same revision returned `OutOfMemory` in 2 of 600 fresh native processes. Each process ran four combinations of publication and phase policy. A sealed identity-page unlink was held while four 8-byte requests competed for a dead 32-byte tuple. Each successful root was retained until all four peers completed. The raw failures do not identify which policy iteration returned the error. Reliable recovery in this case remains a release blocker. [Per-process outcomes and failing output](hard-cap-screen.json.gz)

A follow-up diagnostic at `1bd9360c` parked the physical reaper after it took exclusive ownership of the 32-byte donor and before deallocation. All four 8-byte requests returned terminal OOM before resume under each of the four policy combinations. At each terminal observation the heap still charged 32 bytes and reported 32 retired bytes. Resuming the reaper returned the charge to zero. This forced schedule is distinct from the unsynchronized failures above. The receipt contains the exact source revision, source patch, command and observed counters. [Forced reaper schedule](parked-refund.json.gz)

A 64-byte hard-cap diagnostic at `f2277fde` parked a reaper holding the first 32-byte donor, then dropped the last root of an independent 32-byte donor. Four peers each requested 8 bytes and retained every successful root. In 8 of 1,800 fresh native processes one to three peers returned terminal OOM. Across those failures every publication/phase combination was represented. After all peers returned and before the first reaper resumed, charges were 40, 48 or 56 bytes. Each missing root therefore had 8 bytes of headroom by that observation. The observations do not resolve the ordering of the independent refund and individual OOM decisions. [Reproduction patch, build identity and all 1,800 process outcomes](independent-donors.json.gz)

## Ordering terminal allocation failures against a physical refund

At `3c617b50`, 100 fresh native processes each ran four forced schedules, covering both publication and phase policies. Four 8-byte allocation futures first polled a full 64-byte heap. A reaper held one 32-byte donor before freeing it. One designated allocator retired and claimed the second donor, then paused before freeing it too. With both still parked, an independent helper completed a newly requested full collection. The other three allocators returned terminal OOM on actual wake-driven polls while all 64 bytes remained charged. Resuming only the second reaper let its own 8-byte request succeed and left 24 bytes of headroom. Every run drained to zero.

This establishes the order of events for the forced schedule. The earlier eight failures sampled headroom only after the peers joined. A fix must account for a donor as it moves from publicly reclaimable storage to private destruction and must preserve each pending request's ability to use an independent refund. [Reproduction patch, binary hash, tickets and charges for all 400 schedules](two-private-donors.json.gz)
