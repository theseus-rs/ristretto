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
Both execute the same Java 17 classfiles on the same CPUs with a 64 MiB heap.
The driver compiles the sources once per run, records the workload classfile hashes
in the JSON, and checks each result against an independent integer model.
The JSON records raw workload-call times and wall time for each process,
including startup. The JVM controls when it collects. These finite samples do
not measure sustainable throughput or GC pause percentiles.

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
