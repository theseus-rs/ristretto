use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemFn;

/// Processing for the `async_method` procedural macro.
///
/// This macro applies `async_recursion` with the appropriate `Send` bound:
/// - On non-WASM targets: uses `#[async_recursion]` (with `Send` bound) for `tokio::spawn`
///   compatibility
/// - On WASM targets: uses `#[async_recursion(?Send)]` since WASM types are not `Send`
pub(crate) fn process(item: TokenStream) -> TokenStream {
    let input_fn = match syn::parse2::<ItemFn>(item) {
        Ok(input_fn) => input_fn,
        Err(error) => return error.to_compile_error(),
    };

    quote! {
        #[cfg_attr(not(target_family = "wasm"), async_recursion::async_recursion)]
        #[cfg_attr(target_family = "wasm", async_recursion::async_recursion(?Send))]
        #input_fn
    }
}

#[cfg(test)]
mod tests {
    use super::process;
    use quote::quote;

    #[test]
    fn process_adds_async_recursion_cfg_attributes() {
        let output = process(quote! {
            pub async fn run(value: u8) -> u8 {
                value
            }
        })
        .to_string();

        assert!(output.contains("cfg_attr"));
        assert!(output.contains("not (target_family = \"wasm\")"));
        assert!(output.contains("async_recursion :: async_recursion"));
        assert!(output.contains("target_family = \"wasm\""));
        assert!(output.contains("? Send"));
        assert!(output.contains("pub async fn run"));
    }

    #[test]
    fn process_returns_compile_error_for_invalid_function() {
        let output = process(quote! {
            struct NotAFunction;
        })
        .to_string();

        assert!(output.contains("compile_error"));
    }
}
