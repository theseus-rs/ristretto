use crate::native_methods::jdk_internal_misc_vm;
use crate::native_methods::registry::MethodRegistry;

/// Register all native methods for sun.misc.VM.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/misc/VM";
    registry.register(
        class_name,
        "initialize",
        "()V",
        jdk_internal_misc_vm::initialize,
    );
}
