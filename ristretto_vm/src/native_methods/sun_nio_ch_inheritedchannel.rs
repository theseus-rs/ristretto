use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_11: Version = Version::Java11 { minor: 0 };

/// Register all native methods for `sun.nio.ch.InheritedChannel`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/nio/ch/InheritedChannel";
    let java_version = registry.java_version().clone();

    if java_version >= JAVA_11 {
        registry.register(class_name, "initIDs", "()V", init_ids);
    }

    if java_version <= JAVA_11 {
        registry.register(class_name, "open0", "(Ljava/lang/String;I)I", open_0);
        registry.register(
            class_name,
            "peerAddress0",
            "(I)Ljava/net/InetAddress;",
            peer_address_0,
        );
    } else {
        registry.register(
            class_name,
            "inetPeerAddress0",
            "(I)Ljava/net/InetAddress;",
            inet_peer_address_0,
        );
        registry.register(class_name, "addressFamily", "(I)I", address_family);
        registry.register(class_name, "isConnected", "(I)Z", is_connected);
        registry.register(class_name, "open0", "(Ljava/lang/String;I)I", open_0);
        registry.register(class_name, "unixPeerAddress0", "(I)[B", unix_peer_address_0);
    }

    registry.register(class_name, "close0", "(I)V", close_0);
    registry.register(class_name, "dup", "(I)I", dup);
    registry.register(class_name, "dup2", "(II)V", dup_2);
    registry.register(class_name, "peerPort0", "(I)I", peer_port_0);
    registry.register(class_name, "soType0", "(I)I", so_type_0);
}

#[async_recursion(?Send)]
async fn address_family(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn close_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn dup(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn dup_2(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn inet_peer_address_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn is_connected(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn open_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn peer_address_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn peer_port_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn so_type_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn unix_peer_address_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
