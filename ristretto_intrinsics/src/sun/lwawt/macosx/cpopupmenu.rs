use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/lwawt/macosx/CPopupMenu.nativeCreatePopupMenu()J", Any)]
#[async_method]
pub async fn native_create_popup_menu<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CPopupMenu.nativeCreatePopupMenu()J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CPopupMenu.nativeShowPopupMenu(JII)J", Any)]
#[async_method]
pub async fn native_show_popup_menu<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CPopupMenu.nativeShowPopupMenu(JII)J".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_native_create_popup_menu() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_create_popup_menu(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_show_popup_menu() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_show_popup_menu(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
