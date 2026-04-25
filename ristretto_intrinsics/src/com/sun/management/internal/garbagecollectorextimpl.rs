use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "com/sun/management/internal/GarbageCollectorExtImpl.setNotificationEnabled(Lcom/sun/management/GarbageCollectorMXBean;Z)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_notification_enabled<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _enabled = parameters.pop_bool()?;
    let _gc = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("com.sun.management.internal.GarbageCollectorExtImpl.setNotificationEnabled(Lcom/sun/management/GarbageCollectorMXBean;Z)V".to_string()).into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_set_notification_enabled() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_notification_enabled(
            thread,
            Parameters::new(vec![Value::Object(None), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "com.sun.management.internal.GarbageCollectorExtImpl.setNotificationEnabled(Lcom/sun/management/GarbageCollectorMXBean;Z)V",
            result.unwrap_err().to_string()
        );
    }
}
