//! # Intrinsic Methods Module
//!
//! This module contains implementations of native methods that are intrinsic to the JVM. These
//! methods are part of the Java standard library but require VM-level implementation for proper
//! functionality.
//!
//! The module organizes intrinsic methods by their package structure (e.g., java.*, sun.*), making
//! it easier to maintain and navigate the codebase.
//!
//! ## Usage
//!
//! The module exports a `MethodRegistry` that can be used to register and look up intrinsic method
//! implementations during class loading and method resolution.

/// Methods from the Apple-specific packages
#[cfg(target_os = "macos")]
mod apple;
/// Methods from the COM-related packages
mod com;
/// Core Java standard library methods
pub mod java;
/// JDK-specific internal methods
mod jdk;
/// Methods for handling Java properties
mod properties;
/// Methods from Sun packages
mod sun;
