use crate::arguments::Arguments;
use crate::native_methods::registry::{MethodRegistry, JAVA_8};
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/lwawt/macosx/CRobot";

/// Register all native methods for `sun.lwawt.macosx.CRobot`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_8 {
        registry.register(CLASS_NAME, "mouseEvent", "(IIIIZZ)V", mouse_event);
        registry.register(
            CLASS_NAME,
            "nativeGetScreenPixels",
            "(IIII[I)V",
            native_get_screen_pixels,
        );
    } else {
        registry.register(CLASS_NAME, "mouseEvent", "(IIIZZ)V", mouse_event);
        registry.register(
            CLASS_NAME,
            "nativeGetScreenPixels",
            "(IIIID[I)V",
            native_get_screen_pixels,
        );
    }

    registry.register(CLASS_NAME, "initRobot", "()V", init_robot);
    registry.register(CLASS_NAME, "keyEvent", "(IZ)V", key_event);
    registry.register(CLASS_NAME, "mouseWheel", "(I)V", mouse_wheel);
}

#[async_recursion(?Send)]
async fn init_robot(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CRobot.initRobot()V")
}

#[async_recursion(?Send)]
async fn key_event(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CRobot.keyEvent(IZ)V")
}

#[async_recursion(?Send)]
async fn mouse_event(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CRobot.mouseEvent(IIIIZZ)V")
}

#[async_recursion(?Send)]
async fn mouse_wheel(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CRobot.mouseWheel(I)V")
}

#[async_recursion(?Send)]
async fn native_get_screen_pixels(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CRobot.nativeGetScreenPixels(IIII[I)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.CRobot.initRobot()V")]
    async fn test_init_robot() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_robot(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.CRobot.keyEvent(IZ)V")]
    async fn test_key_event() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = key_event(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.CRobot.mouseEvent(IIIIZZ)V")]
    async fn test_mouse_event() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = mouse_event(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.CRobot.mouseWheel(I)V")]
    async fn test_mouse_wheel() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = mouse_wheel(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CRobot.nativeGetScreenPixels(IIII[I)V"
    )]
    async fn test_native_get_screen_pixels() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_screen_pixels(thread, Arguments::default()).await;
    }
}
