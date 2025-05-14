//! # Ristretto JIT
//!
//! [![Code Coverage](https://codecov.io/gh/theseus-rs/ristretto/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/ristretto)
//! [![Benchmarks](https://img.shields.io/badge/%F0%9F%90%B0_bencher-enabled-6ec241)](https://bencher.dev/perf/theseus-rs-ristretto)
//! [![License](https://img.shields.io/crates/l/ristretto_classloader)](https://github.com/theseus-rs/ristretto#license)
//! [![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)
//!
//! ## Getting Started
//!
//! Implementation of a JIT compiler for the Ristretto VM. The JIT compiler uses generates native
//! code for the Ristretto VM bytecode. The generated code can be executed directly on the host
//! machine.
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
