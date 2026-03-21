use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::{Any, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/management/GarbageCollectorImpl.getCollectionCount()J", Any)]
#[async_method]
pub async fn get_collection_count<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.GarbageCollectorImpl.getCollectionCount()J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/GarbageCollectorImpl.getCollectionTime()J", Any)]
#[async_method]
pub async fn get_collection_time<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.GarbageCollectorImpl.getCollectionTime()J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/management/GarbageCollectorImpl.setNotificationEnabled(Lcom/sun/management/GarbageCollectorMXBean;Z)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn set_notification_enabled<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.management.GarbageCollectorImpl.setNotificationEnabled(Lcom/sun/management/GarbageCollectorMXBean;Z)V".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_collection_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_collection_count(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_collection_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_collection_time(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_notification_enabled() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = set_notification_enabled(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
