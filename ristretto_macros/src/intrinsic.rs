use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream, Result as SynResult};
use syn::{Expr, ItemFn, LitStr, parse_macro_input};

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
    let arguments = parse_macro_input!(attributes as IntrinsicMethodArgs);
    let signature_lit = &arguments.signature;
    let version_specification_expr = &arguments.version_specification;

    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_vis = &input_fn.vis;

    // Generate a unique identifier for the static registration item to ensure registrations do not
    // conflict.
    let constant_name = signature_lit
        .value()
        .replace(['/', '$', '.', '(', ')', ';', '['], "_");
    let registration_ident = format_ident!("_{constant_name}_INTRINSIC_DATA");

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

    TokenStream::from(output)
}
