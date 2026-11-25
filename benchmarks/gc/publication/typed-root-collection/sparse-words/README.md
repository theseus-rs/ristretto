# Collection time with sparse bitmap scans

Full collections of a live graph improved 2.08–2.13× on cooperative native and 1.95–2.05× on cooperative WASI against the original collector. On a mixed heap with two native background workers the speedup was 1.20–1.23×. The 3× full-collection target remains open.

| Execution | Live tree | Live wide graph | Tree with fresh garbage |
|---|---:|---:|---:|
| Native, cooperative | 2.08–2.09× | 2.10–2.13× | 1.46–1.48× |
| Native, two workers and caller | 1.75–1.76× | 1.83–1.89× | 1.20–1.23× |
| WASI, cooperative | 2.04–2.05× | 1.95–1.98× | 1.43–1.46× |

Values are median paired speedups with diagnostics disabled. Ranges span gated and counted publication. Under counted publication the median cooperative native tree cycle went from 1.85 ms to 0.89 ms, and the WASI tree cycle went from 1.43 ms to 0.69 ms. The counted mixed-heap cycle with two native workers went from 4.17 ms to 3.49 ms.

[Native graph](native.svg) · [WASI graph](wasm.svg) · [864 process measurements](processes.csv) · [Medians and exploratory intervals](summary.json) · [Build and runner identities](provenance.json)

A separate profile of the preceding native candidate sampled a cooperative mixed heap. Of 284 samples inside the collection path, 128 contained the sweep selection function, 38 contained object-page unlinking and 38 contained nursery-page unlinking. Inclusive stack counts may overlap. The new bitmap records which 64-bit occupancy words a page has ever used. Scans skip words outside that set and consult the current occupancy bits in each selected word.

A three-thread Loom test models a publisher that has set the summary, a second publisher that reuses the word, and a later scan. The ordered handoff preserves visibility in the model. A negative control removing the acquire handoff failed on a stale summary. [Publication model](../../../../../ristretto_gc/tests/admission_model.rs)

The comparison uses the same 4,095-node fixture and timing boundary as the preceding report. Both arms verified all 65,520 marks and all 16 full cycles per process. Each mixed process reclaimed 16,368 objects and 130,944 bytes. There were 12 independent pairs in each of 36 cells. The baseline was `f1daebeb277323fa0d5e732ae3853db6b97eac8b` with the common fixture. The candidate was `9a7536d9f33d7067dbf01d789852417943c9a90e`. The follow-up for already-strong typed roots during active epochs is on a separate review branch.

An additional 384 process runs isolated the bitmap change against `46ef6e26`. On cooperative native trees it reduced median paired elapsed time by 10–13%, and on cooperative WASI trees by 10–16%. Native mixed heaps with two workers showed no consistent incremental gain. The baseline comparisons in the table were measured directly in a fresh matrix. [Incremental timings](incremental.csv)

Use the reproduction commands in the preceding report, substituting `9a7536d9f33d7067dbf01d789852417943c9a90e` for the candidate revision. CPU sets, graph shapes, warmup, seed and pair counts are identical for the baseline comparison. [Method and commands](../README.md)
