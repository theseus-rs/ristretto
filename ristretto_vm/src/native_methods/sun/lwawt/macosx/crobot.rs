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
