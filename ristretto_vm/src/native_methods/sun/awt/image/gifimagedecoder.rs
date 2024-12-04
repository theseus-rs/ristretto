use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.awt.image.GifImageDecoder`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/awt/image/GifImageDecoder";
    registry.register(class_name, "initIDs", "()V", init_ids);
    registry.register(
        class_name,
        "parseImage",
        "(IIIIZI[B[BLjava/awt/image/IndexColorModel;)Z",
        parse_image,
    );
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn parse_image(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
