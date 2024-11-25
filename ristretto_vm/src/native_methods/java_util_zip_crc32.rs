use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_8: Version = Version::Java8 { minor: 0 };

/// Register all native methods for `java.util.zip.CRC32`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/util/zip/CRC32";
    let java_version = registry.java_version();

    if java_version <= &JAVA_8 {
        registry.register(
            class_name,
            "updateByteBuffer",
            "(IJII)I",
            update_byte_buffer,
        );
        registry.register(class_name, "updateBytes", "(I[BII)I", update_bytes);
    } else {
        registry.register(
            class_name,
            "updateByteBuffer0",
            "(IJII)I",
            update_byte_buffer_0,
        );
        registry.register(class_name, "updateBytes0", "(I[BII)I", update_bytes_0);
    }

    registry.register(class_name, "update", "(II)I", update);
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
async fn update_byte_buffer_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn update_bytes(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn update_bytes_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
