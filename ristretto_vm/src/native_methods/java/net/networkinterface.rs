use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_17: Version = Version::Java17 { minor: 0 };

/// Register all native methods for `java.net.NetworkInterface`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/net/NetworkInterface";
    let java_version = registry.java_version();

    if java_version >= &JAVA_17 {
        registry.register(
            class_name,
            "boundInetAddress0",
            "(Ljava/net/InetAddress;)Z",
            bound_inet_address_0,
        );
    }

    registry.register(
        class_name,
        "getAll",
        "()[Ljava/net/NetworkInterface;",
        get_all,
    );
    registry.register(
        class_name,
        "getByIndex0",
        "(I)Ljava/net/NetworkInterface;",
        get_by_index_0,
    );
    registry.register(
        class_name,
        "getByInetAddress0",
        "(Ljava/net/InetAddress;)Ljava/net/NetworkInterface;",
        get_by_inet_address_0,
    );
    registry.register(
        class_name,
        "getByName0",
        "(Ljava/lang/String;)Ljava/net/NetworkInterface;",
        get_by_name_0,
    );
    registry.register(class_name, "getMTU0", "(Ljava/lang/String;I)I", get_mtu_0);
    registry.register(
        class_name,
        "getMacAddr0",
        "([BLjava/lang/String;I)[B",
        get_mac_addr_0,
    );
    registry.register(class_name, "init", "()V", init);
    registry.register(
        class_name,
        "isLoopback0",
        "(Ljava/lang/String;I)Z",
        is_loopback_0,
    );
    registry.register(class_name, "isP2P0", "(Ljava/lang/String;I)Z", is_p2p_0);
    registry.register(class_name, "isUp0", "(Ljava/lang/String;I)Z", is_up_0);
    registry.register(
        class_name,
        "supportsMulticast0",
        "(Ljava/lang/String;I)Z",
        supports_multicast_0,
    );
}

#[async_recursion(?Send)]
async fn bound_inet_address_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.net.NetworkInterface.boundInetAddress0(Ljava/net/InetAddress;)Z")
}

#[async_recursion(?Send)]
async fn get_all(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.NetworkInterface.getAll()[Ljava/net/NetworkInterface;")
}

#[async_recursion(?Send)]
async fn get_by_index_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.NetworkInterface.getByIndex0(I)Ljava/net/NetworkInterface;")
}

#[async_recursion(?Send)]
async fn get_by_inet_address_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.net.NetworkInterface.getByInetAddress0(Ljava/net/InetAddress;)Ljava/net/NetworkInterface;")
}

#[async_recursion(?Send)]
async fn get_by_name_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.NetworkInterface.getByName0(Ljava/lang/String;)Ljava/net/NetworkInterface;")
}

#[async_recursion(?Send)]
async fn get_mtu_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.NetworkInterface.getMTU0(Ljava/lang/String;I)I")
}

#[async_recursion(?Send)]
async fn get_mac_addr_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.NetworkInterface.getMacAddr0([BLjava/lang/String;I)[B")
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn is_loopback_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.NetworkInterface.isLoopback0(Ljava/lang/String;I)Z")
}

#[async_recursion(?Send)]
async fn is_p2p_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.NetworkInterface.isP2P0(Ljava/lang/String;I)Z")
}

#[async_recursion(?Send)]
async fn is_up_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.NetworkInterface.isUp0(Ljava/lang/String;I)Z")
}

#[async_recursion(?Send)]
async fn supports_multicast_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.net.NetworkInterface.supportsMulticast0(Ljava/lang/String;I)Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::new(&Version::Java17 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "java/net/NetworkInterface";
        assert!(registry
            .method(class_name, "getAll", "()[Ljava/net/NetworkInterface;")
            .is_some());
        assert!(registry
            .method(class_name, "getByIndex0", "(I)Ljava/net/NetworkInterface;")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "getByInetAddress0",
                "(Ljava/net/InetAddress;)Ljava/net/NetworkInterface;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "getByName0",
                "(Ljava/lang/String;)Ljava/net/NetworkInterface;"
            )
            .is_some());
        assert!(registry
            .method(class_name, "getMTU0", "(Ljava/lang/String;I)I")
            .is_some());
        assert!(registry
            .method(class_name, "getMacAddr0", "([BLjava/lang/String;I)[B")
            .is_some());
        assert!(registry.method(class_name, "init", "()V").is_some());
        assert!(registry
            .method(class_name, "isLoopback0", "(Ljava/lang/String;I)Z")
            .is_some());
        assert!(registry
            .method(class_name, "isP2P0", "(Ljava/lang/String;I)Z")
            .is_some());
        assert!(registry
            .method(class_name, "isUp0", "(Ljava/lang/String;I)Z")
            .is_some());
        assert!(registry
            .method(class_name, "supportsMulticast0", "(Ljava/lang/String;I)Z")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.NetworkInterface.boundInetAddress0(Ljava/net/InetAddress;)Z"
    )]
    async fn test_bound_inet_address_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = bound_inet_address_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.NetworkInterface.getAll()[Ljava/net/NetworkInterface;"
    )]
    async fn test_get_all() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_all(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.NetworkInterface.getByIndex0(I)Ljava/net/NetworkInterface;"
    )]
    async fn test_get_by_index_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_by_index_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.NetworkInterface.getByInetAddress0(Ljava/net/InetAddress;)Ljava/net/NetworkInterface;"
    )]
    async fn test_get_by_inet_address_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_by_inet_address_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.NetworkInterface.getByName0(Ljava/lang/String;)Ljava/net/NetworkInterface;"
    )]
    async fn test_get_by_name_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_by_name_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.NetworkInterface.getMTU0(Ljava/lang/String;I)I"
    )]
    async fn test_get_mtu_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_mtu_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.NetworkInterface.getMacAddr0([BLjava/lang/String;I)[B"
    )]
    async fn test_get_mac_addr_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_mac_addr_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init(thread, Arguments::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.NetworkInterface.isLoopback0(Ljava/lang/String;I)Z"
    )]
    async fn test_is_loopback_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_loopback_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.NetworkInterface.isP2P0(Ljava/lang/String;I)Z"
    )]
    async fn test_is_p2p_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_p2p_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.NetworkInterface.isUp0(Ljava/lang/String;I)Z"
    )]
    async fn test_is_up_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_up_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.NetworkInterface.supportsMulticast0(Ljava/lang/String;I)Z"
    )]
    async fn test_supports_multicast_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = supports_multicast_0(thread, Arguments::default()).await;
    }
}
