use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/awt/image/ImagingLib.convolveBI(Ljava/awt/image/BufferedImage;Ljava/awt/image/BufferedImage;Ljava/awt/image/Kernel;I)I",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn convolve_bi(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.awt.image.ImagingLib.convolveBI(Ljava/awt/image/BufferedImage;Ljava/awt/image/BufferedImage;Ljava/awt/image/Kernel;I)I"
    )
}

#[intrinsic_method(
    "sun/awt/image/ImagingLib.convolveRaster(Ljava/awt/image/Raster;Ljava/awt/image/Raster;Ljava/awt/image/Kernel;I)I",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn convolve_raster(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.awt.image.ImagingLib.convolveRaster(Ljava/awt/image/Raster;Ljava/awt/image/Raster;Ljava/awt/image/Kernel;I)I"
    )
}

#[intrinsic_method("sun/awt/image/ImagingLib.init()Z", Any)]
#[async_recursion(?Send)]
pub(crate) async fn init(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.awt.image.ImagingLib.init()Z")
}

#[intrinsic_method(
    "sun/awt/image/ImagingLib.lookupByteBI(Ljava/awt/image/BufferedImage;Ljava/awt/image/BufferedImage;[[B)I",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn lookup_byte_bi(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.awt.image.ImagingLib.lookupByteBI(Ljava/awt/image/BufferedImage;Ljava/awt/image/BufferedImage;[[B)I"
    )
}

#[intrinsic_method(
    "sun/awt/image/ImagingLib.lookupByteRaster(Ljava/awt/image/Raster;Ljava/awt/image/Raster;[[B)I",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn lookup_byte_raster(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.awt.image.ImagingLib.lookupByteRaster(Ljava/awt/image/Raster;Ljava/awt/image/Raster;[[B)I"
    )
}

#[intrinsic_method(
    "sun/awt/image/ImagingLib.transformBI(Ljava/awt/image/BufferedImage;Ljava/awt/image/BufferedImage;[DI)I",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn transform_bi(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.awt.image.ImagingLib.transformBI(Ljava/awt/image/BufferedImage;Ljava/awt/image/BufferedImage;[DI)I"
    )
}

#[intrinsic_method(
    "sun/awt/image/ImagingLib.transformRaster(Ljava/awt/image/Raster;Ljava/awt/image/Raster;[DI)I",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn transform_raster(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
