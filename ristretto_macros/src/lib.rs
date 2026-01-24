mod async_method;
mod intrinsic;

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
/// ```text
/// #[intrinsic_method("java/lang/Object.hashCode()I", Any)]
/// #[async_recursion(?Send)]
/// async fn hash_code(_thread: Arc<Thread>, mut parameters: Parameters) -> Result<Option<Value>> {
///     // actual logic
///    ...
/// }
/// ```
///
/// The macro preserves the original Rust function and generates a static item (currently a tuple
/// `(&'static str, &'static str, ristretto_classfile::VersionSpecification)`) that associates the
/// full intrinsic name, the Rust function's name, and the version specification.
#[proc_macro_attribute]
pub fn intrinsic_method(attributes: TokenStream, item: TokenStream) -> TokenStream {
    intrinsic::process(attributes, item)
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
/// #[intrinsic_method("java/lang/Object.hashCode()I", Any)]
/// #[async_method]
/// async fn hash_code(_thread: Arc<Thread>, mut parameters: Parameters) -> Result<Option<Value>> {
///     // actual logic
///    ...
/// }
/// ```
#[proc_macro_attribute]
pub fn async_method(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    async_method::process(item)
}
