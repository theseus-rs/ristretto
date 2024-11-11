//! # Ristretto `ClassLoader`
//!
//! [![Code Coverage](https://codecov.io/gh/theseus-rs/ristretto/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/ristretto)
//! [![Benchmarks](https://img.shields.io/badge/%F0%9F%90%B0_bencher-enabled-6ec241)](https://bencher.dev/perf/theseus-rs-ristretto)
//! [![License](https://img.shields.io/crates/l/ristretto_classloader)](https://github.com/theseus-rs/ristretto#license)
//! [![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)
//!
//! ## Getting Started
//!
//! Implementation of a [JVM Class Loader](https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html)
//! that is used to load Java classes. Classes can be loaded from the file system or from a URL;
//! jar and modules are supported.  A runtime Java class loader can be created from any version of
//! [AWS Corretto](https://github.com/corretto).  The runtime class loader will download and install
//! the requested version of Corretto into and create a class loader that can be used to load Java
//! classes.
//!
//! ## Safety
//!
//! This crate uses `#![forbid(unsafe_code)]` to ensure everything is implemented in 100% safe Rust.

#![forbid(unsafe_code)]
#![forbid(clippy::allow_attributes)]
#![allow(dead_code)]
#![deny(clippy::pedantic)]
#![deny(clippy::unwrap_in_result)]
#![deny(clippy::unwrap_used)]
mod arguments;
#[expect(clippy::module_name_repetitions)]
mod configuration;
mod error;
mod frame;
mod instruction;
mod java_object;
mod local_variables;
mod native_methods;
mod operand_stack;
mod rust_value;
#[cfg(test)]
pub(crate) mod test;
mod thread;
mod vm;

pub use configuration::{Configuration, ConfigurationBuilder};
pub use error::{Error, Result};
pub(crate) use frame::Frame;
pub(crate) use local_variables::LocalVariables;
pub(crate) use operand_stack::OperandStack;
pub use ristretto_classloader::{Class, ClassPath, Reference, Value};
pub(crate) use thread::Thread;
pub use vm::VM;
