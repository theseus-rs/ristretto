# GC evidence recovery after workspace build-directory removal

At 2026-11-15 22:23:58 UTC, the workspace's entire `target` directory was observed
absent. The cause is unestablished. The current source changes remain present.
Historical evidence had been kept under `target/gc-implementation`; its source
snapshots, executed binaries, raw benchmark samples, graphs, audit logs and verification
receipts are no longer available at those paths. No completed canonical-client
archive was written before removal. The preceding shared-root v2 archive had been
written and verified, but its stored copy was also under the removed directory.

The previously recorded outcomes remain historical observations. Their summaries
and hashes cannot substitute for missing raw evidence, and old links under target
must not be represented as currently available or newly verified. Nothing about this
incident qualifies a performance, pause, correctness or endurance gate. Do not invent
replacement raw samples or reclassify prior failures as passing.

A current-source recovery archive outside target preserves 2,721 existing files
selected from Git's tracked and unignored paths. Three tracked JIT instruction files
were already absent at snapshot time; the receipt records them without restoring or
changing unrelated work. The archive is a current recovery snapshot, not a recreation
of any earlier frozen baseline. Location:

`/home/brianheineman/development/ristretto-gc-evidence/recovery-20261115T222358Z/`

The snapshot manifest SHA-256 is
`31d5859a41a866fb1b75c110e1cb4d6531ba11c58b093a180f5acbb51a5fe494`;
archive SHA-256 is
`fc170709fdfd077e7ba078477bbf164daa88d105ee5b16edc5b5a3558a2a8d7c`.
The incident receipt, Git status, manifest and archive are separate files there.

Fresh source snapshots, results and executed binaries will be retained outside
Cargo build storage. Compiler caches will be kept in separate explicitly selected
target directories. Rebuild current native/WASI/bare-Wasm and memory-tool validation
before making new accepted-result claims. Reproduce current model/fault/accounting
and performance evidence as required by subsequent qualification; the old chained
baselines and their raw measurements are not recoverable from a narrative summary.
The full GC/VM and sustained-load goal remains open.

## Fresh current-source validation

The immutable `current-v1` snapshot contains 2,722 files, including the incident
notice. Its manifest SHA-256 is
`ac7fbdb450b7831cc886ee631c6b5a6f63051ce8ce63258e6908cabe376f0260`;
source archive SHA-256 is
`01836ea594d8140ef58a5741ee096f9ed85730aad3625b485a6872dc62513088`.
The evidence lives in `current-v1/` beneath the recovery directory above;
`summary.json` binds the accepted result receipts and records scope limits.

Fresh checks pass:

- 686 native tests and 379 WASI Preview 2 tests, with native/WASI Clippy,
  formatting, bare-Wasm compilation and 39 focused Python checks.
- All 95 affected canonical ownership fixtures under each of Miri strict
  provenance, Miri Tree Borrows, AddressSanitizer and ThreadSanitizer.
- All 73 corrected catalog models and six pool models. Mutation-anchor preflight
  passes; the full historical negative model matrix has not been rerun.
- One positive and four rejecting public-API compiler controls, plus actual
  native and bare-Wasm client execution with complete payload/credit cleanup.
- Final shared-root backing/refund-order audits on native, WASI, Miri, Tree
  Borrows and both sanitizers. Deliberately restoring ordinary `Arc` destruction
  reproduces the expected refund-before-free failure on native and WASI.

Failed harness attempts remain alongside their corrections: missing offline
dependencies, an external-crate lock mismatch, omitted proc-macro dependencies
in a direct compiler harness, and a mistakenly selected uninstalled WASI target.
They are not accepted compiler or runtime controls. The passing compiler harness
uses captured dependencies from the accepted frozen-source native build.

This restores evidence for the current component code. It does not recreate the
lost timing samples, allocation-performance samples, stress runs, peer comparisons
or graphs. Subsequent canonical-Java, JIT and pause-recorder work uses separate
source/evidence directories and is not covered by this snapshot. No full-goal
qualification gate is closed by these recovery checks.

The recovery evidence archive is `current-v1-evidence.tar.gz` in the recovery
directory: 914,844,191 bytes, SHA-256
`1f471d40873813226eadc69b46d2cb8d3088c5205b0b78da45e5881565e1db29`.
Its 892 members were independently streamed back and checked against the file
manifest; `current-v1-evidence-receipt.json` records that transport check. The
archive includes retained failures as well as accepted results. Archive integrity
does not imply acceptance of a failed control or performance qualification.
