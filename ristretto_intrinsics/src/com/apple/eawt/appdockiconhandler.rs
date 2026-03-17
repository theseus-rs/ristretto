use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("com/apple/eawt/_AppDockIconHandler.nativeGetDockIconImage()J", Any)]
#[async_method]
pub async fn native_get_dock_icon_image<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.apple.eawt._AppDockIconHandler.nativeGetDockIconImage()J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/apple/eawt/_AppDockIconHandler.nativeSetDockIconBadge(Ljava/lang/String;)V",
    Any
)]
#[async_method]
pub async fn native_set_dock_icon_badge<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.apple.eawt._AppDockIconHandler.nativeSetDockIconBadge(Ljava/lang/String;)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method("com/apple/eawt/_AppDockIconHandler.nativeSetDockIconImage(J)V", Any)]
#[async_method]
pub async fn native_set_dock_icon_image<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.apple.eawt._AppDockIconHandler.nativeSetDockIconImage(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/apple/eawt/_AppDockIconHandler.nativeSetDockIconProgress(I)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn native_set_dock_icon_progress<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.apple.eawt._AppDockIconHandler.nativeSetDockIconProgress(I)V".to_string(),
    )
    .into())
}

#[intrinsic_method("com/apple/eawt/_AppDockIconHandler.nativeSetDockMenu(J)V", Any)]
#[async_method]
pub async fn native_set_dock_menu<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.apple.eawt._AppDockIconHandler.nativeSetDockMenu(J)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_native_get_dock_icon_image() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_get_dock_icon_image(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_set_dock_icon_badge() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_set_dock_icon_badge(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_set_dock_icon_image() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_set_dock_icon_image(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_set_dock_icon_progress() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_set_dock_icon_progress(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_set_dock_menu() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_set_dock_menu(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
