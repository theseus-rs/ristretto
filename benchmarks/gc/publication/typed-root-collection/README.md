# Full collection time after typed-root, marking and sweep changes

The fourth campaign covers paged and one-shot root deletion, allocator refund delivery and physical retirement accounting. Its smallest paired median speedups were 3.043× on native and 3.861× on WASI. [Full results and uncertainty](progress-collection/README.md)

The subsequent sparse bitmap experiment reached 2.13× on cooperative native. [Follow-up measurements](sparse-words/README.md)

The target is a threefold reduction in elapsed time for the same complete collection. The largest diagnostics-off improvement here was 1.83× on native. A native mixed heap with two background workers improved 1.22×. Each entry below is the median paired speedup across fresh processes. Ranges span the two publication policies.

| Collector execution | Live tree | Live wide graph | Tree with fresh garbage |
|---|---:|---:|---:|
| Native, cooperative | 1.82–1.83× | 1.81–1.83× | 1.40–1.44× |
| Native, two workers and caller | 1.65–1.72× | 1.77–1.78× | 1.22× |
| WASI, cooperative | 1.73–1.75× | 1.68–1.72× | 1.33–1.34× |

For example, with counted publication the median full mixed cycle fell from 3.16 to 2.19 ms on cooperative native, from 4.03 to 3.31 ms with two workers, and from 2.23 to 1.68 ms on WASI. Speedups use within-pair elapsed times. The two column medians can give a slightly different ratio.

[Native graph](native.svg) · [WASI graph](wasm.svg) · [Individual process measurements in execution order](processes.csv) · [Cell medians and intervals](summary.json) · [Toolchain, runner, source and executable hashes](provenance.json)

## Work and timing

The control is `f1daebeb277323fa0d5e732ae3853db6b97eac8b`. The candidate is `46ef6e262d5a7ac728c5b4a50281cdba727cdea6`. Both were built with the same fixture from the candidate, default Cargo features, the release profile and separate build directories. Each process constructed a 4,095-node graph, warmed it with three full cycles and timed 16 full collections. The mixed shape added 1,023 unreachable eight-byte allocations before each timed cycle. Its timing includes waiting for the last worker to report the cohort swept. Graph construction, cohort allocation and teardown ran outside the timer.

Each of the 36 cells has 12 independently launched before/after pairs. The harness randomized cell and within-pair order, pinned CPUs and ran with diagnostics both enabled and disabled. All 864 processes completed. Each verified 65,520 live-node marks, 16 full cycles, the graph checksum and zero remaining allocations and bytes after teardown. Each mixed process also verified exactly 16,368 objects swept and 130,944 bytes freed. The graphs show candidate/control elapsed ratios and exploratory 95% intervals resampled across the 12 pairs. An initial mixed run exposed a worker accounting lag. Both arms were rebuilt with the full-cohort wait and the entire matrix was rerun.

This experiment measures complete collections of bounded heaps. Sustained JVM latency, mutator pauses, browser Wasm and the remaining active-phase typed-root admission paths require separate tests. The earlier JVM results compare application throughput with G1 and ZGC. [JVM comparison](https://github.com/theseus-rs/ristretto/pull/1159)

## Reproduce

Run from a clone containing the two revisions. The shell example uses Linux CPU affinity, Rust's `wasm32-wasip2` target, Wasmtime and Python with Matplotlib. Adjust the CPU IDs to the available affinity set.

```sh
set -eu
run=$(mktemp -d)
git worktree add --detach "$run/before" f1daebeb277323fa0d5e732ae3853db6b97eac8b
git worktree add --detach "$run/after" 46ef6e262d5a7ac728c5b4a50281cdba727cdea6
fixture=ristretto_gc/examples/mark_cycles.rs
cp "$run/after/$fixture" "$run/before/$fixture"
scripts="$run/after/benchmarks/gc"
for arm in before after; do
  src="$run/$arm"
  export CARGO_TARGET_DIR="$run/build-$arm"
  (cd "$src" && cargo build --locked --release -p ristretto_gc --example mark_cycles)
  python3 "$scripts/mark_cycles_build.py" --source "$src" --target native \
    --binary "$CARGO_TARGET_DIR/release/examples/mark_cycles" --output "$run/$arm-native.json"
  (cd "$src" && cargo build --locked --release -p ristretto_gc --example mark_cycles --target wasm32-wasip2)
  python3 "$scripts/mark_cycles_build.py" --source "$src" --target wasm \
    --binary "$CARGO_TARGET_DIR/wasm32-wasip2/release/examples/mark_cycles.wasm" --output "$run/$arm-wasm.json"
done
python3 "$scripts/mark_cycles.py" --before-build "$run/before-native.json" --after-build "$run/after-native.json" \
  --output "$run/results-native" --cpus 4 6 8 --workers 0 2 --shapes tree wide mixed \
  --nodes 4095 --cycles 16 --repeats 12 --seed 7281 --plot
python3 "$scripts/mark_cycles.py" --before-build "$run/before-wasm.json" --after-build "$run/after-wasm.json" \
  --output "$run/results-wasm" --cpus 10 --workers 0 --wasmtime "$(command -v wasmtime)" \
  --shapes tree wide mixed --nodes 4095 --cycles 16 --repeats 12 --seed 7281 --plot
```

The subsequent full-collection measurements cover bounded retirements and contended claims. [Results and graphs](collection-throughput/README.md)
