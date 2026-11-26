# Canonical allocation diagnostics

This separate binary counts Rust global-allocator activity while executing the
shared `CanonicalRingWorkload` classfiles through canonical interpreted or native
compiled execution. It is an allocation diagnostic, never a timing comparator or
whole-JVM/performance qualification. It does not enable service telemetry.

Build with a source-specific `CARGO_TARGET_DIR`:

```sh
cargo build --offline --locked --manifest-path benchmarks/gc/allocation_capture/Cargo.toml --features compiled
cargo build --offline --locked --manifest-path benchmarks/gc/allocation_capture/Cargo.toml --target wasm32-wasip2
```

The executable accepts `INPUT_DIRECTORY [--check] [--samples N]`, where N is 1–64
and defaults to 1. Inputs are `classes/` containing the exact classfiles and
`admission.json` using the service harness's execution fields:

```json
{
  "schema_version": 1,
  "tier": "interpreted",
  "class": "CanonicalRingWorkload",
  "method": "small",
  "argument": 1,
  "configuration": {"objects": 40, "roots": 128, "mutators": 1, "helpers": 1, "edges_per_object": 3},
  "limits": {"instructions": 4000000, "request_polls": 1000000, "cleanup_polls": 4000000, "frames": 64, "native_slices": 4000000, "array_bytes": 8388608},
  "expected_result": 619975554,
  "require_compiled_backedges": false
}
```

Supply the independently verified expected result for the selected workload and
argument; the example uses the frozen `small(1)` oracle row. Optional service fields `recorder_capacity` and
`window_kind="entry_body_v1"` or `"full_invocation_v1"` are accepted for compatibility and never select this
driver's span. Unknown/duplicate fields, unsupported tiers and zero limits reject.
The driver hashes the exact consumed admission/class bytes using the service
harness's sorted relative-path class-manifest convention. Build/source/executable
and launch identity require separate external receipts.

All results are retained in preallocated stage storage. The first and only stdout
line appears after every stage and validation succeeds:
`RISTRETTO_CANONICAL_ALLOCATIONS {JSON}`. There is no stdout or stderr on the
success path during counted stages. A failed invocation exits unsuccessfully and
emits no success packet. Stages are:

- `admission`: class decoding, capability/layout admission, and strict verification.
- `compilation`: native request-only compilation, only for the compiled tier.
- `full_invocation`: each complete `Program::run` or `CompiledProgram::run`, including
  heap/frame setup, entry execution, GC cleanup, runtime drop and report/profile
  construction. There is no implicit warmup. Every sample validates the supplied
  checksum, zero remaining payload/catalog charge, and released-object count equal
  to allocation count. Compiled samples require actual generated entries and can
  additionally require backedges.
- `program_teardown`: release the compiled program and admitted metadata.

`--check` irreversibly disables allocator accounting before reading inputs, skips
all counter snapshots, and emits `counts: null` with `measurement: false`. Use it
for functional driver checks while a measurement window is unavailable. Meaningful
allocation samples require a separately coordinated quiet-host run. The binary
must stay separate from uninstrumented timing comparisons.

Counters include attempted requests/requested bytes, successful and failed
requests, successful allocated bytes, releases/released bytes, live requested
bytes before/after, and peak live requested bytes above the stage baseline.
`alloc_zeroed` is one request. A successful `realloc` is one request plus one
old-size release and new-size allocation even when in place. Failed requests do
not change live bytes. Counter overflow/underflow invalidates a sample.

The allocator observes Rust global-allocator calls across this process, not all
native memory interfaces. Input reads/hashing, executor startup, output, direct
native allocator calls, JIT executable mappings, allocator metadata/size rounding,
and WASI host allocations are outside these phase counts. Hidden physical overlap
during realloc is unavailable. Live bytes are requested sizes, not RSS or Java
payload-only accounting; zero payload/catalog charge does not imply zero global
live bytes. The current-thread driver places sequential boundaries, but snapshots
of multiple counters are not cross-thread atomic transactions. Any background
allocator traffic is included. No wall-clock values are produced.

`check_allocation_capture.py` runs only `--check` correctness cases using frozen
HotSpot oracle rows and tests invalid checksum/admission rejection. The explicit
allocator unit test checks known allocation, zeroing, realloc and release
semantics; it is not a workload allocation sample.
