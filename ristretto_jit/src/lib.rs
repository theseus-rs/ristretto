//! # Ristretto JIT
//!
//! [![Code Coverage](https://codecov.io/gh/theseus-rs/ristretto/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/ristretto)
//! [![Benchmarks](https://img.shields.io/badge/%F0%9F%90%B0_bencher-enabled-6ec241)](https://bencher.dev/perf/theseus-rs-ristretto)
//! [![License](https://img.shields.io/crates/l/ristretto_jit)](https://github.com/theseus-rs/ristretto#license)
//! [![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)
//!
//! ## Overview
//!
//! Ristretto JIT provides a Just-In-Time compiler for the Ristretto VM. The JIT compiler generates
//! native code from Ristretto VM bytecode, allowing for high-performance execution directly on the
//! host machine. The library supports both standard platforms and WebAssembly targets with tailored
//! compilation strategies. The appropriate compiler is automatically selected based on the target
//! platform.
//!
//! ## Architecture
//!
//! The JIT compiler follows a multi-stage compilation pipeline:
//!
//! 1. **Bytecode Analysis** - Analyzes Java bytecode for control flow patterns
//! 2. **Control Flow Graph Construction** - Builds a block based control flow graph
//! 3. **SSA Transformation** - Converts stack-based operations to Static Single Assignment form
//! 4. **Native Code Generation** - Generates optimized machine code
//! 5. **Function Binding** - Creates callable native functions
//!
//! ## Features
//!
//! - **Fast execution** through native code generation
//! - **Automatic optimization** of bytecode patterns
//! - **Control flow graph analysis** for complex branching logic
//! - **Platform-specific compilation** for x86-64, aarch64 (ARM64), s390x (IBM Z) and riscv64
//! - **Comprehensive error handling** with detailed error messages
//!
//! ## Platform Support
//!
//! The JIT compiler automatically adapts to the target platform:
//!
//! - **Native platforms**:
//!   - x86-64 (Intel/AMD 64-bit)
//!   - aarch64 (ARM 64-bit)
//!   - s390x (IBM Z Architecture)
//!   - riscv64 (RISC-V 64-bit)
//!
//! ## Limitations
//!
//! Current limitations include:
//!
//! - Only static methods and constructors (`<init>`) are supported
//! - Limited object-oriented features (no instance method compilation yet)
//! - No garbage collection integration
//! - Exception handling is not fully implemented
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
mod local_type;
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
