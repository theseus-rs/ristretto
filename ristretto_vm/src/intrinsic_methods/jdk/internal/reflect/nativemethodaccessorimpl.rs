use crate::JavaError::{IllegalAccessException, InaccessibleObjectException};
use crate::Result;
use crate::intrinsic_methods::java::lang::class;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::MethodAccessFlags;
use ristretto_classfile::VersionSpecification::Between;
use ristretto_classfile::{JAVA_11, JAVA_21};
use ristretto_classloader::{Class as RistrettoClass, Value};
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

/// Gets the caller class info from the call stack.
///
/// Walks up the call stack to find the first frame that is not in the reflection
/// implementation, and returns that frame's class name, module name and a hash for the module.
async fn get_caller_info(thread: &Arc<Thread>) -> Result<(String, Option<String>, usize)> {
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
        let module_name = frame.class().module_name()?;
        // Use the class pointer as a proxy for the module identity hash
        let class_ptr = Arc::as_ptr(frame.class()) as usize;
        return Ok((class_name.to_string(), module_name, class_ptr));
    }
    // If all frames are reflection frames, return empty
    Ok((String::new(), None, 0))
}

/// Invokes a method via reflection.
///
/// This method implements JPMS module access checking for reflective method access.
/// For non-public methods, the target module must open the package to the caller module.
#[intrinsic_method(
    "jdk/internal/reflect/NativeMethodAccessorImpl.invoke0(Ljava/lang/reflect/Method;Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;",
    Between(JAVA_11, JAVA_21)
)]
#[async_recursion(?Send)]
#[expect(clippy::too_many_lines)]
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
    let (name, class_object, parameter_types, return_type, modifiers, override_flag) = {
        let method = method.as_object_ref()?;
        let name = method.value("name")?.as_string()?;
        let class_object = method.value("clazz")?;
        let parameter_types: Vec<Value> = method.value("parameterTypes")?.try_into()?;
        let return_type = method.value("returnType")?;
        let modifiers = method
            .value("modifiers")
            .and_then(|v| v.as_i32())
            .unwrap_or(0);
        // Check if setAccessible(true) was called (override flag)
        let override_flag = method
            .value("override")
            .map(|v| v.as_i32().unwrap_or(0) != 0)
            .unwrap_or(false);
        (
            name,
            class_object,
            parameter_types,
            return_type,
            modifiers,
            override_flag,
        )
    };
    let class = class::get_class(&thread, &class_object).await?;

    // Check Java access modifiers unless setAccessible(true) was called
    let access_flags = MethodAccessFlags::from_bits_truncate(u16::try_from(modifiers)?);
    let is_public = access_flags.contains(MethodAccessFlags::PUBLIC);
    let is_private = access_flags.contains(MethodAccessFlags::PRIVATE);
    let is_protected = access_flags.contains(MethodAccessFlags::PROTECTED);

    if !override_flag && !is_public {
        // For non-public methods, we need to check access
        let (caller_class_name, caller_module, caller_module_hash) =
            get_caller_info(&thread).await?;

        // Check if private method is being accessed from a different class
        if is_private {
            let target_class_name = class.name();
            // Private methods can only be accessed from the same class
            if caller_class_name != target_class_name {
                let class_name = class.name().replace('/', ".");
                let caller_display = caller_class_name.replace('/', ".");
                let error_msg = format!(
                    "class {caller_display} cannot access a member of class {class_name} with modifiers \"private\""
                );
                return Err(IllegalAccessException(error_msg).into());
            }
        } else if is_protected {
            // Protected methods: caller must be in same package or a subclass
            let target_package =
                crate::module_system::ModuleSystem::package_from_class_name(class.name());
            let caller_package =
                crate::module_system::ModuleSystem::package_from_class_name(&caller_class_name);

            if target_package != caller_package {
                // Not in same package, check if it's a subclass (simplified check)
                // For now, we allow access if in different packages; a full implementation would
                // check the class hierarchy
                let class_name = class.name().replace('/', ".");
                let caller_display = caller_class_name.replace('/', ".");
                let error_msg = format!(
                    "class {caller_display} cannot access a member of class {class_name} with modifiers \"protected\""
                );
                return Err(IllegalAccessException(error_msg).into());
            }
        } else {
            // Package-private (no public/private/protected modifier)
            let target_package =
                crate::module_system::ModuleSystem::package_from_class_name(class.name());
            let caller_package =
                crate::module_system::ModuleSystem::package_from_class_name(&caller_class_name);

            if target_package != caller_package {
                let class_name = class.name().replace('/', ".");
                let caller_display = caller_class_name.replace('/', ".");
                let error_msg = format!(
                    "class {caller_display} cannot access a member of class {class_name} with modifiers \"\""
                );
                return Err(IllegalAccessException(error_msg).into());
            }
        }

        // Also check JPMS module access
        let vm = thread.vm()?;
        let target_module = class.module_name()?;

        // Only check if modules are different
        if caller_module != target_module {
            let result = vm.module_system().check_reflection_access(
                caller_module.as_deref(),
                target_module.as_deref(),
                class.name(),
            );

            // Enforce module access for all reflective access when denied
            // Exception: system module to system module access is allowed (internal JDK usage)
            if result.is_denied() {
                let caller = caller_module.as_deref().unwrap_or("");
                let target = target_module.as_deref().unwrap_or("");

                let caller_is_system = caller.starts_with("java.")
                    || caller.starts_with("jdk.")
                    || caller.starts_with("sun.")
                    || caller.starts_with("com.sun.");
                let target_is_system = target.starts_with("java.")
                    || target.starts_with("jdk.")
                    || target.starts_with("sun.")
                    || target.starts_with("com.sun.");

                // Allow system-to-system access, deny all other denied cases
                if !(caller_is_system && target_is_system) {
                    // Build JDK-compatible error message:
                    // "Unable to make <modifiers> <return_type> <class>.<method>() accessible:
                    //  module <target> does not "opens <package>" to <caller> @<hash>"
                    let class_name = class.name().replace('/', ".");
                    let package =
                        crate::module_system::ModuleSystem::package_from_class_name(class.name())
                            .replace('/', ".");
                    let target_module_name = target_module.as_deref().unwrap_or("unnamed module");

                    // Build caller display with module hash for unnamed modules
                    // Format: "unnamed module @<hash>" or "module <name>"
                    let caller_display = if caller.is_empty() {
                        // For unnamed modules, include the identity hash code
                        if caller_module_hash != 0 {
                            format!("unnamed module @{:x}", caller_module_hash & 0xFFFF_FFFF)
                        } else {
                            "unnamed module".to_string()
                        }
                    } else {
                        format!("module {caller}")
                    };

                    // Build modifier string
                    let modifier_str = access_flags.as_code();

                    // Get return type name
                    let return_type_class = class::get_class(&thread, &return_type).await?;
                    let return_type_name = return_type_class.name().replace('/', ".");

                    let error_msg = format!(
                        "Unable to make {}{}{}.{}() accessible: module {} does not \"opens {}\" to {}",
                        if modifier_str.is_empty() {
                            String::new()
                        } else {
                            format!("{modifier_str} ")
                        },
                        if return_type_name.is_empty() {
                            String::new()
                        } else {
                            format!("{return_type_name} ")
                        },
                        class_name,
                        name,
                        target_module_name,
                        package,
                        caller_display
                    );
                    return Err(InaccessibleObjectException(error_msg).into());
                }
            }
        }
    }

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

    // For non-static methods, we need to dispatch to the actual implementation on the receiver's
    // class, not the declaring class (which may be an interface or abstract class)
    let is_static = access_flags.contains(MethodAccessFlags::STATIC);
    let (dispatch_class, method) = if is_static {
        // Static methods are called on the declaring class
        let method = class.try_get_method(&name, &descriptor)?;
        (class.clone(), method)
    } else {
        // For instance methods, get the receiver's class and find the method there
        // The receiver is the first argument
        if arguments.is_empty() {
            return Err(crate::Error::InternalError(
                "Instance method invocation requires a receiver".to_string(),
            ));
        }
        let receiver = arguments[0].as_object_ref()?;
        let receiver_class = receiver.class().clone();

        // Search for the method starting from the receiver's class. This handles interface methods,
        // abstract methods, and overridden methods
        if let Some((impl_class, method)) =
            find_method_in_class_hierarchy(&receiver_class, &name, &descriptor)?
        {
            (impl_class, method)
        } else {
            // Fallback: try the declaring class (should not happen for valid calls)
            let method = class.try_get_method(&name, &descriptor)?;
            (class.clone(), method)
        }
    };
    let result = thread.execute(&dispatch_class, &method, &arguments).await?;

    // For void methods, return null (as Object). For other methods, return the result.
    // This is required because Method.invoke() always returns Object in Java.
    if is_void {
        Ok(Some(Value::Object(None)))
    } else {
        Ok(result)
    }
}

/// Search for a non-abstract method implementation in the class hierarchy.
fn find_method_in_class_hierarchy(
    class: &Arc<RistrettoClass>,
    name: &str,
    descriptor: &str,
) -> Result<Option<(Arc<RistrettoClass>, Arc<ristretto_classloader::Method>)>> {
    if let Ok(method) = class.try_get_method(name, descriptor) {
        // Make sure it's not abstract (has code)
        if !method.access_flags().contains(MethodAccessFlags::ABSTRACT) {
            return Ok(Some((class.clone(), method)));
        }
    }

    if let Ok(Some(parent)) = class.parent()
        && let Some(result) = find_method_in_class_hierarchy(&parent, name, descriptor)?
    {
        return Ok(Some(result));
    }

    Ok(None)
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
