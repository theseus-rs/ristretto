# Java latency and memory across integrated collector revisions

[Inline instance fields missed the Java roots throughput gate in nine paired JVM runs.](inline-instance-fields/README.md)

[Unified JVM heap against G1 and ZGC at `b65722a0`, including saturated throughput and scheduled response](unified-heap-roots/README.md).

[Static long stores and scheduled Java roots latency at 1,000 and 1,500 requests/s](ordinary-static-long/README.md).


A 105-process Java roots capacity search at `cdee5888` found that Ristretto could meet the 1 ms scheduled p99.9 limit at 1,000 requests/s in one five-fork run and exceeded it in two confirmation forks. G1 passed the 128,000/s search ceiling. ZGC confirmed a passing 48,000/s and a failing 64,000/s. [Capacity comparison, all process outcomes and graph](roots-capacity/README.md).

At `480d279a` all twelve fresh forks completed their scheduled Java roots requests. At 1,500 requests/s the median scheduled p99.9 was 958 µs for Ristretto, 103.5 µs for G1 and 10 µs for ZGC. [Per-fork measurements, resource limits and graphs](synchronized-methods/README.md).

Each row compares two fresh forks per rate. The table gives the median of their scheduled-response p99.9 values and the highest cgroup memory across both rates. Each runtime completed every offered request.

| Ristretto revision in campaign | Runtime | 1,000/s p99.9 (µs) | 1,500/s p99.9 (µs) | Peak memory (MiB) |
| --- | --- | ---: | ---: | ---: |
| `22b87754` | Ristretto | 126 | 311.5 | 249.0 |
| `22b87754` | G1 | 7 | 47 | 60.5 |
| `22b87754` | ZGC | 6.5 | 243 | 96.5 |
| `ca35c898` | Ristretto | 154 | 310.5 | 251.4 |
| `ca35c898` | G1 | 54 | 11 | 60.7 |
| `ca35c898` | ZGC | 127.5 | 176.5 | 96.0 |
| `d683da11` | Ristretto | 160.5 | 413 | 249.6 |
| `d683da11` | G1 | 2.5 | 90.5 | 71.9 |
| `d683da11` | ZGC | 6.5 | 189.5 | 98.8 |
| `40e19ae0` | Ristretto | 170.5 | 241 | 255.1 |
| `40e19ae0` | G1 | 95.5 | 131.5 | 74.3 |
| `40e19ae0` | ZGC | 7 | 72.5 | 102.9 |
| `849b9f31` | Ristretto | 163 | 1028 | 257.8 |
| `849b9f31` | G1 | 241.5 | 12 | 62.0 |
| `849b9f31` | ZGC | 9.5 | 36.5 | 105.6 |
| `81594aad` | Ristretto | 243 | 315 | 253.0 |
| `81594aad` | G1 | 5.5 | 124.5 | 61.8 |
| `81594aad` | ZGC | 133 | 104.5 | 110.1 |
| `74275197` | Ristretto | 267.5 | 229.5 | 246.1 |
| `74275197` | G1 | 93.5 | 211 | 60.6 |
| `74275197` | ZGC | 156.5 | 156.5 | 109.7 |
| `b041f0c3` | Ristretto | 132 | 259 | 255.6 |
| `b041f0c3` | G1 | 166 | 137 | 62.4 |
| `b041f0c3` | ZGC | 140.5 | 127.5 | 96.7 |
| `ef490d09` | Ristretto | 156.5 | 258.5 | 253.4 |
| `ef490d09` | G1 | 144.5 | 165.5 | 60.5 |
| `ef490d09` | ZGC | 102.5 | 133.5 | 106.0 |
| `8b15e59c` | Ristretto | 405 | 979 | 262.7 |
| `8b15e59c` | G1 | 6.5 | 286.5 | 72.8 |
| `8b15e59c` | ZGC | 52.5 | 22.5 | 107.9 |

For `8b15e59c`, an earlier 12-fork run completed but the driver detected that files in its source checkout changed during execution. Its results are excluded from the table. The published repeat used the same frozen executable and a separate checkout pinned to the measured revision. The driver's input hashes and output validation passed. Other development builds and tests were active on the host during the repeat. [Excluded run and integrity receipt](changed-inputs-8b15e59c.json.gz)

The unchanged Java roots workload used a 64 MiB managed heap, two CPUs, a two-CPU quota, a 512 MiB cgroup, a five-second warmup and a 60-second measurement. The [earlier integrated run](../consolidated-java/README.md) reached 397.9 MiB on Ristretto. These short runs characterize latency and memory at fixed demand. Endurance, maximum sustainable throughput and global mutator pauses have separate gates.

![Per-fork memory and scheduled response](roots.svg)

[Process records](processes.csv) · [Histograms and memory samples](measurements.jsonl) · [Build and campaign provenance](provenance.json) · [Complete collection comparisons](../consolidated-collection-followups/README.md)
