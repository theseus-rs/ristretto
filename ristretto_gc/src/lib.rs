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
#[cfg(feature = "experimental-canonical-runtime")]
mod canonical_adapter;
#[cfg(feature = "experimental-canonical-leaves")]
pub(crate) mod canonical_catalog;
mod canonical_payload;
#[cfg(feature = "experimental-canonical-leaves")]
pub(crate) mod canonical_pool;
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
mod private_allocation;
mod publication_directory;
mod publication_queue;
mod recovery;
pub mod region;
#[expect(
    dead_code,
    reason = "registry migration component awaits coupled phase/root activation"
)]
mod registration_image;
mod registration_key;
mod relocation;
mod relocation_history;
mod root_guard;
mod roots;
mod scheduling;
#[cfg(all(feature = "experimental-canonical-service", target_has_atomic = "64"))]
pub mod service_recorder;
pub mod sync;
mod trace_owner;
mod weak;
mod weak_registry;
mod work;

pub use buffer::HeapBuffer;
#[cfg(feature = "experimental-canonical-runtime")]
pub use canonical_adapter::api as canonical;
#[cfg(feature = "experimental-canonical-audit")]
pub use canonical_catalog::audit as experimental_canonical_audit;
pub use canonical_payload::CanonicalPayload;
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

/// Run the controlled reference-free lifecycle diagnostic; this is not a general
/// canonical allocation API. The result distinguishes operation, cleanup and total
/// lifecycle time. Bare Wasm has no supported timing host for this diagnostic.
///
/// # Errors
/// Rejects invalid fixture inputs, allocation/protocol failures or failed teardown.
#[cfg(feature = "experimental-canonical-leaves")]
#[doc(hidden)]
pub fn experimental_leaf_lifecycle(mode: &str, operations: u64, batch: usize) -> Result<String> {
    gc::leaf_lifecycle_benchmark(mode, operations, batch)
}

/// Measure controlled cyclic-graph allocation, writes, rooted reads and complete
/// reclamation. This diagnostic requires a native or WASI clock and does not
/// select a public collector backend or qualify pauses/endurance.
///
/// # Errors
/// Rejects invalid bounds, failed operations, watchdog expiry or teardown failure.
#[cfg(feature = "experimental-canonical-leaves")]
#[doc(hidden)]
pub fn experimental_graph_lifecycle(rounds: u64, capacity: usize) -> Result<String> {
    gc::graph_lifecycle_benchmark(rounds, capacity)
}

/// Exercise the internal leaf runtime with finite capacities and exact identity,
/// destruction and teardown checks. Cooperative mode needs no clock or threads.
/// This controlled payload does not qualify arbitrary callbacks or graphs.
///
/// # Errors
/// Rejects invalid parameters, unsupported modes, failed progress or integrity checks.
#[cfg(feature = "experimental-canonical-leaves")]
#[doc(hidden)]
pub fn experimental_leaf_contention(
    mode: &str,
    actors: usize,
    helpers: usize,
    operations: usize,
    capacity: usize,
    retained: usize,
) -> Result<String> {
    gc::leaf_contention(mode, actors, helpers, operations, capacity, retained)
}

/// Measure shared owning-root clone/drop costs and validate last-owner cleanup.
/// Checkpoints support a separate allocation audit; timing callers use a no-op.
/// Requires native or WASI clocks. No public backend or pause claim is implied.
///
/// # Errors
/// Rejects invalid inputs, protocol failures, watchdog expiry and failed cleanup.
#[cfg(feature = "experimental-canonical-leaves")]
#[doc(hidden)]
pub fn experimental_root_aliases(
    mode: &str,
    copies: u64,
    checkpoint: impl FnMut(&'static str),
) -> Result<String> {
    gc::root_alias_benchmark(mode, copies, checkpoint)
}

/// Audit final typed-root backing and credit order after all runtime owners leave.
/// The supplied checkpoints are outside catalog operations. No timing is reported.
///
/// # Errors
/// Rejects an unknown mode, protocol failures, watchdog expiry or incomplete cleanup.
#[cfg(feature = "experimental-canonical-audit")]
#[doc(hidden)]
pub fn experimental_root_refund_order(
    mode: &str,
    checkpoint: impl FnMut(&'static str, usize),
) -> Result<()> {
    gc::root_refund_order(mode, checkpoint)
}
