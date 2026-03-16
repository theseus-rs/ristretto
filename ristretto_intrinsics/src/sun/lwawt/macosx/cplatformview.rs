use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/lwawt/macosx/CPlatformView.nativeCreateView(IIIIJ)J", Any)]
#[async_method]
pub async fn native_create_view<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CPlatformView.nativeCreateView(IIIIJ)J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CPlatformView.nativeGetLocationOnScreen(J)Ljava/awt/geom/Rectangle2D;",
    Any
)]
#[async_method]
pub async fn native_get_location_on_screen<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CPlatformView.nativeGetLocationOnScreen(J)Ljava/awt/geom/Rectangle2D;"
            .to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CPlatformView.nativeGetNSViewDisplayID(J)I", Any)]
#[async_method]
pub async fn native_get_ns_view_display_id<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CPlatformView.nativeGetNSViewDisplayID(J)I".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CPlatformView.nativeIsViewUnderMouse(J)Z", Any)]
#[async_method]
pub async fn native_is_view_under_mouse<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CPlatformView.nativeIsViewUnderMouse(J)Z".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CPlatformView.nativeSetAutoResizable(JZ)V", Any)]
#[async_method]
pub async fn native_set_auto_resizable<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CPlatformView.nativeSetAutoResizable(JZ)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_native_create_view() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_create_view(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_get_location_on_screen() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_get_location_on_screen(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_get_ns_view_display_id() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_get_ns_view_display_id(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_is_view_under_mouse() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_is_view_under_mouse(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_set_auto_resizable() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_set_auto_resizable(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
