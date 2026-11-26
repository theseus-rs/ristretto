# GC and whole-JVM performance improvement plan

## Tenfold complete-collection gate

At `9e7e3580`, two full collection tickets can finish while a regional maintenance lock remains parked. Accessible empty backing is physically reclaimed, and a later collection reclaims the deferred shard. All 600 forced regional processes and all 36 complete-collection comparisons passed. The lowest individual 95% speedup bounds were 10.41× native and 10.09× WASI. The default native suite still exposed incomplete destructor counts and two indexed survivors after two explicit full collections. Finite exhaustion with inaccessible regions, explicit callback completion, VM ownership, host-pause coverage and sustained-load qualification remain open. [Controls, measurements and outstanding failures](../benchmarks/gc/publication/typed-root-collection/consolidated-collection-followups/README.md#regional-allocation-and-full-collections-with-a-parked-registry).

At `113da93b`, a waiting buffer allocation helps already released storage at the heap limit while preserving credit assigned to other requests. Workers can reclaim buffers whose elements have no destructor while an unrelated host callback is parked. All 1,300 focused processes, 30 four-mutator stress processes and 36 complete-collection conditions passed. The weakest per-condition 95% bounds were 10.40× native and 10.23× WASI. The full default native suite still observed incomplete destructor counts immediately after two explicit collections, as did the preceding public source. Physical completion for explicit callers, async regional allocation, VM boundaries, pause coverage and endurance remain open. [Controls, stress and full-collection graphs](../benchmarks/gc/publication/typed-root-collection/consolidated-collection-followups/README.md#returning-buffer-storage-to-waiting-allocations).

At `cdee5888`, native Java Reference operations and the collector agree on the declaring referent field when subclasses hide it. The native Java 8 and 25 queue tests, Java 25 WASI launcher check, 104 repeated processes and existing 40-configuration native reference lifecycle passed. The complete-collection gate passed all 36 conditions with lowest individual 95% bounds of 10.86× native and 10.61× WASI. Hard-cap recovery for arbitrary host cleanup, further shared and compiled VM boundaries, whole-host pause coverage, sustainable peer comparisons and endurance remain open. [Controls and full-cycle results](../benchmarks/gc/publication/typed-root-collection/consolidated-collection-followups/README.md#java-reference-fields-hidden-by-subclasses).

At the same source revision, a 105-process Java roots comparison found that Ristretto passed 1,000 requests/s on the first five forks. Two of five confirmation forks exceeded the 1 ms scheduled p99.9 limit. G1 passed through the search ceiling of 128,000/s and ZGC confirmed a 48,000/s pass and a 64,000/s failure. The collector improvement and these short whole-VM results leave sustained Java performance and whole-host pause qualification open. [Limits, graph and individual forks](../benchmarks/gc/publication/typed-root-collection/consolidated-java-followups/roots-capacity/README.md).

At `a0eba4e2`, parked native finalizers, clearing hooks and ordinary destructors allow an independent scalar or bounded buffer to be physically reclaimed. The 2,300 forced schedules and all 36 complete-collection conditions passed, with weakest individual 95% lower bounds of 10.39× native and 10.50× WASI. With 1,024 cheap finalizers and a sleeping requester, physical completion took 28.2% less time with one worker and 32.4% less with two than on `8f0311bf`. A donor requiring a second arbitrary destructor behind a parked one still returned out-of-memory. VM boundary integration, whole-host pause coverage, sustained Java comparisons and endurance remain open. [Cleanup schedules, benchmark and graphs](../benchmarks/gc/publication/typed-root-collection/consolidated-collection-followups/README.md#native-collection-during-parked-cleanup).

The same source completed all requests in twelve fixed-rate Java roots runs. At 1,500 requests/s the median scheduled p99.9 across two forks was 2,992 µs for Ristretto, 58.5 µs for G1 and 111.5 µs for ZGC. The shared-host comparison leaves sustainable capacity and whole-host pause attribution open. [Per-fork memory and response tails](../benchmarks/gc/publication/typed-root-collection/consolidated-java-followups/native-cleanup/README.md).

At `b7eb6c4b`, a young collection completes while the only primary remembered-set writer remains parked, and exact-ID retirement preserves the replacement at a reused address. The 1,100 forced native processes and 36 complete-collection conditions passed. Weakest individual 95% speedup bounds were 10.278× native and 10.580× WASI. A separate native comparison timed collections with the requester asleep. Two-worker medians took 0.770–0.952× the previous source time, and one-worker medians took 0.956–1.048×. Finalizable storage with all native workers inside user cleanup, VM boundary coverage, host-wide pauses and sustained-load qualification remain open. [Controls, workers and complete-collection data](../benchmarks/gc/publication/typed-root-collection/consolidated-collection-followups/README.md#collection-with-a-parked-remembered-set-publisher).

At `559296ae`, buffered owner writes schedule a fresh traversal after an earlier trace read the previous fields. Both drain modes retain the young child, and 500 forced processes passed. The 36-condition complete-collection gate passed with weakest individual 95% lower bounds of 10.096× native and 10.536× WASI. Primary remembered-set contention, finalizable storage, VM boundaries and whole-host qualification remain open. [Regression and complete-collection results](../benchmarks/gc/publication/typed-root-collection/consolidated-collection-followups/README.md#retracing-a-buffered-write-after-an-earlier-traversal).

At `e920e1d3`, deferred deletion uses the original birth identity, and clean marking and prepared dirty enrollment can progress while the object's shard is parked. All 700 forced schedules and 36 collection conditions passed. The weakest individual 95% speedup bounds were 10.61× native and 10.65× WASI. The native legacy embedded-edge barrier used 60.1% less time. The native unbuffered counted barrier for one repeated handle used 9.4% more. The remaining alpha work includes stalled remembered-set publication, finalizable storage and VM boundaries. Host-wide pauses, sustainable Java performance and endurance remain unqualified.
[Identity controls, mechanism costs and full-cycle measurements](../benchmarks/gc/publication/typed-root-collection/consolidated-collection-followups/README.md#exact-identity-during-child-deletion-and-deferred-marking).

The twelve-fork Java roots comparison at `480d279a` completed every scheduled request. At 1,500/s the median scheduled p99.9 was 958 µs for Ristretto, 103.5 µs for G1 and 10 µs for ZGC. Ristretto’s two forks measured 252 and 1,664 µs. The rates leave sustainable throughput and latency superiority open.
[Process-level comparison and graph](../benchmarks/gc/publication/typed-root-collection/consolidated-java-followups/synchronized-methods/README.md).

At `480d279a`, synchronized Java methods retain their receiver while waiting for a monitor and through release. The four return/cancellation cases passed on native and WASI. All 1,008 enabled native VM tests passed and three were ignored. The complete-collection comparison passed all 36 conditions, with weakest individual 95% speedup bounds of 10.03× native and 10.50× WASI. Deletion-barrier identity, broader VM boundaries, pause telemetry and sustained comparisons remain open.
[Synchronized receiver regression and collection measurements](../benchmarks/gc/publication/typed-root-collection/consolidated-collection-followups/README.md#synchronized-method-receiver-lifetime).

At `43ad8138`, peers can deliver allocation and ticket notifications across a parked waiter publisher, executor migration, and the last worker’s exit. All 1,700 forced schedules and 36 complete-collection conditions passed. The lowest individual 95% speedup bounds were 10.14× native and 10.81× WASI. The largest queue improved on enrollment and repeat polls, while executor changes at one and 64 waiters still cost more than the parent. Region-backed or finalizable storage, deletion during address reuse, ordinary VM root ownership, and whole-host qualification remain open.
[Waiter controls, cost intervals and full-cycle measurements](../benchmarks/gc/publication/typed-root-collection/consolidated-collection-followups/README.md#allocation-waiters-across-executor-changes-and-worker-exit).

At `d07426d1`, three supported lock wrappers forward cooperative clearing of their contents. Each had previously stranded an unrelated allocation at the heap limit. All 900 forced native runs and all 36 complete-collection comparisons passed, with weakest individual 95% speedup bounds of 10.43× native and 10.63× WASI.
[Locked-buffer controls and collection measurements](../benchmarks/gc/publication/typed-root-collection/consolidated-collection-followups/README.md#cooperative-clearing-of-locked-buffers).

A withheld allocation-waiter experiment at `53c3f961` passed 1,400 forced native schedules and all 36 complete-collection comparisons. A later test stranded a ticket-only waiter after the last worker exited. Three executor polls plus cancellation took 1.52–2.38× as long as the release source. Both issues remain on the integrated alpha critical path.
[Experiment and reproducible controls](../benchmarks/gc/publication/typed-root-collection/consolidated-collection-followups/README.md#withheld-allocation-waiter-experiment).

At `89c0f698`, a compiled helper returning a retired same-heap edge reports a VM error before caching the reference. The parent panicked on the same input. All 36 complete-collection conditions cleared 10×, with weakest individual 95% speedup bounds of 10.77× native and 10.59× WASI. Allocation-waiter progress, shared VM ownership, native boundaries and broader compiled coverage remain open.
[Retired-edge regression and measurements](../benchmarks/gc/publication/typed-root-collection/consolidated-collection-followups/README.md#retired-same-heap-compiled-helper-results).

At `8b15e59c`, all 12 Java roots forks completed their offered load. At 1,500 requests/s, the median scheduled p99.9 was 979 µs for Ristretto, 286.5 µs for G1 and 22.5 µs for ZGC. The repeat used a fresh checkout after the original run failed its input-integrity check. Two forks per rate leave sustainable capacity and extreme-tail latency open.
[Java process measurements and excluded run](../benchmarks/gc/publication/typed-root-collection/consolidated-java-followups/README.md).

At `8b15e59c`, compiled Java helper results validate the owning heap before publishing a reference into the VM frame cache. On the parent a generated array load returned a foreign heap's child. Both helper entry points now reject it and retain the original foreign value. The native VM suite passed 1,006 tests with three ignored, and all 36 collection conditions exceeded 10×.
[Generated array control and measurements](../benchmarks/gc/publication/typed-root-collection/consolidated-collection-followups/README.md#compiled-helper-result-heap-ownership).

At `f350e347`, optional values also forward cooperative clearing. A parked region registry no longer strands an independent donor in the optional-buffer regression. Eight forced schedules passed in 800 fresh processes. All 36 complete-collection conditions cleared 10×, with weakest individual 95% speedup bounds of 11.02× native and 10.50× WASI. The allocation-waiter index, other composite layouts and shared VM ownership remain open.
[Optional buffer regression and collection measurements](../benchmarks/gc/publication/typed-root-collection/consolidated-collection-followups/README.md#cooperative-clearing-through-optional-values).

At `ed3a9426`, native retirement of a regional descriptor or direct Java array buffer can retry its range return while the worker reclaims an independent global donor. Seven contention schedules passed across 700 fresh processes. All 36 complete-collection conditions exceeded 10× against `f1daebeb`, with weakest individual 95% bounds of 10.58× native and 10.28× WASI. A wrapped regional buffer and a parked allocation-waiter index still block independent work. Shared VM roots, native boundaries and compiled execution remain on the integrated alpha critical path.
[Collection results and remaining schedules](../benchmarks/gc/publication/typed-root-collection/consolidated-collection-followups/README.md#cooperative-regional-descriptor-and-java-array-returns).

At `0a7993f0`, waiting allocations reclaim the same returned donor while its
first selector is parked. One helper wins physical release and an unrelated
publication can only postpone directory cleanup. Ten regressions passed in
100 fresh native processes each. All 36 complete-collection conditions passed
against `f1daebeb`, with weakest individual 95% lower bounds of 10.17× native
and 10.60× WASI. Relative to `ef490d09`, full-heap registration and
cancellation took 0.978×, 0.972× and 1.002× as long at one, 64 and 1,024
pending requests. The parked allocation-waiter index remains a reproduced
alpha blocker. Region-backed and finalizable storage, VM boundaries, and
whole-host pause and endurance evidence remain open.
[Schedules, regressions, samples and graphs](../benchmarks/gc/publication/typed-root-collection/consolidated-collection-followups/README.md#helping-a-stopped-selector-after-lease-release).

At `ef490d09`, synchronous allocation at the logical heap limit can reclaim
unreachable storage, including when a registered native worker is parked.
All 36 complete-collection conditions passed against `f1daebeb`. The weakest
individual 95% lower speedup bounds were 10.88× native and 10.95× WASI.
Three contention tests passed in 100 fresh native processes each. A separate
parked waiter-index test timed out before an unrelated full collection could
finish. Wake delivery, leased and deferred storage, and integrated VM
ownership remain on the alpha critical path.
[Complete-collection data and progress tests](../benchmarks/gc/publication/typed-root-collection/consolidated-collection-followups/README.md#synchronous-allocation-recovery).

At the same revision, all three JVMs completed every scheduled Java roots request
at 1,000/s and 1,500/s in two fresh processes per rate. At 1,500/s the median
scheduled p99.9 was 258.5 µs for Ristretto, 165.5 µs for G1 and 133.5 µs for
ZGC. At 1,000/s the corresponding values were 156.5, 144.5 and 102.5 µs. The
fixed-rate comparison leaves sustainable throughput and superiority open.
[Java process data and graph](../benchmarks/gc/publication/typed-root-collection/consolidated-java-followups/README.md).

At `5c05cc39`, ordered enrollment pages reduced full-heap registration and
cancellation time by 76–83% relative to `a9242931`. With one, 64 and 1,024
pending allocations, the remaining cost relative to `42c37eb4` was 1.75×,
1.83× and 1.30×. A peer can commit a stopped registrar's fully initialized
position. The donor cutoff excludes later enrollments. All 36 complete-
collection conditions passed against `f1daebeb`, with weakest individual 95%
lower speedup bounds of 11.01× native and 11.38× WASI. The original
1,800-process contention campaign and 400 forced four-allocator schedules
passed. Synchronous cap recovery, deferred storage retirement, and wake
delivery with a parked waiter index still need qualification.
[Three-way timings, races and collection results](../benchmarks/gc/publication/typed-root-collection/consolidated-collection-followups/README.md#paged-enrollment).

At `a9242931`, four asynchronous allocations sharing an independent 32-byte
donor retained their eligibility across fresh collection tickets. All 400 forced
native schedules passed, as did all 1,800 fresh processes replaying the earlier
four-allocator contention test. Each forced schedule also checked that a fifth
request arriving after both donors were private returned OOM. All 36 complete-
collection conditions passed against `f1daebeb`. The weakest individual 95%
lower speedup bounds were 10.53× native and 10.63× WASI.

Enrolling and cancelling a suspended allocation on a full heap took 7.30×,
10.86× and 7.59× as long as on `42c37eb4` with one, 64 and 1,024 pending
requests. The next alpha work is to reduce that cost, bound the scan for eligible
wakeups, and qualify synchronous allocation, other storage retirement paths and
the integrated VM. The grant currently applies to asynchronous requests for
storage released directly to the global allocator, with exact matching charges
and no cleanup callbacks. [Process measurements, eligibility and scope](../benchmarks/gc/publication/typed-root-collection/consolidated-collection-followups/README.md#allocation-recovery-with-an-independent-donor).

At `42c37eb4`, a parked allocation registrar permits peer enrolment, cancellation
and spending returned bytes with no completed full ticket. All 36 complete-
collection conditions passed against `f1daebeb`. The weakest individual 95%
lower speedup bounds were 10.37× native and 10.73× WASI. In a separate
comparison with its parent, enrolling and cancelling 64 or 1,024 pending
allocations took less time. The single-request interval included parity. The
independent-donor failure persisted in all 400 forced schedules on this revision.
[Benchmarks, forced schedules and verified package](../benchmarks/gc/publication/typed-root-collection/consolidated-collection-followups/README.md).

At `2b324c42`, allocating and advisory requests register for fitting refunds
before their first cooperative yield. A forced refund stolen during that yield
preserves the retry allowance, and immediate cancellation restores context
batching. All 36 complete-collection conditions passed against `f1daebeb`,
with weakest individual 95% lower bounds of 10.09× native and 11.62× WASI.
The independent-donor hard-cap failure remains open.
[Process results and test outcomes](../benchmarks/gc/publication/typed-root-collection/consolidated-collection-followups/README.md).

At `b041f0c3`, all 36 complete-collection cells passed against `f1daebeb`.
The weakest 95% lower bounds were 10.55× native and 11.76× WASI. The
previous integrated revision missed one native condition at 9.88×. Per-lane
mature strong-mark counts preserve the sweep coverage check. In 200 forced
two-lane schedules the collection ticket stayed open while one lane had
marked targets but had yet to publish its tally.

Shared contexts can spend prepaid bytes with a parked range cache and wake
after a real refund while a collector is paused in tracing. A space observer
keeps refills visible through cancellation, stolen refunds and context changes.
The native default and all-feature GC suites, WASI tests, bare Wasm build,
strict Clippy and verified Cargo package passed. The independent-donor
hard-cap failure below remains open.
[Measurements, process receipts and package identity](../benchmarks/gc/publication/typed-root-collection/consolidated-collection-followups/README.md).

At `74275197`, a parked target address shard permits clean buffered edge
publication during an explicit context flush, a mutator overflow and collector
draining. The forced schedules retain the graph and keep a privately claimed
record pending until its mark work publishes. The collector cleared all 36
native and WASI conditions against `f1daebeb`. The weakest individual 95%
lower bounds were 10.11× and 11.78×.
[Per-process results and forced schedules](../benchmarks/gc/publication/typed-root-collection/consolidated-collection-followups/README.md).

The earlier hard-cap recovery failure blocked the integrated alpha. With one reaper parked and
an independent 32-byte donor available to four 8-byte requests, 8 of 1,800
fresh native processes returned at least one terminal OOM. After all four peers
finished, each failing process had enough headroom for its missing roots before
the original reaper resumed. The release test must exercise competing waiters,
retain their successful roots and account for both donors.
A subsequent forced schedule held both donors before physical release. A
fresh full ticket completed, then the three competing requests returned OOM
while all 64 bytes were still charged. Resuming only the second reaper left
24 bytes free after its own allocation. This ordering reproduced under all
four policies in 100 fresh native processes.
[Forced schedules and process receipts](../benchmarks/gc/publication/compacting-endurance/region-epochs/README.md).

A separate shared-context experiment passed the 10× timing gate but could
return OOM for 8 bytes while 4,088 prepaid bytes remained in the same
context. A forced refill between cache lookup and global reservation reproduced
the failure. The corrected shared-interest path and its wake-driven tests are recorded above.
[Reproduction and timings](../benchmarks/gc/publication/typed-root-collection/consolidated-collection-followups/README.md).

At `b041f0c3`, all three JVMs completed every scheduled Java roots request in two
fresh forks per rate. At 1,000/s, median scheduled p99.9 was 132 µs for
Ristretto, 166 µs for G1 and 140.5 µs for ZGC. At 1,500/s the corresponding
tails were 259, 137 and 127.5 µs. Peak cgroup memory across both rates was
255.6 MiB for Ristretto, 62.4 MiB for G1 and 96.7 MiB for ZGC.
[Process records, distributions and graph](../benchmarks/gc/publication/typed-root-collection/consolidated-java-followups/README.md).

At `74275197`, all three runtimes completed all arrivals. At 1,500/s the
median scheduled p99.9 was 229.5 µs for Ristretto, 211 µs for G1 and
156.5 µs for ZGC. Both HotSpot collectors also had lower tails at 1,000/s. The
JIT-enabled diagnostic interpreted every one of 35,891 entries to
`GcBench.operation`. Its first production rejection was `putstatic` at byte
27. Pinning the physical reaper before freeing the only 32-byte allocation
also reproduces terminal OOM for four waiting peers under both publication
and phase policies. Stable reclamation under contention, execution coverage
and GC-attributable global-pause evidence remain release requirements.
[Java latency](../benchmarks/gc/publication/typed-root-collection/consolidated-java-followups/README.md)
· [JIT coverage](../benchmarks/gc/publication/typed-root-collection/ordinary-roots-jit-coverage/README.md).

Three Chromium workers running the same `74275197` guest each completed 13
collections and reclaimed 10,473,075 bytes. The workers replied to 256 of 257
scheduled pings during Java execution and answered the last ping after the
window. Attributing pauses to GC still requires instrumentation.
[Worker timings and raw captures](../benchmarks/gc/publication/browser-worker-reclamation/consolidated-alpha/README.md#clean-buffered-barriers-at-74275197).

The preceding qualified collector, `1bd9360c`, admits a copy to an available
destination and continues partial chunks while the source registry is parked.
Its minimum per-condition 95% speedup bounds against `f1daebeb` were 10.18×
native and 11.91× WASI over 1,728 fresh processes. Separate one-minute
compacting trials on `81594aad` met the operation tail limits, while a repeated
hard-cap recovery regression returned false OOM in 2 of 600 fresh processes.
[Collection results](../benchmarks/gc/publication/typed-root-collection/consolidated-collection-followups/README.md)
· [Compaction and hard-cap results](../benchmarks/gc/publication/compacting-endurance/region-epochs/README.md).

The integrated region changes at `81594aad` passed all 36 native and WASI
collection conditions against `f1daebeb`. Minimum medians were 10.42× and
11.81×, and the weakest individual 95% lower bounds were 10.18× and
11.68×. The compaction-specific tests cover activation and invalidation while
a region registry is parked, exclusion of stale roles, source release accounting
and bounded helper polls under both publication policies. The final trim test
timed out on its parent and passed with the change. The core benchmark runs
without compaction. The Chromium receipts cover the earlier `849b9f31` source.
[Collection measurements](../benchmarks/gc/publication/typed-root-collection/consolidated-collection-followups/README.md)
· [Chromium worker runs](../benchmarks/gc/publication/browser-worker-reclamation/consolidated-alpha/README.md).

The earlier integrated source at `849b9f31` clears every native and
WASI cell against `f1daebeb`. Its lowest medians were 10.92× and 12.22×,
and its lowest individual 95% bounds were 10.19× and 11.96×. Two
intermediate revisions missed the native gate after the deadline-aware
registry was integrated. The published follow-up series includes those runs.
A one-unit poll now removes at most one retained phantom address and a
parked statistics reader permits root and barrier progress across both phase
boundaries. The existing progress and sustained-load gates still apply.
[Collection follow-ups](../benchmarks/gc/publication/typed-root-collection/consolidated-collection-followups/README.md)
· [Java follow-ups](../benchmarks/gc/publication/typed-root-collection/consolidated-java-followups/README.md).

The integrated collector at `280ad63d` passed all 36 conditions against
`f1daebeb` with individual 95% lower bounds of at least 10.69× native and
10.97× WASI. Each condition used 24 fresh process pairs. A separate Java run
completed every request at 1,000 and 1,500 requests/s. Both HotSpot collectors
had lower median scheduled p99.9. Work continues on reclamation under
contention, global-pause coverage and sustained-load qualification.
[Integrated collection results](../benchmarks/gc/publication/typed-root-collection/consolidated-complete/README.md)
· [Java results](../benchmarks/gc/publication/typed-root-collection/consolidated-java/README.md).

The experiment at a3fef08c cleared 10× against f1daebeb in every collection
condition on the qualified host. Over 24 randomized pairs in each of 36 cells,
the minimum native median and individual 95% lower bound were 10.98× and
10.23×. WASI reached 11.44× and 11.34×. All 1,728 processes passed, including
mixed-cycle physical refunds. Independent builds of the published source
reproduced both measured executables. The integrated follow-up results are recorded above. [Graphs, inputs and
reproduction commands for PR #1233](../benchmarks/gc/publication/typed-root-collection/tenfold-complete/README.md).

The first tenfold collector exceeded the 512 MiB limit during JVM warmup in
all four fixed-arrival forks. Restricting the mature survivor index to sweeps
that visit multiple entries brought all four below 461 MiB. The corrected
experimental collector at de943ebf cleared the same complete-collection gate
with lowest individual 95% bounds of 10.20× native and 11.62× WASI. The
end-to-end comparison still favors HotSpot at both measured rates. [Java
results, individual forks and graph](../benchmarks/gc/publication/typed-root-collection/tenfold-java/README.md).

Continue zero-global-pause work on typed paged-root publication, deferred page
cleanup and whole-JVM pause attribution. Re-run the scheduled Java comparison
and the native/Wasm sustained workloads on the integrated implementation.

The production integration target is 10× against f1daebeb in every existing collection condition.
Use the same full-cycle fixtures, controls, paired schedule, physical refunds
and per-condition uncertainty as the fivefold gate. At 92906c66, 20 of the 36
paired medians and 15 of the 36 individual 95% lower bounds reach 7×.
The release stack at d623c73d (#1231) reached minimum medians of 5.75×
native and 7.04× WASI. Its lowest individual 95% lower bounds were 5.55×
and 6.99×. Twenty-four medians and 22 lower bounds cleared 7× across
36 conditions. From the slowest medians, reaching 10× requires another
42.5% reduction on native and 29.6% on WASI. Require 24 randomized pairs
per condition and individual 95% lower bounds of at least 10×. [Graphs,
raw measurements and source provenance](../benchmarks/gc/publication/typed-root-collection/sole-cycle-progress/README.md).

The build verifier (#1230) compiles each arm in a fresh target and requires
fresh Cargo artifacts for the library and benchmark example. Isolated rebuilds
reproduced all four published #1221 binary hashes, as well as both targets for
#1227, #1229 and #1231. Three private experiments had inadvertently measured
a different checkout through a shared target directory. Their earlier timing
rows are excluded. The isolated replacements cover ranged claims, independent
primitive refund slots and strong mark exchange.

Preserve bounded peer progress throughout. A nested trace can step another
heap after discovering a young edge. Keep the enclosing callback's witness
through the inner callback, remembered-owner pruning, and a subsequent young
collection (#1227). Measure physical clearing and refund through the last
claimed object in every mixed cycle. Preserve callback and descriptor identity
even when other helpers park, reach reference closure or encounter stale work.

The Java roots capacity comparison (#1228) covers 36 fresh 60-second
processes. Ristretto confirmed a passing/failing bracket of 1,437–1,500
arrivals/s. Both HotSpot collectors passed through 16,000/s. Replicate common
rates independently and instrument global mutator pauses before making a
sustained-load or zero-pause release claim.

## Fivefold complete-collection gate

The previous target was a minimum 5× reduction in complete collection time against
f1daebeb in every established native and WASI condition. Keep the 4,095-node
graphs, the 1,023 fresh dead scalars per mixed cycle, both publication policies,
diagnostics modes, worker counts and exact physical refund checks. Repeat the
randomized 24-pair qualification after integration and report each confidence
interval. Resume wider zero-global-pause qualification after this gate.

Integration `92906c66` adds detached numeric root removal, old-callback edge
handoff and the pending-reference FIFO. The 36-condition rerun cleared 5× with
lowest medians of 5.67× native and 6.68× WASI. The lowest individual 95%
lower bounds were 5.53× and 6.46×. A peer collector can finish the source
snapshot, reuse physically refunded space at an exact heap cap and finish
another collection while a covered numeric-root remover is parked.
[Collection results](../benchmarks/gc/publication/typed-root-collection/numeric-root-progress/README.md)

At 1,500 scheduled Java roots operations/s the integrated JVM completed
90,000 arrivals in each 60-second fork. Ristretto scheduled p99.9 was 235
and 1,600 µs. Both G1 and ZGC met 1 ms in both forks. The 500/s arithmetic
control also missed 1 ms in both Ristretto forks despite service p99.9 at or
below 20 µs. The common sustained-load and global-pause gates remain open.
[Java latency comparison](../benchmarks/gc/publication/typed-root-collection/arrival-characterization/README.md)

The integrated field/root/reference fixes at 0ca5947b again cleared all 36
conditions. Across 1,728 processes the slowest median was 5.86× native and
6.66× WASI. The lowest individual 95% lower bounds were 5.57× and 6.43×.
Cargo verified the packaged collector. Deterministic tests cover a parked
writer with a live young child, registry retirement, mutation-counter
saturation, exact root revocation, final/phantom tracing and queue delivery
with parked object shards. [Collection graphs and samples](../benchmarks/gc/publication/typed-root-collection/rooted-writes/README.md)

The combined host reference and mark-lane implementation at 53897798 cleared
all 36 conditions in 1,728 fresh processes. The slowest median was 5.76×
native and 6.80× WASI. Every individual bootstrap 95% lower bound exceeded
5×, with minima of 5.59× and 6.68×. The production browser worker completed
12 collections, swept 47,712 objects and returned all 222 pings during Java
execution. The largest reply gap was 50.7 ms. Global-pause timing remains
unavailable. [Complete collection results](../benchmarks/gc/publication/typed-root-collection/host-reference-progress/README.md)
· [Browser worker receipt](../benchmarks/gc/publication/browser-worker-reclamation/host-reference-progress/README.md)

The native Java roots workload remains far behind HotSpot under the same CPU
and memory limits. A 60-second test of the ordinary integrated JVM at 1,000
arrivals/s completed all 60,000 arrivals per fork. Scheduled p99.9 was 1,392
and 214 µs for Ristretto, 296 and 5 µs for G1, and 8 and 181 µs for ZGC.
Ristretto missed the predeclared 1 ms limit in its first fork. No capacity
bracket or sustained zero-pause claim follows from this single rate. At 500/s
every runtime exceeded 1 ms in at least one fork.
[60-second process results](../benchmarks/gc/publication/typed-root-collection/roots-capacity/README.md)
· [Short paired JVM measurements](../benchmarks/gc/publication/typed-root-collection/peer-lanes/README.md)

The reviewed collector at 0d353249 meets the median gate in 36 of 36
conditions. The weakest paired medians were 5.17× native and 6.85× WASI.
The weakest per-condition 95% lower bounds were 4.80× and 6.63×
respectively. Two native two-worker tree intervals cross 5×. The preceding
integrated build cleared 5× on every lower bound. Each campaign used 1,728
fresh processes. The next gates cover collector progress under parked
participants, physical reclamation, scheduled application latency and
sustained native and WebAssembly workloads.
[Reviewed measurements and figures](../benchmarks/gc/publication/typed-root-collection/fivefold-collection/README.md)
· [Benchmark PR](https://github.com/theseus-rs/ristretto/pull/1194)

At eaff58f2, 20 of the 36 paired medians clear 5×. All 12 mixed cells and
the four native two-worker tree cells remain below target. The lowest
native and WASI medians are 3.064× and 3.907×. Prioritize physical retirement
and contention between marking helpers. [Frozen results](../benchmarks/gc/publication/typed-root-collection/numeric-retirement-collection/README.md)

At 57687beb, bounded small allocation pages improve all 36 paired medians
against their immediate parent. Against f1daebeb, 30 of 36 medians reach 5×.
The minimum is 3.57× on native and 5.01× on WASI. The corresponding weakest
95% lower bounds are 3.47× and 4.94×. [Complete results](../benchmarks/gc/publication/typed-root-collection/small-allocation-pages/README.md)
· [Implementation PR](https://github.com/theseus-rs/ristretto/pull/1186)
· [Benchmark PR](https://github.com/theseus-rs/ristretto/pull/1187)

## Typed roots and complete collection time

The complete-collection benchmark reached 3× in all 36 native and WASI
conditions against f1daebeb. Two campaigns validated 3,456 processes. On the
final throughput build, the slowest median speedups were 3.03× for native and
3.77× for WASI, both on mixed heaps. The weakest per-condition 95% lower
bounds were 2.95× and 3.71×. Native two-worker throughput on this fixture
has little margin. Maintain the mixed-reclamation check after integration.
[Measurements and graphs](../benchmarks/gc/publication/typed-root-collection/collection-throughput/README.md)
· [Stacked throughput PR](https://github.com/theseus-rs/ristretto/pull/1169).

A third campaign used the integrated typed-root and throughput sources. Across
1,728 new processes, the smallest speedups were 3.28× on native and 3.88×
on WASI. The weakest per-condition 95% lower bounds were 3.11× and 3.83×.
The separate whole-JVM screen measured 92,201 saturated allocation operations/s
in the native interpreter, 18.39 million in G1, and 20.29 million in ZGC.
Its 60 short forks leave sustained throughput and scheduled response tails open.
[Integrated data, graphs and qualification results](../benchmarks/gc/publication/typed-root-collection/integrated-collection/README.md).

A parked typed-root lookup previously exhausted a capped allocation's 1,024
polls when the target was a reachable, unvisited child. A one-shot overflow
registration now lets a scanner cancel an undecided attempt. Each accepting
domain keeps an exact descriptor and records traversal independently of later
root removal. A bounded native regression covers both publication policies,
all three root stores, dropping the original parent before or after the
decision, and a child with descendants. The next integration gates include
overlapping generations, VM publication and root removal, arbitrary tracing
callbacks, complete pause attribution, JVM tail latency and sustained throughput
relative to both G1 and ZGC. The browser JShell Java 11 /reload regression
reproduced on both the previous typed-root PR and its parent.
[Parent CI](https://github.com/theseus-rs/ristretto/actions/runs/45611023816/job/139194258132)
· [Typed-root CI](https://github.com/theseus-rs/ristretto/actions/runs/45653867833/job/139345206268).

The covered typed-root deletion path now gives scanners a veto before returning
the address. Its active-epoch schedule checks that a strongly marked parent can
lose its root while the unvisited child survives this collection. A separate
native recovery test parks the sole collector worker and an executor waker.
Independent canonical refunds let another allocation finish with the full ticket
still outstanding. Retired bytes stay charged in telemetry across local clearing
continuations. These tests leave legacy admission and opaque callback stalls open.
[Root removal](https://github.com/theseus-rs/ristretto/pull/1174) ·
[Shared-budget recovery](https://github.com/theseus-rs/ristretto/pull/1175) ·
[Retirement accounting](https://github.com/theseus-rs/ristretto/pull/1176).

Five additional forks per condition ran with an enforced two-CPU quota and
512 MiB process memory limit. On roots the native interpreter reached 7,805
operations/s, HotSpot G1 in interpreter mode reached 1,312,180, and interpreted
ZGC reached 1,249,344. Separate instrumented forks found that the operation
method continued to run interpreted with the production JIT enabled.
[Equal-budget results](../benchmarks/gc/publication/typed-root-collection/integrated-collection/equal-budget/REPORT.md).

## Preceding exact address registration checkpoint

At `7acdabb5`, a constructor reserves its exact address and nursery bit without
taking a live-operation shard guard. With one shard and a two-object cap, a
peer reclaimed garbage and completed a second allocation while the constructor
remained parked. The schedules cover both publication policies in idle, strong
marking and sweep. Page collision, same-address reuse and partial installation
have separate forced schedules. A pin keeps a page reachable between reservation
and setting its bit. Zero claims seal that page permanently. The last remover
checks the owning page identity before unlinking it.

Live root acquisition, weak upgrade and sweep still coordinate on the
live-operation shard. In a forced typed-handle lookup on a two-object heap,
both policies exhausted a peer's 1,024-poll budget with unrelated garbage
available. Dropping the shard before parking the lookup still exhausted both
budgets while root publication admission remained held. The next progress
gate is to let a peer complete across that live-root handoff, including a
publisher stopped after validation. An expired, replaced or otherwise
ineligible descriptor must never acquire a new root. Callback progress,
barrier costs and sustained native and Wasm performance also remain open.
The whole-JVM and unrestricted-write results still miss the alpha thresholds.
[Registration tests](src/publication_tests.rs) ·
[Registration protocol](validation/REGISTRATION_IMAGE_ACTIVATION.md) ·
[Comparisons, raw data and graphs](../benchmarks/gc/publication/7acdabb5/README.md).

## Preceding strong-graph publication checkpoint

On 2026-11-25, ordinary graph construction stopped holding phase admission
across strong marking work. Both publication policies link a provisional
traversal in each affected epoch. A peer can pass a private attempt, take
the traversal of a live parent and reclaim unrelated garbage while its
constructor stays parked. The forced schedules cover a held ordinary injector,
an unpublished slot with its mutex held, the interval between certification and
activation, and disabled numeric certification. Epoch validation, cancellation
across retries and private-payload unwind have separate negative controls.

At this checkpoint, a writer parked in the physical object-shard update
prevented an unrelated capped allocation under both policies. This failure
provided the negative control for the exact-address implementation above.

In 16 randomized pairs per target, one-edge graph allocation plus full
collection took 13.7% longer on native and 2.0% longer on WASI than the
preceding collector. The initial linked-queue version took 91.6% longer on
native. The whole-JVM allocation screen measured 86,247 operations/s in the
native interpreter, 18.47 million in HotSpot G1 and 23.00 million in ZGC.
All three one-minute JVM array-element setter trials completed 600,000
requests. One trial's operation p99.9 was 103 µs against a strict 100 µs
limit. Unrestricted array writes completed 22,065–33,675 of the 600,000
offered. The reporter now retains measurements printed by a process that
subsequently fails a latency assertion.
[Results, graphs and reproduction](../benchmarks/gc/publication/434a074e/README.md).

## Preceding certified publication checkpoint for leaves and post-strong graphs

On 2026-11-25, both ordinary publication policies began using the same phase
clock to publish an indexed, private descriptor. A reference-free object can
initialize its marks without holding phase admission. A graph can use the same
path when all active domains have finished strong tracing. A revision change
or a scan that skips the descriptor forces a retry. The publisher discards
the speculative marks and old-cohort tag before starting that retry. Active
births fall back to ordinary admission when the clock loses certification.

With a single metadata shard and a two-object cap, a peer reclaimed space
and allocated while the leaf publisher remained parked during marking and
sweep. A new strong collection starting between graph validation and activation
traced the exact parent and preserved its child. Disabling the visibility CAS
failed both forced collector schedules and the retry model. A publisher parked
inside the physical shard update, or holding strong-graph admission before or
after publishing its traversal, still prevents capped recovery. Those two
mechanisms keep the integrated alpha PR gate open.

In 24 process pairs, default-policy rooted allocation and read time improved
1.8% native and 2.2% WASI against the preceding collector. Scalar allocation
plus young collection on WASI took 2.8% longer. In the whole-JVM allocation
screen Ristretto's native interpreter reached 93,777 operations/s. HotSpot G1
reached 21.31 million and ZGC reached 25.50 million. Three one-minute trials
of the JVM array-element setter each completed 600,000 requests. The unrestricted
array-write trials completed at most 36,714 of the same offered requests.

[Whole-JVM results, graphs and reproduction commands](../benchmarks/gc/publication/6fd06e30/README.md),
[collector schedules](src/publication_tests.rs), [ordering models](tests/admission_model.rs)
and [full verification record](../../ristretto-gc-evidence/alpha-progress-0134e21a-v1/certified-birth-receipt.json).

## Preceding private-descriptor publication checkpoint

Both publication policies now release the object shard before assigning a
new descriptor's phase marks or queue work. With a two-object cap and one
metadata shard, a peer reclaimed the previous allocation and completed its
own while the indexed constructor remained parked. This passed from idle,
strong marking and sweep. With the previous implementation both policies
exhausted the peer's poll budget.

A scan that passes a private descriptor records the skip. A counted publisher
that observed an idle clock can activate without admission only if no scanner
has passed it. In a forced schedule, the constructor stayed parked after its
idle decision across two full collections and resumed during a later sweep.
It joined that sweep's epoch across all three root stores. Injected unwind
removed the unpublished identity and nursery membership and refunded the
allocation. The model exercises the production visibility word and phase clock.
[Forced collector schedules](src/publication_tests.rs) and
[phase clock and visibility models](tests/admission_model.rs).

The first publisher to initialize an overflow FIFO can now stay parked while a
second publisher installs the FIFO, enqueues a descriptor and a worker claims
it. The production-state Loom test and a forced collector schedule both failed
when the observer skipped a private descriptor without recording its visit.
Parking inside the physical index update or while holding active traversal
admission still blocked capped allocation under both policies. These two
schedules keep the integrated alpha PR gate open.

### Ordinary GC measurements on 2026-11-25

With 32 randomized process pairs against `a3318a1a`, the counted allocation
and read operation on `69b6234e` changed by +0.2% on native (95% bootstrap
interval −0.9% to +1.4%) and +2.6% on WASI (+2.3% to +4.2%). Counted WASI
handle lookup took 1.0% less time. In 24 pairs during marking, the combined
allocation and collection time for a one-edge WASI graph increased 1.4% in
full collections and decreased 1.4% in young collections. The active-birth
control uses the earlier collector with matching dependency declarations.
[Native mechanism report](../../ristretto-gc-evidence/alpha-progress-0134e21a-v1/pending-birth-accepted-timing/native/REPORT.md),
[WASI mechanism report](../../ristretto-gc-evidence/alpha-progress-0134e21a-v1/pending-birth-accepted-timing/wasm/REPORT.md)
and [verification with raw samples](../../ristretto-gc-evidence/alpha-progress-0134e21a-v1/pending-birth-receipt.json).

The whole-JVM allocation screen used the same Java classfiles and a 64 MiB heap.
Each cell used three fresh processes with one second of warmup and three
measured seconds. The fixed-rate trials completed all 3,000 requests per process.

| Runtime | Saturation, operations/s | Scheduled-arrival p99.9 at 1,000/s |
| --- | ---: | ---: |
| Ristretto native interpreter | 92,441 | 38 µs |
| Ristretto native JIT enabled | 89,322 | 186 µs |
| Ristretto WASI interpreter | 68,603 | 34 µs |
| HotSpot G1 | 20,894,307 | 11 µs |
| HotSpot ZGC | 25,119,911 | 6 µs |

[Whole-JVM comparison](../../ristretto-gc-evidence/alpha-progress-0134e21a-v1/pending-birth-accepted-vm/comparison/REPORT.md),
[native throughput](../../ristretto-gc-evidence/alpha-progress-0134e21a-v1/pending-birth-accepted-vm/comparison/graphs/native-throughput.svg),
[WASI throughput](../../ristretto-gc-evidence/alpha-progress-0134e21a-v1/pending-birth-accepted-vm/comparison/graphs/wasm-throughput.svg)
and [scheduled native latency](../../ristretto-gc-evidence/alpha-progress-0134e21a-v1/pending-birth-accepted-vm/comparison/graphs/native-latency.svg).

The compacting endurance driver offered 600,000 mutations per one-minute
process with two workers, a 64 MiB heap, two CPUs and a 512 MiB process cap.
The original unrestricted owner-array write completed 33,731–37,439
operations per trial on the candidate. All three trials on both the candidate
and its predecessor missed the release thresholds. In three separately
instrumented ten-second runs, updating the 2,048-slot owner array accounted for
99.73–99.74% of measured operation time.

The additional `--endurance-array-write element` option exercises the setter
used by JVM `aastore` with the same object graph and slot sequence. All three
processes completed 600,000 operations and drained the heap. 2 of three
satisfied both operation latency thresholds of p99.9 below 100 µs and p99.99
below 1,000 µs. The full one-hour and 24-hour gates remain open.
[Trial results and thresholds](../../ristretto-gc-evidence/alpha-progress-0134e21a-v1/pending-birth-endurance-graphs/endurance.svg),
[raw counts](../../ristretto-gc-evidence/alpha-progress-0134e21a-v1/pending-birth-endurance-graphs/samples.csv)
and [driver instructions](../benchmarks/gc/README.md).

### Preceding indexed-phase checkpoint

On 2026-11-25 UTC, 32 paired processes per workload compared `5fe7f88f` with the
preceding identity-index implementation. With diagnostics off, the median paired
time to allocate and read a rooted 32-byte payload fell **2.7% native** and
**2.3% WASI**. Across full/minor collection and scalar/single-edge graphs, every
interval for combined allocation and collection time included zero. Native
scalar allocation during minor collection took 2.3% longer; the equivalent
WASI interval favored the new version by 2.5% at the median.
[Native paired results](../../ristretto-gc-evidence/alpha-progress-0134e21a-v1/indexed-phase-timing/native-mechanisms/REPORT.md),
[WASI paired results](../../ristretto-gc-evidence/alpha-progress-0134e21a-v1/indexed-phase-timing/wasi-mechanisms/REPORT.md)
and [measurements, graphs and verification](../../ristretto-gc-evidence/alpha-progress-0134e21a-v1/indexed-phase-receipt.json).

The same Java classfiles allocated and published 256-byte arrays on five runtimes.
Each workload/rate cell used three fresh processes, a 64 MiB heap, the same two
CPUs, one second of warmup and three measured seconds. The Ristretto runtimes
used cooperative collection. All 30 processes passed the checksums and retained
graph audit. Each runtime completed all 3,000 fixed-rate requests per process.

| Runtime | Saturation, operations/s | Scheduled-arrival p99.9 at 1,000/s |
| --- | ---: | ---: |
| Ristretto native interpreter | 97,036 | 55 µs |
| Ristretto native JIT enabled | 87,344 | 58 µs |
| Ristretto WASI interpreter | 72,902 | 69 µs |
| HotSpot G1 | 21,859,543 | 238 µs |
| HotSpot ZGC | 25,957,205 | 9 µs |

These are medians of per-process measurements of the entire JVM. The fixed-rate
sample supports an exploratory latency comparison only. Sustained capacity,
extreme-tail latency and zero collector-wide blocking remain open.
[Whole-VM report](../../ristretto-gc-evidence/alpha-progress-0134e21a-v1/indexed-phase-vm/comparison/REPORT.md),
[native throughput](../../ristretto-gc-evidence/alpha-progress-0134e21a-v1/indexed-phase-vm/comparison/graphs/native-throughput.svg),
[native scheduled latency](../../ristretto-gc-evidence/alpha-progress-0134e21a-v1/indexed-phase-vm/comparison/graphs/native-latency.svg)
and [WASI throughput](../../ristretto-gc-evidence/alpha-progress-0134e21a-v1/indexed-phase-vm/comparison/graphs/wasm-throughput.svg).

In a separate instrumented JIT-enabled run, all 420,885 entries to
`GcBench.operation` were interpreted. Production eligibility rejected an
instruction in that method and rejected the histogram recorder as an instance
method. The timed JIT-enabled row therefore needs method coverage before
attributing its throughput to generated allocation code.
[Execution profile](../../ristretto-gc-evidence/alpha-progress-0134e21a-v1/indexed-phase-vm/coverage/REPORT.md).

Ordinary registration now reserves an exact identity slot before phase admission.
A constructor parked with that reservation leaves capped allocation available.
Retirement seals an empty identity page, schedules payload reclamation, then
releases collector guards before unlinking the page. Four allocation threads
starting against the same full heap each reclaimed or used available credit
and obtained a rooted object while the unlink worker remained parked. The
threads repolled only on waker notification. Both publication policies and
both phase schedules passed. Unlinking inside sweep's guards failed in all
four schedules. Parking inside the object-shard registration commit still
prevents recovery. The integrated alpha gate remains open.
[Protocol and stopped-thread tests](validation/REGISTRATION_IMAGE_ACTIVATION.md#validation-boundaries).

On 2026-11-25 UTC, 32 process pairs per workload compared `b4ecb559` with
`3fab3f44`. With counted publication and diagnostics off, median time to
allocate and read a 32-byte payload fell 2.8% on native and 1.4% on WASI.
Including collection between batches changed elapsed time by −1.0% on native
and +1.0% on WASI. The WASI interval for that elapsed-time change includes zero.
The separate strong-mark fixture measured rooted allocation plus the remaining
collection, using 65,536 verified promotions for each minor run.

| Collection | Native scalar | Native single edge | WASI scalar | WASI single edge |
| --- | ---: | ---: | ---: | ---: |
| Minor | −1.34% | −0.87% | +2.12% | +1.59% |
| Full | −0.85% | −0.55% | +0.54% | +0.84% |

Negative values mean less elapsed time. The native graph intervals and the
WASI full scalar interval include zero. Both WASI minor intervals and the WASI
full graph interval are above zero. These collector workloads leave the
whole-VM G1/ZGC and endurance gates open.
[Native allocation and lookup](../../ristretto-gc-evidence/alpha-progress-0134e21a-v1/identity-hints-timing/native-mechanisms/REPORT.md),
[WASI allocation and lookup](../../ristretto-gc-evidence/alpha-progress-0134e21a-v1/identity-hints-timing/wasi-mechanisms/REPORT.md),
[native minor graph](../../ristretto-gc-evidence/alpha-progress-0134e21a-v1/identity-hints-timing/native-young/marking-births.svg),
[WASI minor graph](../../ristretto-gc-evidence/alpha-progress-0134e21a-v1/identity-hints-timing/wasi-young/marking-births.svg),
and [raw data, all graphs and source checks](../../ristretto-gc-evidence/alpha-progress-0134e21a-v1/identity-hints-receipt.json).

The ordinary collector removed the independent nursery mutex on 2026-11-25 UTC.
A parked nursery scanner can retain its directory while a capped peer reclaims
garbage, promotes a live object and allocates. Both publication policies passed
this schedule in 32 fresh native processes. At `d24eb6d7` both exhausted
4,096 allocation polls. A constructor parked inside registration's object-shard
commit still blocks the capped peer with either identity backend. The next
integrated alpha gate remains reclamation under that stopped publisher.
[Schedule and remaining gate](validation/REGISTRATION_IMAGE_ACTIVATION.md#validation-boundaries).

On a single pinned CPU, 32 counterbalanced process pairs per target, collection
kind and payload compared `69211654` to `d24eb6d7`. The table gives the median
change in rooted allocation plus remaining collection time. Each minor run
verified 65,536 promotions. Negative values mean less elapsed time.

| Collector workload | Native scalar | Native single edge | WASI scalar | WASI single edge |
| --- | ---: | ---: | ---: | ---: |
| Minor collection | +1.96% | +3.43% | −7.61% | −5.97% |
| Full collection | −0.03% | +1.56% | −6.47% | −5.36% |

The native full scalar interval includes zero. The native minor regressions
and both WASI improvements for each collection kind persisted across the
paired bootstrap intervals. The graphs show allocation and finish timers
separately. The whole-VM G1/ZGC and endurance gates remain open.
[Native minor](../../ristretto-gc-evidence/alpha-progress-0134e21a-v1/nursery-cursor-timing/native-young/marking-births.svg),
[native full](../../ristretto-gc-evidence/alpha-progress-0134e21a-v1/nursery-cursor-timing/native-full/marking-births.svg),
[WASI minor](../../ristretto-gc-evidence/alpha-progress-0134e21a-v1/nursery-cursor-timing/wasi-young/marking-births.svg),
[WASI full](../../ristretto-gc-evidence/alpha-progress-0134e21a-v1/nursery-cursor-timing/wasi-full/marking-births.svg),
and [samples, build fingerprints and validation](../../ristretto-gc-evidence/alpha-progress-0134e21a-v1/nursery-cursor-receipt.json).

Ordinary graph registration (2026-11-25 UTC) now puts an exact descriptor on
each required mark queue before releasing the index write set. With an indexed
constructor parked and the heap full, a peer traced the single-edge object,
finished the full collection, reclaimed garbage and allocated. Both legacy
and counted publication passed with the injector available and with it held.
The previous checkpoint needed publisher resumption in all four schedules.
Parking the constructor **inside** the index commit still blocked the capped
peer under both policies. The integrated alpha remains open.
[Executable gate](validation/REGISTRATION_IMAGE_ACTIVATION.md#validation-boundaries).

The [strong-mark allocation benchmark](../benchmarks/gc/README.md#allocation-during-a-strong-mark)
compared 32 process pairs per target and payload shape against `7f72cf54`.
Median combined allocation and remaining collection time changed by +0.26%
for native graphs and +0.34% for WASI graphs. Scalar times changed by +1.60%
and −0.54%, respectively. All four combined-time 95% intervals include zero.
The native scalar finish-only timer rose 2.18% (interval +0.06% to +3.38%).
The comparison uses a cooperative collector and a shared anchor.
[Native graph and scalar samples](../../ristretto-gc-evidence/alpha-progress-0134e21a-v1/handoff-segments-timing/native/samples.csv),
[native plot](../../ristretto-gc-evidence/alpha-progress-0134e21a-v1/handoff-segments-timing/native/marking-births.svg),
[WASI samples](../../ristretto-gc-evidence/alpha-progress-0134e21a-v1/handoff-segments-timing/wasi/samples.csv)
and [WASI plot](../../ristretto-gc-evidence/alpha-progress-0134e21a-v1/handoff-segments-timing/wasi/marking-births.svg).

The preceding descriptor registration change (2026-11-25 UTC) completes reference-free
births inline during strong tracing. With the constructor parked after its last
index unlock, a capped peer can reclaim and allocate during idle, mark and
sweep under either publication policy. At this checkpoint graphs still needed a queue handoff after unlocking.
Releasing its admission early let a full collection finish before that handoff
in the negative control. Parking a constructor *inside* the index commit still
prevents capped recovery.

The [marking-birth benchmark](examples/marking_births.rs) allocated 65,536
scalars per process during full tracing. In 16 paired release-build runs per
target, median allocation time fell 8.7% on native and 19.9% on WASI. The sum
of allocation and remaining collection time fell 25.5% and 30.7%, respectively.
Root release and the next collection are outside these timers. The integrated
alpha, G1/ZGC comparison and endurance gates remain open.
[Paired results](../../ristretto-gc-evidence/alpha-progress-0134e21a-v1/descriptor-birth-benchmark.json),
[graph](../../ristretto-gc-evidence/alpha-progress-0134e21a-v1/descriptor-birth-benchmark.svg)
and [raw measurements](../../ristretto-gc-evidence/alpha-progress-0134e21a-v1/descriptor-birth-samples.csv).

Ordinary rooted allocation (2026-11-25 UTC) now installs a private parent root
before entering descriptor publication. With room for two scalar payloads, a
second mutator reclaimed garbage and completed its allocation while the first
constructor stayed parked at initial root insertion. Parking the constructor
later, after it acquired the descriptor index write set, still blocked that
recovery. A 12-pair comparison of 32-byte leaves using context roots and counted
publication measured 6.5% lower native and 11.7% lower WASI time for allocation,
read and root drop. Including collection and cleanup, time fell 3.8% and 4.5%.
The integrated alpha still requires progress during descriptor publication.
[Benchmark and scope](../../ristretto-gc-evidence/alpha-progress-0134e21a-v1/private-root-benchmark.json),
[paired graph](../../ristretto-gc-evidence/alpha-progress-0134e21a-v1/private-root-benchmark.svg)
and [remaining progress witness](../../ristretto-gc-evidence/alpha-progress-0134e21a-v1/private-root-remaining-witness.patch).

Evidence availability notice (2026-11-15 22:23 UTC): the workspace `target`
directory disappeared, including historical GC raw results, graphs and archives.
The source changes remain. Results recorded below are historical observations;
links under target are unavailable and cannot serve as current qualification evidence.
See [the recovery record](../benchmarks/gc/EVIDENCE_RECOVERY.md). Fresh component
validation is complete outside Cargo build storage; no full qualification gate is
closed by the earlier summaries.

An opt-in restricted Java runtime source preview is ready for maintainer
release review. The packaged example ran interpreted and compiled on native
and interpreted on WASI. All three returned 64 and reclaimed all 16 objects.
The extracted archive also passed 284 native and 189 WASI ordinary GC library
tests. The separate review checkout matches all 2,677 original source files
and executable flags in the package.
The source preview includes the Java contract, resource limits and finite peer
measurements. The integrated VM alpha, performance-qualified beta and full
endurance release remain subsequent milestones.
[Preview archive and validation](../benchmarks/gc/CANONICAL_MEASUREMENTS.md#remaining-release-gates).
The [stacked MR release plan](PREVIEW_RELEASE_PLAN.md) maps the preview contract to independently testable reviews and archive verification.

Subsequent ordinary collector changes let slot-backed handles make progress
when all 16 page writers are parked. The same-heap test acquires a handle,
drops its original root, completes three collections, reads the surviving
value and reclaims it after its final root drops. A separate shutdown repair
releases the cycle lock before running pending finalizers and destructors.
The combined source passed 1,387 native and 235 WASI checks and GC lint on
both targets. The numeric default-tree and stalled publication cases remain
open. [Contention, ownership and allocation measurements](../benchmarks/gc/CANONICAL_MEASUREMENTS.md#contention-in-the-ordinary-slot-root-registry).

The revised on-demand field write improved 24 of 27 matched process pairs.
All three large native-compiled pairs slowed, with a median increase of 3.22%.
G1 delivered 16,384 and 32,768 requests/s with zero deadline backlog in all
three one-minute medium-ring forks at each rate. At 32,768/s one fork
exceeded both latency and peak backlog screens. All three 65,536/s forks
missed the delivery deadline, with 121,145–580,991 offers outstanding. The paired capacity,
sustained latency and endurance gates remain open.
[Paired field results](../benchmarks/gc/CANONICAL_MEASUREMENTS.md#on-demand-null-storage-on-the-persistent-java-ring)
and [one-minute G1 results](../benchmarks/gc/CANONICAL_MEASUREMENTS.md#one-minute-medium-ring-g1-baseline).

Holding the ordinary numeric root registry's read view prevented the existing
default collector from reclaiming an unrelated object in 4,096 steps on native
and WASI. An isolated paged registry reclaims it under the same schedule.
The revised candidate passed 325 native and 224 WASI regression checks, three
selected concurrency models and two strict-provenance Miri cases. It still
requires 1.04–3.38% more host allocation calls on the three tested workloads
under the default policy. The existing preview uses the original registry.
[Root progress, costs and limits](../benchmarks/gc/CANONICAL_MEASUREMENTS.md#ordinary-numeric-root-registration).

An isolated catalog candidate uses the retained control slot to check which
catalog owns a snapshot or prepared attempt. This removes a second catalog
owner reference. Nine native/WASI Java cases preserved their per-call work and
cleanup counts. Five selected concurrency models passed, and two guard-removal
controls exposed the expected foreign-snapshot failures. The matched invocation
comparison shortened 16 of 18 native pairs and slowed eight of nine WASI pairs.
The candidate remains experimental.
[Correctness checks](../../ristretto-gc-evidence/catalog-owner-identity-v2/REPORT.md)
and [paired runtime measurements](../benchmarks/gc/CANONICAL_MEASUREMENTS.md#catalog-owner-validation-on-the-persistent-java-ring).

An ordinary borrowed field can now acquire a guard with one root registration.
In slot-backed policies, another writer can also root the same address using
an available shard while its original shard is parked. The previous native
code reserved two roots and timed out in the parked-writer regression. The
updated default-feature suites passed 313 native and 214 WASI tests, including
the direct shard test on both targets. Exhausting every writer shard and
parking a constructor after publication admission remain separate progress
risks. [Native checks](../../ristretto-gc-evidence/root-shard-progress-v1/after-native/summary.json)
and [WASI checks](../../ristretto-gc-evidence/root-shard-progress-v1/after-wasi/summary.json).

On-demand null fields remain experimental. Their first source-isolated Java
comparison improved all small and medium pairs and regressed all large pairs.
A separate ordinary Wasm collector screen completed all 15 Chromium forks
with zero unfinished requests. These runs leave the zero-global-blocking,
sustainable-throughput, scheduled-tail and endurance gates open.
[Paired and browser measurements](../benchmarks/gc/CANONICAL_MEASUREMENTS.md#on-demand-null-storage-on-the-persistent-java-ring).

The combined runtime with two-slot controller transactions passed 119 native
and 86 WASI VM tests, 385 native and 275 WASI GC tests, and strict lint on
both targets. In a separate source-isolated paired comparison, the two-slot
change shortened the median Java ring invocation time in all 27 pairs. Across
the nine size/tier groups, the median within-pair reduction was 4.87–11.41%.
[Combined validation](../../ristretto-gc-evidence/integration-v12/summary.json)
and [paired measurements](../benchmarks/gc/CANONICAL_MEASUREMENTS.md#paired-controller-comparison).

A default-path regression parks an ordinary scalar constructor after admission
and before it takes any registry mutex. A second mutator has reclaimable space
yet cannot start collection during 1,024 recovery polls. It completes after the
first constructor resumes. The counted admission path exhibits the same dependency
in the cooperative test. This keeps the zero-global-blocking gate open.
[Failure witness and protected operations](../../ristretto-gc-evidence/ordinary-admission-progress-v1/design-v1.md).

Ordinary weak upgrades and validated integer handles now recover the pointer
from the matching live descriptor under publication protection. Strict-provenance
Miri passed ten cases covering both handle forms, typed arrays and parked
callbacks. The native and WASI regression suites passed 21 and 18 tests,
respectively, including stale identities and allocation-type checks.
[Miri](../../ristretto-gc-evidence/validated-weak-provenance-v1/miri/result.json),
[native](../../ristretto-gc-evidence/validated-weak-provenance-v1/ordinary-native.json)
and [WASI](../../ristretto-gc-evidence/validated-weak-provenance-v1/ordinary-wasi.json).

The first fixed-arrival peer pilot completed 60 ten-second runs with a
persistent heap. At 128 offered requests/s on the medium ring, G1 and ZGC
each completed 128/s by deadline. Native compiled Ristretto completed 46.1/s,
native interpreted 53.5/s and WASI interpreted 37.7/s. The three Ristretto
tiers accumulated backlog at both 64 and 128/s. Each fork had fewer than
10,000 completions. The sustainable-throughput, scheduled p99.9/p99.99,
zero-global-blocking and endurance gates remain open.
[Scheduled results, graphs and raw evidence](../benchmarks/gc/CANONICAL_MEASUREMENTS.md#first-scheduled-arrival-peer-comparison).

The paired scheduling benchmark completed 54 processes and 1,620 measured
invocations. Median within-pair speedups on the restricted ring workload range
from 1.93× to 2.25× on native and 2.36× to 2.90× on WASI. This establishes a
full-invocation improvement on this workload. Scheduled latency, global pause
coverage, sustainable throughput and endurance remain open. The ordinary GC now
keeps ready allocation and reference-handler notifications accessible to other
helpers during a stalled wake callback. See the
[measurement record](../benchmarks/gc/CANONICAL_MEASUREMENTS.md). The integrated
source, including allocation by field layout, passed 85 native and 65 WASI VM
checks plus 373 native and 266 WASI GC library checks.
[Integration receipt](../../ristretto-gc-evidence/integration-v8/summary.json).

The first persistent-heap peer comparison finished all 63 processes. The
native compiled tier took 19.50–31.89 ms per ring invocation across three
sizes, compared with 0.029–0.089 ms for G1 and 0.028–0.085 ms for ZGC. Heap
setup and final drain were outside the call clocks. Sustainable completed
throughput, qualified scheduled tails and zero global GC blocking remain open.
Pooled reference acquisition and release accounted for about 22–25% of
whole-process native CPU samples in the separate diagnostic. Reusing private
graph-root backing cut host allocation calls for 10,000 empty reads from
10,000 to one on both native and WASI.
[Persistent-heap comparison and diagnostics](../benchmarks/gc/CANONICAL_MEASUREMENTS.md#peer-comparison-with-a-persistent-java-heap).

The shared-heap interpreter now retains static roots across calls. Class
initializers have an exclusive logical owner, can recurse on their own class,
and allow unrelated classes and reclamation to progress when parked. This
source passed 113 native and 89 WASI VM checks. The compiled driver uses the
same ownership protocol, including cancellation. Integrated qualification on
the broader VM and production allocation paths remains outstanding.
[Interpreter validation](../../ristretto-gc-evidence/shared-java-statics-v1/functional-summary-v1.json)
and [combined source integration](../../ristretto-gc-evidence/statics-combined-main-v1/integration.json).

The combined source passed 119 native and 86 WASI canonical VM checks plus
384 native and 274 WASI GC checks. Strict lint passed on both platforms.
[Combined validation](../../ristretto-gc-evidence/integration-v11/summary.json).

The ordinary collector now samples physical recovery capacity outside the
allocation waiter mutex. A regression parks one thread during that sample,
cancels a real physical waiter and delivers a logical allocation wake from a
second thread. The old implementation timed out. The fix passed 383 native
and 273 WASI GC checks.
[Native checks](../../ristretto-gc-evidence/physical-probe-progress-v1/native.json)
and [WASI checks](../../ristretto-gc-evidence/physical-probe-progress-v1/wasi.json).

Earlier shared-heap integration passed 90 native and 68 WASI VM checks.
The native two-thread test completed 64 independent repetitions. The peer
matrix for the preceding dedicated-heap source completed 63 processes.
The persistent-heap results above extend that dedicated-heap comparison.
[Results and graphs](../benchmarks/gc/CANONICAL_MEASUREMENTS.md#integrated-peer-comparison).

With an expiring slot preference for an aged allocation request, the original
72-process leaf contention matrix passes all 24 native threaded, 24 native
cooperative and 24 WASI cooperative cases. The checker verified 4.95 million
destructions and complete refunds with the original watchdogs. Stalled and
cancelled nominees also release the sole available slot to other allocators
in a focused regression. General progress on arbitrary VM graphs remains open.
[Contention evidence and graphs](CANONICAL_CONTENTION.md#current-contention-results).

The earlier recovered leaf contention matrix completed 69 of 72 runs. All three
eight-mutator, one-helper native-thread runs exhausted one allocation request's
poll budget. Cooperative native and WASI passed all 48 runs. The 69 successful
runs verified 4.71 million payload destructions.
[Contention results](CANONICAL_CONTENTION.md#current-contention-results).

The allocator now checks a recently released slot between ordinary probes.
The candidate completed 71 of the same 72 schedules and verified 4.87 million
destructions. One native high-retention request exhausted its poll limit. The
combined tree passed 90 native and 68 WASI restricted VM checks, 633 native
and 503 WASI catalog/GC library checks, and 525 native JIT tests. Compiled
static access requests were integrated at this checkpoint. The subsequent
shared-class implementation is described above.
[Integration results](../../ristretto-gc-evidence/integration-v10/summary.json).

In the ordinary collector, the reference handler's notification now stays
available to other helpers if the first notifier stops before promoting the
waiters. The regression test publishes a real weak reference and delivers its
notification while the first notifier is parked. Progress calls with no pending
reference notice now use an atomic load. The source passed 382 native and 273
WASI GC checks. [Reference notification results](../../ristretto-gc-evidence/reference-notice-progress-v1/results.json).

Current integration and attribution (2026-11-20): closed-layout casts are merged
with 82 native VM checks, 62 executed WASI checks, three focused compiler checks
and strict lint on both targets. Nine independently validated diagnostic captures
identify request gaps and synchronous service polls as the dominant instrumented
entry costs. Heavy recording prevents an ordinary-runtime cost or pause claim.
Lower-overhead phase corroboration is in progress. The paired comparison of
ordinary request scheduling is reported above.
Reports and new phase/exclusive-cost graphs are linked from
[the current measurement record](../benchmarks/gc/CANONICAL_MEASUREMENTS.md).
The ordinary VM remains on its existing collector. Broader VM coverage, global
pause evidence, sustained load and endurance remain required; no full goal gate
is closed by this checkpoint.

Measured checkpoint (2026-11-16): passive primary-cycle observation,
recorded clock-window bounds and immutable metadata links are integrated. The
combined source passes 61 native and 32 WASI tests. Six instrumented captures
independently validate primary-cycle bounds; ordinary timed runs still have no
cycle observations, and global pause coverage remains incomplete. The isolated
metadata comparison reduces small-ring allocator requests by about 74%
interpreted and 70% compiled, with a separate admission/compilation cost.
The first finite G1/ZGC comparison completed 45 timing forks and 18 allocation
diagnostics. It exposes a substantial pre-optimization invocation gap; dedicated
heap setup/cleanup versus deferred peer collection prevents a collector-only
ranking. Source-bound reports, raw evidence and both graphs are linked from
[the current measurement record](../benchmarks/gc/CANONICAL_MEASUREMENTS.md).
The optimized source has now completed the same 63-process matrix. Native
invocation medians change from a 0.15% regression to a 1.92% improvement; WASI
improves 0.63–2.85%. These descriptive three-fork differences establish no clear
speedup despite the large allocation-count reduction. Next attribute
runtime/helper/frame/lifecycle costs, complete broader VM integration and qualify
pauses, resources and sustained load. Reference arrays are integrated with 72
native and 41 WASI combined-source checks and strict library/test lint; their
separate full-size Miri/ASan/TSan evidence passes, along with a finite Tree
follow-up whose production code is identical. PID/run-ID-bound lifetime resource receipts now pass five actual
runtime launch checks; the earlier timing matrices retain unavailable RSS data.
Defaults are unchanged, and no full performance or endurance gate is closed.

Parallel integration checkpoint (2026-11-15): the restricted canonical Java tier
now executes the shared javac graph workload with catalog-owned fields and rooted
frames on native and WASI. The separate ordinary-VM experimental JIT path verifies
compiled entries/backedges, HotSpot parity, suspension, cancellation and exception
ownership. A request-only compiler ABI removes synchronous legacy GC helpers from
the code intended for canonical execution. These checkpoints are integrated;
combined-source validation passes thirteen native and twelve WASI integration
cases, cancellation on both platforms, Clippy and formatting. See [canonical Java](../ristretto_vm/CANONICAL_JAVA.md)
and [experimental dispatch](../ristretto_vm/EXPERIMENTAL_JIT.md).

The compiled canonical bridge is now integrated: bounded token tables, owning
frame snapshots and request-only activations run the shared graph and reference
return fixtures without legacy GC handles. Its isolated native execution and
cancellation checks pass. The merged pause/clock configuration passes eighteen
native and thirteen WASI Java integration cases, five/three ownership cases,
699/381 GC test results, lint and benchmark-checker regressions. This is a
local-task, single-mutator, dedicated-heap tier with explicit
identity/work limits. Broader Java layouts, shared heaps and semantics remain.
Primitive arrays and numeric execution are now integrated. The merged source
passes 301 HotSpot oracle rows in each native tier and in the WASI interpreter,
23/17 Java integration cases, 7/4 ownership cases, 706/388 GC test results and
508 JIT test results. Floating conversions now saturate correctly and integer
returns apply descriptor-specific narrowing. The initial merge-format failure
and its module-order correction are retained separately in `integration-v3`.
The retained-ring workload additionally passes 21 cases on each execution path,
including reclamation while a live ring survives more allocations than catalog
capacity; see [the workload contract](../benchmarks/gc/canonical-ring-oracle/README.md).
These are correctness checks, not measured throughput or pause improvements.
The fixed-capacity service recorder, atomic managed-byte accessor, VM measurement
hooks and positive 63-bit token leasing are now merged. Their combined checkpoint
passes 29/21 native/WASI Java integration cases, 22/17 ownership/module cases,
targeted recorder/clock/accounting tests and lint with recording enabled/disabled.
The 18-process G1/ZGC allocation-exposure check passes all checksums and minimum
payload-demand checks; it supplies no throughput, latency or measured-window GC
exposure result. Independent service export now passes six combined-source
graph/array captures across native interpreted/compiled and WASI interpreted
execution, eleven checker tests and a real overflow rejection. Source, executable,
bytecode and launch receipts are retained outside build storage in
`ristretto-gc-evidence/integration-service-v1/`. That earlier schema has no
completed-cycle accounting; the current observer/schema update is described above.
Global pause coverage remains incomplete. Keep the
restricted runtime preview, integrated VM alpha and performance-qualified beta
as separate release candidates: the alpha requires the compiled canonical path;
the beta additionally requires short-load/resource/coverage/pause gates. Final
release still requires J5/H endurance qualification. No default backend changes
or performance/superiority claims follow from these correctness checkpoints.


Canonical caller API follow-up (2026-11-15): `experimental-canonical-runtime`
now exposes transparent typed roots/views and incremental allocation/read/write and
helper operations outside the GC crate. Native/WASI charged layouts are unchanged;
full regressions pass 686 native / 379 WASI tests. All six external fixtures pass
four memory tools, compiler controls enforce borrowed owners and one request per
mutator, and native/bare-Wasm smoke executions reclaim a real cycle. See
[the API contract](CANONICAL_API.md) and
[the evidence](../target/gc-implementation/canonical-client-REPORT.md).
VM value/array layouts and execution-context callers are still unintegrated. The
current degree 1–8/reference-free payload scope, callbacks, allocation overhead,
compaction, JIT and every full qualification gate remain open. Defaults are unchanged;
no performance improvement is inferred from this interface.

Shared-root accounting correction (2026-11-15): the final alias now uses
`Arc::into_inner` to free its backing before the unique root can refund catalog
credit. Ordinary Arc Drop in the preceding shared-root version reversed that order;
its native/WASI reproducer and historical evidence are preserved. The new leaf/graph
final-owner audit passes on native/WASI and all four memory tools, and restoring
the old teardown reproduces the defect. The 679 native / 373 WASI regressions,
88 affected fixtures per memory tool, 39 Python tests and all 90 contention
processes pass. Copy/drop measures about 10.6 ns native / 3.0 ns WASI with zero
copy allocation. Logical-root creation still adds an allocation, and native lifecycle
capacities 8/64/256 regress about 0.8%/0.7%/1.3% versus the verified phase baseline.
See [the corrected report and graphs](../target/gc-implementation/canonical-shared-root-v2-REPORT.md)
and [contract](CANONICAL_SHARED_ROOTS.md). This supersedes the preceding version's
complete resource-accounting acceptance; it closes no full qualification gate.
VM payload/caller integration, allocation costs, callbacks, compaction, JIT and
external/endurance qualification remain open. Defaults are unchanged.

Shared typed-root follow-up (2026-11-15): leaf/graph aliases now share one unique
root token, with last-owner cleanup and prepaid live/pending wrapper backing.
679 native / 373 WASI tests and the same 88 fixtures under four memory tools pass;
all 90 finite contention processes reclaim 6,927,000 real objects. Owning clone/drop
costs about 11.6 ns native / 3.15 ns WASI versus legacy 23.3 / 12.0 ns, with no clone
allocator work. Logical-root creation adds one allocation: the full lifecycle has
30,000 extra requests per 10,000 rounds and native capacities 64/256 regress about
1.3%/1.2%. Every sample and regression is retained in
[the report and graphs](../target/gc-implementation/canonical-shared-root-REPORT.md).
This is an admitted-caller prerequisite; root-creation overhead, VM payloads/callers,
general layouts, callbacks, compaction, JIT and every full qualification gate remain
open. Defaults are unchanged. See [the contract](CANONICAL_SHARED_ROOTS.md).

Phase-controller follow-up (2026-11-15): effect-free changes now use one prepaid
controller slot and two local facade yields before installation, preserving the
original preparation window. Effect-bearing operations retain their existing
protocol. The revised candidate passes the regression and memory matrix, 73
catalog/six pool models, 68 intended negative controls and all 90 finite contention
processes (6,927,000 real objects). All 2,112 accounting rows are unchanged.
The 224 paired timing processes retain every capacity and regression in
[the report and graphs](../target/gc-implementation/canonical-phase-REPORT.md).
The unpaced candidate and its regressions are preserved separately. Long native
requests remain: the finite stress maximum is 833,993 polls, not a latency bound.
VM payloads/callers, callbacks, compaction, JIT and all full qualification gates remain open; defaults are unchanged. See [the protocol](CANONICAL_PHASE_DECISIONS.md).

Bounded inventory cleanup and publication follow-up (2026-11-15): helpers now
observe at most eight consecutive empty physical root/edge/object slots per visit,
stopping on an occupied or failed observation. Prepared allocation roots are
helpable; owners recover their exact admitted, stably published result. Native
contention exposed a partial-effect race, now covered by a deterministic regression
and fixed by retrying while that root remains Prepared or Live. All 90 final finite native/WASI stress processes pass (6,927,000 real objects).
All regression and affected memory checks pass, with 17 intended fault processes.
At capacity 256, paired lifecycle time ratios are 0.673 native and 0.771 WASI (lower is better). All capacities, regressions and intervals are retained in the report.
Allocator request counts, requested bytes and peak bytes are unchanged. Remaining
small-capacity/controller and allocation/root/barrier costs, VM payloads/callers,
callbacks, compaction, JIT and every full qualification gate remain open; defaults
are unchanged. See [the contract](CANONICAL_SWEEP_BATCH.md) and
[the report and graphs](../target/gc-implementation/canonical-sweep-REPORT.md).

Source-indexed traversal follow-up (2026-11-15): the experimental graph runtime
now uses catalog-enforced source ranges, reducing a successful scan from E positions
to D. All 13 regression commands, affected memory checks, 68 corrected catalog models
and 12 intended native/WASI fault processes pass. Across 224 paired timing processes,
lifecycle time at capacity 256 falls about 19% native and 11% WASI; the capacity-2 WASI
result is inconclusive. All 48 allocator samples balance; request counts are unchanged,
with eight extra native bytes and no measured WASI increase. Sparse cleanup still
consumes about 94% of runtime. Next attribute and reduce remaining sweep/controller
costs. VM payloads/callers, callbacks, compaction, JIT and all full qualification gates
remain open; defaults are unchanged. See [the contract](CANONICAL_INDEXED_EDGES.md)
and [the paired report and graphs](../target/gc-implementation/canonical-indexed-REPORT.md).

Canonical-graph follow-up (2026-11-15): the internal experimental runtime now
combines real typed payloads, complete catalog-owned edges, coupled writes and
independently rooted child reads. Source-owned cycles reclaim before runtime teardown;
full-quota reads can observe a later null replacement. All 13 regression commands,
eight memory checks, 20 intended fault processes and 44 CLI checks pass. All 56 timing
processes and 24 allocator samples pass, but sparse-catalog cleanup costs dominate:
median lifecycle time rises from 12.3 to 89.2 µs/object native and 10.7 to 78.7 µs/object
WASI across capacities 2–256. At 256 slots cleanup consumes about 95% of runtime.
Next enforce source-indexed inventory and reduce measured sweep/controller costs.
VM payloads/callers, callbacks, JIT and every full qualification gate remain open;
defaults are unchanged. See [the contract](CANONICAL_GRAPH.md) and
[the report and graphs](../target/gc-implementation/canonical-graph-REPORT.md).

Canonical-edge follow-up (2026-11-15): the experimental catalog now couples
non-owning strong edges, mutation barriers and child-root activation with complete
Full-domain inventory traversal. Source-owned cyclic fixtures destroy their values
before catalog teardown. Final catalog tests pass 223 native / 206 WASI; 66 catalog
models, 42 distinct checker faults, seven portable edge faults and 840 accounting
rows pass. Traversal is still O(edge capacity) per source and metadata costs rise,
including with edges disabled. The real graph adapter, admitted VM callers, callbacks,
JIT and every full qualification gate remain open; defaults are unchanged. See
[the contract](CANONICAL_EDGES.md) and
[the evidence/cost plots](../target/gc-implementation/canonical-edge-REPORT.md).

The intermediate v2 screen requires sub-millisecond native stop-the-world (STW)
pauses and records comparative G1/ZGC results. Full release also requires zero
collector-wide stops and better sustainable throughput and scheduled tail latency
than both peers on the declared workloads. Correctness, reclamation
progress, resource accounting and endurance qualification remain required.
Current evidence does not establish the pause goal or complete qualification.

Controlled-leaf contention follow-up (2026-11-15): the optional real-value stress
driver checks retained identities, per-actor progress, actual destruction and full
slot/credit cleanup. All 78 final native/WASI processes pass (31.11 million objects),
as do 15 frozen commands, nine fault controls and 39 CLI checks. The original
large-catalog runs failed a fixed poll watchdog; capacity-scaled diagnostic limits
now expose, rather than remove, requests taking millions of polls. This closes no
latency or full sustained-load gate. Next implement complete graph authority and
admitted-caller integration; defaults remain unchanged. See
[the contract](CANONICAL_CONTENTION.md) and
[the evidence](../target/gc-implementation/canonical-contention-REPORT.md).

Bounded-root-scan follow-up (2026-11-15): the internal leaf helper now batches
at most eight root-slot observations, stopping at the first root/target effect.
The original single-slot API and controller/certificate protocol are preserved.
All 27 frozen commands, 59 catalog/six pool models, 32 checker fault controls and
eight additional native/WASI fault processes pass. Across 252 paired timing
processes, canonical lifecycle time falls 18–21% and cleanup time 24–27%, with
unchanged canonical allocator activity. It remains 3.65–4.31× legacy time native
and 2.10–2.20× WASI. General graph/callback/admitted-caller integration, JIT and
all full qualification gates remain open; defaults are unchanged. See
[the protocol](CANONICAL_ROOT_BATCH.md) and
[the evidence](../target/gc-implementation/canonical-cleanup-REPORT.md).

Concrete-leaf cost follow-up (2026-11-15): a feature-gated fixture now measures
allocation, rooting, typed reads, root drop and complete batch reclamation on
native/WASI. All 126 timed processes and 54 separate allocator rounds pass integrity
checks, but canonical lifecycle time is 4.5–5.3× legacy on native and 2.6–2.7× on WASI.
Cleanup consumes about three quarters of canonical time. Fewer system requests do
not offset protocol cost; reducing this cleanup cost is the next measured target.
The 14-command correctness matrix, six intended fault failures and 52 CLI checks
pass. This closes no graph/callback, admitted-caller, JIT, pause, G1/ZGC or endurance
gate. See [the cost report](../target/gc-implementation/canonical-leaf-cost-REPORT.md).
Defaults remain unchanged.

Model-runner follow-up (2026-11-15): the checked-in tagged catalog checker now
reproduces the current 55 catalog and six pool models, all 25 historical faults and
three new retry faults. All 30 executed cases and 15 Python evidence-boundary tests
pass. Mutation preflight, isolated workspace copies, per-variant build storage,
strict result checks and executed-binary hashes make this evidence reproducible.
This tooling closes no GC performance or endurance gate; defaults are unchanged.

Automatic-allocation follow-up (2026-11-15): internal leaf requests now retain unique
payloads across object/root slot collisions and full/contended root quota, probing
one candidate per poll. All 39 frozen commands, 55 corrected models, 18 intended
runtime/model faults and four compiler rejections pass. Existing backing is unchanged;
participant hints add 16 native / 8 WASI bytes, and 816 measured catalog lifecycle
rows request no new backing. Public selection, scheduling, graphs, callbacks,
admitted callers, JIT and qualification remain open. See
[the protocol](CANONICAL_RETRY.md); defaults are unchanged.

Runtime-owner follow-up (2026-11-15): an internal leaf owner now binds participants
and reads to one heap/catalog and prepays bounded owning-wrapper storage. The
35-command matrix, ten intended fault processes and eight compiler rejections pass;
52 unchanged transition models are reused with exact input hashes. Existing catalog
backing remains unchanged; 72 additional allocator rows and 48 production-layout
rows record the supplement. Public selection, automatic slot retry, scheduling,
graphs, callbacks, admitted callers, JIT and qualification remain open; see
[the protocol](CANONICAL_RUNTIME.md). Defaults are unchanged.

Maintenance follow-up (2026-11-15): reserved leaf helpers now perform incremental
root/object/phase cleanup and require complete retirement sweeps. A reproduced
stale Finish race is fixed by retaining its epoch. The 31-command matrix, ten
follow-up checks, 52 corrected models, 20 intended runtime/model fault processes
and four compiler rejections pass. Catalog accounting is unchanged. Public heap
binding/runtime accounting, scheduling, graphs, callbacks, admitted callers, JIT
and qualification remain open; see [the protocol](CANONICAL_MAINTENANCE.md).
Defaults are unchanged.

Allocation-transaction follow-up (2026-11-14): real private preparation now composes
with one bounded request and an internal typed rooted result. Durable cancellation
covers the record-before-root interval and discarded committed results. The scoped
28-command matrix, seven final catalog checks, 50 corrected models, 14 intended
runtime/model fault processes and seven compiler rejections pass. New catalog-only
lifecycle rows allocate no backing; some large-record setup costs rise. Public
selection, fair maintenance, graphs, callback bounds, admitted callers, JIT and all
qualification remain open; see [the protocol](CANONICAL_ALLOCATION.md). Defaults are
unchanged.

Typed-leaf access follow-up (2026-11-14): canonical roots now support internal
borrowed views with exact heap, publication and type checks. Eighteen matrix
commands, six strict-provenance fixtures, 48 scoped corrected models, nine intended
runtime/memory faults and six compiler rejections pass. Retained views permit
unrelated reclamation after revocation. Public allocation/result transfer, graphs,
callbacks, admitted callers, JIT and all qualification remain open; see
[validation](VALIDATION.md#canonical-leaf-root-access-2026-11-14). Defaults are unchanged.

Concrete-leaf follow-up (2026-11-14): an optional internal adapter now transfers
eligible private allocations into canonical records without legacy registration.
Sixteen native / 13 WASI fixtures cover real ownership, cancellation, stopped
admission/destruction and unrelated reclamation; scoped memory tools, sixteen
runtime fault processes and six compiler rejections pass. Public root/result and
graph integration, admitted callers, JIT and all qualification gates remain open;
see [the protocol](CANONICAL_LEAVES.md). Defaults are unchanged.

Canonical-reference follow-up (2026-11-14): the isolated catalog now couples
conditional decisions, construction lifetimes and notification/root transfer with
phase work. Separate bounded quotas and embedded reserved roots allow helpers to
finish stopped origins without consuming caller-root capacity. Eighteen matrix
commands, 48 scoped corrected models, 36 runtime/two model faults and nine compiler
rejections pass; 528 accounting rows show zero lifecycle requests and higher backing.
Actual GC graph/cleanup/root-result integration, admitted callers, JIT and all
qualification gates remain open; see
[validation](VALIDATION.md#canonical-conditional-references-2026-11-14).

Prepared-record follow-up (2026-11-14): real private allocations now reserve unique
record tokens and bind once after matching-heap graph admission. Unbound candidates
can be discarded; bound identity is never reset. The two scoped 13-command matrices, six corrected
models, 13 intended model/runtime failures and three compiler checks pass. This does
not yet publish payloads through the catalog or change constructors. Coupled roots,
real collector integration and all qualification remain open; see
[validation](VALIDATION.md#prepared-payload-record-identity-2026-11-14).

Canonical-root follow-up (2026-11-14): the isolated catalog now couples root
publication/removal, deletion work and per-domain enumeration with its controller.
Unique token Drop is durable; bounded root handles retain quota and catalog charge
through final destruction. Fifteen matrix commands,37 models,14runtime/five model
faults and five compiler checks pass. The288allocator rows show zero lifecycle
requests and increased backing for roots/retained targets. Real collector root/result
transfer, traversal/references, admitted callers and every qualification gate remain
open; see [validation](VALIDATION.md#canonical-root-ownership-2026-11-14).

Bounded-record follow-up (2026-11-14): generic participant requests preserve record
ownership across transient retries and restrict publication to guarded commands.
Actual record layouts are charged, fixing a unit-layout undercount exposed by a new
fixture. Thirteen matrix commands,31 catalog models, eight runtime fault processes
and three compiler checks pass;72 new accounting rows match allocator requests and
24 numeric rows are unchanged. Concrete payload binding/graph integration and all
qualification gates remain open; see
[validation](VALIDATION.md#bounded-opaque-record-requests-2026-11-14).

Record-admission follow-up (2026-11-14): the isolated catalog retains generic records
and helper guards through canonical effects and certification. Preflight rejection
returns the input record. The two scoped matrices, 31 corrected models, 17 runtime/
model negative processes and three compiler checks pass. Numeric backing/refund rows
are unchanged. Concrete GC composition, unique payload binding, bounded record
participants, roots/references and full qualification remain open; see
[validation](VALIDATION.md#guarded-retained-record-publication-2026-11-14).

Domain-state follow-up (2026-11-14): the isolated catalog now couples publication
births, generation, domain scope and trace tickets with both pending counts. Explicit
start/finish and promotion have eleven matrix checks, 27 corrected models and 22
expected negative processes. Zero lifecycle allocations remain, with 8.1–12.5% more
requested setup backing. Actual graph/metadata, roots, reference decisions and every
qualification gate remain open; see
[validation](VALIDATION.md#canonical-numeric-allocation-domains-2026-11-14).

Announced-storage follow-up (2026-11-14): descriptor owners can survive private
cancellation as inert metadata. Exact-header views require admitted helpers, and
one atomic word prevents announcement/restoration races. Thirteen matrix commands,
nine corrected models and seventeen runtime/model negative processes pass their
expected checks, with two lifetime compiler checks. Canonical graph/phase/reference
integration, physical backing accounting and full qualification remain open; see
[validation](VALIDATION.md#announced-private-descriptor-storage-2026-11-14).

Erased-owner follow-up (2026-11-14): prepared allocations of different Rust types
can share one header interface without another allocation. Fourteen shared fixtures
pass under both Miri modes and sanitizers; all 13 matrix commands pass, with six
runtime negative failures and two lifetime compiler checks. This is private
ownership only. Announced-storage cancellation, canonical graph/phase/reference
integration and qualification remain open; see
[validation](VALIDATION.md#heterogeneous-prepared-publication-headers-2026-11-14).

Commit-decision follow-up (2026-11-14): the isolated catalog now prepares the effect
owner before committing, leaving failed preparation cancellable. Exact-version
CAS and certification then require no fallible acquisition. The frozen red fixture,
eleven-command matrix, 22 corrected models and seven negative processes establish
that scoped boundary. Actual graph/phase/reference integration and every full GC
qualification gate remain open; see
[validation](VALIDATION.md#effect-ownership-precedes-the-commit-decision-2026-11-14).

Shared-view follow-up (2026-11-14): an admitted handle now borrows prepared metadata
without taking the private result lock. The 13-command matrix passes, including all
eleven shared fixtures under both Miri modes and sanitizers. Runtime and compile-fail
controls verify lock independence and handle-bound lifetimes. Canonical cancellation,
phase/graph integration and qualification remain open; see
[validation](VALIDATION.md#borrowed-shared-descriptor-views-2026-11-14).

Shared-owner follow-up (2026-11-14): admitted helper handles can own a prepared
graph without a strong collector cycle. Last-handle cleanup preserves root/destructor
ordering and retains payload credit through a stalled destructor. The 13-command
matrix, four models, scoped Miri reuse and seven distinct fault variants pass their
expected checks. Ordinary publication is unchanged; canonical phase-coupled commit,
participant/backing bounds and all qualification gates remain open. See
[validation](VALIDATION.md#shared-private-publication-ownership-2026-11-14).

Reference-free scope follow-up (2026-11-14): vector storage is restored and an
optional scope bypasses TLS for explicitly reference-free constructors. The original
type layouts are preserved. All 25 declared checks and five native/WASI fault pairs
pass. Explicit-leaf operation time falls 1.5–3.0% native (one interval includes zero)
and 1.6–5.6% WASI (all intervals below zero). Retain this candidate for integration,
including its WASI baseline-allocation and leaf-write control regressions. See
[validation](VALIDATION.md#optional-reference-free-construction-scopes-2026-11-14).
Shared publication and complete performance/progress qualification remain open.

Inline-capture experiment (2026-11-14): the four-registration inline representation
passes the 31-command matrix and saves about four requests per nested construction,
but slows allocation by 6.5–8.7% native / 7.0–9.5% WASI and also slows nested
construction. It is rejected as a performance candidate. The immutable candidate,
expanded explicit-leaf benchmark, faults, type-size costs and all controls are in
[validation](VALIDATION.md#rejected-inline-construction-capture-2026-11-14).
The follow-up restores vector storage and isolates the TLS bypass. No goal gate closes.

Generation-budget follow-up (2026-11-14): slot setup now encodes the allocation
budget in its starting generation, removing a pool field and per-allocation read.
The default still starts at generation 1. The final 22-check matrix passes with
six current pool models and scoped reuse of the unchanged 16 catalog models.
Actual backing shrinks by 24 bytes per catalog on both targets. The paired screen
lowers native lifecycle cost by 3–4% while retaining small WASI regressions. See
[validation](VALIDATION.md#generation-budgets-at-setup-2026-11-14).
This closes a local cost follow-up; canonical reachability/reference integration,
all admitted callers and full GC qualification remain open.

Finite-generation follow-up (2026-11-14): the isolated facade now rejects private
preparation after observing permanently exhausted slot backing, retaining valid
owners and allowing installed work to finish. A reduced-generation red driver,
22-command native/Wasm/memory-safety matrix, 22 models and 23 distinct negative
variants validate the scoped behavior. Zero loop allocation counts remain, with
56 native / 44 WASI additional facade backing bytes. The paired screen retains a
2.9% native regression at 16,384 records and mixed results elsewhere. See
[validation](VALIDATION.md#finite-catalog-generation-exhaustion-2026-11-14).
This is conservative selected-slot rejection, not rotation, indefinite service or
contention fairness. Production graph/reference integration and qualification remain open.

Catalog-charge follow-up (2026-11-14): the isolated facade can reserve calculated
backing and participant storage from an external logical budget. Opaque handle
teardown refunds after pool and outer-Arc deallocation. Native/WASI allocation
and ordering audits pass, including two deliberately broken accounting variants;
the full 14-command matrix passes. The cost screen retains a small native regression
and opposite WASI facade/control changes. See
[validation](VALIDATION.md#charged-catalog-backing-2026-11-14).
Finite exhaustion, real heap integration and qualification remain open.

Participant-poll follow-up (2026-11-14): pending numeric requests now remain in
place, and successful preparation reuses its owning membership observation.
The validated paired screen lowers facade lifecycle time by 19–20% native and
about 26% WASI with unchanged backing/allocation counts. The facade still costs
more than the raw API. See [validation](VALIDATION.md#retained-participant-polls-2026-11-14).
Byte charges, exhaustion, real collector integration and qualification remain open.

Participant-limit follow-up (2026-11-14): the isolated numeric catalog now offers
non-cloneable participants, separate helper registrations, derived slot reserves
and resumable private cell creation. Owning observations cannot escape this facade.
The native/WASI, Miri/Tree Borrows and sanitizer matrix passes; stopped ordinary
participants leave capacity for an active reserved helper in the forced fixtures.
This is not a byte budget, contention-fairness proof or production integration.
See [validation](VALIDATION.md#bounded-catalog-participants-2026-11-14).

Private-preparation follow-up (2026-11-14): the isolated tagged catalog retains
partial control allocations across bounded polls, closing a reproduced quiescent
fragmented-slot livelock. Native/WASI, Miri/Tree Borrows, sanitizers and 14 protocol
models pass. The cost screen retains a 9–10% native / 15–16% WASI lifecycle-time
regression with unchanged allocation/backing counts. See
[validation](VALIDATION.md#resumable-private-catalog-preparation-2026-11-14).
The subsequent facade enforces numeric participant quotas; charged progress reserves
and real collector integration remain open.

Initial-root cleanup follow-up (2026-11-14): rooted constructors prepare metadata
before installing their private parent root and retain metadata ownership across
publication. Cancellation removes that exact root before private payload destruction,
while child construction protection retains its existing lifetime. All 26 check
commands pass. The paired screen lowers allocation-operation medians, but retains
mixed native wall times and a 4.3% WASI candidate construction-wall regression.
See [validation](VALIDATION.md#ordered-initial-root-cleanup-2026-11-14).
Shared request ownership, admitted publication progress and qualification remain open.

Construction-root follow-up (2026-11-14): nested scope exit moves registration
batches and reuses empty-parent backing; explicit detached cleanup preserves
deletion barriers without adopting into a helper's ambient scope. The isolated
screen reduces nested-constructor operation time by 9–16% native and 10–13% WASI,
while retaining several WASI write-control regressions. A separate audit reproduces
a pre-existing initial-parent-root cancellation gap, repaired in the follow-up above.
See [the original evidence](VALIDATION.md#construction-root-batches-2026-11-14).
No publication-progress or qualification gate is complete.

Stage D refund follow-up (2026-11-14): scoped cleanup now covers diagnostic unwind
from payload destruction and finalization. Sweep and deferred reclamation validate
and borrow their collector to avoid a weak upgrade during refund. The final
22-command matrix passes, including nine focused Miri/Tree Borrows fixtures. The
paired screen improves native collection-inclusive allocation time by 0.2–1.4%
against the descriptor-owned-charge revision, but retains a 9–16% WASI leaf-write
control regression. See [the complete evidence](VALIDATION.md#guarded-refunds-and-borrowed-reclamation-2026-11-14).
These are ownership/cost results; initial-root migration, canonical publication
and qualification remain open.

Stage D runtime prerequisite (2026-11-14): logical allocation charges now belong
to metadata before publication and are refunded after payload destruction. A
preserved failure shows that constructor unwind previously refunded a registered
object. Native/WASI, Miri/Tree Borrows and sanitizer checks pass; the isolated cost
screen retains a native collection-inclusive regression and mixed WASI results.
See [validation](VALIDATION.md#descriptor-owned-allocation-charges-2026-11-14).
This does not transfer initial roots to a shared request or integrate the numeric
catalog. No milestone or qualification gate is completed by the ownership repair.

Stage D design follow-up (2026-11-14): isolated
[catalog/phase experiments](validation/catalog/README.md) now couple discoverable
numeric membership, durable work and both domain counts through a prepared,
helpable operation. Eleven native fixtures, eight models, WASI and both Miri modes
pass; three broken variants fail. A 60-process allocation-count screen favors
the descriptor registry over immutable tree copying at larger catalog sizes, but
still records six allocations per new descriptor/insertion. These packages are
not used by the collector. Real metadata/initial-root ownership, byte budgets,
the complete reference-floor rules and migration of every admitted operation remain
required before the current stalled-publisher dependency can be removed.

The subsequent [tagged catalog experiment](validation/catalog/catalog-tagged-prototype/README.md)
uses bounded atomic ownership acquisition for all numeric owning links, including
optional discovery and cells. Acquisition failure preserves Pending requests and
remains distinct from empty discovery slots. Native/WASI, Miri/Tree Borrows and
sanitizer checks pass. The allocation screen removes the remaining per-cell system
request at the cost of greater preallocated backing and bounded-search retries.
The paired numeric-catalog screen lowers lifecycle time by 51–53% native and
24–25% WASI, while setup backing at 16,384 residents grows about 40%/79%.
See [the evidence and retained failures](VALIDATION.md#tagged-catalog-integration-2026-11-14).
The collector does not use this catalog. Participant limits, reserved progress
capacity, real graph/phase integration and qualification remain required; arbitrary
value destruction and caller retries are outside the primitive's atomic-attempt bound.

Stage D update (2026-11-13): an owning identity-index prototype is integrated behind
`ristretto_gc/experimental-identity-index`, disabled by default. Exact-link deletion,
resumable sweep removal and cancellation-safe maintenance have executable evidence.
Embedded link markers reduced the initial representation's allocations. Shared
insertion now adds unique bucket versions and immutable retry proposals, so another
helper can finish a stopped insertion without permitting replay after retirement.
This is a scoped index primitive: collector registration still keeps its intent
local and retains admission. The existing sharded identity maps remain selected.
The next catalog work must couple canonical membership, durable traversal visibility
and phase admission, and account for all backing. An optional identity lookup index
alone completes none of those requirements. See
[the protocol](PUBLICATION.md#experimental-owning-identity-index) and
[validation](VALIDATION.md#shared-identity-insertion-intents-2026-11-13).

The next delivery combines the [whole-JVM execution workstream](#whole-jvm-execution-workstream)
with [collector overhead, reclamation progress, and external
qualification](#next-delivery-sequence-overhead-progress-and-external-qualification).
The runnable [GC comparison suite](../benchmarks/gc/README.md) adds identical Java
workloads on Ristretto, HotSpot ZGC, and HotSpot G1, with separate native and WASI
results and exported graphs. Its runtime measurements complement the existing
collector drivers; they do not isolate GC from interpreter/JIT costs. Neither the
suite nor this plan establishes that Ristretto meets the acceptance criteria below.

Scope includes production JIT implementation and interpreter/runtime overhead,
not only collector changes. Native external qualification requires safe compilation
of the declared application hot paths, evidence that those paths execute compiled
code, and integrated runtime/GC measurements. J0–J5 below are required new work;
enabling the existing JIT flag or passing backend tests does not complete them.

Status: implementation in progress. Integrated work includes stall diagnostics,
direct payload construction, indexed region selection, root-registration transfer,
sparse card bitmaps, dirty-chunk relocation, and async allocation recovery for the
interpreter, JIT helpers, native object/array publication, standard Rust vector
arguments, reflective arrays, `Object.clone()`, strings, and runtime constructors. Logical
allocation waits now resume during reclamation, share outstanding cycle requests,
and reserve bytes before publication. Clearing leaf objects uses their useful
work budget. Opt-in prediction now samples allocation rate, reclamation latency,
survival, and region fragmentation to request full cycles before exhaustion. Owning
roots can now opt into reusable sharded slots, with task-independent ownership and
bounded enumeration and address mixing independent of sweep shards;
registered execution contexts now own slots across migration and cancellation.
Opt-in epoch-tagged context buffers retain snapshot edges and dirty-owner records,
with lossless overflow and pending-work checks at phase boundaries. Tracing can
opt into bounded batches with all reserved tasks visible to termination, and object
statistics accumulate per step. Independent marking steps can now claim work
without the coordinator, using a claim/closure handshake. Lost headroom offers
survive notification so competing allocators do not prematurely spend recovery
cycles. Object, identity, young and remembered indexes can now opt into independently
locked shards, with bounded full-heap and generation-index enumeration. Opt-in adaptive native worker activation uses allocation debt, queued work, and
available CPUs with hysteresis; callback storage leases defer individual frees.
Opt-in persistent worker tracing queues now support bounded stealing and per-generation queued/in-flight accounting. Relocation
can also use persistent queues and independent claims. Full cycles can opt into
cost-aware source selection, hot/pin feedback, and a protected shared evacuation
reserve, with backing refunded after actual deallocation. Large async initialization yields between chunks and supports
cancellation. Sharded region registries and independent phase units now distribute
enumeration and reclamation. Conservative old-only passes permit unrelated young
progress and preserve precise full recovery/reference processing. Primitive buffers
skip reference-free tracing and retain bounded destructor work. No complete
milestone or overall performance target has been qualified yet. The default publication gate and synchronous embedding paths remain in use; experimental counted publication now replaces the gate without changing defaults.

Implemented items are checked below; partially implemented items remain unchecked.
[VALIDATION.md](VALIDATION.md) records checks and measurements for each batch.

Improve application throughput and tail latency by reducing allocation overhead,
shared synchronization, and synchronous collection during allocation recovery.
Retain stable descriptors while replacing the global publication gate with a
concurrent publication and reclamation protocol. Implement GC-safe compiled frames,
control flow and calls, then reduce measured compiler, interpreter and runtime costs.
Attribute collector and execution improvements separately before qualifying the
combined JVM against HotSpot G1 and ZGC.

The native full-release target requires zero GC-induced global stops and higher
sustainable throughput and lower scheduled p99.9 and p99.99 than both HotSpot
G1 and ZGC under equal resources. The intermediate v2 screen measures each
observed pause including rendezvous time against a one-millisecond ceiling.
Continue removing heap-wide publication blocking and full-cycle assistance on
ordinary allocation paths inside the measured operating range. The current
collector does not explicitly suspend every application thread; publication locks,
payload locks, and synchronous recovery are the relevant sources of GC-induced
stalls. Removing these sources does not guarantee progress when allocation exceeds
reclamation capacity or the live set exhausts the heap.

Threadless wasm retains cooperative execution with bounded work units. It cannot
execute application code and collection simultaneously. Native and wasm results
must therefore state different latency contracts.

## Existing behavior and constraints

[ARCHITECTURE.md](ARCHITECTURE.md) describes implemented behavior;
[VALIDATION.md](VALIDATION.md) records existing measurements and release gates.
Keep those documents factual as each milestone lands. This plan describes proposed
changes rather than capabilities already available.

Preserve these invariants throughout implementation:

- Stable, non-reused object identities and validated runtime handles remain valid
  across relocation. Arbitrary Rust values stay pinned.
- Roots protect locals, native calls, exceptions, suspended futures, and escaped
  host handles. Root ownership follows execution contexts across OS threads.
- Every required deletion/publication barrier survives buffering, overflow,
  cancellation, and collection phase changes. New allocation construction edges
  remain protected until publication completes.
- Weak upgrades, logical clearing, finalization, resurrection, and phantom handling
  retain the semantics tested with JDK 8, 11, 17, 21, and 25.
- Failed allocation or relocation preserves the source and runs each destructor
  exactly once. Reserved credits and copying destinations count against their
  respective memory budgets.
- Cooperative targets require neither OS threads nor a host clock on bare wasm.
  Collection never runs while the caller holds managed guards that prevent progress.

## Success criteria

The full project goal requires zero GC-induced global stops, including
equivalent blocking when a publisher or callback stalls. It also requires higher
sustainable completed throughput and lower scheduled-arrival p99.9 and p99.99
latency than both G1 and ZGC on the declared native workload matrix under equal
resource limits. Native and Wasm must retain correctness and reclamation
progress, with host-turn responsiveness measured on Wasm. The existing v2
short-run policy below remains an intermediate milestone. Its sub-millisecond
stop allowance and diagnostic peer ratios leave both full-goal claims open.

Freeze the baseline, qualification host, workload matrix, and measurement noise
tolerance in milestone 0. Use identical CPU limits, logical heap limits, and process
memory caps for comparisons. Include metadata, cached allocation buffers, retired
storage, and relocation reserves in physical-memory reporting.

| Measure | Intermediate v2 and supporting checks |
| --- | --- |
| STW pauses | Prefer no GC-induced global stop. If STW phases exist, require every observed native pause <1 ms, including rendezvous time, in each qualification run within the declared operating envelope. Measure count, total stopped time, pause distributions and observed maximum; missing telemetry or unbounded global blocking cannot pass. |
| Collector attribution | Compare throughput and operation latency with the frozen Ristretto baseline to identify costs and regressions. Fixed percentage improvements over that baseline are not release requirements; the whole-JVM, pause, correctness and resource gates determine acceptance. |
| Compiled execution | J0–J3 establish safe production support and measured compiled execution for every required hot method in the predeclared native workload manifest. Unexpected fallback or missing coverage evidence prevents qualification. |
| Whole-JVM performance | Establish sustainable completed throughput and scheduled-arrival p99.9/p99.99 latency on the declared native primary workloads within total CPU/physical-memory ceilings, with bounded backlog and memory. Report independent-run confidence intervals, baseline changes and G1/ZGC comparisons under equal resources; no peer-relative ratio or superiority is required. Complete all J5/H gates. |
| Operation latency diagnostics | Continue reporting the historical p99.9 <100 µs and p99.99 <1 ms collector-workload targets for context. They are not STW measurements or additional release gates. |
| Allocation recovery | No synchronous full-cycle recovery on normal VM allocations within the declared sustainable range. Report all waits and assistance, including overload behavior. |
| Memory and correctness | No lost reachable objects, invalid references, duplicate destruction, or leaked objects/ranges after teardown. Retained caches and deferred frees have explicit accounting and release behavior. |
| Compaction | Preserve the existing gate of reclaiming at least 90% of theoretically reclaimable regions in the equal-size fragmentation fixture. |

Under the intermediate v2 policy, comparisons with G1 and ZGC are descriptive
and observed native pauses must stay below one millisecond. Historical runs
retain their original criteria. Full project acceptance additionally requires
zero GC-induced global stops and a measured win over each peer on sustainable
throughput and scheduled-arrival tails under the declared resource limits.

## Milestones and dependencies

Deliver each milestone in independently reviewable changes. Keep experimental
algorithms selectable until their correctness and qualification gates pass.

| Milestone | Deliverable | Prerequisites |
| --- | --- | --- |
| 0 | Stall instrumentation and reproducible baseline | None |
| 1 | Direct payload allocation and local allocation caches | 0 |
| 2 | Reusable root slots and buffered mutation barriers | 0 |
| 3 | Predictive scheduling and bounded allocation recovery | 0 and 1 |
| 4 | Distributed collector work and metadata | 0 and 2 |
| 5 | Concurrent publication and reclamation without the global gate | 2 and 4; integrate pressure accounting from 3 |
| 6 | Old-only tracing and narrower generation interlocks | 5 |
| 7 | More effective payload relocation | 1 and 3; access-barrier extension also requires 5 |
| J0 | Execution attribution and compilation coverage | 0; coordinated with stage A |
| J1 | GC-safe compiled-frame roots and runtime boundaries | J0 and the root/publication contracts from 2 and 5 |
| J2 | Correct compiled control flow and reference operations | J1 |
| J3 | Calls, exceptions and safe production eligibility | J1 and J2 |
| J4 | Measured JIT, interpreter and runtime overhead reduction | J0; compiled-path changes require J1–J3 |
| J5 | Integrated whole-JVM coverage and performance qualification | J0–J4 and the chosen collector integration from stage G |
| 8 | Endurance qualification and default-policy decisions | 0 through 7 required deliverables; J0–J5 for native external goals; stage H |

### Milestone 0 Measure stalls and establish the baseline

Primary files: [collector.rs](src/collector.rs), [config.rs](src/config.rs),
[cell.rs](src/cell.rs), [gc_endurance.rs](../ristretto_classloader/examples/gc_endurance.rs),
and [VALIDATION.md](VALIDATION.md).

- [x] Separate caller assistance time, collector CPU time, publication-lock waits,
  payload-lock waits, allocation waits, and collection wall time. Preserve or
  deprecate the existing statistics API explicitly; `total_pause_time_us` must not
  be presented as an observed global pause. Optional `Diagnostics` separates
  contended lock waits, caller/worker steps, and full-cycle recovery; worker CPU is
  available on Unix. Existing cycle-wall statistics remain separate. Bare wasm
  records counts without calling a host clock.
- [ ] Add counters for barrier flushes, full-cycle recovery, copied/re-copied bytes,
  skipped relocations, retired storage, and useful worker work versus contention.
  Keep instrumentation allocation-free on hot paths and measure its overhead.
  Counters are implemented, including actual guard holds and old handoff/tracing.
  The predeclared 54-run overhead stage compares unrecorded, diagnostic-only and
  diagnostic/telemetry cases; completion of those measurements remains pending.
- [x] Record source revision plus the working-tree snapshot, compiler, OS, CPU,
  memory caps, worker/mutator counts, seeds, and complete configuration with results.
  `validation/provenance.py` freezes full source archives and binaries; `measure.py`
  verifies their hashes, records RSS histories, and can impose the same per-child
  virtual-address-space cap on both policies. Source hashes now include Java,
  workflow inputs and the predeclared qualification policy. This is an address-space
  cap, not an RSS cap; the managed arena cap and peak RSS remain separately reported.
- [ ] Measure sustainable capacity by increasing offered load while checking that
  backlog and live/retired memory stabilize. Measure latency from scheduled arrival
  through completion so stalled producers do not hide requests that would have arrived.

Exit gate: repeated baseline runs reproduce the workload and expose allocation
recovery and lock stalls separately. Store raw results and the predefined noise
tolerance in `validation/`. Preserve earlier results as separate snapshots.

### Milestone 1 Allocate directly into managed storage

Primary files: [buffer.rs](src/buffer.rs), [region.rs](src/region.rs),
[gc.rs](src/gc.rs), [metadata.rs](src/metadata.rs), and Java object/array constructors
in `ristretto_classloader` and `ristretto_vm`.

- [x] Add fallible direct region initialization for Java fields and arrays, avoiding
  the host allocation followed by a whole-payload copy in `HeapBuffer::prepare`.
  Preserve existing host-backed constructors for embedding compatibility.
- [x] Track initialized elements during construction so initializer failure or
  unwinding destroys only initialized values and releases the reserved range.
  Retry backing allocation before invoking the initializer, so side effects run
  at most once per element.
- [x] Chunk large initialization at VM scheduling boundaries while keeping the
  unfinished object unpublished; test cancellation of the suspended initializer.
  Async field/array constructors yield between 64 KiB batches (at least one
  element). Cancellation and errors release initialized values and physical ranges.
- [x] Index available regions by usable space/alignment rather than searching all
  regions under one mutex. Move backing allocation outside shared critical sections
  using reservations that prevent physical-budget oversubscription. Selection is
  indexed by largest free range, with alignment checked in each candidate. Backing
  deallocation also runs outside the registry lock; credits remain reserved until
  it finishes. Payload and descriptor arenas share those backing credits.
- [x] Add local small-payload allocation buffers and descriptor slabs.
  `local_allocation_buffers(true)` enables separate 16 KiB context-local payload
  and descriptor ranges for allocations up to 1 KiB, aligned up to 64 bytes.
  Logical credits reserve centrally in 4 KiB batches and remain charged while
  unused. Large/overaligned allocations bypass the caches; small heaps fall back
  to exact reservations. Context-handle detach, incremental context enumeration,
  refill failure, and shutdown release unused ranges/credits. Descriptor and
  payload arenas share a physical backing cap but use separate regions so pinned
  descriptors cannot strand movable payload space. Global descriptor fallback
  remains outside arena accounting. Alignment, exhaustion, cancellation/unwind,
  concurrency, and exactly-once destruction pass; performance remains unqualified.

Exit gate: eligible constructors perform no staging copy; alignment, partial
initialization, budget exhaustion, and exactly-once destruction tests pass. At
equal memory budgets, allocation traffic and latency improve beyond baseline noise.

### Milestone 2 Localize roots and mutation records

Primary files: [root_guard.rs](src/root_guard.rs), [roots.rs](src/roots.rs), [gc.rs](src/gc.rs),
[cell.rs](src/cell.rs), [collector.rs](src/collector.rs),
[value.rs](../ristretto_classloader/src/value.rs), and
[array_elements.rs](../ristretto_classloader/src/array_elements.rs).

- [x] Transfer an existing `GcRootGuard` registration into its owning handle.
  Cloned guards share one registration; replacing a guard's handle still registers
  the replacement identity correctly. This removes a redundant root operation but
  does not implement execution-context root slots.
- [x] Add selectable reusable root slots for owning handles and guards. Slot
  locations carry checked 64-bit generations; sparse enumeration retains cursors
  and publication/deletion barriers protect reuse behind either generation's scan.
  Pages are sharded by allocation address, independent of OS-thread ownership.
  Tests cover cross-thread future cancellation, guard transfer/replacement, stale
  removal, page-number wrap, late publication, and bounded cache retirement.
  Numeric root IDs retain their compatibility tree. Measured throughput gains are
  small and native recovery tails regress, so tree storage remains the default.
  Registered contexts and barrier buffers are implemented below; these earlier measurements cover only shared slots.
- [x] Introduce reusable root slots owned by registered execution contexts. Define
  attach, transfer, suspend, resume, cancellation, and detach behavior. Suspended
  futures keep enumerable roots without depending on their original OS thread.
  `RootStorage::Contexts`, `register_context`, and per-poll `ExecutionContext::scope`
  implement this ownership. Java method execution installs its context per poll.
  Escaped owning roots retain slots after task detach; the last owner releases
  cached pages. Unscoped and numeric-ID operations remain compatible.
- [x] Buffer snapshot-preserving edges and dirty-card notifications locally, with
  epoch tags for both active collection domains and a lossless overflow path.
  Flush or transfer records before retiring a context or acknowledging a phase.
  `barrier_buffer_size(0..=64)` defaults to immediate barriers. Nonempty buffers
  have collector ownership; cancellation needs no extra poll. Draining rotates
  among contexts and charges one record per collector work unit. Overflow flushes
  a bounded batch, retries once, then publishes directly if necessary. Terminal
  reference/sweep phases use immediate barriers to avoid repeated buffer refill.
- [x] Replace tree-based card tracking with compact bitmaps where appropriate.
  Each sparse tree entry now holds 64 cards. Existing managed guards still
  serialize stores with card cleanup; collector notifications are immediate by
  default or use the bounded context buffers described above.
- [x] Retain the current publication gate while integrating local buffers into
  mark termination and late-work checks. Preserve the distinction between a traced
  heap edge and an owning root when optimizing `Value` copies. Pending records stay
  charged through mark/remembered publication. All terminal checks recheck the
  count under the writer. See [PUBLICATION.md](PUBLICATION.md) for ordering and
  model scope. The milestone's performance exit gate remains unqualified.

Exit gate: tests cover tiny-buffer overflow, phase changes during a flush, root-slot
reuse, task migration/cancellation, suspended roots, and concurrent card cleanup.
Ordinary stores and repeated root operations reduce shared-lock acquisitions;
barrier records cannot disappear at termination.

### Milestone 3 Predict pressure and bound allocation assistance

Primary files: [collector.rs](src/collector.rs), [config.rs](src/config.rs),
[gc.rs](src/gc.rs), [value.rs](../ristretto_classloader/src/value.rs), and VM allocation
and cooperative scheduling boundaries.

- [x] Coalesce pending young requests and request periodic full cycles when the
  configured young-cycle count completes. This removes scheduling locks from
  repeated requests and prevents a quiet allocator from missing an interval.
  Predictive pressure estimates and async recovery are implemented below.
- [x] Estimate allocation rate, survival, time to usable reclamation, and
  fragmentation. Start cycles using headroom approximately equal to allocation
  rate times time to reclamation, plus burst and relocation allowances.
  `predictive_collection(true)` enables a sampled, bounded full-cycle policy on
  finite heaps. Sampling defers on model/registry contention; requests coalesce and
  require fresh growth after completion. Bare wasm uses clock-free occupancy and
  growth estimates. The policy remains opt-in pending performance qualification;
  its headroom allowance does not create a physical relocation reserve. Native
  host-backed recovery screens eliminate observed waits, but compact-heap recovery
  and throughput costs remain. Sustained reclamation rate and contention need further
  work; time to the first reclaimed bytes alone does not establish sustainable capacity.
- [x] Adapt worker activation and pacing to measured debt and available CPU, with
  hysteresis to prevent oscillation. `adaptive_workers(true)` samples queued work,
  allocation debt, occupancy, and recovery waiters at most every 2 ms. Activation
  is capped by configured workers and available CPU parallelism; decreases need
  three low-demand samples per worker. Mutators never enter the controller, and
  contended advisory queue samples defer. Excess workers retry after a 1 ms sleep;
  worker zero always progresses. Recovery pressure removes productive-step pacing
  while failed coordinator claims retain bounded backoff. Wasm ignores this option
  and retains deterministic work budgets. A soft deadline now permits the first
  work unit even after expensive setup/preemption. Correctness tests pass;
  performance qualification remains pending and the policy defaults to false.
- [x] Add an allocation retry result and an asynchronous VM recovery path. Perform
  bounded assistance outside managed guards, then suspend/wake only the requesting
  task when necessary. Account for reservations across retries and cancellation.
  `Gc::try_new_once` retains failed input. Indexed headroom notifications can wake
  native allocators before cycle completion; each waiter retains a distinct identity
  while sharing an outstanding request. Atomic logical credits prevent oversubscription,
  and observed/offered headroom survives notification, so a lost reservation race
  does not consume the two-cycle failure budget. Size-indexed observations are
  installed before cycle requests and record capacity at the atomic refund, covering
  competition before notification or the first poll. Two cycles with no such opportunity
  still fail; continual competition does not promise bounded waiting. Direct
  payload waits now also accept bounded, advisory physical-space notifications
  before cycle completion. Lost offers re-register; retry exhaustion counts
  completed cycles, and cooperative hints still advance collection. Generic
  preparation hooks retain completion-based physical waits.
  Cooperative recovery polls execute at most one configured step. Ordinary
  interpreter allocation instructions, reflective arrays, and `Object.clone()`
  use this path. Clone preflight and publication now await recovery outside the
  source guard, preserving owning fields and finalizer registration. Pressure
  regression tests reject inline clone recovery and preserve Java OOM behavior;
  guarded copying remains synchronous. UTF-8/UTF-16 string construction now scans
  and initializes in bounded batches, constructs directly in the selected heap,
  and awaits payload/field/publication recovery. String arrays, `Thread::object`,
  primordial-thread fields, module/class-loader objects, and bootstrap lookup and
  argument publication also await recovery. Exhausted runtime-object construction
  now returns Java OOM instead of panicking through the legacy infallible allocator.
  Physical recovery now observes returned ranges and shared backing refunds, so
  competitors consuming reclaimed storage do not exhaust its retry allowance;
  two consecutive cycles without physical progress still fail. Continuous
  competition can delay completion. JIT object, primitive/reference array, sentinel,
  and multidimensional array allocation now use async recovery through the existing
  multi-thread-runtime bridge for synchronous embedding. Experimental resumable
  methods now return allocation requests before awaiting object/array construction,
  publication and `new`-triggered class initialization. Multidimensional construction
  shares the interpreter's builder, charges/roots parents before children, and yields
  after 64 nodes across recursion levels as well as during large backing initialization.
  Other helpers and synchronous embedding still need separate progress qualification.
  Native object/array publication, reflection, module bootstrap, networking, file
  metadata, preferences, and JPEG buffer helpers now await recovery. Native object
  fields use direct async initialization; map buckets use managed storage. Native
  mutexes/managed guards end before allocation awaits. Existing host-backed Java
  payloads now stage in 64 KiB batches with source ownership retained until commit.
  Standard Rust vector arguments also initialize directly and await publication;
  scalar/custom embedding conversions retain their synchronous compatibility hook.
  Arbitrary custom preparation callbacks and host-side MUTF-8 decoding remain
  synchronous. Platform compilation checks and workload qualification remain distinct.
- [x] Preserve explicit synchronous `collect_full()` and document synchronous
  embedding behavior. Handle unsatisfiable requests with `OutOfMemoryError` after
  appropriate recovery; avoid both premature failure and endless retry loops.

Exit gate: normal allocations within the measured operating range no longer run
full collection inline. Burst, high-survival, fragmentation, and exhausted-live-set
tests demonstrate progress or correct failure. Waiting tasks do not spin or prevent
collector/reference-handler execution on the same executor.

### Milestone 4 Distribute collector work

Primary files: [collector.rs](src/collector.rs), [metadata.rs](src/metadata.rs),
[index.rs](src/index.rs), and [region.rs](src/region.rs).

- [x] Add selectable persistent worker tracing queues with bounded stealing and
  per-generation queued/in-flight accounting. `work_stealing(true)` gives native
  workers up to sixteen queues plus an injector per generation. Local overflow is
  lossless; one bounded claim probes each queue at most once. Continuations retain
  their scan state after theft or worker departure. Both marking policies retain
  their dequeue/reservation handshake and conservative generation interlock.
  Wasm keeps a single cooperative queue. Queue capacity is observable and empty
  storage is released on stop. Repeated recovery measurements show compact
  throughput changes of −1.0% to +0.4%, with mixed tails; host throughput falls
  1.9% with four workers. The option remains disabled pending qualification.
- [x] Distribute relocation work through selectable persistent queues.
  `independent_relocation(true)` lets workers claim copy continuations without
  the phase coordinator. Enumeration prefetches a bounded window; each claim
  probes queues once and reserves in-flight work before releasing the claim
  reader. Completion excludes new claims and checks queues plus both generations’
  reservations. Callbacks hold storage leases and publish continuations before
  retiring their reservation. Tests cover worker departure, overflow, stealing,
  coordinator independence, both policies, and actual callback overlap.
- [ ] Qualify combined worker scaling with tracing and relocation distribution.
  Both policies remain opt-in; no milestone performance exit gate has been met.
- [x] Add selectable object/identity index sharding, preserving identity checks and
  traversal safety during concurrent insertion/removal. `metadata_shards(1..=16)`
  rounds up to a power of two, with one shard still the default. Full cross-snapshot,
  sweep, and compaction scans charge one entry or exhausted shard per work unit;
  publication barriers cover insertion behind their cursors. Tests cover sparse
  scans, late publication, identity expiration/reuse, and exactly-once destruction.
  Repeated recovery runs show 0.9–2.5% higher compact throughput but 4.4–22.0%
  worse worker-backed operation p99.99. Host throughput falls 0.9–3.2%. The
  combined target is unqualified, so sharding remains opt-in. Combined root-slot
  recovery exposed page-allocation churn because roots shared the metadata
  bucket function. Independent root-address mixing now prevents that correlation;
  the cache bound remains one empty page per root shard. Paired recovery runs
  create sixteen pages instead of 4,207–72,878, with throughput nearly flat.
  Broader throughput and
  latency qualification remains required before changing defaults.
- [x] Reduce coordinator scope to phase transitions and shard remaining allocation
  registries or use stable indexed slabs.
  Opt-in independent marking separates per-generation mark queues from
  cycle state and performs tracing-only steps without the coordinator. A short
  claim read guard covers dequeue through in-flight reservation; phase closure
  takes its writer under the publication gate. Statistics merge before reservations
  retire. `independent_phases(true)` now processes bounded root/reference/sweep
  units under per-domain cursor locks and publication admission. Clearing and
  deferred destruction release the global coordinator; atomic phase boundaries
  remain coordinated and cycle claims never wait while holding it. Relocation
  has a separate independent-claim option. Object/identity, young
  and remembered indexes now have selectable sharding. Generation scans retain
  separate bounded cursors; promotion and pruning preserve object-shard ordering.
  Forced tests cover unrelated-shard barriers, bounded sparse scans and publication
  after young/full sweep cursors. `region_shards(1..=16)` now distributes payload
  and descriptor registries, probes existing space before new backing, preserves
  one physical cap, and ranks evacuation candidates across shards under one
  four-source quota. Forced contention, cross-shard copy, escaped-range lifetime
  and concurrent cap tests pass. Both options remain disabled pending performance
  qualification. Earlier
  independent-marking measurements from two repeated recovery
  runs per policy and worker count on the same two CPUs reduce compact operation
  p99.99 means 2.0–4.4%, with throughput between −1.9% and +0.6%; host results are
  mixed. Coordinator misses remain close to the control. The option stays disabled
  by default; short combined-feature screens still miss mixed-workload latency gates.
- [x] Aggregate object/reference statistics within each step, merging at step exit
  and before cycle completion; avoid a shared statistics mutex per object.
- [x] Extend termination accounting to stolen work and unpublished barrier batches.
  Closure checks every persistent tracing queue and reservations in
  both generations; forced tests cover stolen work in the dequeue/reservation gap,
  abandoned continuations, overflow, busy/time-limited batches, and generation
  overlap. Unpublished context records now remain charged through publication;
  cancelled contexts transfer nonempty buffers to collector ownership. A forced
  pop/publication gap test and reduced Loom models exercise these handoffs.

Exit gate: forced-interleaving tests cannot terminate a cycle with outstanding
work. Compare 1, 2, and 4 workers under fixed total CPU quotas; report useful work,
application throughput, and tail latency. Additional workers must not be enabled
by default solely because collector wall time decreases.

### Milestone 5 Replace the global publication gate

Primary files: [collector.rs](src/collector.rs), [gc.rs](src/gc.rs),
[weak.rs](src/weak.rs), [metadata.rs](src/metadata.rs), and [identity.rs](src/identity.rs).

- [x] Specify object publication/retirement states, atomic transition points, and
  memory ordering for root acquisition, weak upgrade/clearing, allocation, and
  reclamation. PUBLICATION.md now specifies both policies, frozen reference
  boundaries, expected-identity validation, and stage-5 logical retirement.
- [x] Model phase transitions and distributed termination, including context
  attach/detach and buffers published while a collector believes work is drained.
  Define per-context acknowledgements without simultaneously parking all contexts.
  Eighteen reduced models compile the runtime clock/mark word with Loom atomics,
  covering count/stamp races, completed-publisher ABA, saturation, another publisher
  passing a stalled admission, white upgrade versus retirement, independent domain
  flags, two-domain barriers, final retention, conditional owner publication,
  ungated buffered drain, two-generation final retention, promotion, conflicting
  scans, late context attachment and detached-buffer ownership. Record reservations
  acknowledge publication without asking suspended tasks to park. These are reduced
  protocol models, supplemented by forced full-collector tests, not an exhaustive
  model of user callbacks or all phase combinations. Qualification remains pending.
- [x] Add deferred physical reclamation with per-object storage leases.
  Callback admission and retirement share an atomic state; existing leases delay
  clearing/destruction, with logical bytes still charged. A last release wakes
  bounded reclamation without blocking unrelated cycles. Weak decisions remain
  logically final under the selected publication policy. Production-state Loom models and
  forced stalled-borrower/teardown tests cover the storage protocol; this does
  not replace graph publication or generation interlocks.
- [x] Define quiescence for suspended futures and native calls. Persistent roots
  remain visible; active borrowed payloads retain only the storage they require.
  Track delayed reclamation so a stalled participant cannot silently retain an
  unbounded retired heap. Construction hooks transfer released root
  registrations to a synchronous stack scope instead of holding publication admission
  for strong edges. Conditional-reference owner assignment temporarily roots eligible
  referents, with pending per-slot decisions through registration. Both publication
  policies let unrelated collection proceed while a construction hook is stalled.
  Forced tests complete collection during a stalled hook for all root policies;
  nested/reentrant construction and unwind preserve ownership. Rooted graphs and
  deferred storage remain charged. Suspended futures retain roots without admission;
  stalled synchronous admissions can still delay closure. Qualification remains pending.
- [x] Migrate allocation/root publication, weak decisions, remembered-set pruning,
  marking boundaries, and sweep retirement individually. Remove the global gate
  only after every dependent invariant has a replacement. Remembered-set
  pruning now holds its object shard across version validation and removal,
  replacing that writer dependency. A forced-reader regression and reduced Loom
  mutation/pruning model cover the ordering. Ordinary and explicit context drains
  now rely on pending reservations instead of publication admission; a forced popped
  record excludes actual phase closure while the writer remains obtainable. The
  updated reduced model covers this handoff. `concurrent_publication(true)` now
  migrates metadata/roots to counted admission, phases to atomic publication,
  reference decisions to slot locks and sweep to object shards. A forced test
  holds the old writer locked through complete allocation/collection/teardown.
  Numeric reconstruction revalidates expected identity and logical eligibility.
  Both policies pass construction, context, generation and compaction fixtures;
  the counted path passes the 48,000-allocation stress and supported-JDK reference
  matrix. The counted stress also passes ThreadSanitizer; final focused TSan,
  Miri, WASI and eighteen reduced admission/retention models pass. The repeated
  blocking audit finds production legacy-lock accesses only in the two helpers
  that bypass it for counted publication. Performance/endurance qualification
  remains pending; the gate stays enabled by default.

Exit gate: model checking and targeted race tests cover publication versus retirement,
weak upgrade versus clearing, address reuse, and late barriers. An audit finds no
remaining collector dependency on the heap-wide publication write lock. Prefer
no replacement global rendezvous; any deliberately introduced STW phase must satisfy
the declared pause budget and a verified coordination protocol. A deliberately
stalled participant cannot cause unbounded suspension of unrelated application
contexts; retained memory is accounted for.

### Milestone 6 Separate old collection from young progress

Primary files: [collector.rs](src/collector.rs), [metadata.rs](src/metadata.rs),
[weak.rs](src/weak.rs), and generational/reference integration tests.

Prerequisite implemented: tracing queues retain exact allocation metadata through
claims and continuations, acquire leases only for callbacks, and validate identity
when pruning remembered entries. Stale queued tasks cannot borrow replacement
storage at a reused address. This prepares independent retirement; it does not
yet change the generation algorithm or its interlocks.

Conditional slots also validate owner identity through construction and queue
delivery. Cross-generation marking now preserves final/phantom strength and
retraces earlier children when genuinely stronger reachability arrives before a
boundary. Forced schedules cover final retention into an overlapping full cycle
and resurrection before closure. These corrections retain the current interlocks.

- [x] Define selectable old-only tracing with a young-to-old handoff, publication,
  promotion and external roots. `old_collection_interval(N)` (default zero) enables
  conservative old passes between precise full intervals; `collect_old()` requests
  one directly. A bounded initial-young handoff feeds old-only tracing queues.
  Newborn young objects remain collectible, while actual traced graphs and the
  initial cohort remain charged to the heap. Promotion marks old liveness before
  publishing age and queues old work afterward under the same admission. Full
  requests take priority and old passes cannot satisfy full completion tickets.
  Correctness/qualification work is still in progress; no milestone exit gate is
  implied by this implementation check.
- [x] Allow young reclamation to proceed during old tracing, sweeping, and compaction
  where safe. Replace whole terminal-phase serialization with explicit interlocks
  for conflicting regions or reference decisions.
  `overlapping_generations(true)` allows young cycles during full sweep
  and compaction. Post-trace allocations stay young and are excluded from full
  relocation; existing full-snapshot graphs protect callback access. Copy queues
  yield coordinator turns, and only full completion ends its evacuation selection.
  Forced tests cover repeated young reclamation with held sweeping/copying, callback
  graph preservation, logically retired Reference owners, and deferred promotion.
  The subsequent old-only path also permits young progress during unrelated old
  tracing. Its per-object scan locks protect conflicting remembered graphs during
  tracing and copying. Precise full tracing drains the current young snapshot and
  defers new young starts through reference closure; this prevents conservative
  roots from repeatedly surviving full recovery and causing premature OOM.
  Conditional old referents remain conservative roots until precise full processing.
  Qualification of this narrower algorithm is pending. The earlier full sweep/
  compaction path and the integrated old-only path pass native/JDK, Miri, TSan
  and portable checks, including the 48,000-allocation sanitizer stress. Paired short screens
  improve compact recovery throughput 2–10% and worker-backed p99.99 from
  232–259 to 44–63 microseconds; mixed latency gates remain unmet.
- [ ] Qualify cross-generation soft/weak/final/phantom ordering and finalizer
  resurrection for the new path. Old-only passes conservatively retain conditional
  old referents and make no reference decisions; the precise full algorithm handles
  their ordering and cross-generation cycles. The original full-heap policy remains
  selectable with old-only scheduling disabled. Explicit full calls and allocation
  recovery continue to wait for precise full tickets. Forced schedules, reduced
  two-generation models and the supported-JDK matrix supplement the protocol audit;
  integrated sanitizer and portable checks pass. Endurance and performance
  qualification remain pending.

Exit gate: deterministic tests complete multiple young cycles while old work is
held in each relevant phase. Promotion and reference-processing races retain every
reachable object. High-survival workloads reduce repeated young-heap tracing and
allocation waits. C4's independent generations and interlocks are a design reference,
not a claim of equivalent behavior: [C4 paper](https://lag.net/papers/content/C4-garbage-collector.pdf).

### Milestone 7 Improve relocation efficiency and progress

Primary files: [buffer.rs](src/buffer.rs), [region.rs](src/region.rs),
[cell.rs](src/cell.rs), and Java field/array access paths.

- [x] Select regions using reclaimable bytes versus copy cost, observed pinning,
  and mutation frequency. `cost_aware_relocation(true)` selects sparse full-cycle
  sources in bounded units (at most four per full cycle), defers hot and recently
  pinned regions, and prevents
  destination/source oscillation. Young cycles retain legacy packing. Existing
  retry ceilings bound per-cycle copying; a later selection can retry cooled
  regions. Cold/hot/pin, epoch rollover, cross-cycle partial-copy, alignment,
  exhausted-destination and exactly-once destruction fixtures pass. Performance
  remains unqualified and the option defaults to false.
- [x] Track dirty chunks so writes invalidate only affected copied data. Keep
  exclusive publication and the existing source-valid-on-failure invariant.
  A fallibly allocated bitmap tracks 1 KiB chunks (or oversized elements). Scalar
  and range writes dirty affected chunks; unrestricted mutable slices dirty all
  chunks. Copying honors the configured byte budget across multiple chunks.
- [x] Evaluate a bounded relocation reserve within the physical-memory cap.
  Implemented: `relocation_reserve_bytes` protects new destination backing inside
  the shared payload/descriptor cap and is effective only with the cost policy.
  Source backing is released as evacuation finishes; one reserve can support
  several sources. Actual deallocation, including retained zero-length cache tails,
  determines the refund. Small-heap and insufficient-headroom tests preserve
  accounting and source ownership. Paired screens with a 1 MiB reserve and zero
  reserve are recorded in VALIDATION.md. Recovery tails regress with both; the
  reserve remains zero by default and performance qualification is still required.
- [ ] Measure pinned duration, destination failures, bytes copied per byte reclaimed,
  recopy volume, and fragmentation after sustained mutation. Implemented counters
  include resolved copy-lock deferral (not exact guard duration), selection feedback,
  retry limits, failed cancellation, gross source release, and shared current/peak
  backing including reserve usage. Short mixed mutation/pinning screens record
  these values. Actual managed guard-hold timing and active-guard counts are implemented,
  including mapped ownership and unwinding. Paired short screens record the
  samples; all diagnostics together cost 5.1–7.2% in the on-policy recovery
  screen. Long endurance measurements remain pending. Observed copy deferral
  remains a separate metric. Early physical offers reduce native recovery
  p99.99 from about 2.9 ms to 150–235 µs with the cost policy enabled, but the
  throughput and mixed-workload gates remain unmet.

Exit gate: cold payloads still compact while hot or pinned payloads are present,
unchanged chunks are not repeatedly copied, and low-headroom fixtures either make
documented progress or fail allocation correctly. Preserve content, identity,
alignment, and exactly-once destruction tests.

An additional access-barrier redesign is conditional on measured payload-lock stalls
remaining a material limit. Its design must cover scalar/reference accesses,
volatile/atomic operations, bulk copies, native slices, and any eligible JIT helpers.
Segmented arrays require migration of contiguous-slice APIs or explicit pin/copy
adapters. Concurrent copying needs a proven write-preservation protocol; an atomic
payload-pointer swap alone is insufficient. Retain guarded access for arbitrary
Rust values. Expand JIT eligibility only through J1–J3's compiled-frame, semantic
and VM integration gates. Qualification of the guarded design does not establish
that GC-induced payload blocking has been removed.

### Milestone 8 Qualify and document the completed design

- [ ] Complete J5's execution-coverage and whole-JVM comparison gates before native
  external/endurance qualification. Collector mechanism tests and a JIT-enabled
  runtime label cannot substitute for compiled application execution.
- [ ] Run the matrix below against the frozen baseline and each integrated candidate.
  Complete repeated one-hour runs and a separate 24-hour run on the qualification
  host. Preserve raw distributions, source snapshots, CPU, and memory measurements.
  `validation/qualify.py --stage all` runs/resumes the predeclared recovery,
  overhead, capacity and long-run stages, preserving failed attempts and checking
  source, configuration, caps and repeated comparisons. The integrated sequence
  is queued/running; its manifests in `validation/qualification-old-only-2026-11-07`
  record progress. Tooling and queued runs do not establish a completed gate.
- [x] Audit every remaining blocking operation on allocation, rooting, barriers,
  weak upgrades, and payload access. Distinguish application locks, bounded
  assistance, collector coordination, and exhaustion recovery in the final contract.
  The integrated audit in [PUBLICATION.md](PUBLICATION.md) covers counted
  publication, independent phases, sharded regions and old-only passes. Individual
  payload/registry locks, stalled admissions, conflicting scans and synchronous
  embedding recovery remain explicit limitations.
- [x] Update `ARCHITECTURE.md`, `VALIDATION.md`, configuration documentation, and
  compatibility notes with implemented behavior and measured limits.
  Documentation covers the integrated old-only path, conservative reference/full
  fallback, counted publication audit, primitive-buffer contract and public API
  additions. Pending performance/endurance gates are explicitly separated from
  passing correctness checks; later measurements must update their evidence.
- [ ] Change defaults only for algorithms that pass their correctness and performance
  gates. Keep compaction opt-in until its existing release requirements are met.

Exit gate: the combined success criteria pass on the declared native workload and
resource budgets; wasm meets its separate cooperative contract. Record remaining
limitations explicitly. Benchmark results support latency claims; protocol review
and concurrency verification establish safe coordination. Support a zero-STW claim
with a protocol audit and complete telemetry; otherwise report measured bounded
STW pauses against the declared budget.

## Verification matrix

| Dimension | Required coverage |
| --- | --- |
| Execution | Native with 1, 2, and 4 workers; multiple mutators; cooperative native driver; bare wasm and WASI |
| JIT semantics | Interpreter/compiled differential tests for control flow, reference joins, calls, exceptions, monitors, class initialization and Java memory ordering; native backends supported by the release |
| Compiled roots | Live and dead references across backedges, helper calls, nested mixed frames, suspension/cancellation and unwinding; collection/relocation under pressure; bounded retained roots during long invocations |
| Compilation coverage | Required hot-method manifest, measured compiled entries/backedges, fallback reasons, compilation readiness, code-cache use and profiler/recorder overhead |
| Storage | Host-backed payloads and compactable regions; small and large arrays; mixed alignments |
| Load | Low allocation, steady load, bursts, saturation, high survival, heavy mutation, fragmentation, and exhausted live set |
| Roots and lifetimes | Suspended/migrating/cancelled tasks, native guards, escaped handles, shutdown, root-slot and descriptor reuse |
| References | Soft/weak/final/phantom processing, resurrection, queue delivery, concurrent upgrades, and supported JDK libraries |
| Performance | Completed throughput, arrival-to-completion p50/p99/p99.9/p99.99/max, STW/rendezvous count and distributions, total stopped-time fraction, wait distributions, CPU, RSS, committed/live/retired bytes |

Run focused correctness tests for each change and the affected crate suites before
integration. Use small model-checking harnesses for new atomic protocols; use Miri
and thread sanitizers where supported for the corresponding unsafe and concurrent
paths. They supplement the protocol argument and end-to-end stress tests.
JIT changes additionally require real generated-code VM tests with compilation
assertions, interpreted/compiled differential results and forced collection at
frame boundaries; tools that cannot execute generated code cover only their
supported runtime/protocol components.

Existing integration entry points include:

```sh
cargo test -p ristretto_gc
cargo test -p ristretto_classloader --lib --tests
cargo test -p ristretto_jit
cargo test -p ristretto_vm --lib --test gc --test gc_references
cargo clippy --workspace --tests --examples -- -D warnings
cargo fmt --all -- --check
```

Use the platform setup and endurance commands in [VALIDATION.md](VALIDATION.md).
Extend [.github/workflows/gc-endurance.yml](../.github/workflows/gc-endurance.yml)
to preserve the added measurements. Shared CI enforces correctness and records
timing; controlled qualification hosts enforce performance thresholds.

## Delivery and compatibility

Start with milestone 0, followed by direct region initialization from milestone 1.
Introduce local allocation caches, buffered barriers, and new scheduling policies
in separate changes so their costs and benefits remain attributable.

Keep public synchronous embedding APIs and current unsafe lifetime contracts
explicit. Add asynchronous recovery through new VM/internal paths rather than
silently changing a synchronous call's completion semantics. Preserve a selectable
known-correct implementation during algorithm qualification; never mix incompatible
reclamation protocols within a live heap. Record each milestone's completed tests
and measurements before changing its status.

## Whole-JVM execution workstream

Added 2026-11-08. These milestones are planned, not completed. The frozen
[root-guard comparison](../benchmarks/gc/IMPLEMENTATION.md#refreshed-native-wasi-and-browser-results)
records candidate interpreter throughput of 171,056 allocation operations/s and
10,126 root-workload operations/s, versus G1's 25,535,588/15,381,422 and ZGC's
30,703,440/15,128,520. The candidate with JIT enabled records 155,273/9,613.
These short whole-JVM screens establish a large execution gap, not its allocation
among compiler, interpreter, runtime and GC costs or a sustainable capacity result.

`ristretto_jit::Compiler::can_compile` currently admits static primitive methods,
including forward control flow and resumable loops. Calls and reference-bearing
application methods remain excluded. The experimental direct-backend frame handoff replaces
helper roots during invocation and uses backward reference-local liveness, but
has not qualified all frame lifetimes, mixed frames or publication costs.
Consequently, `GcBench.operation` remains interpreted with the JIT enabled. Fixing
these limitations is an explicit dependency of native execution qualification, not
an assumed side effect of reducing collector overhead. Compilation coverage alone
does not establish performance; J4/J5 must measure sustainable capacity and latency.

### J0: Attribute execution cost and record compilation coverage

Primary locations: [jit.rs](../ristretto_vm/src/jit.rs),
[thread.rs](../ristretto_vm/src/thread.rs), [frame.rs](../ristretto_vm/src/frame.rs),
[compiler.rs](../ristretto_jit/src/compiler.rs),
[gc_compare.rs](../ristretto_vm/examples/gc_compare.rs), and `benchmarks/gc`.

- [ ] Freeze interpreter and JIT-enabled baseline/candidate modes and the existing
  HotSpot JIT and `-Xint` controls. Record startup, class loading, warmup and measured
  execution separately, with total process CPU/memory still charged.
- [ ] Record per-method eligibility/rejection reasons, queued/compiled/failed state,
  compilation duration, code bytes, actual compiled entries and loop execution,
  interpreter fallbacks and mixed-frame transitions. Distinguish compilation from
  execution and pending compilation from permanently unsupported methods.
  The opt-in `execution-profile` feature now supplies lifetime method-instance
  records, production rejection reasons, queue/result state, compilation wall time,
  generated code bytes, actual native entries and interpreter/intrinsic entries.
  `benchmarks/gc/coverage.py` preserves diagnostic forks separately from timing.
  Diagnostic native code now counts taken explicit backward edges, and opaque VM
  checkpoints report counter deltas with compiled-code presence at the starting
  snapshot. The example emits a main-invocation interval, still including warmup
  and reporting. Snapshot cuts are not atomic VM-wide measurement boundaries;
  exact phase separation, mixed-frame transitions, recorder qualification and
  full attribution remain pending, so this item and the milestone remain open.
- [ ] Profile CPU and allocations in separate diagnostic forks, mapping generated
  code to methods. Attribute interpreter dispatch, Java frame/value handling,
  resolution/dispatch, helpers and executor bridges, compilation, rooting/barriers,
  payload access and collection. Report nested timings without double-counting;
  measure recorder overhead before retaining counters in timing runs.
- [ ] Freeze a required hot-method manifest for allocation, roots and mixed workloads,
  including `GcBench.operation`, its measurement loop, `GcBench$Node.<init>` and
  hot dependencies/observer code. Record which execute as compiled code, inlined
  code or declared native helpers. Add allocation-free arithmetic, loop and call
  controls, plus held-out application workloads; preserve existing escaping
  allocations, checksums, arrival schedules and retained-content audits.

Exit gate: each workload has a reproducible cost breakdown and coverage report,
with a ranked bottleneck backlog and measurable CPU/operation and bytes/operation
targets. A JIT flag, compilation log or count of compiled cold methods is insufficient.
The initial coverage report may fail; retain it as the baseline.

### J1: Make compiled frames participate safely in GC

Primary locations: `ristretto_jit/src/{compiler,local_variables,operand_stack}.rs`,
`ristretto_vm/src/{jit,jit_runtime_helpers,thread,frame}.rs`, and
`ristretto_gc/src/{context,roots,publication_state}.rs`.

Partial implementation (2026-11-08): direct backend reference methods publish
operand references and live reference locals at every bytecode boundary.
Replacement owners are prepared before old owners are released; helper results
are rooted immediately. Four real compiled allocation-loop cases (local/operand
reference, gated/counted publication) each allocate 62.5 MiB within a 32 MiB heap
before returning. Failed transfers preserve prior roots, and held context locks
do not prevent collection of discarded results. The production gate is unchanged.
Backward local liveness now follows branches, switches, backedges and conservative
exception edges; primitive/wide stores kill the prior reference kind on normal
paths. A reproduced dead-local retention failure now completes two approximately
20 MiB allocations in a 32 MiB heap. Handler-only locals survive the allocating
loop. Small repeated/subset root snapshots now reuse their map storage and owners;
new identities and larger frames preserve transactional replacement. Isolated
helper screens show lower stable-set cost but a remaining gated changing-root
regression. Boundary reduction, complete call/argument/suspension lifetimes,
relocation and mixed-frame qualification, whole-invocation performance, and the
collector's global progress gates remain open.

Return/exception follow-up: completed contexts move existing result owners into
VM values; pending exceptions reuse frame owners and transfer them into handler
results or uncaught errors. Real compiled tests verify registration counts,
post-context survival and reclamation. The four allocation-pressure fixtures now
cover both publication policies with zero and one actual native worker. A zero
configuration alone is insufficient: VM startup selects one worker, so workerless
tests stop/join it and check `registered_workers()` plus cooperative-assist counters.
These checks do not qualify mixed calls, suspension or relocation.
Small changing root sets now prepare only missing owners and retain their map;
reservation precedes deletion and cleanup runs unlocked. Tests cover rollback,
discarded owners under a held frame mutex, and transitions across the inline limit.
The large fallback shares one context lock across existing-owner copies, releasing
it for missing-identity validation and before cleanup. A fixed-binary, seven-pair
workerless confirmation measures 9–12% lower changing-one-root cost and about 30%
lower stable-64-root cost; other intervals include no change. This qualifies only
the measured helper costs. Whole-invocation, retained-memory and contention gates
remain open, alongside the frame-lifetime and global-progress requirements above.

- [ ] Define and implement a compiled-frame root ABI using execution-context-owned
  slots or an equivalent enumerable representation. Specify live locals, operand
  stack values, register spills, arguments, return values and pending exceptions,
  with publication/deletion ordering at every transfer between representations.
- [ ] Publish roots before an allocation, helper or nested call can collect; retain
  them across mixed compiled/interpreted/native frames, reentrant callbacks,
  executor suspension/migration, cancellation and unwinding. Release dead slots
  promptly. Replace invocation-long transient-root accumulation where liveness
  permits, and test a long-running allocating loop without waiting for its return.
- [ ] Preserve stable identities, guarded payload access and every required barrier
  during relocation. Document permitted raw-pointer lifetimes; compiled accesses
  must not bypass the collector's ownership or write-preservation protocol.
- [ ] Make root observation and handoff compatible with the native pause contract,
  preferring no rendezvous. Define bounded local polls/handshakes where necessary;
  a paused compiled frame may retain its live graph but must not prevent unrelated
  reclamation.
  A short STW phase still requires complete frame-root correctness and verified
  coordination within the pause budget; disabling collection cannot satisfy it.
- [ ] Force young/full collection, relocation and allocation exhaustion across
  publication boundaries, call returns and exceptions in real compiled VM code.
  Test root release and retained-memory bounds, supported reference semantics and
  teardown. Use reduced models/Miri/sanitizers where supported; these do not replace
  generated-code integration tests.

Exit gate: live references survive and dead references become reclaimable during
compiled execution, including a stalled frame and a long invocation. Frame and
boundary correctness must pass before reference-bearing production eligibility
expands; final global-progress qualification also requires collector stages D/E.

### J2: Compile control flow and reference operations correctly

Primary locations: `ristretto_jit/src/control_flow_graph/`,
`ristretto_jit/src/instruction/`, `local_variables.rs`, and `operand_stack.rs`.

Partial implementation (2026-11-08): Java reference/category metadata survives
operand simulation, block parameters and helper continuations. Reachability and
incoming stack checks use a worklist; handler stacks are reset; switches preserve
remaining operands and negative keys. Generated code agrees with the live
interpreter on the covered fixtures, with independent HotSpot verification.
Legacy return-address lowering is rejected. The experimental frame roots and
local-liveness work now cover the allocation-pressure cases described in J1. Reference-loop polls,
mixed calls, complete frame lifetimes and broader collection-pressure qualification
remain outstanding, so these checks do not close J1/J2 or widen the production gate.
Local lowering now assigns distinct SSA variables to each Java slot/kind pair,
including reference versus long, and supports legal slot/type reuse. This removes
the global per-slot type conflict; the separate backward analysis identifies live
reference locals.

Forward-only primitive follow-up: the production gate now admits static primitive
methods containing integer conditions, `goto`/`goto_w` and switches only when
every destination, including the default, advances the bytecode offset. These
methods cannot revisit a bytecode or invoke Java code. They use the rootless
primitive ABI; reference operations, calls, handlers and self/backward edges remain
excluded. Normal VM dispatch compiles and executes 25 fixture methods, with no
test cache injection, and preserves interpreted fallback for a finite loop.
The 388 results cover signed extrema, NaNs/zeroes, duplicate/empty switch cases,
wide operands and reused locals, and agree with HotSpot on the exact exported
classes. This finite instruction bound is not a wall-time or STW bound. That step
left native loop cooperation and reference-bearing production methods open.

Primitive-loop follow-up: static primitive loops now use an owning continuation.
Taken self/backward edges consume a positive local budget; exhaustion saves typed
locals and remaining operands, then returns the native frame. The normal VM
dispatcher yields asynchronously and restores the exact destination on resumption.
Arguments are validated before entry, code remains owned, and native stack capacity
is checked on every slice. A conservative method-size budget bounds bytecode visits;
it does not bound wall time. Tests cover BCI-zero resumption, nested loops, switches,
wide/float values, slot reuse and independent/migrated activations. A bounded VM
subprocess proves cancellation, monitor/frame cleanup, argument lifetime and
unrelated reclamation under both publication policies, zero/one worker and direct/
interpreted callers. A one-worker executor remains responsive to an infinite
compiled loop and aborts it. Native resumes have a separate diagnostic counter.
Reference/call-bearing continuations, helper/native cancellation, polling and
state-transfer overhead, benchmark hot-path coverage and J2/J3 qualification
remain open. No collector-wide rendezvous or policy default was added.

Reference-loop continuation follow-up (2026-11-09): experimental backend methods
without direct invokes, explicit monitors or exception tables can now suspend
with reference locals and operands. One owning runtime context spans the slices;
the borrowed pointer is refreshed on native entry and cleared on return. The
destination's live roots are published before yielding, so consumed reference
conditions do not stay rooted throughout suspension. Live-VM checks cover
collection between slices, migration, return and exception ownership, and
cancellation through direct/interpreted callers under both publication policies
with zero/one worker. This does not expand production reference eligibility.
Helpers still execute synchronously; complete call/handler continuations,
mid-helper cancellation, relocation, measured overhead and hot-path qualification
remain required. No J1–J3 checkbox is closed by these partial lifetime checks.

Reference-constant follow-up (2026-11-09): experimental `ldc`/`ldc_w` string and
class constants return owning, argument-free requests before resolution, interning,
mirror construction or allocation recovery. Interpreter and compiled execution
share the resolver; class literals do not initialize the referenced target. Native
loader `ClassNotFound` failures now become `NoClassDefFoundError` in both paths.
The eight-policy/storage/worker configurations cover 64 forced cold-string/class
loader waits, migration, cancellation, residual roots and unrelated full collection.
Exact exported fixtures pass independent HotSpot checks. Numeric constants remain
inline; method-handle/type/dynamic constants remain unsupported. This does not
expand production eligibility, qualify request overhead or complete J1–J5/D/E.
Java-loader exception translation/access checks and general resolution semantics
still require broader qualification before production reference eligibility.

- [ ] Complete branches, loop backedges, switches and stack/local merges, including
  reference liveness and category-two primitive values. Preserve Java arithmetic,
  evaluation order and exception edges across merges and nested loops.
- [ ] Integrate object/primitive/reference-array allocation, field/static access,
  casts, array loads/stores and bounds/null/type checks with J1's roots and the
  existing allocation, class-initialization, barrier and recovery contracts.
- [ ] Add bounded execution polls where runtime cooperation requires them, preferring
  local cooperation. Any collector-wide rendezvous must meet the pause contract.
  Measure polling and coordination overhead independently.
- [ ] Differentially test compiled and interpreted execution against the same Java
  fixtures and HotSpot results, including exceptional paths and collection inside
  loops. Keep unsupported combinations rejected until their integration cases pass.

Exit gate: supported branch/reference methods execute correctly in the live VM
under collection pressure. Direct backend instruction tests alone do not qualify
production methods; benchmark constructors and calls additionally require J3.

### J3: Implement calls, exceptions and production eligibility

Primary locations: `ristretto_jit/src/instruction/{invoke,invokestatic,invokespecial,
invokevirtual,invokeinterface,invokedynamic,exception,monitor}.rs`,
`ristretto_jit/src/compiler.rs`, and `ristretto_vm/src/{jit,jit_runtime_helpers,thread}.rs`.

Partial implementation (2026-11-08): experimental `invokestatic` lowering now
uses an activation-owned buffer of 255 eight-byte argument cells and the real VM
dispatcher. Static class and interface targets, primitive/reference/null returns,
interpreted/compiled callees, a native intrinsic, and caught/uncaught exceptions
have live-frame tests. A suspended caller's operand survives 62.5 MiB of nested
allocation in a 32 MiB heap while reclamation advances, across both publication
policies, zero/one registered worker and interpreted/compiled callees. Tests also
exercise all 255 argument cells, repeated buffer reuse and malformed buffer or
descriptor rejection. The shared resolver no longer initializes the symbolic
target before checking it; static invocation initializes the resolved declaring
class. Independent HotSpot checks execute the exported fixture class files.
The private test cache injection does not expand production eligibility. Other
invoke forms, full access/initialization semantics, recursion/stack exhaustion,
native/reentrant suspension, monitors, relocation and performance qualification
remain open. This is a partial J3 implementation, not its exit gate.

Special-call follow-up: shared lowering now includes the receiver in the 255-cell
buffer and rejects a receiver plus 255 parameter slots. The runtime reuses the
interpreter's special-target selector, checks null after resolution and before
selection, and owns the receiver/arguments across dispatch. Eight live-frame tests
cover constructors, receiver-edge survival during allocation pressure, caught and
uncaught constructor exceptions, superclass selection independent of overrides,
interface defaults, private calls, wide primitive/reference returns and the argument
limit. Interface-method references now also work in special-call stack simulation;
the live default-method fixture reproduced the prior rejection. HotSpot verifies
the same exported classes. Virtual/interface/dynamic dispatch, full access and
initialization behavior, recursion, reentrancy, monitors, relocation and production
eligibility still require work; this does not close J3 or qualify throughput.

The preserved direct-call inventory of the current Java 8 comparison fixture has
four classes and 17 special, 22 static and 60 virtual invocation sites. No direct
interface/dynamic invocation appears. `main` alone has 46 virtual sites; its
measurement and output code still share the method. Transitive JDK calls and
actual hot-call counts remain unqualified, so the dynamic-call inventory item and
compiled coverage gate remain open.

Virtual-call follow-up: `invokevirtual` now uses the experimental receiver-first
ABI and a selector extracted from the interpreter's existing slow path. The normal
interpreter fast path is unchanged. Eight live-frame tests cover seven receiver
classes (exceeding the four-entry cache), inherited overrides, private resolution,
most-specific interface defaults, primitive-array native receivers, linkage/null/
abstract error ordering, primitive/reference return values and exception ownership.
Eight pressure configurations retain both caller operands and callee return values
while reclaiming allocations before return. HotSpot checks the exact exported
classes. This covers the three direct invoke forms found in the fixture inventory;
it does not qualify every transitive dependency or expand production eligibility.
Interface/dynamic calls, signature-polymorphic behavior, full access/initialization
semantics, recursion, monitors, relocation and measured coverage remain open.

Interface-call follow-up: experimental `invokeinterface` lowering shares the owning
call ABI and the interpreter's interface selector. It preserves membership,
selected-method access/abstract checks and the bounded receiver cache. Direct
compiler calls reject a redundant argument count that disagrees with Java slots.
Eight live-frame/compiler tests cover inherited/default/native targets, cache
overflow, linkage/null/selection errors, primitive bits, reference/exception
ownership and allocation pressure across both policies, zero/one worker and
interpreted/compiled callees. HotSpot verifies the exact exported classes. All
four ordinary invoke forms now have experimental lowering; dynamic and signature-
polymorphic behavior, full access/initialization semantics, recursive/reentrant
boundaries, monitors, relocation and production qualification remain open.

Stack-budget follow-up: compiled frames no longer bypass the logical Java stack
limit. A production-path regression reproduced an oversized method entering native
code despite its budget. The dispatcher now charges declared locals, operands and
one control slot, and existing unwind cleanup releases the charge. Bounded mixed
recursion checks catchable overflow, exact call-site backtraces, receiver-independent
argument/throwable lifetime, synchronized-method monitor release and subsequent
thread reuse. Five nested frames retain distinct arrays while a leaf allocates
beyond the heap capacity under both publication policies, zero/one worker and
mixed execution. These checks do not protect the host's Rust/generated stack from
deep recursion; native stack bounds/exhaustion remain required before production
call eligibility can expand. The J3 exit gate remains open.

Native recursion follow-up: an unguarded release subprocess aborted on a 1 MiB
host stack. A compiled-nesting guard now rejects low native capacity, while outer
entry and throwable construction can use temporary stacks. The Linux matrix tests
1/2 MiB stacks, mixed execution, both policies, root/throwable lifetime and real
monitor cleanup. Static synchronized entry now creates missing class mirrors;
the earlier optional-monitor test did not establish monitor cleanup. Full generated
frame-size protection, cold helper bounds, interpreter-only native reentry and
other host targets remain open. This does not satisfy J3 or any performance gate.

Generated-frame follow-up: functions now carry backend frame spans (including
spills) and check their frame plus runtime reserve before entry. Missing/dynamic/
tail-call metadata is rejected. A large-frame subprocess verifies rejection on a
512 KiB segment and execution on 4 MiB; real class-file tests distinguish optimizer
rematerialization from opaque-call spills. The VM sizes outer segments from the
recorded frame. Compiled entry now checks argument value count and ABI kinds before
generated loads, using metadata from the same descriptor sequence that initializes
locals. Instance receivers count once; wide primitives occupy one ABI value each.
Checks are fused into marshalling, and clones share the immutable signature.
Portable stub tests and real generated methods cover missing/excess arguments,
kind mismatches, receiver position and the inline/heap-buffer boundary. The VM
bridge also checks wide-slot padding conversion and failed-call root lifetime.
Reference validity, helper/context lifetime, full helper bounds, native/JNI reentry,
other native hosts and performance qualification remain required; J3 stays open.

Call-continuation follow-up (2026-11-09): experimental methods without handler tables
or explicit monitor opcodes return their native frame before dispatching static,
special, virtual or interface calls. Owning continuation cells retain arguments,
typed locals and residual operands; the existing frame snapshot and runtime context
retain references. The VM awaits resolution/dispatch and resumes after the call
without replay. Synchronous embedding and handler methods retain the helper bridge.
Nested future polling/drop uses temporary native stack capacity; Java slot limits
still bound logical recursion. Cancelled class initialization releases its claim
as a failed class and wakes waiters without replaying partial side effects.
The full native VM suite now covers call/initializer cancellation and recursion;
portable checks and independent bytecode oracles supplement that evidence.
production eligibility, handler/helper continuations, relocation, measured overhead
and hot-path qualification remain open. This does not close J1–J3.

Handler-continuation follow-up (2026-11-09): experimental exception-table methods
now suspend at calls and selected handler entries. Handler edges consume local
fuel because exception-only cycles need not contain a backward branch. A separate
exception destination map preserves ordinary branch behavior; suspension publishes
the selected handler's live locals and throwable operand. Focused generated-code
tests cover ordered typed matching, local/throwable survival, release of obsolete
exceptions, migration and cancellation through interpreted/compiled nested calls.
At that snapshot explicit monitor methods retained synchronous dispatch, and
their monitor opcodes were still no-ops; the follow-up below corrects that gap.
Production handler/call eligibility, mid-helper/native cancellation, relocation,
measured overhead, complete semantics and hot-path qualification remain open.
This is further J1–J3 implementation, not completion of their exit gates.

Monitor-continuation follow-up (2026-11-09): experimental `monitorenter` and
`monitorexit` now invoke real shared VM semantics instead of discarding operands.
Owning requests return the native frame before waiting and restore locals/residual
operands without replay. A logical-thread ledger roots acquired objects and releases
outstanding entries on host cancellation; nested cancellation preserves older
entries. Tests verify ownership while held, reentrancy, contention, Java errors,
catch-all release, migration, dead-root retention/release and unrelated reclamation.
The prior handler fixture's argument-monitor check did not prove acquisition; its
historical evidence is corrected in VALIDATION.md. The native-capacity fixture now
uses the synchronous embedding API deliberately and checks ownership inside its
monitor region. Production eligibility, wait/notify cancellation, native reentry,
complete synchronization semantics, allocation/helper suspension, relocation and
measured overhead/hot-path qualification remain open. No J1–J3 gate is closed.

Allocation-continuation follow-up (2026-11-09): experimental `new`, `newarray`,
`anewarray` and `multianewarray` now return owning requests before resolving,
initializing or allocating. Dimension bits, residual operands and published roots
survive recovery, migration and cancellation. Synchronous embedding shares the
same async implementation through its existing bridge. Native and owning argument
buffers now use each method's actual maximum request arity. Runtime requests consume
local fuel, carried across immediately ready monitor/allocation requests; resetting
fuel at each request previously allowed ready-only loops to monopolize a poll.
The shared multidimensional builder additionally yields after 64 nodes across depths,
including empty children. Focused native checks cover eight policy/storage/worker
configurations, cancelled class initialization without replay and workerless recovery;
portable interpreter checks exercise cancellation and multidimensional cooperation.
These are work/correctness checks, not pause or throughput qualification. Other
helper/native suspension, full semantics/relocation, production coverage, measured
overhead and collector D/E progress remain open; no J1–J3 gate is closed.

Field-continuation follow-up (2026-11-09): experimental field loads/stores now return
owning requests before resolution or static initialization can suspend. Synchronous
and resumable helpers share the interpreter's checked field resolver, including
descriptor/access/kind/final checks, actual declaring-class initialization and typed
storage. Null checks follow resolution; repeated cached field linkage failures keep
their Java error kind. Generated-code tests cover primitive/reference bits, inherited
statics, errors, cancellation without replay, migration and unrelated reclamation in
eight policy/storage/worker configurations. A checked continuation expansion limit
rejects excessive state-transfer code before emission. This is correctness/progress
work; field transfer/guard costs, remaining helper suspension, production eligibility,
full semantics/relocation, collector D/E and external qualification remain open.

Type-check continuation follow-up (2026-11-09): experimental `checkcast` and
`instanceof` return owning requests before symbolic resolution or assignability
can await. Synchronous embedding shares the request implementation; the interpreter
and compiled paths share the assignability helper. The current frame owns the
operand across resolution, migration and cancellation, and ready requests consume
continuation fuel. Forced class-loader waits permit unrelated full collection in
eight publication/storage/worker configurations. Differential fixtures cover null,
primitive/reference arrays, interfaces, target non-initialization, caught cast
errors and wide residual state. The exported finite fixtures agree with HotSpot.
Production reference eligibility, remaining helper suspension, full resolution and
relocation semantics, measured overhead, hot-path coverage and collector D/E remain
open; this step does not close J1–J3 or a qualification gate.

Reference-array store follow-up (2026-11-09): interpreter and experimental compiled
`aastore` now share checked null/bounds/type ordering and resolved component identity.
The former interpreter omitted the type check; the former JIT checked type before
bounds and resolved names through the wrong loader. Recursive array-component
assignability also fixes valid multidimensional/interface-array stores rejected by
rank heuristics. Experimental compiled stores return owning requests before type
resolution or throwable construction; interpreter stores enter the async boundary.
Neither path holds payload guards across awaits. Sixteen native held-loader schedules
cover both publication/storage policies and zero/one worker; portable interpreter
checks verify cancellation and unrelated reclamation. Exact-bytecode finite fixtures
agree with HotSpot. Required checks and request transfers have unmeasured costs;
remaining array/constant/exception helpers, full semantics/relocation, production
coverage, J1–J5 performance and collector D/E remain open.

Array-access continuation follow-up (2026-11-09): experimental primitive/reference
loads, primitive stores and `arraylength` now return owning requests. Raw float/wide
bits and reference results survive state transfer; ready accesses share continuation
fuel. Busy payload acquisition yields, and error construction happens after all
payload guards end. Successful uncontended synchronous embedding avoids the executor
bridge. Interpreter and compiled boolean stores now retain the low bit, correcting
a reproduced narrowing error. The native matrix covers 112 suspended schedules in
eight configurations: 64 error-construction waits and 16 read-lock waits permit full
collection, while 32 held-writer cases prove request cancellation/migration only.
A held writer on the managed `Reference` enum still prevents tracing even for a
primitive-array variant; the initial timeout and bounded failure remain recorded.
This is an explicit D/E gap. Exact exported finite fixtures agree with HotSpot;
production coverage, constants/exception/native suspension, full relocation,
measured overhead and all qualification gates remain open.

Typed exception-matching follow-up (2026-11-09): experimental typed-handler lookup
now returns an argument-free owning request before loader/assignability waits.
Each table entry has its own resume site at the original throwing BCI, preserving
ordered and duplicate catch types without replaying the throwing instruction.
Requests consume shared fuel. The context retains the throwable and live locals;
busy throwable payload access yields without retaining a guard across an await.
Catch-resolution failure propagates past later entries, and native missing catch
classes become Java linkage errors in interpreter and compiled execution.
Eight configurations cover 16 loader waits with full collection, cancellation and
migration, and 16 held-writer request waits with cancellation/completion. The latter
do not permit full tracing past the held writer; stage E remains open. Exact
exported fixtures have an independent HotSpot oracle. Handler eligibility, remaining
synchronous throw/native helpers, broader Java-loader/access semantics, request
cost, D/E and J0–J5/H qualification remain open; no defaults changed.

- [ ] Implement static, special/constructor, virtual and interface calls with correct
  resolution, dispatch, class initialization and argument/return rooting. Support
  recursion and compiled/interpreted/native crossings through a documented ABI.
  Inventory `invokedynamic` in the declared workloads and implement any required
  bootstrap/call-site behavior; unsupported forms remain explicitly rejected.
- [ ] Complete exception-table dispatch, propagation through mixed frames, stack
  traces and pending-throwable ownership. Implement monitor enter/exit and unwind
  cleanup before admitting affected synchronized methods/blocks; preserve volatile
  and atomic memory semantics and required Java exception behavior.
- [ ] Expand `can_compile` incrementally by verified capability. Fallback must occur
  at a safe entry or a tested state-transfer boundary, without replaying side
  effects or losing roots. If speculative optimization is introduced, implement
  guards, invalidation and state reconstruction before enabling it.
  Eligibility and optimizations must apply to general bytecode/capabilities, not
  benchmark class names; the coverage manifest records execution and must not
  bypass production safety checks.
- [ ] Prove the J0 manifest's hot paths execute compiled code in every measured
  process. Include the measurement loop: compilation after entry must either
  transfer its live state safely or use an explicitly reported, identical workload
  structure that enters compiled code after warmup. A method cached after its
  long-running interpreted invocation began is not proof of compiled loop execution.

Exit gate: allocation, roots and mixed workload hot paths, including required calls,
pass VM semantics/GC stress and measured coverage checks. Unexpected interpreter
fallback for a required hot method fails coverage; declared native helper execution
remains visible and charged. These are coverage gates, not a performance victory.

### J4: Reduce measured execution and runtime overhead

Primary locations: `ristretto_jit/src/`, `ristretto_vm/src/{jit,jit_runtime_helpers,
thread,frame,instruction}.rs`, and the profiled `ristretto_classloader`/`ristretto_types`
object, array and value paths.

- [ ] Re-profile after J3. Rank remaining compiler/runtime/collector costs by their
  share of application CPU and allocation; estimate each proposed change's maximum
  whole-JVM benefit before prioritizing it. Do not extrapolate isolated nanosecond
  improvements into closure of the observed throughput gap.
- [ ] Reduce hot call/helper overhead: repeated context/root setup, argument/value
  conversions, dispatch/resolution, locks, temporary system allocations and
  synchronous/async bridging. Evaluate cached resolution, direct calls and inlining
  only with correct class/dispatch dependencies and J1's lifetime guarantees.
- [ ] Optimize generated allocation and field/array paths while preserving escaping
  objects, heap accounting, barriers, recovery and Java checks. Investigate code
  quality, redundant checks and representation costs using profiles and generated
  code inspection; keep each change independently measurable.
- [ ] Improve measured interpreter dispatch, frame/value and fallback costs as a
  separate deliverable, including WASI. Tune compilation scheduling and cache policy
  using startup, time to stable performance, compiler CPU and retained code memory.
  Compiler workers share the same total CPU/memory caps as mutators and GC workers.

Exit gate: each retained change has a paired whole-JVM improvement and correctness
evidence under fixed resources, or a separately declared correctness/progress reason.
Report cold-start and steady-state tradeoffs and interpreter/Wasm regressions. Keep
the remaining bottleneck backlog open until J5's qualification criteria pass.

### J5: Qualify the combined JVM and collector

Primary locations: `benchmarks/gc/{run,report,qualify,capacity}.py`, `gc_compare.rs`,
the workload/coverage manifests, and the provenance and endurance drivers.

- [ ] Extend the runner and qualification evaluator to require per-process hot-path
  coverage, compilation readiness, fallback reasons, runtime mode, code/compilation
  resource use and measured-window GC activity. Missing evidence blocks qualification;
  preserve exploratory and failed runs instead of dropping them.
- [ ] Remove peer-relative throughput/latency pass/fail thresholds from the
  qualification evaluator while retaining comparison ratios and confidence intervals
  as diagnostics. Add the native STW gate requiring every observed pause <1 ms.
  Version the acceptance policy and reports so historical gate failures remain
  attributable to their original criteria. Check missing pause telemetry, rejection
  at exactly 1 ms and larger pauses, and zero-STW evidence before qualification.
  Partial implementation: the v2 checker freezes absolute limits and a reviewed
  audit, retains diagnostic peer ratios, validates bound per-process evidence and
  rejects missing capture or stops >=1 ms, including rendezvous and overlapping
  intervals. The v1 evaluator remains available for historical replay without
  overwriting reports. An opt-in Linux publication-writer recorder now captures
  real request/acquire/release events and failed acquisition attempts in fixed
  atomic storage. Diagnostic transport binds PID/run, binary/source and workload
  bytecode identities, preserves overflow/open events and rejects relabeling it as
  qualifying telemetry. Explicit shared-bytecode Java boundary hooks now bind
  measurement timestamps to the workload's returned elapsed time and conservatively
  bound real in-window legacy VM collections, excluding warmup/reporting. Native
  `System.nanoTime` now uses a monotonic source shared with these hooks; WASI uses
  a process-local `Instant` origin. Fresh baselines are required after this clock
  correction. Complete global-source coverage, measured compiled execution,
  canonical-heap exposure and recorder-overhead qualification remain missing;
  this partial producer does not complete the gate. See the
  [evidence contract](../benchmarks/gc/QUALIFICATION.md).
- [ ] Use paired execution-before/after × collector-baseline/candidate comparisons
  where configurations are compatible, plus explicit interpreter controls. Freeze
  each combination's source and configuration. If an old collector cannot support
  the new frame ABI, record that limitation and use a compatible fixed-policy
  execution comparison; do not claim an isolated GC effect from inseparable changes.
- [ ] Compare the production JIT configuration with both HotSpot default-JIT peers
  on unchanged Java workloads, equal total resources and the same JDK libraries.
  Retain `-Xint` controls for attribution, never as substitutes for deployment peers.
  Validate warmup, compilation stability and GC exposure before capacity/tail trials.
  Freeze workloads and coverage requirements before tuning. Any harness restructuring
  needed to enter compiled loops requires a new common bytecode snapshot and reruns
  of all compared configurations; earlier workload results remain separate evidence.
- [ ] Apply the sub-millisecond STW budget across all primary/held-out workloads,
  with overload recovery, bounded backlog/memory and sustainable-capacity evidence.
  Report throughput/latency confidence intervals and peer comparisons without a
  required peer ratio. Coverage success cannot waive a resource, progress or pause gate.
  Follow passing short and calibrated-capacity gates with stage H's hour/day trials.

Exit gate: the combined JVM meets the external short/capacity gates with compiled
coverage and attributable costs. Stage H and milestone 8 retain the final endurance
and release decisions. No native JIT claim applies to WASI/bare Wasm; shared runtime
changes must preserve their separate correctness and cooperative-performance gates.

Delivery order: start J0 with A, then implement J1–J3 in independently reviewable
changes while collector B–F continues. J4's interpreter/runtime work can start from
J0; compiled-path tuning follows the relevant J1–J3 correctness gates. Integrate J4
with G, complete J5, then H. Keep JIT expansion opt-in until the corresponding
capability passes its correctness gates; change performance defaults only after
the complete qualification matrix passes. All J0–J5 checkboxes remain open until
linked implementation and validation evidence supports completion.

## Next delivery sequence: overhead, progress, and external qualification

The full objective requires eliminating GC-induced global stops, reducing
allocation, root and barrier costs, and beating both G1 and ZGC on sustainable
throughput and scheduled p99.9 and p99.99 within the declared operating envelope. The J0–J5 workstream
supplies the required JIT/runtime work alongside A–H. Maintain native, WASI, and
bare-Wasm correctness. These are acceptance goals, not implemented guarantees on
every program or hardware configuration.

The native full-release contract requires zero GC-induced all-mutator rendezvous
and zero equivalent global stalls. Remove heap-wide exclusive
publication blocking and synchronous full-cycle assistance in ordinary VM allocation.
A stalled task, publisher, root holder, or payload guard must not prevent unrelated
reclaimable objects from being collected. A retained live object still consumes memory; an
exhausted live heap must fail allocation rather than promise impossible progress.
Explicit synchronous collection waits for its caller; any resulting global stop
also counts toward the pause budget. Document every remaining per-object wait
separately from global stops. A heap-wide lock cannot evade pause accounting by
being described as publication or recovery work.

Threadless-Wasm contract: every GC poll has bounded root, barrier, trace, clear,
copy and retirement work, and yields to the host. There is no simultaneous mutator
and GC execution on one thread. Eliminate unbounded full-heap pauses and expose
host-turn latency; do not describe cooperative work as zero execution interruption.
Bare-Wasm algorithms must also work without a clock. Browser/WASI allocator and
memory-growth delays remain measured separately from GC step work.

The stages below build on existing experimental implementations. A checked item in
an earlier milestone establishes implementation, not qualification of this sequence.

| Stage | Work and primary locations | Required evidence / exit gate |
| --- | --- | --- |
| A: freeze and attribute | Freeze current baseline and candidate, binaries, JDK, workload bytecode, CPU affinity, memory policy and seeds. Extend `diagnostics.rs`, `gc_endurance`, and `gc_recovery_latency` to attribute allocation, root, barrier, payload and recovery costs; J0 adds whole-JVM CPU/allocation profiles and compilation coverage. | Repeated isolated measurements; allocation-free recording; measured recorder overhead; baseline variability and execution bottlenecks known before choosing changes/tolerances. |
| B: allocation fast path | In `allocation.rs`, `region.rs`, `gc.rs`, and `metadata.rs`, finish qualifying local descriptor/payload caches and batched credits; remove redundant preparation/lookups. Evaluate compact/slab metadata and descriptor-local lookup against retained memory. | Cache-hit allocation needs no heap registry mutex or system allocator; refill and cancellation preserve budget/ownership invariants; bytes and CPU per escaping allocation improve at equal RSS. |
| C: roots and barriers | In `context.rs`, `roots.rs`, `gc.rs`, and `cell.rs`, make ordinary root slot operations context-local; batch root retirement and snapshot edges. Avoid repeated shared ownership/collector lookup when a protected context is already available. Evaluate duplicate dirty-owner/card coalescing. | Root churn and mutation tests improve independently; no dropped deletion edges on overflow, migration, cancellation or epoch changes; suspended contexts remain enumerable without rendezvous. |
| D: publication progress | Qualify counted admission; investigate stalled admitted operations and phase closure. Design durable, bounded publication records with idempotent helping or a protocol that safely carries unfinished records across closure. Define atomic linearization points before code. | Forced suspension at every admission/publication transition cannot stop unrelated reclamation. Loom models plus real collector schedules; no legacy publication-writer dependency. Do not declare counted admission alone sufficient. |
| E: reclamation and relocation progress | In `storage_state.rs`, `recovery.rs`, `phase_work.rs`, and `relocation.rs`, isolate retired storage by descriptor, rotate busy work, and reserve progress capacity. Partition callback reservations so one blocked owner cannot hold an entire generation. | Unrelated garbage is reclaimed with a publisher, scan, or payload guard deliberately stalled; memory/waiter bounds hold; finite failures return correctly; no starvation under the declared finite contention assumptions. |
| F: payload access protocol | Measure guard costs first. If necessary, design versioned payload access with per-object leases and write-preserving forwarding, including mapped guards, bulk slices, atomics, native access and cancellation. Compare retaining guarded payloads against the alternative. | Linearizability of reads/writes during copy and exactly-once destruction; no stale references; observable reduction in GC-induced access stalls. No lock removal without a complete access protocol. |
| G: policy and integration | Qualify old-only handoff, full reference fallback, predictive scheduling, worker scaling and evacuation reserve together, then integrate J1–J4's verified execution paths. Tune only on training workloads; hold out seeds, heap sizes and graph shapes. | Integrated correctness, coverage and resource accounting are ready for J5's sustainable-capacity and sub-millisecond STW qualification. Choose defaults only after the full matrix passes. |
| H: external and endurance qualification | After J5's coverage and short/capacity gates, run integrated shared Java comparisons and collector-only drivers, fixed-rate trials, overload/recovery, repeated one-hour runs and a separate 24-hour run. | Reproducible report, graphs and raw data; execution coverage, correctness, progress, throughput, latency, STW pauses, memory and teardown gates all pass. |

Dependencies: A precedes all performance decisions; B/C precede fast-path integration;
D/E precede any no-global-blocking claim; F requires D and a separately reviewed
memory-safety protocol; G follows the chosen B–F implementations and integrates the
J1–J4 capabilities. Native external qualification requires J5 before H; H is the
release gate. J0 attribution starts with A, not after collector tuning is exhausted.
Deliver each stage as a separate change with an ablation, not one all-features result.

Stage-D design evidence (2026-11-09): the new helpable-publication model suite
rejects replaying current mutations behind a separate completion bit. Forced
schedules demonstrate stale-epoch overwrite, reused-identity replacement,
mark-before-queue acknowledgement and announcement crossing closure. Seven positive
checks establish reduced generation-tagged atomic-commit and phase-coupling
relations, not a complete protocol. The next runtime work is canonical descriptor
membership with durable traversal visibility and a scalable catalog/phase handshake;
replacing the admission counter alone cannot preserve existing invariants. See the
[design and integration gates](validation/HELPABLE_PUBLICATION.md). No runtime
admission or D/E completion claim changed.

Durable-work prerequisite (2026-11-09): the production fallback tracing FIFO now
supports owned prepared append records and bounded, idempotent helping attempts.
Consumption preserves insertion evidence before detaching links, preventing late
helpers from requeueing consumed work. Private preparation is cancellable and does
not exclude closure by itself. Existing collector calls still keep these records
local and retain admission; canonical membership, discoverable announcement and
phase-coupled acknowledgement remain the next required D work. Queue-level helping
tests do not establish collector progress past a stalled admitted publisher.
The paired collection screen also exposed sensitivity to invocation-name length;
the driver now fixes `argv[0]` on native and WASI and records the actual executable
separately. Native queue-header alignment reduced the candidate's sensitivity in
three tested invocation layouts, without qualifying all layouts. The retained
candidate's native worker results are mixed and WASI collection cost regresses;
all runs and graphs remain recorded. This prerequisite does not close an overhead,
pause, progress or endurance gate.

Current D/E progress: collector queue/claim/publication-writer acquisitions yield
on contention. Metadata scans now rotate among unfinished shards without losing
an entry, and retirement/promotion acquire their membership guards before changing
indexes or age. Forty-eight held-index schedules reclaim unrelated garbage within
an active sweep, including sixteen with counted admission held. Tree/slot/context
root metadata lookup also yields without committing its cursor; slot retry re-reads
removal/reuse. These changes do not satisfy D/E's exit gates: stalled admissions
still exclude closure and callbacks retain graph reservations. Context root scans
now use an immutable published registry and release cache/final-owner work outside
collector guards. Sixteen forced context configurations allow unrelated full
collections while a registry, cache or root-scan cleanup is stalled. The registry
copy cost and cursor-retained context memory remain unqualified. Collector barrier
claims now try buffer/ready locks transactionally, and apply records and release
owners outside collector coordination. Twenty-four forced configurations cover
busy claims, required application and final owner cleanup. Unapplied records still
exclude closure; after acknowledgement, stalled final context cleanup permits
unrelated full cycles. Optional post-drain certification skips busy metadata.
Required mark publication now tries ordinary queues and uses an owning atomic
FIFO on contention; linked producers can be helped before they advance the tail.
Validated zero counts let closure skip a held empty injector. Four forced
publication/phase configurations complete full collection and reclaim unrelated
garbage while that lock remains held; held metadata still preserves pending work.
Five bounded queue/closure models and native/Wasm checks pass. The fixed queue
nodes, fallback allocation and added fast-path counts remain unqualified costs.
A frozen full-graph collection screen passes 336 native/WASI processes: native
cooperative and broad-graph cases improve, but the four-worker gated tree regresses
by about 14% and all WASI intervals include no change. This net-path comparison
leaves the regression and per-path allocation cost unresolved; see the
[initial collection screen](../benchmarks/gc/IMPLEMENTATION.md#repeated-graph-collection-measures-the-queue-change).
Subsequent empty-claim skipping and bounded probes of other worker queues reduce
fallback publication. Three paired screens pass 1,008 processes. The final native
comparison against the pre-FIFO baseline improves cooperative and broad multiworker
cases; tree intervals now include one, which does not prove parity. Seven of eight
WASI intervals also include one. Sixteen forced application cases verify both the
FIFO and available ordinary queue paths preserve/reclaim their graphs. These remain
partial D/E improvements, with allocator/admission/callback limitations intact;
[final queue results](../benchmarks/gc/IMPLEMENTATION.md#avoid-empty-probes-and-reuse-available-worker-queues)
retain all failures and measurements.
Collector drains now prepare required metadata/remembered shards with try-locks
before claiming a matching buffered record. Eight forced schedules keep the
record pending while the same collector reclaims separately retired storage.
Four young-start schedules preserve remembered insertion across an inactive
observation; portable tests cover changed and identical replacement records.
This frees a collector worker from index waits but does not allow closure past
the unfinished record. Its paired barrier/collection screen finds a small required-
publication cost and a remaining 3–5% native broad-control regression. Outlining
the preparation helper removes the clear WASI broad-control regression; it does
not establish native parity. A redundant pending check was measured and removed;
all 896 process results are retained. See the
[barrier preparation results](../benchmarks/gc/IMPLEMENTATION.md#required-barrier-preparation-yields-on-busy-indexes).
The full goal and D/E exit gates remain open.
Mutator insertion now tries its buffer/ready locks transactionally, and overflow
uses prepared drains that leave busy records pending before falling back to direct
publication. Forty forced configurations exercise root deletion across all root
policies, old-to-young notification and contended overflow. Empty buffer/ready locks
can remain held while unrelated full collections finish. Nonempty inaccessible
records still exclude closure; direct fallback retains target-index/allocation waits.
Explicit mutator flush, reference processing and other context cleanup retain waits;
nonempty ordinary queue holders, host allocation, admitted unfinished publishers
and opaque callbacks still prevent the D/E exit gates. A 784-process paired screen
records about 9% higher context/local overflow-operation cost, about 4% lower native
wide-graph collection time and mostly inconclusive WASI intervals. The change is
retained for forced-contention progress; overflow overhead remains open. See
[validation](VALIDATION.md) and the [cost comparison](../benchmarks/gc/IMPLEMENTATION.md#mutator-barrier-insertion-and-overflow-yield-on-contention)
for frozen evidence. No STW, memory-bound or sustained-load gate is qualified.

Allocation registration now prepares the object/identity/young write set before
changing marks or index membership. Index contention releases all partial guards
and admission before waiting; retries use fresh phase state. Thirty-six forced
configurations cover both publication/phase policies and all root stores. Held
identity/young indexes allow three subsequent full cycles, while held object shards
allow unrelated sweep reclamation without completing the blocked scan. Map-node
allocation and a publisher preempted after successful preparation still retain
global admission. This transaction preparation does not complete D/E or implement
helping. A frozen 1,120-process native/WASI screen records 1–3% higher allocation
operation costs in several policies and mixed root/wall results. The change is
retained for contention progress; allocation overhead and sustained behavior remain
open. See [the comparison](../benchmarks/gc/IMPLEMENTATION.md#allocation-registration-releases-admission-before-index-waits).

A generated array-access fixture now also reproduces full-marking dependence on a
held `Gc<RwLock<Reference>>` writer, even when its current variant is a primitive
array. The initial full-collection attempt timed out; a bounded 100,000-step probe
also failed. Owning runtime requests can now yield and cancel while the writer is
held, but collection still needs a graph-retention/progress solution. The final
matrix preserves this distinction in 32 held-writer probes and permits full
collection in 80 loader/read-lock schedules. The enum may later hold references;
classifying all such descriptors as leaves would violate its current contract.
This is required E work, alongside stalled admitted publication in D.

Investigating that graph contract reproduced element and whole-value ownership
failures: moved values could lose their graph after the source was reclaimed.
Mutable element/slice/iterator access now acquires owning roots; managed guards
also invoke `Trace::root` before unrestricted whole-value access or mapping.
Custom trace implementations must pair demotion with ownership restoration.
The 192-case element and 192-case whole-value matrices pass on native/WASI,
including context detach and transitive graphs; unwind/cancellation checks pass.

The first whole-value repair caused 37–39× native and 46–67× WASI overhead for
32-field stores. Narrow typed guard operations now avoid unrelated root acquisition
for replacement stores and acquire only the selected edge for point borrowing.
Interpreter/JIT field stores, shared `aastore` and multidimensional initialization
use them without bypassing Java checks or barriers. A frozen 1,344-process comparison
restores field/point costs close to the original in this fixture; full results retain
mixed controls and slight slice regressions. Unrestricted whole/slice borrowing still
has linear synchronous work. This repairs ownership before graph-progress reasoning;
it does not allow tracing past a held writer, remove admission, qualify bounded bulk
Wasm access, or complete D/E/J/H. See the
[results](../benchmarks/gc/IMPLEMENTATION.md#whole-value-ownership-and-narrow-managed-mutations).

Scoped E follow-up (2026-11-13): `experimental-trace-projections` now permits
full, young and old tracing to complete during eligible typed primitive-array
borrows. The entire parent must remain free of tracing actions; atomic certification
and guard-owned revocation preserve this scope across nested mapping, failure and
unwind. The feature remains off by default. Four production-state Loom models and
forced native/WASI fixtures cover the lifetime rules, while negative variants reject
missing certification and revocation. Whole-value/reference-bearing writers retain
their ordinary scan requirements. The final 1,344-process screen adds 8 native / 4 WASI bytes per managed Reference
cell, with unchanged guard sizes. Native length-32 typed/byte borrows cost 6–9% /
9–19% more; WASI and reference controls are mixed. Default binaries reproduce the
prior runtime byte-for-byte with the same expanded benchmark. Regular Miri and
sanitizers pass; the full Tree Borrows matrix timed out, while an explicit test-only
8-probe overlay retaining all configurations passes. The original 1,000-probe native
and regular Miri checks remain. No D/E or qualification gate is closed. See
[results](../benchmarks/gc/IMPLEMENTATION.md#scoped-primitive-projections-permit-unrelated-collection).

Ownership follow-up (2026-11-14): generic `Gc::clone` now roots borrowed heap
edges, while owning clones and `GcRootGuard::clone_gc` share their registrations.
Guard cloning retains one registration, and mutable guard exposure establishes
ownership before `mem::replace` can move the handle out. Edge registration validates
identity and logical liveness under admission, rejecting stale/reused addresses.
Legacy cycle fixtures now use managed mutation and paired root/unroot hooks.
Frozen regressions reproduce the borrowed-edge, guard-clone and guard-move losses
on the prior runtime. All 20 combined validation checks pass, including native/WASI,
Miri, Tree Borrows and sanitizers. The 1,008-process paired cost screen shows native
root acquisition rising 8.5–10.6%, with smaller/mixed WASI changes. Edge-clone and
overwrite costs rise because those operations now establish owning escapes; shared
clone controls also include small native regressions. See
[results](../benchmarks/gc/IMPLEMENTATION.md#owning-generic-clones-and-guard-escapes).
Public demotion, root revocation and custom Trace safety contracts remain to be
audited before generalizing writer graph retention. This repair does not close D/E
or a qualification gate.


Root revocation follow-up (2026-11-14): numeric removal now accepts only manual
registrations, and address-wide revocation is unsafe. A kind bit plus 63-bit
non-reused sequence preserves entry/token sizes and exact removal across ID reuse.
The prior-runtime weak-probe regression fails as expected; all 20 final checks pass
across native/WASI, Miri, Tree Borrows and sanitizers. All 672 timing processes pass:
native root acquisition rises 1.7–3.2%, shared cloning is flat, and WASI changes are
mostly mixed. Demotion, attachment and custom Trace contracts remain open; a frozen
compile witness confirms that four intended boundaries still accept safe code.
No general writer scan is bypassed and no D/E or qualification gate is closed. See
[results](../benchmarks/gc/IMPLEMENTATION.md#manual-root-removal-and-explicit-revocation).

Owner-binding follow-up (2026-11-14): demotion/attachment now require an unsafe
owner context and `Trace` is an unsafe implementation contract. Foreign strong
fields keep source ownership; nested cells attach through builtin containers,
detach before movement and bind their destination before demotion. Moved same-heap
Java conditional objects rebind their queue owner without strengthening referents.
All 21 validation checks pass across native/WASI, Miri/Tree Borrows, sanitizers and
VM integrations. All 1,008 cost-screen processes pass: ordinary native controls are
broadly flat, while corrected nested writes cost 14.4–16.1x native / 9.1–9.9x WASI
relative to the old missing-hook path. Conservative owning containers, cross-heap
cycles/Java queue delivery and a separately reproduced foreign-guard mismatch remain
open. No general held-writer certificate or qualification gate is closed. See
[results](../benchmarks/gc/IMPLEMENTATION.md#owner-aware-tracing-and-nested-cell-ownership).

Foreign-guard follow-up (2026-11-14): root guard creation now validates the
allocation's source collector before registering; the fallible API returns a
mismatch error and the infallible API panics before registration. The frozen
weak-probe failure is repaired. All 21 checks pass across native/WASI, VM, sanitizers
and matching-source Miri/Tree Borrows. The 1,008-process paired screen reproduces
the preceding baseline binaries and mostly retains near-one ratios, with small
reported control regressions. No pause, D/E, peer, capacity or endurance gate is
closed. See [results](../benchmarks/gc/IMPLEMENTATION.md#root-guards-validate-their-allocation-heap).

Allocation follow-up (2026-11-14): attachment now borrows the collector-owned weak
context, removing a temporary weak increment/decrement pair confirmed in frozen
native disassembly. All 20 checks and 1,008 paired timing processes pass. Native
allocation falls 1.7–2.3% and managed-cell allocation 1.4–2.7%; WASI results and control
variation remain in the [report](../target/gc-implementation/borrowed-owner-REPORT.md).
This is a measured mechanism improvement; D/E, pause, peer, capacity and endurance
qualification remain open.

Scoped E follow-up (2026-11-14): the optional projection certificate now covers
object-array slices whose direct Value edges stay independently owning. Held-writer
fixtures preserve children, reclaim unrelated/obsolete objects, retain escaped
values and process separately rooted conditional targets on native/WASI. All 25
checks pass, including five-fixture Miri/Tree Borrows and recorded corrected-checker
models; deliberate missing rooting/revocation and weakened ordering fail. Default
binaries are identical and all 1,344 feature timing processes pass, with unchanged
layouts. See [costs and limits](../target/gc-implementation/rooted-array-REPORT.md).
The feature remains off; linear bulk ownership work, general graph/callback progress,
D/E and external qualification remain open.

### Benchmark lanes and workload matrix

1. **Collector mechanisms:** extend the existing Rust drivers with fixed operation
   traces for allocation sizes 32 B–1 MiB, 0/10/50/90% survival, root-only churn,
   overwrite-only barrier traffic, large sparse reference arrays, cyclic graphs,
   weak/final/phantom pressure, and fragmented heaps. Compare baseline, each feature
   alone, and the integrated candidate. Record operations, requested payload bytes,
   actual managed bytes, metadata, peak RSS, copy/recopy bytes and collector CPU.
2. **Same Java application:** `benchmarks/gc` runs identical bytecode under Ristretto,
   G1 and generational ZGC on the same JDK 24+ class libraries. Default HotSpot JIT
   is the deployment comparison against Ristretto's verified production JIT path.
   Keep Ristretto interpreter and JIT modes explicit for both GC policies;
   `g1-int`/`zgc-int` are additional interpretation controls. These do not isolate
   collector implementation cost. Publish J0's actual hot-path coverage, J5's
   execution/GC ablations and allocation-free loop/call/observer controls without
   subtracting quantiles. A missing compiled path remains a failed coverage result.
3. **Wasm:** run identical Java bytecode under WASI, and collector work under bare
   Wasm. Compare baseline/candidate within target; report host-turn gaps and work
   per poll. Browser integration must measure real event-loop delay, host memory
   growth, and pending finalizer/reference-handler progress. Native HotSpot numbers
   are context, never a claim that ZGC or G1 executes in Wasm.

The initial shared workload contains escaping arrays, 16-node root churn and
mixed-size arrays overwriting a persistent reference array. It is a single-mutator
screen. Dedicated multi-mutator publication, pinning, stalled-admission, finalizer,
browser event-loop and fragmentation fixtures remain required work; Java monitors
alone do not model Ristretto's payload guards. Scale independent mutators and GC
workers across 1/2/4/8 available cores, while enforcing the same **total** CPU quota.

Use logical heap limits of 64/256/1024 MiB initially, with live-set sizes appropriate
to each collector's actual layout, then extend to larger native heaps. Equal `-Xmx`
and Ristretto managed limits are not equal RSS. Compare both fixed logical limits
and fixed cgroup physical-memory budgets, recording metadata, code, stacks, staging
and reserves. Do not impose a small `RLIMIT_AS` on ZGC: virtual-address reservations
can be much larger than physical use. Keep large-page/NUMA policy explicit and equal.

### Measurement and acceptance policy

- Freeze the source archive and binary hashes together in one build step. Reject
  source/binary changes or different policy/CPU/memory settings within a comparison.
  Randomize paired runtime order; run one measured child at a time with no concurrent
  external builds, tests or profilers. The measured JVM's own JIT compilation remains
  part of its execution and resource budget. Record JDK flags, Wasmtime and compiler
  versions, compilation policy and code-cache configuration.
- Require J0–J3's per-process compiled-path evidence before qualifying a native JIT
  comparison. Report time-weighted execution coverage, fallback/helper costs,
  compilation during warmup/measurement and code memory. Required hot paths must
  execute compiled code during measurement; compilation after an interpreted loop
  started does not satisfy this. Missing coverage or GC-exposure evidence cannot pass.
- Validate steady state using compilation, throughput, live-set and GC-activity
  histories rather than a fixed warmup duration alone. Report cold-start/time to
  stable performance separately. Profile in diagnostic forks, quantify observer
  overhead, and keep background compiler CPU within the same total process quota.
- Determine saturation separately for each runtime. Search a fixed-rate capacity
  curve, then compare all runtimes at the same absolute offered rates and also at
  50/75/90/100/110% of their separately measured capacities. Label these two views.
  A candidate passes only when completed throughput and bounded backlog/memory
  remain stable; allocation bursts alone are not sustainable capacity.
- Keep the arrival schedule independent of completion. Measure from scheduled
  arrival through completion; retain unfinished requests, errors and timeouts.
  Report service time separately. Quantiles of completed requests are censored
  when requests remain unfinished; such runs cannot pass a latency gate.
- Record p50/p95/p99/p99.9/p99.99/max, full bounded histograms, allocation waits,
  GC steps, heartbeat gaps, RSS/live/retired/committed memory and backlog time series.
  Keep global safepoint pauses, local access waits, caller assists and cooperative
  slices as different metrics. No inference of zero pauses from missing telemetry.
- The intermediate v2 STW screen requires every
  observed native pause **<1 ms** in each qualification run within the declared
  operating envelope. A pause of 1 ms or longer fails, even if percentiles pass.
  This is the sub-millisecond acceptance goal, not a measured result or a proof of
  a universal worst-case bound from finite trials.
  Time a stop from the request to suspend mutators through their release, including
  rendezvous and GC work; retain each component separately. Include equivalent
  heap-wide publication/recovery stalls. Record count, total stopped time, fraction
  of elapsed time and full pause distributions alongside application response tails.
  Missing telemetry or insufficient samples cannot pass a pause-percentile gate.
  The zero-STW path requires a coordination audit and complete event coverage;
  it does not require inventing percentiles for an empty pause distribution.
- Use at least five independent paired forks for comparison claims, sufficient
  warmup validated by stability, and at least 1,000,000 completions per run for
  p99.99 claims. Use paired bootstrap confidence intervals across independent runs;
  preserve raw histograms without treating per-operation samples as independent
  experiments or averaging quantiles into a pooled percentile.
- Performance evidence: measure sustainable completed throughput and scheduled-arrival
  p99.9/p99.99 at common offered rates across the declared native primary workload
  set. Report baseline and G1/ZGC comparisons with 95% confidence intervals under
  equal resource ceilings. The intermediate v2 screen records peer differences.
  Full release requires higher sustainable throughput and lower scheduled
  p99.9 and p99.99 than each peer on the declared native workload matrix. No failed/dropped requests, bounded backlog/memory, declared
  workload latency limits and the separate STW budget remain required. Set workload
  latency limits before tuning, independently of peer results. Label undefined or
  resolution-limited ratios rather than treating them as improvements. Publish every
  workload, including regressions, without averaging away a failed qualification gate.
- Retain the historical native operation targets of p99.9 <100 µs and p99.99 <1 ms
  as diagnostics on their designated workload, not additional release gates. They
  do not measure STW pauses. Set separate Wasm host-turn and operation budgets from
  the observer baseline before tuning; the native STW threshold does not apply.
- After short tests pass, run three paired one-hour trials and a separate 24-hour
  candidate run at the highest **common passing** rate. Fail on growing backlog,
  growing retired/RSS memory, content/reference violations, exhaustion within the
  declared envelope, or teardown leaks. Test overload and recovery separately.

The initial `benchmarks/gc` runner is an exploratory screen, deliberately never a
qualification verdict. It provides the shared workload, source/binary capture,
checksums, fixed-rate backlog, process RSS/CPU, raw histograms, comparison tables,
and SVG/PNG graphs. Enforced per-child cgroups, paired confidence intervals and
an adaptive fixed-arrival capacity screen are implemented. Validated capacity,
compiled-path reporting/gates from J0/J5, observer-overhead experiments, the complete
stalled-publisher/trace fixtures and browser scheduling measurements remain
qualification work. Existing mode labels and short gate reports do not implement
the new execution-coverage requirements. J5 now has a v2 checker without peer-relative
thresholds and with an exclusive 1 ms pause gate, frozen limits and evidence binding.
Complete global-source runtime pause capture (the opt-in publication-writer
producer is diagnostic and partial), measured-window execution/canonical-GC producers and their
overhead qualification remain required before real runs can satisfy that contract.
Historical reports retain their original criteria through the v1 evaluator; the
policy revision does not reclassify any previous run as passing.

### Publication and decision artifacts

For each candidate publish the configuration and source/binary hashes, correctness
results, raw successful and failed runs, and these figures:

- Sustainable throughput versus offered load, with backlog/failure overlays.
- Scheduled-arrival p99.9/p99.99 versus offered load, plus service-time distributions.
- STW/rendezvous pause distributions, observed maxima, event counts and total
  stopped-time fractions against the pause budget, or evidence for zero STW.
- Throughput–latency tradeoff under the same CPU and physical-memory ceilings.
- RSS, logical live bytes, retired storage and committed regions over time.
- CPU per completed operation and worker scaling; root/barrier/allocation ablations.
- Hot-method compilation/entry/backedge coverage and fallback reasons; attributed
  interpreter, generated-code, helper, compiler and collector costs.
- Execution/GC ablations, code memory, cold-start and time-to-stable-performance
  curves, with the required workload manifest and all coverage failures.
- Forced-contention reclamation progress and Wasm host-turn gaps.

Only change defaults after all required gates pass. If Ristretto misses the
sub-millisecond pause budget, declared operating limits or correctness/progress
requirements, retain that result and use the attributed cost to select the next
change in the J0–J5 execution workstream or A–G collector work.
Interpreter/JIT/runtime improvements are required plan scope; report them separately
so a compiler change is not
presented as a GC algorithm win. Keep qualification open if coverage succeeds but
sustainable capacity, declared response-time limits, STW pauses or resource limits
remain unqualified. A measured shortfall against either G1 or ZGC keeps the
full-goal performance gate open.

### Implementation status after the first overhead/progress delivery

- A: optional descriptor-publication, owning-root, barrier-record/drain timings;
  diagnostics-on/off mechanism ablations; frozen before/after binaries; paired
  process confidence intervals. Timing includes nested work and remains opt-in.
- B: context-cache operations borrow the installed context without transient Arc
  increments/decrements. Descriptor/payload range and credit ownership remain
  unchanged. Metadata now owns erased initialized storage directly, removing one
  system allocation for its destructor closure; paired allocator-count trials
  show fewer requests and lower process peak memory, without an established
  timing gain. Shared type-operation tables, one value pointer and typed finalizer
  dispatch reduce requested metadata bytes by 32 per allocation in the follow-up
  screen, with about 7.4% lower whole-process peak memory and no established
  allocation-time gain. Metadata slab allocation and removal of registry allocation on the
  complete publication path remain open.
- C: constant-time duplicate coalescing in a two-record window preserves both epoch
  tags and barrier kind. A popped record retains its separate reservation.
  Owning handles store their root registration instead of retaining a redundant
  weak collector reference, so cloning touches one shared reference count.
  Identity and weak upgrades reuse their original publication token through root
  registration. Context owner caches and fixed slots were evaluated and removed
  after measured root/clone regressions; their raw evidence remains available.
  Root retirement batching and removal of root-page/shared ownership costs remain open.
- D: counted admission now increments count and revision in one atomic CAS;
  production-state Loom tests cover closure, completed-publisher ABA and saturation.
  Conditional-owner construction now retains eligible referents with temporary roots
  and per-slot pending counts, allowing unrelated full collections and reentrant
  collection during attachment. It no longer holds global admission across hooks.
  Collector steps now yield on legacy writer acquisition, mark-queue inspection,
  and terminal mark/relocation claim contention instead of waiting while holding
  their coordinator/publication writer. Forced schedules preserve closure exclusion
  and resume exact queued work after release. This removes those waits but does
  not make the admitted operation helpable or qualify sustained progress.
  **A stalled admitted publisher still prevents closure. Helping and the complete
  no-global-blocking protocol remain unimplemented.**
- E: busy clear continuations leave the sweep cursor and rotate in the retirement
  queue with their offset, finalized state and byte charge intact. A requested full
  start drains an earlier young snapshot to prevent floating-root feedback.
  Forced progress and repeated concurrent allocation tests cover these changes.
  Finalization, clearing and destruction now execute outside cycle/coordinator locks,
  with exclusive descriptor ownership and retired-byte accounting. Forced native
  schedules complete three subsequent full collections during each blocked callback;
  late continuations preserve offsets and exactly-once destruction. Blocking tracing
  and old-handoff callbacks still require a graph-retention/progress protocol.
  Declared reference-free types now omit mark/handoff payload access entirely;
  held managed/standard leaf locks cannot stall collection. Types with trace
  actions retain their existing graph reservations and locking requirements.
- F/G: payload locking and defaults remain unchanged; no complete forwarding
  protocol or passing integrated qualification evidence justifies changing them.
- H: shared-Java runner gains enforced per-process CPU/physical-memory limits and
  memory/CPU time series. Short comparison gates reject insufficient or censored
  evidence. Missing endurance/provenance scripts referenced by CI are implemented.
  Adaptive capacity search preserves every point, requires all forks to pass,
  confirms bracket endpoints, and rejects contradictory or invalid evidence.
  It does not establish sustained capacity or automatically qualify a release.
  An actual Chromium host-turn fixture now covers fixed arrivals, async allocation,
  bounded cooperative teardown, weak clearing, finalization, heartbeat/frame gaps,
  memory growth and failed/censored trials. Statistical memory-trend gates, a full
  browser VM workload, calibrated latency budgets, three paired hours and the
  separate 24-hour qualification remain open.

These changes implement concrete parts of the delivery sequence. They do not
establish the sub-millisecond STW pause goal or complete sustainable-capacity and
endurance qualification. See `benchmarks/gc/IMPLEMENTATION.md` for measured results
and validation limits of this delivery.

Stage D backing follow-up (2026-11-14): the isolated catalog now has a pooled-state
variant with explicit slot ownership and no system allocation for its prepared
control/membership updates. Native/WASI, Miri/Tree Borrows, sanitizers and both stock
and corrected-checker models pass. Negative controls reject early reuse, missing
acquire, replay and late completion. All 90 count processes pass and reproduce the
prior controls: one system allocation per numeric lifecycle instead of 18, at the
cost of about 55% more retained requested backing native / 72% WASI at 16,384
residents. These are not GC timings or RSS results. Actual metadata/initial roots,
byte charging and a participant/progress-reserve policy remain prerequisites. No
runtime admission caller or default changes. See the
[pooled experiment](validation/catalog/catalog-pooled-prototype/README.md).


Stage D tracing follow-up (2026-11-14): the isolated tagged catalog now commits
per-domain epoch, retention strength and exact traversal tickets in one immutable
membership effect. Six floors match the production retention boundaries; stronger
discovery replaces work without double counting, and stale acknowledgements cannot
clear newer work. All 60 native / 53 WASI fixtures, memory tools, 20 catalog and six
pool models pass. The 224-process cost screen retains a 3.7–6.1% native facade
regression and increased backing; zero loop allocations persist. See the
[report](../target/gc-implementation/catalog-trace-REPORT.md). This completes numeric
mark/work coupling only. Real metadata/initial roots, graph publication, birth and
generation exemptions, conditional-reference decisions and every admitted caller
remain required before admission removal. D/E and qualification remain open.


Stage D real-ownership follow-up (2026-11-14): all constructor paths now seal
construction roots and conditional-owner reservations with their prepared metadata
and optional initial registration in one movable private publication record. Real
graph fixtures complete helper publication/cancellation without origin TLS. Thirty
checks pass, including native/WASI, both Miri modes, sanitizers and targeted VM
coverage. Four faults fail on both targets; all 72 counter and 1,176 timing runs
are preserved in the [report](../target/gc-implementation/publication-owner-REPORT.md).
This is owned handoff, not shared helpable registration. The heap borrow still
excludes final collector destruction; collector-owned records need explicit shutdown
ordering. Registration/root cleanup retain admission, and D/E remain open.
