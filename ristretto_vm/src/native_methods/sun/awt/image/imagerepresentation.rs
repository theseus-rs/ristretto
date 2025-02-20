use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/awt/image/ImageRepresentation";

/// Register all native methods for `sun.awt.image.ImageRepresentation`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "initIDs", "()V", init_ids);
    registry.register(
        CLASS_NAME,
        "setDiffICM",
        "(IIII[IIILjava/awt/image/IndexColorModel;[BIILsun/awt/image/ByteComponentRaster;I)Z",
        set_diff_icm,
    );
    registry.register(
        CLASS_NAME,
        "setICMpixels",
        "(IIII[I[BIILsun/awt/image/IntegerComponentRaster;)Z",
        set_icm_pixels,
    );
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn set_diff_icm(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "sun.awt.image.ImageRepresentation.setDiffICM(IIII[IIILjava/awt/image/IndexColorModel;[BIILsun/awt/image/ByteComponentRaster;I)Z"
    )
}

#[async_recursion(?Send)]
async fn set_icm_pixels(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
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
