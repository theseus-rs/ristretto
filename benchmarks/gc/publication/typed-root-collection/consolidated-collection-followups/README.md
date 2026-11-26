# Collection performance through the integrated collector revisions

Each revision, including the withheld experiment, ran the full 36-condition comparison against the same baseline collector at `f1daebeb`. The table shows the weakest of the individual paired bootstrap 95% lower speedup bounds. Every condition must reach 10×.

| Collector change | Revision | Schedule seed | Native | WASI | Passing conditions |
| --- | --- | ---: | ---: | ---: | ---: |
| Idle context root-page repair | `c8716a11` | 251126871 | 10.28× | 11.23× | 36 / 36 |
| Eight address cells per leaf | `22b87754` | 251126877 | 9.72× | 11.22× | 34 / 36 |
| Dense cells for deadline-free sweeps | `ca35c898` | 251126898 | 10.17× | 11.30× | 36 / 36 |
| Deadline-aware cells and buffered dirty-owner publication | `d683da11` | 251126683 | 9.97× | 11.20× | 35 / 36 |
| Bounded phantom cleanup and shorter phase publication | `899fdc20` | 251126899 | 9.60× | 11.40× | 33 / 36 |
| Recovery observation, scalar clearing and nursery sweep changes | `849b9f31` | 251126849 | 10.19× | 11.96× | 36 / 36 |
| Region evacuation epochs and nonblocking trim probes | `81594aad` | 251126815 | 10.18× | 11.68× | 36 / 36 |
| Owned source and destination checks during incremental copies | `1bd9360c` | 251126136 | 10.18× | 11.91× | 36 / 36 |
| Clean buffered edges while the target shard is parked | `74275197` | 251126742 | **10.11×** | **11.78×** | **36 / 36** |
| Experimental shared allocation credit (withheld) | `4302f9b7` | 251126602 | 10.07× | 11.69× | 36 / 36 |
| Waiting allocations publish context credit to the heap (migration failure) | `35e91dbf` | 251126935 | 10.26× | 11.61× | 36 / 36 |
| Shared credit across pending-future context changes | `ac047016` | 251126936 | 9.88× | 11.56× | 35 / 36 |
| Fresh mature edges counted on each marking queue | `b041f0c3` | 251126937 | **10.55×** | **11.76×** | **36 / 36** |

At `b041f0c3`, the weakest median speedups were 11.62× native and 11.95× WASI. The collector records each newly strong mature edge under its destination queue mutex, then sums the lane counts at full sweep. A two-lane diagnostic paused a real marking callback between marking its targets and recording their tally. Another helper kept making bounded calls, the ticket stayed open, and the next full collection reclaimed an unreachable mature object. All 200 forced schedules passed. [Test patches, process results, and Cargo packaging receipt](credit-and-census.json.gz)

Shared-context waiters can now spend prepaid bytes while the range cache is held. Their space observations make unused credit available to the heap through a real refund. The wake tests cover a parked collector, a stolen first refund, partial credit, cancellation, and futures that change attached contexts. The context suite passed in 100 fresh native processes. The intermediate `35e91dbf` missed context changes and `ac047016` missed the native counted two-worker tree with diagnostics enabled. [Waiting-credit processes](waiting-credit.csv) · [Migrated-credit processes](migrated-credit.csv) · [Integrated lane-census processes](lane-census.csv)

A direct comparison used 48 randomized native process pairs per tree condition against the immediate parent `ac047016`. With two workers and diagnostics disabled, median elapsed time decreased by 10.9% under gated publication and 6.4% under counted publication. Both individual 95% intervals excluded parity. With diagnostics enabled, both intervals included parity. The direct comparison does not establish that the census change alone improved diagnostic-enabled tree time. [Parent comparison, raw processes and per-condition intervals](lane-parent.json.gz)


The shared-credit experiment passed the timing gate and 100 repetitions of the context tests. A separate forced schedule exposed an allocation failure. One peer observed an empty context and bypassed a parked range cache. A second peer refilled the context and retained its 8-byte root. With 4,088 bytes now prepaid, the first peer returned OOM for 8 bytes. Subsequent revisions above qualify the corrected path. The diagnostic bundle includes both patches against the recorded base, the test command, and the failing result. [Diagnostic bundle](context-credit-witness.json.gz) · [Process records](context-credit.csv)

At `74275197`, the minimum paired medians were 10.33× native and 11.84× WASI. In 300 separate native processes, a clean buffered edge could be acknowledged with its target shard parked during collector draining, explicit flushing or a mutator's overflow. The collector retained the child graph. A consumer parked after claiming a record still kept its collection ticket open. The parent failed the collector-drain assertion. [Forced schedules and parent failure](barrier-schedules.json.gz)

Collector `1bd9360c` reached minimum medians of 10.83× native and 12.13× WASI. A separate public-buffer regression parked the source registry during both initial and continued partial copies into a different region. The test timed out on the parent and finished both copies and exact byte checks on this revision.

Collector `81594aad` cleared all 36 conditions with minimum medians of 10.42× native and 11.81× WASI. The core fixture does not enable region compaction. Separate forced tests exercise the new region transitions.

Collector `849b9f31` cleared the gate again. Its slowest medians were 10.92× native and 12.22× WASI. It includes the parked phase-statistics and retained phantom cleanup fixes, the independently registered refund observations, and the linked Java class layout change.

The earlier merged revision at `d683da11` had one interval crossing the gate, on the native mixed graph with two workers, gated publication and diagnostics disabled. Its paired median was 10.93×. The slowest median across all its conditions was 10.77× native and 11.34× WASI. Each revision used 1,728 fresh processes and 27,648 timed full cycles. All passed the correctness and managed-byte checks.

![Weakest per-condition confidence bound for each source revision](minimum.svg)

[Per-condition results](summary.json) · [Compiler, source, binary and host provenance](provenance.json) · Ordered process records for [context repair](context.csv), [eight-cell leaves](sparse.csv), [deadline-aware leaves](deadline.csv), [the merge](merged.csv), [phase changes](phase.csv), [recovery and sweep](recovery.csv), [region transitions](regions.csv), [source validation](copy.csv) and [clean buffered edges](clean-edges.csv).

Use the [original integrated protocol](../consolidated-complete/README.md) with the candidate revision and seed in the table. The collection benchmark measures elapsed time to mark, sweep and reclaim. The Java application uses time-limited steps and has separate latency and memory measurements.
