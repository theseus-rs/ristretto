# Canonical runtime measurements — 2026-11-21

The restricted canonical runtime has four finite G1/ZGC invocation comparisons.
The first three used a dedicated Ristretto heap for each invocation. The latest
reuses one heap and mutator context across all warmup and measured calls, with
setup and final drain timed separately.

Evidence is retained in the sibling `ristretto-gc-evidence` directory, outside
Cargo build storage. Each linked report binds source, executed artifacts, inputs
and raw output. Earlier results lost with `target/` remain historical only; see
[the recovery record](EVIDENCE_RECOVERY.md).

## On-demand null storage on the persistent Java ring

Deferring physical field storage until its first non-null write shortened
invocations in all 18 small and medium pairs. All nine large pairs slowed.
Each cell below shows the median within-pair change in elapsed time across
three fresh process pairs. Each process used 100 warmups and 30 measured calls.
Negative values mean faster. The eager-storage version remains the baseline.

| Runtime | Small | Medium | Large |
|---|---:|---:|---:|
| Native interpreted | −4.11% | −2.97% | +5.18% |
| Native compiled | −1.39% | −0.63% | +0.70% |
| WASI interpreted | −3.69% | −3.80% | +4.45% |

All 1,620 measured calls matched the Java oracle. Every process reclaimed its
payload at teardown. The paired experiment measures whole invocations on the
restricted runtime. The first scheduled comparison with HotSpot appears below.
[All fork medians and resource ranges](../../../ristretto-gc-evidence/qualification-lazy-null-paired-v1/REPORT.md),
[paired changes](../../../ristretto-gc-evidence/qualification-lazy-null-paired-v1/paired-changes.png)
and [machine-readable pairs](../../../ristretto-gc-evidence/qualification-lazy-null-paired-v1/paired-ratios.csv).

The revised non-null write checks the source and field index before submitting
the candidate slot for final validation. Against the same eager baseline, this
version shortened 24 of 27 matched pairs. The three large native-compiled pairs all slowed, by
2.60–5.40%. The eager version is in the preview archive.

| Runtime | Small | Medium | Large |
|---|---:|---:|---:|
| Native interpreted | −3.52% | −2.08% | −9.17% |
| Native compiled | −3.22% | −0.42% | +3.22% |
| WASI interpreted | −4.62% | −1.76% | −7.07% |

Each cell is the median within-pair change in invocation time. All 54 processes
checked their Java results and final heap drain. A separate scheduling
experiment reduced superseded requests by 69–79%, while total helper polls
increased 4.8–6.5% on the large rings. That experiment used an earlier source
revision and measured work counts.
[Revised write measurements and graphs](../../../ristretto-gc-evidence/qualification-write-key-paired-v1/portable-report-v2/REPORT.md)
and [scheduling counts](../../../ristretto-gc-evidence/jit-helper-deferral-v1/ablation-summary-v1.json).

## Catalog owner validation on the persistent Java ring

Using the owner identity already retained by the control slot eliminated a
reference-count update on snapshot and preparation paths. On the restricted
Java ring it shortened 16 of 18 native process pairs and one of nine WASI
pairs. All 27 pairs matched on workload counts, helper polls, native slices
and measured collection-cycle bounds. The preview retains its original owner
validation. Each cell gives the median within-pair change in invocation time
across three process pairs, with 100 warmups and 30 measured calls per process.

| Runtime | Small | Medium | Large |
|---|---:|---:|---:|
| Native interpreted | −1.84% | −2.10% | −1.14% |
| Native compiled | −1.90% | −1.70% | −0.30% |
| WASI interpreted | +0.59% | +1.94% | +0.95% |

[Paired report and graph](../../../ristretto-gc-evidence/qualification-owner-identity-paired-v1/portable-report-v1/REPORT.md)
and [independent raw-capture replay](../../../ristretto-gc-evidence/qualification-owner-identity-paired-v1/paired-summary.json).

## One-minute medium-ring G1 baseline

G1 delivered all 983,040 offered calls in each of three independent 60-second
processes at 16,384 requests/s. All three observed collection. Each ended the
offer window with zero backlog. The scheduled response p99.9 histogram upper
bounds were 464, 457 and 457 µs. Each process used a 64 MiB Java heap, two
logical CPUs and a 512 MiB cgroup memory limit.

At 32,768 requests/s, G1 again delivered every offer by the deadline in three
60-second forks, each with 1,966,080 completions and collection activity.
Scheduled response p99.9 reached 0.612, 17.664 and 0.559 ms respectively. The
middle fork exceeded both the 1 ms response and 10 ms peak logical backlog
screens. Counting deadline completions alone would hide that result.

At 65,536 requests/s, all three forks observed collection and missed the delivery
deadline. Of 3,932,160 offers per fork, 580,991, 279,037 and 121,145 respectively
remained outstanding. By-deadline throughput ranged from 55,853 to 63,517/s.
The three tested rates bracket a one-minute delivery boundary between 32,768
and 65,536/s on this setup. Paired Ristretto/ZGC capacity and sustained latency
remain open. The earlier ten-second, 128/s pilot is reported separately.
[Audited 16,384/s forks](../../../ristretto-gc-evidence/delivered-capacity-peers-v1/point-audits/medium-g1-point00-rate16384/forks.csv),
[32,768/s forks](../../../ristretto-gc-evidence/delivered-capacity-peers-v1/point-audits/medium-g1-point01-rate32768/forks.csv)
and [65,536/s forks](../../../ristretto-gc-evidence/delivered-capacity-peers-v1/point-audits/medium-g1-point02-rate65536/forks.csv).

## Contention in the ordinary slot-root registry

With all 16 page writers held, the earlier native collector failed a three-second
deadline for acquiring a second handle and collecting unrelated garbage. The
revised collector uses its ordinary page whenever a writer is available. If
all writers are occupied, it prepares an address-free record in the same
registry, then publishes the validated address under root admission. Possession
of an old directory view gives no right to access an object.

The native parked-thread test passed for both publication policies and for
global and context-owned slots. A cooperative schedule on native and WASI
checked expired-handle cancellation, retention via the new records, duplicate
revocation, and reclamation with the writers held. In 72 untimed allocator
captures, all six context-slot workload/target combinations had zero median
paired change in host allocation calls and requested bytes for uncontended
operations. Timing and memory costs during simultaneous overflow still need
measurement. [Earlier failure](../../../ristretto-gc-evidence/ordinary-slot-overflow-v1/before-v2-native/0.log),
[isolated native checks](../../../ristretto-gc-evidence/ordinary-slot-overflow-v1/after-v3-native/results.json),
[WASI checks](../../../ristretto-gc-evidence/ordinary-slot-overflow-v1/after-v3-wasi/results.json),
[strict-provenance Miri](../../../ristretto-gc-evidence/ordinary-slot-overflow-v1/after-v3-miri/results.json)
and [paired allocator changes](../../../ristretto-gc-evidence/ordinary-slot-overflow-v1/allocation-report/paired-changes.csv).

The combined checkout also transfers the strong VM heap-owner reference into
the payload's destruction record. An obsolete descriptor can no longer retain
another root through that owner after the payload is destroyed. Shutdown
takes a pending reclaim task out of the cycle mutex before invoking its host
callbacks. The combined checks cover 606 GC, 780 classloader and one canonical
heap-owner case on native. The WASI subset covers 230 GC, four classloader and
one canonical heap-owner case. Both passed strict GC lint.
[Combined native run](../../../ristretto-gc-evidence/integration-slot-owner-v1/native/results.json),
[combined WASI run](../../../ristretto-gc-evidence/integration-slot-owner-v1/wasi/results.json)
and [owner-specific validation](../../../ristretto-gc-evidence/ordinary-announced-publication-v1/owner-validation-summary-v2.json).

## Ordinary numeric root registration

On native and WASI, a read view held across 4,096 ordinary collector steps
kept a dead allocation resident with the original default tree. The isolated
paged candidate reclaimed the dead allocation while the same view remained
held, preserved a separately rooted value, and returned managed bytes to zero
at teardown. The candidate stores only numeric addresses and checked root
generations in its pages. Readers can pin a retired page without retaining the
payload. This experiment exercises the root registry. An ordinary constructor
parked after publication admission can still prevent a collection.
[Original native witness](../../../ristretto-gc-evidence/ordinary-tree-root-pages-v1/before-native/0.log)
and [WASI witness](../../../ristretto-gc-evidence/ordinary-tree-root-pages-v1/before-wasi/0.log).

The revised registry keeps its most recent page outside the radix tree and
reuses an empty page's entry storage after its last observer drops it. It
passed 325 native and 224 WASI checks and strict lint for both. Three Loom
models exercised sealing, registration, reading and removal. Bypassing either
the sealing check or the generation recheck made the corresponding model fail.
Strict-provenance Miri passed the parked-view and ID-reuse cases.
[Native checks](../../../ristretto-gc-evidence/ordinary-tree-root-pages-v3/after-native/results.json),
[WASI checks](../../../ristretto-gc-evidence/ordinary-tree-root-pages-v3/after-wasi/results.json),
[checked models](../../../ristretto-gc-evidence/ordinary-tree-root-pages-v3/models/results.json)
and [Miri](../../../ristretto-gc-evidence/ordinary-tree-root-pages-v3/after-miri/results.json).

The table reports median within-pair changes relative to the original
default tree. Each of the two experiments ran three paired forks per fixture
on native and WASI, with 10,000 operations per process. The 144 captures include
the separately selected context-slot policy. Each process checked its checksum,
teardown, CPU quota and 512 MiB memory limit. These measurements cover system
allocator calls and requested bytes. Timing requires a separate comparison.

| Target | Fixture | First paged version, calls | Revised version, calls | Revised version, bytes |
|---|---|---:|---:|---:|
| Native | Ephemeral allocation | +4.38% | +1.83% | +0.68% |
| Native | Weak retention | +3.12% | +1.34% | +0.53% |
| Native | Construction batches | +4.23% | +3.38% | +1.37% |
| WASI | Ephemeral allocation | +3.76% | +1.35% | +0.21% |
| WASI | Weak retention | +2.68% | +1.04% | +0.24% |
| WASI | Construction batches | +4.24% | +2.68% | +0.66% |

The allocation-cost regression keeps this version out of the preview. The
helper that starts a collection, the slot-backed registries and constructor
publication have separate synchronization paths.
[Allocation graph](../../../ristretto-gc-evidence/ordinary-tree-root-pages-v3/allocation-report/host-allocations.png)
and [all process counts](../../../ristretto-gc-evidence/ordinary-tree-root-pages-v3/allocation-report/all-forks.csv).

## Ordinary Wasm collector in Chromium

Fifteen sandboxed Chromium processes ran the bare Wasm collector or an observer
control on one main-thread mutator. Each policy had five fresh processes, two
seconds of warmup, ten seconds at 10,000 scheduled requests/s and at most 64
request polls per turn. CPU affinity allowed both SMT siblings on one physical
core. The collector heap limit was 16 MiB and the cgroup memory limit was
512 MiB. The ranges below preserve all five processes for each policy.

| Policy | Reported completions / offered | Arrival p99.9 (ms) | Request-bearing turn work p99.9 (µs) | Largest heartbeat gap in each fork (ms) |
|---|---:|---:|---:|---:|
| Observer control | 500,000 / 500,000 | 4.832–4.928 | 6–10 | 6.595–7.895 |
| Default tree roots | 500,000 / 500,000 | 4.800–4.960 | 20–25 | 6.355–7.245 |
| Context roots, buffered allocation and overlapping collection | 500,000 / 500,000 | 4.832–4.928 | 20–26 | 7.205–7.755 |

The driver may finish a request that started before the deadline. All forks
reported zero unfinished requests and balanced checksums and teardown. Across
policies, peak cgroup-charged memory was 394.32–416.40 MiB. Heartbeat gaps
include browser and host scheduling. The experiment exercised the Rust
collector directly. Whole-JVM peer comparisons and sustained host
responsiveness require separate measurements.
[Fork observations](../../../ristretto-gc-evidence/bare-host-v12/ten-second-forks.csv),
[graphs](../../../ristretto-gc-evidence/bare-host-v12/ten-second-screen/browser.png)
and [independent revalidation](../../../ristretto-gc-evidence/bare-host-v12/ten-second-checks.json).

## First scheduled-arrival peer comparison

Each of 60 fresh processes ran the same medium Java ring for ten seconds after
100 warmup calls. Each call replaced 257 nodes. The scheduler offered requests
independently of completion to one serial consumer, with two logical CPUs and
512 MiB per process. The following values are median completions per second
**by the deadline** across three processes per runtime and offered rate.

| Runtime | 16 offered/s | 32 offered/s | 64 offered/s | 128 offered/s | Requests still outstanding at 128/s |
|---|---:|---:|---:|---:|---:|
| G1 | 16 | 32 | 64 | 128 | 0 |
| ZGC | 16 | 32 | 64 | 128 | 0 |
| Ristretto native compiled | 16 | 32 | 46.0 | 46.1 | 819 |
| Ristretto native interpreted | 16 | 32 | 53.5 | 53.5 | 745 |
| Ristretto WASI interpreted | 16 | 32 | 39.5 | 37.7 | 903 |

At 128 offered/s the median scheduled response reached the 3.211 s histogram
bucket on native compiled, 2.949 s on native interpreted and 3.572 s on WASI.
The median service-time buckets were 21.760, 18.688 and 26.112 ms respectively.
Of 36,000 offered requests, 26,976 completed by their deadlines and another
18 completed in flight afterward. Both HotSpot collectors served the highest
offered rate without backlog throughout these trials. No process reached the
10,000-completion floor. Sustainable capacity, p99.9/p99.99 qualification and
global pause coverage remain open. The costs include the Java runtime and GC.
[Completion and backlog graph](../../../ristretto-gc-evidence/qualification-scheduled-ring-v1/portable-report-v1/scheduled-pilot-overview.png),
[response distributions](../../../ristretto-gc-evidence/qualification-scheduled-ring-v1/portable-report-v1/scheduled-response-observations.png),
[all 60 forks and protocol limits](../../../ristretto-gc-evidence/qualification-scheduled-ring-v1/portable-report-v1/REPORT.md)
and [independent deadline check](../../../ristretto-gc-evidence/root-scheduled-deadlines-v1/independent-deadlines.json).

## Paired controller comparison

The experimental catalog now reserves two controller slots for a transaction
with effects. One holds the effects. The other contains both possible successor
phases and records the chosen outcome after the effects complete. This replaces
the separately allocated commit and cancellation successors. A quiet paired
comparison varied only that runtime implementation on the persistent Java ring.
Each pair used the same input, two-CPU/512-MiB envelope, 100 warmup calls and
30 measured calls. We ran three independent pairs per size and execution tier.

| Runtime | Small | Medium | Large |
|---|---:|---:|---:|
| Native interpreted | 11.41% shorter | 8.41% shorter | 6.59% shorter |
| Native compiled | 7.16% shorter | 6.63% shorter | 5.22% shorter |
| WASI interpreted | 7.54% shorter | 6.72% shorter | 4.87% shorter |

The table reports the median of the three within-pair changes in median invocation
time. Every one of the 27 pairs favored the candidate, and all 1,620 timed calls
produced the expected Java result. The medium native compiled median fell from
21.624 to 20.222 ms. This comparison leaves scheduled tails, sustainable capacity
and the G1/ZGC qualification gates open.
[All pairs and scope](../../../ristretto-gc-evidence/qualification-controller-paired-v1/REPORT.md),
[paired ratios](../../../ristretto-gc-evidence/qualification-controller-paired-v1/controller-paired-ratios.png)
and [raw capture manifest](../../../ristretto-gc-evidence/qualification-controller-paired-v1/result-manifest.json).

## Allocation requests

Immutable member links remove repeated constant-pool lookup allocation. Compiled
dispatch now borrows method names and descriptors. The isolated comparison keeps
the workload bytecode, admission, allocator driver and remaining runtime source
unchanged. All 18 configurations and 54 after-change invocations passed checksums,
execution counters and complete cleanup.

Median Rust allocator requests per full invocation at 257 ring replacements:

| Workload | Interpreted before | Interpreted after | Compiled before | Compiled after |
|---|---:|---:|---:|---:|
| Small | 22,322 | 5,897 | 34,323 | 10,289 |
| Medium | 24,410 | 6,497 | 37,419 | 11,417 |
| Large | 32,762 | 8,897 | 49,443 | 15,569 |

Native and WASI interpreted request counts match. Each median uses three
successive invocations, rather than independent process forks. Small-workload
requests fall about 74% interpreted and 70% compiled. Admission increases by
46 requests on both platforms; native compilation increases by two requests.
The report includes requested-byte costs and every preparation stage.

[Allocation graph](../../../ristretto-gc-evidence/jit-linked-members-v1/allocation-comparison-final/allocations.svg)
and [count report](../../../ristretto-gc-evidence/jit-linked-members-v1/RESULTS.md)
retain the scope and distributions. These counters exclude direct native
allocations, allocator rounding and WASI host allocations. They measure neither
RSS nor elapsed time. No latency improvement follows from these counts alone.
The graph can be regenerated with [the allocation plotter](plot_canonical_allocations.py).

## First invocation comparison

The first comparison uses the canonical implementation **before** the metadata
optimization above. It completed 45 timed forks plus 18 separate peer allocation
diagnostics in a coordinated quiet window. There are three independent forks for
each runtime and workload size, with 100 warmups and 30 measured invocations per
fork. All 1,350 measured results matched the independent workload model.

Median of the three fork medians, in milliseconds per invocation:

| Workload | HotSpot G1 | HotSpot ZGC | Native compiled | Native interpreted | WASI interpreted |
|---|---:|---:|---:|---:|---:|
| Small | 0.0257 | 0.0298 | 45.946 | 40.269 | 62.680 |
| Medium | 0.0781 | 0.0228 | 51.825 | 45.096 | 69.638 |
| Large | 0.0770 | 0.0826 | 84.639 | 79.897 | 124.708 |

The scopes differ: canonical invocations include dedicated-heap setup and cleanup;
HotSpot calls reuse a heap and can defer collection beyond the invocation clock.
Small peer batches recorded no GC-bean count increases. Other batches recorded
some increases, but those snapshots also include driver gaps. These measurements
do not isolate GC cost or establish collector superiority. Native compiled calls
were 6–15% slower than interpreted calls under the same lifecycle, which warrants
attribution of helper, frame and lifecycle costs before further optimization.

[Readable comparison graph](../../../ristretto-gc-evidence/qualification-ring-v1/first-comparison-log.svg),
[full report](../../../ristretto-gc-evidence/qualification-ring-v1/MEASUREMENTS.md)
and [frozen dataset receipt](../../../ristretto-gc-evidence/qualification-ring-v1/measurement-freeze.json)
include all samples and limitations. The [comparison protocol](RING_COMPARISON.md)
describes the build, launch, validation and plotting commands.

Each fork used two logical CPUs, a two-CPU quota and a 512-MiB cgroup charge cap;
HotSpot used a 64-MiB managed heap. Actual membership and limits were checked.
There were no recorded CPU-throttling or cgroup-memory events. Cgroup charged
memory is not process RSS, and affinity is not exclusive machine isolation.
Duration-derived call rates are not sustained throughput. Scheduled-load tails,
global pause coverage and endurance were not measured by this matrix.

## Optimized invocation comparison

The integration-v5 comparison now also completed all 63 processes and 1,350
timing samples. Inputs, harness, warmup/sample counts, fork order and resource
limits match the baseline. The separate quiet windows and 36 documented source
differences make this an integrated-source comparison, not a randomized paired
ablation.

| Workload | Native compiled ms (change) | Native interpreted ms (change) | WASI interpreted ms (change) |
|---|---:|---:|---:|
| Small | 45.692 (−0.55%) | 40.213 (−0.14%) | 60.892 (−2.85%) |
| Medium | 51.905 (+0.15%) | 44.839 (−0.57%) | 67.675 (−2.82%) |
| Large | 83.014 (−1.92%) | 79.073 (−1.03%) | 123.921 (−0.63%) |

The allocation reduction did not produce a large elapsed-time reduction. Three
forks and these small differences do not establish a causal speedup. Peer timings
also vary substantially between windows; all fork values and ranges are retained.
These results prioritize attribution of helper, frame, setup and cleanup costs.

[Before/after graph](../../../ristretto-gc-evidence/qualification-ring-optimized-v1/canonical-before-after.svg),
[updated G1/ZGC comparison](../../../ristretto-gc-evidence/qualification-ring-optimized-v1/optimized-comparison-log.svg)
and [full optimized report](../../../ristretto-gc-evidence/qualification-ring-optimized-v1/MEASUREMENTS.md)
retain the unequal peer lifecycles, limited GC exposure and descriptive scope.
The original baseline is preserved. Both timing versions exclude the subsequent
reference-array and process-resource observer changes.

## Invocation cost attribution

Nine source-bound diagnostic captures now pass independent raw-interval, input,
execution and cleanup validation: one cold invocation for each of three sizes on
native interpreted/compiled and WASI interpreted execution. Entry accounts for
94.87–99.69% of the instrumented invocation envelope. Within entry, request gaps
account for 48.35–58.71%, helper polls for 20.76–25.57%, and request polls for
16.26–21.36%. About 97.38–98.82% of gap duration follows a Pending request poll.

These captures contain 233,007–511,523 service events each. Their clocks and
recording disproportionately enlarge the request path; gaps include recorder,
runtime and scheduling work. They use a direct-host launch without the ordinary
comparison resource envelope. They neither isolate scheduler cost nor measure GC
pauses. The evidence prioritizes bounded request scheduling for investigation;
phase-only corroboration and an ordinary before/after comparison remain pending.

The [exclusive-cost graph](../../../ristretto-gc-evidence/jit-invocation-attribution-v1/fine-analysis-v1/exclusive-entry-costs.svg),
[phase graph](../../../ristretto-gc-evidence/jit-invocation-attribution-v1/fine-analysis-v1/phase-costs.svg)
and [report](../../../ristretto-gc-evidence/jit-invocation-attribution-v1/fine-analysis-v1/REPORT.md)
retain the raw traces, additive interval checks and explicit measurement limits.
The frozen source predates reference arrays and cast support. No timing claim is
transferred to the current integrated source.

The subsequent phase-only diagnostics ran 18 processes. In the nine instrumented
invocations, entry execution accounted for 95.92–99.76% of elapsed time.
The paired controls produced identical execution counts and checksums. Each
pair contains one observation on pre-scheduling-change source. WASI supplied
monotonic timestamps with unavailable clock resolution. These measurements
point to execution inside the entry method as the larger optimization target.
[Phase results](../../../ristretto-gc-evidence/canonical-phase-only-v1/phase-r3-results.json).

## Request scheduling under the ordinary invocation timer

The 54-process paired comparison completed 1,620 measured invocations on native
and WASI. Each pair compares the same workload and resource limits, with 100
warmups and 30 samples per process. Both versions perform one helper step for
each pending request poll. The candidate yields on the first pending result and
then after at most 16 poll/helper pairs for that request.

| Runtime | Small baseline → candidate (ms) | Medium (ms) | Large (ms) |
|---|---:|---:|---:|
| Native interpreter | 39.933 → 20.776 | 45.222 → 22.585 | 79.586 → 35.184 |
| Native compiler | 45.399 → 23.104 | 52.217 → 25.434 | 83.007 → 39.789 |
| WASI interpreter | 60.716 → 25.717 | 67.717 → 27.578 | 121.119 → 41.579 |

The cells show the median of the three fork medians. Median within-pair speedups
range from 1.93× to 2.25× on native and from 2.36× to 2.90× on WASI. The measured
scope includes setup, Java execution and complete reclamation for each invocation.
The benchmark used the integration-v6 baseline and varied only the runtime's
request scheduling in production code. End-to-end host-turn latency, queued
arrivals, global pauses and endurance require separate runs. This matrix
predates allocation by field layout and the recovery-wakeup fixes. All 27 pairs
improved and performed identical helper-poll counts. One small baseline WASI
process recorded two CPU throttle events totaling 276 µs over its lifetime.
All other processes recorded zero throttle events. Lifetime RSS from this harness
version is excluded from comparisons.
[Invocation times](../../../ristretto-gc-evidence/qualification-request-quantum-v1/analysis-v1/paired-invocation-costs.svg),
[all 27 paired ratios](../../../ristretto-gc-evidence/qualification-request-quantum-v1/analysis-v1/paired-duration-ratios.svg),
[full report](../../../ristretto-gc-evidence/qualification-request-quantum-v1/analysis-v1/REPORT.md)
and [dataset receipt](../../../ristretto-gc-evidence/qualification-request-quantum-v1/analysis-v1/freeze-receipt.json).

## Integrated peer comparison

The integrated scheduler and field-layout allocation ran 45 timing processes
and 18 independent peer allocation diagnostics. Every timed invocation
completed with the expected checksum. The table gives the median of three
independent fork medians, in milliseconds per invocation.

| Runtime | Small | Medium | Large |
|---|---:|---:|---:|
| G1 | 0.0281 | 0.0775 | 0.0765 |
| ZGC | 0.0298 | 0.0355 | 0.0770 |
| Native compiled | 19.436 | 21.471 | 34.136 |
| Native interpreted | 17.166 | 18.975 | 32.889 |
| WASI interpreted | 21.488 | 23.667 | 38.844 |

The timed Ristretto path includes per-call setup and complete reclamation.
HotSpot reuses its heap and can defer work until a later call. All 63 process
cgroups recorded zero throttling and zero memory events. The report includes
process-lifetime RSS and CPU for the guest launcher or Wasmtime host.
[Peer invocation graph](../../../ristretto-gc-evidence/qualification-ring-v8-v1/v8-comparison-log.svg),
[resource graph](../../../ristretto-gc-evidence/qualification-ring-v8-v1/v8-process-resources.svg)
and [full comparison](../../../ristretto-gc-evidence/qualification-ring-v8-v1/MEASUREMENTS.md).

## Peer comparison with a persistent Java heap

The 63-process comparison completed 45 timing forks and 18 separate allocation
diagnostics. The table gives milliseconds per invocation, as the median of
three independent fork medians. Each fork measured 30 calls after 100 warmups.
All 1,350 timed calls returned the expected checksum. Every Ristretto batch
observed completed primary collections and every heap refunded its catalog
charge at teardown.

| Runtime | Small | Medium | Large |
|---|---:|---:|---:|
| HotSpot G1 | 0.028577 | 0.078599 | 0.088897 |
| HotSpot ZGC | 0.027595 | 0.076683 | 0.084789 |
| Ristretto native compiled | 19.495054 | 21.504945 | 31.893539 |
| Ristretto native interpreted | 17.271641 | 18.487892 | 29.553735 |
| Ristretto WASI interpreted | 22.029872 | 23.417772 | 35.986961 |

Both native Ristretto tiers took substantially longer on all three workloads,
even with setup and drain outside the timed calls. Every process had two CPUs
and a 512 MiB memory limit. Ristretto's 64 MiB managed payload limit also has a
separate catalog charge and a workload-specific slot cap. Each peer had a
64 MiB Java heap. The costs include Java execution, representation and
collection. Scheduled-arrival latency and sustainable capacity require
separate measurements.
[Invocation graph](../../../ristretto-gc-evidence/qualification-shared-ring-v9-v1/shared-comparison-log.svg),
[process resource graph](../../../ristretto-gc-evidence/qualification-shared-ring-v9-v1/shared-process-resources.svg)
and [results and reproducibility record](../../../ristretto-gc-evidence/qualification-shared-ring-v9-v1/MEASUREMENTS.md).

In a separate diagnostic, pooled reference acquisition and release accounted
for 25.2% of native interpreted CPU samples and 21.5% of native compiled
samples across twelve full processes. A subsequent graph-root change reused
private backing across empty reads and cancelled requests. In each of three
repetitions on native and WASI, 10,000 empty reads went from 10,000 host
allocation calls to one, and the next 10,000 cancelled reads went from 10,000
to zero. The following successful read returned the expected child and
consumed the prepared backing. Whole-invocation timing of this change is open.
[CPU profile](../../../ristretto-gc-evidence/shared-ring-cpu-profile-v1/REPORT.md),
[native allocation measurements](../../../ristretto-gc-evidence/graph-root-preparation-v1/candidate-native.json)
and [WASI allocation measurements](../../../ristretto-gc-evidence/graph-root-preparation-v1/candidate-wasi.json).

Automatic allocation now reads the catalog occupancy token when scanning past
an occupied object or external root slot. Actual publication still compares
and installs the new owner. A separate diagnostic counted the pooled reference
operations while an allocator skipped 32 occupied slots and then published into
the 33rd. For object slots, 32 acquisitions and 32 releases fell to zero.
For root slots, 64 acquisitions and 64 releases fell to zero. Native and WASI
returned the same counts, preserved all 32 original identities and returned
all root permits. These counts isolate the scan and exclude publication.
[Before](../../../ristretto-gc-evidence/catalog-occupancy-probe-v1/before-pinned.json)
and [after](../../../ristretto-gc-evidence/catalog-occupancy-probe-v1/after-pinned.json).

A rooted edge read can now reuse the caller's retained source and check the
same catalog revision at completion. In a separate 10,000-read diagnostic,
the previous lookup incurred 60,000 pooled acquisitions and 60,000 releases.
The current keyed lookup incurred 50,000 of each and the rooted lookup used
by the graph runtime incurred 40,000 of each. In 10,000 live-root queries,
acquisitions and releases each fell from 20,000 to 10,000. Counts matched
on native and WASI with exact child identity and complete refunds.
[Baseline](../../../ristretto-gc-evidence/borrowed-edge-source-v1/baseline-measured.json)
and [current counts](../../../ristretto-gc-evidence/borrowed-edge-source-v1/candidate-measured.json).

## Reclamation on a shared Java heap

The reusable heap passed 90 native and 68 WASI VM checks. A task parked inside
the Java ring remained live while another made eight calls and advanced primary
collection. Both tasks returned the expected checksums and the 48-slot heap
reclaimed all 194 objects. Separately, 64 native processes each ran two Java
interpreters on independent OS threads. They reclaimed 18,432 objects in total.
Native compiled cancellation and reuse also passed. These finite tests cover a
restricted program and selected suspension points. General stalled-publication
coverage and scheduled-load qualification remain open.
[Integration results](../../../ristretto-gc-evidence/integration-v9/summary.json).

## Collector designs used for comparison

| Runtime | Collection and reference handling | Implication for this comparison |
|---|---|---|
| Ristretto experimental Java tier | Owning roots preserve live values across suspension. The catalog owns heap edges and applies insertion and deletion barriers. A reusable heap supports independently scheduled mutators. | Dedicated runs charge each invocation for full cleanup. A reusable heap can reclaim objects during subsequent calls. On threadless Wasm, application and collector work share executor turns. |
| HotSpot G1 | Concurrent marking identifies live data. Evacuation pauses copy surviving objects out of selected regions. | Record pause count, pause duration and application response time at the same load. [Oracle G1 guide](https://docs.oracle.com/en/java/javase/25/gctuning/garbage-first-g1-garbage-collector1.html) |
| HotSpot ZGC | Generational collection runs the expensive work concurrently. Allocation needs enough headroom for the collector to keep up. | Match CPU and memory budgets and measure allocation stalls along with pauses. [Oracle ZGC guide](https://docs.oracle.com/en/java/javase/24/gctuning/z-garbage-collector.html) |
| Azul C4 | A loaded-value barrier supports marking, relocation and remapping while application threads run. | This is an architectural comparison. The timed peer matrix covers HotSpot. [Azul C4 documentation](https://docs.azul.com/prime/c4-garbage-collection.html) |
| Go | A non-moving concurrent mark-and-sweep collector combines write barriers, per-processor allocation areas and mutator assistance. Sweep termination and mark termination involve global stops. | A Go port would measure a different compiler, object representation and runtime. The present data supports a design comparison only. [Go runtime source](https://go.dev/src/runtime/mgc.go) |

## Recorded cycle windows and integration

Passive primary-cycle observations now bracket the service recorder's actual
entry clock boundaries. The checker independently validates conservative cycle
bounds from the four observations. Missing or ambiguous evidence remains
unavailable; it is never converted to zero cycles. Exact counts are emitted only
when the lower and upper bounds agree.

Six actual graph/array captures pass across native interpreted/compiled and WASI
interpreted execution. The graph records 19 completed primary full epochs in each
interpreted entry window and 10 compiled; the array captures record zero. These
are instrumented executions with different schedules, not performance comparisons.
Primary epochs do not stand for every GC domain, object reclamation, or global
pause coverage. Ordinary timed ring binaries have recording disabled and retain
unavailable cycle counts.

The combined observer, cycle-window and metadata checkpoint passes 61 native and
32 WASI tests, with scoped strict lint and formatting. Source and execution
receipts are in
[integration-v5](../../../ristretto-gc-evidence/integration-v5/summary.json).
This validation is separate from the earlier timing baseline and isolated
allocation comparison; their results are not attributed to one common binary.

Reference arrays are now integrated in both native tiers and the WASI interpreter.
The merged source passes 72 native and 41 WASI checks and strict library/test lint
on both platforms, including the 34-row reference-array oracle and 2,048-element
cases. The separate frozen array checkpoint passes six strict Miri checks,
including full-size pending cancellation, plus eleven ASan and eleven TSan checks.
Its separate finite revision passes one Miri and six Tree Borrows checks,
with cancellation stages scoped to a nine-element tree; production code is
identical. See [the memory validation report](../../../ristretto-gc-evidence/vm-reference-arrays-v1/REPORT.md),
[integration-v6](../../../ristretto-gc-evidence/integration-v6/summary.json) and
[the array contract](../../ristretto_vm/CANONICAL_JAVA.md).

Closed-layout casts are also integrated. The combined source passes 82 native VM
checks, 62 executed WASI checks and three focused request-only compiler checks,
plus strict library/test lint on both targets and formatting. Casts preserve
identity and owning roots; incompatible admitted values report a type fault.
The restricted tier still excludes general exception handling, static/class
initialization roots and general calls. The source-identical isolated cast
checkpoint retains four Miri, four Tree Borrows, nine ASan and nine TSan checks.
See [the integrated receipt](../../../ristretto-gc-evidence/integration-v7/summary.json)
and [cast validation scope](../../../ristretto-gc-evidence/vm-checkcast-v1/REPORT.md).

## Process resource receipts

Schema-2 ring launches now bind child lifetime RSS/CPU observations to the exact
PID, unique run ID, source and requested watchdog. Twenty focused tests and five
actual native interpreted/compiled, WASI, G1 and ZGC checks pass. Rehashed receipts
with a stale run ID or wrong finite watchdog are independently rejected. These
checks use development artifacts alongside other work and establish no RSS budget
or performance result. The [resource contract](PROCESS_RESOURCES.md) explains
launcher/exec, descendant accounting and Wasmtime host scope; metrics are neither
measurement-window peaks nor simultaneous process-tree totals. The older timing
datasets retain unavailable RSS evidence. See
[the resource validation receipt](../../../ristretto-gc-evidence/process-resources-v2/summary.json).

## Work per child read

A child read can submit the key of the observed target directly for root
creation. Activation still validates the source, edge and target. A root
barrier now takes an owning membership reference only when the barrier
needs to publish an effect.

The paired graph fixture ran 50 rounds per process, twice per capacity and
implementation on native and WASI. The poll counts matched across repeats
and platforms. Every process reclaimed 100 objects, 150 roots and 200
materialized edges, with zero remaining payload or catalog charge.

| Object slots | Read polls before | Read polls after | Helper polls before | Helper polls after |
|---:|---:|---:|---:|---:|
| 2 | 954 | 653 | 16,195 | 16,390 |
| 8 | 946 | 657 | 17,479 | 17,599 |
| 64 | 770 | 657 | 41,106 | 40,534 |

Read polls fell by 14.7–31.6%. Total helper polls increased at the two smaller
capacities and decreased at 64 slots. These operation counts measure this
fixture. The processes shared the host with development builds.
[Source changes and run plan](../../../ristretto-gc-evidence/borrowed-barriers-v1/graph-retry/graph-plan.json)
and [all 24 process results](../../../ristretto-gc-evidence/borrowed-barriers-v1/graph-retry/graph-results.json).

## Remaining release gates

The restricted source preview is packaged and locally verified. It includes the
validated eager-field runtime, a runnable Java cycle fixture, the supported
bytecodes and limits, and benchmark data. Two independent packagings produced
identical archive hashes. The native interpreter, native compiled tier and WASI
interpreter returned 64 and reclaimed all 16 objects from the extracted source.
The extracted package passed all 13 Java admission, execution and cleanup tests
on both native and WASI. Validation used Linux x86-64, Rust 1.99.1, a cached
dependency registry and Wasmtime 50.0.1. A public release has not been made.
The extracted source also passed 284 native and 189 WASI ordinary GC library
tests. Cargo's default Java launcher dependency graph excludes the experimental
catalog and pool on both targets. The review checkout reproduces all 2,677
original source paths, contents and executable flags in the package.
[Source archive](../../../ristretto-gc-evidence/restricted-runtime-preview-v1/ristretto-experimental-gc-preview.tar.gz),
[reader guide](../../../ristretto-gc-evidence/restricted-runtime-preview-v1/unpacked/ristretto-experimental-gc-preview/PREVIEW.md),
[verification receipt](../../../ristretto-gc-evidence/restricted-runtime-preview-v1/milestone.json)
and [review guide and source identity](../../../ristretto-gc-evidence/restricted-runtime-preview-v1/REVIEW.md).

| Milestone | Remaining evidence or implementation |
|---|---|
| Restricted experimental source preview | Review and commit the frozen source, tag that commit and distribute the verified archive and checksum. The review patch applies to base commit `59d46079` and reproduces the candidate tree. For crates.io delivery, resolve the path-only catalog and pool dependencies and pass Cargo packaging checks. |
| Integrated VM alpha | Broader VM layouts and semantics, shared-heap/mutator ownership, native boundary coverage, and integration regressions. |
| Performance-qualified beta | Profile and reduce runtime costs; repeat comparable measurements; complete pause, RSS/CPU and scheduled-load coverage. |
| Full release | Sustained-load and 1-hour/24-hour endurance gates, including contention and reclamation progress, on the supported native/Wasm paths. |

Zero collector-wide suspension and better throughput/latency than G1 or ZGC have
not been demonstrated. No default backend switch follows from these measurements.
