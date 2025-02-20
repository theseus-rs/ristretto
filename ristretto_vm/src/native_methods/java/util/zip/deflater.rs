use crate::Result;
use crate::native_methods::registry::{JAVA_8, MethodRegistry};
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/util/zip/Deflater";

/// Register all native methods for `java.util.zip.Deflater`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_8 {
        registry.register(CLASS_NAME, "deflateBytes", "(J[BIII)I", deflate_bytes);
        registry.register(CLASS_NAME, "initIDs", "()V", init_ids);
    } else {
        registry.register(
            CLASS_NAME,
            "deflateBufferBuffer",
            "(JJIJIII)J",
            deflate_buffer_buffer,
        );
        registry.register(
            CLASS_NAME,
            "deflateBufferBytes",
            "(JJI[BIIII)J",
            deflate_buffer_bytes,
        );
        registry.register(
            CLASS_NAME,
            "deflateBytesBuffer",
            "(J[BIIJIII)J",
            deflate_bytes_buffer,
        );
        registry.register(
            CLASS_NAME,
            "deflateBytesBytes",
            "(J[BII[BIIII)J",
            deflate_bytes_bytes,
        );
        registry.register(
            CLASS_NAME,
            "setDictionaryBuffer",
            "(JJI)V",
            set_dictionary_buffer,
        );
    }

    registry.register(CLASS_NAME, "end", "(J)V", end);
    registry.register(CLASS_NAME, "getAdler", "(J)I", get_adler);
    registry.register(CLASS_NAME, "init", "(IIZ)J", init);
    registry.register(CLASS_NAME, "reset", "(J)V", reset);
    registry.register(CLASS_NAME, "setDictionary", "(J[BII)V", set_dictionary);
}

#[async_recursion(?Send)]
async fn deflate_bytes(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.zip.Deflater.deflateBytes(J[BIII)I")
}

#[async_recursion(?Send)]
async fn deflate_buffer_buffer(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.util.zip.Deflater.deflateBufferBuffer(JJIJIII)J")
}

#[async_recursion(?Send)]
async fn deflate_buffer_bytes(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.util.zip.Deflater.deflateBufferBytes(JJI[BIIII)J")
}

#[async_recursion(?Send)]
async fn deflate_bytes_buffer(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.util.zip.Deflater.deflateBytesBuffer(J[BIIJIII)J")
}

#[async_recursion(?Send)]
async fn deflate_bytes_bytes(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.util.zip.Deflater.deflateBytesBytes(J[BII[BIIII)J")
}

#[async_recursion(?Send)]
async fn end(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.zip.Deflater.end(J)V")
}

#[async_recursion(?Send)]
async fn get_adler(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.zip.Deflater.getAdler(J)I")
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.zip.Deflater.init(IIZ)J")
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn reset(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.zip.Deflater.reset(J)V")
}

#[async_recursion(?Send)]
async fn set_dictionary(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.zip.Deflater.setDictionary(J[BII)V")
}

#[async_recursion(?Send)]
async fn set_dictionary_buffer(
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
