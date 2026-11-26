# Controlled leaf contention

## Current contention results

The frozen v6 recovery matrix finished all 72 scheduled processes. Native
threads passed 21 of 24, cooperative native passed 24 of 24 and cooperative
WASI passed 24 of 24. The 69 completed processes verified 4,710,000 payload
destructions, the same number of object and root slot releases, and full
logical-credit refunds. Each process exercised 10,000 allocations per mutator.

All three baseline failures occurred with eight native mutators, one helper, 24 slots
and two retained roots per mutator. One allocation reached the one-million-poll
watchdog in each run. All three repeats with four helpers passed under the same
capacity and retention. The cooperative results demonstrate progress with
explicitly interleaved maintenance. Native single-helper progress remains open.
[Verified matrix](../../ristretto-gc-evidence/qualification-contention-v1/verified-matrix.json)
· [Native thread report](../../ristretto-gc-evidence/qualification-contention-v1/runs/native-threaded/REPORT.md)
· [Native cooperative graph](../../ristretto-gc-evidence/qualification-contention-v1/runs/native-cooperative/progress.svg)
· [WASI cooperative graph](../../ristretto-gc-evidence/qualification-contention-v1/runs/wasi-cooperative/progress.svg)

The first candidate published a numeric hint after a helper successfully clears
a retired object slot. A colliding allocation can try that slot on its next
poll. It still acquires ownership through the catalog's original checks and
compare-and-exchange. Every second collision at the latest advances the
ordinary scan, including when hints are stale or change continuously.

| Execution mode | Baseline passes | Numeric hint | Expiring slot preference |
|---|---:|---:|---:|
| Native threads | 21/24 | 23/24 | 24/24 |
| Cooperative native | 24/24 | 24/24 | 24/24 |
| Cooperative WASI | 24/24 | 24/24 | 24/24 |

The candidate used the same frozen workload, schedule, affinity and poll
limits. It verified 4,870,000 destructions across the 71 completed processes.
All three native single-helper cases passed. One native case with four helpers,
520 slots and 64 retained roots per mutator exhausted its allocation poll
budget. Development work could run concurrently during the candidate screen.
A separate instrumented run also reproduced a single-helper stall. Native
allocation fairness remains open. These runs measure completion and accounting.
[Candidate matrix](../../ristretto-gc-evidence/leaf-vacancy-screen-v1/verified-matrix.json)
· [Native thread report](../../ristretto-gc-evidence/leaf-vacancy-screen-v1/runs/native-threaded/REPORT.md)
· [Native cooperative graph](../../ristretto-gc-evidence/leaf-vacancy-screen-v1/runs/native-cooperative/progress.svg)
· [WASI cooperative graph](../../ristretto-gc-evidence/leaf-vacancy-screen-v1/runs/wasi-cooperative/progress.svg)

The next candidate lets a long-waiting allocation prepare its private backing
while other objects occupy its probe slots. After releasing a retired object,
the helper can offer one slot to that request's unique creation identity.
Competing automatic announcements can defer only a bounded number of times on
the offered slot. Cancellation withdraws the preference. The announcement
still acquires the slot with the existing compare-and-exchange.

All 72 processes passed on the original inputs and limits, including every
repeat of the native eight-mutator, single-helper and high-retention cases.
The checker verified all 4,950,000 payload destructions, object and root slot
releases, per-mutator progress and full payload and catalog refunds. A separate
single-slot regression exercises both a stopped nominee and a cancelled one.
These finite screens cover the leaf allocator. General graph and callback
progress, sustained load and pauses remain open.
[Verified matrix](../../ristretto-gc-evidence/leaf-preferred-screen-v1/verified-matrix.json)
· [Native thread graph](../../ristretto-gc-evidence/leaf-preferred-screen-v1/runs/native-threaded/progress.svg)
· [Native cooperative graph](../../ristretto-gc-evidence/leaf-preferred-screen-v1/runs/native-cooperative/progress.svg)
· [WASI cooperative graph](../../ristretto-gc-evidence/leaf-preferred-screen-v1/runs/wasi-cooperative/progress.svg).

The subsequent change to occupancy probes also passed all 72 schedules and
verified all 4,950,000 destructions. Its occupied-slot checks avoid acquiring
and releasing the object or root they merely skip. A separate counter run
measured this effect on native and WASI.
[Verified contention rerun](../../ristretto-gc-evidence/leaf-occupancy-screen-v1/verified-matrix.json)
and [allocation probe counts](../benchmarks/gc/CANONICAL_MEASUREMENTS.md#peer-comparison-with-a-persistent-java-heap).

## Driver and historical results

The `experimental-canonical-leaves` feature includes `leaf_contention`, a finite
stress diagnostic over the internal typed runtime. It exercises real allocations,
root retention and reads, automatic slot selection, concurrent/cooperative helpers,
actual payload destruction and complete teardown. It does not select a public GC
backend or change default allocation.

```
cargo run -p ristretto_gc --release --features experimental-canonical-leaves \
  --example leaf_contention -- native 8 4 10000 24 2
```

Arguments are mode, mutator count, helper count, operations per mutator, object/root
slot capacity and retained roots per mutator. Mode is `native` or `cooperative`;
Wasm accepts only cooperative mode. Bounds are 1–16 mutators, 1–8 helpers,
1–10,000,000 operations each, 1–16,384 slots and 0–64 retained roots. Capacity must
be at least `mutators * (retained + 1)`. Inputs are rejected before setup.

The payload contains actor/sequence identity, a canary and an independent atomic
Drop counter. It has no managed edges and its destructor performs bounded counter
bookkeeping. Each actor evicts its oldest retained root before replacement, checks
the newly returned value, and rereads every retained root after each successful
allocation. A zero-length ring drops the new root immediately after its read.
A matching checksum alone is insufficient: each read verifies every identity word,
and every actor must finish its own requested work.

Native actors use separate preregistered mutator/helper wrappers on scoped threads.
A start gate aligns their launch; creation unwind releases the gate with failure.
Exit guards cancel peers on errors or panics, and all threads are joined. Native
mutators yield every 32 pending allocation polls; helpers yield every 32 polls.
Cooperative mode retains all requests in a round, interleaves mutator and helper
polls, then releases the request borrows before the next round. The driver needs no
clock, but it deliberately runs a whole finite workload in one invocation: this is
not a browser host-turn scheduling interface.

Each request has a diagnostic watchdog of `max(1,000,000, capacity * 1024)`
polls, recorded as `request_poll_limit` in schema version 2. Helper draining also has
a finite workload-scaled watchdog, and the Python runner has a process timeout.
These are failure detectors, not proven protocol progress or wall-time bounds.
Full arrays remain pending while helpers reclaim dead roots/objects; no value is
silently dropped to make the workload appear complete.

The JSON report records each actor's completed operations, allocation polls, largest
request poll count, reads and checksum; each helper's polls and root/object releases;
actual destruction count; logical catalog reservation; logical object size; and
payload/catalog credits after teardown. Success additionally requires all runtime
participants to unregister and the independent Drop counter to have exactly its
one external owner. Host allocator overhead, diagnostic vectors/threads and RSS
are not represented by the logical-credit fields.

`benchmarks/gc/leaf_contention.py` freezes binary/source/build provenance, input
cases, CPU affinity, seed and schedule. It saves every exit, timeout and raw report,
then independently checks exact schema, integers, per-actor work/checksums/read
counts, physical release totals and teardown. Its report revalidates the frozen
hashes and complete schedule; a failed process cannot be omitted from an all-success
graph. CSV and SVG/PNG plots show poll counts. External elapsed time includes
startup, fixture checks and teardown and serves only as watchdog evidence.

This diagnostic establishes finite controlled-leaf composition evidence. It is not
an infinite-service/fairness proof, opaque callback test, mutable cyclic graph or
VM integration test, pause/latency benchmark, G1/ZGC comparison, or completion of
the plan's capacity/hour/day sustained-load gates. The catalog transition protocol
and existing allocation/root/maintenance algorithms are unchanged by this delivery.

The initial fixed one-million-poll watchdog failed all three 16,384-slot native
runs. A separately recorded ten-million-limit diagnostic completed all three,
with largest requests requiring 2,621,837–2,982,099 polls. The capacity-scaled
allowance corrects the diagnostic window; it does not accelerate reclamation or
qualify allocation latency. Original failures and their raw reports are retained.
Native error reporting now preserves the triggering error across peer cancellation.

An earlier matrix recorded 78 completed processes and 31.11 million
allocations/destructions. Its raw artifacts were lost with the workspace
target directory. The recovery matrix above has its own frozen cases and
durable per-process results.
