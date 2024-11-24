use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.awt.CGraphicsDevice`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/awt/CGraphicsDevice";
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
        "nativeResetDisplayMode",
        "()V",
        native_reset_display_mode,
    );
    registry.register(
        class_name,
        "nativeSetDisplayMode",
        "(IIIII)V",
        native_set_display_mode,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_get_display_mode(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_get_display_modes(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_get_scale_factor(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_get_screen_insets(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_get_x_resolution(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_get_y_resolution(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_reset_display_mode(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_set_display_mode(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}
