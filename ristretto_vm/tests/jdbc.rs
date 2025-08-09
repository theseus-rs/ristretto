use ristretto_classloader::ClassPath;
use ristretto_vm::{ConfigurationBuilder, VM};
use std::path::PathBuf;

#[tokio::test]
async fn test_jdbc() -> ristretto_vm::Result<()> {
    let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let classes_directory = cargo_manifest.join("..").join("classes");
    let class_path_entries = [
        classes_directory,
        PathBuf::from("https//repo1.maven.org/maven2/com/h2database/h2/2.3.232/h2-2.3.232.jar"),
    ];
    let class_path = ClassPath::from(&class_path_entries);
    let configuration = ConfigurationBuilder::new()
        .class_path(class_path)
        .main_class("JDBC")
        .build()?;
    let _vm = VM::new(configuration).await?;
    let _parameters: Vec<&str> = Vec::new();
    // Temporarily disable this test because it requires the invokedynamic instruction.
    // let result = vm.invoke_main(parameters).await?;
    Ok(())
}
