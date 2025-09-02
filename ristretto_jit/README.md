# Ristretto JIT

[![ci](https://github.com/theseus-rs/ristretto/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/theseus-rs/ristretto/actions/workflows/ci.yml)
[![Documentation](https://docs.rs/ristretto_jit/badge.svg)](https://docs.rs/ristretto_jit)
[![Code Coverage](https://codecov.io/gh/theseus-rs/ristretto/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/ristretto)
[![Benchmarks](https://img.shields.io/badge/%F0%9F%90%B0_bencher-enabled-6ec241)](https://bencher.dev/perf/theseus-rs-ristretto)
[![Latest version](https://img.shields.io/crates/v/ristretto_jit.svg)](https://crates.io/crates/ristretto_jit)
[![License](https://img.shields.io/crates/l/ristretto_jit)](https://github.com/theseus-rs/ristretto#license)
[![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)

## Overview

Ristretto JIT provides a Just-In-Time compiler for the Ristretto VM. The JIT compiler generates native code from
Ristretto VM bytecode, allowing for high-performance execution directly on the host machine.

## Architecture

The JIT compiler follows a multi-stage compilation pipeline:

1. **Bytecode Analysis** - Analyzes Java bytecode for control flow patterns
2. **Control Flow Graph Construction** - Builds a block based control flow graph
3. **SSA Transformation** - Converts stack-based operations to Static Single Assignment form
4. **Native Code Generation** - Generates optimized machine code
5. **Function Binding** - Creates callable native functions

## Features

- **Fast execution** through native code generation
- **Automatic optimization** of bytecode patterns
- **Control flow graph analysis** for complex branching logic
- **Platform-specific compilation** for x86-64, aarch64 (ARM64), s390x (IBM Z) and riscv64
- **Comprehensive error handling** with detailed error messages

## Platform Support

The JIT compiler automatically adapts to the target platform:

- **Native platforms**:
    - x86-64 (Intel/AMD 64-bit)
    - aarch64 (ARM 64-bit)
    - s390x (IBM Z Architecture)
    - riscv64 (RISC-V 64-bit)

## Limitations

Current limitations include:

- Only static methods and constructors (`<init>`) are supported
- Limited object-oriented features (no instance method compilation yet)
- No garbage collection integration
- Exception handling is not fully implemented

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
