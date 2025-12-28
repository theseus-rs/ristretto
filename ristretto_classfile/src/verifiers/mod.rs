//! # Verifiers Module
//!
//! This module contains a collection of verifiers that validate different aspects of Java
//! class files according to the JVM specification. Each submodule implements verification
//! for a specific component of the class file structure.
//!
//! These verifiers help ensure that class files meet the structural and semantic requirements
//! before they are processed further or executed.
//!
//! # Bytecode Verification
//!
//! The new `bytecode` module provides a 100% JVMS-compliant bytecode verifier with:
//! - Rigorous type system implementation ([JVMS §4.10.1.2](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.10.1.2))
//! - Complete instruction coverage (all 200+ opcodes)
//! - Optimized dataflow analysis using dense vectors
//! - Comprehensive error reporting
//!
//! # References
//!
//! - [JVMS §4.9 - Constraints on Java Virtual Machine Code](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.9)
//! - [JVMS §4.10 - Verification of class Files](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.10)

/// Performs dataflow analysis and bytecode verification (legacy implementation).
pub(crate) mod analysis;

/// New modular bytecode verification implementation.
///
/// This module provides a 100% JVMS-compliant bytecode verifier organized into:
/// - `type_system`: Rigorous verification type system
/// - `frame`: Optimized stack frame for dataflow analysis
/// - `control_flow`: CFG construction and StackMapTable validation
/// - `handlers`: Specialized instruction verifiers
pub mod bytecode;

/// Validates attributes within a class file.
pub(crate) mod attributes;

/// Verifies the access flags of a class for validity according to the JVM specification.
pub(crate) mod class_access_flags;

/// Provides the verification context interface for type resolution.
pub mod context;

/// Validates the code attribute within a method.
pub(crate) mod code;

/// Validates the constant pool entries for correctness and consistency.
pub(crate) mod constant_pool;

/// Defines errors related to verification.
pub mod error;

/// Verifies the access flags of fields for validity according to the JVM specification.
pub(crate) mod field_access_flags;

/// Validates field definitions within a class file.
pub(crate) mod fields;

/// Defines stack frames used in bytecode verification.
pub(crate) mod frame;

/// Verifies the interfaces implemented by a class.
pub(crate) mod interfaces;

/// Verifies the access flags of methods for validity according to the JVM specification.
pub(crate) mod method_access_flags;

/// Validates method definitions and signatures within a class file.
pub(crate) mod methods;

/// Verifies `NestHost` and `NestMembers` attribute consistency.
pub(crate) mod nest;

/// Verifies `PermittedSubclasses` attribute for sealed classes.
pub(crate) mod permitted_subclasses;

/// Verifies `Record` attribute and record component validation.
pub(crate) mod record;

/// Validates generic signature grammar and type variable references.
pub(crate) mod signature;

/// Defines verification types used in bytecode verification.
pub(crate) mod types;

/// Provides the main verification framework and orchestration for all verifiers.
pub(crate) mod verifier;

pub use bytecode::config::VerifyMode;
pub use error::{Result, VerifyError};
