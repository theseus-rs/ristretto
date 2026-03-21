use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.bindx(I[Ljava/net/InetAddress;IIZZ)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn bindx<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.nio.ch.sctp.SctpNet.bindx(I[Ljava/net/InetAddress;IIZZ)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/nio/ch/sctp/SctpNet.branch0(II)I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn branch_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.nio.ch.sctp.SctpNet.branch0(II)I".to_string()).into())
}

#[intrinsic_method("sun/nio/ch/sctp/SctpNet.close0(I)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn close_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.nio.ch.sctp.SctpNet.close0(I)V".to_string()).into())
}

#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.connect0(ILjava/net/InetAddress;I)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn connect_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.nio.ch.sctp.SctpNet.connect0(ILjava/net/InetAddress;I)I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.getInitMsgOption0(I[I)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_init_msg_option_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.nio.ch.sctp.SctpNet.getInitMsgOption0(I[I)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/nio/ch/sctp/SctpNet.getIntOption0(II)I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn get_int_option_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.nio.ch.sctp.SctpNet.getIntOption0(II)I".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.getLocalAddresses0(I)[Ljava/net/SocketAddress;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_local_addresses_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.nio.ch.sctp.SctpNet.getLocalAddresses0(I)[Ljava/net/SocketAddress;".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.getPrimAddrOption0(II)Ljava/net/SocketAddress;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_prim_addr_option_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.nio.ch.sctp.SctpNet.getPrimAddrOption0(II)Ljava/net/SocketAddress;".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.getRemoteAddresses0(II)[Ljava/net/SocketAddress;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_remote_addresses_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.nio.ch.sctp.SctpNet.getRemoteAddresses0(II)[Ljava/net/SocketAddress;".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/nio/ch/sctp/SctpNet.init()V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn init<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("sun/nio/ch/sctp/SctpNet.listen0(II)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn listen_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.nio.ch.sctp.SctpNet.listen0(II)V".to_string()).into())
}

#[intrinsic_method("sun/nio/ch/sctp/SctpNet.preClose0(I)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn pre_close_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.nio.ch.sctp.SctpNet.preClose0(I)V".to_string()).into())
}

#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.setInitMsgOption0(III)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn set_init_msg_option_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.nio.ch.sctp.SctpNet.setInitMsgOption0(III)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/nio/ch/sctp/SctpNet.setIntOption0(III)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn set_int_option_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.nio.ch.sctp.SctpNet.setIntOption0(III)V".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.setPeerPrimAddrOption0(IILjava/net/InetAddress;IZ)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn set_peer_prim_addr_option_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.nio.ch.sctp.SctpNet.setPeerPrimAddrOption0(IILjava/net/InetAddress;IZ)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.setPrimAddrOption0(IILjava/net/InetAddress;I)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn set_prim_addr_option_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.nio.ch.sctp.SctpNet.setPrimAddrOption0(IILjava/net/InetAddress;I)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/nio/ch/sctp/SctpNet.shutdown0(II)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn shutdown_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.nio.ch.sctp.SctpNet.shutdown0(II)V".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/nio/ch/sctp/SctpNet.socket0(Z)I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn socket_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.nio.ch.sctp.SctpNet.socket0(Z)I".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bindx() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = bindx(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_branch_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = branch_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_close_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = close_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_connect_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = connect_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_init_msg_option_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = get_init_msg_option_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_int_option_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = get_int_option_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_local_addresses_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = get_local_addresses_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_prim_addr_option_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = get_prim_addr_option_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_remote_addresses_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = get_remote_addresses_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let result = init(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_listen_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = listen_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_pre_close_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = pre_close_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_init_msg_option_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = set_init_msg_option_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_int_option_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = set_int_option_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_peer_prim_addr_option_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = set_peer_prim_addr_option_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_prim_addr_option_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = set_prim_addr_option_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_shutdown_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = shutdown_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_socket_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = socket_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
