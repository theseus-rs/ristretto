use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/java2d/OSXOffScreenSurfaceData.clearSurfacePixels(II)Z", Any)]
#[async_method]
pub async fn clear_surface_pixels<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.OSXOffScreenSurfaceData.clearSurfacePixels(IIZ)Z")
}

#[intrinsic_method(
    "sun/java2d/OSXOffScreenSurfaceData.getSurfaceData(Ljava/awt/image/BufferedImage;)Lsun/java2d/SurfaceData;",
    Any
)]
#[async_method]
pub async fn get_surface_data<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.java2d.OSXOffScreenSurfaceData.getSurfaceData(Ljava/awt/image/BufferedImage;)Lsun/java2d/SurfaceData;"
    )
}

#[intrinsic_method(
    "sun/java2d/OSXOffScreenSurfaceData.initCustomRaster(Ljava/nio/IntBuffer;IILjava/nio/ByteBuffer;Ljava/lang/Object;Ljava/nio/ByteBuffer;)V",
    Any
)]
#[async_method]
pub async fn init_custom_raster<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.java2d.OSXOffScreenSurfaceData.initCustomRaster(Ljava/nio/IntBuffer;IILjava/nio/ByteBuffer;Ljava/lang/Object;Ljava/nio/ByteBuffer;)V"
    )
}

#[intrinsic_method("sun/java2d/OSXOffScreenSurfaceData.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "sun/java2d/OSXOffScreenSurfaceData.initRaster(Ljava/lang/Object;IIIIILjava/awt/image/IndexColorModel;ILjava/nio/ByteBuffer;Ljava/lang/Object;Ljava/nio/ByteBuffer;)V",
    Any
)]
#[async_method]
pub async fn init_raster<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.java2d.OSXOffScreenSurfaceData.initRaster(Ljava/lang/Object;IIIIILjava/awt/image/IndexColorModel;ILjava/nio/ByteBuffer;Ljava/lang/Object;Ljava/nio/ByteBuffer;)V"
    )
}

#[intrinsic_method(
    "sun/java2d/OSXOffScreenSurfaceData.setSurfaceData(Ljava/awt/image/BufferedImage;Lsun/java2d/SurfaceData;)V",
    Any
)]
#[async_method]
pub async fn set_surface_data<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.java2d.OSXOffScreenSurfaceData.setSurfaceData(Ljava/awt/image/BufferedImage;Lsun/java2d/SurfaceData;)V"
    )
}

#[intrinsic_method("sun/java2d/OSXOffScreenSurfaceData.syncToJavaPixels()V", Any)]
#[async_method]
pub async fn sync_to_java_pixels<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.OSXOffScreenSurfaceData.syncToJavaPixels()V")
}

#[intrinsic_method(
    "sun/java2d/OSXOffScreenSurfaceData.xorSurfacePixels(Lsun/java2d/SurfaceData;IIIII)Z",
    Any
)]
#[async_method]
pub async fn xor_surface_pixels<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.OSXOffScreenSurfaceData.xorSurfacePixels(Lsun/java2d/SurfaceData;IIIII)Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.OSXOffScreenSurfaceData.clearSurfacePixels(IIZ)Z"
    )]
    async fn test_clear_surface_pixels() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = clear_surface_pixels(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.OSXOffScreenSurfaceData.getSurfaceData(Ljava/awt/image/BufferedImage;)Lsun/java2d/SurfaceData;"
    )]
    async fn test_get_surface_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_surface_data(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.OSXOffScreenSurfaceData.initCustomRaster(Ljava/nio/IntBuffer;IILjava/nio/ByteBuffer;Ljava/lang/Object;Ljava/nio/ByteBuffer;)V"
    )]
    async fn test_init_custom_raster() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_custom_raster(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.OSXOffScreenSurfaceData.initRaster(Ljava/lang/Object;IIIIILjava/awt/image/IndexColorModel;ILjava/nio/ByteBuffer;Ljava/lang/Object;Ljava/nio/ByteBuffer;)V"
    )]
    async fn test_init_raster() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_raster(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.OSXOffScreenSurfaceData.setSurfaceData(Ljava/awt/image/BufferedImage;Lsun/java2d/SurfaceData;)V"
    )]
    async fn test_set_surface_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_surface_data(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.OSXOffScreenSurfaceData.syncToJavaPixels()V"
    )]
    async fn test_sync_to_java_pixels() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = sync_to_java_pixels(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.OSXOffScreenSurfaceData.xorSurfacePixels(Lsun/java2d/SurfaceData;IIIII)Z"
    )]
    async fn test_xor_surface_pixels() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = xor_surface_pixels(thread, Parameters::default()).await;
    }
}
