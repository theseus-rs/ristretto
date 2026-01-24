use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("java/util/zip/Adler32.update(II)I", Any)]
#[async_method]
pub(crate) async fn update(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.zip.Adler32.update(II)I")
}

#[intrinsic_method("java/util/zip/Adler32.updateByteBuffer(IJII)I", Any)]
#[async_method]
pub(crate) async fn update_byte_buffer(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.util.zip.Adler32.updateByteBuffer(IJII)I")
}

#[intrinsic_method("java/util/zip/Adler32.updateBytes(I[BII)I", Any)]
#[async_method]
pub(crate) async fn update_bytes(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
