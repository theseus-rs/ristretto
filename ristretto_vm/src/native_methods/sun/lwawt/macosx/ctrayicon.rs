use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_11: Version = Version::Java11 { minor: 0 };

/// Register all native methods for `sun.lwawt.macosx.CTrayIcon`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/CTrayIcon";
    let java_version = registry.java_version().clone();

    if java_version >= JAVA_11 {
        registry.register(
            class_name,
            "nativeShowNotification",
            "(JLjava/lang/String;Ljava/lang/String;J)V",
            native_show_notification,
        );
    }

    if java_version <= JAVA_11 {
        registry.register(class_name, "setNativeImage", "(JJZ)V", set_native_image);
    } else {
        registry.register(class_name, "setNativeImage", "(JJZZ)V", set_native_image);
    }

    registry.register(class_name, "nativeCreate", "()J", native_create);
    registry.register(
        class_name,
        "nativeGetIconLocation",
        "(J)Ljava/awt/geom/Point2D;",
        native_get_icon_location,
    );
    registry.register(
        class_name,
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
