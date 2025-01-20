use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/java2d/OSXOffScreenSurfaceData";

/// Register all native methods for `sun.java2d.OSXOffScreenSurfaceData`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "clearSurfacePixels",
        "(II)Z",
        clear_surface_pixels,
    );
    registry.register(
        CLASS_NAME,
        "getSurfaceData",
        "(Ljava/awt/image/BufferedImage;)Lsun/java2d/SurfaceData;",
        get_surface_data,
    );
    registry.register(
        CLASS_NAME,
        "initCustomRaster",
        "(Ljava/nio/IntBuffer;IILjava/nio/ByteBuffer;Ljava/lang/Object;Ljava/nio/ByteBuffer;)V",
        init_custom_raster,
    );
    registry.register(CLASS_NAME, "initIDs", "()V", init_ids);
    registry.register(CLASS_NAME, "initRaster", "(Ljava/lang/Object;IIIIILjava/awt/image/IndexColorModel;ILjava/nio/ByteBuffer;Ljava/lang/Object;Ljava/nio/ByteBuffer;)V", init_raster);
    registry.register(
        CLASS_NAME,
        "setSurfaceData",
        "(Ljava/awt/image/BufferedImage;Lsun/java2d/SurfaceData;)V",
        set_surface_data,
    );
    registry.register(CLASS_NAME, "syncToJavaPixels", "()V", sync_to_java_pixels);
    registry.register(
        CLASS_NAME,
        "xorSurfacePixels",
        "(Lsun/java2d/SurfaceData;IIIII)Z",
        xor_surface_pixels,
    );
}

#[async_recursion(?Send)]
async fn clear_surface_pixels(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.OSXOffScreenSurfaceData.clearSurfacePixels(IIZ)Z")
}

#[async_recursion(?Send)]
async fn get_surface_data(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.OSXOffScreenSurfaceData.getSurfaceData(Ljava/awt/image/BufferedImage;)Lsun/java2d/SurfaceData;")
}

#[async_recursion(?Send)]
async fn init_custom_raster(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.OSXOffScreenSurfaceData.initCustomRaster(Ljava/nio/IntBuffer;IILjava/nio/ByteBuffer;Ljava/lang/Object;Ljava/nio/ByteBuffer;)V")
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn init_raster(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.OSXOffScreenSurfaceData.initRaster(Ljava/lang/Object;IIIIILjava/awt/image/IndexColorModel;ILjava/nio/ByteBuffer;Ljava/lang/Object;Ljava/nio/ByteBuffer;)V")
}

#[async_recursion(?Send)]
async fn set_surface_data(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.OSXOffScreenSurfaceData.setSurfaceData(Ljava/awt/image/BufferedImage;Lsun/java2d/SurfaceData;)V")
}

#[async_recursion(?Send)]
async fn sync_to_java_pixels(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.OSXOffScreenSurfaceData.syncToJavaPixels()V")
}

#[async_recursion(?Send)]
async fn xor_surface_pixels(
    _thread: Arc<Thread>,
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
