use crate::Error::InternalError;
use crate::Result;
use crate::intrinsic_methods::java::lang::class;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::Between;
use ristretto_classfile::{JAVA_11, JAVA_21};
use ristretto_classloader::{Class, Value};
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

/// Converts an object in a vector to a primitive value if it is a primitive object.
///
/// # Errors
///
/// If the object is not a primitive or cannot be converted to a primitive.
pub(crate) fn unbox_primitive(values: &mut [Value], index: usize) -> Result<()> {
    let Some(value) = values.get(index) else {
        return Err(InternalError(format!("index out of bounds: {index}")));
    };
    let reference = value.as_reference()?;
    let object = reference.as_object_ref()?;
    let value = object.value("value")?;
    values[index] = value;
    Ok(())
}

#[intrinsic_method(
    "jdk/internal/reflect/NativeConstructorAccessorImpl.newInstance0(Ljava/lang/reflect/Constructor;[Ljava/lang/Object;)Ljava/lang/Object;",
    Between(JAVA_11, JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn new_instance_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let mut arguments: Vec<Value> = parameters.pop()?.try_into()?;
    let method = parameters.pop_object()?;

    let class_object = method.value("clazz")?;
    let class_object = class_object.as_object_ref()?;
    let class = class::get_class(&thread, class_object).await?;
    let class_name = class.name();
    let parameter_types: Vec<Value> = method.value("parameterTypes")?.try_into()?;
    let mut descriptor = String::new();
    for (index, parameter_type) in parameter_types.iter().enumerate() {
        let reference = parameter_type.as_reference()?;
        let parameter_type = reference.as_object_ref()?;
        let parameter_type_class = class::get_class(&thread, parameter_type).await?;
        let class_name = parameter_type_class.name();
        let class_descriptor = Class::convert_to_descriptor(class_name);
        descriptor.push_str(class_descriptor.as_str());
        if parameter_type_class.is_primitive() {
            unbox_primitive(&mut arguments, index)?;
        }
    }

    let result = thread.object(class_name, descriptor, &arguments).await?;
    Ok(Some(result))
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::JavaObject;
    use crate::intrinsic_methods::registry::IntrinsicMethod;

    pub(crate) async fn new_instance_test(new_instance: IntrinsicMethod) -> Result<()> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let integer_class = thread.class("java/lang/Integer").await?;
        let integer_class_object = integer_class.to_object(&thread).await?;

        let class = thread.class("java/lang/Class").await?;
        let string_class = thread.class("java/lang/String").await?;
        let string_class_object = string_class.to_object(&thread).await?;
        let arguments = Value::try_from((class.clone(), vec![string_class_object]))?;

        let constructor = vm
            .invoke(
                "java.lang.Class",
                "getDeclaredConstructor([Ljava/lang/Class;)Ljava/lang/reflect/Constructor;",
                &[integer_class_object, arguments],
            )
            .await?
            .expect("constructor");

        let string_parameter = "42".to_object(&thread).await?;
        let parameters = Value::try_from((class, vec![string_parameter]))?;
        let parameters = Parameters::new(vec![constructor, parameters]);
        let result = new_instance(thread, parameters).await?.expect("integer");
        let result = result.as_object_ref()?;
        let value = result.value("value")?.as_i32()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_new_instance_0() -> Result<()> {
        new_instance_test(new_instance_0).await
    }
}
