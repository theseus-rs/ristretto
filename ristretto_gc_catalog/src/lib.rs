//! Bounded catalog operations for Ristretto's research runtime.
//!
//! The ordinary JVM collector owns registration, publication and reclamation
//! independently of this crate. Research clients supply their own record type
//! and prove payload access through the catalog's admission interface.
#![allow(unsafe_code)]
#![expect(
    missing_debug_implementations,
    reason = "catalog owners and operation cursors deliberately expose no payload or internal state through Debug"
)]
#![cfg_attr(
    test,
    expect(clippy::indexing_slicing, reason = "tests index fixed fixtures")
)]

/// Internal protocol shared by the research adapter and its validation programs.
#[doc(hidden)]
pub mod canonical_catalog;
mod canonical_pool;

/// Requested backing for the shared ownership handle used by catalog adapters.
pub use canonical_pool::arc_requested_bytes;
