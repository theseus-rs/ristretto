//! # Ristretto `JImage`
//!
//! [![Code Coverage](https://codecov.io/gh/theseus-rs/ristretto/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/ristretto)
//! [![Benchmarks](https://img.shields.io/badge/%F0%9F%90%B0_bencher-enabled-6ec241)](https://bencher.dev/perf/theseus-rs-ristretto)
//! [![License](https://img.shields.io/crates/l/ristretto_jimage)](https://github.com/theseus-rs/ristretto#license)
//! [![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)
//!
//! ## Getting Started
//!
//! `JImage` is a file format introduced in Java 9 to store Java runtime resources efficiently. This
//! crate provides a reader for `JImage` files, allowing you to access and manipulate the resources
//! contained within them.
//!
//! # Examples
//!
//! ```rust,no_run
//! use ristretto_jimage::Image;
//! use std::path::PathBuf;
//!
//! fn main() -> ristretto_jimage::Result<()> {
//!     // Typically found at $JAVA_HOME/lib/modules
//!     let path = PathBuf::from("/path/to/java/lib/modules");
//!     let image = Image::from_file(&path)?;
//!     let resource_name = "/java.base/java/lang/Object.class";
//!     let resource = image.get_resource(resource_name)?;
//!     assert_eq!(resource_name, resource.full_name());
//!     Ok(())
//! }
//! ```

mod attribute;
mod byte_source;
mod error;
mod header;
mod image;
mod index;
mod resource;

pub use error::{Error, Result};
pub use image::Image;
pub use resource::Resource;
