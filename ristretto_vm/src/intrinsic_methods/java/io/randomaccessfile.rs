use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual, LessThanOrEqual};
use ristretto_classfile::{JAVA_8, JAVA_17};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("java/io/RandomAccessFile.close0()V", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn close_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.RandomAccessFile.close0()V")
}

#[intrinsic_method("java/io/RandomAccessFile.getFilePointer()J", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_file_pointer(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.RandomAccessFile.getFilePointer()J")
}

#[intrinsic_method("java/io/RandomAccessFile.initIDs()V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn init_ids(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/io/RandomAccessFile.length()J", LessThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn length(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.RandomAccessFile.length()J")
}

#[intrinsic_method("java/io/RandomAccessFile.length0()J", GreaterThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn length_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.RandomAccessFile.length0()J")
}

#[intrinsic_method("java/io/RandomAccessFile.open0(Ljava/lang/String;I)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn open_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.RandomAccessFile.open0(Ljava/lang/String;I)V")
}

#[intrinsic_method("java/io/RandomAccessFile.read0()I", Any)]
#[async_recursion(?Send)]
pub(crate) async fn read_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.RandomAccessFile.read0()I")
}

#[intrinsic_method("java/io/RandomAccessFile.readBytes([BII)I", LessThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn read_bytes(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.RandomAccessFile.readBytes([BII)I")
}

#[intrinsic_method(
    "java/io/RandomAccessFile.readBytes0([BII)I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn read_bytes_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.RandomAccessFile.readBytes0([BII)I")
}

#[intrinsic_method("java/io/RandomAccessFile.seek0(J)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn seek_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.RandomAccessFile.seek0(J)V")
}

#[intrinsic_method("java/io/RandomAccessFile.setLength(J)V", LessThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn set_length(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.RandomAccessFile.setLength(J)V")
}

#[intrinsic_method("java/io/RandomAccessFile.setLength0(J)V", GreaterThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn set_length_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.RandomAccessFile.setLength0(J)V")
}

#[intrinsic_method("java/io/RandomAccessFile.write0(I)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn write_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.RandomAccessFile.write0(I)V")
}

#[intrinsic_method("java/io/RandomAccessFile.writeBytes([BII)V", LessThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn write_bytes(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.RandomAccessFile.writeBytes([BII)V")
}

#[intrinsic_method(
    "java/io/RandomAccessFile.writeBytes0([BII)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn write_bytes_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
