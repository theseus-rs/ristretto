# Bounded canonical root scanning

This historical experiment can be reproduced from the [archived source](https://github.com/theseus-rs/ristretto/blob/0497123e05056546bc526ecc49c081f934f38d64/ristretto_gc). The current Java preview uses the [unified JVM heap](../GC_PREVIEW.md).

The internal leaf helper uses `RootCommand::ScanBatch`. Each preparation examines
at most eight consecutive root slots and stops at the first root or target effect.
The original `Scan` / `begin_scan_root` operations still examine exactly one slot.
The public `Gc` backend and defaults are unchanged.

The batch uses the same barrier and controller commit as a single-slot scan. Empty,
retired, unpublished or already-covered roots may need no effect. The first changed
membership or root state ends the batch; the existing preparation retains at most
one of each. The final cursor and those effects become visible through the same
controller decision and certificate. No new pooled storage, owning-wrapper field,
progress reserve or allocation is introduced.

Each skipped observation is conditional on the original controller snapshot.
Canonical publication, removal, reference-root activation and phase changes make an
older proposal retry. A root announced in a skipped location is still unpublished;
its eventual activation must execute the existing publication/shading barrier, even
if the cursor has already passed. Root Drop posts intent; it does not independently
change canonical visibility. An already-covered live root keeps its required work.

An unsuccessful owning read anywhere in the batch discards the entire private
preparation. It cannot advance the cursor through an unobserved slot. Cancellation
preserves the old cursor, and a superseded preparation cannot refresh its expected
controller. Reserved construction and notification root roles use the same existing
barrier/custody rules. The batched cursor cannot pass an unfinished membership effect;
a second helper can complete that effect while the original helper remains stopped.

Eight slot observations are a fixed work bound, not a wall-clock guarantee. Pool
contention can cause retries. Host allocation/destruction and arbitrary callbacks
retain their existing limitations; this change does not establish scheduling
fairness, complete graph collection or bounded callback time.

The frozen validation matrix passes 27 commands, including native/WASI, both Miri
modes, sanitizers and release builds. Eight new portable fixtures, four new
exact-source concurrency models and deliberate broken variants cover the bound,
effect stopping, late publication, cancellation, failed discovery and reserved
custody. The current model runner executes 59 catalog and six pool models plus
32 fault controls. Separate native/WASI faults verify all four new mistakes.

The 252-process paired lifecycle screen measures about 18–21% lower canonical
lifecycle time and 24–27% lower cleanup time. All 36 canonical allocator rounds
match the prior build's requests, bytes, frees and peak requested storage exactly.
This improves the internal prototype; it remains substantially slower than legacy
in this reference-free workload. General graph/callback/admission integration,
JIT and whole-GC/G1/ZGC/endurance qualification remain open. See
[the measured evidence](../target/gc-implementation/canonical-cleanup-REPORT.md).
