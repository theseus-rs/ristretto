use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::{Any, GreaterThan, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/lwawt/macosx/CRobot.initRobot()V", Any)]
#[async_method]
pub async fn init_robot<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.CRobot.initRobot()V".to_string()).into())
}

#[intrinsic_method("sun/lwawt/macosx/CRobot.keyEvent(IZ)V", Any)]
#[async_method]
pub async fn key_event<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _keydown = parameters.pop_bool()?;
    let _java_key_code = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.CRobot.keyEvent(IZ)V".to_string()).into())
}

#[intrinsic_method("sun/lwawt/macosx/CRobot.mouseEvent(IIIIZZ)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn mouse_event_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg5 = parameters.pop_bool()?;
    let _arg4 = parameters.pop_bool()?;
    let _arg3 = parameters.pop_int()?;
    let _arg2 = parameters.pop_int()?;
    let _arg1 = parameters.pop_int()?;
    let _arg0 = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.CRobot.mouseEvent(IIIIZZ)V".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/lwawt/macosx/CRobot.mouseEvent(IIIZZ)V", GreaterThan(JAVA_8))]
#[async_method]
pub async fn mouse_event_1<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_mouse_move = parameters.pop_bool()?;
    let _is_buttons_down_state = parameters.pop_bool()?;
    let _buttons_state = parameters.pop_int()?;
    let _last_y = parameters.pop_int()?;
    let _last_x = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.CRobot.mouseEvent(IIIZZ)V".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/lwawt/macosx/CRobot.mouseWheel(I)V", Any)]
#[async_method]
pub async fn mouse_wheel<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _wheel_amt = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.CRobot.mouseWheel(I)V".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "sun/lwawt/macosx/CRobot.nativeGetScreenPixels(IIII[I)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_get_screen_pixels_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg4 = parameters.pop_reference()?;
    let _arg3 = parameters.pop_int()?;
    let _arg2 = parameters.pop_int()?;
    let _arg1 = parameters.pop_int()?;
    let _arg0 = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CRobot.nativeGetScreenPixels(IIII[I)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CRobot.nativeGetScreenPixels(IIIID[I)V",
    GreaterThan(JAVA_8)
)]
#[async_method]
pub async fn native_get_screen_pixels_1<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _pixels = parameters.pop_reference()?;
    let _scale = parameters.pop_double()?;
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CRobot.nativeGetScreenPixels(IIIID[I)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init_robot() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_robot(thread, Parameters::default()).await;
        assert_eq!(
            "sun.lwawt.macosx.CRobot.initRobot()V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_key_event() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = key_event(
            thread,
            Parameters::new(vec![Value::Int(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CRobot.keyEvent(IZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_mouse_event_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = mouse_event_0(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::from(false),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CRobot.mouseEvent(IIIIZZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_mouse_event_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = mouse_event_1(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::from(false),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CRobot.mouseEvent(IIIZZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_mouse_wheel() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = mouse_wheel(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.CRobot.mouseWheel(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_get_screen_pixels_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_get_screen_pixels_0(
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
            "sun.lwawt.macosx.CRobot.nativeGetScreenPixels(IIII[I)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_get_screen_pixels_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_get_screen_pixels_1(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Double(0.0),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CRobot.nativeGetScreenPixels(IIIID[I)V",
            result.unwrap_err().to_string()
        );
    }
}
