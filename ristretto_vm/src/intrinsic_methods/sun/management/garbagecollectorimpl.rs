use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::{Any, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("sun/management/GarbageCollectorImpl.getCollectionCount()J", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_collection_count(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.GarbageCollectorImpl.getCollectionCount()J")
}

#[intrinsic_method("sun/management/GarbageCollectorImpl.getCollectionTime()J", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_collection_time(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.GarbageCollectorImpl.getCollectionTime()J")
}

#[intrinsic_method(
    "sun/management/GarbageCollectorImpl.setNotificationEnabled(Lcom/sun/management/GarbageCollectorMXBean;Z)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn set_notification_enabled(
    _thread: Arc<Thread>,
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
