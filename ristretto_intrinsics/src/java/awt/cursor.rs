use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("java/awt/Cursor.finalizeImpl(J)V", Any)]
#[async_method]
pub async fn finalize_impl<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("java.awt.Cursor.finalizeImpl(J)V".to_string()).into())
}

#[intrinsic_method("java/awt/Cursor.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_finalize_impl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = finalize_impl(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "java.awt.Cursor.finalizeImpl(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
