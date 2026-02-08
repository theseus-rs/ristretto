use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.bindx(I[Ljava/net/InetAddress;IIZZ)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn bindx<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.sctp.SctpNet.bindx(I[Ljava/net/InetAddress;IIZZ)V")
}

#[intrinsic_method("sun/nio/ch/sctp/SctpNet.branch0(II)I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn branch_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.sctp.SctpNet.branch0(II)I")
}

#[intrinsic_method("sun/nio/ch/sctp/SctpNet.close0(I)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn close_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.sctp.SctpNet.close0(I)V")
}

#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.connect0(ILjava/net/InetAddress;I)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn connect_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.sctp.SctpNet.connect0(ILjava/net/InetAddress;I)I")
}

#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.getInitMsgOption0(I[I)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_init_msg_option_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.sctp.SctpNet.getInitMsgOption0(I[I)V")
}

#[intrinsic_method("sun/nio/ch/sctp/SctpNet.getIntOption0(II)I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn get_int_option_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.sctp.SctpNet.getIntOption0(II)I")
}

#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.getLocalAddresses0(I)[Ljava/net/SocketAddress;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_local_addresses_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.sctp.SctpNet.getLocalAddresses0(I)[Ljava/net/SocketAddress;")
}

#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.getPrimAddrOption0(II)Ljava/net/SocketAddress;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_prim_addr_option_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.sctp.SctpNet.getPrimAddrOption0(II)Ljava/net/SocketAddress;")
}

#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.getRemoteAddresses0(II)[Ljava/net/SocketAddress;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_remote_addresses_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.sctp.SctpNet.getRemoteAddresses0(II)[Ljava/net/SocketAddress;")
}

#[intrinsic_method("sun/nio/ch/sctp/SctpNet.init()V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn init<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("sun/nio/ch/sctp/SctpNet.listen0(II)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn listen_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.sctp.SctpNet.listen0(II)V")
}

#[intrinsic_method("sun/nio/ch/sctp/SctpNet.preClose0(I)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn pre_close_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.sctp.SctpNet.preClose0(I)V")
}

#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.setInitMsgOption0(III)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn set_init_msg_option_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.sctp.SctpNet.setInitMsgOption0(III)V")
}

#[intrinsic_method("sun/nio/ch/sctp/SctpNet.setIntOption0(III)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn set_int_option_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.sctp.SctpNet.setIntOption0(III)V")
}

#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.setPeerPrimAddrOption0(IILjava/net/InetAddress;IZ)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn set_peer_prim_addr_option_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.sctp.SctpNet.setPeerPrimAddrOption0(IILjava/net/InetAddress;IZ)V")
}

#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.setPrimAddrOption0(IILjava/net/InetAddress;I)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn set_prim_addr_option_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.sctp.SctpNet.setPrimAddrOption0(IILjava/net/InetAddress;I)V")
}

#[intrinsic_method("sun/nio/ch/sctp/SctpNet.shutdown0(II)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn shutdown_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.sctp.SctpNet.shutdown0(II)V")
}

#[intrinsic_method("sun/nio/ch/sctp/SctpNet.socket0(Z)I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn socket_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.sctp.SctpNet.socket0(Z)I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.sctp.SctpNet.bindx(I[Ljava/net/InetAddress;IIZZ)V"
    )]
    async fn test_bindx() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = bindx(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.sctp.SctpNet.branch0(II)I")]
    async fn test_branch_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = branch_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.sctp.SctpNet.close0(I)V")]
    async fn test_close_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = close_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.sctp.SctpNet.connect0(ILjava/net/InetAddress;I)I"
    )]
    async fn test_connect_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = connect_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.sctp.SctpNet.getInitMsgOption0(I[I)V"
    )]
    async fn test_get_init_msg_option_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_init_msg_option_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.sctp.SctpNet.getIntOption0(II)I")]
    async fn test_get_int_option_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_int_option_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.sctp.SctpNet.getLocalAddresses0(I)[Ljava/net/SocketAddress;"
    )]
    async fn test_get_local_addresses_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_local_addresses_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.sctp.SctpNet.getPrimAddrOption0(II)Ljava/net/SocketAddress;"
    )]
    async fn test_get_prim_addr_option_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_prim_addr_option_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.sctp.SctpNet.getRemoteAddresses0(II)[Ljava/net/SocketAddress;"
    )]
    async fn test_get_remote_addresses_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_remote_addresses_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.sctp.SctpNet.listen0(II)V")]
    async fn test_listen_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = listen_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.sctp.SctpNet.preClose0(I)V")]
    async fn test_pre_close_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = pre_close_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.sctp.SctpNet.setInitMsgOption0(III)V"
    )]
    async fn test_set_init_msg_option_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_init_msg_option_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.sctp.SctpNet.setIntOption0(III)V")]
    async fn test_set_int_option_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_int_option_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.sctp.SctpNet.setPeerPrimAddrOption0(IILjava/net/InetAddress;IZ)V"
    )]
    async fn test_set_peer_prim_addr_option_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_peer_prim_addr_option_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.sctp.SctpNet.setPrimAddrOption0(IILjava/net/InetAddress;I)V"
    )]
    async fn test_set_prim_addr_option_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_prim_addr_option_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.sctp.SctpNet.shutdown0(II)V")]
    async fn test_shutdown_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = shutdown_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.sctp.SctpNet.socket0(Z)I")]
    async fn test_socket_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = socket_0(thread, Parameters::default()).await;
    }
}
