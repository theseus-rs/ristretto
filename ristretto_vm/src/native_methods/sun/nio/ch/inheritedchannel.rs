use crate::arguments::Arguments;
use crate::native_methods::registry::{MethodRegistry, JAVA_11};
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/nio/ch/InheritedChannel";

/// Register all native methods for `sun.nio.ch.InheritedChannel`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() >= JAVA_11 {
        registry.register(CLASS_NAME, "initIDs", "()V", init_ids);
    }

    if registry.java_major_version() <= JAVA_11 {
        registry.register(CLASS_NAME, "open0", "(Ljava/lang/String;I)I", open_0);
        registry.register(
            CLASS_NAME,
            "peerAddress0",
            "(I)Ljava/net/InetAddress;",
            peer_address_0,
        );
    } else {
        registry.register(
            CLASS_NAME,
            "inetPeerAddress0",
            "(I)Ljava/net/InetAddress;",
            inet_peer_address_0,
        );
        registry.register(CLASS_NAME, "addressFamily", "(I)I", address_family);
        registry.register(CLASS_NAME, "isConnected", "(I)Z", is_connected);
        registry.register(CLASS_NAME, "open0", "(Ljava/lang/String;I)I", open_0);
        registry.register(CLASS_NAME, "unixPeerAddress0", "(I)[B", unix_peer_address_0);
    }

    registry.register(CLASS_NAME, "close0", "(I)V", close_0);
    registry.register(CLASS_NAME, "dup", "(I)I", dup);
    registry.register(CLASS_NAME, "dup2", "(II)V", dup_2);
    registry.register(CLASS_NAME, "peerPort0", "(I)I", peer_port_0);
    registry.register(CLASS_NAME, "soType0", "(I)I", so_type_0);
}

#[async_recursion(?Send)]
async fn address_family(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.InheritedChannel.addressFamily(I)I");
}

#[async_recursion(?Send)]
async fn close_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.InheritedChannel.close0(I)V");
}

#[async_recursion(?Send)]
async fn dup(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.InheritedChannel.dup(I)I");
}

#[async_recursion(?Send)]
async fn dup_2(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.InheritedChannel.dup2(II)V");
}

#[async_recursion(?Send)]
async fn inet_peer_address_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.InheritedChannel.inetPeerAddress0(I)Ljava/net/InetAddress;");
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn is_connected(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.InheritedChannel.isConnected(I)Z");
}

#[async_recursion(?Send)]
async fn open_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.InheritedChannel.open0(Ljava/lang/String;I)I");
}

#[async_recursion(?Send)]
async fn peer_address_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.InheritedChannel.peerAddress0(I)Ljava/net/InetAddress;");
}

#[async_recursion(?Send)]
async fn peer_port_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.InheritedChannel.peerPort0(I)I");
}

#[async_recursion(?Send)]
async fn so_type_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.InheritedChannel.soType0(I)I");
}

#[async_recursion(?Send)]
async fn unix_peer_address_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.InheritedChannel.unixPeerAddress0(I)[B");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.InheritedChannel.close0(I)V")]
    async fn test_close_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = close_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.InheritedChannel.dup(I)I")]
    async fn test_dup() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = dup(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.InheritedChannel.dup2(II)V")]
    async fn test_dup_2() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = dup_2(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.InheritedChannel.peerPort0(I)I")]
    async fn test_peer_port_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = peer_port_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.InheritedChannel.soType0(I)I")]
    async fn test_so_type_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = so_type_0(thread, Arguments::default()).await;
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
        expected = "not yet implemented: sun.nio.ch.InheritedChannel.inetPeerAddress0(I)Ljava/net/InetAddress;"
    )]
    async fn test_inet_peer_address_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = inet_peer_address_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.InheritedChannel.addressFamily(I)I")]
    async fn test_address_family() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = address_family(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.InheritedChannel.isConnected(I)Z")]
    async fn test_is_connected() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_connected(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.InheritedChannel.open0(Ljava/lang/String;I)I"
    )]
    async fn test_open_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = open_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.InheritedChannel.unixPeerAddress0(I)[B"
    )]
    async fn test_unix_peer_address_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = unix_peer_address_0(thread, Arguments::default()).await;
    }
}
