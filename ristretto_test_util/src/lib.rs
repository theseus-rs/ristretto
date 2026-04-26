//! Shared test utilities for the ristretto workspace.
//!
//! This crate is `publish = false` and is intended to be used as a
//! `dev-dependency` only. Helpers are organized into focused submodules and
//! re-exported from the crate root for ergonomic use at the call site.

pub mod tempdir;

pub use tempdir::init_wasi_tempdir;
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
pub use tempdir::{new_named_tempfile, new_tempdir};
