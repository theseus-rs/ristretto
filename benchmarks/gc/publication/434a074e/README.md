# Strong-graph publication at `434a074e`

## Whole-JVM allocation comparison

Every runtime ran the same Java classfiles with a 64 MiB heap on two CPU affinity slots. Each cell is a median of three fresh processes with one second of warmup and three seconds of measurement. HotSpot used the default JIT in Corretto 25.0.5. Ristretto's JIT lane permits interpreter fallback. Wasmtime 50.0.1 ran the WASI interpreter.

| Runtime | Saturation, operations/s | Scheduled arrival p99.9 at 1,000/s, µs | Scheduled arrival p99.99 at 1,000/s, µs |
| --- | ---: | ---: | ---: |
| Ristretto native interpreter | 86,247 | 31 | 230 |
| Ristretto native with JIT enabled | 81,283 | 207 | 545 |
| Ristretto WASI interpreter | 62,943 | 58 | 328 |
| HotSpot G1 | 18,466,491 | 2 | 97 |
| HotSpot ZGC | 22,995,484 | 9 | 175 |

All fixed-rate byte-array allocation processes finished 3,000 requests each. Each process observed collection during its lifetime. The short measurement window and different execution tiers limit attribution to the collector. The fixed-rate sample places p99.99 in the highest occupied histogram bucket.

## Rotating rooted graphs

Each operation constructs a linked chain of 16 fresh nodes, replaces one of
2,048 roots, and traverses the chain to check its contents. The driver also
audits surviving chains after the timing window. Both workloads used the same
frozen binaries and Java classfiles in separate randomized schedules.

| Runtime | Saturation, operations/s | Completed of 3,000 at 1,000/s | Scheduled arrival p99.9, µs |
| --- | ---: | ---: | ---: |
| Ristretto native interpreter | 6,988 | 3,000 | 168 |
| Ristretto native with JIT enabled | 6,725 | 3,000 | 395 |
| Ristretto WASI interpreter | 7,101 | 3,000 | 48,128 |
| HotSpot G1 | 12,979,182 | 3,000 | 20 |
| HotSpot ZGC | 13,349,086 | 3,000 | 24 |

[All 60 process results](processes.csv) · [native throughput](native-throughput.svg) · [native latency](native-latency.svg) · [WASI throughput](wasm-throughput.svg) · [WASI latency](wasm-latency.svg)

## Allocation during marking

Each process allocated 65,536 rooted scalars or one-edge graphs in 128 batches during strong marking. The fixtures checked the child, checksum, promotion count where applicable, and zero managed bytes after teardown. Sixteen process pairs per shape and collection used randomized, balanced ordering on the same pinned CPU. Positive values are additional elapsed time for the new collector relative to `8d056d58`. Separate host builds and tests ran concurrently with portions of this exploratory screen.

| Target | Collection | Shape | Allocation plus remaining collection | Paired 95% bootstrap interval |
| --- | --- | --- | ---: | ---: |
| Native | full | scalar | +0.1% | -2.8% to +3.2% |
| Native | full | graph | +13.7% | +11.7% to +15.1% |
| Native | young | scalar | +0.9% | -0.3% to +2.2% |
| Native | young | graph | +12.0% | +10.3% to +14.3% |
| WASI | full | scalar | +1.2% | -1.4% to +2.7% |
| WASI | full | graph | +2.0% | +0.6% to +3.6% |
| WASI | young | scalar | +1.1% | +0.7% to +3.0% |
| WASI | young | graph | +4.5% | +0.7% to +6.0% |

The initial linked-queue implementation took 91.6% longer on native in a separate 12-pair full-collection screen. Reusable slots and a descriptor-only first ticket reduced this regression. The graph publisher can stay parked while its peers reclaim space.

[Paired raw times](marking-births.csv) · [native full collection](native-marking-births.svg) · [WASI full collection](wasm-marking-births.svg) · [collector fixture](../../../../ristretto_gc/examples/marking_births.rs)

## One-minute fixed-rate trials

The collector driver offered 10,000 writes per second with two workers, a 64 MiB compacting heap, two CPUs of quota and a 512 MiB process cap. The JVM array-element setter completed 600,000 requests in each of three runs. Operation p99.9 was 39, 103 and 89 µs respectively. The middle run exceeded the strict 100 µs threshold. The unrestricted borrow of the same 2,048-slot owner completed 22,065, 24,304 and 33,675 requests. All six runs ended with zero live objects and zero occupied bytes. The per-process table retains failed runs, both service and scheduled-arrival tails, and backlog through the offered and completed counts.

[Six endurance trials](endurance.csv) · [collector qualification driver](../../../../ristretto_gc/validation/measure.py)

## Reproduction

For the paired collector measurement, run `cargo build --release --locked -p ristretto_gc --example marking_births` at `8d056d58` and `434a074e`. Add `--target wasm32-wasip2` for WASI. Use separate Cargo target directories for both revisions. Record each binary and source tree with `benchmarks/gc/mark_cycles_build.py --fixture marking_births`. Run `benchmarks/gc/marking_births.py --pairs 16 --seed 52189 --collection full` or `--collection young`, passing the two manifests with `--before` and `--after`, a fresh `--output`, and `--wasmtime` for WASI. The native full and young samples used logical CPUs 12 and 20 respectively. WASI used CPUs 14 and 22. [Endurance build and run instructions](../../README.md).

At the measured revision, build both examples and run from the repository root.
Set `JDK` to the JDK directory and `WASMTIME` to the Wasmtime executable.
Select two available physical cores on the measurement host. The recorded
screen used logical CPUs 8 and 10. Use a fresh results directory per run. For the rooted graph screen, repeat the command with `--workloads roots` and a separate output directory.

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
