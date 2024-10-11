use ristretto_vm::{ClassPath, ConfigurationBuilder, Result, Value, VM};
use std::path::PathBuf;

fn vm() -> Result<VM> {
    let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let classes_jar_path = cargo_manifest.join("../classes/classes.jar");
    let class_path = ClassPath::from(classes_jar_path.to_string_lossy());
    let configuration = ConfigurationBuilder::new()
        .class_path(class_path.clone())
        .main_class("HelloWorld")
        .build();
    VM::new(configuration)
}

#[test]
fn test_main_method() -> Result<()> {
    let vm = vm()?;
    let main_class_name = vm.main_class().expect("main class");
    let main_class = vm.load(main_class_name)?;
    let main_method = main_class.main_method().expect("main method");
    let arguments = vec![Value::Object(None)];
    let result = vm.invoke(&main_class, &main_method, arguments)?;
    assert!(result.is_none());
    Ok(())
}
