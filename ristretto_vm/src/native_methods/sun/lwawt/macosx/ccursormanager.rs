use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.lwawt.macosx.CCursorManager`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/CCursorManager";
    registry.register(
        class_name,
        "nativeGetCursorPosition",
        "()Ljava/awt/geom/Point2D;",
        native_get_cursor_position,
    );
    registry.register(
        class_name,
        "nativeSetAllowsCursorSetInBackground",
        "(Z)V",
        native_set_allows_cursor_set_in_background,
    );
    registry.register(
        class_name,
        "nativeSetBuiltInCursor",
        "(ILjava/lang/String;)V",
        native_set_built_in_cursor,
    );
    registry.register(
        class_name,
        "nativeSetCustomCursor",
        "(JDD)V",
        native_set_custom_cursor,
    );
}

#[async_recursion(?Send)]
async fn native_get_cursor_position(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CCursorManager.nativeGetCursorPosition()Ljava/awt/geom/Point2D;")
}

#[async_recursion(?Send)]
async fn native_set_allows_cursor_set_in_background(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CCursorManager.nativeSetAllowsCursorSetInBackground(Z)V")
}

#[async_recursion(?Send)]
async fn native_set_built_in_cursor(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CCursorManager.nativeSetBuiltInCursor(ILjava/lang/String;)V")
}

#[async_recursion(?Send)]
async fn native_set_custom_cursor(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CCursorManager.nativeSetCustomCursor(JDD)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/lwawt/macosx/CCursorManager";
        assert!(registry
            .method(
                class_name,
                "nativeGetCursorPosition",
                "()Ljava/awt/geom/Point2D;"
            )
            .is_some());
        assert!(registry
            .method(class_name, "nativeSetAllowsCursorSetInBackground", "(Z)V")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "nativeSetBuiltInCursor",
                "(ILjava/lang/String;)V"
            )
            .is_some());
        assert!(registry
            .method(class_name, "nativeSetCustomCursor", "(JDD)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.lwawt.macosx.CCursorManager.nativeGetCursorPosition()Ljava/awt/geom/Point2D;"
    )]
    async fn test_native_get_cursor_position() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_cursor_position(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.lwawt.macosx.CCursorManager.nativeSetAllowsCursorSetInBackground(Z)V"
    )]
    async fn test_native_set_allows_cursor_set_in_background() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_allows_cursor_set_in_background(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.lwawt.macosx.CCursorManager.nativeSetBuiltInCursor(ILjava/lang/String;)V"
    )]
    async fn test_native_set_built_in_cursor() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_built_in_cursor(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CCursorManager.nativeSetCustomCursor(JDD)V")]
    async fn test_native_set_custom_cursor() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_custom_cursor(thread, Arguments::default()).await;
    }
}
