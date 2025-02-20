use crate::Result;
use crate::native_methods::registry::{JAVA_17, MethodRegistry};
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/net/NetworkInterface";

/// Register all native methods for `java.net.NetworkInterface`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() >= JAVA_17 {
        registry.register(
            CLASS_NAME,
            "boundInetAddress0",
            "(Ljava/net/InetAddress;)Z",
            bound_inet_address_0,
        );
    }

    registry.register(
        CLASS_NAME,
        "getAll",
        "()[Ljava/net/NetworkInterface;",
        get_all,
    );
    registry.register(
        CLASS_NAME,
        "getByIndex0",
        "(I)Ljava/net/NetworkInterface;",
        get_by_index_0,
    );
    registry.register(
        CLASS_NAME,
        "getByInetAddress0",
        "(Ljava/net/InetAddress;)Ljava/net/NetworkInterface;",
        get_by_inet_address_0,
    );
    registry.register(
        CLASS_NAME,
        "getByName0",
        "(Ljava/lang/String;)Ljava/net/NetworkInterface;",
        get_by_name_0,
    );
    registry.register(CLASS_NAME, "getMTU0", "(Ljava/lang/String;I)I", get_mtu_0);
    registry.register(
        CLASS_NAME,
        "getMacAddr0",
        "([BLjava/lang/String;I)[B",
        get_mac_addr_0,
    );
    registry.register(CLASS_NAME, "init", "()V", init);
    registry.register(
        CLASS_NAME,
        "isLoopback0",
        "(Ljava/lang/String;I)Z",
        is_loopback_0,
    );
    registry.register(CLASS_NAME, "isP2P0", "(Ljava/lang/String;I)Z", is_p2p_0);
    registry.register(CLASS_NAME, "isUp0", "(Ljava/lang/String;I)Z", is_up_0);
    registry.register(
        CLASS_NAME,
        "supportsMulticast0",
        "(Ljava/lang/String;I)Z",
        supports_multicast_0,
    );
}

#[async_recursion(?Send)]
async fn bound_inet_address_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.NetworkInterface.boundInetAddress0(Ljava/net/InetAddress;)Z")
}

#[async_recursion(?Send)]
async fn get_all(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.net.NetworkInterface.getAll()[Ljava/net/NetworkInterface;")
}

#[async_recursion(?Send)]
async fn get_by_index_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.net.NetworkInterface.getByIndex0(I)Ljava/net/NetworkInterface;")
}

#[async_recursion(?Send)]
async fn get_by_inet_address_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.net.NetworkInterface.getByInetAddress0(Ljava/net/InetAddress;)Ljava/net/NetworkInterface;"
    )
}

#[async_recursion(?Send)]
async fn get_by_name_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.net.NetworkInterface.getByName0(Ljava/lang/String;)Ljava/net/NetworkInterface;")
}

#[async_recursion(?Send)]
async fn get_mtu_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.net.NetworkInterface.getMTU0(Ljava/lang/String;I)I")
}

#[async_recursion(?Send)]
async fn get_mac_addr_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.net.NetworkInterface.getMacAddr0([BLjava/lang/String;I)[B")
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn is_loopback_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.net.NetworkInterface.isLoopback0(Ljava/lang/String;I)Z")
}

#[async_recursion(?Send)]
async fn is_p2p_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.net.NetworkInterface.isP2P0(Ljava/lang/String;I)Z")
}

#[async_recursion(?Send)]
async fn is_up_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.net.NetworkInterface.isUp0(Ljava/lang/String;I)Z")
}

#[async_recursion(?Send)]
async fn supports_multicast_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.NetworkInterface.supportsMulticast0(Ljava/lang/String;I)Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.NetworkInterface.boundInetAddress0(Ljava/net/InetAddress;)Z"
    )]
    async fn test_bound_inet_address_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = bound_inet_address_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.NetworkInterface.getAll()[Ljava/net/NetworkInterface;"
    )]
    async fn test_get_all() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_all(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.NetworkInterface.getByIndex0(I)Ljava/net/NetworkInterface;"
    )]
    async fn test_get_by_index_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_by_index_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.NetworkInterface.getByInetAddress0(Ljava/net/InetAddress;)Ljava/net/NetworkInterface;"
    )]
    async fn test_get_by_inet_address_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_by_inet_address_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.NetworkInterface.getByName0(Ljava/lang/String;)Ljava/net/NetworkInterface;"
    )]
    async fn test_get_by_name_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_by_name_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.NetworkInterface.getMTU0(Ljava/lang/String;I)I"
    )]
    async fn test_get_mtu_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_mtu_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.NetworkInterface.getMacAddr0([BLjava/lang/String;I)[B"
    )]
    async fn test_get_mac_addr_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_mac_addr_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.NetworkInterface.isLoopback0(Ljava/lang/String;I)Z"
    )]
    async fn test_is_loopback_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_loopback_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.NetworkInterface.isP2P0(Ljava/lang/String;I)Z"
    )]
    async fn test_is_p2p_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_p2p_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.NetworkInterface.isUp0(Ljava/lang/String;I)Z"
    )]
    async fn test_is_up_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_up_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.NetworkInterface.supportsMulticast0(Ljava/lang/String;I)Z"
    )]
    async fn test_supports_multicast_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = supports_multicast_0(thread, Parameters::default()).await;
    }
}
