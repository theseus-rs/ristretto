use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/awt/image/BufImgSurfaceData.initIDs(Ljava/lang/Class;Ljava/lang/Class;)V",
    Any
)]
#[async_method]
pub(crate) async fn init_ids(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "sun/awt/image/BufImgSurfaceData.initRaster(Ljava/lang/Object;IIIIIILjava/awt/image/IndexColorModel;)V",
    Any
)]
#[async_method]
pub(crate) async fn init_raster(
    _thread: Arc<Thread>,
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
