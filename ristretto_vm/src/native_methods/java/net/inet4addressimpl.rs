use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.net.Inet4AddressImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/net/Inet4AddressImpl";
    registry.register(
        class_name,
        "getHostByAddr",
        "([B)Ljava/lang/String;",
        get_host_by_addr,
    );
    registry.register(
        class_name,
        "getLocalHostName",
        "()Ljava/lang/String;",
        get_local_host_name,
    );
    registry.register(class_name, "isReachable0", "([BI[BI)Z", is_reachable_0);
    registry.register(
        class_name,
        "lookupAllHostAddr",
        "(Ljava/lang/String;)[Ljava/net/InetAddress;",
        lookup_all_host_addr,
    );
}

#[async_recursion(?Send)]
async fn get_host_by_addr(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.Inet4AddressImpl.getHostByAddr([B)Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_local_host_name(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.Inet4AddressImpl.getLocalHostName()Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn is_reachable_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.Inet4AddressImpl.isReachable0([BI[BI)Z")
}

#[async_recursion(?Send)]
async fn lookup_all_host_addr(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.net.Inet4AddressImpl.lookupAllHostAddr(Ljava/lang/String;)[Ljava/net/InetAddress;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "java/net/Inet4AddressImpl";
        assert!(registry
            .method(class_name, "getHostByAddr", "([B)Ljava/lang/String;")
            .is_some());
        assert!(registry
            .method(class_name, "getLocalHostName", "()Ljava/lang/String;")
            .is_some());
        assert!(registry
            .method(class_name, "isReachable0", "([BI[BI)Z")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "lookupAllHostAddr",
                "(Ljava/lang/String;)[Ljava/net/InetAddress;"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.Inet4AddressImpl.getHostByAddr([B)Ljava/lang/String;"
    )]
    async fn test_get_host_by_addr() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_host_by_addr(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.Inet4AddressImpl.getLocalHostName()Ljava/lang/String;"
    )]
    async fn test_get_local_host_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_local_host_name(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.Inet4AddressImpl.isReachable0([BI[BI)Z"
    )]
    async fn test_is_reachable_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_reachable_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.Inet4AddressImpl.lookupAllHostAddr(Ljava/lang/String;)[Ljava/net/InetAddress;"
    )]
    async fn test_lookup_all_host_addr() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = lookup_all_host_addr(thread, Arguments::default()).await;
    }
}
