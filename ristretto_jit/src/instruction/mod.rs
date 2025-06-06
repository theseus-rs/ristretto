//! # JVM Instruction Module
//!
//! This module provides implementations of Java Virtual Machine (JVM) bytecode instructions for the
//! Ristretto JIT compiler.
//!
//! ## Instruction Categories
//!
//! * `array` - Array manipulation instructions (e.g., `aaload`, `aastore`)
//! * `branch` - Branching and control flow instructions (e.g., `if_icmpeq`, `goto`)
//! * `byte` - Byte manipulation instructions
//! * `char` - Character manipulation instructions
//! * `convert` - Type conversion instructions (e.g., `i2l`, `f2d`)
//! * `debug` - Debugging-related instructions
//! * `double` - Double-precision floating point instructions
//! * `exception` - Exception handling instructions (e.g., `athrow`)
//! * `field` - Field access instructions (e.g., `getfield`, `putfield`)
//! * `float` - Floating point instructions
//! * `integer` - Integer arithmetic and logic instructions
//! * `invoke` - Method invocation instructions (e.g., `invokevirtual`, `invokespecial`)
//! * `invokedynamic` - Dynamic method invocation instructions
//! * `ldc` - Constant loading instructions
//! * `long` - Long integer instructions
//! * `monitor` - Monitor-related instructions for synchronization
//! * `nop` - No-operation instruction
//! * `object` - Object manipulation instructions
//! * `push` - Stack value pushing instructions
//! * `short` - Short integer instructions
//! * `stack` - Stack manipulation instructions (e.g., `dup`, `swap`)
//! * `static` - Static field instructions
//! * `wide` - Wide instruction prefix for accessing local variables with wider indices
//!

mod array;
mod branch;
mod byte;
mod char;
mod convert;
mod debug;
mod double;
mod exception;
mod field;
mod float;
mod integer;
mod invoke;
mod invokedynamic;
mod ldc;
mod long;
mod monitor;
mod nop;
mod object;
mod push;
mod short;
mod stack;
mod r#static;
mod wide;

// pub(crate) use array::*;
pub(crate) use branch::*;
// pub(crate) use byte::*;
// pub(crate) use char::*;
pub(crate) use convert::*;
pub(crate) use debug::*;
pub(crate) use double::*;
// pub(crate) use exception::*;
// pub(crate) use field::*;
pub(crate) use float::*;
pub(crate) use integer::*;
// pub(crate) use invoke::*;
// pub(crate) use invokedynamic::*;
pub(crate) use ldc::*;
pub(crate) use long::*;
pub(crate) use monitor::*;
pub(crate) use nop::*;
// pub(crate) use object::*;
pub(crate) use push::*;
// pub(crate) use short::*;
pub(crate) use stack::*;
// pub(crate) use r#static::*;
pub(crate) use wide::*;

/// Constants for trap internal errors
pub(crate) const TRAP_INTERNAL_ERROR: u8 = 127;
