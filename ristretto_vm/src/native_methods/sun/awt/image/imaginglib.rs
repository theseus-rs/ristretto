use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.awt.image.ImagingLib`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/awt/image/ImagingLib";
    registry.register(
        class_name,
        "convolveBI",
        "(Ljava/awt/image/BufferedImage;Ljava/awt/image/BufferedImage;Ljava/awt/image/Kernel;I)I",
        convolve_bi,
    );
    registry.register(
        class_name,
        "convolveRaster",
        "(Ljava/awt/image/Raster;Ljava/awt/image/Raster;Ljava/awt/image/Kernel;I)I",
        convolve_raster,
    );
    registry.register(class_name, "init", "()Z", init);
    registry.register(
        class_name,
        "lookupByteBI",
        "(Ljava/awt/image/BufferedImage;Ljava/awt/image/BufferedImage;[[B)I",
        lookup_byte_bi,
    );
    registry.register(
        class_name,
        "lookupByteRaster",
        "(Ljava/awt/image/Raster;Ljava/awt/image/Raster;[[B)I",
        lookup_byte_raster,
    );
    registry.register(
        class_name,
        "transformBI",
        "(Ljava/awt/image/BufferedImage;Ljava/awt/image/BufferedImage;[DI)I",
        transform_bi,
    );
    registry.register(
        class_name,
        "transformRaster",
        "(Ljava/awt/image/Raster;Ljava/awt/image/Raster;[DI)I",
        transform_raster,
    );
}

#[async_recursion(?Send)]
async fn convolve_bi(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn convolve_raster(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn lookup_byte_bi(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn lookup_byte_raster(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn transform_bi(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn transform_raster(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
