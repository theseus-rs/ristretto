use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/awt/image/ImagingLib";

/// Register all native methods for `sun.awt.image.ImagingLib`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "convolveBI",
        "(Ljava/awt/image/BufferedImage;Ljava/awt/image/BufferedImage;Ljava/awt/image/Kernel;I)I",
        convolve_bi,
    );
    registry.register(
        CLASS_NAME,
        "convolveRaster",
        "(Ljava/awt/image/Raster;Ljava/awt/image/Raster;Ljava/awt/image/Kernel;I)I",
        convolve_raster,
    );
    registry.register(CLASS_NAME, "init", "()Z", init);
    registry.register(
        CLASS_NAME,
        "lookupByteBI",
        "(Ljava/awt/image/BufferedImage;Ljava/awt/image/BufferedImage;[[B)I",
        lookup_byte_bi,
    );
    registry.register(
        CLASS_NAME,
        "lookupByteRaster",
        "(Ljava/awt/image/Raster;Ljava/awt/image/Raster;[[B)I",
        lookup_byte_raster,
    );
    registry.register(
        CLASS_NAME,
        "transformBI",
        "(Ljava/awt/image/BufferedImage;Ljava/awt/image/BufferedImage;[DI)I",
        transform_bi,
    );
    registry.register(
        CLASS_NAME,
        "transformRaster",
        "(Ljava/awt/image/Raster;Ljava/awt/image/Raster;[DI)I",
        transform_raster,
    );
}

#[async_recursion(?Send)]
async fn convolve_bi(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "sun.awt.image.ImagingLib.convolveBI(Ljava/awt/image/BufferedImage;Ljava/awt/image/BufferedImage;Ljava/awt/image/Kernel;I)I"
    )
}

#[async_recursion(?Send)]
async fn convolve_raster(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "sun.awt.image.ImagingLib.convolveRaster(Ljava/awt/image/Raster;Ljava/awt/image/Raster;Ljava/awt/image/Kernel;I)I"
    )
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.awt.image.ImagingLib.init()Z")
}

#[async_recursion(?Send)]
async fn lookup_byte_bi(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "sun.awt.image.ImagingLib.lookupByteBI(Ljava/awt/image/BufferedImage;Ljava/awt/image/BufferedImage;[[B)I"
    )
}

#[async_recursion(?Send)]
async fn lookup_byte_raster(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.awt.image.ImagingLib.lookupByteRaster(Ljava/awt/image/Raster;Ljava/awt/image/Raster;[[B)I"
    )
}

#[async_recursion(?Send)]
async fn transform_bi(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "sun.awt.image.ImagingLib.transformBI(Ljava/awt/image/BufferedImage;Ljava/awt/image/BufferedImage;[DI)I"
    )
}

#[async_recursion(?Send)]
async fn transform_raster(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "sun.awt.image.ImagingLib.transformRaster(Ljava/awt/image/Raster;Ljava/awt/image/Raster;[DI)I"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.image.ImagingLib.convolveBI(Ljava/awt/image/BufferedImage;Ljava/awt/image/BufferedImage;Ljava/awt/image/Kernel;I)I"
    )]
    async fn test_convolve_bi() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = convolve_bi(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.image.ImagingLib.convolveRaster(Ljava/awt/image/Raster;Ljava/awt/image/Raster;Ljava/awt/image/Kernel;I)I"
    )]
    async fn test_convolve_raster() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = convolve_raster(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.awt.image.ImagingLib.init()Z")]
    async fn test_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.image.ImagingLib.lookupByteBI(Ljava/awt/image/BufferedImage;Ljava/awt/image/BufferedImage;[[B)I"
    )]
    async fn test_lookup_byte_bi() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = lookup_byte_bi(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.image.ImagingLib.lookupByteRaster(Ljava/awt/image/Raster;Ljava/awt/image/Raster;[[B)I"
    )]
    async fn test_lookup_byte_raster() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = lookup_byte_raster(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.image.ImagingLib.transformBI(Ljava/awt/image/BufferedImage;Ljava/awt/image/BufferedImage;[DI)I"
    )]
    async fn test_transform_bi() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = transform_bi(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.image.ImagingLib.transformRaster(Ljava/awt/image/Raster;Ljava/awt/image/Raster;[DI)I"
    )]
    async fn test_transform_raster() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = transform_raster(thread, Parameters::default()).await;
    }
}
