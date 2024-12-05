use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.awt.image.BufImgSurfaceData`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/awt/image/BufImgSurfaceData";
    registry.register(
        class_name,
        "initIDs",
        "(Ljava/lang/Class;Ljava/lang/Class;)V",
        init_ids,
    );
    registry.register(
        class_name,
        "initRaster",
        "(Ljava/lang/Object;IIIIIILjava/awt/image/IndexColorModel;)V",
        init_raster,
    );
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn init_raster(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.image.BufImgSurfaceData.initRaster(Ljava/lang/Object;IIIIIILjava/awt/image/IndexColorModel;)V")
}
