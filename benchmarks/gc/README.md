# GC workload comparisons

Run identical Java bytecode on Ristretto, generational ZGC and G1. Save raw
histograms, failures, command lines, source archives, executable hashes, resource
measurements, comparison tables and SVG/PNG graphs. The implementation roadmap is
in [the GC plan](../../ristretto_gc/PLAN.md#next-delivery-sequence-overhead-progress-and-external-qualification).
The [initial 128-run screen](BASELINE.md) records observations and links to local
reports and graphs; it does not qualify throughput, tail latency or endurance.

The strong-graph publication comparison includes 60 whole-JVM processes, paired
native and WASI collector measurements and six one-minute endurance trials.
[Measurements, graphs and reproduction](publication/434a074e/README.md).

For fixed-arrival adaptive searches, render the completed rate points from
their plan directories. The reporter replays each complete fork against its
declared workload, executable, admission, resource limits and measurement
window. It records offered rate and duration for each point and exports the
individual deadline counts and response histogram bounds alongside the graph.

```sh
python3 benchmarks/gc/plot_capacity_points.py PEER_PLAN_DIR RISTRETTO_PLAN_DIR --output NEW_REPORT_DIR
```

The alpha preview runs Java graph, weak-reference and reclamation checks on
the JVM collector across native interpreted, native compiled and WASI tiers.
The retired catalog's finite-slot contention measurements are historical.
[Unified JVM checks](../../GC_PREVIEW.md) · [Historical leaf contention study](../../ristretto_gc/CANONICAL_CONTENTION.md).

These are **end-to-end runtime measurements**, including interpreter/JIT execution
and instrumentation. They are not isolated GC-speed rankings. `ristretto-baseline`
and `ristretto-candidate` explicitly interpret; native `ristretto-baseline-jit` and
`ristretto-candidate-jit` enable Ristretto's existing JIT (unsupported methods can
still fall back to interpretation). The driver records the selected mode. `g1` and
`zgc` use HotSpot's default JIT; `g1-int` and `zgc-int` use `-Xint`. Keep these modes
separate when attributing a difference. Wasm has separate figures and no JIT lane.

The [helpable-publication design checks](../../ristretto_gc/validation/HELPABLE_PUBLICATION.md)
run with `cargo test -p ristretto_gc --test helpable_publication_model -- --nocapture`.
Four expected counterexamples and seven reduced positive checks constrain the next
runtime redesign; they are not collector progress benchmarks or qualification.

## Build and run

The runner currently requires Linux, Python 3.11+, GNU `/usr/bin/time`, and a local
JDK 24+ (both `java` and `javac`). It uses one JDK for all native Java runtimes.
JDK 24+ avoids ambiguity between generational and legacy ZGC modes. Build immediately
before recording; the runner freezes sources and binaries but cannot prove that
a user-supplied binary was built from those sources.

```sh
cargo build --release --locked -p ristretto_vm --example gc_compare
python3 -B -m unittest discover -s benchmarks/gc -p 'test_*.py'
python3 benchmarks/gc/run.py \
  --jdk /absolute/path/to/jdk-25 \
  --cpus 0 1 --workers 2 --heap-mib 256 \
  --seconds 30 --warmup 10 --repeats 3 --timeout 180 \
  --runtimes ristretto-baseline ristretto-candidate g1 zgc g1-int zgc-int \
  --workloads control allocation roots mixed --rates 0 100 1000 \
  --output target/gc-comparison
```

Choose CPUs available in your affinity; `os.sched_getaffinity(0)` lists them.
Runs are sequential and paired in reproducibly shuffled order. Builds and tests
must finish before timing begins. Existing output directories are rejected.
The default three repetitions are a screen, not the five-fork qualification gate.
The default CPU/heap settings are explicit budgets, not tuned recommendations.

| Workload | One operation |
| --- | --- |
| `control` | Volatile scalar publication with the same timer/observer loop; no workload allocation |
| `allocation` | Allocate and publish a 256-byte array, write/read endpoint witnesses |
| `roots` | Allocate and publish a 16-node linked list, traverse and verify all node values |
| `mixed` | Allocate 32–4096-byte arrays plus a 1-MiB array every 4096 operations; overwrite a persistent reference array |

All allocation workloads escape into a static 2048-slot array. Each operation
checks content; a post-measurement audit verifies the retained graph. The runner
independently validates the checksum for the exact completed operation count.
This is one Java mutator; it does not qualify multi-mutator contention or payload
pinning. Allocating nodes is not a pure root-operation microbenchmark.

Rate zero measures saturation. A positive rate uses a fixed arrival schedule,
records service and scheduled-arrival response time, and reports unfinished work
at the end of the window. It never drops scheduled work from the denominator to
make an overloaded run look successful. Completed-request latency is censored
when requests remain unfinished; inspect backlog and completion counts together.
Waiting uses Java sleep/yield, so the control also reveals scheduler/timer costs.

The fixed histogram is exact to one microsecond through 1024 µs, then uses 64 bins
per doubling. Percentiles are upper bucket bounds; maxima retain nanoseconds.
At least one million completions **per process** are needed before treating p99.99
as more than an exploratory value. A warmup duration alone does not prove JIT or
heap steady state. Check longer runs and observer overhead before interpretation.

## Execution coverage diagnostics

Build the separate `execution-profile` feature to record production eligibility
reasons, queued/compiled/failed state, compilation wall time, generated code bytes,
actual generated-code entries, taken backward branches and interpreter/intrinsic entries. Method instances
with identical names remain distinct. Counts at the native call boundary distinguish
compiled-but-never-entered code from execution. This feature adds locks/atomics and
retains method/code metadata; leave it off for throughput/latency measurements.
`run.py` rejects its output as a timing comparison.

```sh
cargo build --release --locked -p ristretto_vm --example gc_compare --features execution-profile
python3 ristretto_gc/validation/provenance.py \
  --binary target/release/examples/gc_compare --output target/gc-profile-build
python3 benchmarks/gc/coverage.py \
  --binary target/gc-profile-build/gc_compare --jdk /absolute/path/to/jdk-25 \
  --policies candidate candidate-jit --workloads allocation roots mixed \
  --cpus 0 1 --output target/gc-coverage --plot
```

The diagnostic runner validates workload content, freezes build/source/bytecode
inputs and preserves every failed fork. It supports the same optional cgroup
ceilings and actual WASI with `--wasmtime` and an instrumented `.wasm` build; Wasm
policies must omit `-jit`. Graphs show invocation shares, not CPU shares. The core
manifest currently requires `GcBench.main`, `operation`, `Histogram.record`, plus
`Node.<init>` for roots. Missing/ambiguous method evidence cannot count as compiled
coverage. Lifetime totals include startup, warmup and output, and do not establish
measurement-phase execution or loop coverage. The full J0/J5 gate remains unmet
until measurement-phase records, hot-loop execution, dependency/observer coverage
and CPU attribution are qualified. New profiles count taken explicit branches to
the same or an earlier bytecode offset, including the selected conditional/switch
edge. These are not inferred loop iterations. Missing counters in historical
profiles remain unavailable.

Embedding diagnostics can bracket work with `VM::execution_profile_checkpoint`
and `VM::execution_profile_since`. The interval retains method identities, counter
deltas, compiled-code presence at the start, and code bytes at both boundaries.
It does not reset lifetime totals, stop execution or wait for queued compilation.
Native snapshots report their boundary costs; Wasm records counters without a
host clock. Concurrent counters have separate snapshot cuts, so these intervals
do not satisfy the v2 evaluator's exact measurement-window contract.
`gc_compare` now emits a separate main-invocation interval in diagnostic builds;
`coverage.py` preserves it as `interval.json`. This excludes VM initialization
before invocation but still includes main-class loading, warmup and reporting.
Compiled-code presence does not by itself establish dispatch readiness.

## Collector policies

`ristretto-baseline` uses the current collector defaults with the requested heap
and worker count. `ristretto-candidate` enables compaction, context roots, 32 barrier
records, local allocation caches, prediction, adaptive workers, counted publication,
independent marking/phases/relocation, work stealing, 16 metadata/region shards,
8-task mark batches, generation overlap, old-only interval 4, cost-aware relocation
and a heap/64 evacuation reserve. The precise configuration lives in
[`gc_compare.rs`](../../ristretto_vm/examples/gc_compare.rs); it is experimental.
The reserve shares the physical arena budget. This combined screen does not
replace individual-feature ablations using the existing collector drivers.

Workers zero stops native GC workers and uses cooperative VM progress. For native
workers, the total process CPU affinity bounds mutator and collector together.
HotSpot receives matching `ActiveProcessorCount`; its collector thread policy is
left ergonomic and recorded in GC logs. Equal numeric heap limits do not mean
equal object representation or RSS. Supplying `--cgroup-parent`, `--rss-mib`, and
`--cpu-quota` together enforces physical-memory and total-process CPU ceilings.
Without those options the runner records usage without installing ceilings.
Process RSS/CPU include startup, warmup, measurements and output.
Do not compare CPU/operation without accounting for these other phases.

For a paired implementation comparison, freeze both freshly built revisions with
`ristretto_gc/validation/provenance.py`, then use `--ristretto-before OLD/gc_compare
--ristretto NEW/gc_compare --runtimes ristretto-candidate-before ristretto-candidate`.
Both rows use the candidate policy; `-before` selects the prior executable. The
same facility supports `ristretto-baseline-before`, `wasm-baseline-before` and
`wasm-candidate-before` with `--wasm-before`. These lanes interpret. Build/source
hashes must match, both source archives are copied into the results, and runtime
order is randomized inside each workload/rate/repetition pair. The existing
G1/ZGC qualification evaluator does not treat a prior revision as an external peer.

Generate paired revision intervals and graphs from those raw results:

```sh
python3 benchmarks/gc/revisions.py RESULTS \
  --before wasm-candidate-before --after wasm-candidate \
  --output target/gc-revision-comparison --plot
```

This requires every selected process pair, matching targets/resource ceilings and
unchanged measurement inputs. Failed, missing or duplicate members invalidate the
comparison. Unfinished work remains in throughput and suppresses latency ratios;
zero-resolution samples also have no multiplicative ratio. The output preserves
raw-input hashes and copies of the analysis sources. Intervals are exploratory,
unadjusted and do not qualify sustained capacity or extreme tails.

The native `ristretto_gc/examples/identities.rs` driver isolates the production ID
allocator with preallocated output vectors and verifies every returned ID for
uniqueness after timing. Build it for each revision, freeze each binary with
`validation/provenance.py`, then use `benchmarks/gc/identities.py --before OLD/identities
--after NEW/identities --output RESULTS --cpus 0 2 4 6 --cpu-quota 4
--cgroup-parent DELEGATED_PARENT --producers 1 2 4 --plot`. The runner preserves
raw evidence, resource caps, source archives and graphs. This isolates shared
counter contention; it does not measure collector or Java throughput. The
`mechanisms` driver additionally supports `owning_allocation`, `edge_clones` and
`unroot` to expose ownership-transfer costs alongside ordinary allocation.
`guard_clones` measures shared `GcRootGuard` clone/drop, while `clones` measures
owning `Gc` clone/drop; both retain the original root through each timed batch.
`pending_poll` measures an empty reference-queue poll. `pending_transfer` prepares
queued references before timing and measures validated transfer/drop of their
owners; wall time also includes preparation and collection.

## Wasm

The comparison candidate uses a cooperative configuration on Wasm: no native
workers, distributed claim/phase coordination, stealing queues or metadata/region
sharding. It retains counted publication, context roots/buffers, local allocation,
prediction, generation overlap and optional guarded compaction. This is an
experimental benchmark policy, not a qualified library default. The retained
policy's short throughput gains, memory tradeoff and uncertain fixed-rate tails
are recorded in [IMPLEMENTATION.md](IMPLEMENTATION.md).

Build the same entry point as a WASI component and provide Wasmtime explicitly:

```sh
cargo build --release --locked --target wasm32-wasip2 \
  -p ristretto_vm --no-default-features --example gc_compare
python3 benchmarks/gc/run.py \
  --jdk /absolute/path/to/jdk-25 --wasmtime /absolute/path/to/wasmtime \
  --runtimes wasm-baseline wasm-candidate --cpus 0 --workers 0 \
  --workloads control allocation roots mixed --rates 0 100 \
  --seconds 10 --warmup 5 --repeats 3 --timeout 180 \
  --heap-mib 256 --output target/gc-comparison-wasi
```

Only the JDK and compiled class directory are preopened. The component reads the
same JDK class-library archive used by native runs. Wasm worker count is always
zero; execution is cooperative. Process RSS includes Wasmtime and its compilation
overhead. This suite does not measure browser event-loop scheduling, bare-Wasm
clockless execution, or claim HotSpot runs on Wasm; those require separate fixtures.

## Reports and graphs

Calibrate fixed-arrival operating rates with fresh forks and explicit response
and backlog limits before choosing rates for longer paired comparisons:

```sh
target/gc-plot-env/bin/python benchmarks/gc/capacity.py \
  --jdk /absolute/path/to/jdk-25 --cpus 0 2 --cpu-quota 2 \
  --cgroup-parent /absolute/path/to/delegated-parent --rss-mib 512 \
  --runtimes ristretto-candidate g1 zgc --workloads allocation roots \
  --initial-rate 10000 --max-rate 100000000 --seconds 10 --warmup 5 \
  --repeats 5 --p999-us 1000 --backlog-us 10000 --plot \
  --output target/gc-capacity
```

The search expands or contracts the rate, bisects a passing/failing bracket,
then repeats both endpoints. Every fork must finish all requests and meet the
declared limits. A timeout, corruption, memory failure, or insufficient sample
count invalidates a point; it cannot become an apparent capacity upper bound.
Contradictory confirmations and lower-rate failures remain visible and invalidate
the bracket. Reaching the point/rate limit can leave the result unbracketed.
Each point retains the normal runner evidence; SVG/PNG graphs show every tested
rate and its outcome. Brackets are observations, not confidence intervals or
endurance certification. Adaptive points must not replace independent confirmation
trials. Native and Wasm searches remain separate runtime comparisons.

The collector-only `mechanisms.py --workloads clones roots handles weak allocation` selection
also measures cloning an existing owning handle separately from creating and
removing a new root registration. The clone fixture registers its shared root inside the execution context. Historical
`mechanisms-owner` results used an unscoped shared owner and are labeled accordingly.
`handles` resolves a live typed identity; `weak` upgrades a live conditional edge.
Both create and retire a fresh owning registration. Older driver binaries lack these
lanes; compare revisions using identical overlaid driver source.

```sh
python3 -m venv target/gc-plot-env
target/gc-plot-env/bin/python -m pip install -r benchmarks/gc/requirements.txt
python3 benchmarks/gc/report.py target/gc-comparison
target/gc-plot-env/bin/python benchmarks/gc/plot.py target/gc-comparison
```

Outputs:

- `metadata.json`, `sources.tar.gz`, frozen Ristretto binary/component, and bytecode:
  source identity, toolchains, host, affinity, cgroup observations and input hashes.
- `schedule.json`, `raw.jsonl`, and `run-*/`: every command, stdout/stderr, exit status,
  timeout, per-process CPU/peak RSS, raw measurement and HotSpot GC/safepoint log.
  GC activity and HotSpot pause summaries cover the entire process, including
  warmup/output. Ristretto global-pause data is null, not a fabricated zero. A run
  without observed collection cannot support a collector-performance conclusion.
- `integrity.json`: detects source/JDK-library changes during measurement.
- `REPORT.md` and `summary.json`: medians and min–max of **per-process** metrics.
  No pooled percentile is synthesized by averaging run percentiles.
- `graphs/native-{throughput,latency,latency-p9999,memory,load}.{svg,png}` and corresponding
  `wasm-*`: throughput, scheduled-arrival p99.9/p99.99, RSS, backlog and latency tradeoffs.
  Log axes are labeled; failed runs are counted rather than plotted as zero.
  Graph provenance binds the input hashes and renderer version; repeated backlog
  traces are shown individually, with workloads in separate panels.

The runner exits unsuccessfully for failed/invalid/timed-out children or changed
inputs and retains their evidence. A missing runtime, low sample count, short run,
or saturated backlog cannot establish a win. Reports always state that qualification
is incomplete. Long-run approval criteria, resource caps, statistical comparisons,
contention fixtures and native/Wasm contracts are specified in the plan.

## Reference algorithms

- [Oracle ZGC documentation](https://docs.oracle.com/en/java/javase/25/gctuning/z-garbage-collector1.html):
  concurrent generational collector; heap headroom, large pages and memory-return
  policy affect latency. A small virtual-address-space cap is not a physical cap.
- [Oracle G1 documentation](https://docs.oracle.com/en/java/javase/25/gctuning/garbage-first-g1-garbage-collector1.html):
  concurrent marking with stop-the-world evacuation. Safepoint durations and
  application response times are distinct measurements.

## Allocation during a strong mark

`marking_births.rs` starts a cooperative full collection and allocates 512
rooted objects before allowing it to finish. Each process repeats this 128
times for either scalars or single-edge objects pointing to one live anchor.
The two timers cover rooted allocation and the remaining collection with those
roots held. Every iteration checks the marked object count. The process checks
the payload checksum and heap drain after dropping the roots. Pass
`--collection young` to the comparison runner to measure minor collections.
The fixture promotes the anchor and drains a warm-up minor collection before
timing. Each measured minor cycle must promote all 512 new objects, for
65,536 verified promotions per process.

Build `marking_births` from the same example source in both checkouts using
`cargo build --locked --release -p ristretto_gc --example marking_births`.
For each checkout, record the build with `mark_cycles_build.py` as shown below.
Supply its checkout, release binary and output path separately for each revision.
The comparison runner needs Python and Matplotlib.

```sh
python3 benchmarks/gc/mark_cycles_build.py --fixture marking_births \
  --source /path/to/before-source --target native \
  --binary /path/to/before-target/release/examples/marking_births --output before.json
python3 benchmarks/gc/mark_cycles_build.py --fixture marking_births \
  --source /path/to/after-source --target native \
  --binary /path/to/after-target/release/examples/marking_births --output after.json
python3 benchmarks/gc/marking_births.py --before before.json --after after.json \
  --pairs 32 --output results-native
```

For WASI, add `--target wasm32-wasip2` to the build, record the resulting
`.wasm` with `--target wasm`, and pass `--wasmtime /path/to/wasmtime` to the
runner. On Linux, prefix the runner with `taskset -c` and an available CPU to
pin all child processes. Each run saves raw samples, a paired SVG, binary and
source fingerprints, and the median after/before time ratio with an exploratory
95% interval from resampling process pairs. These runs measure a single
collector workload. Whole-JVM G1/ZGC comparisons use the qualification suite.

## Repeated full-graph collection ablations

`mark_cycles.rs` builds either an eight-way tree or one broad root, warms three
full cycles, then times repeated complete collections of the same live graph.
Every measured cycle must mark every node. Graph checksums and complete reclamation
after root removal are checked outside timing. The interval includes all collection
phases and waits, but no application work, construction or teardown. It is neither
an allocation benchmark nor an STW/operation-latency measurement. Native worker
counts are checked directly; zero means `start()` is never called. WASI uses zero.

Both binaries receive the fixed invocation name `mark_cycles`. Native runs select
the executable separately from `argv[0]`; WASI uses Wasmtime's `--argv0` option.
The fixture retains its argument strings, and a controlled native experiment found
large timing/ratio changes from invocation-name length alone. Result-directory
names must therefore not select different argument allocations. Metadata records
the fixed name and native raw rows record the separate executable. Historical
screens retain their original command lines. Normalization removes this particular
confound; it does not establish robustness across other allocation layouts.

`--shapes barrier wide --workers 0` adds a required-buffered-edge workload with a
broad graph control. The barrier shape publishes one distinct edge per leaf each
cycle in 32-record contexts, then drives collection with one-unit steps. Publication
is included in timing. Enabled diagnostics must report exactly `(nodes - 1) * cycles`
buffered and flushed records with no overflow. No background worker can pre-mark
the targets. This compares publication plus collection cost, not isolated barrier
instruction time or contention latency. The default shapes remain `tree wide`.

Build the same fixture in two isolated source revisions, using matching Cargo
profiles, dependency locks and toolchains. Record each build before running:

```sh
cargo build --locked --release -p ristretto_gc --example mark_cycles
python3 benchmarks/gc/mark_cycles_build.py \
  --source /path/to/before-source --binary /path/to/before-source/target/release/examples/mark_cycles \
  --target native --output /path/to/before-build.json
python3 benchmarks/gc/mark_cycles_build.py \
  --source /path/to/after-source --binary /path/to/after-source/target/release/examples/mark_cycles \
  --target native --output /path/to/after-build.json
python3 benchmarks/gc/mark_cycles.py \
  --before-build /path/to/before-build.json --after-build /path/to/after-build.json \
  --cpus 2 3 4 5 6 --workers 0 4 --nodes 8191 --cycles 32 --repeats 7 \
  --output target/gc-mark-cycles --plot
```

Build WASI with `--target wasm32-wasip2`, record `--target wasm`, and run the pair
with `--wasmtime /path/to/wasmtime --workers 0`. Native and WASI have separate
reports. The runner freezes binaries (and Wasmtime), shuffles adjacent process
pairs, checks exact work/cleanup, and rejects the entire comparison if any pair
fails or is missing. It exports raw records, process-pair bootstrap intervals and
SVG/PNG graphs. Those intervals are exploratory and unadjusted for multiple
comparisons. Build manifests record supplied sources and binaries; they do not
independently attest which compiler invocation produced a binary. Complete all
builds, tests and source edits before measured runs.

Multiworker contention depends on scheduling. There is no fallback-publication
counter, so this benchmark can assess the net queue change but cannot attribute
elapsed time specifically to contended FIFO publication. Compare external JVMs
using the Java lane above, under its separate resource and coverage requirements.

## Implemented overhead and progress experiments

`ristretto_gc/examples/mechanisms.rs` isolates control, 32-byte payload allocation,
root registration/release, repeated deletion barriers, and managed overwrites.
It runs baseline, counted-admission-only, contexts, local caches, and combined
policies. Collection between 2,048-operation batches is excluded from operation
time and included in wall time. This single-mutator lane measures mechanisms;
it does not measure sustainable capacity or scheduled-arrival latency.

Freeze the old binary before changing the collector, build the new one, then run:

```sh
cargo build --release --locked -p ristretto_gc --example mechanisms
python3 benchmarks/gc/mechanisms.py \
  --before /path/to/frozen/old-mechanisms \
  --after target/release/examples/mechanisms \
  --cpu 0 --repeats 7 --operations 100000 --plot \
  --output target/gc-mechanisms
```

The runner randomizes paired process order, checks checksums and teardown, retains
failures, copies/hashes both binaries, and exports tables and SVG/PNG graphs.
Each condition runs with diagnostics off and on; compare these separately to
measure observer cost. Optional counters now attribute descriptor publication,
root acquisition/removal, barrier recording and drain. Inclusive timings overlap;
never sum them. Bootstrap intervals resample independent process pairs and are
exploratory 95% intervals without multiple-comparison adjustment.

For the separate WASI lane, build the same example with `--target wasm32-wasip2`,
freeze both `mechanisms.wasm` modules, and pass `--wasmtime /path/to/wasmtime` to the
same runner. Native/Wasm mixtures and missing or unnecessary runtimes are rejected.
The runner copies and hashes Wasmtime, records its version and preserves the harness
source. Timings come from inside the guest: they exclude process startup and runtime
compilation, and do not qualify browser host-turn latency or bare-Wasm clockless work.

## Enforced resources and comparison gates

`run.py` accepts `--cgroup-parent PATH --rss-mib 512 --cpu-quota 2` together.
The parent must already delegate the cgroup v2 CPU and memory controllers. Each
measured child gets a fresh cgroup, the same total CPU quota and physical-memory
ceiling, and no swap. The launcher records actual limits, memory events, peak
charged memory, CPU throttling and periodic charged-memory/CPU samples. It fails
on missing delegation; it never substitutes an address-space limit for RSS.
Cgroup charged memory includes file cache and kernel charges, so it is reported
separately from `/usr/bin/time` peak RSS. Limits include bootstrap and JIT overhead.
A process killed by the memory controller remains a failed result.

```sh
python3 benchmarks/gc/qualify.py target/my-comparison
```

The default evaluator uses acceptance policy **v2**, matching the current plan.
G1/ZGC throughput and scheduled-arrival latency ratios and paired confidence
intervals are diagnostics, with no superiority or fixed percentage requirement.
The candidate must meet independently predeclared workload latency/backlog limits,
equal enforced resource ceilings, measured-window compiled coverage and GC exposure,
and an observed native STW maximum strictly below 1 ms, including rendezvous and
equivalent heap-wide publication/recovery stalls. Missing or incomplete pause
capture and an audit identifying unbounded global blocking prevent a pass.
At least five complete paired forks are required; each fixed-rate process needs
one million completions for extreme-tail claims. Failed or censored pairs remain
failures. Saturation runs do not qualify response tails.

Freeze the policy with `run.py --acceptance-policy /path/to/policy.json` before
measurement. [QUALIFICATION.md](QUALIFICATION.md) defines the policy and process
evidence contract. **Current runtime producers do not emit that complete evidence**:
existing runs cannot pass v2, and the synthetic checker fixtures are not benchmark
results. Lifetime execution counters and GC phase durations cannot fill this gap.
The latter are now labeled `gc_phase_pause_ms`; `global_pause_ms` remains null
because phase logs alone omit rendezvous and complete stop coverage.

The evaluator writes a new `gates-v2.json`, exits nonzero on failure, and always
leaves overall `qualification: false`. Warmup/compilation stability, recorder
overhead, sustainable capacity, contention progress, memory/backlog histories and
hour/day endurance still require qualification. Reports are never overwritten;
use `--output` with a new path for reevaluation. Replay historical peer-ratio
criteria explicitly, preserving the original `gates.json`:

```sh
python3 benchmarks/gc/qualify.py target/my-historical-comparison --policy-version 1
```

This writes `gates-v1.json` with `historical_only: true` and retains the original
1.10/0.80/1.05 thresholds. Historical failures are not reclassified by the revision.

## Collector endurance and recovery

The repository's endurance workflow now has executable tooling at
`ristretto_gc/validation/provenance.py` and `measure.py`. It freezes sources and
binaries, maps options to the existing Rust drivers, imposes per-child cgroup
budgets, retains stdout/stderr even on failure, and checks offered work,
histograms, heap teardown, root pages, pending records and reservations. The
workflow creates an isolated delegated budget parent on its hosted runner and
uploads raw `.runs` directories even on failure.

```sh
cargo build --release --locked -p ristretto_classloader \
  --example gc_endurance --example gc_recovery_latency
python3 ristretto_gc/validation/provenance.py \
  --binary target/release/examples/gc_endurance target/release/examples/gc_recovery_latency \
  --output target/gc-frozen
python3 ristretto_gc/validation/measure.py \
  --binary target/gc-frozen/gc_endurance \
  --build-provenance target/gc-frozen/gc_endurance.provenance.json \
  --cgroup-parent /path/to/delegated/parent --process-memory-mib 512 \
  --cpus 0 2 --cpu-quota 2 --seconds 60 --rates 10000 --heap-mib 64 \
  --workers 2 --compact true --endurance-mode async --repeats 3 \
  --concurrent-publication --context-roots --barrier-records 32 --local-allocation \
  --output target/gc-endurance.json
```

The endurance workload defaults to an unrestricted mutable borrow of the
2,048-element owner array. Use `--endurance-array-write element` to exercise
the element setter used by the JVM's `aastore` instruction with the same
allocations, slot sequence, heap, pins and teardown. The harness checks the
reported write mode. Report these workloads separately.

Use `--workload recovery --recovery-mode async` with the recovery binary for
allocation/executor/timer-delay measurements. `--enforce` additionally checks the
endurance driver's operation latency thresholds. The captured capacity samples
and host memory samples still require trend analysis; passing protocol checks is
not a long-run qualification verdict. Run three paired one-hour trials and the
separate 24-hour trial only after the preceding correctness and capacity gates.

Measured implementation changes and limitations are recorded in
[IMPLEMENTATION.md](IMPLEMENTATION.md). Export collector pressure time series with:

```sh
python3 benchmarks/gc/plot_pressure.py target/gc-endurance.json \
  --output target/gc-pressure-graphs
```

## System allocation counts

Build `ristretto_gc`'s `allocation_counts` example from each recorded source
revision, then freeze both binaries. This separate instrumented lane counts
successful allocator requests and requested bytes for 32-byte managed payloads.
Reallocation counts one release and one allocation, even when it grows in place.
Requested bytes exclude allocator headers and fragmentation; charged process peak
includes setup, collection and teardown. These are not timing measurements.

```sh
python3 benchmarks/gc/allocations.py \
  --before /frozen/before/allocation_counts --after /frozen/after/allocation_counts \
  --cpu 0 --cgroup-parent /path/to/delegated/parent --rss-mib 512 \
  --operations 100000 --repeats 5 --plot --output target/gc-allocation-counts
```

For actual Wasm allocation counts, build both examples for `wasm32-wasip2`, pass
their `.wasm` files as `--before`/`--after`, and add `--wasmtime /path/to/wasmtime`.
The runner freezes and checks the runtime executable as well as both modules.
Guest allocator counters exclude Wasmtime allocations; the charged process peak
includes them. Compare revisions within the same target and runtime.

The mechanisms runner also reports and plots wall-time ratios including its
between-batch collections. Operation-only and wall-time results answer different
questions; a fast allocation path can coexist with more expensive collection.

Mechanism workloads `barrier4`, `barrier16` and `barrier64` cycle through that
many independently rooted live targets. Compare matching fixture builds: these
cases include target selection and checksum reads as well as the barrier, and
still exclude collection between batches from operation time. The contention
runner accepts `--targets` (a power of two from 1 through 256) for a shared working
set across its producers. It validates the reported target count and retains
censored cycles; changing the working set does not turn saturation into a
scheduled-arrival latency comparison.

Use `allocations.py --mode weak_retention` with the matching fixture in both
binaries to keep cleared weak handles alive after collecting their targets. This
adds retained bytes after collection and bytes released when those handles are
dropped. Retained bytes cover all counted live system allocations, not managed
live bytes. This mode helps detect metadata retained by dead weak targets; it is
separate from allocator timing and process peak memory.

## Actual browser host-turn screen

`browser.py` runs the collector in headless Chromium on its main thread, with
baseline, candidate and an allocation-free Wasm observer control. It serves only
local frozen assets, enables an isolated clock context, and preserves Chromium's
sandbox by default. Every browser process tree receives the same delegated cgroup
CPU/memory ceiling and CPU affinity. No build, test or profiler should overlap it.

```sh
cargo build --release --locked -p ristretto_gc --example gc_browser --target wasm32-unknown-unknown
python3 ristretto_gc/validation/provenance.py \
  --binary target/wasm32-unknown-unknown/release/examples/gc_browser.wasm \
  --output target/gc-browser-frozen
python3 benchmarks/gc/browser.py --wasm target/gc-browser-frozen/gc_browser.wasm \
  --output target/gc-browser-results --cgroup-parent "$GC_CGROUP_PARENT" \
  --cpus 0,2 --cpu-quota 2 --rss-mib 512 --heap-mib 16 \
  --seconds 10 --warmup 2 --rate 10000 --repeats 5 --plot
```

The clock-free Wasm fixture allocates 32-byte leaf payloads through polled futures,
retains the latest 128 sampled requests (one in eight), checks original sequence numbers after survival,
and keeps up to 128 weak edges to other payloads. One bounded finalizer proves
cooperative finalizer progress. Pending allocation futures own their inputs and are
cancelled during teardown. Root removal, two full collections and weak verification
are polled across host turns; no export invokes synchronous full collection.
The future box and browser/JS work are part of this embedding fixture's cost.

The JavaScript driver posts separate message tasks, offers requests independently
of completion, and caps allocation polls at 64 per host turn. It performs one
additional budgeted GC step per turn. Warmup permits one second to drain its offered
requests; the measured interval has no drain grace. Unfinished measured requests
remain explicit, and their completed-request quantiles are censored. This busy
host-task schedule measures one declared embedding policy; it is not an idle CPU
or arbitrary browser application guarantee.

Raw evidence includes scheduled-arrival and service histograms, individual Wasm
poll cost, host-turn work/gaps, request-bearing-turn work, one-millisecond timer heartbeat gaps, animation-frame
gaps, supported Long Tasks observations, backlog and managed/retired memory, observed
linear-memory growth and process-tree physical memory. Growth events identify the
turn where memory increased; they do not isolate `memory.grow` latency. Wasm linear
memory cannot be expected to shrink when managed objects are reclaimed. Startup,
warmup and cooperative teardown are separate from the measured phase. Histograms
use one-microsecond bins below 1,024 µs and 128 logarithmic subdivisions thereafter;
reported quantiles are bucket upper bounds, with an exact observed maximum. A startup
clock probe records positive timer increments and zero differences so the clock
floor is visible. Graph provenance binds raw data, renderer source and version.
The graph uses request-bearing turns (at least one allocation poll), including
their GC step and bookkeeping. Idle turns can dominate the separate all-turn
histogram. Maintenance-only GC turns remain in that all-turn histogram and the
heartbeat/gap observations; the request-turn percentile does not bound their cost.

The runner retains failed and timed-out browsers, verifies content/teardown and
frozen inputs, and creates SVG/PNG graphs. It always reports qualification false.
This is an actual browser collector screen, not browser execution of the Java VM
and not a Wasm port of G1/ZGC. Long-run memory plateau, sufficient extreme-tail
samples, calibrated observer budgets and the full browser workload matrix remain
required before browser latency qualification.


## Reclamation under barrier or root contention

`reclamation_contention` measures a fixed number of full or young cycles while
one or more native producers repeatedly apply an edge barrier, or acquire and
release a fresh owning registration with `--operation roots`. The root lane uses
heap edges protected by separate resident roots; cloning an already owning handle
would merely share its existing registration and measure a different operation.
Each cycle also
allocates and releases sixteen objects. The resident edge is promoted before
measurement; the young lane therefore stresses irrelevant old-edge notifications.
The driver validates checksums, resident liveness and complete teardown. A deadline
leaves the unfinished cycle explicit, even when cleanup later reclaims everything.

Freeze the example and its source with `ristretto_gc/validation/provenance.py`, then
run `benchmarks/gc/contention.py --before <frozen-example> --after <frozen-example>
--output <new-directory> --cpus 0 2 --cgroup-parent <delegated-parent> --plot`.
Both binaries must use the same example source; the driver checks its recorded hash.
Use `--targets 16 --producers 4 --cycles 1024 --seconds 10 --repeats 5` for the
four-producer screen. The runner applies identical CPU
and memory limits, randomizes independent process pairs, retains raw results and
produces SVG/PNG graphs. Ratios require every paired run to finish; censored runs
are never silently omitted. Diagnostics are enabled in both binaries. Saturating
producers can generate more contention with faster operations, so completion time
must be read alongside the achieved producer throughput. Memory-ceiling events
fail the process even when its exit status is successful. This is a mechanism
screen, not a comparison at a matched offered rate or sustained-load qualification.

## Compiled-call correctness cross-check

Primitive control flow has a separate production-path cross-check. Its
Rust fixtures use ordinary compiler admission and dispatch, asserting actual
compiled entry counts when profiling is enabled. Export the exact classes and
check the same 388 cases with HotSpot:

```sh
RISTRETTO_JIT_CALL_FIXTURES="$PWD/target/gc-primitive-fixtures" \
  cargo test -p ristretto_vm --features execution-profile --lib primitive_control_flow_tests
python3 benchmarks/gc/verify_static_calls.py \
  --fixtures target/gc-primitive-fixtures --jdk /path/to/jdk \
  --driver benchmarks/gc/CheckPrimitiveControlFlow.java \
  --output target/gc-primitive-verification
```

These fixtures check conditions, jumps, switches and a resumable loop for static
primitive methods, including operand/local merges and numeric edge cases. A bounded
Rust subprocess additionally checks native loop cancellation, migration, monitor/
argument lifetime and unrelated reclamation, with direct and interpreted callers.
Native resumes are reported separately from method invocations. Reference/call
continuations and polling overhead remain unqualified. This does not qualify
the shared benchmark's compiled hot-path coverage or any performance gate.

Experimental reference continuations have separate lifetime tests. They collect
between slices, migrate an owning context, release a consumed reference condition,
and transfer reference/exception results. Export their four finite classes for
the independent 12-case HotSpot oracle:

```sh
RISTRETTO_JIT_CALL_FIXTURES="$PWD/target/gc-reference-continuations" \
  cargo test -p ristretto_vm --features execution-profile --lib jit_runtime_helpers::continuation_tests
python3 benchmarks/gc/verify_static_calls.py \
  --fixtures target/gc-reference-continuations --jdk /path/to/jdk \
  --driver benchmarks/gc/CheckReferenceContinuations.java \
  --output target/gc-reference-continuation-verification
cargo test -p ristretto_vm --features execution-profile --lib reference_cooperation
```

The last command uses an externally bounded subprocess to cancel a suspended
reference loop through direct/interpreted callers and verify root/frame/monitor
cleanup. It injects experimental code and does not establish production admission.
The Java oracle does not run that infinite loop or test Ristretto's suspension
protocol. Helpers remain synchronous; these checks do not bound their wall time,
qualify relocation or establish performance.

Handler continuations add a loop whose only backward edge is exception dispatch,
plus a throwing call followed by another call inside its handler. Export the
fixtures and run the six finite Java checks:

```sh
RISTRETTO_JIT_CALL_FIXTURES="$PWD/target/gc-handler-continuations" \
  cargo test -p ristretto_vm --features execution-profile --lib continuation_tests::handlers
python3 benchmarks/gc/verify_static_calls.py \
  --fixtures target/gc-handler-continuations --jdk /path/to/jdk \
  --driver benchmarks/gc/CheckHandlerContinuations.java \
  --output target/gc-handler-continuation-verification
```

The bounded Rust tests additionally force collection at handler suspension, check
local/throwable ownership and obsolete-exception release, and cancel nested
interpreted/compiled calls after migration. These remain experimental backend
checks; production handlers/calls and performance are not qualified.

Explicit monitor continuations add real ownership/reentrancy, blocked-entry and
exception-cleanup checks. Export the fixtures and run their 14 finite Java checks:

```sh
RISTRETTO_JIT_CALL_FIXTURES="$PWD/target/gc-monitor-continuations" \
  cargo test -p ristretto_vm --features execution-profile --lib explicit_monitors_own_contend_and_cancel
python3 benchmarks/gc/verify_static_calls.py \
  --fixtures target/gc-monitor-continuations --jdk /path/to/jdk \
  --driver benchmarks/gc/CheckMonitorContinuations.java \
  --output target/gc-monitor-continuation-verification
```

The bounded Rust subprocess additionally checks migration, cancellation while
holding/waiting, reentrant cleanup, runnable-state restoration, root lifetime after
dead locals, residual references and unrelated reclamation. The stack-budget driver
now exercises the explicit wrappers as well, including their in-lock `holdsLock`
assertions. The Rust native-capacity fixture deliberately calls the synchronous
embedding API. Earlier wrapper post-release checks did not prove acquisition;
see the correction in `ristretto_gc/VALIDATION.md`. These tests establish neither
production monitor eligibility nor performance or full wait/notify cancellation.

The shared interpreter path also has a portable integration test, generating its
class in memory. Run `cargo test -p ristretto_vm --test monitors` natively, or use
the WASI runner/JDK directory setup in `ristretto_gc/VALIDATION.md` with
`cargo test -p ristretto_vm --target wasm32-wasip2 --test monitors`. It exercises
both publication policies without a GC worker. Native JIT and WASI interpreter
evidence remain distinct.

Allocation continuations cover object allocation, all primitive arrays, reference
arrays and complete/partial multidimensional construction. Export the exact fixture
classes and run their 79 finite HotSpot checks:

```sh
RISTRETTO_JIT_CALL_FIXTURES="$PWD/target/gc-allocation-continuations" \
  cargo test -p ristretto_vm --features execution-profile --lib allocation_continuation_tests
python3 benchmarks/gc/verify_static_calls.py \
  --fixtures target/gc-allocation-continuations --jdk /path/to/jdk \
  --driver benchmarks/gc/CheckAllocationContinuations.java \
  --output target/gc-allocation-continuation-verification
```

The Rust subprocess additionally checks suspended allocation roots, migration,
cancellation, recovery without synchronous full collection, exactly-once side
effects and cancelled `new`-triggered class initialization. Ready-only monitor and
allocation loops must consume fuel and return control. These checks inject
experimental code; they do not establish production eligibility or performance.

Run `cargo test -p ristretto_vm --test allocation_cooperation` for the shared
interpreter constructor, or add `--target wasm32-wasip2` with the documented WASI
runner/JDK setup. It checks large initialization and a node budget shared across
multidimensional recursion, cancellation cleanup and successful reuse under both
publication policies and storage modes. Bare-Wasm compilation and actual WASI
execution remain separate evidence from native generated code.

Field continuations additionally cover typed static/instance access, inherited
declaring-class initialization, linkage/null ordering and initialization lifetime:

```sh
RISTRETTO_JIT_CALL_FIXTURES="$PWD/target/gc-field-continuations" \
  cargo test -p ristretto_vm --features execution-profile --lib field_continuation_tests
python3 benchmarks/gc/verify_static_calls.py \
  --fixtures target/gc-field-continuations --jdk /path/to/jdk \
  --driver benchmarks/gc/CheckFieldContinuations.java \
  --output target/gc-field-continuation-verification
```

The Java oracle has 52 finite checks. The Rust subprocess also forces suspension,
migration, cancellation, root reclamation and initialization waiters across eight
policy/storage/worker configurations. These are correctness checks and do not
qualify production compiled execution, pause bounds or performance.

Type-check continuations add `checkcast`/`instanceof` request, resolution-wait,
root-lifetime and cancellation checks:

```sh
RISTRETTO_JIT_CALL_FIXTURES="$PWD/target/gc-type-check-continuations" \
  cargo test -p ristretto_vm --features execution-profile --lib type_check_continuation_tests
python3 benchmarks/gc/verify_static_calls.py \
  --fixtures target/gc-type-check-continuations --jdk /path/to/jdk \
  --driver benchmarks/gc/TypeCheckContinuationOracle.java \
  --output target/gc-type-check-continuation-verification
```

The Java oracle has 93 finite checks on the exact exported classes. Rust additionally
forces actual loader waits with the object owned only by the compiled context,
checks unrelated reclamation, migrates and cancels requests, and checks a ready-only
type-check loop. Its eight configurations cover both publication policies, host/
compact storage and zero/one worker. Target initialization must remain untouched.
These experimental fixtures do not establish production eligibility or timing gates.

Reference-array stores share the interpreter's checked implementation and return
owning requests before type resolution or error construction:

```sh
RISTRETTO_JIT_CALL_FIXTURES="$PWD/target/gc-reference-store" \
  cargo test -p ristretto_vm --features execution-profile --lib reference_store
python3 benchmarks/gc/verify_static_calls.py \
  --fixtures target/gc-reference-store --jdk /path/to/jdk \
  --driver benchmarks/gc/ReferenceStoreOracle.java \
  --output target/gc-reference-store-verification
```

The 38-check Java oracle covers null/bounds/type precedence, unchanged failed stores,
legal and illegal array covariance, wide residual state and caught errors. The native
Rust matrix additionally checks sixteen stalled-loader completion/cancellation cases,
root lifetime, writable destinations during waits, migration and unrelated reclamation.
`cargo test -p ristretto_vm --lib instruction::object` includes the portable interpreter
wait/cancellation and sibling-loader tests; run it on `wasm32-wasip2` with the documented
runner to check the cooperative target. These are correctness/progress fixtures;
production eligibility and performance gates remain open.

The remaining array accesses have owning request, bit-preservation and contention
fixtures, plus a 217-check independent semantic oracle:

```sh
RISTRETTO_JIT_CALL_FIXTURES="$PWD/target/gc-array-access" \
  cargo test -p ristretto_vm --features execution-profile --lib array_access -- --nocapture
cargo test -p ristretto_vm --lib bastore_boolean_uses_low_bit
python3 benchmarks/gc/verify_static_calls.py \
  --fixtures target/gc-array-access --jdk /path/to/jdk \
  --driver benchmarks/gc/ArrayAccessOracle.java \
  --output target/gc-array-access-verification
```

The native matrix covers null/bounds failures, all component kinds, raw float bits,
wide state, result ownership, ready-loop cancellation and 112 suspended schedules.
Eighty loader/read-lock schedules permit unrelated full collection. Thirty-two
held-writer cases establish request cooperation only: the collector still needs
that guard to trace the managed `Reference` enum. The recorded bounded failure is
an open D/E requirement. Run `instruction::byte::` on `wasm32-wasip2` with the runner
above to exercise the portable boolean-store correction. These tests do not qualify
production JIT coverage or performance.

Reference constants have an owning-request matrix and independent semantic oracle:

```sh
RISTRETTO_JIT_CALL_FIXTURES="$PWD/target/gc-constant-requests" \
  cargo test -p ristretto_vm --features execution-profile --lib constant_continuation_tests -- --nocapture
python3 benchmarks/gc/verify_static_calls.py \
  --fixtures target/gc-constant-requests --jdk /path/to/jdk \
  --driver benchmarks/gc/ConstantOracle.java \
  --output target/gc-constant-verification
cargo test -p ristretto_vm --lib instruction::ldc::
```

The native matrix covers narrow/wide indices, interned identity, unpaired UTF-16
surrogates, mirrors without target initialization, wide/reference residuals and
caught/uncaught native loader failures. Sixty-four forced loader waits exercise
cold strings and class literals, migration, cancellation and unrelated reclamation
across eight publication/storage/worker configurations. The oracle checks 24
semantic outcomes on the exported classes; it is not a timing comparison. Run the
portable instruction tests on WASI with the documented runner. These checks retain
experimental JIT eligibility and do not qualify the pause or sustained-load goals.

Typed exception matching has a separate continuation and semantic fixture:

```sh
RISTRETTO_JIT_CALL_FIXTURES="$PWD/target/gc-exception-requests" \
  cargo test -p ristretto_vm --features execution-profile --lib exception_continuation_tests -- --nocapture
python3 benchmarks/gc/verify_static_calls.py \
  --fixtures target/gc-exception-requests --jdk /path/to/jdk \
  --driver benchmarks/gc/ExceptionMatchOracle.java \
  --output target/gc-exception-verification
cargo test -p ristretto_vm --lib instruction::exception::test
```

Across eight publication/storage/worker configurations, 16 forced loader waits
preserve the pending throwable and handler local through full collection,
cancellation and migration. Sixteen held-writer cases cover request yielding and
cancellation/completion only; full collection through that writer remains stage E.
Ordered/duplicate catch types, native missing-class failures and non-initializing
resolution are compared with interpreter and synchronous compiled execution.
The oracle checks 17 outcomes using the seven exported classes. Its runner uses
`-Xint -Xverify:all`; separately run the same driver with `-Xcomp -Xverify:all` for
compiled HotSpot semantics. These are correctness checks, with no timing claim or
production JIT/GC qualification. The portable interpreter tests also run on WASI.

The experimental native call tests use real VM frames and dispatch, with a private
test cache insertion for methods still excluded by production eligibility. Export
their exact class files and execute them with HotSpot's verifier and interpreter:

```sh
RISTRETTO_JIT_CALL_FIXTURES="$PWD/target/gc-static-call-fixtures" \
  cargo test -p ristretto_vm --features execution-profile --lib jit
python3 benchmarks/gc/verify_static_calls.py \
  --fixtures target/gc-static-call-fixtures --jdk /path/to/jdk \
  --output target/gc-static-call-verification
python3 benchmarks/gc/verify_static_calls.py \
  --fixtures target/gc-static-call-fixtures --jdk /path/to/jdk \
  --driver benchmarks/gc/CheckSpecialCalls.java \
  --output target/gc-special-call-verification
python3 benchmarks/gc/verify_static_calls.py \
  --fixtures target/gc-static-call-fixtures --jdk /path/to/jdk \
  --driver benchmarks/gc/CheckVirtualCalls.java \
  --output target/gc-virtual-call-verification
python3 benchmarks/gc/verify_static_calls.py \
  --fixtures target/gc-static-call-fixtures --jdk /path/to/jdk \
  --driver benchmarks/gc/CheckInterfaceCalls.java \
  --output target/gc-interface-call-verification
python3 benchmarks/gc/verify_static_calls.py \
  --fixtures target/gc-static-call-fixtures --jdk /path/to/jdk \
  --driver benchmarks/gc/CheckCallContinuations.java \
  --output target/gc-call-continuation-verification
python3 benchmarks/gc/verify_static_calls.py \
  --fixtures target/gc-static-call-fixtures --jdk /path/to/jdk \
  --driver benchmarks/gc/CheckStackBudget.java \
  --output target/gc-stack-budget-verification
```

Use a new output directory for each verification. The tool preserves copied class
files, the Java driver, input hashes, commands and logs. It verifies primitive bits,
reference identity/nulls, initialization order, native/static interface targets,
exception propagation, nested allocation and 255-argument buffer reuse. This is
correctness evidence, not a G1 timing comparison or production compilation gate.
The corresponding Rust tests additionally check ownership/reclamation, actual
compiled entries and zero/one registered GC worker under both publication policies.
The call-continuation oracle checks loop-local/operand state and exactly-once call
side effects. Bounded Rust subprocesses separately test cancellation before and
during interpreted/compiled callees, migration, root/monitor cleanup and cancelled
class initialization waking waiters without replay. These use experimental cache
injection; production call admission and performance qualification remain open.
The special-call driver additionally covers constructors, superclass selection,
interface defaults, private calls, receiver-inclusive argument limits and constructor
exception propagation. The pressure fixture keeps a newly created array reachable
through the receiver while its constructor allocates beyond the heap's capacity.
The virtual-call driver covers cached/uncached overrides, private and interface
default selection, native primitive-array receivers, error ordering and reference/
exception handoff. Its pressure fixture keeps distinct caller and callee arrays
live through the same allocating invocation. The interface-call driver checks
receiver membership, default/inherited target selection, error ordering, primitive
bits, native array receivers and exception transfer. The Rust tests additionally
reject malformed interface argument-slot counts at the direct compiler boundary.
The stack-budget driver checks bounded recursive returns, leaf allocation and
caught/uncaught overflow with monitor recovery. HotSpot's stack limit and exhaustion
depth differ from Ristretto's logical Java budget; the driver does not compare them.
Rust tests separately enforce exact Java slot budgets, root cleanup, bytecode PCs
and native-entry counts. The corrected synchronized fixture requires a real monitor;
the earlier optional-monitor test did not qualify cleanup.

Linux native-recursion checks run in bounded subprocesses with core dumps disabled:

```sh
cargo test -p ristretto_vm --release --features execution-profile --lib \
  thread::jit_stack_tests::native_recursion -- --nocapture
cargo test -p ristretto_vm --lib native_stack
```

The matrix covers 1/2 MiB OS-thread stacks, both publication policies and three
mixed execution modes, with caught/uncaught overflow, root/throwable reclamation,
monitor release and thread reuse. A distinct production test checks static monitor
creation and contention. These checks qualify the exercised compiled call chains;
generated frames now have a size-aware entry check. Arbitrary native reentry,
helper stack bounds and other native platforms remain open. They do not qualify a performance comparison.

Generated-frame and subprocess-runner regressions can be run independently:

```sh
cargo test -p ristretto_jit --release --features execution-profile --lib native_frame -- --nocapture
cargo test -p ristretto_test_util --lib process
```

The backend fixtures cover register spills, missing metadata, and a frame over
1 MiB rejected on a 512 KiB segment before generated entry. These are correctness
checks; the metadata branch and native-capacity checks need performance measurement.

## Compiled-frame publication helper

`frame_roots.py` compares the real `jit_publish_frame_roots` FFI helper through an
indirect call. It covers empty/stable sets, nulls/duplicates, changing identities
and a 64-root fallback control, under gated and counted publication. Collector
workers are explicitly stopped/joined after VM startup; native `threads=0` alone
would start one worker. Schema 2 validates registered worker counts before/after
measurement and rejects completed collections during the timed loop.
It verifies the final identity set, payloads and reclamation after context teardown.
This measures one-mutator helper cost; startup, warmup, spill stores, compiled
instructions and collection are outside timing. It does not qualify a production
compiled workload, tail latency, contention, Wasm or external GC comparisons.

At each revision, build the VM library test executable with profiling disabled:

```sh
cargo test --release --locked -p ristretto_vm --lib --no-run
python3 ristretto_gc/validation/provenance.py \
  --binary /absolute/path/to/the/test-executable-reported-by-cargo \
  --output target/frame-publication-build
```

Freeze each freshly built revision before editing sources again, and retain the
Cargo command/output. Both source archives must contain the identical
`jit_runtime_helpers/publication_bench.rs` fixture. Use the frozen executable paths
in the paired run:

```sh
python3 benchmarks/gc/frame_roots.py \
  --before /absolute/path/to/frozen-before-test-executable \
  --after /absolute/path/to/frozen-after-test-executable \
  --jdk /absolute/path/to/jdk-25 --cpu 0 \
  --operations 100000 --warmup 10000 --repeats 7 \
  --output target/frame-publication-comparison --plot
```

The runner shuffles adjacent revision pairs, keeps failures/timeouts and incomplete
cells, verifies input hashes, and writes raw JSONL, resource logs, a report and
SVG/PNG graphs. Intervals resample independent process pairs and are exploratory.
Do not build, test, profile or edit sources during measured runs. The fixture is
an ignored test; normal VM test suites do not run performance measurements.

## Mutable-value ownership mechanism screen

`ristretto_classloader/examples/mutation_ownership.rs` and
`benchmarks/gc/mutation_ownership.py` compare field replacement, mutable element
extraction, mutable reference slices and read controls. Build the same driver
against each source snapshot using **separate Cargo target directories**, freeze
the binaries and source manifests, then run:

```sh
python3 benchmarks/gc/mutation_ownership.py \
  --before /absolute/path/to/before --after /absolute/path/to/after \
  --output target/mutation-ownership-screen --cpu 0 --repeats 7 --plot
```

For two WASI modules, add `--wasmtime /absolute/path/to/wasmtime`. Run one screen
at a time with no concurrent builds, tests or profilers. Matplotlib is required
for `--plot`. The runner preserves a randomized paired schedule, raw results,
input hashes, tables and SVG/PNG graphs, and rejects identical before/after binaries.
Timings are aggregate operation costs, excluding setup, collection and startup;
they cannot qualify pauses, capacity, whole-JVM performance or equal-RSS behavior.
The fixture retains the source while measuring the old extraction path, whose
missing ownership guarantee is tested separately. See the
[ownership results](IMPLEMENTATION.md#mutable-array-extraction-retains-owning-roots)
for correctness costs, controls and rejected measurement attempts.



The mechanism runner fixes `argv[0]` to `mechanisms` for both native and WASI
processes and records the selected executable separately. `--policies` can narrow
the predeclared policy matrix; the default matrix is unchanged. This controls the
invocation-name allocation confound, not all heap/code layout sensitivity. Do not
pool historical path-dependent samples with these normalized runs. Build manifests
from `mark_cycles_build.py` accept `--features` to record explicit experimental Cargo
features. The owning identity-index prototype requires
`--features ristretto_gc/experimental-identity-index`; it is not enabled by default
and its measured regressions do not satisfy an overhead or release gate.


### Certificate model checker

Stock Loom 0.7.2 has a reported RMW/store atomicity defect affecting one concurrent
certificate regression. Run the exact production state and all six models, including
the documented stock-checker skip, with the isolated recorded upstream repair:

```sh
python3 benchmarks/gc/trace_certificate_check.py --output target/certificate-model-check
```

The output directory must be new. The runner copies the installed Loom 0.7.2 source,
applies the checked-in patch from unmerged upstream PR #391, preserves workspace
dependency versions and collector atomics, and records source hashes and logs.
This is a scoped model check with explicit root-barrier assumptions, not a full
collector or performance qualification. See the
[rooted-projection results](IMPLEMENTATION.md#rooted-object-array-projections-permit-unrelated-collection).

## Reproduce the current tagged catalog models

```sh
python3 benchmarks/gc/tagged_catalog_model_check.py --output target/tagged-catalog-current
```

Use an unused output directory. The runner requires offline Rust dependencies and
Loom 0.7.2 source (`--loom-source` can select its installed directory). It applies
the recorded RMW correction only to its isolated checker copy, then runs 59 catalog
and six pool models plus 32 fault controls. All mutation anchors are checked before
building. `--preflight-only` explicitly skips execution and cannot qualify evidence.
Per-variant source/build roots, source/checker hashes, logs and actual binaries are
recorded. Strict results reject incomplete suites and failures that never ran the
intended fixture. These models exclude real pointer protection, concrete payloads,
facade capacities, callbacks and full-GC performance/progress qualification.
See [the runner report](../../target/gc-implementation/canonical-checker-REPORT.md).

## Concrete leaf lifecycle screen

The feature-gated `leaf_lifecycle` example takes exactly MODE OPERATIONS BATCH.
MODE is `legacy`, `canonical-explicit` or `canonical-auto`; operations are 1..10000000,
batch capacity 1..16384. `leaf_lifecycle_counts` has identical arguments but uses a
separate counting allocator. It performs one complete warm invocation then three
recorded heap lifetimes, including setup, teardown and report formatting. Its timing
values are discarded. Never run counting during performance measurements.

```sh
cargo build --release -p ristretto_gc --features experimental-canonical-leaves   --example leaf_lifecycle --example leaf_lifecycle_counts
cargo build --release -p ristretto_gc --features experimental-canonical-leaves   --example leaf_lifecycle --example leaf_lifecycle_counts --target wasm32-wasip2
```

`leaf_lifecycle.py run` requires `--binary`, `--provenance`, `--sources`, `--output`,
`--kind timing|counts` and `--cpu`. Build provenance must contain matching
`binary_sha256` and `source_archive_sha256`. `--wasmtime` is required for WASI and
must be omitted for native. Use a fresh output directory; the runner copies and hashes
inputs and fixes argv0. Timing defaults to seven process blocks, randomized modes,
batches 64/1024/16384; pass `--operations 1000000` to reproduce the recorded screen.
The measured policy, toolchains and build records must accompany a new binary; hashes
alone do not establish that an arbitrary executable was built from a claimed source.
Generate reports/CSV and optional standalone SVG/PNG only after measurements finish:

```sh
python3 benchmarks/gc/leaf_lifecycle.py report --output target/leaf-screen --plot
```

Plotting requires the repository's Matplotlib environment. Lifecycle includes actual
batch reclamation and validation; setup/teardown are separate. The workload has one
mutator, no background workers and reference-free 32-byte values. It is not an
operation-latency, pause, contention or JVM comparison. The first recorded screen
shows substantial canonical regressions despite fewer allocator calls; see
[the evidence and graphs](../../target/gc-implementation/canonical-leaf-cost-REPORT.md).

The first bounded root-scan optimization is compared against those frozen binaries
in a separate 252-process paired experiment. Its standalone runner, declared policy,
both source/binary revisions, raw rows and graphs are retained with the
[batching evidence](../../target/gc-implementation/canonical-cleanup-REPORT.md).
Canonical lifecycle time improves 18–21% with unchanged allocation activity, while
remaining slower than legacy. The original single-slot scan API remains available.
