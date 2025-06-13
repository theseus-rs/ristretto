use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("sun/lwawt/macosx/CPlatformComponent.nativeCreateComponent(J)J", Any)]
#[async_recursion(?Send)]
pub(crate) async fn native_create_component(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformComponent.nativeCreateComponent(J)J")
}

#[intrinsic_method("sun/lwawt/macosx/CPlatformComponent.nativeSetBounds(JIIII)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn native_set_bounds(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
