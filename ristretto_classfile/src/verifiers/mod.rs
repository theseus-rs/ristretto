//! # Verifiers Module
//!
//! This module contains a collection of verifiers that validate different aspects of Java
//! class files according to the JVM specification. Each submodule implements verification
//! for a specific component of the class file structure.
//!
//! These verifiers help ensure that class files meet the structural and semantic requirements
//! before they are processed further or executed.

/// Verifies the access flags of a class for validity according to the JVM specification.
pub mod class_access_flags;

/// Validates the constant pool entries for correctness and consistency.
pub mod constant_pool;

/// Verifies the access flags of fields for validity according to the JVM specification.
pub mod field_access_flags;

/// Validates field definitions within a class file.
pub mod fields;

/// Verifies the interfaces implemented by a class.
pub mod interfaces;

/// Verifies the access flags of methods for validity according to the JVM specification.
pub mod method_access_flags;

/// Validates method definitions and signatures within a class file.
pub mod methods;

/// Provides the main verification framework and orchestration for all verifiers.
pub mod verifier;
