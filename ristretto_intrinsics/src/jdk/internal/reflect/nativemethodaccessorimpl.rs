use crate::java::lang::class;
use ristretto_classfile::MethodAccessFlags;
use ristretto_classfile::VersionSpecification::Between;
use ristretto_classfile::{JAVA_11, JAVA_21};
use ristretto_classloader::{Class as RistrettoClass, ResolvedMethod, Value};
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use ristretto_types::{Thread, method_resolution};
use std::sync::Arc;

/// Invoke the target after Method.invoke has checked access and setAccessible permissions.
/// The accessor can hold the root Method, whose override flag differs from the checked copy.
#[intrinsic_method(
    "jdk/internal/reflect/NativeMethodAccessorImpl.invoke0(Ljava/lang/reflect/Method;Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;",
    Between(JAVA_11, JAVA_21)
)]
pub async fn invoke_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let mut arguments: Vec<Value> = parameters.pop()?.try_into()?;
    let object = parameters.pop_reference()?;
    if let Some(object) = object {
        arguments.insert(0, Value::from(object));
    }
    let method = parameters.pop()?;
    let (name, class_object, parameter_types, return_type, modifiers) = {
        let method = method.as_object_ref()?;
        let parameter_types: Vec<Value> = method.value("parameterTypes")?.try_into()?;
        (
            method.value("name")?.as_string()?,
            method.value("clazz")?,
            parameter_types,
            method.value("returnType")?,
            method.value("modifiers")?.as_i32()?,
        )
    };
    let class = class::get_class(&thread, &class_object).await?;
    let access_flags = MethodAccessFlags::from_bits_truncate(u16::try_from(modifiers)?);

    let mut method_parameters = String::new();
    for parameter_type in &parameter_types {
        let parameter_type_class = class::get_class(&thread, parameter_type).await?;
        let descriptor = RistrettoClass::convert_to_descriptor(parameter_type_class.name());
        method_parameters.push_str(&descriptor);
    }

    let return_type_class = class::get_class(&thread, &return_type).await?;
    let is_void = return_type_class.name() == "void";
    let return_type_descriptor = RistrettoClass::convert_to_descriptor(return_type_class.name());
    let descriptor = format!("({method_parameters}){return_type_descriptor}");

    let method = class.try_get_method(&name, &descriptor)?;
    let resolved = ResolvedMethod {
        declaring_class: class,
        method,
    };
    let target = if access_flags.contains(MethodAccessFlags::STATIC) {
        resolved
    } else {
        let receiver = arguments
            .first()
            .ok_or(ristretto_types::JavaError::NullPointerException(None))?;
        if receiver.is_null() {
            return Err(ristretto_types::JavaError::NullPointerException(None).into());
        }
        let receiver_class = method_resolution::receiver_class(thread.as_ref(), receiver).await?;
        receiver_class.select_concrete_method(&resolved)?
    };
    let result = thread
        .execute(&target.declaring_class, &target.method, &arguments)
        .await?;

    // For void methods, return null (as Object). For other methods, return the result.
    // This is required because Method.invoke() always returns Object in Java.
    if is_void {
        Ok(Some(Value::Object(None)))
    } else {
        Ok(result)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use ristretto_classloader::Reference;
    use ristretto_types::JavaObject;
    use std::future::Future;

    pub async fn invoke_test<F: Future<Output = Result<Option<Value>>>>(
        invoke: impl Fn(Arc<ristretto_vm::Thread>, Parameters) -> F,
    ) -> Result<()> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let integer_class = thread.class("java/lang/Integer").await?;
        let integer_class_object = integer_class.to_object(&thread).await?;

        let method_name = "valueOf".to_object(&thread).await?;
        let class = thread.class("java/lang/Class").await?;
        let string_class = thread.class("java/lang/String").await?;
        let string_class_object = string_class.to_object(&thread).await?;
        let reference = Reference::try_from((class.clone(), vec![string_class_object]))?;
        let arguments = Value::new_object(vm.garbage_collector(), reference);

        let method = vm
            .invoke(
                "java.lang.Class",
                "getDeclaredMethod(Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/reflect/Method;",
                &[integer_class_object, method_name, arguments],
            )
            .await?
            .expect("method");

        let string_parameter = "42".to_object(&thread).await?;
        let reference = Reference::try_from((class, vec![string_parameter]))?;
        let parameters = Value::new_object(vm.garbage_collector(), reference);
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
