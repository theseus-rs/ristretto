use ristretto_vm::{Result, VM};

#[tokio::test]
async fn test_is_enum() -> Result<()> {
    let vm = VM::default().await?;
    let class_object = vm
        .invoke(
            "java.lang.Class",
            "forName(Ljava/lang/String;)Ljava/lang/Class;",
            &["java.util.concurrent.TimeUnit"],
        )
        .await?
        .expect("class object");

    let modifiers = vm
        .invoke(
            "java.lang.Class",
            "getModifiers()I",
            &[class_object.clone()],
        )
        .await?
        .expect("modifiers")
        .as_i32()?;
    assert_eq!(16_401, modifiers);

    let super_class = vm
        .invoke(
            "java.lang.Class",
            "getSuperclass()Ljava/lang/Class;",
            &[class_object.clone()],
        )
        .await?
        .expect("super class");
    let super_class_name = {
        let super_class = super_class.as_object_ref()?;
        super_class.value("name")?.as_string()?
    };
    assert_eq!("java.lang.Enum", super_class_name);

    let is_enum = vm
        .invoke("java.lang.Class", "isEnum()Z", &[class_object])
        .await?
        .expect("is enum")
        .as_bool()?;
    assert!(is_enum);

    Ok(())
}
