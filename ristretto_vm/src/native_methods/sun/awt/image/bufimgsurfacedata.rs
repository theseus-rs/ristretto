use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/awt/image/BufImgSurfaceData";

/// Register all native methods for `sun.awt.image.BufImgSurfaceData`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "initIDs",
        "(Ljava/lang/Class;Ljava/lang/Class;)V",
        init_ids,
    );
    registry.register(
        CLASS_NAME,
        "initRaster",
        "(Ljava/lang/Object;IIIIIILjava/awt/image/IndexColorModel;)V",
        init_raster,
    );
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn init_raster(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
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
