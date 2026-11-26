# Release the first GC preview through stacked MRs

This document records the earlier restricted source preview. The [alpha release plan](PLAN.md#alpha-collector-and-review-gates) governs the shipping JVM.


The release is an **opt-in source prerelease for restricted Java programs**. A user should be able to build from the archive, run the same Java 17 classfiles on the native interpreter, native compiled tier and WASI interpreter, and observe cyclic reclamation. The example must return **64**, allocate **16** objects, leave **zero** payload bytes after cleanup and **zero** catalog bytes after drop. Its compiled execution must report **25** compiled entries and **8** compiled backedges. The rest of the supported Java surface is also part of the milestone. [Java contract](../ristretto_vm/CANONICAL_JAVA.md) · [Runnable example](../ristretto_vm/examples/canonical_java.rs)

## Source and merge destination

Use base commit `59d46079a08f1d623b509d96af1f3a65085c8349` and the frozen Git tree `4e8b7e5ca04eba08950ab59ac94a2c3870d756bf` as the porting reference. The reference has 2,677 source files. Its diff changes 840 paths, with 185,846 insertions and 5,677 deletions. In particular, `collector.rs` adds 5,843 lines and removes 874. Moving that file wholesale would bring unrelated behavior into a single review. Start from a clean checkout of the base and keep the staged reference intact. [Frozen review inventory](../../ristretto-gc-evidence/restricted-runtime-preview-v1/REVIEW.md)

I recommend a dedicated `gc-preview/1` integration branch based on that commit. The first MR targets this branch. Each dependent MR targets its immediate predecessor until that predecessor lands. The preview tag goes on the integrated, tested commit. This fits the source release because the frozen optional catalog and pool use path dependencies. The current `cargo package` invocation fails on the catalog's missing version requirement. Merging that wiring into a branch used for ordinary crate publication needs its own packaging solution and verification. [Packaging result](../../ristretto-gc-evidence/restricted-runtime-preview-v1/cargo-package.log)

**Port the supported behavior and its tests from the frozen tree.** Keep a disposition for each group in the 840-path diff in the first MR description: the MR that carries it, the existing base implementation that satisfies it, or the later milestone that owns it. Track mixed files by symbol or hunk. For example, split `collector.rs` by reservation, publication and cleanup, `canonical.rs` by admitted Java operation, and `canonical_compiled.rs` by native request. Give an ordinary GC, classloader, types or JIT prerequisite its own tested MR when a preview slice actually needs it. This keeps the source reviewable and makes omitted changes visible.

## Proposed MR stack

Each ID below is a proposed MR with a runnable acceptance check. Within each table, land rows top to bottom unless the dependency column gives a separate base. Target roughly **150–400 hand-written implementation lines per review** as a planning guide. Count tests and fixtures separately and show them in the diffstat. When a row grows into two independently testable changes, split it before opening the MR. Include the regression, any required manifest or lockfile hunk, and the implementation together.

### Establish a Java oracle and a working Rust collector

After C01, C02–C03 and C04–C05 are independently reviewable. C06 takes both landed tips. C07–C12 form a linear stack from C06.

| MR | Reviewable change | Proof at this tip |
|---|---|---|
| C01 | Check in the cycle fixture, its reproducible Java 17 classfiles and a small oracle driver. | A stock JVM executes `CanonicalGraphWorkload.run(8)` and returns 64. Regeneration reproduces the classfiles. |
| C02 | Reserve managed payload bytes before allocation and refund the owning reservation on failure. | An over-budget allocation fails cleanly. A successful value releases its charge exactly once. |
| C03 | Give private construction one owner through publication or cancellation. Port the required descriptor and trace-layout checks here or as individual prerequisites. | A cancelled constructor drops its payload once and returns child protection and budget. Ordinary rooted allocation still passes. |
| C04 | Introduce the bounded slot pool with a checked generation on reuse. | A retained observer cannot acquire the replacement value. Exhaustion and reuse run on native and WASI. |
| C05 | Publish or replace one retained pool link conditionally. | Two competing proposals have one winner. A stale or foreign proposal preserves the installed value. Include the corresponding concurrency model. |
| C06 | Register bounded mutator/helper participants and reserve catalog backing. | Exceed each quota, drop a participant, and reacquire its capacity. A failed reservation restores its charge. |
| C07a | Commit one object-record request through the catalog control state. | A second helper finishes a pending request and an old attempt cannot commit twice. |
| C07b | Cancel an object-record request across the selection boundary. | Drop the creator before and after selection. The exact payload is returned or released once, and another request can use the quota. |
| C08 | Register a logical root and acquire a typed view of a certified leaf. Wire the optional pool/catalog dependency with its first GC consumer. | A crate-level Rust test reads a scalar. Wrong heap, wrong type, finalizer or unsupported layout is rejected. Default launcher dependency check stays green. |
| C09 | Trace and retire an unrooted leaf incrementally. | A live root survives a helper pass. After its last root drops, the payload is reclaimed and its slot becomes reusable. Include the stalled-reader and drop-order checks. |
| C10 | Install and clear one strong graph edge, including the write barrier. | Replacing an edge during marking preserves the reachable child. A stale write cannot change a reused slot. |
| C11 | Read a child into an independent root. | A child remains readable after unlinking. Null, cancelled and wrong-heap reads leave no new root. |
| C12 | Complete incremental graph traversal and cyclic reclamation through the public Rust API. | Two rooted nodes form a cycle. Removing the last external root lets helper polls reclaim both. Repeat at finite capacity and verify payload and catalog refunds. |

Extract the pool and catalog from `ristretto_gc/validation/catalog/catalog-tagged{,-pool}-prototype`. The GC adapter lives in `publication_owner.rs`, `canonical_leaf*.rs`, `canonical_graph*.rs` and `canonical_api.rs`. Review ownership transfer, memory ordering and reclamation with the relevant model or Miri witness in the MR that introduces the invariant. **C12 is the first useful internal checkpoint** and lets a reviewer evaluate the collector independently of the JVM. [GC caller contract](CANONICAL_API.md) · [Public Rust API tests](tests/canonical_api.rs)

### Run the published Java subset on native and WASI

Start V01 on C12. The two targets use the same admitted classfiles. Expose each opcode family only once its execution and rejection paths work.

| MR | Reviewable change | Proof at this tip |
|---|---|---|
| V01 | Admit and link a closed class set, then execute a scalar static entry using the existing verifier. | A real classfile returns the expected scalar. A missing class, malformed method or unsupported opcode fails before heap creation. |
| V02a | Execute integer locals, arithmetic and branches. | A scalar loop agrees with the Java oracle. Invalid stack shapes fail admission. |
| V02b | Add the supported wider scalar locals and conversions. | Boundary inputs cover narrowing, category-two locals and return values. |
| V03 | Allocate a Java instance, invoke its constructor and access an integer field. | A tiny class initializes and reads a field. A faulting or cancelled call releases its frame roots. |
| V04a | Implement reference fields and identity checks. | A local retains a child across a field unlink. Two distinct allocations compare unequal. |
| V04b | Transfer reference arguments and results through static calls. Expose the interpreter example here. | The C01 fixture returns 64 and reclaims all 16 objects on native and WASI. An unlinked child survives through a call. |
| V05 | Reuse a bounded heap across invocations and detach a thread cleanly. | Run two entries on one heap. A parked thread keeps its graph while another reuses slots. Verify fault, cancellation and final drain. |
| V06 | Support static integer/reference fields and their heap-owned roots. | Two invocations observe the same static value. Replacing a static reference releases the old graph. |
| V07 | Add supported class initialization and handoff across attached contexts. | Initialization executes once. A waiting or cancelled context preserves ordering and can release its roots. |
| V08 | Add one-dimensional primitive arrays. | Run the eight admitted element kinds with load/store, narrowing, null and bounds cases on both targets. Slice by element family if needed. |
| V09 | Add one-dimensional reference arrays and their checked element edges. | Store, load and clear a child. A wrong-component store preserves the prior element and heap accounting. |
| V10 | Add the supported closed-layout type checks and finish the admission matrix. | Positive, null and failing cast cases agree with the contract. General dispatch, monitors, native calls, exception handlers and unsupported layouts are rejected. |

**V04b is the first end-to-end demonstration.** V05–V10 are part of the published preview contract even though the demonstration is already green. Use the existing `canonical_java`, `canonical_heap`, `canonical_statics`, `canonical_static_handoff`, array and cast tests as the source of the individual MRs. [Interpreter contract and fixtures](../ristretto_vm/CANONICAL_JAVA.md)

### Run the same program through the native JIT

J01 can be reviewed against the base after C01. It has its own runnable JIT test. Land or rebase it onto the preview integration branch before J02. Base J02 on the integrated interpreter and J01. Keep subsequent JIT work in a single dependent stack.

| MR | Reviewable change | Proof at this tip |
|---|---|---|
| J01 | Return a typed request or completed scalar from a native activation under the experimental dispatch feature. | A compiled scalar method yields and resumes with the expected result. Existing ordinary JIT callers pass their tests. |
| J02 | Map live references in native frames to canonical root tokens across suspension. | A live reference survives a backedge. Frame teardown releases it. Foreign, expired and out-of-range tokens are rejected. |
| J03 | Lower allocation and supported constructor requests using that frame ABI. | Compiled construction works under a small heap. Cancellation during the request releases frame and allocation owners. |
| J04 | Lower scalar and reference field operations through the runtime. | A compiled store, load and unlink preserve object identity and barriers through a collection. |
| J05 | Implement supported static calls, returns and backedges; add `--compiled` to the example. | The C01 fixture gives **64 / 16 / 25 / 8** for result, allocations, native entries and backedges, with complete cleanup. |
| J06a | Lower supported scalar conversions and narrow returns. | Compare boundary results with V02b, including category-two arguments and locals. |
| J06b | Lower primitive array operations. | Compare loads, stores, bounds and null failures with V08. |
| J07 | Lower reference-array and checked-cast requests. | Native and interpreter outcomes agree for element identity, null, wrong component and failed casts. |
| J08 | Route compiled static state and initialization through the shared Java heap. | A value written in one tier is read in the other. A suspended initializer preserves its roots and resumes once. |

Use `ristretto_jit`'s activation and request interfaces, `canonical_frame_table.rs` and `canonical_compiled.rs` as the extraction boundaries. Reviewers can reproduce every JIT row on native. The WASI lane continues to run the interpreter and build the supported feature configuration. [Compiled execution contract](../ristretto_vm/CANONICAL_JAVA.md) · [Compiled static-state cases](../ristretto_vm/CANONICAL_COMPILED_STATICS.md)

### Package, compare and release

| MR | Base | Deliverable and acceptance |
|---|---|---|
| R01 | C01 | Package the Java ring fixture and a finite G1/ZGC baseline runner. Both collectors execute identical bytecode with the same input, checksum, CPU and process-memory limits. Record the JDK, flags, warmup and heap settings. |
| R02a | Integrated J08 + R01 | Run the same finite fixture on the reviewed native/WASI binaries. Preserve raw per-process measurements, source and executable hashes, arrival settings and heap accounting. Reject a checksum or resource mismatch. |
| R02b | R02a | Reproduce the table and graph from the saved captures. Document differences between Ristretto's object-slot limit and HotSpot's heap-byte limit. Make per-process values available alongside the figure. |
| R03 | R02b | Produce the deterministic **source** archive, checksum, reader guide and CI smoke. Extract the archive into a clean checkout and run the release checks below before tagging. |

The historical G1/ZGC pilot may be included with its original revision, duration and limits. Report any measurements attributed to this release from the reviewed tip. The reader guide must disclose the known stalled-publication case in the ordinary collector. Zero global stops, throughput or tail-latency superiority, a default collector switch and one-hour/24-hour endurance qualification belong to subsequent milestones. Keep post-freeze optimizations in their own series. [Existing measurements and release gates](../benchmarks/gc/CANONICAL_MEASUREMENTS.md#remaining-release-gates)

## How to run the reviews

1. Open **C01 first**. Put the release contract, base and frozen tree in its MR description. Make the first independent review batches **C02 → C03** and **C04 → C05**. Rebase C04 onto the integrated preview line before C06 combines the two batches. The same pattern allows J01 and R01 to be reviewed early.
2. Give each MR a parent MR/commit, an exact test command, one observable result and a short explanation of any unsafe or concurrent invariant. Show `git diff <parent>...HEAD`. Reviewers should be able to check out the single MR tip and run its tests without a child branch. A missing cross-crate dependency becomes a small prerequisite with its own consumer or reproducer.
3. Merge bottom-up. After a parent merges or is squashed, replay the child's own commits on the resulting base, retarget the MR and compare the old and new patches with `git range-diff`. Re-run relevant CI on the new tip. If a slice regresses after landing, revert it and its dependent slices in reverse order, leaving the last green checkpoint usable.
4. Run native tests for the touched crates and execute WASI tests when the change affects shared code. Add strict lint, the applicable concurrency model or memory check, and a downstream smoke at the first MR that needs them. For common-path changes, run the ordinary caller regressions with the feature off. Ship fixture source and classfile together and preserve generated-file provenance.

Reviewers can use these commands as the corresponding targets land. Put the exact smaller test selector for each MR in its description.

```sh
# C04–C05, then the external Rust caller at C12
cargo test --locked --manifest-path ristretto_gc/validation/catalog/catalog-tagged-pool-prototype/Cargo.toml --lib
cargo test --locked -p ristretto_gc --features experimental-canonical-runtime --test canonical_api

# V04b and J05, running the same classfiles
cargo run --locked -p ristretto_vm --no-default-features --features experimental-canonical-runtime --example canonical_java
cargo run --locked -p ristretto_vm --no-default-features --features experimental-canonical-jit --example canonical_java -- --compiled

# WASI interpreter, on a host with the pinned target and Wasmtime installed
export CARGO_TARGET_WASM32_WASIP2_RUNNER='wasmtime run -W max-wasm-stack=8388608'
cargo run --locked -p ristretto_vm --target wasm32-wasip2 --no-default-features --features experimental-canonical-runtime --example canonical_java
```

## Final release checks

- Run the documented commands from the extracted archive with the pinned toolchain, `--locked`, and a fresh Cargo target. Exercise native interpreted, native compiled and `wasm32-wasip2` interpreted execution with Wasmtime. Check the outputs and the two zero-byte conditions explicitly. The frozen reference used Rust 1.99.1 and Wasmtime 50.0.1. [Preview commands](../../ristretto-gc-evidence/restricted-runtime-preview-v1/PREVIEW.md)
- Run the full supported Java suites and ordinary GC regression suites on their supported targets. The reference archive passed the 13 `canonical_java` cases per target and 284 native / 189 WASI ordinary GC library tests. Compare test **names and results** with the reference and explain any removed or inapplicable case. Use `cargo tree --locked -e normal -p ristretto_java` on native and WASI to verify default dependency isolation, then check that enabling the preview feature includes its catalog. [Reference receipts](../../ristretto-gc-evidence/restricted-runtime-preview-v1/milestone.json)
- Reconcile all source dispositions against the release contract, including the public Rust API, Java admission surface, native compiled cases, WASI interpreter, resource limits and comparison artifacts. Build the archive twice from the reviewed commit. Verify identical hashes, file contents and executable modes, and make the evidence downloadable with the release.
- The old archive's SHA-256 is `35479deb93b6d5ca7d2f247ede837fc5d1ddca4761ab4e5d72e33f5e7233fb0e`. A curated stack gets its own source manifest, test receipts, archive and checksum. After maintainer approval, tag the tested tip with a dedicated prerelease tag such as `gc-preview-1` and attach the verified archive and `SHA256SUMS`. That tag falls outside the current version-shaped binary-release trigger. [Release workflow](../.github/workflows/release.yml)
