use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/lwawt/macosx/CCursorManager";

/// Register all native methods for `sun.lwawt.macosx.CCursorManager`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "nativeGetCursorPosition",
        "()Ljava/awt/geom/Point2D;",
        native_get_cursor_position,
    );
    registry.register(
        CLASS_NAME,
        "nativeSetAllowsCursorSetInBackground",
        "(Z)V",
        native_set_allows_cursor_set_in_background,
    );
    registry.register(
        CLASS_NAME,
        "nativeSetBuiltInCursor",
        "(ILjava/lang/String;)V",
        native_set_built_in_cursor,
    );
    registry.register(
        CLASS_NAME,
        "nativeSetCustomCursor",
        "(JDD)V",
        native_set_custom_cursor,
    );
}

#[async_recursion(?Send)]
async fn native_get_cursor_position(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CCursorManager.nativeGetCursorPosition()Ljava/awt/geom/Point2D;")
}

#[async_recursion(?Send)]
async fn native_set_allows_cursor_set_in_background(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CCursorManager.nativeSetAllowsCursorSetInBackground(Z)V")
}

#[async_recursion(?Send)]
async fn native_set_built_in_cursor(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CCursorManager.nativeSetBuiltInCursor(ILjava/lang/String;)V")
}

#[async_recursion(?Send)]
async fn native_set_custom_cursor(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CCursorManager.nativeSetCustomCursor(JDD)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CCursorManager.nativeGetCursorPosition()Ljava/awt/geom/Point2D;"
    )]
    async fn test_native_get_cursor_position() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_cursor_position(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CCursorManager.nativeSetAllowsCursorSetInBackground(Z)V"
    )]
    async fn test_native_set_allows_cursor_set_in_background() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_allows_cursor_set_in_background(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CCursorManager.nativeSetBuiltInCursor(ILjava/lang/String;)V"
    )]
    async fn test_native_set_built_in_cursor() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_built_in_cursor(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CCursorManager.nativeSetCustomCursor(JDD)V"
    )]
    async fn test_native_set_custom_cursor() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_custom_cursor(thread, Parameters::default()).await;
    }
}
