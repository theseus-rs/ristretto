use crate::Result;
use crate::intrinsic_methods::java::lang::class;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::Between;
use ristretto_classfile::{JAVA_11, JAVA_21};
use ristretto_classloader::{Object, Value};
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "jdk/internal/reflect/NativeConstructorAccessorImpl.newInstance0(Ljava/lang/reflect/Constructor;[Ljava/lang/Object;)Ljava/lang/Object;",
    Between(JAVA_11, JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn new_instance_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let arguments: Vec<Value> = parameters.pop()?.try_into()?;
    let method = parameters.pop_object()?;

    let class_object: Object = method.value("clazz")?.try_into()?;
    let class = class::get_class(&thread, &class_object).await?;
    let class_name = class.name();
    let parameter_types: Vec<Value> = method.value("parameterTypes")?.try_into()?;
    let mut descriptor = String::new();
    for parameter_type in parameter_types {
        let parameter_type: Object = parameter_type.try_into()?;
        let parameter_type_class = class::get_class(&thread, &parameter_type).await?;
        if parameter_type_class.is_array() || parameter_type_class.is_primitive() {
            descriptor.push_str(parameter_type_class.name());
        } else {
            let parameter_type = format!("L{};", parameter_type_class.name());
            descriptor.push_str(parameter_type.as_str());
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
        let integer_class_object = integer_class.to_object(&vm).await?;

        let class = thread.class("java/lang/Class").await?;
        let string_class = thread.class("java/lang/String").await?;
        let string_class_object = string_class.to_object(&vm).await?;
        let arguments = Value::try_from((class.clone(), vec![string_class_object]))?;

        let constructor = vm
            .invoke(
                "java.lang.Class",
                "getDeclaredConstructor",
                "([Ljava/lang/Class;)Ljava/lang/reflect/Constructor;",
                &[integer_class_object, arguments],
            )
            .await?
            .expect("constructor");

        let string_parameter = "42".to_object(&vm).await?;
        let parameters = Value::try_from((class, vec![string_parameter]))?;
        let parameters = Parameters::new(vec![constructor, parameters]);
        let result: Object = new_instance(thread, parameters)
            .await?
            .expect("integer")
            .try_into()?;
        let value: i32 = result.value("value")?.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_new_instance_0() -> Result<()> {
        new_instance_test(new_instance_0).await
    }
}
