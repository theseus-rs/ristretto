# Conditional references in the isolated catalog

This protocol extends the canonical-root experiment. It is not selected by the GC
runtime. Copied keys, observations and these bounded handles grant no payload
access. Concrete graph traversal, prepared-record cleanup, admitted callers,
callback scheduling, JIT and whole-GC qualification remain separate obligations.

A unique reference token identifies an exact reference object and registry slot.
Its immutable pooled state contains kind, target and owner identities, per-domain
late-birth eligibility, activation/pending flags and optional construction/queue
root keys. Only a canonical controller replaces this state. Drop posts removal
intent; a pending notification or live construction lifetime prevents premature
slot retirement. A retired slot releases its registry location only after both
embedded root links are empty. Stale keys cannot name a replacement at that index.

Soft and Weak decisions run at stage 1, Final at stage 2, and Phantom at stage 3.
Retained Phantom first makes its group decision at stage 3 and participates at
stage 2 after activation. The per-mark phantom decision bit shares the mark epoch,
survives promotion and retention strengthening within that epoch, and resets in a
new epoch. Birth retention is distinct from reachability before the boundary.
Upgrade requires an exact eligible referent and a fresh caller root. A clone can
preserve existing same-epoch eligibility; it cannot mint a late-birth exemption.
Clear preserves notification/construction custody that already exists.

A managed reference binds one exact owner. Retaining edges and notifications depend
on that owner's reachability or generation exclusion. Host-only references never
produce managed notifications. Final activation can queue immediately; other kinds
record pending work and queue a still-live owner at the final reference stage.
Self-references merge referent and owner-root effects before accounting/certification.
Each domain has a reference cursor for its current stage; phase advancement requires
that cursor and ordinary traversal tickets to be complete. Failed owning discovery
cannot certify an absent reference or skip its work.

Every exact reference object embeds distinct Construction and Notification root
locations. Root creation retains that object through announcement. A late creator
therefore writes into the old object even if its registry index has been reused;
it cannot consume the replacement reference's reserve. Root origin records the
reference identity, index and role. Ordinary caller-root activation cannot adopt a
ReservedPrepared root. Reserved roots can be released once internally Retired even
if an old creation token still exists; their status never resets. Ordinary root
slots preserve the earlier token-Drop-before-reuse rule.

Construction reservation publishes an InternalLive root for an eligible referent
and returns its unique construction lifetime only after certification. A private
callback must not rely on a merely Prepared root. Several references can prebind to
one Prepared owner before ordinary guarded owner publication. Alternatively one
reference can bind in the same operation as owner publication. The exact prepared
owner's Record guard spans the immutable decision, all effects, cleanup latch and
certificate. A secondary object effect is only a barrier on an already tracing
object; it never publishes another Prepared owner without a guard.

Helpers release construction custody after the exact owner's publication becomes
certified, or after the lifetime token posts cancellation. Absence/retirement of a
prebound owner alone cannot end a still-live private callback lifetime. A stopped
constructor retains its graph but need not stop other cycles or unrelated
reclamation. Cancellation shades the required deletion edge before retiring the
internal root. An exported construction handle keeps its distinct quota until its
own token and retained payload destruction have finished, even after publication.

A scan that needs notification capacity returns a resumable plan with its exact
snapshot, slot, referent proposal and phase work. It discovers an existing reserved
root or creates one in the exact reference object's notification location. Failed
owner acquisition preserves the plan; a superseded plan never refreshes its head.
Target retention, owner rooting, reference state, cursor and pending counts are
certified by one operation. All fallible proposed-owner acquisitions happen before
that helper's irreversible COMMIT. Each exact effect uses a strong version CAS so a
late helper cannot reactivate a retired root or overwrite a newer reference state.

A notification's internal root retains custody after its creator token is dropped.
Take transfers it into a fresh caller root of that exact owner, retiring the old
internal root and consuming the queued key in the same operation. Delivery can only
win once. A separate helper can finish the scan and delivery with all mutators
stopped, including with every caller-root permit occupied by existing handles.
Actual scheduling, queue consumers and fair maintenance must be supplied by runtime
integration; this data structure is not a wakeup policy.

The bounded facade provides Q reference permits, Q independent construction permits,
M caller-root permits and K participant permits including separately reserved
helpers. Neither new handle is Clone and neither exports a raw owning share. Its
lease follows its token in field order and retains the complete catalog charge
through final payload destruction. Participant state precedes its registration
lease. Cancellation and Participant Drop reach nested installed attempts; a won
COMMIT cannot be revoked. Ready results retain their permit until take/cancel/drop.
Notification creation tokens remain inside a charged participant and do not consume
an exported permit that stopped callers could exhaust.

The owner census in [the design census](../../../../target/gc-implementation/canonical-reference-facade-design.md) includes old reference
objects with late embedded-root announcements and temporary helped controllers.
Q-enabled pre-rounding reserves are controls8K+8, references2Q+6K+4,
reference-states2Q+20K+8, roots2M+6Q+24K+16, cellsN+2M+6Q+40K+24 and
membershipsN+2M+6Q+56K+32. Counts round upward to checked powers of two. Q=0 preserves
the previous pool counts but larger inline layouts can still increase byte costs.
Footprint uses actual generic layouts and includes bounded inline owners. Indirect
R allocations, application containers, allocator overhead and RSS are excluded.

These reserves do not establish fairness, infinite service with finite generations,
bounded arbitrary payload destruction, collection of application ownership cycles,
or whole-GC pause/throughput/latency behavior. Full runtime integration and the
accepted qualification plan remain open. Development validation and allocator costs
must be cited with their exact source manifests; no defaults change here.
