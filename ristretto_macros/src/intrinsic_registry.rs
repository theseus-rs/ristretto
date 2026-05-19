use ahash::AHashMap;
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
use syn::{Expr, ExprCall, ExprLit, Item, LitStr, Meta, Result as SynResult};
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
pub(crate) fn process(input: TokenStream2) -> TokenStream2 {
    let input_str = input.to_string();
    let source_path = if input_str.is_empty() {
        // Default path relative to the workspace root
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
        PathBuf::from(&manifest_dir)
            .join("..")
            .join("ristretto_intrinsics")
            .join("src")
    } else {
        let lit: LitStr = syn::parse2(input).expect("Expected a string literal path");
        PathBuf::from(lit.value())
    };

    let intrinsic_methods = get_intrinsic_methods(&source_path)
        .unwrap_or_else(|error| panic!("Failed to get intrinsic methods: {error}"));

    let mut version_maps = Vec::new();
    let mut signature_slices = Vec::new();
    for (version_name, version) in JAVA_VERSIONS {
        let map = generate_intrinsic_method_map(version_name, version, &intrinsic_methods);
        version_maps.push(map);
        for os in OS_VARIANTS {
            let slice = generate_signature_slice(version_name, version, os, &intrinsic_methods);
            signature_slices.push(slice);
        }
    }

    let output = quote! {
        use crate::intrinsic_methods::IntrinsicMethod;
        use std::sync::LazyLock;
        use ahash::AHashMap;

        #(#version_maps)*

        #(#signature_slices)*
    };

    output
}

/// Candidate OS targets used by the cross-OS introspection signature slices.
const OS_VARIANTS: [&str; 3] = ["macos", "linux", "windows"];

/// Retrieves intrinsic methods from the source path.
fn get_intrinsic_methods(
    source_path: &PathBuf,
) -> Result<AHashMap<String, Vec<IntrinsicMethodData>>, Box<dyn std::error::Error>> {
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
    intrinsic_methods: &mut AHashMap<String, Vec<IntrinsicMethodData>>,
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
    intrinsic_methods: &mut AHashMap<String, Vec<IntrinsicMethodData>>,
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

            intrinsic_methods
                .entry(signature)
                .or_default()
                .push(IntrinsicMethodData {
                    function_name,
                    version_specification,
                    cfg_conditions,
                });
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
        let segment = version_path
            .path
            .segments
            .first()
            .expect("version path should contain a segment");
        let version = segment.ident.to_string();
        versions.push(parse_java_version(&version));
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
    intrinsic_methods: &AHashMap<String, Vec<IntrinsicMethodData>>,
) -> TokenStream2 {
    // Group entries by their cfg conditions (sorted for deterministic output)
    let mut groups: BTreeMap<Vec<String>, Vec<(String, String)>> = BTreeMap::new();

    for (signature, datas) in intrinsic_methods {
        for data in datas {
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

/// Generate a `pub(crate) static JAVA_<version>_<OS>_SIGNATURES: &[&str]` slice listing every
/// intrinsic method signature that would be registered for the given Java version on the given
/// OS, by evaluating each intrinsic's collected `cfg(...)` conditions as a boolean expression
/// against the (`target_os`, `target_family`) hypothesis for that OS.
///
/// These slices are unconditionally compiled (the values are plain string literals, not
/// function pointers) so they remain valid on every host and enable cross-OS introspection
/// from tests without changing actual dispatch behavior.
fn generate_signature_slice(
    version_name: &str,
    version: &Version,
    os: &str,
    intrinsic_methods: &AHashMap<String, Vec<IntrinsicMethodData>>,
) -> TokenStream2 {
    let mut signatures: Vec<String> = intrinsic_methods
        .iter()
        .filter(|(_, datas)| {
            datas.iter().any(|data| {
                data.version_specification.matches(version)
                    && data
                        .cfg_conditions
                        .iter()
                        .all(|condition| eval_cfg_condition(condition, os))
            })
        })
        .map(|(signature, _)| signature.clone())
        .collect();
    signatures.sort();

    let slice_ident = syn::Ident::new(
        &format!("{version_name}_{}_SIGNATURES", os.to_uppercase()),
        proc_macro2::Span::call_site(),
    );
    let entries: Vec<TokenStream2> = signatures.iter().map(|s| quote! { #s }).collect();

    quote! {
        #[cfg(test)]
        pub(crate) static #slice_ident: &[&str] = &[
            #(#entries,)*
        ];
    }
}

/// Evaluate the textual contents of a `#[cfg(...)]` condition (the `...` part) for a hypothetical
/// target OS. Returns `true` if the condition would match for that OS.
///
/// Predicate handling:
/// - `target_os = "X"` → `os == "X"`
/// - `target_family = "windows"` → `os == "windows"`
/// - `target_family = "unix"` → `os` ∈ {macos, linux, ...non-windows-non-wasm}
/// - `target_family = "wasm"` → false (we never test for wasm here)
/// - `unix` (predicate) → equivalent to `target_family = "unix"`
/// - `windows` (predicate) → equivalent to `target_family = "windows"`
/// - `target_endian = …`, `target_pointer_width = …`, `test` → assumed true (don't differentiate)
/// - Combinators `any(...)`, `all(...)`, `not(...)` evaluated recursively.
///
/// Unknown predicates are conservatively treated as true so that a method is included in the
/// signature slice unless its cfg explicitly excludes the target OS.
fn eval_cfg_condition(condition: &str, os: &str) -> bool {
    let Ok(expression) = syn::parse_str::<Expr>(condition) else {
        return true;
    };
    eval_cfg_expr(&expression, os)
}

fn eval_cfg_expr(expression: &Expr, os: &str) -> bool {
    match expression {
        Expr::Call(call) => {
            let combinator = match call.func.as_ref() {
                Expr::Path(path) => path
                    .path
                    .get_ident()
                    .map(ToString::to_string)
                    .unwrap_or_default(),
                _ => return true,
            };
            match combinator.as_str() {
                "any" => call.args.iter().any(|argument| eval_cfg_expr(argument, os)),
                "all" => call.args.iter().all(|argument| eval_cfg_expr(argument, os)),
                "not" => call
                    .args
                    .first()
                    .is_none_or(|argument| !eval_cfg_expr(argument, os)),
                _ => true,
            }
        }
        Expr::Assign(assign) => {
            let key = match assign.left.as_ref() {
                Expr::Path(path) => path
                    .path
                    .get_ident()
                    .map(ToString::to_string)
                    .unwrap_or_default(),
                _ => return true,
            };
            let value = match assign.right.as_ref() {
                Expr::Lit(ExprLit {
                    lit: syn::Lit::Str(string),
                    ..
                }) => string.value(),
                _ => return true,
            };
            eval_predicate(&key, Some(&value), os)
        }
        Expr::Path(path) => {
            let name = path
                .path
                .get_ident()
                .map(ToString::to_string)
                .unwrap_or_default();
            eval_predicate(&name, None, os)
        }
        _ => true,
    }
}

fn eval_predicate(name: &str, value: Option<&str>, os: &str) -> bool {
    match (name, value) {
        ("target_os", Some(target)) => os == target,
        ("target_family", Some("windows")) | ("windows", None) => os == "windows",
        ("target_family", Some("unix")) | ("unix", None) => os == "macos" || os == "linux",
        ("target_family", Some(_)) => false,
        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::Span;
    use quote::quote;
    use ristretto_classfile::{JAVA_8, JAVA_11, JAVA_17, JAVA_21, JAVA_25, VersionSpecification};
    use std::any::Any;
    use std::fs;
    use std::panic::{UnwindSafe, catch_unwind};
    use std::path::Path;
    use syn::parse_quote;
    use tempfile::TempDir;

    fn assert_panics<F>(operation: F, expected_message: &str)
    where
        F: FnOnce() + UnwindSafe,
    {
        let error = catch_unwind(operation).expect_err("operation should panic");
        let message = panic_message(error.as_ref());
        assert!(
            message.contains(expected_message),
            "panic message `{message}` did not contain `{expected_message}`"
        );
    }

    fn panic_message(error: &(dyn Any + Send)) -> String {
        if let Some(message) = error.downcast_ref::<String>() {
            return message.clone();
        }
        if let Some(message) = error.downcast_ref::<&str>() {
            return (*message).to_string();
        }
        "<non-string panic>".to_string()
    }

    #[test]
    fn panic_message_handles_str_and_non_string_payloads() {
        let str_error = catch_unwind(|| std::panic::panic_any("literal panic"))
            .expect_err("operation should panic");
        assert_eq!(panic_message(str_error.as_ref()), "literal panic");

        let non_string_error =
            catch_unwind(|| std::panic::panic_any(7_u8)).expect_err("operation should panic");
        assert_eq!(
            panic_message(non_string_error.as_ref()),
            "<non-string panic>"
        );
    }

    fn expression(source: &str) -> Expr {
        syn::parse_str(source).expect("expression should parse")
    }

    fn method(
        function_name: &str,
        version_specification: VersionSpecification,
        cfg_conditions: &[&str],
    ) -> IntrinsicMethodData {
        IntrinsicMethodData {
            function_name: function_name.to_string(),
            version_specification,
            cfg_conditions: cfg_conditions
                .iter()
                .map(std::string::ToString::to_string)
                .collect(),
        }
    }

    fn write_file(root: &Path, relative_path: &str, contents: &str) {
        let path = root.join(relative_path);
        fs::create_dir_all(path.parent().expect("test file should have a parent"))
            .expect("test directory should be created");
        fs::write(path, contents).expect("test file should be written");
    }

    fn fixture_source_tree() -> TempDir {
        let temp_dir = TempDir::new().expect("temp dir should be created");
        let root = temp_dir.path();
        write_file(
            root,
            "lib.rs",
            r#"
                #[cfg(target_family = "unix")]
                pub mod unix_mod;
                #[cfg(target_os = "windows")]
                pub mod windows_mod;
                pub mod plain_mod;

                fn helper_without_attribute() {}

                #[intrinsic_method(123)]
                pub fn invalid_attribute() {}
            "#,
        );
        write_file(
            root,
            "unix_mod.rs",
            r#"
                #[allow(dead_code)]
                #[cfg(target_family = "unix")]
                #[intrinsic_method("pkg/Unix.any()V", Any)]
                pub fn any() {}

                #[cfg(target_os = "macos")]
                #[intrinsic_method("pkg/Unix.macos()V", Equal(JAVA_8))]
                pub fn macos() {}

                #[intrinsic_method("pkg/Unix.not11()V", NotEqual(JAVA_11))]
                pub fn not11() {}

                #[intrinsic_method("pkg/Unix.lt17()V", LessThan(JAVA_17))]
                pub fn lt17() {}

                #[intrinsic_method("pkg/Unix.le17()V", LessThanOrEqual(JAVA_17))]
                pub fn le17() {}

                #[intrinsic_method("pkg/Unix.gt11()V", GreaterThan(JAVA_11))]
                pub fn gt11() {}

                #[intrinsic_method("pkg/Unix.ge21()V", GreaterThanOrEqual(JAVA_21))]
                pub fn ge21() {}

                #[intrinsic_method("pkg/Unix.between()V", Between(JAVA_11, JAVA_21))]
                pub fn between() {}

                #[intrinsic_method("pkg/Unix.in()V", In(&[JAVA_8, 42, JAVA_25]))]
                pub fn in_versions() {}

                pub fn helper() {}
            "#,
        );
        write_file(
            root,
            "windows_mod.rs",
            r#"
                #[intrinsic_method("pkg/Windows.any()V", Any)]
                pub fn any() {}
            "#,
        );
        write_file(
            root,
            "plain_mod.rs",
            r#"
                #[intrinsic_method("pkg/Plain.any()V", Any)]
                pub fn plain() {}
            "#,
        );
        write_file(
            root,
            "type.rs",
            r#"
                #[intrinsic_method("pkg/Keyword.any()V", Any)]
                pub fn keyword() {}
            "#,
        );
        write_file(
            root,
            "nested/mod.rs",
            r#"
                #[cfg(target_os = "linux")]
                pub mod child;
            "#,
        );
        write_file(
            root,
            "nested/child.rs",
            r#"
                #[cfg(target_pointer_width = "64")]
                #[intrinsic_method("pkg/Nested.child()V", Any)]
                pub fn child() {}
            "#,
        );
        write_file(root, "broken/mod.rs", "pub mod ;");
        write_file(root, "broken_file.rs", "fn broken(");
        write_file(root, "not_rust.txt", "ignored");
        temp_dir
    }

    #[test]
    fn parse_java_version_accepts_supported_versions() {
        assert_eq!(parse_java_version("JAVA_8").major(), JAVA_8.major());
        assert_eq!(parse_java_version("JAVA_11").major(), JAVA_11.major());
        assert_eq!(parse_java_version("JAVA_17").major(), JAVA_17.major());
        assert_eq!(parse_java_version("JAVA_21").major(), JAVA_21.major());
        assert_eq!(parse_java_version("JAVA_25").major(), JAVA_25.major());
    }

    #[test]
    fn parse_java_version_rejects_unknown_version() {
        assert_panics(
            || {
                let _ = parse_java_version("JAVA_99");
            },
            "Unsupported intrinsic method Java version",
        );
    }

    #[test]
    fn intrinsic_method_args_parse_signature_and_version_specification() {
        let arguments: IntrinsicMethodArgs =
            syn::parse_str(r#""pkg/Example.method()V", Any"#).expect("arguments should parse");

        assert_eq!(arguments.signature.value(), "pkg/Example.method()V");
        assert!(
            version_specification(&arguments.version_specification).matches(&JAVA_8),
            "Any should match Java 8"
        );
        assert!(syn::parse_str::<IntrinsicMethodArgs>(r#""pkg/Example.method()V""#).is_err());
        assert!(syn::parse_str::<IntrinsicMethodArgs>(r#""pkg/Example.method()V","#).is_err());
    }

    #[test]
    fn extract_cfg_content_returns_non_empty_list_tokens_only() {
        let cfg_attr: syn::Attribute = parse_quote!(#[cfg(target_os = "macos")]);
        assert_eq!(
            extract_cfg_content(&cfg_attr).as_deref(),
            Some(r#"target_os = "macos""#)
        );

        let empty_cfg_attr: syn::Attribute = parse_quote!(#[cfg()]);
        assert_eq!(extract_cfg_content(&empty_cfg_attr), None);

        let path_attr: syn::Attribute = parse_quote!(#[test]);
        assert_eq!(extract_cfg_content(&path_attr), None);
    }

    #[test]
    fn get_intrinsic_methods_collects_versions_modules_cfgs_and_keywords() {
        let source_tree = fixture_source_tree();
        let methods =
            get_intrinsic_methods(&source_tree.path().to_path_buf()).expect("scan should succeed");

        let plain = methods
            .get("pkg/Plain.any()V")
            .expect("plain intrinsic should be found");
        assert_eq!(
            plain[0].function_name,
            "ristretto_intrinsics::plain_mod::plain"
        );
        assert!(plain[0].cfg_conditions.is_empty());

        let keyword = methods
            .get("pkg/Keyword.any()V")
            .expect("keyword module intrinsic should be found");
        assert_eq!(
            keyword[0].function_name,
            "ristretto_intrinsics::r#type::keyword"
        );

        let unix = methods
            .get("pkg/Unix.any()V")
            .expect("unix intrinsic should be found");
        assert_eq!(unix[0].cfg_conditions, [r#"target_family = "unix""#]);
        assert!(unix[0].version_specification.matches(&JAVA_8));
        assert!(unix[0].version_specification.matches(&JAVA_25));

        let macos = methods
            .get("pkg/Unix.macos()V")
            .expect("macos intrinsic should be found");
        assert_eq!(
            macos[0].cfg_conditions,
            [r#"target_family = "unix""#, r#"target_os = "macos""#]
        );

        let windows = methods
            .get("pkg/Windows.any()V")
            .expect("windows intrinsic should be found");
        assert_eq!(windows[0].cfg_conditions, [r#"target_os = "windows""#]);

        let nested = methods
            .get("pkg/Nested.child()V")
            .expect("nested intrinsic should be found");
        assert_eq!(
            nested[0].cfg_conditions,
            [r#"target_os = "linux""#, r#"target_pointer_width = "64""#]
        );

        let in_versions = methods
            .get("pkg/Unix.in()V")
            .expect("In version intrinsic should be found");
        assert!(in_versions[0].version_specification.matches(&JAVA_8));
        assert!(!in_versions[0].version_specification.matches(&JAVA_11));
        assert!(in_versions[0].version_specification.matches(&JAVA_25));

        assert!(!methods.contains_key("invalid_attribute"));
    }

    #[cfg(unix)]
    #[test]
    fn get_intrinsic_methods_returns_error_when_rs_file_cannot_be_read() {
        use std::os::unix::fs::PermissionsExt;

        let source_tree = TempDir::new().expect("temp dir should be created");
        let unreadable_file = source_tree.path().join("unreadable.rs");
        fs::write(&unreadable_file, "fn unreadable() {}").expect("test file should be written");
        fs::set_permissions(&unreadable_file, fs::Permissions::from_mode(0o000))
            .expect("test file permissions should be changed");

        let result = get_intrinsic_methods(&source_tree.path().to_path_buf());

        fs::set_permissions(&unreadable_file, fs::Permissions::from_mode(0o644))
            .expect("test file permissions should be restored");
        assert!(result.is_err());
    }

    #[test]
    fn get_intrinsic_methods_returns_error_when_rs_file_is_not_utf8() {
        let source_tree = TempDir::new().expect("temp dir should be created");
        let invalid_utf8_file = source_tree.path().join("invalid_utf8.rs");
        fs::write(invalid_utf8_file, [0xff]).expect("test file should be written");

        let result = get_intrinsic_methods(&source_tree.path().to_path_buf());

        assert!(result.is_err());
    }

    #[cfg(unix)]
    #[test]
    fn get_intrinsic_methods_returns_error_when_module_file_cannot_be_opened() {
        use std::os::unix::fs::PermissionsExt;

        let source_tree = TempDir::new().expect("temp dir should be created");
        let unreadable_file = source_tree.path().join("lib.rs");
        fs::write(&unreadable_file, "pub mod unreadable;").expect("test file should be written");
        fs::set_permissions(&unreadable_file, fs::Permissions::from_mode(0o000))
            .expect("test file permissions should be changed");

        let result = get_intrinsic_methods(&source_tree.path().to_path_buf());

        fs::set_permissions(&unreadable_file, fs::Permissions::from_mode(0o644))
            .expect("test file permissions should be restored");
        assert!(result.is_err());
    }

    #[test]
    fn get_intrinsic_methods_returns_error_when_module_file_cannot_be_read() {
        let source_tree = TempDir::new().expect("temp dir should be created");
        fs::create_dir(source_tree.path().join("lib.rs"))
            .expect("test directory should be created");

        let result = get_intrinsic_methods(&source_tree.path().to_path_buf());

        assert!(result.is_err());
    }

    #[cfg(unix)]
    #[test]
    fn process_panics_when_source_scan_fails() {
        use std::os::unix::fs::PermissionsExt;

        let source_tree = TempDir::new().expect("temp dir should be created");
        let unreadable_file = source_tree.path().join("unreadable.rs");
        fs::write(&unreadable_file, "fn unreadable() {}").expect("test file should be written");
        fs::set_permissions(&unreadable_file, fs::Permissions::from_mode(0o000))
            .expect("test file permissions should be changed");
        let path = source_tree
            .path()
            .to_str()
            .expect("temp path should be valid UTF-8");
        let path = LitStr::new(path, Span::call_site());

        assert_panics(
            || {
                let _ = process(quote! { #path });
            },
            "Failed to get intrinsic methods",
        );

        fs::set_permissions(&unreadable_file, fs::Permissions::from_mode(0o644))
            .expect("test file permissions should be restored");
    }

    #[test]
    fn process_generates_registry_for_explicit_source_path() {
        let source_tree = fixture_source_tree();
        let path = source_tree
            .path()
            .to_str()
            .expect("temp path should be valid UTF-8");
        let path = LitStr::new(path, Span::call_site());
        let output = process(quote! { #path }).to_string();

        assert!(output.contains("static JAVA_8"));
        assert!(output.contains("JAVA_8_MACOS_SIGNATURES"));
        assert!(output.contains("\"pkg/Plain.any()V\""));
        assert!(output.contains("\"pkg/Unix.any()V\""));
        assert!(output.contains("# [cfg (target_family = \"unix\")]"));
    }

    #[test]
    fn process_generates_registry_for_default_source_path() {
        let output = process(TokenStream2::new()).to_string();

        assert!(output.contains("static JAVA_8"));
        assert!(output.contains("JAVA_25_WINDOWS_SIGNATURES"));
    }

    #[test]
    fn process_rejects_non_string_literal_input() {
        assert_panics(
            || {
                let _ = process(quote! { not_a_string_literal });
            },
            "Expected a string literal path",
        );
    }

    #[test]
    fn version_specification_parses_all_supported_forms() {
        assert!(version_specification(&expression("Any")).matches(&JAVA_8));
        assert!(matches!(
            version_specification(&expression("Equal(JAVA_8)")),
            VersionSpecification::Equal(version) if version.major() == JAVA_8.major()
        ));
        assert!(matches!(
            version_specification(&expression("NotEqual(JAVA_11)")),
            VersionSpecification::NotEqual(version) if version.major() == JAVA_11.major()
        ));
        assert!(matches!(
            version_specification(&expression("LessThan(JAVA_17)")),
            VersionSpecification::LessThan(version) if version.major() == JAVA_17.major()
        ));
        assert!(matches!(
            version_specification(&expression("LessThanOrEqual(JAVA_17)")),
            VersionSpecification::LessThanOrEqual(version) if version.major() == JAVA_17.major()
        ));
        assert!(matches!(
            version_specification(&expression("GreaterThan(JAVA_11)")),
            VersionSpecification::GreaterThan(version) if version.major() == JAVA_11.major()
        ));
        assert!(matches!(
            version_specification(&expression("GreaterThanOrEqual(JAVA_21)")),
            VersionSpecification::GreaterThanOrEqual(version) if version.major() == JAVA_21.major()
        ));
        assert!(matches!(
            version_specification(&expression("Between(JAVA_11, JAVA_21)")),
            VersionSpecification::Between(start, end)
                if start.major() == JAVA_11.major() && end.major() == JAVA_21.major()
        ));
        assert!(matches!(
            version_specification(&expression("In(&[JAVA_8, 42, JAVA_25])")),
            VersionSpecification::In(versions)
                if versions.len() == 2
                    && versions[0].major() == JAVA_8.major()
                    && versions[1].major() == JAVA_25.major()
        ));
    }

    #[test]
    fn version_specification_rejects_unsupported_forms() {
        assert_panics(
            || {
                let _ = version_specification(&expression("Unsupported"));
            },
            "Unsupported version specification in intrinsic method attribute",
        );
        assert_panics(
            || {
                let _ = version_specification(&expression("1"));
            },
            "[call] Unsupported version specification",
        );
        assert_panics(
            || {
                let _ = version_specification(&expression("(factory())(JAVA_8)"));
            },
            "[call.path] Unsupported version specification",
        );
        assert_panics(
            || {
                let _ = version_specification(&expression("spec::Equal(JAVA_8)"));
            },
            "[call.path.ident] Unsupported version specification",
        );
        assert_panics(
            || {
                let _ = version_specification(&expression("Unknown(JAVA_8)"));
            },
            "Unsupported version specification in intrinsic method attribute \"Unknown\"",
        );
        assert_panics(
            || {
                let _ = version_specification(&expression("In(JAVA_8)"));
            },
            "(call.args[0]]) Unsupported expression in call",
        );
        assert_panics(
            || {
                let _ = version_specification(&expression("In(&JAVA_8)"));
            },
            "(call.args[0].expr) Unsupported expression in call",
        );
        assert_panics(
            || {
                let _ = version_specification(&expression("Equal()"));
            },
            "Unsupported Java version in intrinsic method attribute",
        );
        assert_panics(
            || {
                let _ = version_specification(&expression("Equal(42)"));
            },
            "Unsupported Java version in intrinsic method attribute",
        );
    }

    #[test]
    fn generate_intrinsic_method_map_groups_filters_and_sorts_entries() {
        let mut methods = AHashMap::default();
        methods.insert(
            "pkg/B.b()V".to_string(),
            vec![method(
                "ristretto_intrinsics::plain_mod::b",
                VersionSpecification::Any,
                &[],
            )],
        );
        methods.insert(
            "pkg/A.a()V".to_string(),
            vec![method(
                "ristretto_intrinsics::plain_mod::a",
                VersionSpecification::Any,
                &[r#"target_os = "macos""#],
            )],
        );
        methods.insert(
            "pkg/Skipped.skip()V".to_string(),
            vec![method(
                "ristretto_intrinsics::plain_mod::skip",
                VersionSpecification::Equal(JAVA_11),
                &[],
            )],
        );

        let output = generate_intrinsic_method_map("JAVA_8", &JAVA_8, &methods).to_string();

        assert!(output.contains("with_capacity (2usize)"));
        assert!(output.contains("\"pkg/A.a()V\""));
        assert!(output.contains("\"pkg/B.b()V\""));
        assert!(output.contains("# [cfg (target_os = \"macos\")]"));
        assert!(!output.contains("pkg/Skipped.skip()V"));
    }

    #[test]
    fn generate_intrinsic_method_map_rejects_invalid_generated_tokens() {
        let mut bad_function = AHashMap::default();
        bad_function.insert(
            "pkg/Bad.function()V".to_string(),
            vec![method(
                "ristretto_intrinsics::plain_mod::bad(",
                VersionSpecification::Any,
                &[],
            )],
        );
        assert_panics(
            || {
                let _ = generate_intrinsic_method_map("JAVA_8", &JAVA_8, &bad_function);
            },
            "Failed to parse function reference",
        );

        let mut bad_cfg = AHashMap::default();
        bad_cfg.insert(
            "pkg/Bad.cfg()V".to_string(),
            vec![method(
                "ristretto_intrinsics::plain_mod::valid",
                VersionSpecification::Any,
                &[")"],
            )],
        );
        assert_panics(
            || {
                let _ = generate_intrinsic_method_map("JAVA_8", &JAVA_8, &bad_cfg);
            },
            "Failed to parse cfg condition",
        );
    }

    #[test]
    fn generate_signature_slice_filters_by_version_and_cfg_conditions() {
        let mut methods = AHashMap::default();
        methods.insert(
            "pkg/Mac.only()V".to_string(),
            vec![method(
                "ristretto_intrinsics::plain_mod::mac",
                VersionSpecification::Any,
                &[r#"target_os = "macos""#],
            )],
        );
        methods.insert(
            "pkg/Windows.only()V".to_string(),
            vec![method(
                "ristretto_intrinsics::plain_mod::windows",
                VersionSpecification::Any,
                &[r#"target_family = "windows""#],
            )],
        );
        methods.insert(
            "pkg/Unknown.included()V".to_string(),
            vec![method(
                "ristretto_intrinsics::plain_mod::unknown",
                VersionSpecification::Any,
                &["custom_predicate"],
            )],
        );
        methods.insert(
            "pkg/Skipped.version()V".to_string(),
            vec![method(
                "ristretto_intrinsics::plain_mod::skip",
                VersionSpecification::Equal(JAVA_11),
                &[],
            )],
        );

        let macos_output =
            generate_signature_slice("JAVA_8", &JAVA_8, "macos", &methods).to_string();
        assert!(macos_output.contains("JAVA_8_MACOS_SIGNATURES"));
        assert!(macos_output.contains("pkg/Mac.only()V"));
        assert!(macos_output.contains("pkg/Unknown.included()V"));
        assert!(!macos_output.contains("pkg/Windows.only()V"));
        assert!(!macos_output.contains("pkg/Skipped.version()V"));

        let windows_output =
            generate_signature_slice("JAVA_8", &JAVA_8, "windows", &methods).to_string();
        assert!(windows_output.contains("pkg/Windows.only()V"));
        assert!(!windows_output.contains("pkg/Mac.only()V"));
    }

    #[test]
    fn eval_cfg_condition_handles_predicates_and_combinators() {
        assert!(eval_cfg_condition(r#"target_os = "macos""#, "macos"));
        assert!(!eval_cfg_condition(r#"target_os = "linux""#, "macos"));
        assert!(eval_cfg_condition(
            r#"target_family = "windows""#,
            "windows"
        ));
        assert!(!eval_cfg_condition(r#"target_family = "windows""#, "linux"));
        assert!(eval_cfg_condition(r#"target_family = "unix""#, "linux"));
        assert!(eval_cfg_condition("unix", "macos"));
        assert!(!eval_cfg_condition("unix", "windows"));
        assert!(eval_cfg_condition("windows", "windows"));
        assert!(!eval_cfg_condition("windows", "macos"));
        assert!(!eval_cfg_condition(r#"target_family = "wasm""#, "macos"));
        assert!(eval_cfg_condition(
            r#"any(target_os = "macos", target_os = "windows")"#,
            "macos"
        ));
        assert!(eval_cfg_condition(
            r#"all(unix, not(target_os = "windows"))"#,
            "linux"
        ));
        assert!(!eval_cfg_condition(r#"not(target_os = "macos")"#, "macos"));
        assert!(eval_cfg_condition("not()", "macos"));
        assert!(eval_cfg_condition("unknown_predicate", "macos"));
        assert!(eval_cfg_condition("=", "macos"));
    }

    #[test]
    fn eval_cfg_expr_treats_unknown_or_unhandled_shapes_as_true() {
        assert!(eval_cfg_expr(&expression("unknown(unix)"), "macos"));
        assert!(eval_cfg_expr(&expression("module::unknown(unix)"), "macos"));
        assert!(eval_cfg_expr(&expression("(predicate)(unix)"), "macos"));
        assert!(eval_cfg_expr(
            &expression(r#"module::key = "value""#),
            "macos"
        ));
        assert!(eval_cfg_expr(&expression(r"target_os = 1"), "macos"));
        assert!(eval_cfg_expr(&expression("module::predicate"), "macos"));
        assert!(eval_cfg_expr(&expression("1"), "macos"));

        let left_not_path: Expr = parse_quote!((target_os) = "macos");
        assert!(eval_cfg_expr(&left_not_path, "macos"));
    }
}
