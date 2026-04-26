use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/java2d/SurfaceData.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "sun/java2d/SurfaceData.isOpaqueGray(Ljava/awt/image/IndexColorModel;)Z",
    Any
)]
#[async_method]
pub async fn is_opaque_gray<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _icm = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.SurfaceData.isOpaqueGray(Ljava/awt/image/IndexColorModel;)Z".to_string(),
    )
    .into())
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
    async fn test_is_opaque_gray() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_opaque_gray(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun.java2d.SurfaceData.isOpaqueGray(Ljava/awt/image/IndexColorModel;)Z",
            result.unwrap_err().to_string()
        );
    }
}
