use crate::Result;
use crate::intrinsic_methods::java::lang::class;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::Between;
use ristretto_classfile::{JAVA_11, JAVA_21};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "jdk/internal/reflect/NativeMethodAccessorImpl.invoke0(Ljava/lang/reflect/Method;Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;",
    Between(JAVA_11, JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn invoke_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let mut arguments: Vec<Value> = parameters.pop()?.try_into()?;
    let object = parameters.pop_reference()?;
    if let Some(object) = object {
        arguments.insert(0, Value::from(object));
    }
    let method = parameters.pop()?;
    let (name, class_object, parameter_types, return_type) = {
        let method = method.as_object_ref()?;
        let name = method.value("name")?.as_string()?;
        let class_object = method.value("clazz")?;
        let parameter_types: Vec<Value> = method.value("parameterTypes")?.try_into()?;
        let return_type = method.value("returnType")?;
        (name, class_object, parameter_types, return_type)
    };
    let class = class::get_class(&thread, &class_object).await?;
    let mut parameters = String::new();
    for parameter_type in &parameter_types {
        let parameter_type_class = class::get_class(&thread, parameter_type).await?;
        if parameter_type_class.is_array() || parameter_type_class.is_primitive() {
            parameters.push_str(parameter_type_class.name());
        } else {
            let parameter_type = format!("L{};", parameter_type_class.name());
            parameters.push_str(parameter_type.as_str());
        }
    }

    let return_type_class = class::get_class(&thread, &return_type).await?;
    let return_type_class = if return_type_class.is_array() || return_type_class.is_primitive() {
        return_type_class.name().to_string()
    } else {
        format!("L{};", return_type_class.name())
    };
    let descriptor = format!("({parameters}){return_type_class}");

    let method = class.try_get_method(name, descriptor)?;
    thread.execute(&class, &method, &arguments).await
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::JavaObject;
    use crate::intrinsic_methods::registry::IntrinsicMethod;

    pub(crate) async fn invoke_test(invoke: IntrinsicMethod) -> Result<()> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let integer_class = thread.class("java/lang/Integer").await?;
        let integer_class_object = integer_class.to_object(&thread).await?;

        let method_name = "valueOf".to_object(&thread).await?;
        let class = thread.class("java/lang/Class").await?;
        let string_class = thread.class("java/lang/String").await?;
        let string_class_object = string_class.to_object(&thread).await?;
        let arguments = Value::try_from((class.clone(), vec![string_class_object]))?;

        let method = vm
            .invoke(
                "java.lang.Class",
                "getDeclaredMethod(Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/reflect/Method;",
                &[integer_class_object, method_name, arguments],
            )
            .await?
            .expect("method");

        let string_parameter = "42".to_object(&thread).await?;
        let parameters = Value::try_from((class, vec![string_parameter]))?;
        let parameters = Parameters::new(vec![method, Value::Object(None), parameters]);
        let value = invoke(thread, parameters)
            .await?
            .expect("integer")
            .as_i32()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_invoke_0() -> Result<()> {
        invoke_test(invoke_0).await
    }
}
