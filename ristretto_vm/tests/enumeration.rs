use ristretto_classloader::Object;
use ristretto_vm::{JavaObject, Result, VM};

#[tokio::test]
async fn test_is_enum() -> Result<()> {
    let vm = VM::default().await?;
    let class = vm.class("java.util.concurrent.TimeUnit").await?;
    let class_object = class.to_object(&vm).await?;

    let modifiers: i32 = vm
        .invoke(
            "java.lang.Class",
            "getModifiers",
            "()I",
            &[class_object.clone()],
        )
        .await?
        .expect("modifiers")
        .try_into()?;
    assert_eq!(16_401, modifiers);

    let super_class: Object = vm
        .invoke(
            "java.lang.Class",
            "getSuperclass",
            "()Ljava/lang/Class;",
            &[class_object.clone()],
        )
        .await?
        .expect("super class")
        .try_into()?;
    let super_class_name: String = super_class.value("name")?.try_into()?;
    assert_eq!("java.lang.Enum", super_class_name);

    let is_enum: bool = vm
        .invoke("java.lang.Class", "isEnum", "()Z", &[class_object])
        .await?
        .expect("is enum")
        .try_into()?;
    assert!(is_enum);

    Ok(())
}
