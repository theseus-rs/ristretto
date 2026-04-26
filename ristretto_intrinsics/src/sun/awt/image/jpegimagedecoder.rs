use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/image/JPEGImageDecoder.initIDs(Ljava/lang/Class;)V", Any)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "sun/awt/image/JPEGImageDecoder.readImage(Ljava/io/InputStream;[B)V",
    Any
)]
#[async_method]
pub async fn read_image<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _buf = parameters.pop_reference()?;
    let _is = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.image.JPEGImageDecoder.readImage(Ljava/io/InputStream;[B)V".to_string(),
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
    async fn test_read_image() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = read_image(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.awt.image.JPEGImageDecoder.readImage(Ljava/io/InputStream;[B)V",
            result.unwrap_err().to_string()
        );
    }
}
