use crate::Result;
use crate::native_methods::registry::{JAVA_8, MethodRegistry};
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/net/PlainDatagramSocketImpl";

/// Register all native methods for `java.net.PlainDatagramSocketImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_8 {
        registry.register(CLASS_NAME, "send", "(Ljava/net/DatagramPacket;)V", send);
    } else {
        registry.register(CLASS_NAME, "send0", "(Ljava/net/DatagramPacket;)V", send_0);
    }

    registry.register(CLASS_NAME, "bind0", "(ILjava/net/InetAddress;)V", bind_0);
    registry.register(
        CLASS_NAME,
        "connect0",
        "(Ljava/net/InetAddress;I)V",
        connect_0,
    );
    registry.register(CLASS_NAME, "dataAvailable", "()I", data_available);
    registry.register(
        CLASS_NAME,
        "datagramSocketClose",
        "()V",
        datagram_socket_close,
    );
    registry.register(
        CLASS_NAME,
        "datagramSocketCreate",
        "()V",
        datagram_socket_create,
    );
    registry.register(CLASS_NAME, "disconnect0", "(I)V", disconnect_0);
    registry.register(CLASS_NAME, "getTTL", "()B", get_ttl);
    registry.register(CLASS_NAME, "getTimeToLive", "()I", get_time_to_live);
    registry.register(CLASS_NAME, "init", "()V", init);
    registry.register(
        CLASS_NAME,
        "join",
        "(Ljava/net/InetAddress;Ljava/net/NetworkInterface;)V",
        join,
    );
    registry.register(
        CLASS_NAME,
        "leave",
        "(Ljava/net/InetAddress;Ljava/net/NetworkInterface;)V",
        leave,
    );
    registry.register(CLASS_NAME, "peek", "(Ljava/net/InetAddress;)I", peek);
    registry.register(
        CLASS_NAME,
        "peekData",
        "(Ljava/net/DatagramPacket;)I",
        peek_data,
    );
    registry.register(
        CLASS_NAME,
        "receive0",
        "(Ljava/net/DatagramPacket;)V",
        receive_0,
    );
    registry.register(CLASS_NAME, "setTTL", "(B)V", set_ttl);
    registry.register(CLASS_NAME, "setTimeToLive", "(I)V", set_time_to_live);
    registry.register(
        CLASS_NAME,
        "socketGetOption",
        "(I)Ljava/lang/Object;",
        socket_get_option,
    );
    registry.register(
        CLASS_NAME,
        "socketSetOption0",
        "(ILjava/lang/Object;)V",
        socket_set_option_0,
    );
}

#[async_recursion(?Send)]
async fn bind_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.bind0(ILjava/net/InetAddress;)V")
}

#[async_recursion(?Send)]
async fn connect_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.connect0(Ljava/net/InetAddress;I)V")
}

#[async_recursion(?Send)]
async fn data_available(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.dataAvailable()I")
}

#[async_recursion(?Send)]
async fn datagram_socket_close(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.datagramSocketClose()V")
}

#[async_recursion(?Send)]
async fn datagram_socket_create(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.datagramSocketCreate()V")
}

#[async_recursion(?Send)]
async fn disconnect_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.disconnect0(I)V")
}

#[async_recursion(?Send)]
async fn get_ttl(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.getTTL()B")
}

#[async_recursion(?Send)]
async fn get_time_to_live(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.getTimeToLive()I")
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn join(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.net.PlainDatagramSocketImpl.join(Ljava/net/InetAddress;Ljava/net/NetworkInterface;)V"
    )
}

#[async_recursion(?Send)]
async fn leave(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.net.PlainDatagramSocketImpl.leave(Ljava/net/InetAddress;Ljava/net/NetworkInterface;)V"
    )
}

#[async_recursion(?Send)]
async fn peek(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.peek(Ljava/net/InetAddress;)I")
}

#[async_recursion(?Send)]
async fn peek_data(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.peekData(Ljava/net/DatagramPacket;)I")
}

#[async_recursion(?Send)]
async fn receive_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.receive0(Ljava/net/DatagramPacket;)V")
}

#[async_recursion(?Send)]
async fn send(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.send(Ljava/net/DatagramPacket;)V")
}

#[async_recursion(?Send)]
async fn send_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.send0(Ljava/net/DatagramPacket;)V")
}

#[async_recursion(?Send)]
async fn set_ttl(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.setTTL(B)V")
}

#[async_recursion(?Send)]
async fn set_time_to_live(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.setTimeToLive(I)V")
}

#[async_recursion(?Send)]
async fn socket_get_option(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.socketGetOption(I)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn socket_set_option_0(
    _thread: Arc<Thread>,
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
