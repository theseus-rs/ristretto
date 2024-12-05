use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_8: Version = Version::Java8 { minor: 0 };

/// Register all native methods for `java.util.zip.Deflater`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/util/zip/Deflater";
    let java_version = registry.java_version();

    if java_version <= &JAVA_8 {
        registry.register(class_name, "deflateBytes", "(J[BIII)I", deflate_bytes);
        registry.register(class_name, "initIDs", "()V", init_ids);
    } else {
        registry.register(
            class_name,
            "deflateBufferBuffer",
            "(JJIJIII)J",
            deflate_buffer_buffer,
        );
        registry.register(
            class_name,
            "deflateBufferBytes",
            "(JJI[BIIII)J",
            deflate_buffer_bytes,
        );
        registry.register(
            class_name,
            "deflateBytesBuffer",
            "(J[BIIJIII)J",
            deflate_bytes_buffer,
        );
        registry.register(
            class_name,
            "deflateBytesBytes",
            "(J[BII[BIIII)J",
            deflate_bytes_bytes,
        );
        registry.register(
            class_name,
            "setDictionaryBuffer",
            "(JJI)V",
            set_dictionary_buffer,
        );
    }

    registry.register(class_name, "end", "(J)V", end);
    registry.register(class_name, "getAdler", "(J)I", get_adler);
    registry.register(class_name, "init", "(IIZ)J", init);
    registry.register(class_name, "reset", "(J)V", reset);
    registry.register(class_name, "setDictionary", "(J[BII)V", set_dictionary);
}

#[async_recursion(?Send)]
async fn deflate_bytes(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.util.zip.Deflater.deflateBytes(J[BIII)I")
}

#[async_recursion(?Send)]
async fn deflate_buffer_buffer(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.util.zip.Deflater.deflateBufferBuffer(JJIJIII)J")
}

#[async_recursion(?Send)]
async fn deflate_buffer_bytes(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.util.zip.Deflater.deflateBufferBytes(JJI[BIIII)J")
}

#[async_recursion(?Send)]
async fn deflate_bytes_buffer(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.util.zip.Deflater.deflateBytesBuffer(J[BIIJIII)J")
}

#[async_recursion(?Send)]
async fn deflate_bytes_bytes(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.util.zip.Deflater.deflateBytesBytes(J[BII[BIIII)J")
}

#[async_recursion(?Send)]
async fn end(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.util.zip.Deflater.end(J)V")
}

#[async_recursion(?Send)]
async fn get_adler(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.util.zip.Deflater.getAdler(J)I")
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.util.zip.Deflater.init(IIZ)J")
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn reset(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.util.zip.Deflater.reset(J)V")
}

#[async_recursion(?Send)]
async fn set_dictionary(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.util.zip.Deflater.setDictionary(J[BII)V")
}

#[async_recursion(?Send)]
async fn set_dictionary_buffer(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.util.zip.Deflater.setDictionaryBuffer(JJI)V")
}
