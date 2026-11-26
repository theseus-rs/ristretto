# Explicit Java execution-window evidence

`experimental-execution-measurement` adds an opt-in native Linux diagnostic to
`gc_compare`. It binds execution-counter snapshots to the actual
`GcTelemetry.beginMeasurement()` / `endMeasurement()` calls used by `GcBench`.
The returned timestamps are the same monotonic timestamps exported by the
publication pause recorder and used by Java for `elapsed_ns`. The initial
snapshot precedes the begin timestamp; the final snapshot follows the end
timestamp. Each boundary exports its enclosing observer and snapshot intervals.

This closes the missing **measurement-window counter producer**. It does not
complete J0/J5 coverage, qualify throughput/latency, or establish zero GC pauses.
The existing main-invocation and lifetime packets remain distinct. The default
VM feature set and unarmed behavior are unchanged.

Run from a clean source tree, using source-specific build storage outside it:

```sh
python3 benchmarks/gc/build_execution_measurement.py \
  --source "$PWD" --target-dir /absolute/build/execution-window \
  --output /absolute/evidence/execution-window-build
python3 benchmarks/gc/run_execution_measurement.py \
  --build /absolute/evidence/execution-window-build \
  --output /absolute/evidence/execution-window-probe --jdk /absolute/jdk \
  --fixture probe --policy candidate-jit --require-compiled
python3 benchmarks/gc/execution_measurement_artifacts.py \
  /absolute/evidence/execution-window-probe
EXECUTION_WINDOW_CAPTURE=/absolute/evidence/execution-window-probe \
  python3 -m unittest discover -s benchmarks/gc -p test_execution_measurement.py -v
```

`--fixture benchmark --workload allocation` runs the unchanged ordinary
`GcBench`; `control`, `roots` and `mixed` are also supported. Durations default
to 100 ms plus 100 ms warmup and are only finite functional diagnostics. The
checker reconciles measured operation entries, histogram entries and (for the
roots fixture) node constructor entries against completed Java work. It reports
the observed rejection reason, bytecode offset and opcode when available. Each
compiled roots operation must take exactly 32 backward branches, one for each
of 16 node constructions and 16 visits. The control and array paths have zero.
The oracle checks mixed interpreter/compiled dispatch against the number of
compiled calls. `--require-compiled` rejects fallback and compilation that
was not ready at the start. It never treats a compiled counter as CPU share.

The separate count-bounded probe uses ordinary VM dispatch and the same Java
boundary helper. It executes `kernel(5)` 9 times before, 7 inside and 11 after
measurement, with a bounded one-second compiler settling opportunity before
begin (actual readiness is checked, never inferred from the delay). It also executes `outside` only before/after measurement. The
independent oracle requires checksums 135/70/176; exactly 7 measured kernel calls
and zero outside calls; and, when wholly compiled, exactly 35 taken backedges.
This is coverage ground truth, not a performance workload or GC exposure proof.

The launcher preserves build/source archives, exact javac inputs/class hashes,
JDK executable/modules hashes, admission, actual Popen PID and PID-preserving
exec receipt, launch arguments/watchdog, raw output and failures. The offline
checker validates all of these and requires exact execution/pause/Java timestamp
agreement. The same JDK bytes must remain available at the recorded path for
replay. The runner preserves failed/overflowed captures and does not retry.
Source archives contain the declared git source inventory, including untracked
source files, and are verified before use. These are local custody receipts,
not a cryptographic attestation of the compiler or machine.

Recorder storage after arming is bounded by explicit method, per-method function,
identity/error string and retained generated-code caps (defaults 4096 methods,
4 functions, 4096 string bytes and 64 MiB code). Exceeding a cap records evidence
loss and makes the checker fail. Startup records are checked before arming.
Copied snapshots and method owners still have allocator/VM metadata overhead;
these limits are not a claim about total RSS. No per-operation snapshots or
new global waits are added. Boundary ownership uses `try_lock` and competing
boundaries fail. The existing execution profiler uses a mutex and first-seen
allocations; compiled activity counters are sampled separately. Snapshots can
wait on that existing mutex. Snapshot costs are reported, but their effects on
caches/scheduling and existing hot-path instrumentation overhead are unmeasured.

There is deliberately **no atomic VM-wide cut**. The boundary's Java thread is
identified, but concurrent VM actors and queued compilation are not stopped or
awaited. Other actors can execute between a per-method counter read and the
returned timestamp. Exact count assertions are grounded in the single-caller
fixture; they are not a general multi-mutator coverage proof. Only one window
per VM is accepted. Duplicate, reversed, foreign-thread, missing-end and
contended boundaries cannot produce valid evidence.

Still open: complete hot-loop/dependency and actor coverage, observer overhead
calibration and CPU attribution, allocation-recovery/safepoint/global-equivalent
stall coverage, held-out multi-mutator progress, and sustained scheduled-load
and tail qualification. Publication-only pause captures retain
`global_source_coverage_complete=false`; absent events do not imply zero STW.
