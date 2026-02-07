use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/lwawt/macosx/CPlatformView.nativeCreateView(IIIIJ)J", Any)]
#[async_method]
pub async fn native_create_view<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformView.nativeCreateView(IIIIJ)J")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CPlatformView.nativeGetLocationOnScreen(J)Ljava/awt/geom/Rectangle2D;",
    Any
)]
#[async_method]
pub async fn native_get_location_on_screen<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformView.nativeGetLocationOnScreen(J)Ljava/awt/geom/Rectangle2D;")
}

#[intrinsic_method("sun/lwawt/macosx/CPlatformView.nativeGetNSViewDisplayID(J)I", Any)]
#[async_method]
pub async fn native_get_ns_view_display_id<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformView.nativeGetNSViewDisplayID(J)I")
}

#[intrinsic_method("sun/lwawt/macosx/CPlatformView.nativeIsViewUnderMouse(J)Z", Any)]
#[async_method]
pub async fn native_is_view_under_mouse<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformView.nativeIsViewUnderMouse(J)Z")
}

#[intrinsic_method("sun/lwawt/macosx/CPlatformView.nativeSetAutoResizable(JZ)V", Any)]
#[async_method]
pub async fn native_set_auto_resizable<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformView.nativeSetAutoResizable(JZ)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CPlatformView.nativeCreateView(IIIIJ)J"
    )]
    async fn test_native_create_view() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_create_view(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CPlatformView.nativeGetLocationOnScreen(J)Ljava/awt/geom/Rectangle2D;"
    )]
    async fn test_native_get_location_on_screen() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_location_on_screen(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CPlatformView.nativeGetNSViewDisplayID(J)I"
    )]
    async fn test_native_get_ns_view_display_id() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_ns_view_display_id(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CPlatformView.nativeIsViewUnderMouse(J)Z"
    )]
    async fn test_native_is_view_under_mouse() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_is_view_under_mouse(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CPlatformView.nativeSetAutoResizable(JZ)V"
    )]
    async fn test_native_set_auto_resizable() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_auto_resizable(thread, Parameters::default()).await;
    }
}
