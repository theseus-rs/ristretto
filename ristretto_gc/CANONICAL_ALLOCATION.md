# Canonical leaf allocation transactions

This historical experiment can be reproduced from the [archived source](https://github.com/theseus-rs/ristretto/blob/0497123e05056546bc526ecc49c081f934f38d64/ristretto_gc). The current Java preview uses the [unified JVM heap](../GC_PREVIEW.md).

This optional internal path composes private payload preparation, bounded catalog
creation, coupled initial-root publication, and one typed returned root. Existing
public `Gc` constructors and default policies still use legacy registration.
`LeafAllocation` is not yet a public heap API or a whole-collector integration.

`Gc::prepare_reserved` now owns the shared descriptor preparation sequence. The
legacy caller requests an initial legacy root and finishes legacy publication.
The canonical caller first rejects types whose unsafe `Trace` capability permits
managed edges, prepares backing once, reserves exact payload credit once, and
requests unrooted private preparation. Its existing leaf eligibility checks then
reject callbacks, finalizers, captured construction protection, and legacy roots.
Unsupported or failed preparation destroys the original private owners in their
existing order. It does not run allocation-recovery collection. Host allocation,
credit reservation and arbitrary preparation/attachment hooks remain unqualified.

## Catalog ownership and linearization

`submit_rooted_record` accepts a unique record only after reserving one root permit.
Busy, invalid-slot/address or quota rejection returns the same record. The request
uses the participant's existing inline state; there is no per-request box or extra
root-token permit. Object and root discovery slots are explicit inputs to this original API. The
subsequent [automatic API](CANONICAL_RETRY.md) retains requests across slot
collisions and full/contended root quota, reserving before any cell announcement.

| State | Retained ownership | Drop/cancellation consequence |
| --- | --- | --- |
| Starting/creating record | Private record or cell preparation; root permit | Private storage drops; no installed object is silently detached |
| Announced record | Exact pooled cell; root permit | Store a monotonic abandonment request in that exact cell |
| Creating root | Exact cell and private root preparation; root permit | Same cell intent; private root storage drops |
| Preparing/attempting publication | Exact cell, root token, canonical preparation/attempt | Request cancellation, post cell intent, drop the token to request root removal |
| Ready | Certified live-publication result, held by its unique root token | Root removal requested if result is discarded |
| Taken | Root token transferred once | Caller root lifetime controls its removal request |

Cell discovery remains the creation CAS. Publication still linearizes at the
catalog commit decision and couples target tracing membership, root activation,
phase work and the record's certificate. RootCreated is returned only after this
certificate is established. A ready request stays Busy until take or discard.
`LeafAllocation` exclusively borrows its participant and matching heap; callers
cannot interleave a second request or steal its untyped root. `LeafRoot<T>` owns
the resulting token and uses the existing heap/publication/type-checked read view.
It neither clones the token nor keeps a strong heap owner.
The [shared-root follow-up](CANONICAL_SHARED_ROOTS.md) allows typed aliases of that
one token; only the last alias requests removal. It prepays private wrapper backing
for each live logical root and simultaneous pending typed request.
The corrected final-alias teardown frees that backing through `Arc::into_inner`
before dropping the unique root and refunding catalog credit; a populated final-owner
audit covers both leaf and graph roots. Ordinary Arc teardown in the preceding
shared-root version did not preserve that order.

Ordinary cancel returns false when commit already won. `discard_allocation` gives
up the result even in that case: it drops only this request's ownership, leaving
installed work helpable. Request destruction attempts cancellation before posting
cell intent. Root-token destruction posts its existing removal request. A helper
therefore finishes a winning publication before applying deletion; losing private
publication can instead leave Prepared membership for abandonment. No independent
status write replaces the canonical membership transition.

An abandonment observation authorizes only an exact-key Prepared cleanup attempt.
Helpers use the controller's existing Abandon operation, whose membership check
excludes Tracing values. The intent is monotonic and belongs to retained Cell
storage, not a reusable discovery index. A late request cannot affect a replacement
cell in the same slot. Publication may already have won when the intent is posted;
that intent is harmless for Tracing membership. Requested root cleanup remains
separate and preserves the canonical deletion barrier.

## Bounds and remaining integration

A rooted transaction adds at most one retained cell per participant to the prior
root-enabled census. Its creation, root preparation and publication phases are
mutually exclusive. Existing root and controller bounds remain unchanged. The
conservative cell bound becomes N+2M+5K+2, below the N+2M+6K+4 reserve; membership
ownership becomes N+2M+9K+4, below N+2M+10K+8. Reference-enabled reserves are larger.
The abandonment byte and larger inline request can change actual backing sizes;
Footprint calculates the concrete layouts and accounting checks those requests.

Request fields drop before participant quota. Root storage and payload destruction
precede root quota release; catalog deallocation precedes its charge refund.
State transitions bound catalog work, but a poll or Drop can run an arbitrary
payload destructor. These APIs do not yet bound callback time or host-turn latency.
The internal [maintenance policy](CANONICAL_MAINTENANCE.md) now supplies incremental
leaf cleanup with complete sweeps and epoch-bound Finish. The subsequent internal
[runtime owner](CANONICAL_RUNTIME.md) binds participants to a heap/catalog and
reserves owning-wrapper storage at setup. Public selection, scheduling/wakeup,
indefinite generation handling, general graphs,
conditional references with real payloads, admitted callers, JIT, performance and
endurance qualification remain required. Finite exhaustion and explicit-slot collisions
remain failures; automatic requests stay pending on full discovery arrays or root
quota. Neither API promises allocation progress
in an exhausted live heap.
