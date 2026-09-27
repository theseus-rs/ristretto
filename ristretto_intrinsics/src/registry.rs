//! Generated typed intrinsic registrations.
include!(concat!(env!("OUT_DIR"), "/intrinsic_registry.rs"));

#[cfg(test)]
mod tests {
    #[test]
    fn audio_registration_uses_intrinsics_features() -> ristretto_types::Result<()> {
        let mut registry = ristretto_types::IntrinsicRegistry::<ristretto_vm::Thread>::new(
            &ristretto_classfile::JAVA_21,
        );
        super::register(&mut registry)?;
        assert_eq!(
            registry
                .methods()
                .contains_key("com/sun/media/sound/DirectAudioDeviceProvider.nGetNumDevices()I"),
            cfg!(feature = "audio")
        );
        Ok(())
    }

    // Check the feature in its owning crate: dependents can enable audio directly.
    #[cfg(not(target_family = "wasm"))]
    #[tokio::test]
    async fn test_audio_dispatch_uses_intrinsics_features() -> ristretto_types::Result<()> {
        use ristretto_classfile::{ClassFile, ConstantPool, MethodAccessFlags};
        use ristretto_classloader::{Class, Value};
        use ristretto_types::{Error, JavaError::UnsatisfiedLinkError};

        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut constant_pool = ConstantPool::default();
        let this_class =
            constant_pool.add_class("com/sun/media/sound/DirectAudioDeviceProvider")?;
        let name_index = constant_pool.add_utf8("nGetNumDevices")?;
        let descriptor_index = constant_pool.add_utf8("()I")?;
        let method = ristretto_classfile::Method {
            access_flags: MethodAccessFlags::PUBLIC
                | MethodAccessFlags::STATIC
                | MethodAccessFlags::NATIVE,
            name_index,
            descriptor_index,
            ..Default::default()
        };
        let class_file = ClassFile {
            constant_pool,
            this_class,
            methods: vec![method],
            ..Default::default()
        };
        let class = Class::from(None, class_file)?;
        let method = class.try_get_method("nGetNumDevices", "()I")?;

        let result = thread.execute(&class, &method, &[] as &[Value]).await;
        if cfg!(feature = "audio") {
            assert_eq!(result?, Some(Value::Int(1)));
        } else {
            let error = result.expect_err("disabled audio intrinsic should fail");
            assert!(matches!(
                error,
                Error::JavaError(UnsatisfiedLinkError(message))
                    if message == "'com/sun/media/sound/DirectAudioDeviceProvider.nGetNumDevices()I'"
            ));
        }
        Ok(())
    }
}
