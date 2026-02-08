use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::{Any, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/management/GarbageCollectorImpl.getCollectionCount()J", Any)]
#[async_method]
pub async fn get_collection_count<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.GarbageCollectorImpl.getCollectionCount()J")
}

#[intrinsic_method("sun/management/GarbageCollectorImpl.getCollectionTime()J", Any)]
#[async_method]
pub async fn get_collection_time<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.GarbageCollectorImpl.getCollectionTime()J")
}

#[intrinsic_method(
    "sun/management/GarbageCollectorImpl.setNotificationEnabled(Lcom/sun/management/GarbageCollectorMXBean;Z)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn set_notification_enabled<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.management.GarbageCollectorImpl.setNotificationEnabled(Lcom/sun/management/GarbageCollectorMXBean;Z)V"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.GarbageCollectorImpl.getCollectionCount()J"
    )]
    async fn test_get_collection_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_collection_count(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.GarbageCollectorImpl.getCollectionTime()J"
    )]
    async fn test_get_collection_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_collection_time(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.GarbageCollectorImpl.setNotificationEnabled(Lcom/sun/management/GarbageCollectorMXBean;Z)V"
    )]
    async fn test_set_notification_enabled() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_notification_enabled(thread, Parameters::default()).await;
    }
}
