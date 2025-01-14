use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `com.sun.management.internal.GcInfoBuilder`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "com/sun/management/internal/GcInfoBuilder";
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

#[async_recursion(?Send)]
async fn fill_gc_attribute_info(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.sun.management.internal.GcInfoBuilder.fillGcAttributeInfo(Ljava/lang/management/GarbageCollectorMXBean;I[Ljava/lang/String;[C[Ljava/lang/String;)V")
}

#[async_recursion(?Send)]
async fn get_last_gc_info_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.management.internal.GcInfoBuilder.getLastGcInfo0(Ljava/lang/management/GarbageCollectorMXBean;I[Ljava/lang/Object;[C[Ljava/lang/management/MemoryUsage;[Ljava/lang/management/MemoryUsage;)Lcom/sun/management/GcInfo;")
}

#[async_recursion(?Send)]
async fn get_num_gc_ext_attributes(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.sun.management.internal.GcInfoBuilder.getNumGcExtAttributes(Ljava/lang/management/GarbageCollectorMXBean;)I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "com/sun/management/internal/GcInfoBuilder";
        assert!(registry
            .method(
                class_name,
                "fillGcAttributeInfo",
                "(Ljava/lang/management/GarbageCollectorMXBean;I[Ljava/lang/String;[C[Ljava/lang/String;)V"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "getLastGcInfo0",
                "(Ljava/lang/management/GarbageCollectorMXBean;I[Ljava/lang/Object;[C[Ljava/lang/management/MemoryUsage;[Ljava/lang/management/MemoryUsage;)Lcom/sun/management/GcInfo;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "getNumGcExtAttributes",
                "(Ljava/lang/management/GarbageCollectorMXBean;)I"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.management.internal.GcInfoBuilder.fillGcAttributeInfo(Ljava/lang/management/GarbageCollectorMXBean;I[Ljava/lang/String;[C[Ljava/lang/String;)V"
    )]
    async fn test_fill_gc_attribute_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fill_gc_attribute_info(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.management.internal.GcInfoBuilder.getLastGcInfo0(Ljava/lang/management/GarbageCollectorMXBean;I[Ljava/lang/Object;[C[Ljava/lang/management/MemoryUsage;[Ljava/lang/management/MemoryUsage;)Lcom/sun/management/GcInfo;"
    )]
    async fn test_get_last_gc_info_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_last_gc_info_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.management.internal.GcInfoBuilder.getNumGcExtAttributes(Ljava/lang/management/GarbageCollectorMXBean;)I"
    )]
    async fn test_get_num_gc_ext_attributes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_num_gc_ext_attributes(thread, Arguments::default()).await;
    }
}
