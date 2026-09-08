use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("java/awt/AWTEvent.initIDs()V", Any)]
pub fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "java/awt/AWTEvent.nativeSetSource(Ljava/awt/peer/ComponentPeer;)V",
    Any
)]
pub fn native_set_source<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _peer = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java.awt.AWTEvent.nativeSetSource(Ljava/awt/peer/ComponentPeer;)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default())?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_native_set_source() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_set_source(thread, Parameters::new(vec![Value::Object(None)]));
        assert_eq!(
            "java.awt.AWTEvent.nativeSetSource(Ljava/awt/peer/ComponentPeer;)V",
            result.unwrap_err().to_string()
        );
    }
}
