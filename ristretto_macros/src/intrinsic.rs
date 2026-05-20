use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream, Result as SynResult};
use syn::{Expr, ItemFn, LitStr};

/// Helper struct for parsing macro attributes
struct IntrinsicMethodArgs {
    signature: LitStr,
    version_specification: Expr,
}

impl Parse for IntrinsicMethodArgs {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let signature: LitStr = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let version_spec: Expr = input.parse()?;
        Ok(IntrinsicMethodArgs {
            signature,
            version_specification: version_spec,
        })
    }
}

/// Processing for the `intrinsic_method` procedural macro.
pub(crate) fn process(attributes: TokenStream, item: TokenStream) -> TokenStream {
    let arguments = match syn::parse2::<IntrinsicMethodArgs>(attributes) {
        Ok(arguments) => arguments,
        Err(error) => return error.to_compile_error(),
    };
    let signature_lit = &arguments.signature;
    let version_specification_expr = &arguments.version_specification;

    let input_fn = match syn::parse2::<ItemFn>(item) {
        Ok(input_fn) => input_fn,
        Err(error) => return error.to_compile_error(),
    };
    let fn_name = &input_fn.sig.ident;
    let fn_vis = &input_fn.vis;

    // Generate a unique identifier for the static registration item to ensure registrations do not
    // conflict.
    let constant_name = signature_lit
        .value()
        .replace(['/', '$', '.', '(', ')', ';', '['], "_");
    let fn_name_str = fn_name.to_string();
    let fn_name_clean = fn_name_str.strip_prefix("r#").unwrap_or(&fn_name_str);
    let registration_ident = format_ident!("_{constant_name}_{fn_name_clean}_INTRINSIC_DATA");

    let intrinsic_name_expr = quote! { #signature_lit };

    // The generated static item will hold the intrinsic name, the function name, and the version
    // specification. This verifies:
    //
    // 1. The intrinsic name registration will not conflict within a single source file, as the
    //    identifier name generated based on the signature.
    // 2. The version specification is a valid `ristretto_classfile::VersionSpecification`.
    let generated_registration_code = quote! {
        #[doc(hidden)]
        #[allow(non_upper_case_globals)]
        #fn_vis static #registration_ident: (&'static str, &'static str, ristretto_classfile::VersionSpecification) =
            (#intrinsic_name_expr, stringify!(#fn_name), #version_specification_expr);
    };

    // Output the original function definition and the generated registration logic.
    let output = quote! {
        // The original function definition, with original visibility
        #input_fn
        // The generated static item
        #generated_registration_code
    };

    output
}

#[cfg(test)]
mod tests {
    use super::process;
    use quote::quote;

    #[test]
    fn process_preserves_function_and_generates_registration() {
        let output = process(
            quote! { "java/lang/Object.hashCode()I", Any },
            quote! {
                pub fn hash_code() -> i32 {
                    42
                }
            },
        )
        .to_string();

        assert!(output.contains("pub fn hash_code"));
        assert!(
            output.contains("pub static _java_lang_Object_hashCode__I_hash_code_INTRINSIC_DATA")
        );
        assert!(output.contains("\"java/lang/Object.hashCode()I\""));
        assert!(output.contains("stringify ! (hash_code)"));
        assert!(output.contains("Any"));
    }

    #[test]
    fn process_cleans_raw_identifier_for_registration_name() {
        let output = process(
            quote! { "pkg/Example.$init([I)V", Any },
            quote! {
                fn r#type() {}
            },
        )
        .to_string();

        assert!(output.contains("fn r#type"));
        assert!(output.contains("static _pkg_Example__init__I_V_type_INTRINSIC_DATA"));
        assert!(output.contains("stringify ! (r#type)"));
    }

    #[test]
    fn process_returns_compile_error_for_invalid_attributes() {
        let output = process(
            quote! { "java/lang/Object.hashCode()I" },
            quote! { fn hash_code() {} },
        )
        .to_string();

        assert!(output.contains("compile_error"));
    }

    #[test]
    fn process_returns_compile_error_for_non_string_signature() {
        let output = process(quote! { 123, Any }, quote! { fn hash_code() {} }).to_string();

        assert!(output.contains("compile_error"));
    }

    #[test]
    fn process_returns_compile_error_for_missing_version_expression() {
        let output = process(
            quote! { "java/lang/Object.hashCode()I", },
            quote! { fn hash_code() {} },
        )
        .to_string();

        assert!(output.contains("compile_error"));
    }

    #[test]
    fn process_returns_compile_error_for_invalid_item() {
        let output = process(
            quote! { "java/lang/Object.hashCode()I", Any },
            quote! { struct NotAFunction; },
        )
        .to_string();

        assert!(output.contains("compile_error"));
    }
}
