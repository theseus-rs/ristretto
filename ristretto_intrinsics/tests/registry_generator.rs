//! Exercise generation independently of the production module tree.
//! The generator runs on the build host; its fixtures require native temporary files and `rustc`.
#![cfg(not(target_family = "wasm"))]
#![expect(
    clippy::panic_in_result_fn,
    reason = "fixture assertions diagnose generation failures"
)]
#[path = "../build/registry.rs"]
mod registry;

use std::path::Path;
use tempfile::TempDir;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn fixture(source: &str) -> Result<TempDir> {
    let root = TempDir::new()?;
    std::fs::create_dir(root.path().join("src"))?;
    std::fs::write(root.path().join("src/lib.rs"), source)?;
    Ok(root)
}

#[test]
fn follows_modules_paths_inline_modules_and_cfg_attr() -> Result<()> {
    let root = fixture(
        r#"
        #[path = "alternate.rs"] pub mod logical;
        #[cfg_attr(feature = "audio", cfg(target_os = "linux"))]
        pub mod inline {
            #[intrinsic_method("pkg/Example.inline()V", Any)]
            pub fn run() {}
        }
    "#,
    )?;
    std::fs::write(
        root.path().join("src/alternate.rs"),
        r#"
        #[ristretto_macros::intrinsic_method("pkg/Example.path()V", Between(JAVA_11, JAVA_21))]
        pub async fn run() {}
    "#,
    )?;
    // A filesystem walk would attempt to parse this unreferenced file.
    std::fs::write(root.path().join("src/unreferenced.rs"), "not valid Rust {")?;
    let output = registry::generate(root.path())?;
    assert!(output.contains("crate :: logical :: run"));
    assert!(output.contains("crate :: inline :: run"));
    assert!(output.contains("versions : 14u8"));
    assert!(output.contains("Box :: pin"));
    assert!(output.contains("any (not (feature = \"audio\") , target_os = \"linux\")"));
    Ok(())
}

#[test]
fn rejects_missing_empty_malformed_and_invalid_sources() -> Result<()> {
    for source in [
        "pub mod missing;",
        "pub fn ordinary() {}",
        "pub fn broken(",
        r#"#[intrinsic_method("pkg/Example.run(Q)V", Any)] pub fn run() {}"#,
        r#"#[intrinsic_method("pkg/Example.run()V", Between(JAVA_21, JAVA_8))] pub fn run() {}"#,
        r#"#[intrinsic_method("pkg/Example.run()V", In(&[JAVA_8, 42]))] pub fn run() {}"#,
        r#"#[intrinsic_method("pkg/Example.run()V", Equal(JAVA_11, JAVA_17))] pub fn run() {}"#,
    ] {
        let root = fixture(source)?;
        assert!(
            registry::generate(root.path()).is_err(),
            "accepted {source}"
        );
    }
    Ok(())
}

fn compile_overlap_checks(root: &Path, generated: &str, audio: bool) -> Result<bool> {
    let generated = syn::parse_file(generated)?;
    let checks = generated.items.iter().filter(
        |item| matches!(item, syn::Item::Macro(item) if item.mac.path.is_ident("compile_error")),
    );
    let source = quote::quote!(#(#checks)* fn main() {});
    let path = root.join("checks.rs");
    std::fs::write(&path, source.to_string())?;
    let mut command =
        std::process::Command::new(std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into()));
    command.arg(&path).arg("--out-dir").arg(root);
    if audio {
        command.args(["--cfg", "feature=\"audio\""]);
    }
    Ok(command.output()?.status.success())
}

#[test]
fn compiler_rejects_only_simultaneously_active_overlaps() -> Result<()> {
    let root = fixture(
        r#"
        #[intrinsic_method("pkg/Example.run()V", Any)] pub fn first() {}
        #[cfg(feature = "audio")]
        #[intrinsic_method("pkg/Example.run()V", Equal(JAVA_21))] pub fn second() {}
    "#,
    )?;
    let output = registry::generate(root.path())?;
    assert!(compile_overlap_checks(root.path(), &output, false)?);
    assert!(!compile_overlap_checks(root.path(), &output, true)?);
    Ok(())
}

#[test]
fn disjoint_versions_and_platforms_are_valid() -> Result<()> {
    let root = fixture(
        r#"
        #[intrinsic_method("pkg/Example.run()V", Equal(JAVA_8))] pub fn first() {}
        #[intrinsic_method("pkg/Example.run()V", In(&[JAVA_11, JAVA_17]))] pub fn second() {}
        #[cfg(feature = "audio")]
        #[intrinsic_method("pkg/Example.run()V", GreaterThanOrEqual(JAVA_21))] pub fn third() {}
        #[cfg(not(feature = "audio"))]
        #[intrinsic_method("pkg/Example.run()V", GreaterThanOrEqual(JAVA_21))] pub fn fourth() {}
    "#,
    )?;
    let output = registry::generate(root.path())?;
    assert!(compile_overlap_checks(root.path(), &output, false)?);
    assert!(compile_overlap_checks(root.path(), &output, true)?);
    Ok(())
}

#[test]
fn resolves_paths_relative_to_external_files_and_inline_directories() -> Result<()> {
    let root = fixture("pub mod outer;")?;
    std::fs::create_dir_all(root.path().join("src/outer/inline"))?;
    std::fs::create_dir_all(root.path().join("src/override"))?;
    for (file, source) in [
        (
            "outer.rs",
            r#"
            #[path = "sibling.rs"] pub mod renamed;
            pub mod inline { #[path = "leaf.rs"] pub mod leaf; }
            #[path = "override"] pub mod overridden { pub mod child; }
        "#,
        ),
        (
            "sibling.rs",
            r#"#[intrinsic_method("pkg/Example.sibling()V", Any)] pub fn run() {}"#,
        ),
        (
            "outer/inline/leaf.rs",
            r#"#[intrinsic_method("pkg/Example.leaf()V", Any)] pub fn run() {}"#,
        ),
        (
            "override/child.rs",
            r#"#[intrinsic_method("pkg/Example.child()V", Any)] pub fn run() {}"#,
        ),
    ] {
        std::fs::write(root.path().join("src").join(file), source)?;
    }
    let output = registry::generate(root.path())?;
    for path in [
        "outer :: renamed",
        "outer :: inline :: leaf",
        "outer :: overridden :: child",
    ] {
        assert!(output.contains(path));
    }
    Ok(())
}

#[test]
fn nested_cfg_attr_cannot_hide_overlaps_or_unsupported_paths() -> Result<()> {
    let root = fixture(
        r#"
        #[intrinsic_method("pkg/Example.run()V", Any)] pub fn first() {}
        #[cfg_attr(all(), cfg_attr(all(), cfg(feature = "audio")))]
        #[intrinsic_method("pkg/Example.run()V", Any)] pub fn second() {}
    "#,
    )?;
    let output = registry::generate(root.path())?;
    assert!(compile_overlap_checks(root.path(), &output, false)?);
    assert!(!compile_overlap_checks(root.path(), &output, true)?);
    let root = fixture(r#"#[cfg_attr(all(), cfg_attr(all(), path = "hidden.rs"))] mod hidden;"#)?;
    assert!(
        registry::generate(root.path())
            .unwrap_err()
            .to_string()
            .contains("unsupported")
    );
    Ok(())
}

#[test]
fn rejects_cfg_attr_intrinsics_instead_of_silently_omitting_them() -> Result<()> {
    for attribute in [
        r#"#[cfg_attr(feature = "audio", intrinsic_method("pkg/Example.conditional()V", Any))]"#,
        r#"#[cfg_attr(feature = "audio", ristretto_macros::intrinsic_method("pkg/Example.conditional()V", Any))]"#,
        r#"#[cfg_attr(all(), cfg_attr(feature = "audio", intrinsic_method("pkg/Example.conditional()V", Any)))]"#,
    ] {
        let root = fixture(&format!(
            r#"
            #[intrinsic_method("pkg/Example.always()V", Any)] pub fn always() {{}}
            {attribute}
            pub fn conditional() {{}}
            "#
        ))?;
        let error = registry::generate(root.path())
            .expect_err("conditional intrinsic declarations must not be silently omitted");
        assert!(error.to_string().contains("unsupported"), "{error}");
    }
    Ok(())
}
