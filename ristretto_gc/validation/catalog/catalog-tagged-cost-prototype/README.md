# Tagged versus pooled catalog cost screen

This executable compares two isolated numeric catalogs, not collectors or JVMs.
Both perform the same lifecycle: prepare and insert a record, acknowledge both
domains, retire it, and release its registry slot. Resident records remain live
through the measured loop and are retired during teardown.

Build with `--features counts` for allocator requests and requested bytes. Build
without that feature for elapsed loop time; this build uses the normal allocator
without counter updates. Report setup, loop and teardown separately. The pooled
baseline retains an ArcSwap thread-local cache after catalog teardown; the tagged
candidate releases all requested backing in this single-thread fixture. Requested
bytes are neither process RSS nor Wasm host memory.

```sh
cargo run --release --locked --manifest-path ristretto_gc/validation/catalog/catalog-tagged-cost-prototype/Cargo.toml --features counts -- tagged 16384 65536
cargo run --release --locked --manifest-path ristretto_gc/validation/catalog/catalog-tagged-cost-prototype/Cargo.toml -- pooled 16384 500000
```

Use `--target wasm32-wasip2` with Wasmtime for guest results. The arguments are
`pooled|tagged`, resident record count, and lifecycle count. Runs should exceed the
cell-pool wraparound point. Preparation retries are reported and included in loop
time; a failed bounded slot search is not proof of whole-pool exhaustion. The
fixture permits repeated attempts in a quiescent catalog, with a finite cap.

Fixed capacities and representation differ intentionally: tagged cells occupy a
preallocated pool, while baseline cells use individual Arc allocations. Both retain
the prototype's control/membership reserve. This screen does not establish a
production participant bound, contention fairness, a GC poll deadline, reclamation
progress, end-to-end latency or G1/ZGC qualification.

The recorded 48-process allocation screen and 112-process timing screen reduce
loop allocation requests from one to zero, and median lifecycle time by 51–53%
native and 24–25% WASI. At 16,384 residents, setup backing grows about 40% native
and 79% WASI. [Report and figures](../../../../target/gc-implementation/catalog-tagged-REPORT.md)
retain paired intervals, process wall times, preparation retries and exact inputs.

The subsequent [resumable-preparation screen](../../../../target/gc-implementation/catalog-resume-REPORT.md)
fixes a fragmented-slot livelock while preserving those allocation/backing counts.
It adds 9–10% native / 15–16% WASI lifecycle cost relative to that first tagged
revision. Both screens remain available; the original speedups are historical
measurements of their recorded binaries.


The participant-limit screen adds `bounded` and compares it with `tagged` using
identical capacities derived for one mutator and one helper. It measures complete
create/insert/two-ack/retire/release lifecycles; `bounded` also validates copied keys
and retains state across explicit polls. Individual insertion preparation/commit
stage boundaries differ, so only aggregate lifecycle allocation/time is compared.
The legacy `prepare_retries` column is zero for `bounded`; it does not measure polls.
Setup includes the facade's extra Arc allocation. Participant values live inline
on the stack; this screen does not claim their storage is charged as heap bytes.
Older tagged/pooled results use their archived sources and original capacities.
See [the participant screen](../../../../target/gc-implementation/catalog-quota-REPORT.md).
