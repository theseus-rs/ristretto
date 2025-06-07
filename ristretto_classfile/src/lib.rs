//! # Ristretto `ClassFile`
//!
//! [![Code Coverage](https://codecov.io/gh/theseus-rs/ristretto/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/ristretto)
//! [![Benchmarks](https://img.shields.io/badge/%F0%9F%90%B0_bencher-enabled-6ec241)](https://bencher.dev/perf/theseus-rs-ristretto)
//! [![License](https://img.shields.io/crates/l/ristretto_classfile)](https://github.com/theseus-rs/ristretto#license)
//! [![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)
//!
//! ## Getting Started
//!
//! Implementation of the [JVM Class File Format](https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html)
//! that is used to read, write and verify Java classes.
//!
//! Supports reading and writing class files for any version of Java version up to 25. Verification
//! of class files is supported, but is still a work in progress.
//!
//! ## Examples
//!
//! ### Creating a Simple Class File
//!
//! ```rust
//! use ristretto_classfile::{ClassFile, ConstantPool, Version, JAVA_21};
//!
//! let mut constant_pool = ConstantPool::default();
//! let this_class = constant_pool.add_class("Foo")?;
//! let class_file = ClassFile {
//!     version: JAVA_21,
//!     constant_pool,
//!     this_class,
//!     ..Default::default()
//! };
//! class_file.verify()?;
//! # Ok::<(), ristretto_classfile::Error>(())
//! ```
//!
//! ### Reading a Class File from Bytes
//!
//! ```rust,no_run
//! use ristretto_classfile::ClassFile;
//! use std::fs;
//! use std::io::Cursor;
//!
//! // Read the bytes of a class file
//! let bytes = fs::read("path/to/Example.class")?;
//!
//! // Parse the bytes into a ClassFile
//! let class_file = ClassFile::from_bytes(&mut Cursor::new(bytes))?;
//!
//! // Now you can inspect the class
//! println!("Class name: {}", class_file.class_name()?);
//! println!("Class version: {}", class_file.version);
//! # Ok::<(), ristretto_classfile::Error>(())
//! ```
//!
//! ### Writing a Class File to Bytes
//!
//! ```rust,no_run
//! use ristretto_classfile::{ClassFile, ConstantPool, Version, ClassAccessFlags, JAVA_21};
//! use std::fs;
//! use std::io::{Cursor, Write};
//!
//! // Create a new class file
//! let mut constant_pool = ConstantPool::default();
//! let this_class = constant_pool.add_class("HelloWorld")?;
//! let super_class = constant_pool.add_class("java/lang/Object")?;
//!
//! let class_file = ClassFile {
//!     version: JAVA_21,
//!     access_flags: ClassAccessFlags::PUBLIC,
//!     constant_pool,
//!     this_class,
//!     super_class,
//!     ..Default::default()
//! };
//!
//! // Verify the class file is valid
//! class_file.verify()?;
//!
//! // Write the class file to a vector of bytes
//! let mut buffer = Vec::new();
//! class_file.to_bytes(&mut buffer)?;
//!
//! // Now you can save these bytes to a file
//! fs::write("HelloWorld.class", buffer)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
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
extern crate core;

pub mod attributes;
mod base_type;
mod class_access_flags;
mod class_file;
mod constant;
mod constant_pool;
mod display;
mod error;
mod field;
mod field_access_flags;
mod field_type;
mod method;
mod method_access_flags;
pub mod mutf8;
mod reference_kind;
mod verifiers;
mod version;

pub use base_type::BaseType;
pub use class_access_flags::ClassAccessFlags;
pub use class_file::ClassFile;
pub use constant::Constant;
pub use constant_pool::ConstantPool;
pub use error::{Error, Result};
pub use field::Field;
pub use field_access_flags::FieldAccessFlags;
pub use field_type::FieldType;
pub use method::Method;
pub use method_access_flags::MethodAccessFlags;
pub use reference_kind::ReferenceKind;
pub use version::{
    JAVA_1_0_2, JAVA_1_1, JAVA_1_2, JAVA_1_3, JAVA_1_4, JAVA_5, JAVA_6, JAVA_7, JAVA_8, JAVA_9,
    JAVA_10, JAVA_11, JAVA_12, JAVA_13, JAVA_14, JAVA_15, JAVA_16, JAVA_17, JAVA_18, JAVA_19,
    JAVA_20, JAVA_21, JAVA_22, JAVA_23, JAVA_24, JAVA_25, JAVA_PREVIEW_MINOR_VERSION, Version,
};
