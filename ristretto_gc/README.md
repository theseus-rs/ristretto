# Ristretto GC

[![Documentation](https://docs.rs/ristretto_gc/badge.svg)](https://docs.rs/ristretto_gc)
[![Code Coverage](https://codecov.io/gh/theseus-rs/ristretto/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/ristretto)
[![Benchmarks](https://img.shields.io/badge/%F0%9F%90%B0_bencher-enabled-6ec241)](https://bencher.dev/perf/theseus-rs-ristretto)
[![Latest version](https://img.shields.io/crates/v/ristretto_gc.svg)](https://crates.io/crates/ristretto_gc)
[![License](https://img.shields.io/crates/l/ristretto_gc)](https://github.com/theseus-rs/ristretto#license)
[![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)

A pauseless, concurrent and parallel mark and sweep garbage collector implementation for the Ristretto VM. This crate
provides `Gc<T>` types for garbage-collected references, using a pure reachability analysis algorithm with automatic
cycle detection and collection.
