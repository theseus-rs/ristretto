use ahash::AHashMap;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use ristretto_classfile::{
    JAVA_8, JAVA_11, JAVA_17, JAVA_21, JAVA_25, Version, VersionSpecification,
};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, ExprCall, Item, LitStr, Result as SynResult};
use walkdir::WalkDir;

/// The supported Java versions for intrinsic methods.
const JAVA_VERSIONS: [(&str, &Version); 5] = [
    ("JAVA_8", &JAVA_8),
    ("JAVA_11", &JAVA_11),
    ("JAVA_17", &JAVA_17),
    ("JAVA_21", &JAVA_21),
    ("JAVA_25", &JAVA_25),
];

static RUST_KEYWORDS: &[&str] = &[
    "Self", "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum",
    "extern", "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move",
    "mut", "pub", "ref", "return", "self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while",
];

/// Returns the Java version based on the provided version string.
fn parse_java_version(version: &str) -> Version {
    match version {
        "JAVA_8" => JAVA_8,
        "JAVA_11" => JAVA_11,
        "JAVA_17" => JAVA_17,
        "JAVA_21" => JAVA_21,
        "JAVA_25" => JAVA_25,
        _ => panic!("Unsupported intrinsic method Java version: {version}"),
    }
}

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

/// Processing for the `generate_intrinsic_registry` procedural macro.
///
/// This scans `ristretto_intrinsics/src/` for `#[intrinsic_method]` attributes and generates
/// static PHF maps for each Java version, replacing the build script approach.
pub(crate) fn process(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();
    let source_path = if input_str.is_empty() {
        // Default path relative to the workspace root
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
        PathBuf::from(&manifest_dir)
            .join("..")
            .join("ristretto_intrinsics")
            .join("src")
    } else {
        let lit: LitStr = syn::parse(input).expect("Expected a string literal path");
        PathBuf::from(lit.value())
    };

    let intrinsic_methods = get_intrinsic_methods(&source_path)
        .unwrap_or_else(|e| panic!("Failed to get intrinsic methods: {e}"));

    let mut version_maps = Vec::new();
    for (version_name, version) in JAVA_VERSIONS {
        let map = generate_intrinsic_method_map(version_name, version, &intrinsic_methods);
        version_maps.push(map);
    }

    let output = quote! {
        use crate::intrinsic_methods::IntrinsicMethod;

        #(#version_maps)*
    };

    TokenStream::from(output)
}

/// Retrieves intrinsic methods from the source path.
fn get_intrinsic_methods(
    source_path: &PathBuf,
) -> Result<AHashMap<String, (String, VersionSpecification)>, Box<dyn std::error::Error>> {
    let mut intrinsic_methods = AHashMap::default();
    for entry in WalkDir::new(source_path.clone())
        .into_iter()
        .filter_map(Result::ok)
    {
        process_file_entry(source_path, &entry, &mut intrinsic_methods)?;
    }
    Ok(intrinsic_methods)
}

/// Processes a single directory entry.
fn process_file_entry(
    source_path: &PathBuf,
    entry: &walkdir::DirEntry,
    intrinsic_methods: &mut AHashMap<String, (String, VersionSpecification)>,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_name = entry.file_name().to_string_lossy();
    if !file_name.ends_with(".rs") {
        return Ok(());
    }
    let relative_path = entry
        .path()
        .strip_prefix(source_path)
        .unwrap_or(entry.path());
    let mut module = relative_path.to_string_lossy().to_string();
    module = module
        .strip_suffix("/mod.rs")
        .unwrap_or(&module)
        .to_string();
    module = module
        .strip_suffix("\\mod.rs")
        .unwrap_or(&module)
        .to_string();
    module = module.strip_suffix(".rs").unwrap_or(&module).to_string();
    module = module.replace(['/', '\\'], "::");
    for keyword in RUST_KEYWORDS {
        module = module.replace(&format!("::{keyword}"), &format!("::r#{keyword}"));
    }
    module = format!("ristretto_intrinsics::{module}");

    let mut file_content = String::new();
    let mut file = File::open(entry.path())?;
    file.read_to_string(&mut file_content)?;

    if let Ok(syn_file) = syn::parse_file(&file_content) {
        for item in syn_file.items {
            process_item(&module, &item, intrinsic_methods);
        }
    }
    Ok(())
}

/// Processes a `syn::Item` to find intrinsic method definitions.
fn process_item(
    module: &str,
    item: &Item,
    intrinsic_methods: &mut AHashMap<String, (String, VersionSpecification)>,
) {
    if let Item::Fn(function) = item {
        let attribute = function
            .attrs
            .iter()
            .find(|attribute| attribute.path().is_ident("intrinsic_method"));
        if let Some(attribute) = attribute
            && let Ok(arguments) = attribute.parse_args::<IntrinsicMethodArgs>()
        {
            let function_name = format!("{module}::{}", function.sig.ident);
            let signature = arguments.signature.value();
            let version_specification = version_specification(&arguments.version_specification);
            intrinsic_methods.insert(signature, (function_name, version_specification));
        }
    }
}

/// Parses the version specification expression into a `VersionSpecification`.
fn version_specification(expression: &Expr) -> VersionSpecification {
    if let Expr::Path(path) = expression {
        if path.path.is_ident("Any") {
            return VersionSpecification::Any;
        }
        panic!(
            "Unsupported version specification in intrinsic method attribute: {:?}",
            path.path.get_ident(),
        );
    }

    let Expr::Call(call) = expression else {
        panic!(
            "[call] Unsupported version specification in intrinsic method attribute: {expression:?}"
        );
    };
    let Expr::Path(function) = &*call.func else {
        panic!(
            "[call.path] Unsupported version specification in intrinsic method attribute: {expression:?}"
        );
    };
    let Some(specification_type) = function.path.get_ident() else {
        panic!(
            "[call.path.ident] Unsupported version specification in intrinsic method attribute: {expression:?}"
        );
    };
    let specification_type = specification_type.to_string();
    if specification_type == "In" {
        let versions = java_version_vec(call);
        // Leak the Vec to avoid lifetime issues with the VersionSpecification::In variant
        return VersionSpecification::In(versions.leak());
    }

    let version = java_version(call.args.first());
    match specification_type.as_str() {
        "Equal" => VersionSpecification::Equal(version),
        "NotEqual" => VersionSpecification::NotEqual(version),
        "LessThan" => VersionSpecification::LessThan(version),
        "LessThanOrEqual" => VersionSpecification::LessThanOrEqual(version),
        "GreaterThan" => VersionSpecification::GreaterThan(version),
        "GreaterThanOrEqual" => VersionSpecification::GreaterThanOrEqual(version),
        "Between" => {
            let end_version = java_version(call.args.get(1));
            VersionSpecification::Between(version, end_version)
        }
        _ => panic!(
            "Unsupported version specification in intrinsic method attribute \"{specification_type}\": {call:?}"
        ),
    }
}

/// Parses the Java versions from a call expression containing `&[version_1, version_2]`
fn java_version_vec(call: &ExprCall) -> Vec<Version> {
    let Some(Expr::Reference(reference_expr)) = call.args.first() else {
        panic!("(call.args[0]]) Unsupported expression in call: {call:?}");
    };
    let Expr::Array(array) = &*reference_expr.expr else {
        panic!("(call.args[0].expr) Unsupported expression in call: {call:?}");
    };
    let mut versions = Vec::new();
    for element in &array.elems {
        let Expr::Path(version_path) = element else {
            continue;
        };
        if let Some(segment) = version_path.path.segments.first() {
            let version = segment.ident.to_string();
            versions.push(parse_java_version(&version));
        }
    }
    versions
}

/// Returns the Java version based on the provided expression.
fn java_version(expression: Option<&Expr>) -> Version {
    if let Some(Expr::Path(path)) = expression
        && let Some(segment) = path.path.segments.first()
    {
        let version = segment.ident.to_string();
        return parse_java_version(&version);
    }
    panic!("Unsupported Java version in intrinsic method attribute: {expression:?}");
}

/// Generates a PHF map for a specific Java version as a token stream.
fn generate_intrinsic_method_map(
    version_name: &str,
    version: &Version,
    intrinsic_methods: &AHashMap<String, (String, VersionSpecification)>,
) -> TokenStream2 {
    let mut map_builder = phf_codegen::Map::<&str>::new();

    for (signature, (function, version_specification)) in intrinsic_methods {
        if !version_specification.matches(version) {
            continue;
        }
        let function = format!("{function}::<crate::thread::Thread> as IntrinsicMethod");
        map_builder.entry(signature, function);
    }

    let version_ident = syn::Ident::new(version_name, proc_macro2::Span::call_site());
    let map_str = map_builder.build().to_string();
    let map_tokens: TokenStream2 = map_str
        .parse()
        .unwrap_or_else(|e| panic!("Failed to parse generated PHF map: {e}"));

    quote! {
        #[deny(clippy::large_stack_arrays)]
        #[expect(clippy::unreadable_literal)]
        pub(crate) static #version_ident: phf::Map<&'static str, IntrinsicMethod> = #map_tokens;
    }
}
