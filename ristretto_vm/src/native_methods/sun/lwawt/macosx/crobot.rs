use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_8: Version = Version::Java8 { minor: 0 };

/// Register all native methods for `sun.lwawt.macosx.CRobot`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/CRobot";
    let java_version = registry.java_version().clone();

    if java_version <= JAVA_8 {
        registry.register(class_name, "mouseEvent", "(IIIIZZ)V", mouse_event);
        registry.register(
            class_name,
            "nativeGetScreenPixels",
            "(IIII[I)V",
            native_get_screen_pixels,
        );
    } else {
        registry.register(class_name, "mouseEvent", "(IIIZZ)V", mouse_event);
        registry.register(
            class_name,
            "nativeGetScreenPixels",
            "(IIIID[I)V",
            native_get_screen_pixels,
        );
    }

    registry.register(class_name, "initRobot", "()V", init_robot);
    registry.register(class_name, "keyEvent", "(IZ)V", key_event);
    registry.register(class_name, "mouseWheel", "(I)V", mouse_wheel);
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

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/lwawt/macosx/CRobot";
        assert!(registry.method(class_name, "initRobot", "()V").is_some());
        assert!(registry.method(class_name, "keyEvent", "(IZ)V").is_some());
        assert!(registry
            .method(class_name, "mouseEvent", "(IIIIZZ)V")
            .is_some());
        assert!(registry.method(class_name, "mouseWheel", "(I)V").is_some());
        assert!(registry
            .method(class_name, "nativeGetScreenPixels", "(IIII[I)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CRobot.initRobot()V")]
    async fn test_init_robot() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_robot(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CRobot.keyEvent(IZ)V")]
    async fn test_key_event() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = key_event(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CRobot.mouseEvent(IIIIZZ)V")]
    async fn test_mouse_event() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = mouse_event(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CRobot.mouseWheel(I)V")]
    async fn test_mouse_wheel() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = mouse_wheel(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CRobot.nativeGetScreenPixels(IIII[I)V")]
    async fn test_native_get_screen_pixels() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_screen_pixels(thread, Arguments::default()).await;
    }
}
