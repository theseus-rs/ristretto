use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::{Any, GreaterThan, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("sun/lwawt/macosx/CRobot.initRobot()V", Any)]
#[async_method]
pub(crate) async fn init_robot(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CRobot.initRobot()V")
}

#[intrinsic_method("sun/lwawt/macosx/CRobot.keyEvent(IZ)V", Any)]
#[async_method]
pub(crate) async fn key_event(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CRobot.keyEvent(IZ)V")
}

#[intrinsic_method("sun/lwawt/macosx/CRobot.mouseEvent(IIIIZZ)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub(crate) async fn mouse_event_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CRobot.mouseEvent(IIIIZZ)V")
}

#[intrinsic_method("sun/lwawt/macosx/CRobot.mouseEvent(IIIZZ)V", GreaterThan(JAVA_8))]
#[async_method]
pub(crate) async fn mouse_event_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CRobot.mouseEvent(IIIZZ)V")
}

#[intrinsic_method("sun/lwawt/macosx/CRobot.mouseWheel(I)V", Any)]
#[async_method]
pub(crate) async fn mouse_wheel(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CRobot.mouseWheel(I)V")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CRobot.nativeGetScreenPixels(IIII[I)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn native_get_screen_pixels_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CRobot.nativeGetScreenPixels(IIII[I)V")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CRobot.nativeGetScreenPixels(IIIID[I)V",
    GreaterThan(JAVA_8)
)]
#[async_method]
pub(crate) async fn native_get_screen_pixels_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CRobot.nativeGetScreenPixels(IIIID[I)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.CRobot.initRobot()V")]
    async fn test_init_robot() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_robot(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.CRobot.keyEvent(IZ)V")]
    async fn test_key_event() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = key_event(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.CRobot.mouseEvent(IIIIZZ)V")]
    async fn test_mouse_event_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = mouse_event_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.CRobot.mouseEvent(IIIZZ)V")]
    async fn test_mouse_event_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = mouse_event_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.CRobot.mouseWheel(I)V")]
    async fn test_mouse_wheel() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = mouse_wheel(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CRobot.nativeGetScreenPixels(IIII[I)V"
    )]
    async fn test_native_get_screen_pixels_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_screen_pixels_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CRobot.nativeGetScreenPixels(IIIID[I)V"
    )]
    async fn test_native_get_screen_pixels_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_screen_pixels_1(thread, Parameters::default()).await;
    }
}
