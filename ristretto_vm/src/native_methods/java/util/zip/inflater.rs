use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.util.zip.Inflater`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/util/zip/Inflater";
    registry.register(class_name, "end", "(J)V", end);
    registry.register(class_name, "getAdler", "(J)I", get_adler);
    registry.register(
        class_name,
        "inflateBufferBuffer",
        "(JJIJI)J",
        inflate_buffer_buffer,
    );
    registry.register(
        class_name,
        "inflateBufferBytes",
        "(JJI[BII)J",
        inflate_buffer_bytes,
    );
    registry.register(
        class_name,
        "inflateBytesBuffer",
        "(J[BIIJI)J",
        inflate_bytes_buffer,
    );
    registry.register(
        class_name,
        "inflateBytesBytes",
        "(J[BII[BII)J",
        inflate_bytes_bytes,
    );
    registry.register(class_name, "init", "(Z)J", init);
    registry.register(class_name, "initIDs", "()V", init_ids);
    registry.register(class_name, "reset", "(J)V", reset);
    registry.register(class_name, "setDictionary", "(J[BII)V", set_dictionary);
    registry.register(
        class_name,
        "setDictionaryBuffer",
        "(JJI)V",
        set_dictionary_buffer,
    );
}

#[async_recursion(?Send)]
async fn end(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.util.zip.Inflater.end(J)V")
}

#[async_recursion(?Send)]
async fn get_adler(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.util.zip.Inflater.getAdler(J)I")
}

#[async_recursion(?Send)]
async fn inflate_buffer_buffer(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.util.zip.Inflater.inflateBufferBuffer(JJIJI)J")
}

#[async_recursion(?Send)]
async fn inflate_buffer_bytes(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.util.zip.Inflater.inflateBufferBytes(JJI[BII)J")
}

#[async_recursion(?Send)]
async fn inflate_bytes_buffer(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.util.zip.Inflater.inflateBytesBuffer(J[BIIJI)J")
}

#[async_recursion(?Send)]
async fn inflate_bytes_bytes(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.util.zip.Inflater.inflateBytesBytes(J[BII[BII)J")
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.util.zip.Inflater.init(Z)J")
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn reset(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.util.zip.Inflater.reset(J)V")
}

#[async_recursion(?Send)]
async fn set_dictionary(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.util.zip.Inflater.setDictionary(J[BII)V")
}

#[async_recursion(?Send)]
async fn set_dictionary_buffer(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.util.zip.Inflater.setDictionaryBuffer(JJI)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "java/util/zip/Inflater";
        assert!(registry.method(class_name, "end", "(J)V").is_some());
        assert!(registry.method(class_name, "getAdler", "(J)I").is_some());
        assert!(registry
            .method(class_name, "inflateBufferBuffer", "(JJIJI)J")
            .is_some());
        assert!(registry
            .method(class_name, "inflateBufferBytes", "(JJI[BII)J")
            .is_some());
        assert!(registry
            .method(class_name, "inflateBytesBuffer", "(J[BIIJI)J")
            .is_some());
        assert!(registry
            .method(class_name, "inflateBytesBytes", "(J[BII[BII)J")
            .is_some());
        assert!(registry.method(class_name, "init", "(Z)J").is_some());
        assert!(registry.method(class_name, "initIDs", "()V").is_some());
        assert!(registry.method(class_name, "reset", "(J)V").is_some());
        assert!(registry
            .method(class_name, "setDictionary", "(J[BII)V")
            .is_some());
        assert!(registry
            .method(class_name, "setDictionaryBuffer", "(JJI)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.Inflater.end(J)V")]
    async fn test_end() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = end(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.Inflater.getAdler(J)I")]
    async fn test_get_adler() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_adler(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.zip.Inflater.inflateBufferBuffer(JJIJI)J"
    )]
    async fn test_inflate_buffer_buffer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = inflate_buffer_buffer(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.zip.Inflater.inflateBufferBytes(JJI[BII)J"
    )]
    async fn test_inflate_buffer_bytes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = inflate_buffer_bytes(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.zip.Inflater.inflateBytesBuffer(J[BIIJI)J"
    )]
    async fn test_inflate_bytes_buffer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = inflate_bytes_buffer(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.zip.Inflater.inflateBytesBytes(J[BII[BII)J"
    )]
    async fn test_inflate_bytes_bytes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = inflate_bytes_bytes(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.Inflater.init(Z)J")]
    async fn test_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Arguments::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.Inflater.reset(J)V")]
    async fn test_reset() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = reset(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.Inflater.setDictionary(J[BII)V")]
    async fn test_set_dictionary() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_dictionary(thread, Arguments::default()).await;
    }
}
