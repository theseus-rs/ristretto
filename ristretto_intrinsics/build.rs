#[path = "build/registry.rs"]
mod registry;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = std::path::PathBuf::from(
        std::env::var_os("CARGO_MANIFEST_DIR").ok_or("missing manifest directory")?,
    );
    let output =
        std::path::PathBuf::from(std::env::var_os("OUT_DIR").ok_or("missing output directory")?)
            .join("intrinsic_registry.rs");
    let generated = registry::generate(&root)?;
    if std::fs::read_to_string(&output).ok().as_deref() != Some(&generated) {
        std::fs::write(output, generated)?;
    }
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=build");
    Ok(())
}
