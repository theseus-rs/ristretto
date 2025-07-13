//! # Ristretto GC
//!
//! [![Code Coverage](https://codecov.io/gh/theseus-rs/ristretto/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/ristretto)
//! [![Benchmarks](https://img.shields.io/badge/%F0%9F%90%B0_bencher-enabled-6ec241)](https://bencher.dev/perf/theseus-rs-ristretto)
//! [![License](https://img.shields.io/crates/l/ristretto_gc)](https://github.com/theseus-rs/ristretto#license)
//! [![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)
//!
//! ## Overview
//!
//! A pauseless, concurrent mark and sweep garbage collector implementation for the Ristretto VM.
//! This crate provides `Gc<T>` types for garbage-collected references, using a pure reachability
//! analysis algorithm with automatic cycle detection and collection.
//!
//! ## Features
//!
//! ### 🔄 **Pauseless Concurrent Collection**
//! - **Sub-millisecond pause times** (default: 100 microseconds maximum)
//! - **Concurrent mark-and-sweep** algorithm with reachability analysis
//! - **Background collection thread** that runs alongside application code
//! - **Incremental processing** to maintain low latency
//!
//! ### ⚡ **Smart Pointer Type**
//! - **[`Gc<T>`]**: Garbage-collected smart pointer with reachability-based collection
//! - **Thread-safe**: Full [`Send`] + [`Sync`] support for concurrent access
//! - **Cycle-safe**: Automatic detection and collection of circular references
//! - **[`Finalize`]**: Optional custom cleanup for objects before deallocation
//!
//! ### 🛠️ **Configurable Performance**
//! - **Allocation thresholds**: Trigger collection based on memory usage
//! - **Time budgets**: Configurable maximum pause times per collection step
//! - **Step sizes**: Adjustable incremental processing batch sizes
//! - **Concurrent sweeping**: Optional background memory reclamation
//!
//! ## Architecture
//!
//! ### Reachability Analysis Algorithm
//!
//! The garbage collector implements a pure reachability analysis with concurrent collection:
//!
//! 1. **Initial Mark Phase**
//!    - Brief pause to mark root objects
//!    - Pauseless for small root sets
//!    - Prepares concurrent marking phase
//!
//! 2. **Concurrent Mark Phase**
//!    - Mark reachable objects concurrently with application
//!    - Incremental processing with configurable time budgets
//!    - Tri-color marking for correctness
//!
//! 3. **Final Mark Phase**
//!    - Brief pause to handle objects modified during concurrent marking
//!    - Processes write barrier notifications
//!    - Ensures marking completeness
//!
//! 4. **Concurrent Sweep Phase**
//!    - Reclaim unmarked objects in background
//!    - Yields regularly to application threads
//!    - Optional concurrent operation
//!
//! ### Tri-Color Marking Algorithm
//!
//! The garbage collector uses the tri-color marking algorithm to ensure correctness and efficiency during concurrent
//! collection. Objects are classified into three colors:
//!
//! - **White**: Unreachable objects, candidates for reclamation.
//! - **Gray**: Reachable objects whose references have not yet been fully scanned.
//! - **Black**: Reachable objects whose references have been completely scanned.
//!
//! During the mark phases:
//! - The collector starts by marking root objects gray.
//! - It scans gray objects, marking their referenced objects gray and turning the scanned object black.
//! - This process continues until no gray objects remain.
//! - At the end, white objects are considered unreachable and are reclaimed during the sweep phase.
//!
//! Tri-color marking, combined with write barriers, ensures that objects modified during concurrent marking are correctly
//! processed, preventing premature reclamation and maintaining application correctness.
//!
//! ### Performance Characteristics
//!
//! - **Pause Times**: Sub-millisecond (default 100μs maximum)
//! - **Throughput**: Minimal impact on application performance
//! - **Memory Overhead**: Low overhead tri-color marking
//! - **Scalability**: Configurable for different workload patterns
//!
//! ### Thread Safety
//!
//! All operations are fully thread-safe:
//! - Lock-free fast paths for common operations
//! - Safe concurrent collection with proper synchronization
//! - Cross-thread garbage collection coordination
//!
//! ## Safety and Correctness
//!
//! - **Memory Safety**: All operations are memory-safe with no dangling pointers
//! - **Cycle Detection**: Automatic detection and collection of circular references
//! - **Concurrent Correctness**: Proper synchronization for multithreaded environments
//! - **Leak Prevention**: Guaranteed collection of unreachable object graphs
//!
//! ## Integration with Ristretto VM
//!
//! This garbage collector is specifically designed for the Ristretto JVM implementation:
//! - **Low-latency requirements**: Suitable for real-time and interactive applications
//! - **High-throughput**: Minimal impact on JVM bytecode execution
//! - **Memory efficiency**: Compact object representation and fast allocation
//! - **JVM compatibility**: Supports Java memory model requirements

#![allow(dead_code)]
#![deny(clippy::pedantic)]
#![deny(clippy::unwrap_in_result)]
#![deny(clippy::unwrap_used)]

mod collector;
mod config;
mod error;
mod finalizer;
mod gc;
mod gc_box;
mod metadata;
mod pointers;
mod root_guard;

pub use collector::{GC, GarbageCollector, Trace};
pub use config::{Configuration, Statistics};
pub use error::{Error, Result};
pub use finalizer::Finalize;
pub use gc::Gc;
pub use root_guard::GcRootGuard;
