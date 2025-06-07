use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("com/apple/eawt/_AppDockIconHandler.nativeGetDockIconImage()J", Any)]
#[async_recursion(?Send)]
pub(crate) async fn native_get_dock_icon_image(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.eawt._AppDockIconHandler.nativeGetDockIconImage()J")
}

#[intrinsic_method(
    "com/apple/eawt/_AppDockIconHandler.nativeSetDockIconBadge(Ljava/lang/String;)V",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn native_set_dock_icon_badge(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.eawt._AppDockIconHandler.nativeSetDockIconBadge(Ljava/lang/String;)V")
}

#[intrinsic_method("com/apple/eawt/_AppDockIconHandler.nativeSetDockIconImage(J)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn native_set_dock_icon_image(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.eawt._AppDockIconHandler.nativeSetDockIconImage(J)V")
}

#[intrinsic_method(
    "com/apple/eawt/_AppDockIconHandler.nativeSetDockIconProgress(I)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn native_set_dock_icon_progress(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.eawt._AppDockIconHandler.nativeSetDockIconProgress(I)V")
}

#[intrinsic_method("com/apple/eawt/_AppDockIconHandler.nativeSetDockMenu(J)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn native_set_dock_menu(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.eawt._AppDockIconHandler.nativeSetDockMenu(J)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.eawt._AppDockIconHandler.nativeGetDockIconImage()J"
    )]
    async fn test_native_get_dock_icon_image() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_dock_icon_image(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.eawt._AppDockIconHandler.nativeSetDockIconBadge(Ljava/lang/String;)V"
    )]
    async fn test_native_set_dock_icon_badge() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_dock_icon_badge(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.eawt._AppDockIconHandler.nativeSetDockIconImage(J)V"
    )]
    async fn test_native_set_dock_icon_image() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_dock_icon_image(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.eawt._AppDockIconHandler.nativeSetDockIconProgress(I)V"
    )]
    async fn test_native_set_dock_icon_progress() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_dock_icon_progress(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.eawt._AppDockIconHandler.nativeSetDockMenu(J)V"
    )]
    async fn test_native_set_dock_menu() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_dock_menu(thread, Parameters::default()).await;
    }
}
