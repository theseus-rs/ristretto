use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/lwawt/macosx/CCursorManager.nativeGetCursorPosition()Ljava/awt/geom/Point2D;",
    Any
)]
#[async_method]
pub async fn native_get_cursor_position<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CCursorManager.nativeGetCursorPosition()Ljava/awt/geom/Point2D;")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CCursorManager.nativeSetAllowsCursorSetInBackground(Z)V",
    Any
)]
#[async_method]
pub async fn native_set_allows_cursor_set_in_background<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CCursorManager.nativeSetAllowsCursorSetInBackground(Z)V")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CCursorManager.nativeSetBuiltInCursor(ILjava/lang/String;)V",
    Any
)]
#[async_method]
pub async fn native_set_built_in_cursor<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CCursorManager.nativeSetBuiltInCursor(ILjava/lang/String;)V")
}

#[intrinsic_method("sun/lwawt/macosx/CCursorManager.nativeSetCustomCursor(JDD)V", Any)]
#[async_method]
pub async fn native_set_custom_cursor<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
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
