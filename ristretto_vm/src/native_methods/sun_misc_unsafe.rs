use crate::native_methods::jdk_internal_misc_unsafe;
use crate::native_methods::registry::MethodRegistry;

/// Register all native methods for sun.misc.Unsafe.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/misc/Unsafe";
    registry.register(
        class_name,
        "addressSize",
        "()I",
        jdk_internal_misc_unsafe::address_size_0,
    );
    registry.register(
        class_name,
        "arrayBaseOffset",
        "(Ljava/lang/Class;)I",
        jdk_internal_misc_unsafe::array_base_offset_0,
    );
    registry.register(
        class_name,
        "arrayIndexScale",
        "(Ljava/lang/Class;)I",
        jdk_internal_misc_unsafe::array_index_scale_0,
    );
    registry.register(
        class_name,
        "registerNatives",
        "()V",
        jdk_internal_misc_unsafe::register_natives,
    );
}
