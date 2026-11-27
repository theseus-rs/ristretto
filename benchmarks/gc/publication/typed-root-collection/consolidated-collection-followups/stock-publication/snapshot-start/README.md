# Publication during snapshot preparation

The counted-policy experiment at `6a165bbc` starts a prepared epoch when a
mutator has completed root publication before the final phase observation.
A public root and its leaf survived the first ticket. A later full request
kept its own cleanup group, and removing the root made the leaf collectible.
Holding admission through activation still forced rollback. Both the stale
start and a late precise full request racing an old-only claim reproduced
on the parent. The fresh observation applies only before enumeration.

The candidate is withheld. Native median capacity was **94.1%** of its
immediate control, under the declared 95% floor. All four two-minute native
processes completed 60,000 arrivals, with median response p99.9 moving from
2,800 to 2,736 µs. All four WASIp2 processes also completed the offered work.
WASIp2 capacity was 102.9% of control and sustained median p99.9 was
98.8% of control. These are short release screens with three pairs at
capacity and two pairs at sustained load. They do not establish endurance.
[Paired Java measurements and gate policy](qualification.json) ·
[32 fresh Java processes](jvm.jsonl.gz) · [Candidate patch](snapshot-start.patch.gz)

All 36 complete-collection comparisons against `f1daebeb` passed the 10×
threshold. The minimum individual lower 95% bounds were 10.312× native and
10.159× WASIp2. The isolated collection timings do not pay for a whole-JVM
capacity loss. [Cycle results](cycles/complete-cycles.csv) ·
[Speedup graph](cycles/speedup.png) · [Build and run provenance](cycles/provenance.json)

A separate experiment at `129d6efc` temporarily borrowed a real reservation
when checking array headroom. Ten of twelve native JVM probes still returned
Java OOM. Incrementally cleared Java arrays cannot directly grant a whole
logical reservation, and a single callback turn can end before physical
release. The alpha keeps its advisory semantics.
[Probe outcomes](advisory-probe.json) · [Experiment patch](advisory-loan.patch.gz)
