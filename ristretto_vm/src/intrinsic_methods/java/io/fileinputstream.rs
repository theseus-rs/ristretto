use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual, LessThanOrEqual};
use ristretto_classfile::{JAVA_8, JAVA_17, JAVA_24};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("java/io/FileInputStream.available0()I", Any)]
#[async_recursion(?Send)]
pub(crate) async fn available_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.FileInputStream.available0()I")
}

#[intrinsic_method("java/io/FileInputStream.close0()V", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn close_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.FileInputStream.close0()V")
}

#[intrinsic_method("java/io/FileInputStream.initIDs()V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn init_ids(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "java/io/FileInputStream.isRegularFile0(Ljava/io/FileDescriptor;)Z",
    GreaterThanOrEqual(JAVA_24)
)]
#[async_recursion(?Send)]
pub(crate) async fn is_regular_file_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.FileInputStream.isRegularFile0(Ljava/io/FileDescriptor;)Z")
}

#[intrinsic_method("java/io/FileInputStream.length0()J", GreaterThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn length_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.FileInputStream.length0()J")
}

#[intrinsic_method("java/io/FileInputStream.open0(Ljava/lang/String;)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn open_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.FileInputStream.open0(Ljava/lang/String;)V")
}

#[intrinsic_method("java/io/FileInputStream.position0()J", GreaterThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn position_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.FileInputStream.position0()J")
}

#[intrinsic_method("java/io/FileInputStream.read0()I", Any)]
#[async_recursion(?Send)]
pub(crate) async fn read_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.FileInputStream.read0()I")
}

#[intrinsic_method("java/io/FileInputStream.readBytes([BII)I", Any)]
#[async_recursion(?Send)]
pub(crate) async fn read_bytes(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.FileInputStream.readBytes([BII)I")
}

#[intrinsic_method("java/io/FileInputStream.skip0(J)J", Any)]
#[async_recursion(?Send)]
pub(crate) async fn skip_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
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
