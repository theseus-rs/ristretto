use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::{Any, GreaterThan, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/nio/ch/DatagramChannelImpl.disconnect0(Ljava/io/FileDescriptor;Z)V",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn disconnect_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.DatagramChannelImpl.disconnect0(Ljava/io/FileDescriptor;Z)V");
}

#[intrinsic_method("sun/nio/ch/DatagramChannelImpl.initIDs()V", LessThanOrEqual(JAVA_11))]
#[async_recursion(?Send)]
pub(crate) async fn init_ids(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "sun/nio/ch/DatagramChannelImpl.receive0(Ljava/io/FileDescriptor;JIZ)I",
    LessThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn receive_0_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.DatagramChannelImpl.receive0(Ljava/io/FileDescriptor;JIZ)I");
}

#[intrinsic_method(
    "sun/nio/ch/DatagramChannelImpl.receive0(Ljava/io/FileDescriptor;JIJZ)I",
    GreaterThan(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn receive_0_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.DatagramChannelImpl.receive0(Ljava/io/FileDescriptor;JIJZ)I");
}

#[intrinsic_method(
    "sun/nio/ch/DatagramChannelImpl.send0(ZLjava/io/FileDescriptor;JILjava/net/InetAddress;I)I",
    LessThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn send_0_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.nio.ch.DatagramChannelImpl.send0(ZLjava/io/FileDescriptor;JILjava/net/InetAddress;I)I"
    );
}

#[intrinsic_method(
    "sun/nio/ch/DatagramChannelImpl.send0(Ljava/io/FileDescriptor;JIJI)I",
    GreaterThan(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn send_0_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.DatagramChannelImpl.send0(Ljava/io/FileDescriptor;JIJI)I");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.DatagramChannelImpl.disconnect0(Ljava/io/FileDescriptor;Z)V"
    )]
    async fn test_disconnect_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = disconnect_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.DatagramChannelImpl.receive0(Ljava/io/FileDescriptor;JIZ)I"
    )]
    async fn test_receive_0_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = receive_0_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.DatagramChannelImpl.receive0(Ljava/io/FileDescriptor;JIJZ)I"
    )]
    async fn test_receive_0_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = receive_0_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.DatagramChannelImpl.send0(ZLjava/io/FileDescriptor;JILjava/net/InetAddress;I)I"
    )]
    async fn test_send_0_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = send_0_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.DatagramChannelImpl.send0(Ljava/io/FileDescriptor;JIJI)I"
    )]
    async fn test_send_0_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = send_0_1(thread, Parameters::default()).await;
    }
}
