# Finite HotSpot ring baseline

Use a HotSpot JDK with G1 and ZGC on Linux. Put its `java` and `javac` on PATH.
Run the driver in a cgroup v2 with enforced CPU and memory limits. For example,

```sh
python3 benchmarks/gc/ring/verify_oracle.py ristretto_vm/tests/fixtures/canonical/ring-oracle.tsv
systemd-run --user --scope -p MemoryMax=1G -p CPUQuota=200% \
  python3 benchmarks/gc/ring/hotspot.py baseline.json
```

Each fork launches a fresh JVM for each collector and alternates the launch order.
Both execute the same checked Java 17 classfiles on the same CPUs with a 64 MiB
heap. The driver checks each result against an independent integer model.
The JSON records raw workload-call times and wall time for each process,
including startup. The JVM controls when it collects. These finite samples do
not measure sustainable throughput or GC pause percentiles.

Generate a per-process table and an SVG from a saved capture using Python with
Matplotlib installed:

```sh
python3 benchmarks/gc/ring/report.py baseline.json forks.csv forks.svg
```

Each point is the median workload-call time in one fresh JVM. The CSV uses the
zero-based fork index stored in the capture. The plot labels forks starting at
one. See `processes[].samples` in the input JSON for the individual observations.

The driver records the elapsed interval of the timed batch in `batch_ns`. It
reads the JVM’s collection count and collection time before and after that
batch, then writes the deltas by collector bean in `collection_changes`.
An unavailable counter is JSON null. ZGC cycle and pause beans can describe
the same collection. Keep the bean names when reporting their values.
