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
    let value = {
        let reference = value.as_reference()?;
        let object = reference.as_object_ref()?;
        object.value("value")?
    };
    values[index] = value;
    Ok(())
}

/// Gets the caller module from the call stack.
///
/// Walks up the call stack to find the first frame that is not in the reflection
/// implementation, and returns that frame's module.
async fn get_caller_module(thread: &Arc<Thread>) -> Result<Option<String>> {
    let frames = thread.frames().await?;
    // Skip reflection frames to find the actual caller
    for frame in frames.iter().rev() {
        let class_name = frame.class().name();
        // Skip reflection implementation classes
        if class_name.starts_with("java/lang/reflect/")
            || class_name.starts_with("jdk/internal/reflect/")
            || class_name.starts_with("sun/reflect/")
        {
            continue;
        }
        return frame.class().module_name().map_err(Into::into);
    }
    // If all frames are reflection frames, return unnamed module
    Ok(None)
}

/// Creates a new instance via reflection.
///
/// This method implements JPMS module access checking for reflective constructor access.
/// For non-public constructors, the target module must open the package to the caller module.
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
    let method = parameters.pop()?;
    let (class_object, parameter_types, override_flag) = {
        let method = method.as_object_ref()?;
        let class_object = method.value("clazz")?;
        let parameter_types: Vec<Value> = method.value("parameterTypes")?.try_into()?;
        // Check if setAccessible(true) was called (override flag)
        let override_flag = method
            .value("override")
            .map(|v| v.as_i32().unwrap_or(0) != 0)
            .unwrap_or(false);
        (class_object, parameter_types, override_flag)
    };
    let class = class::get_class(&thread, &class_object).await?;

    // Check module reflection access unless setAccessible(true) was called
    // Note: In a full implementation, even setAccessible requires proper module opens
    // via --add-opens or Module.addOpens(). For now, we allow setAccessible to bypass.
    if !override_flag {
        let vm = thread.vm()?;
        let caller_module = get_caller_module(&thread).await?;
        let target_module = class.module_name()?;

        // Only check if modules are different
        if caller_module != target_module {
            let result = vm.module_system().check_reflection_access(
                caller_module.as_deref(),
                target_module.as_deref(),
                class.name(),
            );

            // For system modules, allow access (they handle their own opens)
            // For application modules, enforce strictly
            if result.is_denied() {
                let target = target_module.as_deref().unwrap_or("");
                if !target.starts_with("java.")
                    && !target.starts_with("jdk.")
                    && !target.starts_with("sun.")
                    && !target.starts_with("com.sun.")
                {
                    vm.module_system().require_reflection_access(
                        caller_module.as_deref(),
                        target_module.as_deref(),
                        class.name(),
                    )?;
                }
            }
        }
    }

    let class_name = class.name();
    let mut descriptor = String::new();
    for (index, parameter_type) in parameter_types.iter().enumerate() {
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
