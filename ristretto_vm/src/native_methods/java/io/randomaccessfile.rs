use crate::native_methods::registry::{MethodRegistry, JAVA_18, JAVA_8};
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/io/RandomAccessFile";

/// Register all native methods for `java.io.RandomAccessFile`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_8 {
        registry.register(CLASS_NAME, "close0", "()V", close_0);
    }

    if registry.java_major_version() <= JAVA_18 {
        registry.register(CLASS_NAME, "length", "()J", length);
        registry.register(CLASS_NAME, "readBytes", "([BII)I", read_bytes);
        registry.register(CLASS_NAME, "setLength", "(J)V", set_length);
        registry.register(CLASS_NAME, "writeBytes", "([BII)V", write_bytes);
    } else {
        registry.register(CLASS_NAME, "length0", "()J", length_0);
        registry.register(CLASS_NAME, "readBytes0", "([BII)I", read_bytes_0);
        registry.register(CLASS_NAME, "setLength0", "(J)V", set_length_0);
        registry.register(CLASS_NAME, "writeBytes0", "([BII)V", write_bytes_0);
    }

    registry.register(CLASS_NAME, "getFilePointer", "()J", get_file_pointer);
    registry.register(CLASS_NAME, "initIDs", "()V", init_ids);
    registry.register(CLASS_NAME, "open0", "(Ljava/lang/String;I)V", open_0);
    registry.register(CLASS_NAME, "read0", "()I", read_0);
    registry.register(CLASS_NAME, "seek0", "(J)V", seek_0);
    registry.register(CLASS_NAME, "write0", "(I)V", write_0);
}

#[async_recursion(?Send)]
async fn close_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.RandomAccessFile.close0()V")
}

#[async_recursion(?Send)]
async fn get_file_pointer(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.RandomAccessFile.getFilePointer()J")
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn length(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.RandomAccessFile.length()J")
}

#[async_recursion(?Send)]
async fn length_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.RandomAccessFile.length0()J")
}

#[async_recursion(?Send)]
async fn open_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.RandomAccessFile.open0(Ljava/lang/String;I)V")
}

#[async_recursion(?Send)]
async fn read_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.RandomAccessFile.read0()I")
}

#[async_recursion(?Send)]
async fn read_bytes(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.RandomAccessFile.readBytes([BII)I")
}

#[async_recursion(?Send)]
async fn read_bytes_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.RandomAccessFile.readBytes0([BII)I")
}

#[async_recursion(?Send)]
async fn seek_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.RandomAccessFile.seek0(J)V")
}

#[async_recursion(?Send)]
async fn set_length(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.RandomAccessFile.setLength(J)V")
}

#[async_recursion(?Send)]
async fn set_length_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.RandomAccessFile.setLength0(J)V")
}

#[async_recursion(?Send)]
async fn write_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.RandomAccessFile.write0(I)V")
}

#[async_recursion(?Send)]
async fn write_bytes(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.RandomAccessFile.writeBytes([BII)V")
}

#[async_recursion(?Send)]
async fn write_bytes_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.RandomAccessFile.writeBytes0([BII)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.io.RandomAccessFile.close0()V")]
    async fn test_close_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = close_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.io.RandomAccessFile.length()J")]
    async fn test_length() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = length(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.io.RandomAccessFile.length0()J")]
    async fn test_length_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = length_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.io.RandomAccessFile.readBytes([BII)I")]
    async fn test_read_bytes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = read_bytes(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.io.RandomAccessFile.readBytes0([BII)I")]
    async fn test_read_bytes_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = read_bytes_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.io.RandomAccessFile.setLength(J)V")]
    async fn test_set_length() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_length(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.io.RandomAccessFile.setLength0(J)V")]
    async fn test_set_length_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_length_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.io.RandomAccessFile.getFilePointer()J")]
    async fn test_get_file_pointer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_file_pointer(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.RandomAccessFile.open0(Ljava/lang/String;I)V"
    )]
    async fn test_open_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = open_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.io.RandomAccessFile.read0()I")]
    async fn test_read_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = read_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.io.RandomAccessFile.seek0(J)V")]
    async fn test_seek_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = seek_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.io.RandomAccessFile.write0(I)V")]
    async fn test_write_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = write_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.io.RandomAccessFile.writeBytes([BII)V")]
    async fn test_write_bytes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = write_bytes(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.io.RandomAccessFile.writeBytes0([BII)V")]
    async fn test_write_bytes_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = write_bytes_0(thread, Parameters::default()).await;
    }
}
