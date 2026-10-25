# Playground toolchain notices

`scripts/licenses.mjs` includes these files in `runtime/THIRD_PARTY_LICENSES.txt`.
Cargo metadata does not describe the Rust standard library or the native libraries
bundled with the `wasm32-wasip2` target. These checked-in notices cover that gap
without requiring network access during notice generation.

`toolchain.json` records the Rust release and commit, upstream revisions, file
paths, and SHA-256 hashes. The generator checks the active compiler's release and
commit and verifies every file before including it. Missing files, changed files,
and an unreviewed Rust upgrade fail the build instead of silently omitting notices.

## Provenance and coverage

- Rust's `COPYRIGHT`, `LICENSE-MIT`, and `LICENSE-APACHE` come from
  [Rust 1.99.1, commit 58cf8f9c](https://github.com/rust-lang/rust/tree/58cf8f9c35b355199727627d07ae34a4e2846d67).
- That release's
  [WASI target build configuration](https://github.com/rust-lang/rust/blob/58cf8f9c35b355199727627d07ae34a4e2846d67/src/ci/docker/host-x86_64/dist-various-2/Dockerfile)
  selects wasi-sdk 33. The SDK's
  [wasi-libc submodule](https://github.com/WebAssembly/wasi-sdk/tree/wasi-sdk-33/src/wasi-libc)
  points to `161b3195fc2558d2b1ba3eb9ffae3b2b47407623`, and its
  [LLVM submodule](https://github.com/WebAssembly/wasi-sdk/tree/wasi-sdk-33/src/llvm-project)
  points to `4434dabb69916856b824f68a64b029c67175e532`.
- wasi-libc's top-level licenses and its cloudlibc, musl, and musl-fts notices are
  copied verbatim. `wasi-libc/dlmalloc/NOTICE` contains the opening public-domain
  dedication from `dlmalloc/src/malloc.c`, through the line before `* Version`,
  with a closing comment delimiter added.
- wasi-libc's
  [default allocator is dlmalloc](https://github.com/WebAssembly/wasi-libc/blob/161b3195fc2558d2b1ba3eb9ffae3b2b47407623/CMakeLists.txt),
  and the SDK's
  [sysroot build](https://github.com/WebAssembly/wasi-sdk/blob/wasi-sdk-33/cmake/wasi-sdk-sysroot.cmake)
  does not override it. The alternative emmalloc allocator is not covered here.
- The complete LLVM compiler-rt and libunwind license files are included
  conservatively, as is musl-fts. Inclusion does not assert that every component
  survives linking into the playground's WebAssembly module.

## Updating the Rust toolchain

1. Run `rustc --version --verbose` from the repository root. Record `release` and
   `commit-hash` in `toolchain.json` when updating `rust-toolchain.toml`.
2. At that Rust commit, check the WASI target build configuration for its SDK
   version. Resolve the SDK's wasi-libc and LLVM submodules to full commit hashes;
   do not assume the SDK installed on the build host produced Rust's target.
3. Download each listed upstream file at its recorded revision, preserving its
   bytes. Refresh the dlmalloc notice excerpt as described above. Review the
   upstream license inventories and allocator selection for new components, and
   update this provenance section and the component names and revisions.
4. Update each file's SHA-256 hash (`shasum -a 256 <file>`), then run
   `npm run test:unit` and `npm run build:runtime` from `web`. Inspect the generated
   notice file for the Rust, wasi-libc, cloudlibc, and musl notices.

The upstream license files are not inputs to Prettier. Keep them verbatim.
When using `build:runtime -- --skip-rust`, the existing WebAssembly binary must
have been built with the same toolchain; rebuild it after a toolchain change.
