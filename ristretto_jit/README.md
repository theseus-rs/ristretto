# Ristretto JIT

[![ci](https://github.com/theseus-rs/ristretto/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/theseus-rs/ristretto/actions/workflows/ci.yml)
[![Documentation](https://docs.rs/ristretto_jit/badge.svg)](https://docs.rs/ristretto_jit)
[![Code Coverage](https://codecov.io/gh/theseus-rs/ristretto/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/ristretto)
[![Benchmarks](https://img.shields.io/badge/%F0%9F%90%B0_bencher-enabled-6ec241)](https://bencher.dev/perf/theseus-rs-ristretto)
[![Latest version](https://img.shields.io/crates/v/ristretto_jit.svg)](https://crates.io/crates/ristretto_jit)
[![License](https://img.shields.io/crates/l/ristretto_jit)](https://github.com/theseus-rs/ristretto#license)
[![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)

## Overview

Ristretto JIT is an implementation of a Java Virtual Machine (JVM) Just-In-Time (JIT) compiler written in Rust. It
dynamically compiles Java bytecode to native machine code at runtime to improve performance.

## Features

- Fast execution through native code generation
- Automatic optimization of bytecode
- Control flow graph analysis
- Platform-specific compilation for x86-64, aarch64 (aka ARM64), s390x (aka IBM Z) and riscv64
- Comprehensive error handling

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
