use ristretto_classfile::VersionSpecification::{Any, Equal, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_8, JAVA_11};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/awt/X11/XRobotPeer.getRGBPixelsImpl(Lsun/awt/X11GraphicsConfig;IIII[I)V",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn get_rgbpixels_impl<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _pixel_array = parameters.pop_reference()?;
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _xgc = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XRobotPeer.getRGBPixelsImpl(Lsun/awt/X11GraphicsConfig;IIII[I)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/X11/XRobotPeer.getRGBPixelsImpl(Lsun/awt/X11GraphicsConfig;IIII[IZ)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_rgbpixels_impl_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_gtk_supported = parameters.pop_bool()?;
    let _pixel_array = parameters.pop_reference()?;
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _xgc = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XRobotPeer.getRGBPixelsImpl(Lsun/awt/X11GraphicsConfig;IIII[IZ)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XRobotPeer.keyPressImpl(I)V", Any)]
#[async_method]
pub async fn key_press_impl<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _keycode = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XRobotPeer.keyPressImpl(I)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XRobotPeer.keyReleaseImpl(I)V", Any)]
#[async_method]
pub async fn key_release_impl<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _keycode = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XRobotPeer.keyReleaseImpl(I)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XRobotPeer.loadNativeLibraries()V", Any)]
#[async_method]
pub async fn load_native_libraries<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XRobotPeer.loadNativeLibraries()V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/X11/XRobotPeer.mouseMoveImpl(Lsun/awt/X11GraphicsConfig;II)V",
    Any
)]
#[async_method]
pub async fn mouse_move_impl<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _xgc = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XRobotPeer.mouseMoveImpl(Lsun/awt/X11GraphicsConfig;II)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XRobotPeer.mousePressImpl(I)V", Any)]
#[async_method]
pub async fn mouse_press_impl<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _buttons = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XRobotPeer.mousePressImpl(I)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XRobotPeer.mouseReleaseImpl(I)V", Any)]
#[async_method]
pub async fn mouse_release_impl<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _buttons = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XRobotPeer.mouseReleaseImpl(I)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XRobotPeer.mouseWheelImpl(I)V", Any)]
#[async_method]
pub async fn mouse_wheel_impl<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _wheel_amt = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XRobotPeer.mouseWheelImpl(I)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XRobotPeer.setup(I[I)V", Any)]
#[async_method]
pub async fn setup<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _button_down_masks = parameters.pop_reference()?;
    let _number_of_buttons = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("sun/awt/X11/XRobotPeer.setup(I[I)V".to_string()).into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_rgbpixels_impl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_rgbpixels_impl(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XRobotPeer.getRGBPixelsImpl(Lsun/awt/X11GraphicsConfig;IIII[I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_rgbpixels_impl_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_rgbpixels_impl_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XRobotPeer.getRGBPixelsImpl(Lsun/awt/X11GraphicsConfig;IIII[IZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_key_press_impl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = key_press_impl(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/X11/XRobotPeer.keyPressImpl(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_key_release_impl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = key_release_impl(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/X11/XRobotPeer.keyReleaseImpl(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_load_native_libraries() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = load_native_libraries(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/X11/XRobotPeer.loadNativeLibraries()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_mouse_move_impl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = mouse_move_impl(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XRobotPeer.mouseMoveImpl(Lsun/awt/X11GraphicsConfig;II)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_mouse_press_impl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = mouse_press_impl(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/X11/XRobotPeer.mousePressImpl(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_mouse_release_impl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = mouse_release_impl(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/X11/XRobotPeer.mouseReleaseImpl(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_mouse_wheel_impl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = mouse_wheel_impl(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/X11/XRobotPeer.mouseWheelImpl(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_setup() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = setup(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XRobotPeer.setup(I[I)V",
            result.unwrap_err().to_string()
        );
    }
}
