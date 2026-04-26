use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/awt/screencast/ScreencastHelper.closeSession()V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn close_session<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/screencast/ScreencastHelper.closeSession()V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/screencast/ScreencastHelper.getRGBPixelsImpl(IIII[I[ILjava/lang/String;)I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn get_rgbpixels_impl<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _jtoken = parameters.pop_reference()?;
    let _affected_screens_bounds_array = parameters.pop_reference()?;
    let _pixel_array = parameters.pop_reference()?;
    let _jheight = parameters.pop_int()?;
    let _jwidth = parameters.pop_int()?;
    let _jy = parameters.pop_int()?;
    let _jx = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/screencast/ScreencastHelper.getRGBPixelsImpl(IIII[I[ILjava/lang/String;)I"
            .to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/screencast/ScreencastHelper.loadPipewire(IZ)Z",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn load_pipewire<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_debug = parameters.pop_bool()?;
    let _method = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/screencast/ScreencastHelper.loadPipewire(IZ)Z".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/screencast/ScreencastHelper.remoteDesktopKeyImpl(ZILjava/lang/String;)I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn remote_desktop_key_impl<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _jtoken = parameters.pop_reference()?;
    let _jkey = parameters.pop_int()?;
    let _is_press = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/screencast/ScreencastHelper.remoteDesktopKeyImpl(ZILjava/lang/String;)I"
            .to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/screencast/ScreencastHelper.remoteDesktopMouseButtonImpl(ZILjava/lang/String;)I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn remote_desktop_mouse_button_impl<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _jtoken = parameters.pop_reference()?;
    let _buttons = parameters.pop_int()?;
    let _is_press = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/screencast/ScreencastHelper.remoteDesktopMouseButtonImpl(ZILjava/lang/String;)I"
            .to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/screencast/ScreencastHelper.remoteDesktopMouseMoveImpl(IILjava/lang/String;)I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn remote_desktop_mouse_move_impl<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _jtoken = parameters.pop_reference()?;
    let _jy = parameters.pop_int()?;
    let _jx = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/screencast/ScreencastHelper.remoteDesktopMouseMoveImpl(IILjava/lang/String;)I"
            .to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/screencast/ScreencastHelper.remoteDesktopMouseWheelImpl(ILjava/lang/String;)I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn remote_desktop_mouse_wheel_impl<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _jtoken = parameters.pop_reference()?;
    let _j_wheel_amt = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/screencast/ScreencastHelper.remoteDesktopMouseWheelImpl(ILjava/lang/String;)I"
            .to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_close_session() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = close_session(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/screencast/ScreencastHelper.closeSession()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_rgbpixels_impl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_rgbpixels_impl(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/screencast/ScreencastHelper.getRGBPixelsImpl(IIII[I[ILjava/lang/String;)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_load_pipewire() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = load_pipewire(
            thread,
            Parameters::new(vec![Value::Int(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun/awt/screencast/ScreencastHelper.loadPipewire(IZ)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_remote_desktop_key_impl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = remote_desktop_key_impl(
            thread,
            Parameters::new(vec![Value::from(false), Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/awt/screencast/ScreencastHelper.remoteDesktopKeyImpl(ZILjava/lang/String;)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_remote_desktop_mouse_button_impl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = remote_desktop_mouse_button_impl(
            thread,
            Parameters::new(vec![Value::from(false), Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/awt/screencast/ScreencastHelper.remoteDesktopMouseButtonImpl(ZILjava/lang/String;)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_remote_desktop_mouse_move_impl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = remote_desktop_mouse_move_impl(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/awt/screencast/ScreencastHelper.remoteDesktopMouseMoveImpl(IILjava/lang/String;)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_remote_desktop_mouse_wheel_impl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = remote_desktop_mouse_wheel_impl(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/awt/screencast/ScreencastHelper.remoteDesktopMouseWheelImpl(ILjava/lang/String;)I",
            result.unwrap_err().to_string()
        );
    }
}
