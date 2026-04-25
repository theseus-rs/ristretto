use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/awt/Win32GraphicsDevice.configDisplayMode(ILjava/awt/peer/WindowPeer;IIII)V",
    Any
)]
#[async_method]
pub async fn config_display_mode<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _refresh_rate = parameters.pop_int()?;
    let _bit_depth = parameters.pop_int()?;
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _window_peer = parameters.pop_reference()?;
    let _screen = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/Win32GraphicsDevice.configDisplayMode(ILjava/awt/peer/WindowPeer;IIII)V"
            .to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/Win32GraphicsDevice.enterFullScreenExclusive(ILjava/awt/peer/WindowPeer;)V",
    Any
)]
#[async_method]
pub async fn enter_full_screen_exclusive<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _window_peer = parameters.pop_reference()?;
    let _screen = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/Win32GraphicsDevice.enterFullScreenExclusive(ILjava/awt/peer/WindowPeer;)V"
            .to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/Win32GraphicsDevice.enumDisplayModes(ILjava/util/ArrayList;)V",
    Any
)]
#[async_method]
pub async fn enum_display_modes<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _array_list = parameters.pop_reference()?;
    let _screen = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/Win32GraphicsDevice.enumDisplayModes(ILjava/util/ArrayList;)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/Win32GraphicsDevice.exitFullScreenExclusive(ILjava/awt/peer/WindowPeer;)V",
    Any
)]
#[async_method]
pub async fn exit_full_screen_exclusive<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _window_peer = parameters.pop_reference()?;
    let _screen = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/Win32GraphicsDevice.exitFullScreenExclusive(ILjava/awt/peer/WindowPeer;)V"
            .to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/Win32GraphicsDevice.getCurrentDisplayMode(I)Ljava/awt/DisplayMode;",
    Any
)]
#[async_method]
pub async fn get_current_display_mode<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/Win32GraphicsDevice.getCurrentDisplayMode(I)Ljava/awt/DisplayMode;".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/Win32GraphicsDevice.getDefaultPixIDImpl(I)I", Any)]
#[async_method]
pub async fn get_default_pix_idimpl<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/Win32GraphicsDevice.getDefaultPixIDImpl(I)I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/Win32GraphicsDevice.getMaxConfigsImpl(I)I", Any)]
#[async_method]
pub async fn get_max_configs_impl<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/Win32GraphicsDevice.getMaxConfigsImpl(I)I".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/Win32GraphicsDevice.getNativeScaleX(I)F",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_native_scale_x<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/Win32GraphicsDevice.getNativeScaleX(I)F".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/Win32GraphicsDevice.getNativeScaleY(I)F",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_native_scale_y<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/Win32GraphicsDevice.getNativeScaleY(I)F".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/Win32GraphicsDevice.initDevice(I)V", Any)]
#[async_method]
pub async fn init_device<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/Win32GraphicsDevice.initDevice(I)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/Win32GraphicsDevice.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/Win32GraphicsDevice.initIDs()V".to_string())
            .into(),
    )
}
#[intrinsic_method(
    "sun/awt/Win32GraphicsDevice.initNativeScale(I)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn init_native_scale<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/Win32GraphicsDevice.initNativeScale(I)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/Win32GraphicsDevice.isPixFmtSupported(II)Z", Any)]
#[async_method]
pub async fn is_pix_fmt_supported<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen = parameters.pop_int()?;
    let _pix_fmt_id = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/Win32GraphicsDevice.isPixFmtSupported(II)Z".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/Win32GraphicsDevice.makeColorModel(IZ)Ljava/awt/image/ColorModel;",
    Any
)]
#[async_method]
pub async fn make_color_model<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _dynamic = parameters.pop_bool()?;
    let _screen = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/Win32GraphicsDevice.makeColorModel(IZ)Ljava/awt/image/ColorModel;".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/Win32GraphicsDevice.setNativeScale(IFF)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_native_scale<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _scale_y = parameters.pop_float()?;
    let _scale_x = parameters.pop_float()?;
    let _screen = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/Win32GraphicsDevice.setNativeScale(IFF)V".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_config_display_mode() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = config_display_mode(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/Win32GraphicsDevice.configDisplayMode(ILjava/awt/peer/WindowPeer;IIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_enter_full_screen_exclusive() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = enter_full_screen_exclusive(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/awt/Win32GraphicsDevice.enterFullScreenExclusive(ILjava/awt/peer/WindowPeer;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_enum_display_modes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = enum_display_modes(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/awt/Win32GraphicsDevice.enumDisplayModes(ILjava/util/ArrayList;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_exit_full_screen_exclusive() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = exit_full_screen_exclusive(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/awt/Win32GraphicsDevice.exitFullScreenExclusive(ILjava/awt/peer/WindowPeer;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_current_display_mode() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_current_display_mode(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/Win32GraphicsDevice.getCurrentDisplayMode(I)Ljava/awt/DisplayMode;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_default_pix_idimpl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_default_pix_idimpl(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/Win32GraphicsDevice.getDefaultPixIDImpl(I)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_max_configs_impl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_max_configs_impl(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/Win32GraphicsDevice.getMaxConfigsImpl(I)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_native_scale_x() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_native_scale_x(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/Win32GraphicsDevice.getNativeScaleX(I)F",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_native_scale_y() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_native_scale_y(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/Win32GraphicsDevice.getNativeScaleY(I)F",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init_device() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_device(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/Win32GraphicsDevice.initDevice(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init_ids() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/Win32GraphicsDevice.initIDs()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init_native_scale() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_native_scale(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/Win32GraphicsDevice.initNativeScale(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_is_pix_fmt_supported() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            is_pix_fmt_supported(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/Win32GraphicsDevice.isPixFmtSupported(II)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_make_color_model() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = make_color_model(
            thread,
            Parameters::new(vec![Value::Int(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun/awt/Win32GraphicsDevice.makeColorModel(IZ)Ljava/awt/image/ColorModel;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_native_scale() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_native_scale(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Float(0.0), Value::Float(0.0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/Win32GraphicsDevice.setNativeScale(IFF)V",
            result.unwrap_err().to_string()
        );
    }
}
