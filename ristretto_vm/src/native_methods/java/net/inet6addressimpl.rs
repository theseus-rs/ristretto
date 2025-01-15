use crate::arguments::Arguments;
use crate::native_methods::registry::{MethodRegistry, JAVA_17};
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/net/Inet6AddressImpl";

/// Register all native methods for `java.net.Inet6AddressImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_17 {
        registry.register(
            CLASS_NAME,
            "lookupAllHostAddr",
            "(Ljava/lang/String;)[Ljava/net/InetAddress;",
            lookup_all_host_addr,
        );
    } else {
        registry.register(
            CLASS_NAME,
            "lookupAllHostAddr",
            "(Ljava/lang/String;I)[Ljava/net/InetAddress;",
            lookup_all_host_addr,
        );
    }

    registry.register(
        CLASS_NAME,
        "getHostByAddr",
        "([B)Ljava/lang/String;",
        get_host_by_addr,
    );
    registry.register(
        CLASS_NAME,
        "getLocalHostName",
        "()Ljava/lang/String;",
        get_local_host_name,
    );
    registry.register(CLASS_NAME, "isReachable0", "([BII[BII)Z", is_reachable_0);
}

#[async_recursion(?Send)]
async fn get_host_by_addr(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.Inet6AddressImpl.getHostByAddr([B)Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_local_host_name(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.Inet6AddressImpl.getLocalHostName()Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn is_reachable_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.Inet6AddressImpl.isReachable0([BII[BII)Z")
}

#[async_recursion(?Send)]
async fn lookup_all_host_addr(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.net.Inet6AddressImpl.lookupAllHostAddr(Ljava/lang/String;)[Ljava/net/InetAddress;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.Inet6AddressImpl.getHostByAddr([B)Ljava/lang/String;"
    )]
    async fn test_get_host_by_addr() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_host_by_addr(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.Inet6AddressImpl.getLocalHostName()Ljava/lang/String;"
    )]
    async fn test_get_local_host_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_local_host_name(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.Inet6AddressImpl.isReachable0([BII[BII)Z"
    )]
    async fn test_is_reachable_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_reachable_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.Inet6AddressImpl.lookupAllHostAddr(Ljava/lang/String;)[Ljava/net/InetAddress;"
    )]
    async fn test_lookup_all_host_addr() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = lookup_all_host_addr(thread, Arguments::default()).await;
    }
}
