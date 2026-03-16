use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/awt/X11GraphicsDevice.configDisplayMode(IIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn config_display_mode<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.X11GraphicsDevice.configDisplayMode(IIII)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/awt/X11GraphicsDevice.enterFullScreenExclusive(J)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn enter_full_screen_exclusive<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.X11GraphicsDevice.enterFullScreenExclusive(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/awt/X11GraphicsDevice.enumDisplayModes(ILjava/util/ArrayList;)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn enum_display_modes<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.X11GraphicsDevice.enumDisplayModes(ILjava/util/ArrayList;)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/awt/X11GraphicsDevice.exitFullScreenExclusive(J)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn exit_full_screen_exclusive<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.X11GraphicsDevice.exitFullScreenExclusive(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/awt/X11GraphicsDevice.getConfigColormap(II)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_config_colormap<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.X11GraphicsDevice.getConfigColormap(II)I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/awt/X11GraphicsDevice.getConfigDepth(II)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_config_depth<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.X11GraphicsDevice.getConfigDepth(II)I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/awt/X11GraphicsDevice.getConfigVisualId(II)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_config_visual_id<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.X11GraphicsDevice.getConfigVisualId(II)I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/awt/X11GraphicsDevice.getCurrentDisplayMode(I)Ljava/awt/DisplayMode;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_current_display_mode<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.X11GraphicsDevice.getCurrentDisplayMode(I)Ljava/awt/DisplayMode;".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/awt/X11GraphicsDevice.getDisplay()J", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn get_display<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.awt.X11GraphicsDevice.getDisplay()J".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "sun/awt/X11GraphicsDevice.getDoubleBufferVisuals(I)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_double_buffer_visuals<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.X11GraphicsDevice.getDoubleBufferVisuals(I)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/awt/X11GraphicsDevice.getNumConfigs(I)I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn get_num_configs<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.awt.X11GraphicsDevice.getNumConfigs(I)I".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/awt/X11GraphicsDevice.initIDs()V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "sun/awt/X11GraphicsDevice.initXrandrExtension()Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn init_xrandr_extension<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.X11GraphicsDevice.initXrandrExtension()Z".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/awt/X11GraphicsDevice.isDBESupported()Z", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn is_dbe_supported<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.awt.X11GraphicsDevice.isDBESupported()Z".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "sun/awt/X11GraphicsDevice.resetNativeData(I)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn reset_native_data<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.X11GraphicsDevice.resetNativeData(I)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_config_display_mode() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = config_display_mode(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_enter_full_screen_exclusive() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = enter_full_screen_exclusive(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_enum_display_modes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = enum_display_modes(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_exit_full_screen_exclusive() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = exit_full_screen_exclusive(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_config_colormap() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_config_colormap(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_config_depth() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_config_depth(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_config_visual_id() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_config_visual_id(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_current_display_mode() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_current_display_mode(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_display() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_display(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_double_buffer_visuals() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_double_buffer_visuals(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_num_configs() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_num_configs(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_init_xrandr_extension() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_xrandr_extension(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_is_dbe_supported() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_dbe_supported(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_reset_native_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = reset_native_data(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
