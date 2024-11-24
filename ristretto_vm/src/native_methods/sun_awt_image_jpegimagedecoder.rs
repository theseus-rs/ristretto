use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.awt.image.JPEGImageDecoder`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/awt/image/JPEGImageDecoder";
    registry.register(class_name, "initIDs", "(Ljava/lang/Class;)V", init_ids);
    registry.register(
        class_name,
        "readImage",
        "(Ljava/io/InputStream;[B)V",
        read_image,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn read_image(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
