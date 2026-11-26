# Timed Java roots execution

The ordinary native compiler now accepts verified static methods that use null constants, reference locals and null branches with primitive parameters and a primitive or void return. The remaining eligible instructions provide no way to acquire a non-null reference. These methods use the existing frame handoff at backward branches. The VM regression compares interpreter and compiled returns, exercises short, indexed and wide locals, and counts three actual compiled backward branches on a finite loop.

The ordinary Java roots benchmark still requires object allocation. The explicit measurement interval moved its first rejection from `aconst_null` at byte 38 to `new` at byte 51. No measured roots operation executed compiled code in either revision.

| Native revision | Policy | Timed roots operations | Compiled operation entries | Compiled operation backedges | First rejection |
|---|---|---:|---:|---:|---|
| `dd877809` | Interpreter | 493 | 0 | 0 | 38, `aconst_null` |
| `dd877809` | JIT enabled | 446 | 0 | 0 | 38, `aconst_null` |
| `9ad56496` | Interpreter | 407 | 0 | 0 | 51, `new` |
| `9ad56496` | JIT enabled | 373 | 0 | 0 | 51, `new` |

Each completed operation constructs 16 nodes and visits all 16. In the frozen Java 8 classfile, the construction loop takes its backward branch at byte 70 and the traversal at byte 125. The checker requires 32 taken compiled backedges per compiled roots invocation, including in mixed-dispatch windows. It reconciles observed calls with two histogram entries and 16 constructor entries per completion. Separate positive controls on both revisions entered the compiled kernel exactly seven times and took 35 backward branches during the measured window. Each excluded the outside-only method.

These functional diagnostic forks used Corretto 25, a 500 ms warmup and a 250 ms roots measurement. The three diagnostic processes for a revision ran concurrently. Their completion counts identify precisely what executed inside the Java boundary. Use the [isolated scheduled comparison](../reference-cards/README.md) for throughput and tail latency.

To reproduce, build each revision with `build_execution_measurement.py`. Run the ordinary benchmark with `--fixture benchmark --workload roots --warmup-ms 500 --duration-ms 250` under both `--policy candidate` and `--policy candidate-jit`. Run the positive control with `--fixture probe --policy candidate-jit --require-compiled`. Use the checker at `8bc336b7` or later to replay the timed backedge and rejection checks on either producer. [Build and replay instructions](../../../../EXECUTION_MEASUREMENT.md).

[All required method counts](timed-methods.csv), [six process outputs](diagnostic-forks.jsonl.gz), [source, launch, binary and JDK receipts](provenance.json).
