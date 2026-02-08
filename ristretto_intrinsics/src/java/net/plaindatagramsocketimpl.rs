use ristretto_classfile::VersionSpecification::{Between, LessThanOrEqual};
use ristretto_classfile::{JAVA_8, JAVA_11, JAVA_17};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.bind0(ILjava/net/InetAddress;)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn bind_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.bind0(ILjava/net/InetAddress;)V")
}

#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.connect0(Ljava/net/InetAddress;I)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn connect_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.connect0(Ljava/net/InetAddress;I)V")
}

#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.dataAvailable()I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn data_available<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.dataAvailable()I")
}

#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.datagramSocketClose()V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn datagram_socket_close<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.datagramSocketClose()V")
}

#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.datagramSocketCreate()V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn datagram_socket_create<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.datagramSocketCreate()V")
}

#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.disconnect0(I)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn disconnect_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.disconnect0(I)V")
}

#[intrinsic_method("java/net/PlainDatagramSocketImpl.getTTL()B", LessThanOrEqual(JAVA_17))]
#[async_method]
pub async fn get_ttl<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.getTTL()B")
}

#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.getTimeToLive()I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn get_time_to_live<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.getTimeToLive()I")
}

#[intrinsic_method("java/net/PlainDatagramSocketImpl.init()V", LessThanOrEqual(JAVA_17))]
#[async_method]
pub async fn init<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.join(Ljava/net/InetAddress;Ljava/net/NetworkInterface;)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn join<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.net.PlainDatagramSocketImpl.join(Ljava/net/InetAddress;Ljava/net/NetworkInterface;)V"
    )
}

#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.leave(Ljava/net/InetAddress;Ljava/net/NetworkInterface;)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn leave<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.net.PlainDatagramSocketImpl.leave(Ljava/net/InetAddress;Ljava/net/NetworkInterface;)V"
    )
}

#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.peek(Ljava/net/InetAddress;)I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn peek<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.peek(Ljava/net/InetAddress;)I")
}

#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.peekData(Ljava/net/DatagramPacket;)I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn peek_data<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.peekData(Ljava/net/DatagramPacket;)I")
}

#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.receive0(Ljava/net/DatagramPacket;)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn receive_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.receive0(Ljava/net/DatagramPacket;)V")
}

#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.send(Ljava/net/DatagramPacket;)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn send<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.send(Ljava/net/DatagramPacket;)V")
}

#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.send0(Ljava/net/DatagramPacket;)V",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn send_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.send0(Ljava/net/DatagramPacket;)V")
}

#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.setTTL(B)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn set_ttl<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.setTTL(B)V")
}

#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.setTimeToLive(I)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn set_time_to_live<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.setTimeToLive(I)V")
}

#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.socketGetOption(I)Ljava/lang/Object;",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn socket_get_option<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.socketGetOption(I)Ljava/lang/Object;")
}

#[intrinsic_method(
    "java/net/PlainDatagramSocketImpl.socketSetOption0(ILjava/lang/Object;)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn socket_set_option_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.socketSetOption0(ILjava/lang/Object;)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainDatagramSocketImpl.bind0(ILjava/net/InetAddress;)V"
    )]
    async fn test_bind_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = bind_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainDatagramSocketImpl.connect0(Ljava/net/InetAddress;I)V"
    )]
    async fn test_connect_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = connect_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainDatagramSocketImpl.dataAvailable()I"
    )]
    async fn test_data_available() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = data_available(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainDatagramSocketImpl.datagramSocketClose()V"
    )]
    async fn test_datagram_socket_close() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = datagram_socket_close(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainDatagramSocketImpl.datagramSocketCreate()V"
    )]
    async fn test_datagram_socket_create() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = datagram_socket_create(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainDatagramSocketImpl.disconnect0(I)V"
    )]
    async fn test_disconnect_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = disconnect_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.net.PlainDatagramSocketImpl.getTTL()B")]
    async fn test_get_ttl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_ttl(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainDatagramSocketImpl.getTimeToLive()I"
    )]
    async fn test_get_time_to_live() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_time_to_live(thread, Parameters::default()).await;
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
        expected = "not yet implemented: java.net.PlainDatagramSocketImpl.join(Ljava/net/InetAddress;Ljava/net/NetworkInterface;)V"
    )]
    async fn test_join() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = join(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainDatagramSocketImpl.leave(Ljava/net/InetAddress;Ljava/net/NetworkInterface;)V"
    )]
    async fn test_leave() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = leave(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainDatagramSocketImpl.peek(Ljava/net/InetAddress;)I"
    )]
    async fn test_peek() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = peek(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainDatagramSocketImpl.peekData(Ljava/net/DatagramPacket;)I"
    )]
    async fn test_peek_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = peek_data(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainDatagramSocketImpl.receive0(Ljava/net/DatagramPacket;)V"
    )]
    async fn test_receive_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = receive_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainDatagramSocketImpl.send(Ljava/net/DatagramPacket;)V"
    )]
    async fn test_send() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = send(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainDatagramSocketImpl.send0(Ljava/net/DatagramPacket;)V"
    )]
    async fn test_send_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = send_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.net.PlainDatagramSocketImpl.setTTL(B)V")]
    async fn test_set_ttl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_ttl(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainDatagramSocketImpl.setTimeToLive(I)V"
    )]
    async fn test_set_time_to_live() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_time_to_live(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainDatagramSocketImpl.socketGetOption(I)Ljava/lang/Object;"
    )]
    async fn test_socket_get_option() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = socket_get_option(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainDatagramSocketImpl.socketSetOption0(ILjava/lang/Object;)V"
    )]
    async fn test_socket_set_option_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = socket_set_option_0(thread, Parameters::default()).await;
    }
}
