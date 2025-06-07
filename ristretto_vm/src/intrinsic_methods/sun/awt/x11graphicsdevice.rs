use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/awt/X11GraphicsDevice.configDisplayMode(IIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn config_display_mode(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsDevice.configDisplayMode(IIII)V")
}

#[intrinsic_method(
    "sun/awt/X11GraphicsDevice.enterFullScreenExclusive(J)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn enter_full_screen_exclusive(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsDevice.enterFullScreenExclusive(J)V")
}

#[intrinsic_method(
    "sun/awt/X11GraphicsDevice.enumDisplayModes(ILjava/util/ArrayList;)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn enum_display_modes(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsDevice.enumDisplayModes(ILjava/util/ArrayList;)V")
}

#[intrinsic_method(
    "sun/awt/X11GraphicsDevice.exitFullScreenExclusive(J)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn exit_full_screen_exclusive(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsDevice.exitFullScreenExclusive(J)V")
}

#[intrinsic_method(
    "sun/awt/X11GraphicsDevice.getConfigColormap(II)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_config_colormap(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsDevice.getConfigColormap(II)I")
}

#[intrinsic_method(
    "sun/awt/X11GraphicsDevice.getConfigDepth(II)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_config_depth(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsDevice.getConfigDepth(II)I")
}

#[intrinsic_method(
    "sun/awt/X11GraphicsDevice.getConfigVisualId(II)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_config_visual_id(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsDevice.getConfigVisualId(II)I")
}

#[intrinsic_method(
    "sun/awt/X11GraphicsDevice.getCurrentDisplayMode(I)Ljava/awt/DisplayMode;",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_current_display_mode(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsDevice.getCurrentDisplayMode(I)Ljava/awt/DisplayMode;")
}

#[intrinsic_method("sun/awt/X11GraphicsDevice.getDisplay()J", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn get_display(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsDevice.getDisplay()J")
}

#[intrinsic_method(
    "sun/awt/X11GraphicsDevice.getDoubleBufferVisuals(I)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_double_buffer_visuals(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsDevice.getDoubleBufferVisuals(I)V")
}

#[intrinsic_method("sun/awt/X11GraphicsDevice.getNumConfigs(I)I", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn get_num_configs(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsDevice.getNumConfigs(I)I")
}

#[intrinsic_method("sun/awt/X11GraphicsDevice.initIDs()V", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn init_ids(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "sun/awt/X11GraphicsDevice.initXrandrExtension()Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn init_xrandr_extension(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsDevice.initXrandrExtension()Z")
}

#[intrinsic_method("sun/awt/X11GraphicsDevice.isDBESupported()Z", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn is_dbe_supported(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsDevice.isDBESupported()Z")
}

#[intrinsic_method(
    "sun/awt/X11GraphicsDevice.resetNativeData(I)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn reset_native_data(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
