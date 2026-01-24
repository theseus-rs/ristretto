use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::{Any, GreaterThan, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("java/util/zip/CRC32.update(II)I", Any)]
#[async_method]
pub(crate) async fn update(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.zip.CRC32.update(II)I")
}

#[intrinsic_method("java/util/zip/CRC32.updateByteBuffer(IJII)I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub(crate) async fn update_byte_buffer(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    update_byte_buffer_0(thread.clone(), parameters).await
}

#[intrinsic_method("java/util/zip/CRC32.updateByteBuffer0(IJII)I", GreaterThan(JAVA_8))]
#[async_method]
pub(crate) async fn update_byte_buffer_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.util.zip.CRC32.updateByteBuffer0(IJII)I")
}

#[intrinsic_method("java/util/zip/CRC32.updateBytes(I[BII)I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub(crate) async fn update_bytes(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    update_bytes_0(thread.clone(), parameters).await
}

#[intrinsic_method("java/util/zip/CRC32.updateBytes0(I[BII)I", GreaterThan(JAVA_8))]
#[async_method]
pub(crate) async fn update_bytes_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
