use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/awt/image/BufImgSurfaceData.initIDs(Ljava/lang/Class;Ljava/lang/Class;)V",
    Any
)]
#[async_method]
pub async fn init_ids<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "sun/awt/image/BufImgSurfaceData.initRaster(Ljava/lang/Object;IIIIIILjava/awt/image/IndexColorModel;)V",
    Any
)]
#[async_method]
pub async fn init_raster<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.awt.image.BufImgSurfaceData.initRaster(Ljava/lang/Object;IIIIIILjava/awt/image/IndexColorModel;)V"
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
        expected = "not yet implemented: sun.awt.image.BufImgSurfaceData.initRaster(Ljava/lang/Object;IIIIIILjava/awt/image/IndexColorModel;)V"
    )]
    async fn test_init_raster() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_raster(thread, Parameters::default()).await;
    }
}
