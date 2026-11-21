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
