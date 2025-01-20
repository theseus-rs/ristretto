use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/awt/X11GraphicsDevice";

/// Register all native methods for `sun.awt.X11GraphicsDevice`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "configDisplayMode",
        "(IIII)V",
        config_display_mode,
    );
    registry.register(
        CLASS_NAME,
        "enterFullScreenExclusive",
        "(J)V",
        enter_full_screen_exclusive,
    );
    registry.register(
        CLASS_NAME,
        "enumDisplayModes",
        "(ILjava/util/ArrayList;)V",
        enum_display_modes,
    );
    registry.register(
        CLASS_NAME,
        "exitFullScreenExclusive",
        "(J)V",
        exit_full_screen_exclusive,
    );
    registry.register(
        CLASS_NAME,
        "getConfigColormap",
        "(II)I",
        get_config_colormap,
    );
    registry.register(CLASS_NAME, "getConfigDepth", "(II)I", get_config_depth);
    registry.register(
        CLASS_NAME,
        "getConfigVisualId",
        "(II)I",
        get_config_visual_id,
    );
    registry.register(
        CLASS_NAME,
        "getCurrentDisplayMode",
        "(I)Ljava/awt/DisplayMode;",
        get_current_display_mode,
    );
    registry.register(CLASS_NAME, "getDisplay", "()J", get_display);
    registry.register(
        CLASS_NAME,
        "getDoubleBufferVisuals",
        "(I)V",
        get_double_buffer_visuals,
    );
    registry.register(CLASS_NAME, "getNumConfigs", "(I)I", get_num_configs);
    registry.register(CLASS_NAME, "initIDs", "()V", init_ids);
    registry.register(
        CLASS_NAME,
        "initXrandrExtension",
        "()Z",
        init_xrandr_extension,
    );
    registry.register(CLASS_NAME, "isDBESupported", "()Z", is_dbe_supported);
    registry.register(CLASS_NAME, "resetNativeData", "(I)V", reset_native_data);
}

#[async_recursion(?Send)]
async fn config_display_mode(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsDevice.configDisplayMode(IIII)V")
}

#[async_recursion(?Send)]
async fn enter_full_screen_exclusive(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsDevice.enterFullScreenExclusive(J)V")
}

#[async_recursion(?Send)]
async fn enum_display_modes(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsDevice.enumDisplayModes(ILjava/util/ArrayList;)V")
}

#[async_recursion(?Send)]
async fn exit_full_screen_exclusive(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsDevice.exitFullScreenExclusive(J)V")
}

#[async_recursion(?Send)]
async fn get_config_colormap(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsDevice.getConfigColormap(II)I")
}

#[async_recursion(?Send)]
async fn get_config_depth(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsDevice.getConfigDepth(II)I")
}

#[async_recursion(?Send)]
async fn get_config_visual_id(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsDevice.getConfigVisualId(II)I")
}

#[async_recursion(?Send)]
async fn get_current_display_mode(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsDevice.getCurrentDisplayMode(I)Ljava/awt/DisplayMode;")
}

#[async_recursion(?Send)]
async fn get_display(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsDevice.getDisplay()J")
}

#[async_recursion(?Send)]
async fn get_double_buffer_visuals(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsDevice.getDoubleBufferVisuals(I)V")
}

#[async_recursion(?Send)]
async fn get_num_configs(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsDevice.getNumConfigs(I)I")
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn init_xrandr_extension(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsDevice.initXrandrExtension()Z")
}

#[async_recursion(?Send)]
async fn is_dbe_supported(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsDevice.isDBESupported()Z")
}

#[async_recursion(?Send)]
async fn reset_native_data(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsDevice.resetNativeData(I)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.X11GraphicsDevice.configDisplayMode(IIII)V"
    )]
    async fn test_config_display_mode() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = config_display_mode(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.X11GraphicsDevice.enterFullScreenExclusive(J)V"
    )]
    async fn test_enter_full_screen_exclusive() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = enter_full_screen_exclusive(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.X11GraphicsDevice.enumDisplayModes(ILjava/util/ArrayList;)V"
    )]
    async fn test_enum_display_modes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = enum_display_modes(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.X11GraphicsDevice.exitFullScreenExclusive(J)V"
    )]
    async fn test_exit_full_screen_exclusive() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = exit_full_screen_exclusive(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.X11GraphicsDevice.getConfigColormap(II)I"
    )]
    async fn test_get_config_colormap() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_config_colormap(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.awt.X11GraphicsDevice.getConfigDepth(II)I")]
    async fn test_get_config_depth() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_config_depth(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.X11GraphicsDevice.getConfigVisualId(II)I"
    )]
    async fn test_get_config_visual_id() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_config_visual_id(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.X11GraphicsDevice.getCurrentDisplayMode(I)Ljava/awt/DisplayMode;"
    )]
    async fn test_get_current_display_mode() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_current_display_mode(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.awt.X11GraphicsDevice.getDisplay()J")]
    async fn test_get_display() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_display(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.X11GraphicsDevice.getDoubleBufferVisuals(I)V"
    )]
    async fn test_get_double_buffer_visuals() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_double_buffer_visuals(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.awt.X11GraphicsDevice.getNumConfigs(I)I")]
    async fn test_get_num_configs() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_num_configs(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.X11GraphicsDevice.initXrandrExtension()Z"
    )]
    async fn test_init_xrandr_extension() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_xrandr_extension(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.awt.X11GraphicsDevice.isDBESupported()Z")]
    async fn test_is_dbe_supported() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_dbe_supported(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.awt.X11GraphicsDevice.resetNativeData(I)V")]
    async fn test_reset_native_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = reset_native_data(thread, Parameters::default()).await;
    }
}
