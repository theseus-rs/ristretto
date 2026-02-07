use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::{Any, GreaterThan, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("java/net/Inet6AddressImpl.getHostByAddr([B)Ljava/lang/String;", Any)]
#[async_method]
pub async fn get_host_by_addr<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.Inet6AddressImpl.getHostByAddr([B)Ljava/lang/String;")
}

#[intrinsic_method("java/net/Inet6AddressImpl.getLocalHostName()Ljava/lang/String;", Any)]
#[async_method]
pub async fn get_local_host_name<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.Inet6AddressImpl.getLocalHostName()Ljava/lang/String;")
}

#[intrinsic_method("java/net/Inet6AddressImpl.isReachable0([BII[BII)Z", Any)]
#[async_method]
pub async fn is_reachable_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.Inet6AddressImpl.isReachable0([BII[BII)Z")
}

#[intrinsic_method(
    "java/net/Inet6AddressImpl.lookupAllHostAddr(Ljava/lang/String;)[Ljava/net/InetAddress;",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn lookup_all_host_addr_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.Inet6AddressImpl.lookupAllHostAddr(Ljava/lang/String;)[Ljava/net/InetAddress;")
}

#[intrinsic_method(
    "java/net/Inet6AddressImpl.lookupAllHostAddr(Ljava/lang/String;I)[Ljava/net/InetAddress;",
    GreaterThan(JAVA_17)
)]
#[async_method]
pub async fn lookup_all_host_addr_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.Inet6AddressImpl.lookupAllHostAddr(Ljava/lang/String;I)[Ljava/net/InetAddress;")
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
        let _ = get_host_by_addr(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.Inet6AddressImpl.getLocalHostName()Ljava/lang/String;"
    )]
    async fn test_get_local_host_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_local_host_name(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.Inet6AddressImpl.isReachable0([BII[BII)Z"
    )]
    async fn test_is_reachable_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_reachable_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.Inet6AddressImpl.lookupAllHostAddr(Ljava/lang/String;)[Ljava/net/InetAddress;"
    )]
    async fn test_lookup_all_host_addr_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = lookup_all_host_addr_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.Inet6AddressImpl.lookupAllHostAddr(Ljava/lang/String;I)[Ljava/net/InetAddress;"
    )]
    async fn test_lookup_all_host_addr_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = lookup_all_host_addr_1(thread, Parameters::default()).await;
    }
}
