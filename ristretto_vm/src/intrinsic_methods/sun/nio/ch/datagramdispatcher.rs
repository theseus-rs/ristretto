use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/nio/ch/DatagramDispatcher.dup0(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub(crate) async fn dup_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.DatagramDispatcher.dup0(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;)V");
}

#[intrinsic_method(
    "sun/nio/ch/DatagramDispatcher.read0(Ljava/io/FileDescriptor;JI)I",
    Any
)]
#[async_method]
pub(crate) async fn read_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.DatagramDispatcher.read0(Ljava/io/FileDescriptor;JI)I");
}

#[intrinsic_method(
    "sun/nio/ch/DatagramDispatcher.readv0(Ljava/io/FileDescriptor;JI)J",
    Any
)]
#[async_method]
pub(crate) async fn readv_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.DatagramDispatcher.readv0(Ljava/io/FileDescriptor;JI)J");
}

#[intrinsic_method(
    "sun/nio/ch/DatagramDispatcher.write0(Ljava/io/FileDescriptor;JI)I",
    Any
)]
#[async_method]
pub(crate) async fn write_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.DatagramDispatcher.write0(Ljava/io/FileDescriptor;JI)I");
}

#[intrinsic_method(
    "sun/nio/ch/DatagramDispatcher.writev0(Ljava/io/FileDescriptor;JI)J",
    Any
)]
#[async_method]
pub(crate) async fn writev_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.DatagramDispatcher.writev0(Ljava/io/FileDescriptor;JI)J");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.DatagramDispatcher.dup0(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;)V"
    )]
    async fn test_dup_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = dup_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.DatagramDispatcher.read0(Ljava/io/FileDescriptor;JI)I"
    )]
    async fn test_read_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = read_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.DatagramDispatcher.readv0(Ljava/io/FileDescriptor;JI)J"
    )]
    async fn test_readv_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = readv_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.DatagramDispatcher.write0(Ljava/io/FileDescriptor;JI)I"
    )]
    async fn test_write_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = write_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.DatagramDispatcher.writev0")]
    async fn test_writev_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = writev_0(thread, Parameters::default()).await;
    }
}
