use ristretto_vm::{ClassPath, ConfigurationBuilder, Result, VM};
use std::io::Cursor;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

async fn test_helloworld(java_verison: &str) -> Result<()> {
    let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let classes_jar_path = cargo_manifest
        .join("..")
        .join("classes")
        .join("classes.jar");
    let class_path = ClassPath::from(&[classes_jar_path]);
    let stdout_buffer = Arc::new(Mutex::new(Cursor::new(Vec::<u8>::new())));

    let configuration = ConfigurationBuilder::new()
        .class_path(class_path.clone())
        .main_class("HelloWorld")
        .java_version(java_verison)
        .stdout(stdout_buffer.clone())
        .build()?;
    let vm = VM::new(configuration).await?;
    let parameters = vec!["world!"];

    let result = vm.invoke_main(&parameters).await?;
    assert!(result.is_none());
    let output = stdout_buffer.lock().await;
    let output_bytes = output.get_ref();
    let output_str = String::from_utf8(output_bytes.clone()).expect("Invalid UTF-8 output");
    assert_eq!(output_str.trim(), "Hello world!");
    Ok(())
}

#[ignore = "Java 8 support not fully implemented"]
#[tokio::test]
async fn test_helloworld_v8() -> Result<()> {
    test_helloworld("8.472.08.1").await
}

#[tokio::test]
async fn test_helloworld_v11() -> Result<()> {
    test_helloworld("11.0.29.7.1").await
}

#[tokio::test]
async fn test_helloworld_v17() -> Result<()> {
    test_helloworld("17.0.17.10.1").await
}

#[tokio::test]
async fn test_helloworld_v21() -> Result<()> {
    test_helloworld("21.0.9.10.1").await
}

#[tokio::test]
async fn test_helloworld_v25() -> Result<()> {
    test_helloworld("25.0.1.8.1").await
}
