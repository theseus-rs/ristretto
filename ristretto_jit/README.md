# Ristretto JIT

[![Documentation](https://docs.rs/ristretto_jit/badge.svg)](https://docs.rs/ristretto_jit)
[![Code Coverage](https://codecov.io/gh/theseus-rs/ristretto/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/ristretto)
[![Benchmarks](https://img.shields.io/badge/%F0%9F%90%B0_bencher-enabled-6ec241)](https://bencher.dev/perf/theseus-rs-ristretto)
[![Latest version](https://img.shields.io/crates/v/ristretto_jit.svg)](https://crates.io/crates/ristretto_jit)
[![License](https://img.shields.io/crates/l/ristretto_jit)](https://github.com/theseus-rs/ristretto#license)
[![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)

Ristretto JIT provides a Just-In-Time compiler for the Ristretto VM. The JIT compiler generates native code from
Ristretto VM bytecode, allowing for high-performance execution directly on the host machine.

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
