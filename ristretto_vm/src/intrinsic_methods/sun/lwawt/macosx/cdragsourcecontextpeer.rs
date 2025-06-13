use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::{Any, GreaterThan, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/lwawt/macosx/CDragSourceContextPeer.createNativeDragSource(Ljava/awt/Component;JLjava/awt/datatransfer/Transferable;Ljava/awt/event/InputEvent;IIIIJLsun/lwawt/macosx/CImage;III[JLjava/util/Map;)J",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn create_native_drag_source_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.lwawt.macosx.CDragSourceContextPeer.createNativeDragSource(Ljava/awt/Component;JLjava/awt/datatransfer/Transferable;Ljava/awt/event/InputEvent;IIIIJLsun/lwawt/macosx/CImage;III[JLjava/util/Map;)J"
    )
}

#[intrinsic_method(
    "sun/lwawt/macosx/CDragSourceContextPeer.createNativeDragSource(Ljava/awt/Component;JLjava/awt/datatransfer/Transferable;Ljava/awt/event/InputEvent;IIIIJJIII[JLjava/util/Map;)J",
    GreaterThan(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn create_native_drag_source_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.lwawt.macosx.CDragSourceContextPeer.createNativeDragSource(Ljava/awt/Component;JLjava/awt/datatransfer/Transferable;Ljava/awt/event/InputEvent;IIIIJJIII[JLjava/util/Map;)J"
    )
}

#[intrinsic_method("sun/lwawt/macosx/CDragSourceContextPeer.doDragging(J)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn do_dragging(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CDragSourceContextPeer.doDragging(J)V")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CDragSourceContextPeer.releaseNativeDragSource(J)V",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn release_native_drag_source(
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
    async fn test_create_native_drag_source_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_native_drag_source_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CDragSourceContextPeer.createNativeDragSource(Ljava/awt/Component;JLjava/awt/datatransfer/Transferable;Ljava/awt/event/InputEvent;IIIIJJIII[JLjava/util/Map;)J"
    )]
    async fn test_create_native_drag_source_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_native_drag_source_1(thread, Parameters::default()).await;
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
