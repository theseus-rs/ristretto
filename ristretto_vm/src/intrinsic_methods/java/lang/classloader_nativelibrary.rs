use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "java/lang/ClassLoader$NativeLibrary.find(Ljava/lang/String;)J",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn find(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(Some(Value::Long(1)))
}

#[intrinsic_method(
    "java/lang/ClassLoader$NativeLibrary.findEntry(Ljava/lang/String;)J",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn find_entry(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Long(2)))
}

#[intrinsic_method(
    "java/lang/ClassLoader$NativeLibrary.load(Ljava/lang/String;Z)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn load(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_builtin = parameters.pop_bool()?;
    let _name = parameters.pop()?.as_string()?;
    let native_library_value = parameters.pop()?;
    let mut native_library = native_library_value.as_object_mut()?;
    native_library.set_value("handle", Value::Long(1))?;
    native_library.set_value("jniVersion", Value::Int(0x0001_0008))?; // JNI 1.8
    native_library.set_value("loaded", Value::from(true))?;

    Ok(None)
}

#[intrinsic_method(
    "java/lang/ClassLoader$NativeLibrary.load0(Ljava/lang/String;ZZ)Z",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn load_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(Some(Value::from(true)))
}

#[intrinsic_method(
    "java/lang/ClassLoader$NativeLibrary.unload(Ljava/lang/String;Z)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn unload_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "java/lang/ClassLoader$NativeLibrary.unload(Ljava/lang/String;ZJ)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn unload_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::java_object::JavaObject;
    use ristretto_classfile::{BaseType, FieldType};
    use ristretto_classloader::Reference;

    #[tokio::test]
    async fn test_find() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = find(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Long(1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_find_entry() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = find_entry(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Long(2)));
        Ok(())
    }

    #[tokio::test]
    async fn test_load() -> Result<()> {
        let (vm, thread) = crate::test::thread().await?;
        let mut parameters = Parameters::default();

        // Create a mock java.lang.ClassLoader$NativeLibrary class
        let class_name = "java/lang/ClassLoader$NativeLibrary";
        let mut constant_pool = ristretto_classfile::ConstantPool::default();
        let this_class = constant_pool.add_class(class_name)?;
        let handle_name_index = constant_pool.add_utf8("handle")?;
        let handle_descriptor_index = constant_pool.add_utf8("J")?;
        let jni_version_name_index = constant_pool.add_utf8("jniVersion")?;
        let jni_version_descriptor_index = constant_pool.add_utf8("I")?;
        let loaded_name_index = constant_pool.add_utf8("loaded")?;
        let loaded_descriptor_index = constant_pool.add_utf8("Z")?;

        let mut class_file = ristretto_classfile::ClassFile {
            constant_pool,
            this_class,
            ..Default::default()
        };

        class_file.fields.push(ristretto_classfile::Field {
            access_flags: ristretto_classfile::FieldAccessFlags::PUBLIC,
            name_index: handle_name_index,
            descriptor_index: handle_descriptor_index,
            attributes: vec![],
            field_type: FieldType::Base(BaseType::Long),
        });
        class_file.fields.push(ristretto_classfile::Field {
            access_flags: ristretto_classfile::FieldAccessFlags::PUBLIC,
            name_index: jni_version_name_index,
            descriptor_index: jni_version_descriptor_index,
            attributes: vec![],
            field_type: FieldType::Base(BaseType::Int),
        });
        class_file.fields.push(ristretto_classfile::Field {
            access_flags: ristretto_classfile::FieldAccessFlags::PUBLIC,
            name_index: loaded_name_index,
            descriptor_index: loaded_descriptor_index,
            attributes: vec![],
            field_type: FieldType::Base(BaseType::Boolean),
        });
        let class = ristretto_classloader::Class::from(None, class_file)?;
        thread.register_class(class.clone()).await?;

        let native_library_class = thread.class(class_name).await?;
        let native_library_instance = ristretto_classloader::Object::new(native_library_class)?;
        let native_library_value = Value::new_object(
            vm.garbage_collector(),
            Reference::Object(native_library_instance),
        );
        parameters.push(native_library_value);
        parameters.push("foo".to_object(&thread).await?); // name
        parameters.push_bool(false); // is_builtin
        let result = load(thread, parameters).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_load_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = load_0(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::from(true)));
        Ok(())
    }

    #[tokio::test]
    async fn test_unload_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = unload_0(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_unload_1() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = unload_1(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
