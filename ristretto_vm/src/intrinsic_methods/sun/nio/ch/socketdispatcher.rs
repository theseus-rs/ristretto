use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classfile::{JAVA_17, JAVA_21};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/nio/ch/SocketDispatcher.read0(Ljava/io/FileDescriptor;JI)I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn read_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.SocketDispatcher.read0(Ljava/io/FileDescriptor;JI)I")
}

#[intrinsic_method(
    "sun/nio/ch/SocketDispatcher.readv0(Ljava/io/FileDescriptor;JI)J",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn readv_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.SocketDispatcher.readv0(Ljava/io/FileDescriptor;JI)J")
}

#[intrinsic_method(
    "sun/nio/ch/SocketDispatcher.write0(Ljava/io/FileDescriptor;JI)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn write_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.SocketDispatcher.write0(Ljava/io/FileDescriptor;JI)I")
}

#[intrinsic_method(
    "sun/nio/ch/SocketDispatcher.writev0(Ljava/io/FileDescriptor;JI)J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn writev_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.SocketDispatcher.writev0(Ljava/io/FileDescriptor;JI)J")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.SocketDispatcher.read0(Ljava/io/FileDescriptor;JI)I"
    )]
    async fn test_read_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = read_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.SocketDispatcher.readv0(Ljava/io/FileDescriptor;JI)J"
    )]
    async fn test_readv_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = readv_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.SocketDispatcher.write0(Ljava/io/FileDescriptor;JI)I"
    )]
    async fn test_write_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = write_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.SocketDispatcher.writev0(Ljava/io/FileDescriptor;JI)J"
    )]
    async fn test_writev_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = writev_0(thread, Parameters::default()).await;
    }
}
