use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/image/ImageRepresentation.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "sun/awt/image/ImageRepresentation.setDiffICM(IIII[IIILjava/awt/image/IndexColorModel;[BIILsun/awt/image/ByteComponentRaster;I)Z",
    Any
)]
#[async_method]
pub async fn set_diff_icm<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.awt.image.ImageRepresentation.setDiffICM(IIII[IIILjava/awt/image/IndexColorModel;[BIILsun/awt/image/ByteComponentRaster;I)Z"
    )
}

#[intrinsic_method(
    "sun/awt/image/ImageRepresentation.setICMpixels(IIII[I[BIILsun/awt/image/IntegerComponentRaster;)Z",
    Any
)]
#[async_method]
pub async fn set_icm_pixels<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.awt.image.ImageRepresentation.setICMpixels(IIII[I[BIILsun/awt/image/IntegerComponentRaster;)Z"
    )
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
        expected = "not yet implemented: sun.awt.image.ImageRepresentation.setDiffICM(IIII[IIILjava/awt/image/IndexColorModel;[BIILsun/awt/image/ByteComponentRaster;I)Z"
    )]
    async fn test_set_diff_icm() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_diff_icm(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.image.ImageRepresentation.setICMpixels(IIII[I[BIILsun/awt/image/IntegerComponentRaster;)Z"
    )]
    async fn test_set_icm_pixels() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_icm_pixels(thread, Parameters::default()).await;
    }
}
