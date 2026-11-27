//! # Ristretto GC
//!
//! [![Code Coverage](https://codecov.io/gh/theseus-rs/ristretto/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/ristretto)
//! [![Benchmarks](https://img.shields.io/badge/%F0%9F%90%B0_bencher-enabled-6ec241)](https://bencher.dev/perf/theseus-rs-ristretto)
//! [![License](https://img.shields.io/crates/l/ristretto_gc)](https://github.com/theseus-rs/ristretto#license)
//! [![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)
//!
//! A generational, incremental mark-and-sweep garbage collector implementation for the
//! Ristretto VM. This crate provides `Gc<T>` types for garbage collected references, using a pure
//! reachability analysis algorithm with automatic cycle detection and collection. Native hosts use
//! a concurrent worker; wasm hosts drive the same bounded state machine cooperatively.

#![allow(unsafe_code)]
#![cfg_attr(
    test,
    expect(
        clippy::indexing_slicing,
        reason = "unit tests assert fixture contents by index"
    )
)]

mod allocation;
mod allocation_budget;
mod buffer;
pub mod cell;
pub mod clock;
mod collector;
mod config;
mod construction;
mod context;
mod diagnostics;
mod error;
mod finalizer;
mod gc;
mod identity;
#[cfg(feature = "experimental-identity-index")]
mod identity_index;
#[cfg(not(feature = "experimental-identity-index"))]
mod identity_pages;
mod index;
#[cfg(test)]
mod managed_bytes_tests;
mod metadata;
#[cfg(not(target_family = "wasm"))]
mod pacing;
#[cfg(all(
    feature = "experimental-pause-recorder",
    target_os = "linux",
    target_has_atomic = "64"
))]
pub mod pause_recorder;
mod pointers;
mod publication_directory;
mod publication_queue;
mod recovery;
#[cfg(all(test, not(target_family = "wasm"), not(miri)))]
mod refund_tests;
pub mod region;
mod registration_key;
mod relocation;
mod relocation_history;
mod retirement_grants;
mod root_guard;
mod roots;
mod scheduling;
mod snapshot;
mod stale_epochs;
pub mod sync;
mod trace_owner;
mod weak;
mod weak_registry;
mod work;

pub use buffer::HeapBuffer;
pub use collector::{CollectionKind, GarbageCollector, HeapOwner, Trace, TraceProgress};
pub use config::{Configuration, ConfigurationBuilder, RootStorage, Statistics};
pub use context::ExecutionContext;
pub use diagnostics::{Diagnostics, TimingStatistics};
pub use error::{Error, Result};
pub use finalizer::Finalize;
pub use gc::Gc;
pub use identity::ObjectId;
pub use root_guard::GcRootGuard;
pub use roots::RootStatistics;
pub use scheduling::SchedulingStatistics;
pub use trace_owner::TraceOwner;

pub use weak::{GcWeak, ReferenceKind};
