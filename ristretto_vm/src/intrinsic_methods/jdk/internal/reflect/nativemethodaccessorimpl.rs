use crate::Result;
use crate::intrinsic_methods::java::lang::class;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::{Object, Value};
use std::sync::Arc;

const CLASS_NAME: &str = "jdk/internal/reflect/NativeMethodAccessorImpl";

/// Register all intrinsic methods for `jdk.internal.reflect.NativeMethodAccessorImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "invoke0",
        "(Ljava/lang/reflect/Method;Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;",
        invoke_0,
    );
}

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
    let method = parameters.pop_object()?;

    let name: String = method.value("name")?.try_into()?;
    let class_object: Object = method.value("clazz")?.try_into()?;
    let class = class::get_class(&thread, &class_object).await?;
    let parameter_types: Vec<Value> = method.value("parameterTypes")?.try_into()?;
    let mut parameters = String::new();
    for parameter_type in parameter_types {
        let parameter_type: Object = parameter_type.try_into()?;
        let parameter_type_class = class::get_class(&thread, &parameter_type).await?;
        if parameter_type_class.is_array() || parameter_type_class.is_primitive() {
            parameters.push_str(parameter_type_class.name());
        } else {
            let parameter_type = format!("L{};", parameter_type_class.name());
            parameters.push_str(parameter_type.as_str());
        }
    }

    let return_type: Object = method.value("returnType")?.try_into()?;
    let return_type_class = class::get_class(&thread, &return_type).await?;
    let return_type_class = if return_type_class.is_array() || return_type_class.is_primitive() {
        return_type_class.name().to_string()
    } else {
        format!("L{};", return_type_class.name())
    };
    let descriptor = format!("({parameters}){return_type_class}");

    let method = class.try_get_method(name, descriptor)?;
    let result = thread.execute(&class, &method, arguments).await?;
    Ok(result)
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::JavaObject;
    use crate::intrinsic_methods::registry::IntrinsicMethod;

    pub(crate) async fn invoke_test(invoke: IntrinsicMethod) -> Result<()> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let integer_class = vm.class("java/lang/Integer").await?;
        let integer_class_object = integer_class.to_object(&vm).await?;

        let method_name = "valueOf".to_object(&vm).await?;
        let class = vm.class("java/lang/Class").await?;
        let string_class = vm.class("java/lang/String").await?;
        let string_class_object = string_class.to_object(&vm).await?;
        let arguments = Value::try_from((class.clone(), vec![string_class_object]))?;

        let method = vm
            .invoke(
                "java.lang.Class",
                "getDeclaredMethod",
                "(Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/reflect/Method;",
                vec![integer_class_object, method_name, arguments],
            )
            .await?
            .expect("method");

        let string_parameter = "42".to_object(&vm).await?;
        let parameters = Value::try_from((class, vec![string_parameter]))?;
        let parameters = Parameters::new(vec![method, Value::Object(None), parameters]);
        let value: i32 = invoke(thread, parameters)
            .await?
            .expect("integer")
            .try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_invoke_0() -> Result<()> {
        invoke_test(invoke_0).await
    }
}
