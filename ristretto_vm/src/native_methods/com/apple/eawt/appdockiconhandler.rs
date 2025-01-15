use crate::arguments::Arguments;
use crate::native_methods::registry::{MethodRegistry, JAVA_11};
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "com/apple/eawt/_AppDockIconHandler";

/// Register all native methods for `com.apple.eawt._AppDockIconHandler`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() >= JAVA_11 {
        registry.register(
            CLASS_NAME,
            "nativeSetDockIconProgress",
            "(I)V",
            native_set_dock_icon_progress,
        );
    }

    registry.register(
        CLASS_NAME,
        "nativeGetDockIconImage",
        "()J",
        native_get_dock_icon_image,
    );
    registry.register(
        CLASS_NAME,
        "nativeSetDockIconBadge",
        "(Ljava/lang/String;)V",
        native_set_dock_icon_badge,
    );
    registry.register(
        CLASS_NAME,
        "nativeSetDockIconImage",
        "(J)V",
        native_set_dock_icon_image,
    );
    registry.register(
        CLASS_NAME,
        "nativeSetDockMenu",
        "(J)V",
        native_set_dock_menu,
    );
}

#[async_recursion(?Send)]
async fn native_get_dock_icon_image(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.apple.eawt._AppDockIconHandler.nativeGetDockIconImage()J")
}

#[async_recursion(?Send)]
async fn native_set_dock_icon_badge(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.apple.eawt._AppDockIconHandler.nativeSetDockIconBadge(Ljava/lang/String;)V")
}

#[async_recursion(?Send)]
async fn native_set_dock_icon_image(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.apple.eawt._AppDockIconHandler.nativeSetDockIconImage(J)V")
}

#[async_recursion(?Send)]
async fn native_set_dock_icon_progress(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.apple.eawt._AppDockIconHandler.nativeSetDockIconProgress(I)V")
}

#[async_recursion(?Send)]
async fn native_set_dock_menu(
    _thread: Arc<Thread>,
    _arguments: Arguments,
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
        let _ = native_get_dock_icon_image(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.eawt._AppDockIconHandler.nativeSetDockIconBadge(Ljava/lang/String;)V"
    )]
    async fn test_native_set_dock_icon_badge() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_dock_icon_badge(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.eawt._AppDockIconHandler.nativeSetDockIconImage(J)V"
    )]
    async fn test_native_set_dock_icon_image() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_dock_icon_image(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.eawt._AppDockIconHandler.nativeSetDockIconProgress(I)V"
    )]
    async fn test_native_set_dock_icon_progress() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_dock_icon_progress(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.eawt._AppDockIconHandler.nativeSetDockMenu(J)V"
    )]
    async fn test_native_set_dock_menu() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_dock_menu(thread, Arguments::default()).await;
    }
}
