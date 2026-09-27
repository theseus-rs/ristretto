use proc_macro2::TokenStream;
use quote::quote;
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
        if input.peek(syn::Token![,]) {
            input.parse::<syn::Token![,]>()?;
        }
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
    if let Err(message) = validate_signature(&signature_lit.value()) {
        return syn::Error::new_spanned(signature_lit, message).to_compile_error();
    }
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
    if input_fn.sig.asyncness.is_some() {
        input_fn.attrs.push(syn::parse_quote! {
            #[allow(clippy::unused_async, reason = "the declared async calling convention may suspend on other targets")]
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
    // Rust checks the version expression; the crate-owned generator checks active duplicates.
    let generated_registration_code = quote! {
        const _: ristretto_classfile::VersionSpecification = #version_specification_expr;
    };

    // Ordinary async bodies stay unboxed. The typed registry boxes only at dispatch.
    // An explicit async_method attribute remains available for recursive cycles.
    let function = quote! { #input_fn };

    // Output the function definition and the generated registration logic.
    let output = quote! {
        // The original function definition, with original visibility
        #function
        // The generated static item
        #generated_registration_code
    };

    output
}

fn validate_signature(signature: &str) -> Result<(), String> {
    let (owner, descriptor) = signature
        .split_once('(')
        .ok_or("missing method descriptor")?;
    let (class, method) = owner
        .rsplit_once('.')
        .ok_or("missing class or method name")?;
    if class.is_empty()
        || class.split('/').any(str::is_empty)
        || class.contains(['.', ';', '['])
        || method.is_empty()
        || method.contains(['/', '.', ';', '['])
    {
        return Err(format!("invalid intrinsic signature {signature}"));
    }
    ristretto_classfile::FieldType::parse_method_descriptor(
        &ristretto_classfile::JavaStr::cow_from_str(&format!("({descriptor}")),
    )
    .map(|_| ())
    .map_err(|error| error.to_string())
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
        assert!(output.contains("const _ : ristretto_classfile :: VersionSpecification"));
        assert!(output.contains("Any"));
        assert!(!output.contains("async_recursion"));
    }

    #[test]
    fn process_keeps_async_bodies_unboxed() {
        let output = process(
            quote! { "pkg/Example.run()I", Any },
            quote! { pub async fn run() -> u8 { 7 } },
        )
        .to_string();
        assert!(!output.contains("async_recursion"));
        assert!(!output.contains("Box"));
        let recursive = process(
            quote! { "pkg/Example.run()I", Any },
            quote! { #[async_method] pub async fn run() -> u8 { 7 } },
        )
        .to_string();
        assert!(recursive.contains("async_method"));
    }

    #[test]
    fn process_rejects_bad_descriptors() {
        for attributes in [
            quote! { "pkg/Example.run(Q)V", Any },
            quote! { "pkg/Example.run()II", Any },
            quote! { "run()V", Any },
        ] {
            assert!(
                process(attributes, quote! { fn run() {} })
                    .to_string()
                    .contains("compile_error")
            );
        }
        assert!(
            !process(quote! { "pkg/Example.run()V", Any }, quote! { fn run() {} })
                .to_string()
                .contains("compile_error")
        );
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
        assert!(output.contains("const _ : ristretto_classfile :: VersionSpecification"));
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
