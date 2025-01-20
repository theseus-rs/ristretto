use crate::native_methods::registry::{MethodRegistry, JAVA_8};
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/util/zip/Inflater";

/// Register all native methods for `java.util.zip.Inflater`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_8 {
        registry.register(CLASS_NAME, "inflateBytes", "(J[BII)I", inflate_bytes);
    } else {
        registry.register(
            CLASS_NAME,
            "inflateBufferBuffer",
            "(JJIJI)J",
            inflate_buffer_buffer,
        );
        registry.register(
            CLASS_NAME,
            "inflateBufferBytes",
            "(JJI[BII)J",
            inflate_buffer_bytes,
        );
        registry.register(
            CLASS_NAME,
            "inflateBytesBuffer",
            "(J[BIIJI)J",
            inflate_bytes_buffer,
        );
        registry.register(
            CLASS_NAME,
            "inflateBytesBytes",
            "(J[BII[BII)J",
            inflate_bytes_bytes,
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
    registry.register(CLASS_NAME, "init", "(Z)J", init);
    registry.register(CLASS_NAME, "initIDs", "()V", init_ids);
    registry.register(CLASS_NAME, "reset", "(J)V", reset);
    registry.register(CLASS_NAME, "setDictionary", "(J[BII)V", set_dictionary);
}

#[async_recursion(?Send)]
async fn end(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.zip.Inflater.end(J)V")
}

#[async_recursion(?Send)]
async fn get_adler(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.zip.Inflater.getAdler(J)I")
}

#[async_recursion(?Send)]
async fn inflate_buffer_buffer(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.util.zip.Inflater.inflateBufferBuffer(JJIJI)J")
}

#[async_recursion(?Send)]
async fn inflate_buffer_bytes(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.util.zip.Inflater.inflateBufferBytes(JJI[BII)J")
}

#[async_recursion(?Send)]
async fn inflate_bytes_buffer(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.util.zip.Inflater.inflateBytesBuffer(J[BIIJI)J")
}

#[async_recursion(?Send)]
async fn inflate_bytes(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.zip.Inflater.inflateBytes(J[BII)I")
}

#[async_recursion(?Send)]
async fn inflate_bytes_bytes(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.util.zip.Inflater.inflateBytesBytes(J[BII[BII)J")
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.zip.Inflater.init(Z)J")
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn reset(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.zip.Inflater.reset(J)V")
}

#[async_recursion(?Send)]
async fn set_dictionary(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.zip.Inflater.setDictionary(J[BII)V")
}

#[async_recursion(?Send)]
async fn set_dictionary_buffer(
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
