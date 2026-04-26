use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/awt/windows/WKeyboardFocusManagerPeer.getNativeFocusOwner()Ljava/awt/Component;",
    Any
)]
#[async_method]
pub async fn get_native_focus_owner<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WKeyboardFocusManagerPeer.getNativeFocusOwner()Ljava/awt/Component;"
            .to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WKeyboardFocusManagerPeer.getNativeFocusedWindow()Ljava/awt/Window;",
    Any
)]
#[async_method]
pub async fn get_native_focused_window<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WKeyboardFocusManagerPeer.getNativeFocusedWindow()Ljava/awt/Window;"
            .to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WKeyboardFocusManagerPeer.setNativeFocusOwner(Ljava/awt/peer/ComponentPeer;)V",
    Any
)]
#[async_method]
pub async fn set_native_focus_owner<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _comp_peer = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/awt/windows/WKeyboardFocusManagerPeer.setNativeFocusOwner(Ljava/awt/peer/ComponentPeer;)V".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_native_focus_owner() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_native_focus_owner(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WKeyboardFocusManagerPeer.getNativeFocusOwner()Ljava/awt/Component;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_native_focused_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_native_focused_window(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WKeyboardFocusManagerPeer.getNativeFocusedWindow()Ljava/awt/Window;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_native_focus_owner() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            set_native_focus_owner(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WKeyboardFocusManagerPeer.setNativeFocusOwner(Ljava/awt/peer/ComponentPeer;)V",
            result.unwrap_err().to_string()
        );
    }
}
