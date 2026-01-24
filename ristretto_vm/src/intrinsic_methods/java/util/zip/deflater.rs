use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::{Any, GreaterThan, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "java/util/zip/Deflater.deflateBytes(J[BIII)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn deflate_bytes(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.util.zip.Deflater.deflateBytes(J[BIII)I")
}

#[intrinsic_method(
    "java/util/zip/Deflater.deflateBufferBuffer(JJIJIII)J",
    GreaterThan(JAVA_8)
)]
#[async_method]
pub(crate) async fn deflate_buffer_buffer(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.util.zip.Deflater.deflateBufferBuffer(JJIJIII)J")
}

#[intrinsic_method(
    "java/util/zip/Deflater.deflateBufferBytes(JJI[BIIII)J",
    GreaterThan(JAVA_8)
)]
#[async_method]
pub(crate) async fn deflate_buffer_bytes(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.util.zip.Deflater.deflateBufferBytes(JJI[BIIII)J")
}

#[intrinsic_method(
    "java/util/zip/Deflater.deflateBytesBuffer(J[BIIJIII)J",
    GreaterThan(JAVA_8)
)]
#[async_method]
pub(crate) async fn deflate_bytes_buffer(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.util.zip.Deflater.deflateBytesBuffer(J[BIIJIII)J")
}

#[intrinsic_method(
    "java/util/zip/Deflater.deflateBytesBytes(J[BII[BIIII)J",
    GreaterThan(JAVA_8)
)]
#[async_method]
pub(crate) async fn deflate_bytes_bytes(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.util.zip.Deflater.deflateBytesBytes(J[BII[BIIII)J")
}

#[intrinsic_method("java/util/zip/Deflater.end(J)V", Any)]
#[async_method]
pub(crate) async fn end(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.zip.Deflater.end(J)V")
}

#[intrinsic_method("java/util/zip/Deflater.getAdler(J)I", Any)]
#[async_method]
pub(crate) async fn get_adler(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.util.zip.Deflater.getAdler(J)I")
}

#[intrinsic_method("java/util/zip/Deflater.init(IIZ)J", Any)]
#[async_method]
pub(crate) async fn init(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.zip.Deflater.init(IIZ)J")
}

#[intrinsic_method("java/util/zip/Deflater.initIDs()V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub(crate) async fn init_ids(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/util/zip/Deflater.reset(J)V", Any)]
#[async_method]
pub(crate) async fn reset(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.zip.Deflater.reset(J)V")
}

#[intrinsic_method("java/util/zip/Deflater.setDictionary(J[BII)V", Any)]
#[async_method]
pub(crate) async fn set_dictionary(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.util.zip.Deflater.setDictionary(J[BII)V")
}

#[intrinsic_method(
    "java/util/zip/Deflater.setDictionaryBuffer(JJI)V",
    GreaterThan(JAVA_8)
)]
#[async_method]
pub(crate) async fn set_dictionary_buffer(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.util.zip.Deflater.setDictionaryBuffer(JJI)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.Deflater.deflateBytes(J[BIII)I")]
    async fn test_deflate_bytes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = deflate_bytes(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.zip.Deflater.deflateBufferBuffer(JJIJIII)J"
    )]
    async fn test_deflate_buffer_buffer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = deflate_buffer_buffer(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.zip.Deflater.deflateBufferBytes(JJI[BIIII)J"
    )]
    async fn test_deflate_buffer_bytes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = deflate_buffer_bytes(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.zip.Deflater.deflateBytesBuffer(J[BIIJIII)J"
    )]
    async fn test_deflate_bytes_buffer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = deflate_bytes_buffer(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.zip.Deflater.deflateBytesBytes(J[BII[BIIII)J"
    )]
    async fn test_deflate_bytes_bytes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = deflate_bytes_bytes(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.Deflater.end(J)V")]
    async fn test_end() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = end(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.Deflater.getAdler(J)I")]
    async fn test_get_adler() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_adler(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.Deflater.init(IIZ)J")]
    async fn test_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.Deflater.reset(J)V")]
    async fn test_reset() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = reset(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.Deflater.setDictionary(J[BII)V")]
    async fn test_set_dictionary() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_dictionary(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.zip.Deflater.setDictionaryBuffer(JJI)V"
    )]
    async fn test_set_dictionary_buffer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_dictionary_buffer(thread, Parameters::default()).await;
    }
}
