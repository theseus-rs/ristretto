use crate::Result;
use crate::native_methods::registry::{JAVA_8, JAVA_17, JAVA_24, MethodRegistry};
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/io/FileInputStream";

/// Register all native methods for `java.io.FileInputStream`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_8 {
        registry.register(CLASS_NAME, "close0", "()V", close_0);
    } else if registry.java_major_version() >= JAVA_17 {
        registry.register(CLASS_NAME, "length0", "()J", length_0);
        registry.register(CLASS_NAME, "position0", "()J", position_0);
    }

    if registry.java_major_version() >= JAVA_24 {
        registry.register(
            CLASS_NAME,
            "isRegularFile0",
            "(Ljava/io/FileDescriptor;)Z",
            is_regular_file_0,
        );
    }

    registry.register(CLASS_NAME, "available0", "()I", available_0);
    registry.register(CLASS_NAME, "initIDs", "()V", init_ids);
    registry.register(CLASS_NAME, "open0", "(Ljava/lang/String;)V", open_0);
    registry.register(CLASS_NAME, "read0", "()I", read_0);
    registry.register(CLASS_NAME, "readBytes", "([BII)I", read_bytes);
    registry.register(CLASS_NAME, "skip0", "(J)J", skip_0);
}

#[async_recursion(?Send)]
async fn available_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.FileInputStream.available0()I")
}

#[async_recursion(?Send)]
async fn close_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.FileInputStream.close0()V")
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn is_regular_file_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.FileInputStream.isRegularFile0(Ljava/io/FileDescriptor;)Z")
}

#[async_recursion(?Send)]
async fn length_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.FileInputStream.length0()J")
}

#[async_recursion(?Send)]
async fn open_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.FileInputStream.open0(Ljava/lang/String;)V")
}

#[async_recursion(?Send)]
async fn position_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.FileInputStream.position0()J")
}

#[async_recursion(?Send)]
async fn read_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.FileInputStream.read0()I")
}

#[async_recursion(?Send)]
async fn read_bytes(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.FileInputStream.readBytes([BII)I")
}

#[async_recursion(?Send)]
async fn skip_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.FileInputStream.skip0(J)J")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.io.FileInputStream.available0()I")]
    async fn test_available_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = available_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.io.FileInputStream.close0()V")]
    async fn test_close_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = close_0(thread, Parameters::default()).await;
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
        expected = "not yet implemented: java.io.FileInputStream.isRegularFile0(Ljava/io/FileDescriptor;)Z"
    )]
    async fn test_is_regular_file_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_regular_file_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.io.FileInputStream.length0()J")]
    async fn test_length_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = length_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.FileInputStream.open0(Ljava/lang/String;)V"
    )]
    async fn test_open_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = open_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.io.FileInputStream.position0()J")]
    async fn test_position_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = position_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.io.FileInputStream.read0()I")]
    async fn test_read_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = read_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.io.FileInputStream.readBytes([BII)I")]
    async fn test_read_bytes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = read_bytes(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.io.FileInputStream.skip0(J)J")]
    async fn test_skip_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = skip_0(thread, Parameters::default()).await;
    }
}
