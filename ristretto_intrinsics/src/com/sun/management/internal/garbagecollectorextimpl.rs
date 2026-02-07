use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "com/sun/management/internal/GarbageCollectorExtImpl.setNotificationEnabled(Lcom/sun/management/GarbageCollectorMXBean;Z)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_notification_enabled<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "com.sun.management.internal.GarbageCollectorExtImpl.setNotificationEnabled(Lcom/sun/management/GarbageCollectorMXBean;Z)V"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.management.internal.GarbageCollectorExtImpl.setNotificationEnabled(Lcom/sun/management/GarbageCollectorMXBean;Z)V"
    )]
    async fn test_set_notification_enabled() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_notification_enabled(thread, Parameters::default()).await;
    }
}
