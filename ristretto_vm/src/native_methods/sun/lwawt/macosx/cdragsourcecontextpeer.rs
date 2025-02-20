use crate::Result;
use crate::native_methods::registry::{JAVA_8, MethodRegistry};
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/lwawt/macosx/CDragSourceContextPeer";

/// Register all native methods for `sun.lwawt.macosx.CDragSourceContextPeer`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_8 {
        registry.register(CLASS_NAME, "createNativeDragSource", "(Ljava/awt/Component;JLjava/awt/datatransfer/Transferable;Ljava/awt/event/InputEvent;IIIIJLsun/lwawt/macosx/CImage;III[JLjava/util/Map;)J", create_native_drag_source);
    } else {
        registry.register(CLASS_NAME, "createNativeDragSource", "(Ljava/awt/Component;JLjava/awt/datatransfer/Transferable;Ljava/awt/event/InputEvent;IIIIJJIII[JLjava/util/Map;)J", create_native_drag_source);
    }

    registry.register(CLASS_NAME, "doDragging", "(J)V", do_dragging);
    registry.register(
        CLASS_NAME,
        "releaseNativeDragSource",
        "(J)V",
        release_native_drag_source,
    );
}

#[async_recursion(?Send)]
async fn create_native_drag_source(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.lwawt.macosx.CDragSourceContextPeer.createNativeDragSource(Ljava/awt/Component;JLjava/awt/datatransfer/Transferable;Ljava/awt/event/InputEvent;IIIIJLsun/lwawt/macosx/CImage;III[JLjava/util/Map;)J"
    )
}

#[async_recursion(?Send)]
async fn do_dragging(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CDragSourceContextPeer.doDragging(J)V")
}

#[async_recursion(?Send)]
async fn release_native_drag_source(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CDragSourceContextPeer.releaseNativeDragSource(J)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CDragSourceContextPeer.createNativeDragSource(Ljava/awt/Component;JLjava/awt/datatransfer/Transferable;Ljava/awt/event/InputEvent;IIIIJLsun/lwawt/macosx/CImage;III[JLjava/util/Map;)J"
    )]
    async fn test_create_native_drag_source() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_native_drag_source(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CDragSourceContextPeer.doDragging(J)V"
    )]
    async fn test_do_dragging() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = do_dragging(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CDragSourceContextPeer.releaseNativeDragSource(J)V"
    )]
    async fn test_release_native_drag_source() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = release_native_drag_source(thread, Parameters::default()).await;
    }
}
