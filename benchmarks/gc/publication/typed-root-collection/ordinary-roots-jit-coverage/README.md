# JIT coverage of the scheduled Java roots fixture

In a separate diagnostic build at `e2d0a176`, the JIT-enabled process interpreted all 35,891 entries into `GcBench.operation`. The compiler rejected the whole method at bytecode offset 27, opcode `0xb3` (`putstatic`). This instruction belongs to the control branch. The roots branch also contains reference creation, constructor calls and array operations. Both the histogram recorder and node constructor were rejected as instance methods.

The two fresh processes used a 5-second warmup, a 4-second closed-loop measurement, a 64 MiB heap, two CPUs, two GC workers and a 512 MiB cgroup. The diagnostic profile collects lifetime invocation counts, including warmup and reporting. Timings from these instrumented processes are excluded from the Java latency comparison.

[Per-method counters, rejection offsets, configuration and build hashes](required-methods.json) · [Uninstrumented Java comparison](../consolidated-java-followups/README.md)
