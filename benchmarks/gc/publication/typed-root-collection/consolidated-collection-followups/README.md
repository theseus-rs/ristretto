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
| Observe a fitting refund before the first allocation yield | `2b324c42` | 251126939 | **10.09×** | **11.62×** | **36 / 36** |
| Allocation refund observations while a registrar is parked | `42c37eb4` | 251126941 | **10.37×** | **10.73×** | **36 / 36** |
| Reserve physically released donor charges for waiting allocations | `a9242931` | 251126942 | **10.53×** | **10.63×** | **36 / 36** |
| Paged enrollment with a helpable commit | `5c05cc39` | 251126945 | **11.01×** | **11.38×** | **36 / 36** |
| Synchronous recovery at the managed heap limit | `ef490d09` | 251126947 | **10.88×** | **10.95×** | **36 / 36** |
| Public deferred selection, initial poll regression | `e8b0e6d2` | 251126951 | 10.43× | 11.01× | 36 / 36 |
| Terminal poll budget, eager grant records | `e5d1b3d5` | 251126953 | 10.57× | 11.20× | 36 / 36 |
| Per-donor physical claim and lazy grant preparation | `0a7993f0` | 251126959 | **10.17×** | **10.60×** | **36 / 36** |

## Helping a stopped selector after lease release

At `0a7993f0`, four 8-byte requests spent one returned 32-byte donor while the original selector remained parked. The test stops the selector before binding, after binding, after a partial scan and immediately before the physical claim. Peer helpers retain the exact request generation across cancellation and reopening. One per-donor state transition gives a helper exclusive responsibility for physical release. Concurrent insertion of a second donor can postpone directory cleanup without revoking the first claim.

Ten regressions passed in 100 fresh native processes each. They cover preparations for ordinary and returned storage, cancellation between binding and cursor publication, four policy combinations for donor sharing, and an eligible request behind 4,096 oversized interests. The latter case checks at most one measured collection batch per poll. A separate test changes the caller's managed guard or available credit while the terminal poll is suspended. Native passed 516 default library tests and 1,161 all-feature checks. WASI passed 313 library tests and 16 allocation and recovery integration tests. Formatting, native Clippy with warnings denied, the bare Wasm build and verified Cargo packaging passed. [Tests, original failures, schedules and package hash](deferred-help-tests.json.gz) · [Complete-collection process measurements](deferred-help.csv)

The initial selection change passed all collection timing conditions but failed the existing async poll-budget tests. Eager allocation of grant records in the next revision increased full-heap registration and cancellation time by 66–70% versus `ef490d09`. Preparing a record only for a fitting request brought the median paired elapsed-time ratios to 0.978, 0.972 and 1.002 at one, 64 and 1,024 pending requests. The last interval included parity. Costs relative to the earlier `42c37eb4` control were 1.75×, 1.84× and 1.31×. [Enrollment comparison and 95% intervals](deferred-enrollment.svg) · [Initial collection timing](deferred-help-initial.csv) · [Poll-budget revision timing](deferred-help-poll.csv)

Coverage here is for directly transferable storage after callback lease release. A parked allocation-waiter index still blocks an unrelated full collection in the consolidated implementation. Region storage, finalizer paths, integrated VM semantics and host pause qualification remain open.

## Synchronous allocation recovery

At `ef490d09`, an 8-byte synchronous allocation can reclaim an unreachable 32-byte donor at the configured heap limit. The single-attempt constructor returns its unpublished value when the heap is full. The recovering constructor roots outgoing edges before helping its own collection ticket. A private donor must release its storage before a waiting allocation can spend the charge. On a single-threaded Wasm host the synchronous constructor drives collection on the calling turn. Callers that need to yield to the host can use the asynchronous constructor.

All 300 fresh native processes passed across three tests covering sleeping synchronous allocators, a parked native collector worker, and asynchronous donor sharing. The tests retained the resulting objects and checked the heap charge. Native passed 505 default library tests and 1,150 all-feature checks. WASI passed 311 library tests and 16 allocation and recovery integration tests. The initial all-feature run exposed a shutdown test reading its flag while a native worker could still own the collector. The verified test waits for destruction before checking finalization. Strict native Clippy, the bare Wasm build and Cargo package verification passed. [Test outcomes, original failures and package identity](sync-recovery-tests.json.gz) · [Paired collection samples](sync-recovery.csv)

A separate native test parked the allocation waiter index. A peer exceeded its one-second deadline to reclaim an unrelated object and finish its ticket. Independent wake delivery and deferred storage retirement remain open.

## Reclamation after a callback lease returns

On the consolidated source at `2fb3cb90`, three of four eligible 8-byte requests returned OOM while the second 32-byte donor was privately held for destruction. The test first retained a callback lease across collection. It then enrolled the competing requests either before lease release or between release and the first reaper claim. All 800 native schedules reproduced the three premature results across the two enrollment orders and four policy combinations.

A local prototype captured enrolled requests on the first reaper claim and retained that selection across bounded batches. It passed all 800 matched schedules, 507 native library tests and strict native Clippy. While a helper owns a partially scanned donor outside the queue, another helper cannot resume that particular scan. Suspension inside selection, independently available donors, native/Wasm qualification and the collection performance gate remain open for the prototype. [Matched process results, source patches and verification](leased-donor-trials.json.gz)

## Paged enrollment

At `5c05cc39`, the collector commits interests in chronological order. A peer can commit a linked slot while its owner is stopped. A donor sees precisely the positions below its cutoff and retains the original request order through cancellation. Cancellation during assignment leaves the full share available to a later request. Both forced races and the four-allocator test passed in 100 fresh processes each. The original four-allocator fixture passed in all 1,800 fresh processes. The native default library suite passed 503 tests, the native all-feature suite passed 1,146, and the WASI library suite passed 311. The bare Wasm build, strict native Clippy and verified Cargo package passed. [Schedules, process outcomes and build identities](paged-enrollment-tests.json.gz)

The allocation benchmark ran all three revisions on the same core with randomized order in 32 fresh process triples at each queue size. Relative to the first donor-grant revision, the time for registration and cancellation fell to 0.24× at one pending request, 0.17× at 64 and 0.17× at 1,024. Relative to the earlier revision without donor funding the corresponding ratios were 1.75×, 1.83× and 1.30×. Each process submitted and cancelled 32,768 futures against an already full heap. Actual collection had a separate 24-pair schedule per condition. Weakest median collection speedups were 11.69× native and 12.00× WASI. [Allocation cost and 95% intervals](paged-enrollment-cost.svg) · [Collection processes](enrollment-pages.csv) · [Validation and allocation samples](paged-enrollment-tests.json.gz)

## Allocation recovery with an independent donor

At `a9242931`, the two 32-byte donors filled a 64-byte heap. Four pending requests each needed eight bytes. A fresh full ticket completed while both reapers were parked before physical deallocation. The three competing requests remained pending, and a fifth request arriving after selection returned OOM. Releasing only the second donor let all four original requests acquire distinct rooted values. All 64 bytes remained accounted for until the first reaper resumed. Every run drained to zero. All four policy combinations passed in each of 100 fresh native processes. The original contention test also passed in all 1,800 fresh processes, compared with eight failing processes in its earlier run. [Forced schedules, original fixture reference and outcomes](retirement-grant-tests.json.gz)

Complete collections cleared all 36 conditions with 24 fresh matched pairs per condition. The weakest median speedups were 11.01× native and 10.99× WASI. On native, 499 default library tests and 1,142 all-feature tests passed. On WASI, 309 library tests passed, including cancellation on either side of payment and workerless help during partial selection. The bare Wasm build, strict all-target native Clippy and verified Cargo package passed. [Collection processes](retirement-grants.csv) · [Build and source identities](provenance.json) · [Validation and package hash](retirement-grant-tests.json.gz)

This correctness change increased allocation cost under pressure. The native comparison enqueued and cancelled 32,768 real allocation futures on an occupied eight-byte heap in each process, with 32 randomized process pairs per queue size. Median elapsed time relative to `42c37eb4` increased to 7.30× at one pending request, 10.86× at 64 and 7.59× at 1,024. The corresponding median times per request were 620, 862 and 939 ns. Reducing the cost of enrolment and cancellation remains an alpha task. [Timing distribution](retirement-grant-cost.svg) · [Fixture, samples and bootstrap intervals](retirement-grant-tests.json.gz)

Eligibility covers asynchronous requests enrolled at donor selection, direct global backing, and an exact match between layout size and charge. Finalizers, heap owners, clearing callbacks, drop glue, small-page and region storage, and deferred lease retirement have separate ownership rules. Additional qualification must cover those paths, synchronous callers, late arrivals during selection and the length of the wakeup scan on cooperative hosts.

## Earlier collection revisions

At `42c37eb4`, the lowest complete-collection medians were 11.01× native and 10.84× WASI. A native test held a registrar immediately before publication. A peer registered, cancelled an advisory request, observed a real refund and allocated the returned bytes before the registrar resumed. The parent timed out. A second test parked a refund after recording it for the current group. A later observer of the same size saw only the next refund. Each test and the cancellation ordering model passed in 100 fresh processes. Separate runs verified that 1,024 simultaneous 8-byte observers could share a group through repeated 1-byte refunds. The earlier independent-donor schedule still returned three terminal OOMs in all 400 attempts. [Collection processes](refund-groups.csv) · [Test patches, outcomes, timings and Cargo package identity](refund-observer-evidence.json.gz)

The allocation comparison used 32 randomized native pairs at each queue size. Each process submitted and cancelled 32,768 real allocation futures at their first suspension on an occupied 8-byte heap. Relative to parent `907522d7`, the median elapsed-time ratios were 0.981 at one pending request (95% interval 0.937–1.003), 0.775 at 64 (0.754–0.792) and 0.923 at 1,024 (0.909–0.945). The fixture measures enrolment and cancellation without servicing a full ticket. Refund delivery can still traverse multiple overlapping registration groups, and an observer records its opportunity when the collector announces the returned headroom. The measured interval does not bound time between physical return and that announcement.

![Relative time to enrol and cancel pressured allocations](refund-observers.svg)

At `2b324c42`, the weakest median speedups were 10.44× native and 11.75× WASI. With an 8-byte cap, a real allocation and an advisory request both received a refund during their first suspension. A competitor immediately took the bytes. The parent exhausted its recovery allowance after two tickets. The revised requests observed the refund and completed three tickets before returning OOM on the still-full heap. With no refund they completed two. Cancellation during the first yield released the observer and restored local allocation batching. The independent-donor case still requires a way to hand returned storage to existing requests. [All process results](first-yield.csv)

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
