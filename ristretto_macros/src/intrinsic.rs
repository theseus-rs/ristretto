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

    let mut input_fn = match syn::parse2::<ItemFn>(item) {
        Ok(input_fn) => input_fn,
        Err(error) => return error.to_compile_error(),
    };
    // Every registry entry shares an owned-argument, fallible calling convention, including
    // implementations that happen not to consume their arguments or produce an error.
    if input_fn.sig.asyncness.is_none() {
        input_fn.attrs.push(syn::parse_quote! {
            #[allow(
                clippy::needless_pass_by_value,
                clippy::unnecessary_wraps,
                reason = "intrinsic registry entries share an owned-argument, fallible calling convention"
            )]
        });
    }
    let returns_result = matches!(&input_fn.sig.output, syn::ReturnType::Type(_, ty)
        if matches!(ty.as_ref(), syn::Type::Path(path)
            if path.path.segments.last().is_some_and(|segment| segment.ident == "Result")));
    let documents_errors = input_fn.attrs.iter().any(|attribute| {
        attribute.path().is_ident("doc")
            && matches!(&attribute.meta, syn::Meta::NameValue(value)
                if matches!(&value.value, Expr::Lit(literal)
                    if matches!(&literal.lit, syn::Lit::Str(text) if text.value().contains("# Errors"))))
    });
    if returns_result && !documents_errors {
        input_fn.attrs.push(syn::parse_quote! {
            #[doc = "\n# Errors\n\nPropagates errors from argument decoding or the intrinsic implementation."]
        });
        input_fn.attrs.push(syn::parse_quote! {
            #[allow(clippy::missing_errors_doc, reason = "the intrinsic macro supplies the standard error contract")]
        });
    }
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

    // Intrinsics own their boxing so callers need only one attribute. Consume the legacy
    // paired attribute before applying the platform-aware transformation exactly once.
    let function = if input_fn.sig.asyncness.is_some() {
        input_fn
            .attrs
            .retain(|attribute| !attribute.path().is_ident("async_method"));
        crate::async_method::process(quote! { #input_fn })
    } else {
        quote! { #input_fn }
    };

    // Output the function definition and the generated registration logic.
    let output = quote! {
        // The original function definition, with original visibility
        #function
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
        assert!(!output.contains("async_recursion"));
    }

    #[test]
    fn process_boxes_async_functions_once_with_platform_bounds() {
        for function in [
            quote! { pub async fn run() -> u8 { 7 } },
            quote! { #[async_method] pub async fn run() -> u8 { 7 } },
        ] {
            let output = process(quote! { "pkg/Example.run()I", Any }, function).to_string();
            assert!(output.contains("pub async fn run"));
            assert_eq!(output.matches("cfg_attr").count(), 2);
            assert!(output.contains("? Send"));
            assert!(!output.contains("# [async_method]"));
            assert!(output.contains("_pkg_Example_run__I_run_INTRINSIC_DATA"));
        }
    }

    #[test]
    fn process_documents_result_errors_without_replacing_explicit_docs() {
        let generated = process(
            quote! { "pkg/Example.run()I", Any },
            quote! { pub fn run() -> Result<Option<Value>> { Ok(None) } },
        )
        .to_string();
        assert!(generated.contains("# Errors"));
        assert!(generated.contains("Propagates errors"));

        let documented = process(
            quote! { "pkg/Example.run()I", Any },
            quote! {
                /// # Errors
                /// Returns an error when the input is invalid.
                pub fn run() -> Result<Option<Value>> { Ok(None) }
            },
        )
        .to_string();
        assert!(documented.contains("Returns an error when the input is invalid."));
        assert!(!documented.contains("Propagates errors"));
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
