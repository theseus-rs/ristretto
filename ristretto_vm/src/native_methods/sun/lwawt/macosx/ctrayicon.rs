use crate::arguments::Arguments;
use crate::native_methods::registry::{MethodRegistry, JAVA_11};
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/lwawt/macosx/CTrayIcon";

/// Register all native methods for `sun.lwawt.macosx.CTrayIcon`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() >= JAVA_11 {
        registry.register(
            CLASS_NAME,
            "nativeShowNotification",
            "(JLjava/lang/String;Ljava/lang/String;J)V",
            native_show_notification,
        );
    }

    if registry.java_major_version() <= JAVA_11 {
        registry.register(CLASS_NAME, "setNativeImage", "(JJZ)V", set_native_image);
    } else {
        registry.register(CLASS_NAME, "setNativeImage", "(JJZZ)V", set_native_image);
    }

    registry.register(CLASS_NAME, "nativeCreate", "()J", native_create);
    registry.register(
        CLASS_NAME,
        "nativeGetIconLocation",
        "(J)Ljava/awt/geom/Point2D;",
        native_get_icon_location,
    );
    registry.register(
        CLASS_NAME,
        "nativeSetToolTip",
        "(JLjava/lang/String;)V",
        native_set_tool_tip,
    );
}

#[async_recursion(?Send)]
async fn native_create(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CTrayIcon.nativeCreate()J")
}

#[async_recursion(?Send)]
async fn native_get_icon_location(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CTrayIcon.nativeGetIconLocation(J)Ljava/awt/geom/Point2D;")
}

#[async_recursion(?Send)]
async fn native_set_tool_tip(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CTrayIcon.nativeSetToolTip(JLjava/lang/String;)V")
}

#[async_recursion(?Send)]
async fn native_show_notification(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CTrayIcon.nativeShowNotification(JLjava/lang/String;Ljava/lang/String;J)V")
}

#[async_recursion(?Send)]
async fn set_native_image(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CTrayIcon.setNativeImage(JJZ)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.CTrayIcon.nativeCreate()J")]
    async fn test_native_create() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_create(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CTrayIcon.nativeGetIconLocation(J)Ljava/awt/geom/Point2D;"
    )]
    async fn test_native_get_icon_location() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_icon_location(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CTrayIcon.nativeSetToolTip(JLjava/lang/String;)V"
    )]
    async fn test_native_set_tool_tip() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_tool_tip(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CTrayIcon.nativeShowNotification(JLjava/lang/String;Ljava/lang/String;J)V"
    )]
    async fn test_native_show_notification() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_show_notification(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CTrayIcon.setNativeImage(JJZ)V"
    )]
    async fn test_set_native_image() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_native_image(thread, Arguments::default()).await;
    }
}
