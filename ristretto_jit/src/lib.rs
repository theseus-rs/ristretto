//! # Ristretto JIT
//!
//! [![Code Coverage](https://codecov.io/gh/theseus-rs/ristretto/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/ristretto)
//! [![Benchmarks](https://img.shields.io/badge/%F0%9F%90%B0_bencher-enabled-6ec241)](https://bencher.dev/perf/theseus-rs-ristretto)
//! [![License](https://img.shields.io/crates/l/ristretto_jit)](https://github.com/theseus-rs/ristretto#license)
//! [![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)
//!
//! Ristretto JIT provides a Just-In-Time compiler for the Ristretto VM. The JIT compiler generates
//! native code from Ristretto VM bytecode, allowing for high-performance execution directly on the
//! host machine.
//!
//! ## Platform Support
//!
//! The JIT compiler automatically adapts to the target platform:
//!
//! - **Native platforms**:
//!   - x86-64 (Intel/AMD 64-bit)
//!   - aarch64 (ARM 64-bit)
//!   - s390x (IBM Z Architecture)
//!   - riscv64 (RISC-V 64-bit)
//!
//! ## Limitations
//!
//! Current limitations include:
//!
//! - `invokestatic`, `invokespecial`, `invokevirtual` and `invokeinterface` have experimental native lowering
//!   through the VM runtime bridge or owning call continuations;
//!   production eligibility still excludes calls and reference operations. Static primitive
//!   control flow is supported; native loops use owning, resumable activations.
//! - `invokedynamic` remains unsupported.

#![allow(unsafe_code)]
#![cfg_attr(
    test,
    expect(
        clippy::panic_in_result_fn,
        reason = "unit tests use assertions with Result-returning tests"
    )
)]
#[cfg(all(
    not(target_family = "wasm"),
    target_endian = "little",
    not(any(
        target_arch = "mips",
        target_arch = "mips64",
        target_os = "dragonfly",
        target_os = "solaris"
    ))
))]
mod compiler;
#[cfg(all(
    not(target_family = "wasm"),
    target_endian = "little",
    not(any(
        target_arch = "mips",
        target_arch = "mips64",
        target_os = "dragonfly",
        target_os = "solaris"
    ))
))]
mod control_flow_graph;
mod error;
mod function;
#[cfg(all(
    not(target_family = "wasm"),
    target_endian = "little",
    not(any(
        target_arch = "mips",
        target_arch = "mips64",
        target_os = "dragonfly",
        target_os = "solaris"
    ))
))]
mod instruction;
mod jit_value;
#[cfg(all(
    not(target_family = "wasm"),
    target_endian = "little",
    not(any(
        target_arch = "mips",
        target_arch = "mips64",
        target_os = "dragonfly",
        target_os = "solaris"
    ))
))]
mod local_type;
#[cfg(all(
    not(target_family = "wasm"),
    target_endian = "little",
    not(any(
        target_arch = "mips",
        target_arch = "mips64",
        target_os = "dragonfly",
        target_os = "solaris"
    ))
))]
mod local_variables;
#[cfg(all(
    not(target_family = "wasm"),
    target_endian = "little",
    not(any(
        target_arch = "mips",
        target_arch = "mips64",
        target_os = "dragonfly",
        target_os = "solaris"
    ))
))]
mod operand_stack;
#[cfg(all(
    not(target_family = "wasm"),
    target_endian = "little",
    not(any(
        target_arch = "mips",
        target_arch = "mips64",
        target_os = "dragonfly",
        target_os = "solaris"
    ))
))]
mod runtime_helpers;
#[cfg(all(
    test,
    not(target_family = "wasm"),
    target_endian = "little",
    not(any(
        target_arch = "mips",
        target_arch = "mips64",
        target_os = "dragonfly",
        target_os = "solaris"
    ))
))]
mod test;
mod value;
#[cfg(any(
    target_family = "wasm",
    target_endian = "big",
    target_arch = "mips",
    target_arch = "mips64",
    target_os = "dragonfly",
    target_os = "solaris"
))]
mod wasm_compiler;

#[cfg(all(
    not(target_family = "wasm"),
    target_endian = "little",
    not(any(
        target_arch = "mips",
        target_arch = "mips64",
        target_os = "dragonfly",
        target_os = "solaris"
    ))
))]
pub use compiler::Compiler;
pub use error::{Error, Result};
pub use function::{
    Activation, AllocationKind, AllocationRequest, ArrayAccessKind, ArrayAccessRequest,
    ArrayStoreRequest, CallKind, CallRequest, ConstantRequest, ExceptionMatchRequest,
    ExecutionStep, FieldAccess, FieldRequest, Function, MonitorKind, MonitorRequest, TypeCheckKind,
    TypeCheckRequest,
};
pub(crate) use jit_value::JitValue;
pub use value::Value;

/// Maximum identities in the experimental compiled-frame root transfer ABI.
/// Larger reference-bearing frames remain unsupported until their stack use is qualified.
pub const FRAME_ROOT_CAPACITY: usize = 4096;

/// Maximum logical arguments (including an instance receiver) in the experimental compiled-call ABI. Each argument
/// occupies one eight-byte cell; long/double still count as two Java local slots.
pub const CALL_ARGUMENT_CAPACITY: usize = 255;
#[cfg(any(
    target_family = "wasm",
    target_endian = "big",
    target_arch = "mips",
    target_arch = "mips64",
    target_os = "dragonfly",
    target_os = "solaris"
))]
pub use wasm_compiler::Compiler;
