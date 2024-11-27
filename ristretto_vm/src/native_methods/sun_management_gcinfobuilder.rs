use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.management.GcInfoBuilder`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/management/GcInfoBuilder";
    registry.register(
        class_name,
        "fillGcAttributeInfo",
        "(Ljava/lang/management/GarbageCollectorMXBean;I[Ljava/lang/String;[C[Ljava/lang/String;)V",
        fill_gc_attribute_info,
    );
    registry.register(class_name, "getLastGcInfo0", "(Ljava/lang/management/GarbageCollectorMXBean;I[Ljava/lang/Object;[C[Ljava/lang/management/MemoryUsage;[Ljava/lang/management/MemoryUsage;)Lcom/sun/management/GcInfo;", get_last_gc_info_0);
    registry.register(
        class_name,
        "getNumGcExtAttributes",
        "(Ljava/lang/management/GarbageCollectorMXBean;)I",
        get_num_gc_ext_attributes,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn fill_gc_attribute_info(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_last_gc_info_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_num_gc_ext_attributes(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}
