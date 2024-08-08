//! # Ristretto `ClassLoader`
//!
//! [![Code Coverage](https://codecov.io/gh/theseus-rs/ristretto/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/ristretto)
//! [![Benchmarks](https://img.shields.io/badge/%F0%9F%90%B0_bencher-enabled-6ec241)](https://bencher.dev/perf/theseus-rs-ristretto)
//! [![License](https://img.shields.io/crates/l/ristretto_classloader)](https://github.com/theseus-rs/ristretto#license)
//! [![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)
//!
//! ## Getting Started
//!
//! Implementation of a [JVM Class Loader](https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html) that is used to load Java classes.
//!
//! # Examples
//!
//! ```rust
//! use ristretto_classloader::{ClassLoader, ClassPath, Result};
//! use std::sync::Arc;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let class_loader = Arc::new(ClassLoader::new("example", ClassPath::from("../classes")));
//!     let class = ClassLoader::load_class(&class_loader, "HelloWorld").await?;
//!     println!("{class:?}");
//!     Ok(())
//! }
//! ```
//!
//! ## Safety
//!
//! These crates use `#![forbid(unsafe_code)]` to ensure everything is implemented in 100% safe Rust.

#![forbid(unsafe_code)]
#![allow(dead_code)]
#![deny(clippy::pedantic)]
#![deny(clippy::unwrap_in_result)]
#![deny(clippy::unwrap_used)]

mod class;
mod class_loader;
mod class_path;
mod class_path_entry;
mod error;

pub use class::Class;
pub use class_loader::ClassLoader;
pub use class_path::ClassPath;
pub use error::{Error, Result};
