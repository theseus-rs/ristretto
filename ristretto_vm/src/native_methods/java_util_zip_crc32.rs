use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.util.zip.CRC32`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/util/zip/CRC32";
    registry.register(class_name, "update", "(II)I", update);
    registry.register(
        class_name,
        "updateByteBuffer",
        "(IJII)I",
        update_byte_buffer,
    );
    registry.register(class_name, "updateBytes", "(I[BII)I", update_bytes);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn update(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn update_byte_buffer(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn update_bytes(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
