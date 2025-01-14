use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_11: Version = Version::Java11 { minor: 0 };

/// Register all native methods for `sun.nio.ch.DatagramChannelImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/nio/ch/DatagramChannelImpl";
    let java_version = registry.java_version();

    if java_version <= &JAVA_11 {
        registry.register(class_name, "initIDs", "()V", init_ids);
        registry.register(
            class_name,
            "receive0",
            "(Ljava/io/FileDescriptor;JIZ)I",
            receive_0,
        );
        registry.register(
            class_name,
            "send0",
            "(ZLjava/io/FileDescriptor;JILjava/net/InetAddress;I)I",
            send_0,
        );
    } else {
        registry.register(
            class_name,
            "receive0",
            "(Ljava/io/FileDescriptor;JIJZ)I",
            receive_0,
        );
        registry.register(
            class_name,
            "send0",
            "(Ljava/io/FileDescriptor;JIJI)I",
            send_0,
        );
    }

    registry.register(
        class_name,
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

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/nio/ch/DatagramChannelImpl";
        assert!(registry.method(class_name, "initIDs", "()V").is_some());
        assert!(registry
            .method(class_name, "receive0", "(Ljava/io/FileDescriptor;JIZ)I")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "send0",
                "(ZLjava/io/FileDescriptor;JILjava/net/InetAddress;I)I"
            )
            .is_some());
        assert!(registry
            .method(class_name, "disconnect0", "(Ljava/io/FileDescriptor;Z)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.nio.ch.DatagramChannelImpl.disconnect0(Ljava/io/FileDescriptor;Z)V"
    )]
    async fn test_disconnect_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = disconnect_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.nio.ch.DatagramChannelImpl.receive0(Ljava/io/FileDescriptor;JIZ)I"
    )]
    async fn test_receive_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = receive_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.nio.ch.DatagramChannelImpl.send0(ZLjava/io/FileDescriptor;JILjava/net/InetAddress;I)I"
    )]
    async fn test_send_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = send_0(thread, Arguments::default()).await;
    }
}
