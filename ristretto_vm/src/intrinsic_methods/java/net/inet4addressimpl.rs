use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("java/net/Inet4AddressImpl.getHostByAddr([B)Ljava/lang/String;", Any)]
#[async_method]
pub(crate) async fn get_host_by_addr(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.Inet4AddressImpl.getHostByAddr([B)Ljava/lang/String;")
}

#[intrinsic_method("java/net/Inet4AddressImpl.getLocalHostName()Ljava/lang/String;", Any)]
#[async_method]
pub(crate) async fn get_local_host_name(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.Inet4AddressImpl.getLocalHostName()Ljava/lang/String;")
}

#[intrinsic_method("java/net/Inet4AddressImpl.isReachable0([BI[BI)Z", Any)]
#[async_method]
pub(crate) async fn is_reachable_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.Inet4AddressImpl.isReachable0([BI[BI)Z")
}

#[intrinsic_method(
    "java/net/Inet4AddressImpl.lookupAllHostAddr(Ljava/lang/String;)[Ljava/net/InetAddress;",
    Any
)]
#[async_method]
pub(crate) async fn lookup_all_host_addr(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.Inet4AddressImpl.lookupAllHostAddr(Ljava/lang/String;)[Ljava/net/InetAddress;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.Inet4AddressImpl.getHostByAddr([B)Ljava/lang/String;"
    )]
    async fn test_get_host_by_addr() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_host_by_addr(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.Inet4AddressImpl.getLocalHostName()Ljava/lang/String;"
    )]
    async fn test_get_local_host_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_local_host_name(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.Inet4AddressImpl.isReachable0([BI[BI)Z"
    )]
    async fn test_is_reachable_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_reachable_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.Inet4AddressImpl.lookupAllHostAddr(Ljava/lang/String;)[Ljava/net/InetAddress;"
    )]
    async fn test_lookup_all_host_addr() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = lookup_all_host_addr(thread, Parameters::default()).await;
    }
}
