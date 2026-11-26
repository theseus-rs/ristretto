# Finite HotSpot ring baseline

Use a HotSpot JDK with G1 and ZGC on Linux. Put its `java` and `javac` on PATH.
The Bash driver uses jq, GNU coreutils, taskset, and Perl's Time::HiRes.
Run the driver in a cgroup v2 with enforced CPU and memory limits. For example,

```sh
benchmarks/gc/ring/verify_oracle.sh ristretto_vm/tests/fixtures/canonical/ring-oracle.tsv
systemd-run --user --scope -p MemoryMax=1G -p CPUQuota=200% \
  benchmarks/gc/ring/hotspot.sh baseline.json
```

Each fork launches a fresh JVM for each collector and alternates the launch order.
G1 runs first by default. Pass `--first-collector ZGC` to start with ZGC.
Both execute the same Java 17 classfiles on the same CPUs with a 64 MiB heap.
The driver compiles the sources once per run, records the workload classfile hashes
in the JSON, and checks each result against an independent integer model.
The JSON records raw workload-call times and wall time for each process,
including startup. The JVM controls when it collects. These finite samples do
not measure sustainable throughput or GC pause percentiles.

Each process records the elapsed time for the timed batch in `batch_ns`.
Its `collection_changes` object gives the change in collection count and
collection time in milliseconds for each named JVM collector bean. An
unavailable counter is JSON null. ZGC cycle and pause beans can describe the
same collection, so keep their names when reporting the values.

Generate a per-process table and an SVG from a saved capture with Python 3.8
or later. The report uses the Python standard library and verifies the workload
and sample checksums offline.

```sh
python3 benchmarks/gc/ring/report.py baseline.json forks.csv forks.svg
```

The report includes every collector in the capture, in alphabetical order.
Each fork must have one process per collector and all processes must have the
same number of samples. The CSV uses the capture's zero-based fork indices and
records `median_call_us`, `p25_call_us`, and `p75_call_us` in microseconds.
Percentiles use linear interpolation between sorted samples. With one sample,
all three values equal that sample.

The chart labels forks starting at one. Within each fork, the markers are
offset horizontally by collector. Each marker shows a process median and its
vertical bar spans the 25th to 75th percentiles. The individual measurements
are in `processes[].samples`.

To regenerate existing outputs, pass `-f` or `--overwrite`.

To include Ristretto on the same two CPUs and in the same 1 GiB cgroup, build
the ring example on the Git revision being measured. The example embeds both
Java classes. Record the four absolute executable paths for the capture.

```sh
export RISTRETTO_GC_BENCH_REVISION=$(git rev-parse HEAD)
cargo build --locked --release -p ristretto_vm_research --no-default-features --features experimental-canonical-jit --example canonical_ring_samples
cargo build --locked --release -p ristretto_vm_research --no-default-features --features experimental-canonical-runtime --target wasm32-wasip2 --example canonical_ring_samples
systemd-run --user --scope -p MemoryMax=1G -p CPUQuota=200% \
  python3 benchmarks/gc/ring/compare.py comparison.json \
  --native target/release/examples/canonical_ring_samples \
  --compiled target/release/examples/canonical_ring_samples \
  --wasi target/wasm32-wasip2/release/examples/canonical_ring_samples.wasm \
  --wasmtime "$(command -v wasmtime)" --revision "$RISTRETTO_GC_BENCH_REVISION"
python3 benchmarks/gc/ring/report.py comparison.json comparison.csv comparison.svg
```

The capture records elapsed time for each invocation, including runtime and
collector work during that invocation. Each process reuses one heap. The
workloads run closed-loop, with the next call starting when the previous one
finishes. Process wall times also include startup and teardown. Both HotSpot
collectors use a 64 MiB Java heap. Ristretto limits requested payload to
64 MiB and separately bounds object slots, root slots and reference edges.
The cgroup limit covers the whole process. The SVG uses a log scale for
cross-runtime measurements. Use the CSV for exact per-process medians and the
JSON for raw samples, executable hashes, bytecode hashes and runtime settings.

New captures record the complete interval from the start of the first timed
call to completion of the batch. Both drivers buffer their measurements and
print after that interval. The second panel and the last CSV column divide
completed calls by the measured batch duration. This finite closed-loop rate
excludes process startup, warmup and final heap teardown. Scheduled-load
capacity and tail latency require longer independent runs.

For each HotSpot process, `collection_changes` contains the change in each
collector bean’s count and reported collection time around the measured batch.
A missing counter is stored as JSON null. Concurrent-cycle and pause beans can
describe the same collection. For Ristretto, each timed invocation records its
allocations, helper polls and released objects. Final teardown runs after the
batch. Each call time includes VM execution as well as any collector work.

[Measured results, graphs and raw samples for `449a2ee81c5d`](results/449a2ee81c5d/README.md). The archived runs used CPU affinity 8 and 10, three fresh processes per execution tier and eight warmup calls. The small ring used 17 steps and 12 timed calls. The large ring used 257 steps and 48 timed calls.
