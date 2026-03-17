use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/lwawt/macosx/CDropTargetContextPeer.addTransfer(JJJ)V", Any)]
#[async_method]
pub async fn add_transfer<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CDropTargetContextPeer.addTransfer(JJJ)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CDropTargetContextPeer.dropDone(JJZZI)V", Any)]
#[async_method]
pub async fn drop_done<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CDropTargetContextPeer.dropDone(JJZZI)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CDropTargetContextPeer.startTransfer(JJ)J", Any)]
#[async_method]
pub async fn start_transfer<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CDropTargetContextPeer.startTransfer(JJ)J".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_transfer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = add_transfer(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_drop_done() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = drop_done(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_start_transfer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = start_transfer(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
