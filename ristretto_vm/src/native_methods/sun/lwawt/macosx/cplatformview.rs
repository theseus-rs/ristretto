use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.lwawt.macosx.CPlatformView`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/CPlatformView";
    registry.register(
        class_name,
        "nativeCreateView",
        "(IIIIJ)J",
        native_create_view,
    );
    registry.register(
        class_name,
        "nativeGetLocationOnScreen",
        "(J)Ljava/awt/geom/Rectangle2D;",
        native_get_location_on_screen,
    );
    registry.register(
        class_name,
        "nativeGetNSViewDisplayID",
        "(J)I",
        native_get_ns_view_display_id,
    );
    registry.register(
        class_name,
        "nativeIsViewUnderMouse",
        "(J)Z",
        native_is_view_under_mouse,
    );
    registry.register(
        class_name,
        "nativeSetAutoResizable",
        "(JZ)V",
        native_set_auto_resizable,
    );
}

#[async_recursion(?Send)]
async fn native_create_view(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformView.nativeCreateView(IIIIJ)J")
}

#[async_recursion(?Send)]
async fn native_get_location_on_screen(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformView.nativeGetLocationOnScreen(J)Ljava/awt/geom/Rectangle2D;")
}

#[async_recursion(?Send)]
async fn native_get_ns_view_display_id(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformView.nativeGetNSViewDisplayID(J)I")
}

#[async_recursion(?Send)]
async fn native_is_view_under_mouse(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformView.nativeIsViewUnderMouse(J)Z")
}

#[async_recursion(?Send)]
async fn native_set_auto_resizable(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformView.nativeSetAutoResizable(JZ)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/lwawt/macosx/CPlatformView";
        assert!(registry
            .method(class_name, "nativeCreateView", "(IIIIJ)J")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "nativeGetLocationOnScreen",
                "(J)Ljava/awt/geom/Rectangle2D;"
            )
            .is_some());
        assert!(registry
            .method(class_name, "nativeGetNSViewDisplayID", "(J)I")
            .is_some());
        assert!(registry
            .method(class_name, "nativeIsViewUnderMouse", "(J)Z")
            .is_some());
        assert!(registry
            .method(class_name, "nativeSetAutoResizable", "(JZ)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CPlatformView.nativeCreateView(IIIIJ)J")]
    async fn test_native_create_view() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_create_view(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.lwawt.macosx.CPlatformView.nativeGetLocationOnScreen(J)Ljava/awt/geom/Rectangle2D;"
    )]
    async fn test_native_get_location_on_screen() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_location_on_screen(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CPlatformView.nativeGetNSViewDisplayID(J)I")]
    async fn test_native_get_ns_view_display_id() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_ns_view_display_id(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CPlatformView.nativeIsViewUnderMouse(J)Z")]
    async fn test_native_is_view_under_mouse() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_is_view_under_mouse(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CPlatformView.nativeSetAutoResizable(JZ)V")]
    async fn test_native_set_auto_resizable() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_auto_resizable(thread, Arguments::default()).await;
    }
}
