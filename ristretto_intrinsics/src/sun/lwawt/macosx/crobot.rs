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
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.CRobot.keyEvent(IZ)V".to_string()).into())
}

#[intrinsic_method("sun/lwawt/macosx/CRobot.mouseEvent(IIIIZZ)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn mouse_event_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.CRobot.mouseEvent(IIIIZZ)V".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/lwawt/macosx/CRobot.mouseEvent(IIIZZ)V", GreaterThan(JAVA_8))]
#[async_method]
pub async fn mouse_event_1<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.CRobot.mouseEvent(IIIZZ)V".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/lwawt/macosx/CRobot.mouseWheel(I)V", Any)]
#[async_method]
pub async fn mouse_wheel<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_key_event() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = key_event(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_mouse_event_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = mouse_event_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_mouse_event_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = mouse_event_1(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_mouse_wheel() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = mouse_wheel(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_get_screen_pixels_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_get_screen_pixels_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_get_screen_pixels_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_get_screen_pixels_1(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
