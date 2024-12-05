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
