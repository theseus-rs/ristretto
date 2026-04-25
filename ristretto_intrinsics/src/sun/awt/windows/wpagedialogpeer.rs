use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/windows/WPageDialogPeer._show()Z", Any)]
#[async_method]
pub async fn show<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WPageDialogPeer._show()Z".to_string())
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
            "sun/awt/windows/WPageDialogPeer._show()Z",
            result.unwrap_err().to_string()
        );
    }
}
