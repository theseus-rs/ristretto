use ahash::AHashMap;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use ristretto_classfile::{
    JAVA_8, JAVA_11, JAVA_17, JAVA_21, JAVA_25, Version, VersionSpecification,
};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, ExprCall, Item, LitStr, Meta, Result as SynResult};
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

/// Data for a single intrinsic method.
struct IntrinsicMethodData {
    function_name: String,
    version_specification: VersionSpecification,
    /// Cfg conditions that must all be satisfied for this method to be available.
    /// Each string is the token content inside `#[cfg(...)]`.
    cfg_conditions: Vec<String>,
}

/// Processing for the `generate_intrinsic_registry` procedural macro.
///
/// This scans `ristretto_intrinsics/src/` for `#[intrinsic_method]` attributes and generates
/// static `LazyLock<AHashMap>` maps for each Java version. Cfg conditions on modules and
/// individual functions are detected and propagated as `#[cfg(...)]` guards on the generated
/// map insertion code, allowing arbitrary conditional compilation attributes to work with
/// intrinsic methods.
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
        .unwrap_or_else(|error| panic!("Failed to get intrinsic methods: {error}"));

    let mut version_maps = Vec::new();
    for (version_name, version) in JAVA_VERSIONS {
        let map = generate_intrinsic_method_map(version_name, version, &intrinsic_methods);
        version_maps.push(map);
    }

    let output = quote! {
        use crate::intrinsic_methods::IntrinsicMethod;
        use std::sync::LazyLock;
        use ahash::AHashMap;

        #(#version_maps)*
    };

    TokenStream::from(output)
}

/// Retrieves intrinsic methods from the source path.
fn get_intrinsic_methods(
    source_path: &PathBuf,
) -> Result<AHashMap<String, IntrinsicMethodData>, Box<dyn std::error::Error>> {
    // First pass: collect cfg-gated module paths from mod.rs and lib.rs files.
    // Maps module path -> list of cfg condition strings.
    let mut cfg_gated_modules: AHashMap<String, Vec<String>> = AHashMap::default();
    for entry in WalkDir::new(source_path.clone())
        .into_iter()
        .filter_map(Result::ok)
    {
        let file_name = entry.file_name().to_string_lossy();
        if file_name == "mod.rs" || file_name == "lib.rs" {
            collect_cfg_gated_modules(source_path, &entry, &mut cfg_gated_modules)?;
        }
    }

    // Second pass: collect intrinsic methods with their cfg conditions
    let mut intrinsic_methods = AHashMap::default();
    for entry in WalkDir::new(source_path.clone())
        .into_iter()
        .filter_map(Result::ok)
    {
        process_file_entry(
            source_path,
            &entry,
            &cfg_gated_modules,
            &mut intrinsic_methods,
        )?;
    }
    Ok(intrinsic_methods)
}

/// Collects module names that have `#[cfg(...)]` attributes from mod.rs and lib.rs files.
fn collect_cfg_gated_modules(
    source_path: &PathBuf,
    entry: &walkdir::DirEntry,
    gated_modules: &mut AHashMap<String, Vec<String>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file_content = String::new();
    let mut file = File::open(entry.path())?;
    file.read_to_string(&mut file_content)?;

    let relative_dir = entry
        .path()
        .parent()
        .unwrap_or(entry.path())
        .strip_prefix(source_path)
        .unwrap_or(entry.path().parent().unwrap_or(entry.path()));
    let dir_module = relative_dir.to_string_lossy().replace(['/', '\\'], "::");

    if let Ok(syn_file) = syn::parse_file(&file_content) {
        for item in &syn_file.items {
            if let Item::Mod(item_mod) = item {
                let cfg_conditions: Vec<String> = item_mod
                    .attrs
                    .iter()
                    .filter(|attr| attr.path().is_ident("cfg"))
                    .filter_map(extract_cfg_content)
                    .collect();

                if !cfg_conditions.is_empty() {
                    let mod_name = item_mod.ident.to_string();
                    let full_path = if dir_module.is_empty() {
                        mod_name
                    } else {
                        format!("{dir_module}::{mod_name}")
                    };
                    gated_modules.insert(full_path, cfg_conditions);
                }
            }
        }
    }
    Ok(())
}

/// Extracts the token content inside `#[cfg(...)]` as a string.
fn extract_cfg_content(attr: &syn::Attribute) -> Option<String> {
    if let Meta::List(meta_list) = &attr.meta {
        let tokens = meta_list.tokens.to_string();
        if tokens.is_empty() {
            return None;
        }
        Some(tokens)
    } else {
        None
    }
}

/// Processes a single directory entry to find intrinsic methods.
fn process_file_entry(
    source_path: &PathBuf,
    entry: &walkdir::DirEntry,
    cfg_gated_modules: &AHashMap<String, Vec<String>>,
    intrinsic_methods: &mut AHashMap<String, IntrinsicMethodData>,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_name = entry.file_name().to_string_lossy();
    if !file_name.ends_with(".rs") {
        return Ok(());
    }
    let relative_path = entry
        .path()
        .strip_prefix(source_path)
        .unwrap_or(entry.path());
    let mut raw_module = relative_path.to_string_lossy().to_string();
    raw_module = raw_module
        .strip_suffix("/mod.rs")
        .unwrap_or(&raw_module)
        .to_string();
    raw_module = raw_module
        .strip_suffix("\\mod.rs")
        .unwrap_or(&raw_module)
        .to_string();
    raw_module = raw_module
        .strip_suffix(".rs")
        .unwrap_or(&raw_module)
        .to_string();
    raw_module = raw_module.replace(['/', '\\'], "::");

    // Escape Rust keywords in path segments for code generation
    let escaped_module = raw_module
        .split("::")
        .map(|segment| {
            if RUST_KEYWORDS.contains(&segment) {
                format!("r#{segment}")
            } else {
                segment.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("::");
    let module = format!("ristretto_intrinsics::{escaped_module}");

    let mut file_content = String::new();
    let mut file = File::open(entry.path())?;
    file.read_to_string(&mut file_content)?;

    if let Ok(syn_file) = syn::parse_file(&file_content) {
        // Use raw module path (no keyword escaping) for cfg matching,
        // since collect_cfg_gated_modules uses raw filesystem paths
        let mut module_cfg_conditions = Vec::new();
        for (gated_module, conditions) in cfg_gated_modules {
            if raw_module == *gated_module || raw_module.starts_with(&format!("{gated_module}::")) {
                module_cfg_conditions.extend(conditions.iter().cloned());
            }
        }

        for item in syn_file.items {
            process_item(&module, &item, &module_cfg_conditions, intrinsic_methods);
        }
    }
    Ok(())
}

/// Processes a `syn::Item` to find intrinsic method definitions.
fn process_item(
    module: &str,
    item: &Item,
    module_cfg_conditions: &[String],
    intrinsic_methods: &mut AHashMap<String, IntrinsicMethodData>,
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

            // Collect cfg conditions: inherit from modules + any on the function itself
            let mut cfg_conditions: Vec<String> = module_cfg_conditions.to_vec();
            for attr in &function.attrs {
                if attr.path().is_ident("cfg")
                    && let Some(content) = extract_cfg_content(attr)
                {
                    cfg_conditions.push(content);
                }
            }
            cfg_conditions.sort();
            cfg_conditions.dedup();

            intrinsic_methods.insert(
                signature,
                IntrinsicMethodData {
                    function_name,
                    version_specification,
                    cfg_conditions,
                },
            );
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

/// Generates a `LazyLock<AHashMap>` static for a specific Java version.
///
/// Entries are grouped by their cfg conditions. Each group is wrapped in the
/// appropriate `#[cfg(...)]` attribute(s) so that platform-specific intrinsic
/// methods are only included when compiling for the matching target.
fn generate_intrinsic_method_map(
    version_name: &str,
    version: &Version,
    intrinsic_methods: &AHashMap<String, IntrinsicMethodData>,
) -> TokenStream2 {
    // Group entries by their cfg conditions (sorted for deterministic output)
    let mut groups: BTreeMap<Vec<String>, Vec<(String, String)>> = BTreeMap::new();

    for (signature, data) in intrinsic_methods {
        if !data.version_specification.matches(version) {
            continue;
        }
        let function = format!(
            "{}::<crate::thread::Thread> as IntrinsicMethod",
            data.function_name
        );
        groups
            .entry(data.cfg_conditions.clone())
            .or_default()
            .push((signature.clone(), function));
    }

    // Sort entries within each group for deterministic output
    for entries in groups.values_mut() {
        entries.sort_by(|a, b| a.0.cmp(&b.0));
    }

    let version_ident = syn::Ident::new(version_name, proc_macro2::Span::call_site());
    let total_entries: usize = groups.values().map(Vec::len).sum();

    // Generate insert blocks for each group
    let mut insert_blocks = Vec::new();
    for (cfg_conditions, entries) in &groups {
        let inserts: Vec<TokenStream2> = entries
            .iter()
            .map(|(sig, func)| {
                let func_tokens: TokenStream2 = func
                    .parse()
                    .unwrap_or_else(|e| panic!("Failed to parse function reference '{func}': {e}"));
                quote! {
                    map.insert(#sig, #func_tokens);
                }
            })
            .collect();

        if cfg_conditions.is_empty() {
            insert_blocks.push(quote! {
                #(#inserts)*
            });
        } else {
            // Generate #[cfg(condition)] for each condition (multiple attrs are ANDed)
            let cfg_attrs: Vec<TokenStream2> = cfg_conditions
                .iter()
                .map(|cond| {
                    let cond_tokens: TokenStream2 = cond
                        .parse()
                        .unwrap_or_else(|e| panic!("Failed to parse cfg condition '{cond}': {e}"));
                    quote! { #[cfg(#cond_tokens)] }
                })
                .collect();

            insert_blocks.push(quote! {
                #(#cfg_attrs)*
                {
                    #(#inserts)*
                }
            });
        }
    }

    quote! {
        pub(crate) static #version_ident: LazyLock<AHashMap<&'static str, IntrinsicMethod>> =
            LazyLock::new(|| {
                let mut map = AHashMap::with_capacity(#total_entries);
                #(#insert_blocks)*
                map
            });
    }
}
