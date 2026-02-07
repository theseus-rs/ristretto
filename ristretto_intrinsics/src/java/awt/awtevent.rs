use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("java/awt/AWTEvent.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "java/awt/AWTEvent.nativeSetSource(Ljava/awt/peer/ComponentPeer;)V",
    Any
)]
#[async_method]
pub async fn native_set_source<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.awt.AWTEvent.nativeSetSource(Ljava/awt/peer/ComponentPeer;)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.awt.AWTEvent.nativeSetSource(Ljava/awt/peer/ComponentPeer;)V"
    )]
    async fn test_native_set_source() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_source(thread, Parameters::default()).await;
    }
}
