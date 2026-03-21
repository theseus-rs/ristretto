use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::{Any, GreaterThan, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/lwawt/macosx/CDragSourceContextPeer.createNativeDragSource(Ljava/awt/Component;JLjava/awt/datatransfer/Transferable;Ljava/awt/event/InputEvent;IIIIJLsun/lwawt/macosx/CImage;III[JLjava/util/Map;)J",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn create_native_drag_source_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.CDragSourceContextPeer.createNativeDragSource(Ljava/awt/Component;JLjava/awt/datatransfer/Transferable;Ljava/awt/event/InputEvent;IIIIJLsun/lwawt/macosx/CImage;III[JLjava/util/Map;)J".to_string()).into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CDragSourceContextPeer.createNativeDragSource(Ljava/awt/Component;JLjava/awt/datatransfer/Transferable;Ljava/awt/event/InputEvent;IIIIJJIII[JLjava/util/Map;)J",
    GreaterThan(JAVA_8)
)]
#[async_method]
pub async fn create_native_drag_source_1<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.CDragSourceContextPeer.createNativeDragSource(Ljava/awt/Component;JLjava/awt/datatransfer/Transferable;Ljava/awt/event/InputEvent;IIIIJJIII[JLjava/util/Map;)J".to_string()).into())
}

#[intrinsic_method("sun/lwawt/macosx/CDragSourceContextPeer.doDragging(J)V", Any)]
#[async_method]
pub async fn do_dragging<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CDragSourceContextPeer.doDragging(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CDragSourceContextPeer.releaseNativeDragSource(J)V",
    Any
)]
#[async_method]
pub async fn release_native_drag_source<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CDragSourceContextPeer.releaseNativeDragSource(J)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_native_drag_source_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = create_native_drag_source_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_native_drag_source_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_native_drag_source_1(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_do_dragging() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_dragging(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_release_native_drag_source() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = release_native_drag_source(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
