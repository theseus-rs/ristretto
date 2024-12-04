use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.awt.image.ImageRepresentation`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/awt/image/ImageRepresentation";
    registry.register(class_name, "initIDs", "()V", init_ids);
    registry.register(
        class_name,
        "setDiffICM",
        "(IIII[IIILjava/awt/image/IndexColorModel;[BIILsun/awt/image/ByteComponentRaster;I)Z",
        set_diff_icm,
    );
    registry.register(
        class_name,
        "setICMpixels",
        "(IIII[I[BIILsun/awt/image/IntegerComponentRaster;)Z",
        set_ic_mpixels,
    );
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn set_diff_icm(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn set_ic_mpixels(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
