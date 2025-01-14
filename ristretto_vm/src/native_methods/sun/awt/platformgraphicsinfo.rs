use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.awt.PlatformGraphicsInfo`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/awt/PlatformGraphicsInfo";
    registry.register(class_name, "isInAquaSession", "()Z", is_in_aqua_session);
}

#[async_recursion(?Send)]
async fn is_in_aqua_session(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.PlatformGraphicsInfo.isInAquaSession()Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/awt/PlatformGraphicsInfo";
        assert!(registry
            .method(class_name, "isInAquaSession", "()Z")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.awt.PlatformGraphicsInfo.isInAquaSession()Z")]
    async fn test_is_in_aqua_session() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_in_aqua_session(thread, Arguments::default()).await;
    }
}
