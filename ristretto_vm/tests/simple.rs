use ristretto_vm::{ClassPath, ConfigurationBuilder, Result, Value, VM};
use std::path::PathBuf;
use std::sync::Arc;

async fn vm() -> Result<Arc<VM>> {
    let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let classes_jar_path = cargo_manifest.join("../classes/classes.jar");
    let class_path = ClassPath::from(classes_jar_path.to_string_lossy());
    let configuration = ConfigurationBuilder::new()
        .class_path(class_path.clone())
        .main_class("Simple")
        .build()?;
    VM::new(configuration).await
}

#[tokio::test]
async fn test_main_method() -> Result<()> {
    let vm = vm().await?;
    let main_class_name = vm.main_class().expect("main class");
    let main_class = vm.class(main_class_name).await?;
    let main_method = main_class.main_method().expect("main method");
    let arguments = vec![Value::Object(None)];
    let result = vm.invoke(&main_class, &main_method, arguments).await?;
    assert!(result.is_none());
    Ok(())
}
