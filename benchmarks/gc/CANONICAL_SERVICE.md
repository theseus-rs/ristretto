# Canonical service evidence

`service_capture/` is a separate diagnostic Rust workspace. It runs ordinary admitted classfiles through `Program::run_recorded` or its actual compiled equivalent, then streams one `RISTRETTO_CANONICAL_SERVICE` JSON packet. The recorded window is `entry_body_v1`: after heap/frame-table setup, immediately before the entry call, ending after the call/return and before cleanup. It contains no inferred stdout or whole-run interval. This finite harness is not a sustainable-load benchmark.

`build_canonical_service.py` freezes every source file before building, copies the compiler-reported executable outside Cargo targets and rejects changed source. `run_canonical_service.py` copies the executable, exact classfiles, admission JSON and source evidence into a fresh run directory. A launcher writes its actual PID and command then uses exec. Native guest PID must equal that host PID. WASI's guest PID remains null; its host PID belongs to the exact launched Wasmtime binary and module command. The host executable is preserved for offline verification.

`canonical_service_artifacts.py RUN_DIRECTORY` independently verifies source-archive members, executable/source/admission/bytecode hashes, Popen/exec receipts and the JSON capture. It recomputes request containment, synchronous overlap, explicit window crossings and Pending gaps; it compares request/helper events and primitive-array preparation with VM counters. Parented helpers must carry their request operation; parentless helpers must carry maintenance. Each successful request must include exactly one matching preparation and a final completed poll. A producer completeness flag is insufficient. Duplicate keys, missing/unknown fields, unsupported sources, overflow, open/abandoned/failed events and mismatched counters reject validation. Raw files remain on failure.

Service elapsed time includes instrumentation and descheduling. Gaps can contain scheduler delay, observer work or uninstrumented runtime work. Neither is automatically a global GC pause. `released_objects` is not a completed-cycle counter. Global coverage and qualification remain false even after a finite capture validates. Owner release, native/frame publication, cancellation/teardown and global actor coverage remain missing.

Admission JSON must include schema_version=1, tier (`interpreted` or `compiled`), class, method, Java-int argument and expected_result, window_kind=`entry_body_v1`, recorder_capacity, require_compiled_backedges, the five catalog Configuration fields, and all limits: instructions, request_polls, cleanup_polls, frames, native_slices, array_bytes. The tool uses these exact bytes and values; the source/build identity binds the sealed runtime's other fixed policies. It does not silently substitute unbound execution caps.

The current fixed-capacity append-only recorder is suitable only for finite diagnostics. It does not scale to a 24-hour event stream. Capacity is charged as observer memory; the snapshot and export also have a bounded-by-capacity memory cost. Export serializes events individually so it does not retain a second full array of JSON objects. Exhaustion continues runtime work but invalidates capture. No overwrite, silent sampling, producer wait or implicit capacity growth is permitted.

The combined-source checkpoint in
`/home/brianheineman/development/ristretto-gc-evidence/integration-service-v1/`
passes native interpreted, native compiled and WASI interpreted captures for both
graph and primitive-array fixtures. All six match the expected result, release
every allocated object and leave zero payload/catalog charge. Eleven checker tests
pass. A capacity-four recording lets Java finish but is correctly rejected as
incomplete evidence. Build receipts bind the same 2,504-file source manifest;
executables, exact classfiles, admissions, launch receipts and raw packets are
retained outside Cargo build storage. This was a functional check alongside
development work. Recorded times are not qualified performance measurements,
completed-cycle counts remain unavailable, and global coverage remains false.

Schema version 2 adds `cycle_observation` for the completed primary full epochs
inside the actual `entry_body_v1` clock window. Four raw intervals bracket start
and end: before-start, after-start, before-end and after-end. The checker verifies
their event identity, unsigned 64-bit limits and feasible monotone ordering, then
independently derives lower/upper cycle bounds. A missing observation remains
unavailable. An exact `gc_cycles_completed` appears only when the two bounds agree;
`gc_cycle_scope` identifies primary full epochs, excluding the secondary domain's
initial cycle. The old schema remains readable with unavailable cycle evidence.
Neither schema interprets released objects as cycles or establishes full pause
coverage. These observer calls are confined to recorded runs and can release
temporary owners with destruction work; their overhead remains unqualified.

## Endurance telemetry design to implement later

1. Preallocate a fixed number of fixed-size segments under a frozen memory budget. A single actor may reserve/write a slot and attempt one segment handoff; if no segment is free it records irreversible evidence loss and continues runtime work. The mutator never waits for filesystem I/O or a consumer. Consumer CPU/RSS/I/O belongs to the observer's resource budget.
2. Persist ordered segments with run/source/admission/actor identity, monotonic epoch, sequence ranges, event counts and hash-chained checksums. A terminal manifest seals the stream only after consumers stop and every queued segment is accounted for. A missing segment, crash, unsealed tail, reuse-generation mismatch or unclosed operation makes the affected capture incomplete. Export/flush may occur outside measurement, but must not erase events crossing its boundaries.
3. Keep active-request reconstruction state bounded by admitted actor/request quotas. A request may span segments; its begin record, Pending transitions, synchronous service intervals and final/cancel state must remain linkable. Carry unresolved state explicitly between segments. Never interpret an unseen begin/end as zero duration.
4. Maintain fixed-size per-source operation counts, elapsed sums, maxima and a frozen histogram alongside streaming. Retain complete intervals for violations and boundary crossings; if the reserved violation buffer exhausts, mark evidence loss. Histograms and maxima can support bounded diagnostics but cannot replace source coverage, all-actor state, exact scheduled-tail semantics or required raw evidence without a separately validated acceptance contract. Missing source categories remain missing.
5. Measure observer overhead on a coordinated quiet host with paired instrumentation-disabled/enabled runs and equal resource envelopes. Do not assume atomic/streaming work is free. Freeze any accepted aggregation precision and storage budget before endurance runs. Three paired hours and a separate 24 hours remain future qualification work; this design and finite tests satisfy neither gate.
