use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

/// Processing for the `async_method` procedural macro.
///
/// This macro applies `async_recursion` with the appropriate `Send` bound:
/// - On non-WASM targets: uses `#[async_recursion]` (with `Send` bound) for `tokio::spawn`
///   compatibility
/// - On WASM targets: uses `#[async_recursion(?Send)]` since WASM types are not `Send`
pub(crate) fn process(item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    let output = quote! {
        #[cfg_attr(not(target_family = "wasm"), async_recursion::async_recursion)]
        #[cfg_attr(target_family = "wasm", async_recursion::async_recursion(?Send))]
        #input_fn
    };

    TokenStream::from(output)
}
