use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_8: Version = Version::Java8 { minor: 0 };
const JAVA_17: Version = Version::Java17 { minor: 0 };

/// Register all native methods for `sun.awt.CGraphicsDevice`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/awt/CGraphicsDevice";
    let java_version = registry.java_version().clone();

    if java_version <= JAVA_8 {
        registry.register(
            class_name,
            "nativeResetDisplayMode",
            "()V",
            native_reset_display_mode,
        );
    } else {
        registry.register(
            class_name,
            "nativeGetBounds",
            "(I)Ljava/awt/geom/Rectangle2D;",
            native_get_bounds,
        );
    }

    if java_version >= JAVA_17 {
        registry.register(
            class_name,
            "nativeResetDisplayMode",
            "()V",
            native_reset_display_mode,
        );
    }

    registry.register(
        class_name,
        "nativeGetDisplayMode",
        "(I)Ljava/awt/DisplayMode;",
        native_get_display_mode,
    );
    registry.register(
        class_name,
        "nativeGetDisplayModes",
        "(I)[Ljava/awt/DisplayMode;",
        native_get_display_modes,
    );
    registry.register(
        class_name,
        "nativeGetScaleFactor",
        "(I)D",
        native_get_scale_factor,
    );
    registry.register(
        class_name,
        "nativeGetScreenInsets",
        "(I)Ljava/awt/Insets;",
        native_get_screen_insets,
    );
    registry.register(
        class_name,
        "nativeGetXResolution",
        "(I)D",
        native_get_x_resolution,
    );
    registry.register(
        class_name,
        "nativeGetYResolution",
        "(I)D",
        native_get_y_resolution,
    );
    registry.register(
        class_name,
        "nativeSetDisplayMode",
        "(IIIII)V",
        native_set_display_mode,
    );
}

#[async_recursion(?Send)]
async fn native_get_bounds(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn native_get_display_mode(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn native_get_display_modes(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn native_get_scale_factor(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn native_get_screen_insets(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn native_get_x_resolution(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn native_get_y_resolution(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn native_reset_display_mode(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn native_set_display_mode(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}
