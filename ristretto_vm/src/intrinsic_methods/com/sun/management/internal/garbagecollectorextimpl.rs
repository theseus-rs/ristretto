use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "com/sun/management/internal/GarbageCollectorExtImpl";

/// Register all intrinsic methods for `com.sun.management.internal.GarbageCollectorExtImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "setNotificationEnabled",
        "(Lcom/sun/management/GarbageCollectorMXBean;Z)V",
        set_notification_enabled,
    );
}

#[async_recursion(?Send)]
async fn set_notification_enabled(
    _thread: Arc<Thread>,
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
