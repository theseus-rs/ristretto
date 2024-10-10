//! # Ristretto `ClassFile`
//!
//! [![Code Coverage](https://codecov.io/gh/theseus-rs/ristretto/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/ristretto)
//! [![Benchmarks](https://img.shields.io/badge/%F0%9F%90%B0_bencher-enabled-6ec241)](https://bencher.dev/perf/theseus-rs-ristretto)
//! [![License](https://img.shields.io/crates/l/ristretto_classfile)](https://github.com/theseus-rs/ristretto#license)
//! [![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)
//!
//! ## Getting Started
//!
//! Implementation of the [JVM Class File Format](https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html) that
//! is used to read, write and verify Java classes.
//!
//! Supports reading and writing class files for any version of Java version up to 24. Verification
//! of class files is supported, but is still a work in progress.
//!
//! # Examples
//!
//! ```rust
//! use ristretto_classfile::{ClassFile, ConstantPool, Result, Version};
//!
//! fn main() -> Result<()> {
//!     let mut constant_pool = ConstantPool::default();
//!     let this_class = constant_pool.add_class("Foo")?;
//!     let class_file = ClassFile {
//!         version: Version::Java21 { minor: 0 },
//!         constant_pool,
//!         this_class,
//!         ..Default::default()
//!     };
//!     class_file.verify()
//! }
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
pub use version::Version;
