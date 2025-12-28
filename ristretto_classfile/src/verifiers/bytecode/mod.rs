//! # Bytecode Verification Module
//!
//! This module implements a 100% compliant JVM bytecode verifier according to
//! JVMS Chapter 4 (The class File Format) and Chapter 5 (Loading, Linking, and Initializing).
//!
//! # Architecture
//!
//! The verifier is organized into several submodules with a fast optimization strategy:
//!
//! ## Core Components
//!
//! - `type_system`: Rigorous implementation of the JVM verification type system
//! - `frame`: Optimized stack frame implementation for dataflow analysis
//! - `control_flow`: Control flow graph construction and `StackMapTable` validation
//! - `handlers`: Specialized verifiers for instruction groups
//!
//! ## Optimization Modules
//!
//! - `config`: Verifier configuration (verify mode, fallback strategy, etc.)
//! - `fast_path`: StackMapTable-driven verification (primary, single-pass)
//! - `inference`: Type-inference verification (fallback, iterative dataflow)
//! - `stackmap`: Efficient StackMapTable decoding and caching
//! - `cache`: Verification artifact caching
//! - `diagnostics`: Detailed error reporting and tracing
//! - `unified`: Main entry point combining both verification paths
//!
//! # Verification Strategy
//!
//! The verifier uses a two-path strategy:
//!
//! 1. **Fast Path (Primary)**: StackMapTable-driven "type-checking" verification
//!    - Single pass through bytecode
//!    - Uses StackMapTable frames as trusted anchors
//!    - Avoids iterative dataflow merging
//!    - Used for class files version 50+ (Java 6+)
//!
//! 2. **Slow Path (Fallback)**: Type-inference verification
//!    - Classic iterative dataflow analysis
//!    - Computes types via merging at control flow points
//!    - Used for pre-Java 6 classes or when fast path fails
//!    - Controlled by `FallbackStrategy` configuration
//!
//! # Usage
//!
//! ```rust,ignore
//! use ristretto_classfile::verifiers::bytecode::unified::verify_method;
//! use ristretto_classfile::verifiers::bytecode::config::VerifierConfig;
//!
//! // Default configuration (fast path with no fallback)
//! let config = VerifierConfig::default();
//! let result = verify_method(&class_file, &method, &context, &config)?;
//!
//! // Permissive configuration (allows fallback to inference)
//! let config = VerifierConfig::permissive();
//! let result = verify_method(&class_file, &method, &context, &config)?;
//! ```
//!
//! # References
//!
//! - [JVMS §4.9 - Constraints on Java Virtual Machine Code](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.9)
//! - [JVMS §4.10 - Verification of class Files](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.10)
//! - [JVMS §4.10.1 - Verification by Type Checking](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.10.1)
//! - [JVMS §4.10.2 - Verification by Type Inference](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.10.2)

// Core verification components
pub(crate) mod control_flow;
pub(crate) mod frame;
pub(crate) mod handlers;
pub(crate) mod type_system;

// fast optimization modules
pub(crate) mod cache;
pub(crate) mod config;
pub(crate) mod diagnostics;
pub(crate) mod fast_path;
pub(crate) mod inference;
pub(crate) mod stackmap;
pub(crate) mod unified;
pub(crate) mod verifier;

// Re-export main entry points
pub use config::{FallbackStrategy, VerifierConfig, VerifierFlags, VerifyMode};
pub use unified::{VerificationPath, VerificationResult, verify_class, verify_method};
pub use verifier::verify;
