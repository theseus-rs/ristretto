//! # Ristretto JIT
//!
//! [![Code Coverage](https://codecov.io/gh/theseus-rs/ristretto/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/ristretto)
//! [![Benchmarks](https://img.shields.io/badge/%F0%9F%90%B0_bencher-enabled-6ec241)](https://bencher.dev/perf/theseus-rs-ristretto)
//! [![License](https://img.shields.io/crates/l/ristretto_classloader)](https://github.com/theseus-rs/ristretto#license)
//! [![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)
//!
//! ## Overview
//!
//! Ristretto JIT provides a Just-In-Time compiler for the Ristretto VM. The JIT compiler generates
//! native code from Ristretto VM bytecode, allowing for high-performance execution directly on the
//! host machine. The library supports both standard platforms and WebAssembly targets with tailored
//! compilation strategies.  The appropriate compiler is automatically selected based on the target
//! platform.
//!
//! ## Features
//!
//! - Fast execution through native code generation
//! - Automatic optimization of bytecode
//! - Control flow graph analysis
//! - Platform-specific compilation for x86-64, aarch64 (aka ARM64), s390x (aka IBM Z) and riscv64
//! - Comprehensive error handling
//!
#![forbid(clippy::allow_attributes)]
#![allow(dead_code)]
#![deny(clippy::pedantic)]
#![deny(clippy::unwrap_in_result)]
#![deny(clippy::unwrap_used)]

#[cfg(not(target_family = "wasm"))]
mod compiler;
mod control_flow_graph;
mod error;
mod function;
#[cfg(not(target_family = "wasm"))]
mod instruction;
mod jit_value;
mod local_variables;
mod operand_stack;
#[cfg(test)]
mod test;
mod value;
#[cfg(target_family = "wasm")]
mod wasm_compiler;

#[cfg(not(target_family = "wasm"))]
pub use compiler::Compiler;
pub use error::{Error, Result};
pub use function::Function;
pub(crate) use jit_value::JitValue;
pub use value::Value;
#[cfg(target_family = "wasm")]
pub use wasm_compiler::Compiler;
