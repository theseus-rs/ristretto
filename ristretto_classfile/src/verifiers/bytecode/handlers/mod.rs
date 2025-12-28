//! # Instruction Handlers for Bytecode Verification
//!
//! This module contains specialized verifiers for different groups of JVM instructions.
//! Each submodule handles a specific category of instructions according to the JVMS.
//!
//! # Modules
//!
//! - `load_store`: Local variable load/store operations
//! - `stack`: Stack manipulation operations (dup, pop, swap)
//! - `math`: Arithmetic and logical operations
//! - `conversion`: Type conversion operations
//! - `comparison`: Comparison operations
//! - `references`: Object/field/method operations
//! - `control`: Control flow operations (branches, switches, returns)
//! - `exceptions`: Exception handling (athrow)
//! - `misc`: Miscellaneous operations (nop, monitor*)
//!
//! # References
//!
//! - [JVMS §6 - The Java Virtual Machine Instruction Set](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html)

pub mod comparison;
pub mod control;
pub mod conversion;
pub mod exceptions;
pub mod load_store;
pub mod math;
pub mod misc;
pub mod references;
pub mod stack;

#[cfg(test)]
pub mod test_utils;
