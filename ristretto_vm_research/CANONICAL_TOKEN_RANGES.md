# Canonical token ranges

`src/canonical/token_ranges.rs` supplies the experimental canonical frame table.
Earlier frozen graph/array checkpoints retain their original invocation-prefix
plus 32-bit object-token encoding. The range integration is a separate checkpoint.

The new allocator has one private process-wide `AtomicU64` frontier starting at
one. Successful reservations claim disjoint ranges of positive `i64` IDs. The
terminal frontier is `2^63`, which is never issued. Arithmetic is checked; the
final reservation may be shorter than requested so that `i64::MAX` remains usable.
Nothing resets, wraps, or recycles the frontier. Dropping a partially used range
permanently discards its remaining IDs. Separate loaded copies of this allocator
must not exchange token tables; they would be distinct identity domains.

The crate-private API is:

```rust,ignore
TokenRange::try_acquire(NonZeroU64) -> Poll<Result<TokenRange, LeaseError>>
TokenRange::issue(&mut self) -> Option<i64>
TokenRange::remaining(&self) -> u64
```

`TokenRange` is neither `Clone` nor `Copy` and has private fields. It retains only
two integers. The source retains only the frontier, with no managed roots,
history, or registry of leased ranges. Refilling an invocation can therefore
continue beyond the former roughly 4.29 billion object IDs with constant allocator
storage. This is still a finite process-wide identity domain; exhaustion is an
explicit terminal error, not a claim of unlimited operation.

Reservation performs one atomic load and at most one weak compare-and-exchange.
Contention or a spurious failure returns `Pending`; there is no retry loop or lock
fallback. Targets without native 64-bit atomics return an unsupported-target
error. Issuing an ID from an owned range uses no atomics or allocation. The bounds
describe operation counts and do not establish a latency bound under scheduling
or hardware contention.

## Frame-table integration

1. Exact live-entry membership remains authoritative. Leased-range membership cannot
   establish that a token was issued, is live, or belongs to the current table.
   Validate frame ownership with the existing independent registry identity.
2. The table performs `same_object` deduplication and frame/object capacity checks before
   consuming a new ID. Reference publication validates existing entries only and
   never reserves IDs.
3. If a new object needs an ID and the local lease is empty, `intern` returns
   `Pending` without changing the frame or entry. The host adapter keeps the
   incoming owning root or result alive, releases table/frame borrows, and attempts
   reservation outside native callbacks. On contention it yields to the host and
   resumes later with ownership intact. There is no spin loop in `intern` or
   publication. `request_polls` bounds lease attempts per exported reference.
4. Each table retains at most its current lease (normally up to 4096 IDs). There
   are no strong historical roots or growing lists of old ranges. An ID consumed before a later operation fails
   stays consumed. Exact-entry lookup rejects foreign and stale IDs even when a
   physical entry slot is reused.
5. Integrated lifecycle tests exercise foreign tokens between independently
   refilled tables, stale tokens after entry reuse, duplicate-root interning with
   an empty lease, exhausted leases, contention during rooted return handoff, and
   cancellation while waiting to refill. A native test runs the shared primitive
   array/call/return fixture with one-ID leases. Pending injection deterministically
   covers suspension ownership; separate allocator tests exercise real CAS races.

The current tests cover positive/null boundaries, a refill across `u32::MAX`,
last-range exhaustion at `i64::MAX`, dropped-range nonreuse, deterministic losing
CAS observations, concurrent disjoint leases, constant range size, and an exact
entry-lookup model. The lookup model demonstrates only nonaliasing of IDs; it does
not validate actual GC root ownership. Native threaded testing is a small finite
correctness case, not an endurance or performance measurement.

The source and its adjacent test module can be checked directly with `rustc
--test --edition 2024`, including `--target wasm32-wasip2`. The integrated table and host-refill
tests also run on WASI without generated machine code.
