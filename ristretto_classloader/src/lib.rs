//! # Ristretto `ClassLoader`
//!
//! [![Code Coverage](https://codecov.io/gh/theseus-rs/ristretto/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/ristretto)
//! [![Benchmarks](https://img.shields.io/badge/%F0%9F%90%B0_bencher-enabled-6ec241)](https://bencher.dev/perf/theseus-rs-ristretto)
//! [![License](https://img.shields.io/crates/l/ristretto_classloader)](https://github.com/theseus-rs/ristretto#license)
//! [![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)
//!
//! ## Getting Started
//!
//! Implementation of a [JVM Class Loader](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html)
//! that is used to load Java classes. Classes can be loaded from the file system or from a URL;
//! jar and modules are supported.  A runtime Java class loader can be created from a LTS version of
//! [AWS Corretto](https://github.com/corretto).  The runtime class loader will download and install
//! the requested version of Corretto and create a class loader that can be used to load Java
//! classes.
//!
//! The AWS Corretto runtime is installed in the following directory:
//!
//! - Unix: `$HOME/.ristretto/<version>`
//! - Windows: `%USERPROFILE%\.ristretto\<version>`
//!
//! # Examples
//!
//! ```rust
//! use ristretto_classloader::{runtime, ClassLoader, Result};
//! use std::sync::Arc;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let (_java_home, java_version, class_loader) = runtime::version_class_loader("21").await?;
//!     let class_name = "java.util.HashMap";
//!     println!("Loading {class_name} from Java version {java_version}");
//!     let class = class_loader.load(class_name).await?;
//!     println!("{class:?}");
//!     Ok(())
//! }
//! ```
//!
//! ## Feature flags
//!
//! The following features are available:
//!
//! | Name      | Description                    | Default? |
//! |-----------|--------------------------------|----------|
//! | `url`     | Enables url class path entries | No       |
//!
//! ## Safety
//!
//! This crate uses `#![forbid(unsafe_code)]` to ensure everything is implemented in 100% safe Rust.

#![forbid(unsafe_code)]
#![forbid(clippy::allow_attributes)]
#![allow(dead_code)]
#![deny(clippy::pedantic)]
#![deny(clippy::unwrap_used)]

mod class;
mod class_loader;
mod class_path;
mod class_path_entry;
mod error;
mod field;
mod method;
mod object;
mod reference;
pub mod runtime;
mod value;

pub use class::Class;
pub use class_loader::ClassLoader;
pub use class_path::ClassPath;
pub use class_path_entry::{ClassPathEntry, Manifest, manifest};
pub use error::{Error, Result};
pub use field::Field;
pub use method::Method;
pub use object::Object;
pub use reference::{ObjectArray, Reference};
pub use ristretto_classfile::{BaseType, FieldAccessFlags, FieldType, MethodAccessFlags};
pub use runtime::DEFAULT_JAVA_VERSION;
pub use value::Value;
