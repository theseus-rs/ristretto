# Ristretto GC

[![ci](https://github.com/theseus-rs/ristretto/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/theseus-rs/ristretto/actions/workflows/ci.yml)
[![Documentation](https://docs.rs/ristretto_gc/badge.svg)](https://docs.rs/ristretto_gc)
[![Code Coverage](https://codecov.io/gh/theseus-rs/ristretto/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/ristretto)
[![Benchmarks](https://img.shields.io/badge/%F0%9F%90%B0_bencher-enabled-6ec241)](https://bencher.dev/perf/theseus-rs-ristretto)
[![Latest version](https://img.shields.io/crates/v/ristretto_gc.svg)](https://crates.io/crates/ristretto_gc)
[![License](https://img.shields.io/crates/l/ristretto_gc)](https://github.com/theseus-rs/ristretto#license)
[![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)

## Overview

A pauseless, concurrent and parallel mark and sweep garbage collector implementation for the Ristretto VM. This crate
provides `Gc<T>` types for garbage-collected references, using a pure reachability analysis algorithm with automatic
cycle detection and collection.

## Features

### **Pauseless Concurrent Collection**

- **Sub-millisecond pause times** (default: 100 microseconds maximum)
- **Concurrent mark-and-sweep** algorithm with 4 distinct phases
- **Background collection thread** that runs alongside application code
- **Incremental processing** to maintain low latency

### **Parallel Processing**

- **Parallel marking and sweeping** for multi-core utilization
- **Configurable parallel threshold** (default: 1,000,000 objects)
- **Automatic fallback** to sequential processing for smaller object sets
- **Work-stealing parallelism** for efficient load distribution

###️ **Smart Pointer Type**

- **`Gc<T>`**: Garbage-collected smart pointer with reachability-based collection
- **Thread-safe**: Full `Send` + `Sync` support for concurrent access
- **Cycle-safe**: Automatic detection and collection of circular references
- **[`Finalize`]**: Optional custom cleanup for objects before deallocation

### **Configurable Performance**

- **Allocation thresholds**: Trigger collection based on memory usage (default: 8MB)
- **Time budgets**: Configurable maximum pause times per collection step
- **Step sizes**: Adjustable incremental processing batch sizes (default: 1000 objects)
- **Parallel threshold**: Objects count threshold for parallel vs sequential processing

## Architecture

### Parallel Concurrent Mark-and-Sweep Algorithm

The garbage collector implements a sophisticated 4-phase parallel concurrent collection cycle:

1. **Initial Mark Phase**
    - Brief pause to mark root objects
    - Parallel unmarking of all objects when object count exceeds threshold
    - Sequential processing for smaller object sets to avoid parallelization overhead
    - Prepares concurrent marking phase

2. **Concurrent Mark Phase**
    - Mark reachable objects concurrently with application
    - Incremental processing with configurable time budgets
    - Write barriers to handle concurrent modifications

3. **Final Mark Phase**
    - Brief pause to handle objects modified during concurrent marking
    - Processes write barrier notifications
    - Ensures marking completeness

4. **Concurrent Sweep Phase**
    - Parallel identification of unmarked objects
    - Automatic selection between parallel and sequential sweeping
    - Reclaim unmarked objects in background
    - Yields regularly to application threads

### Parallel Processing Strategy

The collector automatically selects between parallel and sequential processing based on workload characteristics:

- **Parallel Processing**: Used when object count exceeds the configured threshold (default: 1,000,000 objects)
    - Utilizes work-stealing thread pool for optimal performance
    - Parallel iteration over object collections during mark and sweep phases
    - Efficient for large heaps with many objects

- **Sequential Processing**: Used for smaller object sets
    - Avoids parallelization overhead for small workloads
    - Single-threaded iteration for optimal cache locality
    - Better performance for applications with smaller heaps

### Performance Characteristics

- **Pause Times**: Sub-millisecond (default 100μs maximum)
- **Throughput**: Minimal impact on application performance with parallel efficiency
- **Scalability**: Configurable for different workload patterns and core counts
- **Multi-core Utilization**: Automatic scaling to available CPU cores during collection

### Thread Safety

All operations are fully thread-safe with enhanced parallel processing:

- Lock-free fast paths for common operations
- Safe concurrent collection with proper synchronization
- Cross-thread garbage collection coordination
- Work-stealing parallel processing

## Safety and Correctness

- **Memory Safety**: All operations are memory-safe with no dangling pointers
- **Cycle Detection**: Automatic detection and collection of circular references
- **Concurrent Correctness**: Proper synchronization for multithreaded environments
- **Parallel Safety**: Thread-safe parallel processing with proper coordination
- **Leak Prevention**: Guaranteed collection of unreachable object graphs

## Integration with Ristretto VM

This garbage collector is specifically designed for the Ristretto JVM implementation:

- **Low-latency requirements**: Suitable for real-time and interactive applications
- **High-throughput**: Minimal impact on JVM bytecode execution with parallel efficiency
- **Memory efficiency**: Compact object representation and fast allocation
- **JVM compatibility**: Supports Java memory model requirements
- **Multi-core optimization**: Leverages modern multi-core processors for collection

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
