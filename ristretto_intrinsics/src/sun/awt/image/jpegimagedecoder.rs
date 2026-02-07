use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/image/JPEGImageDecoder.initIDs(Ljava/lang/Class;)V", Any)]
#[async_method]
pub async fn init_ids<T: ristretto_types::Thread + 'static>(
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
pub async fn read_image<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.image.JPEGImageDecoder.readImage(Ljava/io/InputStream;[B)V")
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
        expected = "not yet implemented: sun.awt.image.JPEGImageDecoder.readImage(Ljava/io/InputStream;[B)V"
    )]
    async fn test_read_image() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = read_image(thread, Parameters::default()).await;
    }
}
