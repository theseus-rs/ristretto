use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_8: Version = Version::Java8 { minor: 0 };

/// Register all native methods for `java.util.zip.CRC32`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/util/zip/CRC32";
    let java_version = registry.java_version();

    if java_version <= &JAVA_8 {
        registry.register(
            class_name,
            "updateByteBuffer",
            "(IJII)I",
            update_byte_buffer,
        );
        registry.register(class_name, "updateBytes", "(I[BII)I", update_bytes);
    } else {
        registry.register(
            class_name,
            "updateByteBuffer0",
            "(IJII)I",
            update_byte_buffer_0,
        );
        registry.register(class_name, "updateBytes0", "(I[BII)I", update_bytes_0);
    }

    registry.register(class_name, "update", "(II)I", update);
}

#[async_recursion(?Send)]
async fn update(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.util.zip.CRC32.update(II)I")
}

#[async_recursion(?Send)]
async fn update_byte_buffer(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.util.zip.CRC32.updateByteBuffer(IJII)I")
}

#[async_recursion(?Send)]
async fn update_byte_buffer_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.util.zip.CRC32.updateByteBuffer0(IJII)I")
}

#[async_recursion(?Send)]
async fn update_bytes(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.util.zip.CRC32.updateBytes(I[BII)I")
}

#[async_recursion(?Send)]
async fn update_bytes_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.util.zip.CRC32.updateBytes0(I[BII)I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::new(&Version::Java8 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "java/util/zip/CRC32";
        assert!(registry
            .method(class_name, "updateByteBuffer", "(IJII)I")
            .is_some());
        assert!(registry
            .method(class_name, "updateBytes", "(I[BII)I")
            .is_some());
    }

    #[test]
    fn test_register_java_11() {
        let mut registry = MethodRegistry::new(&Version::Java9 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "java/util/zip/CRC32";
        assert!(registry
            .method(class_name, "updateByteBuffer0", "(IJII)I")
            .is_some());
        assert!(registry
            .method(class_name, "updateBytes0", "(I[BII)I")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.CRC32.update(II)I")]
    async fn test_update() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = update(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.CRC32.updateByteBuffer(IJII)I")]
    async fn test_update_byte_buffer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = update_byte_buffer(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.CRC32.updateByteBuffer0(IJII)I")]
    async fn test_update_byte_buffer_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = update_byte_buffer_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.CRC32.updateBytes(I[BII)I")]
    async fn test_update_bytes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = update_bytes(thread, Arguments::default()).await;
    }
}
