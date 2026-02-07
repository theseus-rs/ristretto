use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/image/GifImageDecoder.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "sun/awt/image/GifImageDecoder.parseImage(IIIIZI[B[BLjava/awt/image/IndexColorModel;)Z",
    Any
)]
#[async_method]
pub async fn parse_image<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.image.GifImageDecoder.parseImage(IIIIZI[B[BLjava/awt/image/IndexColorModel;)Z")
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
        expected = "not yet implemented: sun.awt.image.GifImageDecoder.parseImage(IIIIZI[B[BLjava/awt/image/IndexColorModel;)Z"
    )]
    async fn test_parse_image() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = parse_image(thread, Parameters::default()).await;
    }
}
