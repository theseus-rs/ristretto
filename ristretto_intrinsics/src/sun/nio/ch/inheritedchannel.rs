use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::{
    Any, GreaterThan, GreaterThanOrEqual, LessThanOrEqual,
};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/nio/ch/InheritedChannel.addressFamily(I)I", GreaterThan(JAVA_11))]
#[async_method]
pub async fn address_family<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.nio.ch.InheritedChannel.addressFamily(I)I".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/nio/ch/InheritedChannel.close0(I)V", Any)]
#[async_method]
pub async fn close_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun.nio.ch.InheritedChannel.close0(I)V".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/nio/ch/InheritedChannel.dup(I)I", Any)]
#[async_method]
pub async fn dup<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("sun.nio.ch.InheritedChannel.dup(I)I".to_string()).into())
}

#[intrinsic_method("sun/nio/ch/InheritedChannel.dup2(II)V", Any)]
#[async_method]
pub async fn dup_2<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd2 = parameters.pop_int()?;
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("sun.nio.ch.InheritedChannel.dup2(II)V".to_string()).into())
}

#[intrinsic_method(
    "sun/nio/ch/InheritedChannel.inetPeerAddress0(I)Ljava/net/InetAddress;",
    GreaterThan(JAVA_11)
)]
#[async_method]
pub async fn inet_peer_address_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.nio.ch.InheritedChannel.inetPeerAddress0(I)Ljava/net/InetAddress;".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/nio/ch/InheritedChannel.initIDs()V", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("sun/nio/ch/InheritedChannel.isConnected(I)Z", GreaterThan(JAVA_11))]
#[async_method]
pub async fn is_connected<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun.nio.ch.InheritedChannel.isConnected(I)Z".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/nio/ch/InheritedChannel.open0(Ljava/lang/String;I)I", Any)]
#[async_method]
pub async fn open_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _oflag = parameters.pop_int()?;
    let _path = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.nio.ch.InheritedChannel.open0(Ljava/lang/String;I)I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/nio/ch/InheritedChannel.peerAddress0(I)Ljava/net/InetAddress;",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn peer_address_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.nio.ch.InheritedChannel.peerAddress0(I)Ljava/net/InetAddress;".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/nio/ch/InheritedChannel.peerPort0(I)I", Any)]
#[async_method]
pub async fn peer_port_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun.nio.ch.InheritedChannel.peerPort0(I)I".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/nio/ch/InheritedChannel.soType0(I)I", Any)]
#[async_method]
pub async fn so_type_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun.nio.ch.InheritedChannel.soType0(I)I".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "sun/nio/ch/InheritedChannel.unixPeerAddress0(I)[B",
    GreaterThan(JAVA_11)
)]
#[async_method]
pub async fn unix_peer_address_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.nio.ch.InheritedChannel.unixPeerAddress0(I)[B".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_close_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = close_0(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun.nio.ch.InheritedChannel.close0(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_dup() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = dup(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun.nio.ch.InheritedChannel.dup(I)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_dup_2() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = dup_2(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)])).await;
        assert_eq!(
            "sun.nio.ch.InheritedChannel.dup2(II)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_peer_port_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = peer_port_0(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun.nio.ch.InheritedChannel.peerPort0(I)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_so_type_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = so_type_0(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun.nio.ch.InheritedChannel.soType0(I)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_inet_peer_address_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = inet_peer_address_0(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun.nio.ch.InheritedChannel.inetPeerAddress0(I)Ljava/net/InetAddress;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_address_family() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = address_family(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun.nio.ch.InheritedChannel.addressFamily(I)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_is_connected() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_connected(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun.nio.ch.InheritedChannel.isConnected(I)Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_open_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = open_0(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun.nio.ch.InheritedChannel.open0(Ljava/lang/String;I)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_peer_address_0() {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let result = peer_address_0(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun.nio.ch.InheritedChannel.peerAddress0(I)Ljava/net/InetAddress;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_unix_peer_address_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = unix_peer_address_0(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun.nio.ch.InheritedChannel.unixPeerAddress0(I)[B",
            result.unwrap_err().to_string()
        );
    }
}
