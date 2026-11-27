# Whole-JVM allocation comparison at `6fd06e30`

Each operation allocates a 256-byte Java array, writes both ends and publishes the
array. All five runtimes executed the same classfiles. The two native HotSpot
lanes used OpenJDK 25.0.5 with its default JIT. Ristretto's JIT-enabled lane
allows interpreter fallback. Wasmtime 50.0.1 ran the WASI interpreter.

| Runtime | Saturation, ops/s | Scheduled p99.9 at 1,000/s, µs | Scheduled p99.99 at 1,000/s, µs |
| --- | ---: | ---: | ---: |
| Ristretto native interpreter | 93,777 | 83 | 119 |
| Ristretto native with JIT enabled | 85,264 | 31 | 37 |
| Ristretto WASI interpreter | 69,938 | 36 | 171 |
| HotSpot G1 | 21,312,476 | 3 | 9 |
| HotSpot ZGC | 25,502,067 | 153 | 220 |

Cells are medians across three fresh processes with a 64 MiB logical heap, two
CPU affinity slots, one second of warmup and three seconds of measurement.
Each fixed-rate process completed all 3,000 requests. That sample puts p99.99
in the highest occupied histogram bucket. Elapsed request time includes
scheduled-arrival delay. Bootstrap and application execution contribute to
whole-JVM costs. Each process supplied evidence of collection somewhere in its
lifetime. This short screen leaves measured-window GC exposure, sustainable
capacity, physical-memory parity and extreme-tail qualification open.

[All 30 process measurements](processes.csv) ·
[native throughput](native-throughput.svg) · [native p99.9](native-latency.svg) ·
[WASI throughput](wasm-throughput.svg) · [WASI p99.9](wasm-latency.svg)

## Collector measurements

Twenty-four randomized process pairs compared the new collector with
`69b6234e` on the same host. With diagnostics disabled, default-policy rooted
allocation and read time fell 1.8% on native (95% bootstrap interval from 1.3%
to 2.5% faster) and 2.2% on WASI (0.3% to 3.2% faster). For counted publication,
the allocation-time intervals included zero change on both targets. WASI
handle lookup took 1.1% longer under the default policy and 1.3% longer under
counted publication. During young collection on WASI, scalar allocation plus
remaining collection time increased 2.8% (1.5% to 3.5%). For the single-edge
graph that interval included zero on both targets in full and young collections.

Three one-minute native trials per array-write mode used two GC workers, a
64 MiB compacting heap, two CPUs of quota and a 512 MiB process limit. At
10,000 offered operations/s the JVM's array-element setter completed all
600,000 requests in every trial. Its operation p99.9 ranged from 44 to 50 µs
and p99.99 from 556 to 625 µs. The unrestricted mutable borrow of the same
2,048-slot owner completed 34,046 to 36,714 requests per trial. Every trial
in both modes ended with zero live objects and zero occupied bytes. The CSV
also preserves the preceding checkpoint's three element-setter runs.

[Per-process endurance results](endurance.csv) ·
[collector fixtures and endurance commands](../../README.md)

## Reproduce the screen

At the measured revision, build both examples and run from the repository root.
Set `JDK` to the JDK directory and `WASMTIME` to the Wasmtime executable.
Select two available physical cores on the measurement host. The recorded
screen used logical CPUs 8 and 10. Use a fresh results directory per run.

```sh
cargo build --locked --release -p ristretto_vm --example gc_compare
cargo build --locked --release --target wasm32-wasip2 -p ristretto_vm \
  --no-default-features --example gc_compare
python3 -B ristretto_gc/validation/provenance.py \
  --binary target/release/examples/gc_compare target/wasm32-wasip2/release/examples/gc_compare.wasm \
  --output target/gc-publication-frozen
python3 -B benchmarks/gc/run.py --jdk "$JDK" --wasmtime "$WASMTIME" \
  --ristretto target/gc-publication-frozen/gc_compare \
  --wasm target/gc-publication-frozen/gc_compare.wasm \
  --runtimes ristretto-candidate ristretto-candidate-jit wasm-candidate g1 zgc \
  --workloads allocation --rates 0 1000 --workers 0 --cpus 8 10 --heap-mib 64 \
  --seconds 3 --warmup 1 --repeats 3 --seed 251125 --timeout 120 \
  --output target/gc-publication-comparison
python3 -B benchmarks/gc/plot.py target/gc-publication-comparison
```
