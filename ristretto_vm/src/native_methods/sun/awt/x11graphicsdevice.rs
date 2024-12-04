use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.awt.X11GraphicsDevice`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/awt/X11GraphicsDevice";
    registry.register(
        class_name,
        "configDisplayMode",
        "(IIII)V",
        config_display_mode,
    );
    registry.register(
        class_name,
        "enterFullScreenExclusive",
        "(J)V",
        enter_full_screen_exclusive,
    );
    registry.register(
        class_name,
        "enumDisplayModes",
        "(ILjava/util/ArrayList;)V",
        enum_display_modes,
    );
    registry.register(
        class_name,
        "exitFullScreenExclusive",
        "(J)V",
        exit_full_screen_exclusive,
    );
    registry.register(
        class_name,
        "getConfigColormap",
        "(II)I",
        get_config_colormap,
    );
    registry.register(class_name, "getConfigDepth", "(II)I", get_config_depth);
    registry.register(
        class_name,
        "getConfigVisualId",
        "(II)I",
        get_config_visual_id,
    );
    registry.register(
        class_name,
        "getCurrentDisplayMode",
        "(I)Ljava/awt/DisplayMode;",
        get_current_display_mode,
    );
    registry.register(class_name, "getDisplay", "()J", get_display);
    registry.register(
        class_name,
        "getDoubleBufferVisuals",
        "(I)V",
        get_double_buffer_visuals,
    );
    registry.register(class_name, "getNumConfigs", "(I)I", get_num_configs);
    registry.register(class_name, "initIDs", "()V", init_ids);
    registry.register(
        class_name,
        "initXrandrExtension",
        "()Z",
        init_xrandr_extension,
    );
    registry.register(class_name, "isDBESupported", "()Z", is_dbe_supported);
    registry.register(class_name, "resetNativeData", "(I)V", reset_native_data);
}

#[async_recursion(?Send)]
async fn config_display_mode(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn enter_full_screen_exclusive(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn enum_display_modes(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn exit_full_screen_exclusive(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_config_colormap(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_config_depth(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_config_visual_id(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_current_display_mode(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_display(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_double_buffer_visuals(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_num_configs(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn init_xrandr_extension(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn is_dbe_supported(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn reset_native_data(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
