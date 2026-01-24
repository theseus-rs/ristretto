use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classfile::{JAVA_8, JAVA_11};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/nio/ch/SocketChannelImpl.checkConnect(Ljava/io/FileDescriptor;ZZ)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn check_connect_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.SocketChannelImpl.checkConnect(Ljava/io/FileDescriptor;ZZ)I")
}

#[intrinsic_method(
    "sun/nio/ch/SocketChannelImpl.checkConnect(Ljava/io/FileDescriptor;Z)I",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn check_connect_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.SocketChannelImpl.checkConnect(Ljava/io/FileDescriptor;Z)I")
}

#[intrinsic_method(
    "sun/nio/ch/SocketChannelImpl.sendOutOfBandData(Ljava/io/FileDescriptor;B)I",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn send_out_of_band_data(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.SocketChannelImpl.sendOutOfBandData(Ljava/io/FileDescriptor;B)I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.SocketChannelImpl.checkConnect(Ljava/io/FileDescriptor;ZZ)I"
    )]
    async fn test_check_connect_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = check_connect_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.SocketChannelImpl.checkConnect(Ljava/io/FileDescriptor;Z)I"
    )]
    async fn test_check_connect_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = check_connect_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.SocketChannelImpl.sendOutOfBandData(Ljava/io/FileDescriptor;B)I"
    )]
    async fn test_send_out_of_band_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = send_out_of_band_data(thread, Parameters::default()).await;
    }
}
