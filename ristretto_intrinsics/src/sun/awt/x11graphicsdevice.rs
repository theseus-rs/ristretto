use ristretto_classfile::JAVA_8;
#[cfg(target_os = "linux")]
use ristretto_classfile::JAVA_11;
#[cfg(target_os = "linux")]
use ristretto_classfile::JAVA_17;
#[cfg(target_os = "linux")]
use ristretto_classfile::VersionSpecification::Equal;
#[cfg(target_os = "linux")]
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
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
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _display_mode = parameters.pop_int()?;
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _screen = parameters.pop_int()?;
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
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _window = parameters.pop_long()?;
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
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _modes = parameters.pop_reference()?;
    let _screen = parameters.pop_int()?;
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
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _window = parameters.pop_long()?;
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
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen = parameters.pop_int()?;
    let _index = parameters.pop_int()?;
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
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen = parameters.pop_int()?;
    let _index = parameters.pop_int()?;
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
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen = parameters.pop_int()?;
    let _index = parameters.pop_int()?;
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
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen = parameters.pop_int()?;
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
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.X11GraphicsDevice.getDoubleBufferVisuals(I)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/awt/X11GraphicsDevice.getNumConfigs(I)I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn get_num_configs<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen = parameters.pop_int()?;
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
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.X11GraphicsDevice.resetNativeData(I)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/awt/X11GraphicsDevice.getNativeScaleFactor(I)D",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_native_scale_factor<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11GraphicsDevice.getNativeScaleFactor(I)D".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/awt/X11GraphicsDevice.initXrandrExtension(Z)Z",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn init_xrandr_extension_linux_ge_v17_v1<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _use_old_config_display_mode = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11GraphicsDevice.initXrandrExtension(Z)Z".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/awt/X11GraphicsDevice.pGetBounds(I)Ljava/awt/Rectangle;",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn p_get_bounds<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen_num = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11GraphicsDevice.pGetBounds(I)Ljava/awt/Rectangle;".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/awt/X11GraphicsDevice.configDisplayMode(IIII)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn config_display_mode_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _display_mode = parameters.pop_int()?;
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _screen = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11GraphicsDevice.configDisplayMode(IIII)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/awt/X11GraphicsDevice.enterFullScreenExclusive(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn enter_full_screen_exclusive_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _window = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11GraphicsDevice.enterFullScreenExclusive(J)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/awt/X11GraphicsDevice.enumDisplayModes(ILjava/util/ArrayList;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn enum_display_modes_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _modes = parameters.pop_reference()?;
    let _screen = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11GraphicsDevice.enumDisplayModes(ILjava/util/ArrayList;)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/awt/X11GraphicsDevice.exitFullScreenExclusive(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn exit_full_screen_exclusive_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _window = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11GraphicsDevice.exitFullScreenExclusive(J)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/awt/X11GraphicsDevice.getConfigColormap(II)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_config_colormap_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen = parameters.pop_int()?;
    let _index = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11GraphicsDevice.getConfigColormap(II)I".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/awt/X11GraphicsDevice.getConfigDepth(II)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_config_depth_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen = parameters.pop_int()?;
    let _index = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11GraphicsDevice.getConfigDepth(II)I".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/awt/X11GraphicsDevice.getConfigVisualId(II)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_config_visual_id_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen = parameters.pop_int()?;
    let _index = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11GraphicsDevice.getConfigVisualId(II)I".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/awt/X11GraphicsDevice.getCurrentDisplayMode(I)Ljava/awt/DisplayMode;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_current_display_mode_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11GraphicsDevice.getCurrentDisplayMode(I)Ljava/awt/DisplayMode;".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method("sun/awt/X11GraphicsDevice.getDisplay()J", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn get_display_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11GraphicsDevice.getDisplay()J".to_string())
            .into(),
    )
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/awt/X11GraphicsDevice.getDoubleBufferVisuals(I)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_double_buffer_visuals_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11GraphicsDevice.getDoubleBufferVisuals(I)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/awt/X11GraphicsDevice.getNativeScaleFactor(I)D",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_native_scale_factor_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11GraphicsDevice.getNativeScaleFactor(I)D".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/awt/X11GraphicsDevice.getNumConfigs(I)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_num_configs_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11GraphicsDevice.getNumConfigs(I)I".to_string())
            .into(),
    )
}
#[cfg(target_os = "linux")]
#[intrinsic_method("sun/awt/X11GraphicsDevice.initIDs()V", Equal(JAVA_11))]
#[async_method]
pub async fn init_ids_linux_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun/awt/X11GraphicsDevice.initIDs()V".to_string()).into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method("sun/awt/X11GraphicsDevice.initXrandrExtension()Z", Equal(JAVA_11))]
#[async_method]
pub async fn init_xrandr_extension_linux_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11GraphicsDevice.initXrandrExtension()Z".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/awt/X11GraphicsDevice.initXrandrExtension(Z)Z",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn init_xrandr_extension_linux_ge_v17_v2<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _use_old_config_display_mode = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11GraphicsDevice.initXrandrExtension(Z)Z".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/awt/X11GraphicsDevice.isDBESupported()Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn is_dbesupported_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11GraphicsDevice.isDBESupported()Z".to_string())
            .into(),
    )
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/awt/X11GraphicsDevice.pGetBounds(I)Ljava/awt/Rectangle;",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn p_get_bounds_linux_ge_v17<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen_num = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11GraphicsDevice.pGetBounds(I)Ljava/awt/Rectangle;".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method("sun/awt/X11GraphicsDevice.resetNativeData(I)V", Equal(JAVA_11))]
#[async_method]
pub async fn reset_native_data_linux_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11GraphicsDevice.resetNativeData(I)V".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_config_display_mode() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = config_display_mode(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.awt.X11GraphicsDevice.configDisplayMode(IIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_enter_full_screen_exclusive() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result =
            enter_full_screen_exclusive(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.awt.X11GraphicsDevice.enterFullScreenExclusive(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_enum_display_modes() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = enum_display_modes(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.awt.X11GraphicsDevice.enumDisplayModes(ILjava/util/ArrayList;)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_exit_full_screen_exclusive() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result =
            exit_full_screen_exclusive(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.awt.X11GraphicsDevice.exitFullScreenExclusive(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_config_colormap() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result =
            get_config_colormap(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)])).await;
        assert_eq!(
            "sun.awt.X11GraphicsDevice.getConfigColormap(II)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_config_depth() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result =
            get_config_depth(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)])).await;
        assert_eq!(
            "sun.awt.X11GraphicsDevice.getConfigDepth(II)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_config_visual_id() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result =
            get_config_visual_id(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)])).await;
        assert_eq!(
            "sun.awt.X11GraphicsDevice.getConfigVisualId(II)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_current_display_mode() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = get_current_display_mode(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun.awt.X11GraphicsDevice.getCurrentDisplayMode(I)Ljava/awt/DisplayMode;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_display() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = get_display(thread, Parameters::default()).await;
        assert_eq!(
            "sun.awt.X11GraphicsDevice.getDisplay()J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_double_buffer_visuals() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = get_double_buffer_visuals(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun.awt.X11GraphicsDevice.getDoubleBufferVisuals(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_num_configs() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = get_num_configs(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun.awt.X11GraphicsDevice.getNumConfigs(I)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_init_xrandr_extension() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = init_xrandr_extension(thread, Parameters::default()).await;
        assert_eq!(
            "sun.awt.X11GraphicsDevice.initXrandrExtension()Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_is_dbe_supported() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = is_dbe_supported(thread, Parameters::default()).await;
        assert_eq!(
            "sun.awt.X11GraphicsDevice.isDBESupported()Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_reset_native_data() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = reset_native_data(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun.awt.X11GraphicsDevice.resetNativeData(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_native_scale_factor() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_native_scale_factor(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/X11GraphicsDevice.getNativeScaleFactor(I)D",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_init_xrandr_extension_linux_ge_v17_v1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_xrandr_extension_linux_ge_v17_v1(
            thread,
            Parameters::new(vec![Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11GraphicsDevice.initXrandrExtension(Z)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_p_get_bounds() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = p_get_bounds(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/X11GraphicsDevice.pGetBounds(I)Ljava/awt/Rectangle;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_config_display_mode_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = config_display_mode_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11GraphicsDevice.configDisplayMode(IIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_enter_full_screen_exclusive_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            enter_full_screen_exclusive_linux_ge_v11(thread, Parameters::new(vec![Value::Long(0)]))
                .await;
        assert_eq!(
            "sun/awt/X11GraphicsDevice.enterFullScreenExclusive(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_enum_display_modes_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = enum_display_modes_linux_ge_v11(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11GraphicsDevice.enumDisplayModes(ILjava/util/ArrayList;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_exit_full_screen_exclusive_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            exit_full_screen_exclusive_linux_ge_v11(thread, Parameters::new(vec![Value::Long(0)]))
                .await;
        assert_eq!(
            "sun/awt/X11GraphicsDevice.exitFullScreenExclusive(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_config_colormap_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_config_colormap_linux_ge_v11(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11GraphicsDevice.getConfigColormap(II)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_config_depth_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_config_depth_linux_ge_v11(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11GraphicsDevice.getConfigDepth(II)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_config_visual_id_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_config_visual_id_linux_ge_v11(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11GraphicsDevice.getConfigVisualId(II)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_current_display_mode_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            get_current_display_mode_linux_ge_v11(thread, Parameters::new(vec![Value::Int(0)]))
                .await;
        assert_eq!(
            "sun/awt/X11GraphicsDevice.getCurrentDisplayMode(I)Ljava/awt/DisplayMode;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_display_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_display_linux_ge_v11(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/X11GraphicsDevice.getDisplay()J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_double_buffer_visuals_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            get_double_buffer_visuals_linux_ge_v11(thread, Parameters::new(vec![Value::Int(0)]))
                .await;
        assert_eq!(
            "sun/awt/X11GraphicsDevice.getDoubleBufferVisuals(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_native_scale_factor_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            get_native_scale_factor_linux_ge_v11(thread, Parameters::new(vec![Value::Int(0)]))
                .await;
        assert_eq!(
            "sun/awt/X11GraphicsDevice.getNativeScaleFactor(I)D",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_num_configs_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            get_num_configs_linux_ge_v11(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/X11GraphicsDevice.getNumConfigs(I)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_init_ids_linux_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids_linux_v11(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/X11GraphicsDevice.initIDs()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_init_xrandr_extension_linux_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_xrandr_extension_linux_v11(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/X11GraphicsDevice.initXrandrExtension()Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_init_xrandr_extension_linux_ge_v17_v2() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_xrandr_extension_linux_ge_v17_v2(
            thread,
            Parameters::new(vec![Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11GraphicsDevice.initXrandrExtension(Z)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_is_dbesupported_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_dbesupported_linux_ge_v11(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/X11GraphicsDevice.isDBESupported()Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_p_get_bounds_linux_ge_v17() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = p_get_bounds_linux_ge_v17(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/X11GraphicsDevice.pGetBounds(I)Ljava/awt/Rectangle;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_reset_native_data_linux_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            reset_native_data_linux_v11(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/X11GraphicsDevice.resetNativeData(I)V",
            result.unwrap_err().to_string()
        );
    }
}
