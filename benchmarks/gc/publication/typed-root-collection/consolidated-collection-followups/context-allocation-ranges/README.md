# Context allocation ranges and logical credit

The release fixture at `5ba3b207` ran with one native worker, a 16 MiB compact heap, registered context roots and asynchronous allocation. The physical arm enabled the descriptor region and both managed-payload range paths. The credit arm enabled the headroom, interest and reservation paths for cached logical credit. The three experimental builds retained the same context-root, flush and collection settings. The switch-off and combined controls used the identical unmodified binary. None of these experiments add production settings.

Each row contains eight fresh one-second processes. Each randomized block covered all five arms. Values are medians over successful processes. Every success returned live objects and occupied region bytes to zero. At 4 KiB the switch-off, neither and combined arms each had one out-of-memory exit. The separate paired comparisons include only blocks where both corresponding arms finished.

| Payload | Physical ranges / credit | Completed | Allocations/s | Operation p99.99 µs |
|---|---|---:|---:|---:|
| 256 B | Buffer switch off | 8/8 | 384,722 | 94 |
| 256 B | Off / off | 8/8 | 389,688 | 86 |
| 256 B | Off / on | 8/8 | 414,907 | 86.5 |
| 256 B | On / off | 8/8 | 299,765 | 334 |
| 256 B | On / on | 8/8 | 277,728 | 343 |
| 4096 B | Buffer switch off | 7/8 | 363,049 | 69 |
| 4096 B | Off / off | 7/8 | 388,309 | 57 |
| 4096 B | Off / on | 8/8 | 373,168 | 64.5 |
| 4096 B | On / off | 8/8 | 327,831 | 140 |
| 4096 B | On / on | 7/8 | 340,661 | 131 |

At 256 B the physical-only arm lowered throughput and increased p99.99 relative to the neither arm. At 4 KiB its successful processes also ran slower and had higher tails. Logical credit alone had a different cost profile. These short native samples leave a general-purpose allocation-buffer default unqualified.

To reproduce, apply the [physical-only](physical.patch.gz), [credit-only](credit.patch.gz) or [neither](neither.patch.gz) benchmark patch to a clean `5ba3b207` checkout. Decompress the selected patch with `gzip -dc` and pipe it to `git apply`. Build `cargo build --locked --release -p ristretto_classloader --example gc_recovery_latency`. The combined and switch-off arms use the unmodified checkout. Run the executable on CPUs 4, 6 and 9 with `1 1 16 true async 8 false false 1 false 1 false true 0 LOCAL SIZE`, where LOCAL is false only for the switch-off arm and SIZE is 256 or 4096. Process order used seed 701832. An initial harness attempt stopped at its first nonzero exit before recording any samples. The reported run retained every outcome.

[All process outputs](processes.jsonl.gz), [outcomes](outcomes.csv), [paired intervals](paired.csv), [binary and collector hashes](binaries.json).
