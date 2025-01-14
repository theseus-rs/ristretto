use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.lwawt.macosx.CPlatformComponent`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/CPlatformComponent";
    registry.register(
        class_name,
        "nativeCreateComponent",
        "(J)J",
        native_create_component,
    );
    registry.register(class_name, "nativeSetBounds", "(JIIII)V", native_set_bounds);
}

#[async_recursion(?Send)]
async fn native_create_component(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformComponent.nativeCreateComponent(J)J")
}

#[async_recursion(?Send)]
async fn native_set_bounds(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformComponent.nativeSetBounds(JIIII)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/lwawt/macosx/CPlatformComponent";
        assert!(registry
            .method(class_name, "nativeCreateComponent", "(J)J")
            .is_some());
        assert!(registry
            .method(class_name, "nativeSetBounds", "(JIIII)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CPlatformComponent.nativeCreateComponent(J)J")]
    async fn test_native_create_component() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_create_component(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CPlatformComponent.nativeSetBounds(JIIII)V")]
    async fn test_native_set_bounds() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_bounds(thread, Arguments::default()).await;
    }
}
