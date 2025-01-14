use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.java2d.OSXOffScreenSurfaceData`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/OSXOffScreenSurfaceData";
    registry.register(
        class_name,
        "clearSurfacePixels",
        "(II)Z",
        clear_surface_pixels,
    );
    registry.register(
        class_name,
        "getSurfaceData",
        "(Ljava/awt/image/BufferedImage;)Lsun/java2d/SurfaceData;",
        get_surface_data,
    );
    registry.register(
        class_name,
        "initCustomRaster",
        "(Ljava/nio/IntBuffer;IILjava/nio/ByteBuffer;Ljava/lang/Object;Ljava/nio/ByteBuffer;)V",
        init_custom_raster,
    );
    registry.register(class_name, "initIDs", "()V", init_ids);
    registry.register(class_name, "initRaster", "(Ljava/lang/Object;IIIIILjava/awt/image/IndexColorModel;ILjava/nio/ByteBuffer;Ljava/lang/Object;Ljava/nio/ByteBuffer;)V", init_raster);
    registry.register(
        class_name,
        "setSurfaceData",
        "(Ljava/awt/image/BufferedImage;Lsun/java2d/SurfaceData;)V",
        set_surface_data,
    );
    registry.register(class_name, "syncToJavaPixels", "()V", sync_to_java_pixels);
    registry.register(
        class_name,
        "xorSurfacePixels",
        "(Lsun/java2d/SurfaceData;IIIII)Z",
        xor_surface_pixels,
    );
}

#[async_recursion(?Send)]
async fn clear_surface_pixels(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.java2d.OSXOffScreenSurfaceData.clearSurfacePixels(IIZ)Z")
}

#[async_recursion(?Send)]
async fn get_surface_data(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.OSXOffScreenSurfaceData.getSurfaceData(Ljava/awt/image/BufferedImage;)Lsun/java2d/SurfaceData;")
}

#[async_recursion(?Send)]
async fn init_custom_raster(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.OSXOffScreenSurfaceData.initCustomRaster(Ljava/nio/IntBuffer;IILjava/nio/ByteBuffer;Ljava/lang/Object;Ljava/nio/ByteBuffer;)V")
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn init_raster(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.OSXOffScreenSurfaceData.initRaster(Ljava/lang/Object;IIIIILjava/awt/image/IndexColorModel;ILjava/nio/ByteBuffer;Ljava/lang/Object;Ljava/nio/ByteBuffer;)V")
}

#[async_recursion(?Send)]
async fn set_surface_data(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.OSXOffScreenSurfaceData.setSurfaceData(Ljava/awt/image/BufferedImage;Lsun/java2d/SurfaceData;)V")
}

#[async_recursion(?Send)]
async fn sync_to_java_pixels(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.OSXOffScreenSurfaceData.syncToJavaPixels()V")
}

#[async_recursion(?Send)]
async fn xor_surface_pixels(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.OSXOffScreenSurfaceData.xorSurfacePixels(Lsun/java2d/SurfaceData;IIIII)Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/java2d/OSXOffScreenSurfaceData";
        assert!(registry
            .method(class_name, "clearSurfacePixels", "(II)Z")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "getSurfaceData",
                "(Ljava/awt/image/BufferedImage;)Lsun/java2d/SurfaceData;"
            )
            .is_some());
        assert!(registry.method(class_name, "initCustomRaster", "(Ljava/nio/IntBuffer;IILjava/nio/ByteBuffer;Ljava/lang/Object;Ljava/nio/ByteBuffer;)V").is_some());
        assert!(registry.method(class_name, "initIDs", "()V").is_some());
        assert!(registry.method(class_name, "initRaster", "(Ljava/lang/Object;IIIIILjava/awt/image/IndexColorModel;ILjava/nio/ByteBuffer;Ljava/lang/Object;Ljava/nio/ByteBuffer;)V").is_some());
        assert!(registry
            .method(
                class_name,
                "setSurfaceData",
                "(Ljava/awt/image/BufferedImage;Lsun/java2d/SurfaceData;)V"
            )
            .is_some());
        assert!(registry
            .method(class_name, "syncToJavaPixels", "()V")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "xorSurfacePixels",
                "(Lsun/java2d/SurfaceData;IIIII)Z"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.OSXOffScreenSurfaceData.clearSurfacePixels(IIZ)Z")]
    async fn test_clear_surface_pixels() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = clear_surface_pixels(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.java2d.OSXOffScreenSurfaceData.getSurfaceData(Ljava/awt/image/BufferedImage;)Lsun/java2d/SurfaceData;"
    )]
    async fn test_get_surface_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_surface_data(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.java2d.OSXOffScreenSurfaceData.initCustomRaster(Ljava/nio/IntBuffer;IILjava/nio/ByteBuffer;Ljava/lang/Object;Ljava/nio/ByteBuffer;)V"
    )]
    async fn test_init_custom_raster() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_custom_raster(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.java2d.OSXOffScreenSurfaceData.initRaster(Ljava/lang/Object;IIIIILjava/awt/image/IndexColorModel;ILjava/nio/ByteBuffer;Ljava/lang/Object;Ljava/nio/ByteBuffer;)V"
    )]
    async fn test_init_raster() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_raster(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.java2d.OSXOffScreenSurfaceData.setSurfaceData(Ljava/awt/image/BufferedImage;Lsun/java2d/SurfaceData;)V"
    )]
    async fn test_set_surface_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_surface_data(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.OSXOffScreenSurfaceData.syncToJavaPixels()V")]
    async fn test_sync_to_java_pixels() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = sync_to_java_pixels(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.java2d.OSXOffScreenSurfaceData.xorSurfacePixels(Lsun/java2d/SurfaceData;IIIII)Z"
    )]
    async fn test_xor_surface_pixels() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = xor_surface_pixels(thread, Arguments::default()).await;
    }
}
