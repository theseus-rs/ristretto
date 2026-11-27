# Reproduce the reference-array collection probe

Use Linux, the repository Rust toolchain, the `wasm32-wasip2` target and Wasmtime 50.0.1. The probe builds as an external crate with a pinned dependency lockfile. The build script takes a checkout and a new output directory and prints the compiled executable path. Run it from this directory.

```sh
native=$(python3 build.py /path/to/checkout /tmp/card-native)
taskset -c 10 "$native" endpoints

wasm=$(python3 build.py /path/to/checkout /tmp/card-wasi --target wasm32-wasip2)
taskset -c 10 wasmtime run -W max-wasm-stack=8388608 -S http "$wasm" endpoints
```

The four inputs are `null`, `endpoints`, `alias` and `unique`. One process constructs and roots one 4,097-slot Java reference array, warms three full collections, times 16 full collections with automatic scheduling disabled, checks its values, drops its root and checks the final refund. The three warmups, value checks and teardown are outside the timed samples.

The published parent is `46d0afec`. To reproduce the exact candidate source tree, apply the included patch to a separate checkout of that parent. The source tree ID is recorded in the validation receipt.

```sh
git worktree add --detach /tmp/card-candidate 46d0afec
gzip -dc measured-tree.patch.gz | git -C /tmp/card-candidate apply --index -
git -C /tmp/card-candidate write-tree
```

The published run used CPU 10, 24 pairs per target and shape, and random seed 251130319. For each target, Python shuffled the four shapes within a repeat and the two arms within each shape. Each pair contributes the ratio of the medians of its 16 native or WASI timings. The reported speedup is the median paired ratio. The individual intervals use 16,000 bootstrap resamples with replacement and seed 251130321, iterating native then WASI and shapes in the order above. [Gate thresholds](selection.json) · [Per-process data](processes.jsonl.gz) · [Source, toolchain and executable identities](provenance.json).
