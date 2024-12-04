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
    todo!()
}

#[async_recursion(?Send)]
async fn get_surface_data(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn init_custom_raster(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn init_raster(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn set_surface_data(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn sync_to_java_pixels(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn xor_surface_pixels(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
