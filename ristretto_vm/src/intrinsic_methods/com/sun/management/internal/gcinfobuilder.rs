use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "com/sun/management/internal/GcInfoBuilder";

/// Register all intrinsic methods for `com.sun.management.internal.GcInfoBuilder`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "fillGcAttributeInfo",
        "(Ljava/lang/management/GarbageCollectorMXBean;I[Ljava/lang/String;[C[Ljava/lang/String;)V",
        fill_gc_attribute_info,
    );
    registry.register(CLASS_NAME, "getLastGcInfo0", "(Ljava/lang/management/GarbageCollectorMXBean;I[Ljava/lang/Object;[C[Ljava/lang/management/MemoryUsage;[Ljava/lang/management/MemoryUsage;)Lcom/sun/management/GcInfo;", get_last_gc_info_0);
    registry.register(
        CLASS_NAME,
        "getNumGcExtAttributes",
        "(Ljava/lang/management/GarbageCollectorMXBean;)I",
        get_num_gc_ext_attributes,
    );
}

#[async_recursion(?Send)]
async fn fill_gc_attribute_info(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "com.sun.management.internal.GcInfoBuilder.fillGcAttributeInfo(Ljava/lang/management/GarbageCollectorMXBean;I[Ljava/lang/String;[C[Ljava/lang/String;)V"
    )
}

#[async_recursion(?Send)]
async fn get_last_gc_info_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "com.sun.management.internal.GcInfoBuilder.getLastGcInfo0(Ljava/lang/management/GarbageCollectorMXBean;I[Ljava/lang/Object;[C[Ljava/lang/management/MemoryUsage;[Ljava/lang/management/MemoryUsage;)Lcom/sun/management/GcInfo;"
    )
}

#[async_recursion(?Send)]
async fn get_num_gc_ext_attributes(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "com.sun.management.internal.GcInfoBuilder.getNumGcExtAttributes(Ljava/lang/management/GarbageCollectorMXBean;)I"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.management.internal.GcInfoBuilder.fillGcAttributeInfo(Ljava/lang/management/GarbageCollectorMXBean;I[Ljava/lang/String;[C[Ljava/lang/String;)V"
    )]
    async fn test_fill_gc_attribute_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fill_gc_attribute_info(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.management.internal.GcInfoBuilder.getLastGcInfo0(Ljava/lang/management/GarbageCollectorMXBean;I[Ljava/lang/Object;[C[Ljava/lang/management/MemoryUsage;[Ljava/lang/management/MemoryUsage;)Lcom/sun/management/GcInfo;"
    )]
    async fn test_get_last_gc_info_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_last_gc_info_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.management.internal.GcInfoBuilder.getNumGcExtAttributes(Ljava/lang/management/GarbageCollectorMXBean;)I"
    )]
    async fn test_get_num_gc_ext_attributes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_num_gc_ext_attributes(thread, Parameters::default()).await;
    }
}
