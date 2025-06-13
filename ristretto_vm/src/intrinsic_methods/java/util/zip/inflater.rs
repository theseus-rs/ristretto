use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::{Any, GreaterThan, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("java/util/zip/Inflater.end(J)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn end(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.zip.Inflater.end(J)V")
}

#[intrinsic_method("java/util/zip/Inflater.getAdler(J)I", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_adler(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.util.zip.Inflater.getAdler(J)I")
}

#[intrinsic_method(
    "java/util/zip/Inflater.inflateBufferBuffer(JJIJI)J",
    GreaterThan(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn inflate_buffer_buffer(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.util.zip.Inflater.inflateBufferBuffer(JJIJI)J")
}

#[intrinsic_method(
    "java/util/zip/Inflater.inflateBufferBytes(JJI[BII)J",
    GreaterThan(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn inflate_buffer_bytes(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.util.zip.Inflater.inflateBufferBytes(JJI[BII)J")
}

#[intrinsic_method(
    "java/util/zip/Inflater.inflateBytesBuffer(J[BIIJI)J",
    GreaterThan(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn inflate_bytes_buffer(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.util.zip.Inflater.inflateBytesBuffer(J[BIIJI)J")
}

#[intrinsic_method("java/util/zip/Inflater.inflateBytes(J[BII)I", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn inflate_bytes(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.util.zip.Inflater.inflateBytes(J[BII)I")
}

#[intrinsic_method(
    "java/util/zip/Inflater.inflateBytesBytes(J[BII[BII)J",
    GreaterThan(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn inflate_bytes_bytes(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.util.zip.Inflater.inflateBytesBytes(J[BII[BII)J")
}

#[intrinsic_method("java/util/zip/Inflater.init(Z)J", Any)]
#[async_recursion(?Send)]
pub(crate) async fn init(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.zip.Inflater.init(Z)J")
}

#[intrinsic_method("java/util/zip/Inflater.initIDs()V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn init_ids(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/util/zip/Inflater.reset(J)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn reset(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.zip.Inflater.reset(J)V")
}

#[intrinsic_method("java/util/zip/Inflater.setDictionary(J[BII)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn set_dictionary(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.util.zip.Inflater.setDictionary(J[BII)V")
}

#[intrinsic_method(
    "java/util/zip/Inflater.setDictionaryBuffer(JJI)V",
    GreaterThan(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn set_dictionary_buffer(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.util.zip.Inflater.setDictionaryBuffer(JJI)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.Inflater.end(J)V")]
    async fn test_end() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = end(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.Inflater.getAdler(J)I")]
    async fn test_get_adler() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_adler(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.zip.Inflater.inflateBufferBuffer(JJIJI)J"
    )]
    async fn test_inflate_buffer_buffer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = inflate_buffer_buffer(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.zip.Inflater.inflateBufferBytes(JJI[BII)J"
    )]
    async fn test_inflate_buffer_bytes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = inflate_buffer_bytes(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.zip.Inflater.inflateBytesBuffer(J[BIIJI)J"
    )]
    async fn test_inflate_bytes_buffer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = inflate_bytes_buffer(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.Inflater.inflateBytes(J[BII)I")]
    async fn test_inflate_bytes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = inflate_bytes(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.zip.Inflater.inflateBytesBytes(J[BII[BII)J"
    )]
    async fn test_inflate_bytes_bytes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = inflate_bytes_bytes(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.Inflater.init(Z)J")]
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
    #[should_panic(expected = "not yet implemented: java.util.zip.Inflater.reset(J)V")]
    async fn test_reset() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = reset(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.Inflater.setDictionary(J[BII)V")]
    async fn test_set_dictionary() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_dictionary(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.zip.Inflater.setDictionaryBuffer(JJI)V"
    )]
    async fn test_set_dictionary_buffer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_dictionary_buffer(thread, Parameters::default()).await;
    }
}
