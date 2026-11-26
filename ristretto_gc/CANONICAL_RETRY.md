# Automatic canonical leaf allocation retries

The optional internal `LeafMutator::begin_any(value)` and underlying
`submit_rooted_record_any(address, record)` choose object and root discovery
locations incrementally. One exclusive request retains the exact payload and
prepared record through contention, full root quota and slot collisions. The
explicit-slot APIs retain their existing terminal collision/error behavior.
Public `Gc` constructors and defaults still select the existing backend.

## Ownership and publication

An automatic submission tries one root-permit reservation. A full quota or lost
quota CAS retains its record in the participant's Starting state without a root
permit. Each later poll tries that reservation once. No cell is announced before
success. The participant already bounds this one private record; an unreserved
request does not create an additional root owner. A full quota needs existing
caller roots to be dropped. Helpers cannot take those caller-owned permits away.

An occupied object preflight returns the same record to the request, which advances
one candidate slot. Once cell backing exists, `Creation::poll_rotating` retains
that exact cell, identity, record, address and membership after a losing discovery
CAS. It advances the private cell's location for the next poll. Root creation uses
the same rule for an external root, retaining its target and the original permit.
No payload preparation, attachment, identity assignment or backing allocation is
repeated merely because an announcement loses. The subsequent publication decision,
certificate, root transfer and durable cancellation protocol are unchanged.

Cell and Root discovery locations are private atomic words with load-only
accessors. Only the exclusive private creator changes its location after a failed
announcement. A failed link CAS exposes no proposal; its temporary owning share
has returned before that store. A successful release CAS publishes the location,
and all external owning discovery uses acquire protection. The location never
changes after successful announcement, so subsequent relaxed loads observe its
fixed value. A completed creation rejects further polls. No mutable slot access
escapes. Reference construction and notification roots reject rotating polls;
their fixed reference identity and role remain usable through explicit polling.

Per-participant object/root hints start from the observed registration count and
advance from the actual returned root when it is taken. They are hints, not unique
identities or reservations. Concurrent participants may choose the same location.
Each poll performs at most one quota attempt, backing attempt, discovery attempt
or existing controller step. There is no whole-array scan or hidden collection.

## Progress, cancellation and bounds

Full discovery arrays remain Pending while retaining bounded storage. Callers must
poll maintenance and the allocation, release live roots, or cancel. Under a
quiescent catalog with available capacity, rotating attempts visit every discovery
location. This is not a fairness or finite-time guarantee under adversarial
scheduling. Permanent pool-generation or identity exhaustion remains an explicit
error. Host allocation, managed payload credit, preparation, admission callbacks
and actual destructors still require separate qualification.

Cancelling an unannounced request destroys its exact private record; an announced
cell posts its existing monotonic abandonment intent. Root-token destruction posts
its existing removal intent. Publication that already won remains helpable. Field
destruction retains participant/root quotas and catalog charge through actual
payload cleanup. No new owning pooled observations or per-request boxes are added.
The prior cell, membership, root and controller capacity bounds remain valid.

The measured production participant grows by 16 bytes on native 64-bit and 8 bytes
on WASI 32-bit. Footprint charges this storage for all participant quotas, including
helpers. At seven mutators and two helpers this adds 144 / 72 logical bytes.
Requested catalog backing, setup allocation counts and the separately charged
runtime-wrapper supplement remain unchanged. With 16,384 records and 64 roots,
the runtime's total reservation is 5,835,968 native / 5,043,756 WASI bytes. These are
frozen-toolchain layouts, not a stable ABI or RSS cap.

## Evidence and remaining integration

The 39-command frozen matrix passes native and WASI suites, bare Wasm, Clippy,
formatting and accounting. All 60 native leaf fixtures pass strict-provenance Miri,
Tree Borrows, ASan and TSan; 51 pass WASI release. Ten catalog fixtures, including the nine new cases, also
pass both interpreters and both sanitizers. All 55 corrected transition models
pass, including competing private cell/root announcements and fixed-role rejection.
They exclude actual pointer protection, concrete payloads and facade quotas.
Sixteen deliberately broken native/WASI processes and two model faults fail at
intended invariants; a positive compiler control and four rejected programs check
slot privacy and allocation-borrow lifetimes.

The allocator audit compares 744 existing rows, all retaining the same backing and
request count. Another 96 rows force both announcement collisions. All 816 measured
catalog lifecycle rows request zero new backing. Forty-eight production-layout
rows record participant growth and unchanged wrapper supplement. Payload preparation,
borrowed/caller storage, allocator metadata, RSS and execution time are excluded.
See [validation](VALIDATION.md#automatic-canonical-allocation-retries-2026-11-15)
and [the archived report](../target/gc-implementation/canonical-retry-REPORT.md).

This closes automatic selection and payload-preserving collision/quota retries for
the internal leaf path. Public backend/package selection, scheduling/wakeups,
general graphs, real conditional references, bounded callback capabilities,
admitted callers, JIT, physical accounting and G1/ZGC performance/endurance
qualification remain open. No throughput or pause claim follows from these checks.
