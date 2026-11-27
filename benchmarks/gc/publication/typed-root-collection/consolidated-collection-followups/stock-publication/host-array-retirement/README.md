# Bounded recovery for host-backed arrays

The experiment keeps the full managed charge on a retired host buffer until
destruction. When a clear step yields a continuation, it observes how many
initialized bytes disappeared from that same exclusively held buffer. Those
bytes can justify additional recovery attempts up to the requesting thread's
starting heap footprint. Every allocation still checks the actual headroom.
A busy callback, clearing an unrelated buffer, and incremental relocation
supply no host progress credit. Regional partial returns retain their own
accounting.

The workerless control holds a storage lease over a 4,097-element host array.
Both implementations exhaust the request while the lease is held. Once it is
released, the exact-charge parent still reported exhaustion before the ready
array could pay. The candidate ran all 4,097 destructors once, returned the
charge and satisfied the request. The test drives actual wakes with a bounded
poll count. Each poll performs at most one collector step. Native and WASIp2
tests and Clippy passed. The post-edit review checked that the progress
counter can only extend the retry window. A successful allocation still
requires actual funds. [Candidate patch and tests](candidate.patch.gz) ·
[Build identities, gates, test totals and audit](qualification.json)

## Native Java

The candidate produced valid Java results in all 20 stock 64 MiB forks
across both publication policies. All 20 pre-exact-charge controls returned
Java OOM. A separate matched comparison against the immediate exact-charge
parent produced valid results in **all ten forks of each revision**. At
500 arrivals/s the parent left 18 of 25,000 arrivals unfinished and the
candidate left 14. Both release launchers also passed three fresh 512 MiB
checks apiece. The Java comparison therefore cannot attribute elimination
of the observed OOM solely to host-clear tracking. The held-lease fixture
distinguishes that mechanism directly. [116 process observations](observations.jsonl.gz)

## Release decision

The candidate is **withheld**. Native bundled JVM capacity was 93.68% of
control, below the 95% floor. All four sustained native processes completed
60,000 arrivals, with median p99.9 moving from 2,832 to 2,768 µs. WASIp2
bundled capacity was 103.25% of control and its sustained p99.9 ratio
was 0.927. These are three capacity pairs and two sustained pairs per
target. All 36 complete-cycle lower 95% bounds versus `f1daebeb` exceeded
10×, with minima of 11.006× on native and 10.407× on WASIp2.
[Complete-cycle estimates](cycles/complete-cycles.csv) ·
[Complete-cycle graph](cycles/speedup.png) · [Cycle build receipts](cycles/provenance.json)
