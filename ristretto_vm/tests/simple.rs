use ristretto_vm::{ClassPath, ConfigurationBuilder, Result, VM};
use std::path::PathBuf;

#[tokio::test]
async fn test_simple_main_method() -> Result<()> {
    let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let classes_jar_path = cargo_manifest
        .join("..")
        .join("classes")
        .join("classes.jar");
    let class_path = ClassPath::from(classes_jar_path.to_string_lossy());
    let configuration = ConfigurationBuilder::new()
        .class_path(class_path.clone())
        .main_class("Simple")
        .build()?;
    let vm = VM::new(configuration).await?;
    let arguments: Vec<&str> = Vec::new();
    let result = vm.invoke_main(arguments).await?;
    assert!(result.is_none());
    Ok(())
}
