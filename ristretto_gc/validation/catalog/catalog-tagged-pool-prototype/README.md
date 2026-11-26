# Generation-tagged owning links

This isolated package replaces weak pointer protection with indexed, generation-tagged
reference acquisition. The tagged numeric catalog imports it; the collector does not.

Each pool slot packs its generation and reference state into one atomic word. A
busy state excludes acquisition and reuse during initialization/destruction. A link
token identifies a slot and generation; loading it obtains an owner only by a CAS
against that exact live generation. An acquisition attempt makes one observation
and at most one CAS, returning contention instead of retrying internally. Generation
exhaustion refuses reuse. Strong-owner cloning is also fallible.

The link owns one count. Its compare operation transfers a successful proposal and
returns rejected ownership without running the value's destructor. On success the
expected owner keeps the old value alive while the link releases its previous count.
Each occupied slot retains one aggregate pool owner until its last value destructor
finishes, including unwind. Tokens never reconstruct raw pointers: the retained pool
supplies the actual slot reference, and owners retain its pointer provenance.

These bounds count atomic API operations, not elapsed time. Backing initialization,
arbitrary value destruction, caller retries and final backing release are outside
the acquisition bound. A finite pool is not a proof of progress under contention:
participant limits, retained-state bounds, reserved progress capacity and charged
backing remain required. The owning handle contains both a pointer and token;
representation cost still needs evaluation before integration.

```sh
cargo test --locked --manifest-path ristretto_gc/validation/catalog/catalog-tagged-pool-prototype/Cargo.toml
python3 benchmarks/gc/tagged_pool_model_check.py --output target/tagged-pool-model-check
cargo +nightly miri test --locked --manifest-path ristretto_gc/validation/catalog/catalog-tagged-pool-prototype/Cargo.toml --lib
```

Use `MIRIFLAGS="-Zmiri-disable-isolation -Zmiri-tree-borrows"` for Tree Borrows.
Actual WASI tests use `--target wasm32-wasip2 --lib` with a configured Wasmtime
runner; bare Wasm compilation uses `cargo check --target wasm32-unknown-unknown`.
Set a separate `CARGO_TARGET_DIR` when retaining exact binaries.

The integration follow-up passes nine native, four WASI, nine Miri/Tree Borrows
and nine ASan/TSan fixtures. It adds diagnostic occupancy and an observer that pins
the pool backing without retaining a value. Failed observer acquisition may mean
contention; it is not a destruction certificate during concurrent use.

Final release uses an acquire load of the BUSY word before destruction. No other
writer can change the word until that final owner publishes it free, so the load
acquires the release sequence of prior decrements. The original acquire fence
triggered a ThreadSanitizer report in a preserved read-before-reuse fixture; TSan
does not model atomic fences. The acquire-load implementation is identical in
ordinary, modeled and sanitizer builds. See the [Rust sanitizer documentation](https://doc.rust-lang.org/unstable-book/compiler-flags/sanitizer.html).
Five models import the exact state atomics. They cover stale acquisition, exclusion
through destruction, initialization ordering, final-owner ordering and exhaustion.
The reproducible runner applies the already recorded unmerged Loom RMW correction
in an isolated checker copy and rejects four broken variants. Production dependencies
are unchanged. These models exclude pointer provenance, aggregate pool ownership
and the full link implementation; native and Miri fixtures exercise those schedules.
The runner also requires an executed failing test, rejecting compilation failures
that happen to return the same exit status.

The original seven-test revision's [evidence report](../../../../target/gc-implementation/tagged-pool-REPORT.md)
preserves successful and failed runs, sources and binaries, including the first
negative runner's overly narrow expected diagnostic and the initial Clippy failure.
This does not qualify metadata/graph ownership, real phase semantics, GC progress,
throughput, latency, pauses or endurance.


## Detailed finite-generation failure

`try_allocate_status` and `try_allocate_at_status` preserve the rejected value and
distinguish invalid selection, unavailable/contended storage and permanent selected
slot generation exhaustion. Existing compatibility calls return only the rejected
value. `with_generation_limit` bounds successful allocations per slot; the
default allows the existing full 40-bit range. `generation_exhaustion_observed` is sticky and does not
certify whole-pool exhaustion. Final-generation owners remain acquirable until
their final release. No generation wraps or reuses exhausted storage.

The six exact state models now include competing final-generation claims; an
ignored-limit mutation fails. Eleven native and six WASI fixtures plus both Miri
modes and both sanitizers pass. The catalog follow-up includes two further protocol
faults and real backing/order audits. See the [complete report](../../../../target/gc-implementation/catalog-exhaust-REPORT.md).


Setup now encodes budget L in the free generation MAX_GENERATION − L. Claiming
always checks the constant maximum, without a separate pool ceiling field or read.
The default first token remains 1. The new complete-budget fixture and final-claim
model exercise this encoding; the final pool matrix passes 12 native/seven WASI
and 12 Miri/Tree/ASan/TSan tests. Catalog backing shrinks by 24 bytes on both
targets. The [paired cost report](../../../../target/gc-implementation/catalog-limit-REPORT.md)
records native improvements and retained WASI regressions.
