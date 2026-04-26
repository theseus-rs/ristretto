use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/windows/WPopupMenuPeer._show(Ljava/awt/Event;)V", Any)]
#[async_method]
pub async fn show<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WPopupMenuPeer._show(Ljava/awt/Event;)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WPopupMenuPeer.createMenu(Lsun/awt/windows/WComponentPeer;)V",
    Any
)]
#[async_method]
pub async fn create_menu<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _parent = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WPopupMenuPeer.createMenu(Lsun/awt/windows/WComponentPeer;)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_show() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = show(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WPopupMenuPeer._show(Ljava/awt/Event;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_create_menu() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_menu(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WPopupMenuPeer.createMenu(Lsun/awt/windows/WComponentPeer;)V",
            result.unwrap_err().to_string()
        );
    }
}
