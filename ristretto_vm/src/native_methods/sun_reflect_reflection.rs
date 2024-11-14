use crate::native_methods::jdk_internal_reflect_reflection;
use crate::native_methods::registry::MethodRegistry;

/// Register all native methods for sun.reflect.Reflection.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/reflect/Reflection";
    registry.register(
        class_name,
        "getCallerClass",
        "()Ljava/lang/Class;",
        jdk_internal_reflect_reflection::get_caller_class,
    );
}
