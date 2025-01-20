use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/lwawt/macosx/CPlatformComponent";

/// Register all native methods for `sun.lwawt.macosx.CPlatformComponent`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "nativeCreateComponent",
        "(J)J",
        native_create_component,
    );
    registry.register(CLASS_NAME, "nativeSetBounds", "(JIIII)V", native_set_bounds);
}

#[async_recursion(?Send)]
async fn native_create_component(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformComponent.nativeCreateComponent(J)J")
}

#[async_recursion(?Send)]
async fn native_set_bounds(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformComponent.nativeSetBounds(JIIII)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CPlatformComponent.nativeCreateComponent(J)J"
    )]
    async fn test_native_create_component() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_create_component(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CPlatformComponent.nativeSetBounds(JIIII)V"
    )]
    async fn test_native_set_bounds() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_bounds(thread, Parameters::default()).await;
    }
}
