use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/util/zip/Adler32";

/// Register all intrinsic methods for `java.util.zip.Adler32`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "update", "(II)I", update);
    registry.register(
        CLASS_NAME,
        "updateByteBuffer",
        "(IJII)I",
        update_byte_buffer,
    );
    registry.register(CLASS_NAME, "updateBytes", "(I[BII)I", update_bytes);
}

#[async_recursion(?Send)]
async fn update(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.zip.Adler32.update(II)I")
}

#[async_recursion(?Send)]
async fn update_byte_buffer(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.util.zip.Adler32.updateByteBuffer(IJII)I")
}

#[async_recursion(?Send)]
async fn update_bytes(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.zip.Adler32.updateBytes(I[BII)I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.Adler32.update(II)I")]
    async fn test_update() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = update(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.Adler32.updateByteBuffer(IJII)I")]
    async fn test_update_byte_buffer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = update_byte_buffer(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.Adler32.updateBytes(I[BII)I")]
    async fn test_update_bytes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = update_bytes(thread, Parameters::default()).await;
    }
}
