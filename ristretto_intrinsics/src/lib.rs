//! # Ristretto Intrinsics
//!
//! [![Code Coverage](https://codecov.io/gh/theseus-rs/ristretto/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/ristretto)
//! [![Benchmarks](https://img.shields.io/badge/%F0%9F%90%B0_bencher-enabled-6ec241)](https://bencher.dev/perf/theseus-rs-ristretto)
//! [![License](https://img.shields.io/crates/l/ristretto_intrinsics)](https://github.com/theseus-rs/ristretto#license)
//! [![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)
//!
//! This crate provides intrinsics methods for the Ristretto VM.

#![forbid(unsafe_code)]
// The async_recursion macro adds Send bounds that overlap with the generic parameter bounds.
#![expect(clippy::multiple_bound_locations)]

#[cfg(test)]
pub(crate) mod test;

/// Methods from the Apple-specific packages
pub mod apple;
/// Methods from the COM-related packages
pub mod com;
/// Core Java standard library methods
pub mod java;
/// JDK-specific internal methods
pub mod jdk;
/// Methods for handling Java properties
pub mod properties;
/// Methods from Sun packages
pub mod sun;

// Re-export specific functions needed by the VM
pub use java::lang::invoke::methodhandle::call_method_handle_target;
pub use java::lang::object::get_monitor_id;
