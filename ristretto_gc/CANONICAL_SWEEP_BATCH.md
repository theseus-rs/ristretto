# Bounded inventory cleanup and helped allocation publication

This internal experimental policy scans at most eight consecutive root, edge or
object inventory positions per helper visit. It stops at the first occupied
position or failed observation and submits at most one copied request. Pending
requests and physical releases still take their existing individual steps. No
additional owning storage or quota is introduced.

Only confirmed empty positions advance inside a batch. The final observed
position advances through the original terminal lane path. Object sweep credit
counts the skipped positions and then the final completed position separately;
a failed discovery or cancelled/failed command clears both accumulated visits and
certified sweeps. A batch never observes the same position twice when capacity is
less than eight. A complete consecutive N-position sweep, bounding stable phase
observations at retirement floor 5 in the same active epoch, and epoch-bound Finish
remain required. Logical root enumeration remains a separate controller operation.

The first batching candidate exposed deterministic starvation in the cooperative
three-mutator/two-helper minimum-capacity fixture: phase changes repeatedly
superseded one allocation's private publication preparation. Increasing the
watchdog was not accepted. The revised policy uses the prepared external root as
discoverable publication work. After giving durable removal intent priority, a
helper can submit coupled Publish for a Prepared external root. The controller
still requires a Prepared target and exact root status. A root prepared for an
already-tracing child fails that precondition, so this policy cannot bypass the
child-edge activation protocol. Cancellation and publication share the existing
canonical decision/effect ordering.

An allocation whose private proposal was superseded may find its own retained
root already Live. Recovery requires admission to that exact record and a Live
observation between stable controller snapshots with equal, nonwrapping revisions.
Failed acquisitions or a changed revision remain Pending. A foreign or closed
record cannot yield a result. Recovery clears only the private cell owner and
returns the existing root exactly once; the root permit and catalog charge follow
that same token. Discarding a helped allocation posts normal durable root removal.
No key lookup substitutes a different root or target.

A second, native contention failure exposed a partially applied publication:
an owner could retain an older stable controller snapshot while a helper changed
the target membership and then stopped before changing the root. Publication
preflight then reported Conflict although that exact root was still Prepared.
The final allocation request retries such conflicts while its retained root is
Prepared or Live. It still rejects a Retired root and does not return a result
before admitted stable completion. The deterministic native fixture stops the
helper at this exact boundary, checks target/root/certificate state, resumes it,
and verifies one transfer of the original root. Reverting this retry reproduces
the original failure. The native schedule is not represented as a threadless-Wasm
interleaving; the portable guards and cooperative workloads are tested separately.

The stable revision check establishes completed publication rather than relying
on a root effect that may precede its record certificate. Each intermediate
snapshot is dropped before the next acquisition. Request storage and catalog
reservations remain the existing ones. Record admission and arbitrary payload
Drop/host allocation can still execute unbounded caller code. Eight observations
bound work count, not elapsed time or a global pause.

The final source passes native/WASI regressions, the affected memory checks and
17 intended fault processes. Reused transition models cover unchanged raw
controller inputs; the bounded retry has a separate deterministic native fixture.
Finite contention and paired cost results, including regressions, are recorded in
[the report](../target/gc-implementation/canonical-sweep-REPORT.md). The scan-only development
failure, model-development failures/timeouts and intended fault variants are
preserved separately. This stage does not qualify a public collector selection,
VM payload/caller integration, arbitrary callbacks, compaction, JIT, G1/ZGC
performance, host-turn latency or sustained/endurance load.
