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
