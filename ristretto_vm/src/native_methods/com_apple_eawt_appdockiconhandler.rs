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
    todo!()
}

#[async_recursion(?Send)]
async fn native_set_dock_icon_badge(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn native_set_dock_icon_image(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn native_set_dock_icon_progress(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn native_set_dock_menu(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}
