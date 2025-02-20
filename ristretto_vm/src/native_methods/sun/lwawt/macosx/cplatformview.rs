use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/lwawt/macosx/CPlatformView";

/// Register all native methods for `sun.lwawt.macosx.CPlatformView`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "nativeCreateView",
        "(IIIIJ)J",
        native_create_view,
    );
    registry.register(
        CLASS_NAME,
        "nativeGetLocationOnScreen",
        "(J)Ljava/awt/geom/Rectangle2D;",
        native_get_location_on_screen,
    );
    registry.register(
        CLASS_NAME,
        "nativeGetNSViewDisplayID",
        "(J)I",
        native_get_ns_view_display_id,
    );
    registry.register(
        CLASS_NAME,
        "nativeIsViewUnderMouse",
        "(J)Z",
        native_is_view_under_mouse,
    );
    registry.register(
        CLASS_NAME,
        "nativeSetAutoResizable",
        "(JZ)V",
        native_set_auto_resizable,
    );
}

#[async_recursion(?Send)]
async fn native_create_view(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformView.nativeCreateView(IIIIJ)J")
}

#[async_recursion(?Send)]
async fn native_get_location_on_screen(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformView.nativeGetLocationOnScreen(J)Ljava/awt/geom/Rectangle2D;")
}

#[async_recursion(?Send)]
async fn native_get_ns_view_display_id(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformView.nativeGetNSViewDisplayID(J)I")
}

#[async_recursion(?Send)]
async fn native_is_view_under_mouse(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformView.nativeIsViewUnderMouse(J)Z")
}

#[async_recursion(?Send)]
async fn native_set_auto_resizable(
    _thread: Arc<Thread>,
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
