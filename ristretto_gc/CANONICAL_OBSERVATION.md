# Passive primary-cycle observation

This historical experiment can be reproduced from the [archived source](https://github.com/theseus-rs/ristretto/blob/0497123e05056546bc526ecc49c081f934f38d64/ristretto_gc). The current Java preview uses the [unified JVM heap](../GC_PREVIEW.md).

The opt-in canonical `Mutator::primary_cycles()` copies `PrimaryCycles { lower,
upper }`: conservative bounds on completed primary-domain full epochs at an
instant during that call. It does not count reclaimed objects, released slots,
payload destruction, or callback completion. The secondary domain's initial full
epoch is excluded. Observation is not a global collection or pause barrier.

The initial primary state is Full epoch 1. The sealed graph runtime has catalog
edges and permits only Full starts; maintenance restarts only domain 0. Therefore
Full epoch e corresponds to e - 1 completed primary full epochs, and Inactive e
to e. Checked arithmetic rejects epoch 0 and modes other than Full/Inactive.
Generic bounded catalogs may have prior Young/Old epochs, so this interpretation
is implemented only at the sealed graph layer, not on generic phase copies.
Epoch overflow continues to fail closed; generation width and RETIREMENT_FLOOR
are unchanged. No completion counter or persistent runtime field is added.

The raw operation makes one controller-link load and one exact-generation owning
acquisition attempt. Failure returns contention; it never retries. It copies:

- Stable: that phase twice.
- Effect-free Phase operation: the chosen outcome for a decided COMMIT/CANCEL;
  both possible outcomes while undecided. Its decision is monotonic.
- Busy operation: both immutable Stable children regardless of decision or
  completion flags. A COMMIT decision alone does not certify effect completion.

It never calls `snapshot_in`, `finish`, or `Record::enter`, changes an operation
or controller decision, or allocates a pooled object. The possible phase outcomes
are not linearizable full Phase snapshots: their revisions can both be newer
than the predecessor. The cycle projection nevertheless bounds an instant in
the observation interval. If another participant advances while the acquired
head is parked, the result is not necessarily current at return. The API does
not refresh its acquired head to make that claim.

Access requires an IDLE reserved Participant, checked before touching the head.
The public facade requires a mutable Mutator borrow, excluding an overlapping
allocation/read/write request. There is no arbitrary Runtime-reader entry point.
Only numeric copies escape; no controller, cell, root, slot, or pool owner does.
Releasing the temporary head can be the final release of its transitive owners,
including a record/payload destructor. Thus “passive” means no protocol helping;
it does not mean destructor-free, callback-free in Drop, or bounded wall time.

The ownership census is a subset of an existing IDLE `phase_in` call: one acquired
head, with none of `finish`'s additional proposed-value clones/guards. A Busy head
owns at most two controls (itself and its successor with two immutable outcomes). Its conservative
transitive union, before aliases reduce it, is at most four Roots, six Cells, ten
Memberships, one reference slot and three reference states, and one edge slot and
three edge states. This follows from its two cell Effects, two RootEffects, one
ReferenceEffect (with two embedded root links), and one EdgeEffect. Edge states
contain keys, not target-Cell ownership. The existing edge/reference-enabled
reserve factors (root 24K, cell 40K, membership 56K, slot 6K, state 20K) already
include this temporary path. Observation acquires no second head or new kind of
owner, and it cannot overlap its participant's outstanding request. This is a
closure/subset argument, not a new claim based on missing historical artifacts.

Deterministic tests cover idle-before-load, one failed acquisition, unchanged
Phase decisions and Busy decisions/completion flags, zero record entries,
transitive last-owner destruction, checked numeric projection, public cancellation
and credit teardown, and completed cycles while a real payload remains rooted.
Native parked-observer tests let another reserved participant complete real Busy
publication and tracing effects and check existing pool occupancy bounds. A
separate parked Stable test advances multiple epochs. Thread coordination has
bounded channel waits. These are finite correctness cases, not stress or timing
measurements; the Busy test exercises cell/membership effects, while the wider
root/reference/edge closure is justified by the ownership subset above.

This component does not wire cycle observation into VM telemetry or the exported
measurement boundary. Current captures continue to report cycles as unavailable.
A later binding must preserve observation intervals with clock brackets, reject
unavailable/inconsistent observations, and separately review subtraction of start
and end bounds. No zero-pause, sub-millisecond, throughput, collector comparison,
or sustained-load qualification follows from these APIs or tests.
