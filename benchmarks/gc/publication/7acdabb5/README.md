# Stalled registration, allocation cost and whole-JVM performance

A constructor parked inside physical registration can now share the only metadata
shard with a peer that reclaims garbage and completes a capped allocation. This
screen measures the cost of making the address reservation independent. The
candidate is integrated revision `7acdabb58b46a7823e052196485bfb7fb3f3f14a`.
The collector code in [PR #1158](https://github.com/theseus-rs/ristretto/pull/1158)
has the same source tree under `ristretto_gc/src`. The integrated JVM includes
additional interpreter, managed-field and barrier work beyond the review stack.

## Whole-JVM comparison

| Runtime | Byte arrays ops/s | Byte arrays p99.9, µs | Rooted graphs ops/s | Rooted graphs p99.9, µs |
| --- | ---: | ---: | ---: | ---: |
| Native interpreter | 70,348 | 268 | 4,117 | 549 |
| Native, JIT enabled | 67,541 | 58 | 4,749 | 233 |
| WASI interpreter | 62,590 | 50 | 6,290 | 89,088 |
| HotSpot G1 | 17,275,202 | 2 | 8,068,877 | 12 |
| HotSpot ZGC | 23,316,936 | 7 | 11,823,680 | 13 |

Throughput is the median of three fresh processes running without a rate limit.
Latency is the median of their three separate, fixed-rate counterparts' scheduled-arrival
p99.9 at 1,000 operations/s. Every fork observed collection during its lifetime.
The graph workload constructs and checks 16 nodes per operation and rotates
2,048 roots. One WASI graph fork completed 2,900 of 3,000 scheduled operations.
Every other fixed-rate fork completed all 3,000. The three-second windows give
little precision at p99.99. Execution tier, class library and allocation code
all contribute to the observed JVM differences.

All lanes ran the same Java 8 classfiles with a 64 MiB heap, CPU affinity 8
and 10, one second of warmup and three seconds of measurement per process.
HotSpot used Corretto 25.0.5 with its default execution tier. Ristretto's native
JIT can fall back to interpretation. Wasmtime 50.0.1 ran the WASI interpreter.
The randomized schedule used seed 251125. Other builds ran on the host during
this exploratory screen. Release qualification requires a sustained run.

[Throughput graph](throughput.svg) · [Latency graph](latency.svg) ·
[All 60 process results, including unfinished work](processes.csv)

## Collector allocation during marking

The fixture allocates 65,536 scalars or one-edge graphs during strong marking.
Time covers allocation and the remaining collection. Each native/WASI, young/full
and scalar/graph cell uses 16 balanced randomized pairs, seed 52189. Each pair
runs the baseline and candidate in fresh processes on the same pinned CPU.
The ratio is the median of the paired elapsed-time ratios. Intervals use 10,000
bootstrap resamples of process pairs. The fixture checks the checksum, child
reachability, expected promotions and zero managed bytes at teardown.

In the narrow-page scalar profile, 64,344 of 65,537 reservations created
a directory page. Grouping 4,096 addresses in a page and allocating descriptor
chunks on demand reduced that cost. On native full collection, narrow and wide
scalar timings were 2.542× and 1.265× the common baseline. One-edge graphs
measured 1.919× and 1.192×. The wide WASI timings were 0.983× for scalars and
1.026× for graphs. The two candidates ran in separate paired experiments.
The narrow candidate also predates the sweep-budget correction.

[Collector graph with 95% intervals](collector.svg) · [Paired times](marking-births.csv) ·
[Interval estimates](marking-ratios.csv) · [Binary and source digests](provenance.json)

## One-minute write trials

Two asynchronous workers received 600,000 requests in each trial with a
64 MiB compacting heap, two CPUs of quota and a 512 MiB process limit.
Each single-element setter trial completed all requests. Its service p99.9
was 50, 84 and 91 µs, below the 100 µs threshold. Its scheduled-arrival
p99.9 was 900, 3,072 and 2,688 µs. The unrestricted mutation of the same
2,048-slot owner completed 25,730, 27,033 and 28,737 requests. All six
trials ended with zero occupied bytes and zero live objects. The generic
write path and longer endurance runs remain release gates.

[Six trial results, including the failed executions](endurance.csv)

## Reproduction

To regenerate the three figures from the portable CSV files, install Matplotlib
and run `python3 benchmarks/gc/publication/7acdabb5/plot.py`. The VM figures
show each process and the median. The collector figure shows paired estimates.

The Rust fixture is available on the public review stack. For a single native
observation run the following command on commit `bc5015f6`. Use `graph` and
`young` for the other shapes and collection types. Build with
`--target wasm32-wasip2` and run the example with Wasmtime for the WASI lane.

```sh
cargo run --locked --release -p ristretto_gc --example marking_births -- scalar full
```

To repeat the paired collector screen, use independent source and Cargo target
directories for each version. The measured baseline, narrow and wide integrated
revisions were `7f18a4eb`, `38c38d9d` and `7acdabb5`. For a public-source
comparison, use `6c629ef8` and `bc5015f6`. In the baseline worktree apply
the candidate's `ristretto_gc/Cargo.toml` and `Cargo.lock` so both builds use
the same dependencies. The public baseline and candidate collector source trees
match their integrated counterparts. On the integrated checkout, record the
release executables with `benchmarks/gc/mark_cycles_build.py --fixture marking_births`
and pass those manifests to `benchmarks/gc/marking_births.py` using `--before`,
`--after`, `--pairs 16`, `--collection full` or `young`, and a fresh `--output`.
Add `--wasmtime` for WASI. Native full/young ran on CPUs 12/20 and WASI on 14/22.

The shared Java fixture and telemetry boundary can also run directly on HotSpot.
Set `JDK` to Corretto 25.0.5 and run from the repository root. Use `UseZGC`
for the ZGC lane, `roots` for the graph workload and `1000` in place of the
zero for the fixed-rate screen. Each invocation is one process.

```sh
classes=$(mktemp -d)
"$JDK/bin/javac" --release 8 -Xlint:-options -d "$classes" benchmarks/gc/GcBench.java benchmarks/gc/GcTelemetry.java
taskset -c 8,10 "$JDK/bin/java" -XX:+UseG1GC -Xms64m -Xmx64m -XX:ActiveProcessorCount=2 \
  "-Xlog:gc*,safepoint:file=$classes/gc.log:uptime,level,tags" \
  -cp "$classes" GcBench allocation 3000 1000 0 251125
```

For the integrated five-runtime screen, build `ristretto_vm --example gc_compare`
with `cargo build --release --locked` on native and with `--no-default-features
--target wasm32-wasip2` on WASI. Freeze both executables using
`ristretto_gc/validation/provenance.py`. Run the following driver against the
frozen files, setting `NATIVE`, `WASM`, `WASMTIME`, `JDK` and `OUT` to their
paths. The full integrated source archive and its digest are retained with the
original run. This full runtime extends the publicly staged collector PRs.

```sh
python3 -B benchmarks/gc/run.py --jdk "$JDK" --wasmtime "$WASMTIME" \
  --ristretto "$NATIVE" --wasm "$WASM" \
  --runtimes ristretto-candidate ristretto-candidate-jit wasm-candidate g1 zgc \
  --workloads allocation roots --rates 0 1000 --workers 0 --cpus 8 10 --heap-mib 64 \
  --seconds 3 --warmup 1 --repeats 3 --seed 251125 --timeout 120 --output "$OUT"
```
