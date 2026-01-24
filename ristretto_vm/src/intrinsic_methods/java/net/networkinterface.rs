use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "java/net/NetworkInterface.boundInetAddress0(Ljava/net/InetAddress;)Z",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub(crate) async fn bound_inet_address_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.NetworkInterface.boundInetAddress0(Ljava/net/InetAddress;)Z")
}

#[intrinsic_method("java/net/NetworkInterface.getAll()[Ljava/net/NetworkInterface;", Any)]
#[async_method]
pub(crate) async fn get_all(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.NetworkInterface.getAll()[Ljava/net/NetworkInterface;")
}

#[intrinsic_method(
    "java/net/NetworkInterface.getByIndex0(I)Ljava/net/NetworkInterface;",
    Any
)]
#[async_method]
pub(crate) async fn get_by_index_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.NetworkInterface.getByIndex0(I)Ljava/net/NetworkInterface;")
}

#[intrinsic_method(
    "java/net/NetworkInterface.getByInetAddress0(Ljava/net/InetAddress;)Ljava/net/NetworkInterface;",
    Any
)]
#[async_method]
pub(crate) async fn get_by_inet_address_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.net.NetworkInterface.getByInetAddress0(Ljava/net/InetAddress;)Ljava/net/NetworkInterface;"
    )
}

#[intrinsic_method(
    "java/net/NetworkInterface.getByName0(Ljava/lang/String;)Ljava/net/NetworkInterface;",
    Any
)]
#[async_method]
pub(crate) async fn get_by_name_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.NetworkInterface.getByName0(Ljava/lang/String;)Ljava/net/NetworkInterface;")
}

#[intrinsic_method("java/net/NetworkInterface.getMTU0(Ljava/lang/String;I)I", Any)]
#[async_method]
pub(crate) async fn get_mtu_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.NetworkInterface.getMTU0(Ljava/lang/String;I)I")
}

#[intrinsic_method("java/net/NetworkInterface.getMacAddr0([BLjava/lang/String;I)[B", Any)]
#[async_method]
pub(crate) async fn get_mac_addr_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.NetworkInterface.getMacAddr0([BLjava/lang/String;I)[B")
}

#[intrinsic_method("java/net/NetworkInterface.init()V", Any)]
#[async_method]
pub(crate) async fn init(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/net/NetworkInterface.isLoopback0(Ljava/lang/String;I)Z", Any)]
#[async_method]
pub(crate) async fn is_loopback_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.NetworkInterface.isLoopback0(Ljava/lang/String;I)Z")
}

#[intrinsic_method("java/net/NetworkInterface.isP2P0(Ljava/lang/String;I)Z", Any)]
#[async_method]
pub(crate) async fn is_p2p_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.NetworkInterface.isP2P0(Ljava/lang/String;I)Z")
}

#[intrinsic_method("java/net/NetworkInterface.isUp0(Ljava/lang/String;I)Z", Any)]
#[async_method]
pub(crate) async fn is_up_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.NetworkInterface.isUp0(Ljava/lang/String;I)Z")
}

#[intrinsic_method(
    "java/net/NetworkInterface.supportsMulticast0(Ljava/lang/String;I)Z",
    Any
)]
#[async_method]
pub(crate) async fn supports_multicast_0(
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
