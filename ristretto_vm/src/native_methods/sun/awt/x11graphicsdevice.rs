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
    todo!("sun.awt.X11GraphicsDevice.configDisplayMode(IIII)V")
}

#[async_recursion(?Send)]
async fn enter_full_screen_exclusive(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsDevice.enterFullScreenExclusive(J)V")
}

#[async_recursion(?Send)]
async fn enum_display_modes(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsDevice.enumDisplayModes(ILjava/util/ArrayList;)V")
}

#[async_recursion(?Send)]
async fn exit_full_screen_exclusive(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsDevice.exitFullScreenExclusive(J)V")
}

#[async_recursion(?Send)]
async fn get_config_colormap(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsDevice.getConfigColormap(II)I")
}

#[async_recursion(?Send)]
async fn get_config_depth(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsDevice.getConfigDepth(II)I")
}

#[async_recursion(?Send)]
async fn get_config_visual_id(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsDevice.getConfigVisualId(II)I")
}

#[async_recursion(?Send)]
async fn get_current_display_mode(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsDevice.getCurrentDisplayMode(I)Ljava/awt/DisplayMode;")
}

#[async_recursion(?Send)]
async fn get_display(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsDevice.getDisplay()J")
}

#[async_recursion(?Send)]
async fn get_double_buffer_visuals(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsDevice.getDoubleBufferVisuals(I)V")
}

#[async_recursion(?Send)]
async fn get_num_configs(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsDevice.getNumConfigs(I)I")
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
    todo!("sun.awt.X11GraphicsDevice.initXrandrExtension()Z")
}

#[async_recursion(?Send)]
async fn is_dbe_supported(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsDevice.isDBESupported()Z")
}

#[async_recursion(?Send)]
async fn reset_native_data(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsDevice.resetNativeData(I)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/awt/X11GraphicsDevice";
        assert!(registry
            .method(class_name, "configDisplayMode", "(IIII)V")
            .is_some());
        assert!(registry
            .method(class_name, "enterFullScreenExclusive", "(J)V")
            .is_some());
        assert!(registry
            .method(class_name, "enumDisplayModes", "(ILjava/util/ArrayList;)V")
            .is_some());
        assert!(registry
            .method(class_name, "exitFullScreenExclusive", "(J)V")
            .is_some());
        assert!(registry
            .method(class_name, "getConfigColormap", "(II)I")
            .is_some());
        assert!(registry
            .method(class_name, "getConfigDepth", "(II)I")
            .is_some());
        assert!(registry
            .method(class_name, "getConfigVisualId", "(II)I")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "getCurrentDisplayMode",
                "(I)Ljava/awt/DisplayMode;"
            )
            .is_some());
        assert!(registry.method(class_name, "getDisplay", "()J").is_some());
        assert!(registry
            .method(class_name, "getDoubleBufferVisuals", "(I)V")
            .is_some());
        assert!(registry
            .method(class_name, "getNumConfigs", "(I)I")
            .is_some());
        assert!(registry.method(class_name, "initIDs", "()V").is_some());
        assert!(registry
            .method(class_name, "initXrandrExtension", "()Z")
            .is_some());
        assert!(registry
            .method(class_name, "isDBESupported", "()Z")
            .is_some());
        assert!(registry
            .method(class_name, "resetNativeData", "(I)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.awt.X11GraphicsDevice.configDisplayMode(IIII)V")]
    async fn test_config_display_mode() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = config_display_mode(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.awt.X11GraphicsDevice.enterFullScreenExclusive(J)V")]
    async fn test_enter_full_screen_exclusive() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = enter_full_screen_exclusive(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.awt.X11GraphicsDevice.enumDisplayModes(ILjava/util/ArrayList;)V"
    )]
    async fn test_enum_display_modes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = enum_display_modes(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.awt.X11GraphicsDevice.exitFullScreenExclusive(J)V")]
    async fn test_exit_full_screen_exclusive() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = exit_full_screen_exclusive(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.awt.X11GraphicsDevice.getConfigColormap(II)I")]
    async fn test_get_config_colormap() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_config_colormap(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.awt.X11GraphicsDevice.getConfigDepth(II)I")]
    async fn test_get_config_depth() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_config_depth(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.awt.X11GraphicsDevice.getConfigVisualId(II)I")]
    async fn test_get_config_visual_id() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_config_visual_id(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.awt.X11GraphicsDevice.getCurrentDisplayMode(I)Ljava/awt/DisplayMode;"
    )]
    async fn test_get_current_display_mode() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_current_display_mode(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.awt.X11GraphicsDevice.getDisplay()J")]
    async fn test_get_display() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_display(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.awt.X11GraphicsDevice.getDoubleBufferVisuals(I)V")]
    async fn test_get_double_buffer_visuals() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_double_buffer_visuals(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.awt.X11GraphicsDevice.getNumConfigs(I)I")]
    async fn test_get_num_configs() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_num_configs(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Arguments::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(expected = "sun.awt.X11GraphicsDevice.initXrandrExtension()Z")]
    async fn test_init_xrandr_extension() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_xrandr_extension(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.awt.X11GraphicsDevice.isDBESupported()Z")]
    async fn test_is_dbe_supported() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_dbe_supported(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.awt.X11GraphicsDevice.resetNativeData(I)V")]
    async fn test_reset_native_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = reset_native_data(thread, Arguments::default()).await;
    }
}
