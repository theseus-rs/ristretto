use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::{Any, Equal};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/windows/WRobotPeer._dispose()V", Equal(JAVA_8))]
#[async_method]
pub async fn dispose<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WRobotPeer._dispose()V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WRobotPeer.create()V", Equal(JAVA_8))]
#[async_method]
pub async fn create<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun/awt/windows/WRobotPeer.create()V".to_string()).into())
}
#[intrinsic_method("sun/awt/windows/WRobotPeer.getRGBPixels(IIII[I)V", Any)]
#[async_method]
pub async fn get_rgbpixels<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _pixel_array = parameters.pop_reference()?;
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WRobotPeer.getRGBPixels(IIII[I)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WRobotPeer.keyPress(I)V", Any)]
#[async_method]
pub async fn key_press<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _javakey = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WRobotPeer.keyPress(I)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WRobotPeer.keyRelease(I)V", Any)]
#[async_method]
pub async fn key_release<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _javakey = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WRobotPeer.keyRelease(I)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WRobotPeer.mouseMoveImpl(II)V", Any)]
#[async_method]
pub async fn mouse_move_impl<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WRobotPeer.mouseMoveImpl(II)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WRobotPeer.mousePress(I)V", Any)]
#[async_method]
pub async fn mouse_press<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _buttons = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WRobotPeer.mousePress(I)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WRobotPeer.mouseRelease(I)V", Any)]
#[async_method]
pub async fn mouse_release<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _buttons = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WRobotPeer.mouseRelease(I)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WRobotPeer.mouseWheel(I)V", Any)]
#[async_method]
pub async fn mouse_wheel<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _wheel_amt = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WRobotPeer.mouseWheel(I)V".to_string())
            .into(),
    )
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dispose() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = dispose(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WRobotPeer._dispose()V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_create() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WRobotPeer.create()V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_rgbpixels() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_rgbpixels(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WRobotPeer.getRGBPixels(IIII[I)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_key_press() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = key_press(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/windows/WRobotPeer.keyPress(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_key_release() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = key_release(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/windows/WRobotPeer.keyRelease(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_mouse_move_impl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            mouse_move_impl(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/windows/WRobotPeer.mouseMoveImpl(II)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_mouse_press() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = mouse_press(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/windows/WRobotPeer.mousePress(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_mouse_release() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = mouse_release(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/windows/WRobotPeer.mouseRelease(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_mouse_wheel() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = mouse_wheel(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/windows/WRobotPeer.mouseWheel(I)V",
            result.unwrap_err().to_string()
        );
    }
}
