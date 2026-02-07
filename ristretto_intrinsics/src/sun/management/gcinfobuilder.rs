use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/management/GcInfoBuilder.fillGcAttributeInfo(Ljava/lang/management/GarbageCollectorMXBean;I[Ljava/lang/String;[C[Ljava/lang/String;)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn fill_gc_attribute_info<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.management.GcInfoBuilder.fillGcAttributeInfo(Ljava/lang/management/GarbageCollectorMXBean;I[Ljava/lang/String;[C[Ljava/lang/String;)V"
    )
}

#[intrinsic_method(
    "sun/management/GcInfoBuilder.getLastGcInfo0(Ljava/lang/management/GarbageCollectorMXBean;I[Ljava/lang/Object;[C[Ljava/lang/management/MemoryUsage;[Ljava/lang/management/MemoryUsage;)Lcom/sun/management/GcInfo;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_last_gc_info_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.management.GcInfoBuilder.getLastGcInfo0(Ljava/lang/management/GarbageCollectorMXBean;I[Ljava/lang/Object;[C[Ljava/lang/management/MemoryUsage;[Ljava/lang/management/MemoryUsage;)Lcom/sun/management/GcInfo;"
    )
}

#[intrinsic_method(
    "sun/management/GcInfoBuilder.getNumGcExtAttributes(Ljava/lang/management/GarbageCollectorMXBean;)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_num_gc_ext_attributes<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.management.GcInfoBuilder.getNumGcExtAttributes(Ljava/lang/management/GarbageCollectorMXBean;)I"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.GcInfoBuilder.fillGcAttributeInfo(Ljava/lang/management/GarbageCollectorMXBean;I[Ljava/lang/String;[C[Ljava/lang/String;)V"
    )]
    async fn test_fill_gc_attribute_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fill_gc_attribute_info(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.GcInfoBuilder.getLastGcInfo0(Ljava/lang/management/GarbageCollectorMXBean;I[Ljava/lang/Object;[C[Ljava/lang/management/MemoryUsage;[Ljava/lang/management/MemoryUsage;)Lcom/sun/management/GcInfo;"
    )]
    async fn test_get_last_gc_info_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_last_gc_info_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.GcInfoBuilder.getNumGcExtAttributes(Ljava/lang/management/GarbageCollectorMXBean;)I"
    )]
    async fn test_get_num_gc_ext_attributes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_num_gc_ext_attributes(thread, Parameters::default()).await;
    }
}
