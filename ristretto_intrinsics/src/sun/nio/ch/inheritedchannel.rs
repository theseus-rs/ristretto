use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::{
    Any, GreaterThan, GreaterThanOrEqual, LessThanOrEqual,
};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/nio/ch/InheritedChannel.addressFamily(I)I", GreaterThan(JAVA_11))]
#[async_method]
pub async fn address_family<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.InheritedChannel.addressFamily(I)I");
}

#[intrinsic_method("sun/nio/ch/InheritedChannel.close0(I)V", Any)]
#[async_method]
pub async fn close_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.InheritedChannel.close0(I)V");
}

#[intrinsic_method("sun/nio/ch/InheritedChannel.dup(I)I", Any)]
#[async_method]
pub async fn dup<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.InheritedChannel.dup(I)I");
}

#[intrinsic_method("sun/nio/ch/InheritedChannel.dup2(II)V", Any)]
#[async_method]
pub async fn dup_2<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.InheritedChannel.dup2(II)V");
}

#[intrinsic_method(
    "sun/nio/ch/InheritedChannel.inetPeerAddress0(I)Ljava/net/InetAddress;",
    GreaterThan(JAVA_11)
)]
#[async_method]
pub async fn inet_peer_address_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.InheritedChannel.inetPeerAddress0(I)Ljava/net/InetAddress;");
}

#[intrinsic_method("sun/nio/ch/InheritedChannel.initIDs()V", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn init_ids<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("sun/nio/ch/InheritedChannel.isConnected(I)Z", GreaterThan(JAVA_11))]
#[async_method]
pub async fn is_connected<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.InheritedChannel.isConnected(I)Z");
}

#[intrinsic_method("sun/nio/ch/InheritedChannel.open0(Ljava/lang/String;I)I", Any)]
#[async_method]
pub async fn open_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.InheritedChannel.open0(Ljava/lang/String;I)I");
}

#[intrinsic_method(
    "sun/nio/ch/InheritedChannel.peerAddress0(I)Ljava/net/InetAddress;",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn peer_address_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.InheritedChannel.peerAddress0(I)Ljava/net/InetAddress;");
}

#[intrinsic_method("sun/nio/ch/InheritedChannel.peerPort0(I)I", Any)]
#[async_method]
pub async fn peer_port_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.InheritedChannel.peerPort0(I)I");
}

#[intrinsic_method("sun/nio/ch/InheritedChannel.soType0(I)I", Any)]
#[async_method]
pub async fn so_type_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.InheritedChannel.soType0(I)I");
}

#[intrinsic_method(
    "sun/nio/ch/InheritedChannel.unixPeerAddress0(I)[B",
    GreaterThan(JAVA_11)
)]
#[async_method]
pub async fn unix_peer_address_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.InheritedChannel.unixPeerAddress0(I)[B");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.InheritedChannel.close0(I)V")]
    async fn test_close_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = close_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.InheritedChannel.dup(I)I")]
    async fn test_dup() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = dup(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.InheritedChannel.dup2(II)V")]
    async fn test_dup_2() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = dup_2(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.InheritedChannel.peerPort0(I)I")]
    async fn test_peer_port_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = peer_port_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.InheritedChannel.soType0(I)I")]
    async fn test_so_type_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = so_type_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.InheritedChannel.inetPeerAddress0(I)Ljava/net/InetAddress;"
    )]
    async fn test_inet_peer_address_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = inet_peer_address_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.InheritedChannel.addressFamily(I)I")]
    async fn test_address_family() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = address_family(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.InheritedChannel.isConnected(I)Z")]
    async fn test_is_connected() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_connected(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.InheritedChannel.open0(Ljava/lang/String;I)I"
    )]
    async fn test_open_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = open_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.InheritedChannel.peerAddress0(I)Ljava/net/InetAddress;"
    )]
    async fn test_peer_address_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = peer_address_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.InheritedChannel.unixPeerAddress0(I)[B"
    )]
    async fn test_unix_peer_address_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = unix_peer_address_0(thread, Parameters::default()).await;
    }
}
