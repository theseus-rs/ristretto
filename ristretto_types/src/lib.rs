//! # Ristretto Types
//!
//! Shared types and traits for the Ristretto JVM implementation. This crate provides the core
//! error types, parameter handling, and value conversion traits used by the VM and intrinsic
//! method implementations.

#![forbid(unsafe_code)]
#![cfg_attr(
    test,
    expect(
        clippy::panic_in_result_fn,
        clippy::unwrap_in_result,
        reason = "tests use assertions and small fixtures returning Result for concise setup"
    )
)]

mod assignable;
mod error;
mod frame;
pub mod handles;
mod java_error;
mod java_object;
pub mod module_access;
pub mod monitor;
pub mod native_memory;
mod parameters;
mod resource_manager;
mod rust_value;
#[cfg(test)]
mod test_utils;
mod thread;
mod vm;

pub use assignable::Assignable;
pub use error::{Error, Result};
pub use frame::Frame;
pub use java_error::JavaError;
pub use java_object::JavaObject;
pub use module_access::{
    ALL_UNNAMED, AccessCheckResult, DefinedModule, ModuleAccess, package_from_class_name,
};
pub use native_memory::NativeMemory;
pub use parameters::Parameters;
pub use resource_manager::ResourceManager;
pub use rust_value::RustValue;
use std::pin::Pin;
pub use thread::Thread;
pub use vm::{FIRST_NIO_FD, VM};

/// A boxed future type that is `Send` on non-wasm targets and not `Send` on wasm targets.
#[cfg(not(target_family = "wasm"))]
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// A boxed future type that is `Send` on non-wasm targets and not `Send` on wasm targets.
#[cfg(target_family = "wasm")]
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;
