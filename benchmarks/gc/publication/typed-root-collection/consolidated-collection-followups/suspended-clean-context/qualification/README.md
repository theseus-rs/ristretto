# Traced deletions at the snapshot boundary

The traced-edge change passed the recorded release thresholds. Candidate `729aea8f` used the shared JVM collector. The comparison JVM at `ab7a3000` contains the borrowed queue links. The runtime trees measured here are identical to shipping commit `cc51335c`. [Git object identities and standalone tests](shipping-source.json). [Decision](decision.json) · [Predeclared policy](policy.json).

## Progress and graph lifetime

On the shipping control, dropping the last embedded edge after the full snapshot activated could clear both a child and its grandchild on that ticket. The legacy marking flag still read idle. The correction samples both domains from the authoritative phase clock. The forced schedule verifies both original weak identities through the running cycle, then checks that both clear in the next cycle. [Shipping failure](barriers/phase-cut-shipping-control.log.gz) · [Candidate replay](barriers/phase-cut-candidate.log.gz).

For an attached context and an exact same-heap traced edge during strong marking, the candidate offers the edge to each receiving epoch. It validates the epoch and object lifetime before returning. When certification cannot finish, the ordinary publication path handles the edge. Bounding the queued offers and sharing the record only when needed avoids queuing 4,096 proposals for one repeated edge. Raw edges, dirty-owner publication and other phases still need their own progress checks. The five deterministic schedules exercised 1,200 cases. [Validation](validation.json) · [Schedule records](forced-schedules.jsonl.gz) · [Source patch from `8cd9d2e4`](barriers/shipping-proposal.patch.gz).

## Complete collection and barrier cost

All 36 conditions cleared the 10× threshold relative to `f1daebeb`. The smallest individual lower 95% bounds were **10.430× native** and **10.443× WASI**, from 2,304 fresh process runs across the two targets. In the 576 final paired-process barrier measurements, the largest candidate/parent point estimates across operation and total time were **0.998 native** and **1.007 WASI**. Earlier variants exceeded the 1.10 limit and were revised. [Collection graph](cycles/speedup.png) · [Per-condition intervals](cycles/complete-cycles.csv) · [Final barrier graph](barriers/shipping-cost.png) · [Earlier variants](barriers/barrier-cost.png).

## Whole-JVM reference-array comparison

Each fork used two CPUs, a 64 MiB managed heap and a 512 MiB process memory cap. The short columns are medians of three forks. The two-minute column is one fork per implementation at 500 scheduled arrivals/s. Latencies include time waiting behind earlier arrivals and exclude unfinished requests.

| Runtime | Saturation ops/s | 10 s scheduled p99.9, ms | 120 s completed / 60,000 | 120 s scheduled p99.9, ms |
|---|---:|---:|---:|---:|
| Ristretto before | 533 | 251.904 | 59,999 | 729.088 |
| Ristretto candidate | 537 | 3.264 | 60,000 | 1,261.568 |
| HotSpot G1 | 912,567 | 0.023 | 60,000 | 0.514 |
| HotSpot ZGC | 489,136 | 0.297 | 60,000 | 0.259 |

All three short candidate forks completed 5,000 arrivals. Short-run saturation was 100.7% of the parent. The two-minute candidate p99.9 rose from 729.088 ms on the parent to 1,261.568 ms, with a peak backlog of 664. G1 recorded 0.514 ms and ZGC 0.259 ms. These single forks establish a tail-latency gap and leave sustained latency unqualified. Three Chromium candidate forks each finished 4,000 arrivals. Their request-turn p99.9 values were 50, 50 and 66 µs, with a largest heartbeat delay of 9.74 ms. Chromium exercised the Rust collector on the main thread. [All Java forks](java/forks.csv) · [Short comparison](java/comparison.png) · [Two-minute results](java/sustained.csv) · [Two-minute graph](java/sustained.png) · [Chromium graph](browser/browser.png).
