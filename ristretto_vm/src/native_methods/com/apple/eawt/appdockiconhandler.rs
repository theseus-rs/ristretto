use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_11: Version = Version::Java11 { minor: 0 };

/// Register all native methods for `com.apple.eawt._AppDockIconHandler`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "com/apple/eawt/_AppDockIconHandler";
    let java_version = registry.java_version();

    if java_version >= &JAVA_11 {
        registry.register(
            class_name,
            "nativeSetDockIconProgress",
            "(I)V",
            native_set_dock_icon_progress,
        );
    }

    registry.register(
        class_name,
        "nativeGetDockIconImage",
        "()J",
        native_get_dock_icon_image,
    );
    registry.register(
        class_name,
        "nativeSetDockIconBadge",
        "(Ljava/lang/String;)V",
        native_set_dock_icon_badge,
    );
    registry.register(
        class_name,
        "nativeSetDockIconImage",
        "(J)V",
        native_set_dock_icon_image,
    );
    registry.register(
        class_name,
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

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "com/apple/eawt/_AppDockIconHandler";
        assert!(registry
            .method(class_name, "nativeGetDockIconImage", "()J")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "nativeSetDockIconBadge",
                "(Ljava/lang/String;)V"
            )
            .is_some());
        assert!(registry
            .method(class_name, "nativeSetDockIconImage", "(J)V")
            .is_some());
        assert!(registry
            .method(class_name, "nativeSetDockMenu", "(J)V")
            .is_some());
    }

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
