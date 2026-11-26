# Ordinary registration image activation

## Ordinary collector at `7acdabb5`

An ordinary constructor exclusively reserves an exact address in the object
directory and an identity in the selected identity backend. It retains those
reservations while checking the descriptor and young-index preconditions.
Live lookups require a published descriptor. A private or partially indexed
birth grants no access to its payload. Dropping the preparation clears only
its own entries and refunds its byte reservation. Duplicate attempts must
still fail even if they present the same descriptor.

A shard owns separate directories for descriptors and young membership. Each
populated page covers 4,096 slots. Descriptor storage grows one 64-slot chunk
at a time. A page pin counts an in-flight claim before setting a bit. Clearing
the last bit can seal the page only after the last claim is released. A
deferred unlink validates both the exact owning leaf and the directory root.
A stopped allocator or remover can retain storage while a peer uses another slot.
Young collections enumerate young pages and validate the live descriptor under
the shard before promotion or retirement. Full collections enumerate object pages.

Index installation precedes the phase decision. A scan can pass a private
descriptor and veto its pending-to-live transition. The publisher then clears
the speculative marks and old-cohort tag and retries against the phase clock.
During strong tracing, graph births link a provisional traversal in each
strong domain. Owning epoch identities distinguish successive incarnations
with the same flags. Cancellation prevents work from an abandoned attempt
from tracing a later construction. Native schedules also cover a held marking
injector and disabled numeric certification.
[Ordinary publication schedules](../src/publication_tests.rs) ·
[Address-page implementation](../src/nursery.rs).

### Remaining live-root gate

A native probe on this revision parked a real `Gc::from_handle_rooted` call
during typed descriptor lookup. A two-object heap held one rooted survivor and
one unreachable allocation. A third allocation exhausted its 1,024-poll budget
under both legacy and counted publication. Repeating the schedule with an
owning descriptor copy and the object guard already released produced the same
result. Root publication admission still spanned the stopped lookup. The
survivor remained valid and teardown released all charged bytes in both
controls. Any replacement must preserve validation against the exact identity,
type and current reference decision until its root is published.

## Earlier immutable-image experiment

At `d8766145`, ordinary registration held the object shard while inserting
the descriptor. An immutable three-index image was separately explored as a
private migration component. The design and model below record that experiment.
Commit `eabee970` removed the private image and its model tests. The source and
test descriptions below refer to its parent revision. The active collector uses
exact slots and phase-certified visibility as described above.

One `ArcSwap` head owns three immutable `Directory` roots. Preparing an insert,
exact removal or promotion changes no shared membership. One compare-and-swap
selects the complete image. Expected/proposed owning heads prevent pointer ABA.
Each registry has an owning identity: a completed operation from another registry
is rejected even if its installation certificate is already set.

Every successful head replacement certifies its predecessor before publishing
the new head. Therefore a helper can recover a stopped CAS winner's installation
after that image has already been removed or replaced, without reinstalling it.
`Installed` certifies only historical image selection. `Stale` certifies that
this proposal never won; `Retry` leaves its outcome unresolved. None of these
results certifies tracing, root visibility, a phase transition or safe payload
access. In particular, callers must not manufacture a new proposal for a retired
descriptor just because the indexes are now vacant. Activation must bind the
existing one-shot prepared-record custody to this image transition.

Preparation allocates at most three changed radix paths. Commit attempts contain
one owning load and at most one CAS, with no protocol retry loop. `ArcSwap`,
allocation and last-owner path/descriptor destruction have no latency bound.
The real-descriptor tests demonstrate that retained images do not retain retired
payload bytes or the transitive managed roots of a heap owner.

Four Loom tests compile the same image state source with instrumented owning
links/atomics and the unchanged immutable directory. They cover competing
complete images, a stopped successful insertion followed by removal/address
reuse, a stopped pre-CAS writer, and a stopped removal. The exploration uses a
preemption bound of two and at most 10,000 permutations; `ArcSwap` internals and
host allocation are outside that model. Native forced schedules exercise the
actual owning link with a publisher parked before and after CAS. Those are image
component schedules, **not** the stopped ordinary-admission progress gate.

### Activation joins

The intended image uses the existing immutable `publication_directory::Directory`
for address-to-descriptor, identity-to-address and young membership. A single
owning head selects all three. Expected/proposed heads must retain their exact
identity until the operation terminates; addresses and numeric revisions alone
cannot identify an expected version. No mutable-index mirror may authorize a
root or weak upgrade after the image becomes authoritative.

The following transitions are a coupled activation requirement, not a sequence
of independently safe default switches:

| Current authority / operation | Required replacement and obligation |
| --- | --- |
| `prepare_registration` / `register_prepared` | A prepared owning record retains the initialized parent, initial root and construction child/conditional-owner protection. One image transition publishes membership with both domains' phase obligations. |
| `metadata_for_address`, `identity_for_address`, `address_for_identity` | Resolve exact identity and address from one image. A metadata owner alone grants neither payload access nor root resurrection. |
| Root installation, removal, `lookup_rooted` and validated weak reconstruction | Fresh image/phase validation must cover the owning registration and strong/weak barrier decision. Numeric lookup remains a hint until that decision succeeds. Initial root tokens cannot be copied into an image as if copying owned root permission. |
| `mark_registration_birth`, `mark_born`, `shade_metadata` | A late helper must not overwrite a later epoch. A mark is not a durable traversal obligation; queue membership must be coupled to the admitted image/phase transition. |
| Both domains' start, strong close, reference stages and finish | Close must observe every committed registration obligation without waiting for its original publisher. Merely observing an empty announcement slot is insufficient. |
| Object/young scans and remembered/deletion barriers | Scan one authoritative image; preserve newly installed objects behind the numeric cursor through the coupled barrier. No old mutable index may silently omit them. |
| Promotion, sweep, identity removal and shutdown | Validate the exact descriptor and replace the complete image. Stale removal cannot erase a reused address. Extract callback owners before entering caller-controlled destruction; this is coordinated with the separate reclamation work. |
| Statistics and diagnostic snapshots | Derive counts from the selected image; label observations rather than treating descriptor snapshots as live payloads. |

Old images may retain immutable descriptors/storage identity. They must not keep
retired payloads, heap-owner roots, construction roots or allocation credit alive.
The existing weak `MetadataRef.owner` and detached payload owner are necessary
but do not discharge arbitrary callback/finalizer progress obligations.

## Validation boundaries

The active prerequisite tests use real ordinary parent/child/weak references,
all three owning-root modes, and both legacy and counted admission. Controlled
partial write sets are restored before collecting or accessing payloads. The
tests check rejection before effects, release of admission on error, continued
reachability and final zero managed charge.

The ordinary stopped-publication regression parks the constructor after its
address claim or after its nursery update. In each schedule an unrelated
capped allocation must reclaim garbage and finish before the constructor
resumes. The test runs under both publication policies with one shard and a
two-object cap. At the earlier staging base `398c4656`, the same regression
failed under both policies when explicitly run with `-- --ignored`.

```sh
cargo test --locked -p ristretto_gc --lib capped_allocation_reclaims_inside_registration_commit
```

The adjacent post-unlock regression runs in the default test suite. It forces
the graph's trace hook to run while its constructor stays parked and checks the
child, reclamation and allocation under both available and held injector schedules.
In a separate control at `d24eb6d7` with the default single shard, the constructor released
phase admission before parking and retained just one index guard. Holding the
object, identity or young guard individually kept the peer pending with zero
bytes freed under both policies. Releasing all three let the peer reclaim eight
bytes and finish under both policies. A fresh ordinary registration took place
after resumption.
[Control fixture](../../../ristretto-gc-evidence/alpha-progress-0134e21a-v1/handoff-single-index-probe.patch).

The ordinary nursery now stores an address bitmap per populated page in an
owning directory for each object shard. A reader can retain its directory while
registration, promotion and retirement update the shard. If a writer empties a
page and later reuses its key, a fresh reader sees the new page. A parked reader
can retain old bitmap storage until it resumes. No directory entry owns a
descriptor, payload, root or managed-byte reservation.

Within a page, a cursor reads subsequent bits directly. When the cached page
has no next bit, the cursor rechecks its key in a fresh directory. A retry on
a different shard cannot read the previous shard's page. Under its object
shard, the collector checks that the current descriptor is still young before
minor sweep, cross-domain shading or relocation. Promotion updates age and
removes the corresponding bit while it holds that same shard.

The parked-reader test holds a real scan view on the default single shard.
A capped peer must reclaim an unreachable payload, allocate and promote a
rooted survivor before the reader resumes. Both publication policies exhausted
4,096 polls at `d24eb6d7` and pass with the independent nursery mutex removed.
At that checkpoint the default identity map retained its writer mutex.
[Parked-reader regression](../src/index_progress_tests.rs).

The ordinary identity index now uses one occupancy bit and one atomic address
per ID on a populated page. A capped peer can allocate while a constructor
retains a page reservation. It can also reclaim while the last-slot remover
is parked before directory unlink. In the latter regression, four peer threads
first encounter the same full heap, then repoll only on waker notification and
retain all four returned roots until the last peer finishes. Both publication
policies and both phase scheduling modes use ordinary GC entry points.
The earlier inside-commit schedule parked a publisher holding the object shard.
Reclamation then required garbage on a different shard. At `7acdabb5`, two
publishers can collide on the same page, and a parked claim leaves unrelated
reclamation on its shard available. Structural CAS retry, host allocation,
live root/sweep guards, callbacks and whole-VM latency remain separate obligations.
[Last-page progress test](../src/identity_pages_tests.rs).

The synchronous VM equivalent calls `make_room` before `Gc::try_new`. Passing an immutable-directory model,
this precondition check, or the bounded catalog's separate protocol does not
establish that gate. Allocation, obsolete-path destruction and arbitrary Trace
callbacks have no latency bound here.
