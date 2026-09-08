//! # Ristretto Macros
//!
//! [![Code Coverage](https://codecov.io/gh/theseus-rs/ristretto/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/ristretto)
//! [![Benchmarks](https://img.shields.io/badge/%F0%9F%90%B0_bencher-enabled-6ec241)](https://bencher.dev/perf/theseus-rs-ristretto)
//! [![License](https://img.shields.io/crates/l/ristretto_macros)](https://github.com/theseus-rs/ristretto#license)
//! [![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)
//!
//! This crate provides procedural macros for the Ristretto JVM.

mod async_method;
mod intrinsic;
mod intrinsic_registry;

extern crate proc_macro;

use proc_macro::TokenStream;

/// A procedural attribute macro to mark a Rust function as an intrinsic method for the Ristretto
/// VM.
///
/// This macro is intended to be used on functions that provide native implementations for specific
/// Java methods, which the Ristretto VM can then call directly.
///
/// # Usage
///
/// The macro takes two arguments:
/// 1. `signature`: A string literal representing the full method signature, including the class
///    name, method name, and method descriptor
///    (e.g., `"java/lang/System.currentTimeMillis:()J"`).
/// 2. `version_specification`: A `ristretto_classfile::VersionSpecification` enum variant that
///    specifies the Java versions for which this intrinsic is applicable
///    (e.g., `VersionSpecification::Any` or `VersionSpecification::GreaterThanOrEqual(JAVA_17)`).
///
/// The macro generates a static item that associates the full intrinsic identifier, the Rust
/// function's name, and the version specification.  This static is only used for compile time
/// verification of `signature` uniqueness within a source file and proper definition of the
/// `version_specification` enum variant.
///
/// # Examples
///
/// Synchronous intrinsics return directly, without allocating a future:
///
/// ```text
/// #[intrinsic_method("java/lang/Float.floatToRawIntBits(F)I", Any)]
/// pub fn float_to_raw_int_bits<T: Thread + 'static>(
///     _thread: Arc<T>, mut parameters: Parameters,
/// ) -> Result<Option<Value>> {
///     Ok(Some(Value::Int(parameters.pop_float()?.to_bits() as i32)))
/// }
/// ```
///
/// Async intrinsics automatically receive platform-aware recursive future boxing:
///
/// ```text
/// #[intrinsic_method("java/lang/Object.getClass()Ljava/lang/Class;", Any)]
/// pub async fn get_class<T: Thread + 'static>(
///     thread: Arc<T>, mut parameters: Parameters,
/// ) -> Result<Option<Value>> {
///     // Async class loading and Java calls can be awaited here.
///     ...
/// }
/// ```
///
/// The macro detects the `async` declaration, rather than inferring it from the body.
/// Async futures are `Send` on native targets and may be non-`Send` on WebAssembly.
/// An existing `#[async_method]` immediately below this attribute is accepted without
/// double boxing, but is unnecessary. The generated registration metadata preserves
/// the intrinsic name, Rust function name, and Java version specification.
#[proc_macro_attribute]
pub fn intrinsic_method(attributes: TokenStream, item: TokenStream) -> TokenStream {
    intrinsic::process(attributes.into(), item.into()).into()
}

/// A procedural attribute macro that applies `async_recursion` with platform-appropriate Send bounds.
///
/// This macro wraps `async_recursion` and handles the `Send` bound automatically:
/// - On non-WASM targets: applies `#[async_recursion]` (with `Send` bound) for `tokio::spawn` compatibility
/// - On WASM targets: applies `#[async_recursion(?Send)]` since WASM types like `JsValue` are not `Send`
///
/// # Usage
///
/// Use this macro instead of `#[async_recursion]` for async functions that need to work on both
/// WASM and non-WASM targets:
///
/// ```text
/// #[async_method]
/// async fn hash_code(_thread: Arc<Thread>, mut parameters: Parameters) -> Result<Option<Value>> {
///     // actual logic
///    ...
/// }
/// ```
#[proc_macro_attribute]
pub fn async_method(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    async_method::process(item.into()).into()
}

/// A procedural macro that generates the intrinsic method registry at compile time.
///
/// This macro scans the `ristretto_intrinsics/src/` directory for functions annotated with
/// `#[intrinsic_method]`, extracts their signatures and version specifications, and generates
/// static `LazyLock<AHashMap>` registries for each supported Java version.
///
/// # Usage
///
/// ```text
/// generate_intrinsic_registry!();
/// ```
///
/// This generates static registry maps named `JAVA_8`, `JAVA_11`, `JAVA_17`, `JAVA_21`, and `JAVA_25`,
/// each mapping method signatures to their intrinsic method implementations.
#[proc_macro]
pub fn generate_intrinsic_registry(input: TokenStream) -> TokenStream {
    intrinsic_registry::process(input.into()).into()
}
