use crate::Result;
use crate::intrinsic_methods::registry::{JAVA_8, MethodRegistry};
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/util/zip/CRC32";

/// Register all intrinsic methods for `java.util.zip.CRC32`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_8 {
        registry.register(
            CLASS_NAME,
            "updateByteBuffer",
            "(IJII)I",
            update_byte_buffer,
        );
        registry.register(CLASS_NAME, "updateBytes", "(I[BII)I", update_bytes);
    } else {
        registry.register(
            CLASS_NAME,
            "updateByteBuffer0",
            "(IJII)I",
            update_byte_buffer_0,
        );
        registry.register(CLASS_NAME, "updateBytes0", "(I[BII)I", update_bytes_0);
    }

    registry.register(CLASS_NAME, "update", "(II)I", update);
}

#[async_recursion(?Send)]
async fn update(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.zip.CRC32.update(II)I")
}

#[async_recursion(?Send)]
async fn update_byte_buffer(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    update_byte_buffer_0(thread.clone(), parameters).await
}

#[async_recursion(?Send)]
async fn update_byte_buffer_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.util.zip.CRC32.updateByteBuffer0(IJII)I")
}

#[async_recursion(?Send)]
async fn update_bytes(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    update_bytes_0(thread.clone(), parameters).await
}

#[async_recursion(?Send)]
async fn update_bytes_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.zip.CRC32.updateBytes0(I[BII)I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.CRC32.update(II)I")]
    async fn test_update() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = update(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.CRC32.updateByteBuffer0(IJII)I")]
    async fn test_update_byte_buffer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = update_byte_buffer(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.CRC32.updateByteBuffer0(IJII)I")]
    async fn test_update_byte_buffer_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = update_byte_buffer_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.CRC32.updateBytes0(I[BII)I")]
    async fn test_update_bytes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = update_bytes(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.CRC32.updateBytes0(I[BII)I")]
    async fn test_update_bytes_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = update_bytes_0(thread, Parameters::default()).await;
    }
}
