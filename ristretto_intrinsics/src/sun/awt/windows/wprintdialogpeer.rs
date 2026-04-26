use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/windows/WPrintDialogPeer._show()Z", Any)]
#[async_method]
pub async fn show<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WPrintDialogPeer._show()Z".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WPrintDialogPeer.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WPrintDialogPeer.initIDs()V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WPrintDialogPeer.toBack()V", Any)]
#[async_method]
pub async fn to_back<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WPrintDialogPeer.toBack()V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WPrintDialogPeer.toFront()V", Any)]
#[async_method]
pub async fn to_front<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WPrintDialogPeer.toFront()V".to_string())
            .into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_show() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = show(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WPrintDialogPeer._show()Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_ids() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WPrintDialogPeer.initIDs()V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_to_back() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = to_back(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WPrintDialogPeer.toBack()V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_to_front() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = to_front(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WPrintDialogPeer.toFront()V",
            result.unwrap_err().to_string()
        );
    }
}
