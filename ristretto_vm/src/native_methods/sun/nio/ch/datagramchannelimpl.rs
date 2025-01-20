use crate::arguments::Arguments;
use crate::native_methods::registry::{MethodRegistry, JAVA_11};
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/nio/ch/DatagramChannelImpl";

/// Register all native methods for `sun.nio.ch.DatagramChannelImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_11 {
        registry.register(CLASS_NAME, "initIDs", "()V", init_ids);
        registry.register(
            CLASS_NAME,
            "receive0",
            "(Ljava/io/FileDescriptor;JIZ)I",
            receive_0,
        );
        registry.register(
            CLASS_NAME,
            "send0",
            "(ZLjava/io/FileDescriptor;JILjava/net/InetAddress;I)I",
            send_0,
        );
    } else {
        registry.register(
            CLASS_NAME,
            "receive0",
            "(Ljava/io/FileDescriptor;JIJZ)I",
            receive_0,
        );
        registry.register(
            CLASS_NAME,
            "send0",
            "(Ljava/io/FileDescriptor;JIJI)I",
            send_0,
        );
    }

    registry.register(
        CLASS_NAME,
        "disconnect0",
        "(Ljava/io/FileDescriptor;Z)V",
        disconnect_0,
    );
}

#[async_recursion(?Send)]
async fn disconnect_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.DatagramChannelImpl.disconnect0(Ljava/io/FileDescriptor;Z)V");
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn receive_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.DatagramChannelImpl.receive0(Ljava/io/FileDescriptor;JIZ)I");
}

#[async_recursion(?Send)]
async fn send_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!(
        "sun.nio.ch.DatagramChannelImpl.send0(ZLjava/io/FileDescriptor;JILjava/net/InetAddress;I)I"
    );
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
        let _ = disconnect_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Arguments::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.DatagramChannelImpl.receive0(Ljava/io/FileDescriptor;JIZ)I"
    )]
    async fn test_receive_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = receive_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.DatagramChannelImpl.send0(ZLjava/io/FileDescriptor;JILjava/net/InetAddress;I)I"
    )]
    async fn test_send_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = send_0(thread, Arguments::default()).await;
    }
}
