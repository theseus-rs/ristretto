# One-slot decisions for phase-only work

The experimental canonical catalog prepares an effect-free phase change with one
controller slot. Operations that change object membership, roots, conditional
references or edges now use two slots. The first stores both immutable phase
outcomes. The second holds the effects and the irrevocable choice to commit or
cancel. Helpers publish the chosen outcome into the first slot only after all
chosen effects complete. A helper can then install that slot as the stable head,
including when the helper that certified completion has stopped.

On the medium ring the two-slot change reduced helper polls for one compiled
Java invocation from 108,095 to 98,342 and interpreted polls from 114,583 to
101,513 on native and WASI. All nine size/tier checks completed with matching
Java results and complete refunds. These are single-invocation operation counts. A separate 54-process experiment
measured shorter median invocation times in each of 27 matched pairs.
[Paired timing and individual results](../benchmarks/gc/CANONICAL_MEASUREMENTS.md#paired-controller-comparison).
[Native evidence](../../ristretto-gc-evidence/controller-outcome-v1/native-functional.json)
and [WASI evidence](../../ristretto-gc-evidence/controller-outcome-v1/wasi-functional.json).

Eligibility requires all six optional effects to be absent and no partially
prepared slow-path backing to exist. A failed selected-slot allocation retains the
request's original phase and expected controller. Generation exhaustion remains a
terminal error. Reservations still cover the larger effect-bearing operation.

The new controller owns two immutable phase outcomes and one atomic decision. It
owns no predecessor controller or payload record. Installation still compares
against the exact retained expected head. Only a successfully installed or observed
head may be helped toward COMMIT. The first UNDECIDED-to-COMMIT or
UNDECIDED-to-CANCEL transition selects its outcome permanently. For this variant,
that decision also completes the operation because no external effects remain.

Cancellation may select CANCEL before installation. If that proposal is installed,
it exposes the original cancelled phase outcome, preserving the existing revision and
cancellation rules. A cancelled private proposal cannot publish its committed
phase. Failed owning acquisitions cannot certify installation or completion;
superseded private work still returns Retry unless its exact retained decision
already establishes a terminal result.

An undecided head cannot produce a snapshot. Snapshot acquisition performs one
help attempt and asks the caller to retry. A decided controller remains the head
without a second head replacement. Snapshot reads select the immutable phase using
an acquire load of the decision. A retained snapshot cannot change outcomes, and
a late helper neither reinstalls an old head nor alters the current phase.

The original owner can recover the outcome from its retained proposal after other
operations replace the head. Its private expected-head owner remains charged as
before. Repeated updates retain no chain through the installed controller. This
reduces preparation probes, temporary ownership and head replacement work, while
adding a decision read when a snapshot contains the new variant. End-to-end cost
and backing requirements require measurement; slot-count reduction alone is not a
memory or throughput result.

The bounded facade keeps two local Pending returns after one-slot preparation,
before installing the proposal. These preserve the original three-slot preparation
window for both numeric and root commands. The proposal remains private and retains
its exact expected head throughout. Each poll consumes local state; no clock, loop,
owner observation or shared gate controls this window. Cancellation, discard and
Drop release every private prefix before the participant lease. Another helper may
finish and restart the epoch while this requester is stopped; the old request cannot
close that restarted epoch. Effect-bearing preparation gets no additional yields.

An unpaced version passed its correctness matrix but regressed lifecycle time at
capacity eight, with more phase cycles and effect-bearing retries in separate
instrumented diagnostics. Its sources, failures and measurements remain retained.
The paced revision uses one rule for all capacities and both native/Wasm targets.
Neither a fixed yield window nor finite stress completion proves per-request fairness
or a wall-clock latency bound.

Effect-bearing operations preserve their admission, cancellation, effect and
certificate order. The optimization cannot bypass coupled publication, deletion
barriers, trace tickets, child-root activation or reference notifications. Epoch
and revision arithmetic, complete retirement sweeps, the retirement floor of five
and participant quotas remain unchanged.

Validation covers cancellation before and after installation, stopped owners,
failed owning acquisitions, stale preparations, full backing, exact late results
and retained snapshots. A separate effect-bearing exhaustion fixture preserves
coverage of releasing the prepaid two-outcome successor when the final allocation fails.
The new fault controls reject extra controller reservations, early snapshots,
overwritten cancellation, private completion, stale head replacement and bypassed
object effects. Two further controls reject skipped facade yield windows. The existing quota fault's expected occupied-slot count changes
from four to two for a phase-only request.

This does not establish a global pause bound, arbitrary callback isolation,
whole-JVM throughput, G1/ZGC superiority or sustained-load qualification. Real
payload destruction and host allocation retain their existing limits. Public
backend integration, VM payloads and callers, compaction, JIT and the complete
qualification matrix remain open.

Native/WASI correctness, memory checks, 73 catalog/six pool models and 68
intended negative controls pass. Finite contention, byte accounting and paired costs,
including all regressions, are recorded in [the report](../target/gc-implementation/canonical-phase-REPORT.md).
