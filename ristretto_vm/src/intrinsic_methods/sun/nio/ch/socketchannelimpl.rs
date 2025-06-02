use crate::Result;
use crate::intrinsic_methods::registry::{JAVA_8, MethodRegistry};
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/nio/ch/SocketChannelImpl";

/// Register all intrinsic methods for `sun.nio.ch.SocketChannelImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_8 {
        registry.register(
            CLASS_NAME,
            "checkConnect",
            "(Ljava/io/FileDescriptor;ZZ)I",
            check_connect,
        );
    } else {
        registry.register(
            CLASS_NAME,
            "checkConnect",
            "(Ljava/io/FileDescriptor;Z)I",
            check_connect,
        );
    }

    registry.register(
        CLASS_NAME,
        "sendOutOfBandData",
        "(Ljava/io/FileDescriptor;B)I",
        send_out_of_band_data,
    );
}

#[async_recursion(?Send)]
async fn check_connect(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.SocketChannelImpl.checkConnect(Ljava/io/FileDescriptor;ZZ)I")
}

#[async_recursion(?Send)]
async fn send_out_of_band_data(
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
    async fn test_check_connect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = check_connect(thread, Parameters::default()).await;
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
