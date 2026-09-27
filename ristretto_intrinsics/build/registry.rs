//! Build-time discovery follows Rust modules and emits a registry compiled in this crate.

use proc_macro2::TokenStream;
use quote::quote;
use ristretto_classfile::{
    FieldType, JAVA_8, JAVA_11, JAVA_17, JAVA_21, JAVA_25, JavaStr, Version, VersionSpecification,
};
use std::collections::BTreeMap;
use std::path::Path;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Attribute, Expr, Item, LitStr, Meta, Token};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const VERSIONS: [Version; 5] = [JAVA_8, JAVA_11, JAVA_17, JAVA_21, JAVA_25];

struct Arguments {
    signature: LitStr,
    version: Expr,
}

impl Parse for Arguments {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let signature = input.parse()?;
        input.parse::<Token![,]>()?;
        let version = input.parse()?;
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }
        Ok(Self { signature, version })
    }
}

struct Entry {
    signature: String,
    function: syn::Path,
    source: String,
    versions: u8,
    is_async: bool,
    conditions: Vec<Meta>,
}

/// Generate registry code from the crate's actual module graph.
pub fn generate(root: &Path) -> Result<String> {
    let mut entries = Vec::new();
    let lib = root.join("src/lib.rs");
    scan_file(root, &lib, &root.join("src"), &[], &[], &mut entries)?;
    if entries.is_empty() {
        return Err("No intrinsic methods found".into());
    }
    entries.sort_by_key(|entry| {
        let function = &entry.function;
        (
            entry.signature.clone(),
            entry.source.clone(),
            quote!(#function).to_string(),
        )
    });
    emit(&entries).map(|tokens| tokens.to_string())
}

fn scan_file(
    root: &Path,
    file: &Path,
    directory: &Path,
    module: &[syn::Ident],
    inherited: &[Meta],
    entries: &mut Vec<Entry>,
) -> Result<()> {
    println!("cargo::rerun-if-changed={}", file.display());
    let source = std::fs::read_to_string(file).map_err(|e| format!("{}: {e}", file.display()))?;
    let parsed = syn::parse_file(&source).map_err(|e| format!("{}: {e}", file.display()))?;
    let mut conditions = inherited.to_vec();
    conditions.extend(cfgs(&parsed.attrs)?);
    scan_items(
        root,
        (file, file.parent().ok_or("module has no parent")?),
        directory,
        module,
        &conditions,
        &parsed.items,
        entries,
    )
}

fn cfgs(attributes: &[Attribute]) -> Result<Vec<Meta>> {
    let mut conditions = Vec::new();
    for attribute in attributes {
        conditions.extend(cfg_conditions(&attribute.meta)?);
    }
    Ok(conditions)
}

fn cfg_conditions(attribute: &Meta) -> Result<Vec<Meta>> {
    let mut conditions = Vec::new();
    if let Meta::List(list) = attribute {
        if list.path.is_ident("cfg") {
            conditions.push(syn::parse2(list.tokens.clone())?);
        } else if list.path.is_ident("cfg_attr") {
            let arguments =
                list.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
            let mut arguments = arguments.into_iter();
            let predicate = arguments.next().ok_or("empty cfg_attr")?;
            for attribute in arguments {
                if attribute.path().is_ident("path")
                    || attribute
                        .path()
                        .segments
                        .last()
                        .is_some_and(|segment| segment.ident == "intrinsic_method")
                {
                    return Err("cfg_attr changing module paths or intrinsic attributes is unsupported; use explicit cfg-gated declarations".into());
                }
                for condition in cfg_conditions(&attribute)? {
                    conditions.push(syn::parse_quote!(any(not(#predicate), #condition)));
                }
            }
        }
    }
    Ok(conditions)
}

fn scan_items(
    root: &Path,
    (file, path_directory): (&Path, &Path),
    directory: &Path,
    module: &[syn::Ident],
    inherited: &[Meta],
    items: &[Item],
    entries: &mut Vec<Entry>,
) -> Result<()> {
    for item in items {
        match item {
            Item::Mod(item) => {
                // Test modules do not participate in production registration.
                let mut conditions = inherited.to_vec();
                conditions.extend(cfgs(&item.attrs)?);
                if conditions
                    .iter()
                    .any(|condition| matches!(condition, Meta::Path(path) if path.is_ident("test")))
                {
                    continue;
                }
                let mut path = module.to_vec();
                path.push(item.ident.clone());
                let child_directory =
                    directory.join(item.ident.to_string().trim_start_matches("r#"));
                let explicit = item.attrs.iter().find(|a| a.path().is_ident("path"));
                let explicit_path = if let Some(attribute) = explicit {
                    let Meta::NameValue(value) = &attribute.meta else {
                        return Err("invalid path attribute".into());
                    };
                    let Expr::Lit(literal) = &value.value else {
                        return Err("path must be a string".into());
                    };
                    let syn::Lit::Str(value) = &literal.lit else {
                        return Err("path must be a string".into());
                    };
                    Some(path_directory.join(value.value()))
                } else {
                    None
                };
                if let Some((_, children)) = &item.content {
                    let child_directory = explicit_path.unwrap_or(child_directory);
                    scan_items(
                        root,
                        (file, &child_directory),
                        &child_directory,
                        &path,
                        &conditions,
                        children,
                        entries,
                    )?;
                } else {
                    let child = if let Some(path) = explicit_path {
                        path
                    } else {
                        let flat = child_directory.with_extension("rs");
                        let nested = child_directory.join("mod.rs");
                        if flat.is_file() && nested.is_file() {
                            return Err(
                                format!("ambiguous module {}", child_directory.display()).into()
                            );
                        }
                        if flat.is_file() { flat } else { nested }
                    };
                    let child_base = if explicit.is_some()
                        || child.file_name().is_some_and(|name| name == "mod.rs")
                    {
                        child.parent().ok_or("module has no parent")?.to_path_buf()
                    } else {
                        child.with_extension("")
                    };
                    scan_file(root, &child, &child_base, &path, &conditions, entries)?;
                }
            }
            Item::Fn(function) => scan_function(root, file, module, inherited, function, entries)?,
            _ => {}
        }
    }
    Ok(())
}

fn scan_function(
    root: &Path,
    file: &Path,
    module: &[syn::Ident],
    inherited: &[Meta],
    function: &syn::ItemFn,
    entries: &mut Vec<Entry>,
) -> Result<()> {
    // Validate conditional attributes even when no direct intrinsic attribute is present.
    let mut conditions = inherited.to_vec();
    conditions.extend(cfgs(&function.attrs)?);
    for attribute in &function.attrs {
        if attribute
            .path()
            .segments
            .last()
            .is_none_or(|s| s.ident != "intrinsic_method")
        {
            continue;
        }
        let args: Arguments = attribute
            .parse_args()
            .map_err(|e| format!("{}::{}: {e}", file.display(), function.sig.ident))?;
        let signature = args.signature.value();
        validate_signature(&signature)
            .map_err(|e| format!("{}::{}: {e}", file.display(), function.sig.ident))?;
        let versions = version_mask(&args.version)?;
        let name = &function.sig.ident;
        let path = quote!(crate::#(#module::)*#name);
        entries.push(Entry {
            signature,
            function: syn::parse2(path)?,
            source: file
                .strip_prefix(root)?
                .to_string_lossy()
                .replace('\\', "/"),
            versions,
            is_async: function.sig.asyncness.is_some(),
            conditions: conditions.clone(),
        });
    }
    Ok(())
}

fn validate_signature(signature: &str) -> Result<()> {
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
        return Err(format!("invalid intrinsic signature {signature}").into());
    }
    FieldType::parse_method_descriptor(&JavaStr::cow_from_str(&format!("({descriptor}")))?;
    Ok(())
}

fn java_version(expression: &Expr) -> Result<Version> {
    let Expr::Path(path) = expression else {
        return Err("expected a Java version constant".into());
    };
    match path
        .path
        .segments
        .last()
        .map(|s| s.ident.to_string())
        .as_deref()
    {
        Some("JAVA_8") => Ok(JAVA_8),
        Some("JAVA_11") => Ok(JAVA_11),
        Some("JAVA_17") => Ok(JAVA_17),
        Some("JAVA_21") => Ok(JAVA_21),
        Some("JAVA_25") => Ok(JAVA_25),
        _ => Err(format!("unsupported Java version: {}", quote!(#expression)).into()),
    }
}

fn version_mask(expression: &Expr) -> Result<u8> {
    if let Expr::Call(call) = expression
        && let Expr::Path(path) = call.func.as_ref()
        && path.path.segments.last().is_some_and(|s| s.ident == "In")
    {
        if call.args.len() != 1 {
            return Err("In expects one version array".into());
        }
        let Some(Expr::Reference(reference)) = call.args.first() else {
            return Err("In expects a borrowed version array".into());
        };
        let Expr::Array(array) = reference.expr.as_ref() else {
            return Err("In expects a version array".into());
        };
        let selected = array
            .elems
            .iter()
            .map(java_version)
            .collect::<Result<Vec<_>>>()?;
        return Ok(VERSIONS.iter().enumerate().fold(0, |bits, (i, version)| {
            bits | if selected.contains(version) {
                1 << i
            } else {
                0
            }
        }));
    }
    let specification = version_specification(expression)?;
    Ok(VERSIONS.iter().enumerate().fold(0, |bits, (i, version)| {
        bits | if specification.matches(version) {
            1 << i
        } else {
            0
        }
    }))
}

fn version_specification(expression: &Expr) -> Result<VersionSpecification> {
    if let Expr::Path(path) = expression
        && path.path.segments.last().is_some_and(|s| s.ident == "Any")
    {
        return Ok(VersionSpecification::Any);
    }
    let Expr::Call(call) = expression else {
        return Err("expected a version specification".into());
    };
    let Expr::Path(path) = call.func.as_ref() else {
        return Err("expected a version variant".into());
    };
    let name = path
        .path
        .segments
        .last()
        .ok_or("missing variant")?
        .ident
        .to_string();
    let expected = if name == "Between" { 2 } else { 1 };
    if call.args.len() != expected {
        return Err(format!("{name} expects {expected} arguments").into());
    }
    let version = java_version(call.args.first().ok_or("missing version")?)?;
    Ok(match name.as_str() {
        "Equal" => VersionSpecification::Equal(version),
        "NotEqual" => VersionSpecification::NotEqual(version),
        "LessThan" => VersionSpecification::LessThan(version),
        "LessThanOrEqual" => VersionSpecification::LessThanOrEqual(version),
        "GreaterThan" => VersionSpecification::GreaterThan(version),
        "GreaterThanOrEqual" => VersionSpecification::GreaterThanOrEqual(version),
        "Between" => {
            let end = java_version(call.args.last().ok_or("missing end version")?)?;
            if version.major() > end.major() {
                return Err("reversed version range".into());
            }
            VersionSpecification::Between(version, end)
        }
        _ => return Err(format!("unsupported version specification {name}").into()),
    })
}

/// Preserve the existing OS-only registration coverage approximation. Other predicates
/// are assumed true; actual dispatch is always selected by the Rust compiler.
fn matches_os(condition: &Meta, os: &str) -> Result<bool> {
    Ok(match condition {
        Meta::Path(path) if path.is_ident("unix") => matches!(os, "macos" | "linux"),
        Meta::Path(path) if path.is_ident("windows") => os == "windows",
        Meta::NameValue(value) => {
            let Expr::Lit(literal) = &value.value else {
                return Ok(true);
            };
            let syn::Lit::Str(value_text) = &literal.lit else {
                return Ok(true);
            };
            let value_text = value_text.value();
            if value.path.is_ident("target_os") {
                os == value_text
            } else if value.path.is_ident("target_family") {
                match value_text.as_str() {
                    "unix" => matches!(os, "macos" | "linux"),
                    "windows" => os == "windows",
                    _ => false,
                }
            } else {
                true
            }
        }
        Meta::List(list) => {
            let children = list.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
            let children = children
                .iter()
                .map(|child| matches_os(child, os))
                .collect::<Result<Vec<_>>>()?;
            if list.path.is_ident("all") {
                children.iter().all(|child| *child)
            } else if list.path.is_ident("any") {
                children.iter().any(|child| *child)
            } else if list.path.is_ident("not") {
                children.first().is_none_or(|child| !child)
            } else {
                true
            }
        }
        Meta::Path(_) => true,
    })
}

fn coverage_slices(entries: &[Entry]) -> Result<TokenStream> {
    let mut arms = Vec::new();
    for (index, _) in VERSIONS.iter().enumerate() {
        let bit = 1u8 << index;
        for os in ["macos", "linux", "windows"] {
            let mut signatures = Vec::new();
            for entry in entries {
                if entry.versions & bit != 0
                    && entry
                        .conditions
                        .iter()
                        .map(|condition| matches_os(condition, os))
                        .collect::<Result<Vec<_>>>()?
                        .into_iter()
                        .all(|enabled| enabled)
                {
                    signatures.push(&entry.signature);
                }
            }
            signatures.sort_unstable();
            signatures.dedup();
            arms.push(quote!((#bit, #os) => &[#(#signatures),*],));
        }
    }
    Ok(quote! {
        /// Signature lists used by cross-OS registration coverage tests.
        /// Non-OS configuration predicates are assumed true.
        #[doc(hidden)]
        #[must_use]
        pub fn signatures_for_os(version: &ristretto_classfile::Version, os: &str) -> &'static [&'static str] {
            match (ristretto_types::intrinsic_version_bit(version), os) {
                #(#arms)*
                _ => &[],
            }
        }
    })
}

fn emit(entries: &[Entry]) -> Result<TokenStream> {
    let mut registrations = Vec::new();
    let mut checks = Vec::new();
    let mut seen: BTreeMap<&str, Vec<&Entry>> = BTreeMap::new();
    for entry in entries {
        let Entry {
            signature,
            function,
            source,
            versions,
            is_async,
            conditions,
        } = entry;
        let function_name = quote!(#function).to_string();
        let method = if *is_async {
            quote!(IntrinsicMethod::Async(|thread, parameters| Box::pin(#function::<T>(thread, parameters))))
        } else {
            quote!(IntrinsicMethod::Sync(#function::<T>))
        };
        registrations.push(quote! {
            #[cfg(all(#(#conditions),*))]
            registry.register(&IntrinsicMetadata { signature: #signature, implementation: #function_name, source: #source, versions: #versions }, #method)?;
        });
        let previous = seen.entry(signature).or_default();
        for other in previous.iter() {
            if other.versions & versions == 0 {
                continue;
            }
            let other_conditions = &other.conditions;
            let other_function = &other.function;
            let message = format!(
                "Overlapping intrinsic {signature}: {} ({}) and {function_name} ({source})",
                quote!(#other_function),
                other.source
            );
            checks.push(quote!(#[cfg(all(#(#conditions,)* #(#other_conditions),*))] compile_error!(#message);));
        }
        previous.push(entry);
    }
    let coverage = coverage_slices(entries)?;
    Ok(quote! {
        use ristretto_types::{IntrinsicMetadata, IntrinsicMethod, IntrinsicRegistry, Result, Thread};
        #(#checks)*
        #coverage
        /// Register compiler-enabled implementations for the selected Java release.
        ///
        /// # Errors
        /// Returns an error if two active implementations have the same signature.
        pub fn register<T: Thread + 'static>(registry: &mut IntrinsicRegistry<T>) -> Result<()> {
            #(#registrations)*
            Ok(())
        }
    })
}
