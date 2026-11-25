# Whole-VM throughput with a cached finalizer lookup

This experiment caches whether an instance of an already linked class has a Java finalizer override. The JIT-enabled roots workload gained 2.37% in the median paired comparison. The other three per-condition intervals include parity. Each estimate uses twelve randomized pairs of fresh native VM processes and 10,000 paired bootstrap resamples. Intervals are unadjusted across the four comparisons.

| Workload | Runtime | Before ops/s | After ops/s | Paired speedup | 95% interval |
|---|---|---:|---:|---:|---:|
| allocation | Interpreter | 96,652 | 99,819 | 1.000× | 0.984–1.069× |
| allocation | JIT enabled | 92,072 | 93,882 | 1.000× | 0.929–1.049× |
| roots | Interpreter | 7,434 | 7,669 | 1.041× | 0.955–1.176× |
| roots | JIT enabled | 6,724 | 6,881 | 1.024× | 1.007–1.055× |

Each fork ran two seconds of warmup and four seconds at saturation, on CPUs 4 and 6, with a two-CPU quota, 512 MiB process memory, swap disabled and a 64 MiB managed heap. Both arms ran the same Java workload. The source comparison found three changed Rust files and a release-plan Markdown file present only in the control archive. Compiled code still preserves finalization registration for inherited or declared overrides. An independent VM regression test checked finalization ordering and resurrection.

Full collection speedup, global pauses, and steady-state superiority over G1 and ZGC have separate gates. [Process measurements](processes.csv) · [Paired estimates](summary.json) · [Frozen input hashes](provenance.json) · [Experimental source difference](linked-class.patch)
