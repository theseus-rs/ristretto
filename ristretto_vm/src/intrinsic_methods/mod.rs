//! # Intrinsic Methods Module
//!
//! This module contains the registry and lookup functionality for intrinsic method
//! implementations. The actual implementations are in the `ristretto_intrinsics` crate.

/// Mapping of intrinsic method names to their implementations
mod intrinsics;
/// Registry for storing and retrieving intrinsic method implementations
mod registry;

/// Re-export the `MethodRegistry` for use by other modules in the VM
pub use registry::{IntrinsicMethod, MethodRegistry};
