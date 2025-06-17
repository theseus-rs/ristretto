//! # Ristretto VM
//!
//! [![Code Coverage](https://codecov.io/gh/theseus-rs/ristretto/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/ristretto)
//! [![Benchmarks](https://img.shields.io/badge/%F0%9F%90%B0_bencher-enabled-6ec241)](https://bencher.dev/perf/theseus-rs-ristretto)
//! [![License](https://img.shields.io/crates/l/ristretto_vm)](https://github.com/theseus-rs/ristretto#license)
//! [![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)
//!
//! Ristretto VM is a Java Virtual Machine implementation written in pure Rust. It executes Java
//! bytecode by interpreting class files loaded through the Ristretto classloader.
//!
//! ## Features
//!
//! - Bytecode interpretation with no dependencies on existing JVM implementations
//! - Pure Rust implementation for memory safety and performance
//! - Support for Java class loading and execution
//! - Configurable VM parameters
//! - Basic JIT compilation capabilities
//!
//! ## Examples
//!
//! ```rust,no_run
//! use ristretto_vm::{VM, Configuration, ConfigurationBuilder};
//! use ristretto_classloader::ClassPath;
//!
//! # #[tokio::main]
//! # async fn main() -> ristretto_vm::Result<()> {
//! // Create a VM configuration
//! let configuration = ConfigurationBuilder::new()
//!     .class_path(ClassPath::from("/path/to/classes"))
//!     .build()?;
//!
//! // Create the VM instance
//! let mut vm = VM::new(configuration).await?;
//!
//! // Execute main method of a class
//! let _ = vm.invoke_main(Vec::<String>::new()).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Safety
//!
//! This crate uses `#![forbid(unsafe_code)]` to ensure everything is implemented in 100% safe Rust.

#![forbid(unsafe_code)]
#![forbid(clippy::allow_attributes)]
#![allow(dead_code)]
#![allow(clippy::only_used_in_recursion)]
// Disable pedantic clippy lint due to error when running clippy on this crate
// cargo clippyerror: allocating a local array larger than 16384 bytes
//    |
//    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#large_stack_arrays
// note: the lint level is defined here
//   --> ristretto_vm/src/lib.rs:25:9
//    |
// 25 | #![deny(clippy::pedantic)]
//    |         ^^^^^^^^^^^^^^^^
//    = note: `#[deny(clippy::large_stack_arrays)]` implied by `#[deny(clippy::pedantic)]`
// #![deny(clippy::pedantic)]
#![deny(clippy::unwrap_in_result)]
#![deny(clippy::unwrap_used)]
mod configuration;
mod error;
mod frame;
mod handle;
mod handles;
mod instruction;
mod intrinsic_methods;
mod java_error;
mod java_object;
mod jit;
mod local_variables;
mod operand_stack;
mod parameters;
mod rust_value;
#[cfg(test)]
pub(crate) mod test;
mod thread;
mod vm;

pub use configuration::{Configuration, ConfigurationBuilder};
pub use error::{Error, Result};
pub(crate) use frame::Frame;
pub use java_error::JavaError;
pub use java_object::JavaObject;
pub(crate) use local_variables::LocalVariables;
pub(crate) use operand_stack::OperandStack;
pub use ristretto_classloader::{Class, ClassPath, DEFAULT_JAVA_VERSION, Reference, Value};
pub(crate) use thread::Thread;
pub use vm::VM;
