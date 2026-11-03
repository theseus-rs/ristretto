# Classfile review benchmarks

`review_baselines` exercises public APIs with deterministic fixtures. Fixture creation and validation happen outside the timed loops; an invalid fixture fails the benchmark instead of silently timing an error path.

Run the same benchmark source and toolchain at both revisions:

```sh
cargo bench -p ristretto_classfile --bench review_baselines -- \
  --save-baseline before --sample-size 50 --warm-up-time 3 --measurement-time 5
cargo bench -p ristretto_classfile --bench review_baselines -- \
  --baseline before --sample-size 50 --warm-up-time 3 --measurement-time 5
```

To compare independent branches before this benchmark is merged, use a temporary external Cargo project with `ristretto_classfile` as a path dependency and `criterion = "0.8.2"`. Copy `review_baselines.rs` into its `benches` directory, register it as a benchmark with `harness = false`, and change only the classfile dependency path between runs. This leaves the implementation branches independent of the benchmark commit.

| Workload | Fixture and timed operation |
| --- | --- |
| parse-borrowed / parse-owned | Parse the same serialized class with 32 methods and eight constant loads per method |
| serialize | Serialize that class into a new output buffer |
| cache-hit | Verify the same class against a populated verification cache |
| verify-code | Verify an inference-path method with 128 constant loads and pops |
| verify-handlers | Verify 128 constant loads and pops covered by a catch-all handler, with a declared handler frame and 64 local slots |
| verify-fallback | Verify a modern method with a branch and no stack map under permissive fallback |
| signature | Validate a generic method signature with nested parameterized types and a type variable |
| string-owned | Convert an owned string containing ASCII and BMP Unicode; input cloning is outside the timed conversion |
| format-ascii / format-mutf8 | Format ASCII, or mixed ASCII, modified NUL, and supplementary characters |

The verification context is deliberately limited to the fixture's Object and Throwable relationships. These are microbenchmarks, not end-to-end VM loading measurements. Run comparisons on an otherwise idle machine, repeat small effects, and report confidence intervals. Allocation counts and retained cache memory require a separate allocator profile; Criterion timings do not establish either.
