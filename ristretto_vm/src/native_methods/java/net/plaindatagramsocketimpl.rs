use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_8: Version = Version::Java8 { minor: 0 };

/// Register all native methods for `java.net.PlainDatagramSocketImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/net/PlainDatagramSocketImpl";
    let java_version = registry.java_version();

    if java_version <= &JAVA_8 {
        registry.register(class_name, "send", "(Ljava/net/DatagramPacket;)V", send);
    } else {
        registry.register(class_name, "send0", "(Ljava/net/DatagramPacket;)V", send_0);
    }

    registry.register(class_name, "bind0", "(ILjava/net/InetAddress;)V", bind_0);
    registry.register(
        class_name,
        "connect0",
        "(Ljava/net/InetAddress;I)V",
        connect_0,
    );
    registry.register(class_name, "dataAvailable", "()I", data_available);
    registry.register(
        class_name,
        "datagramSocketClose",
        "()V",
        datagram_socket_close,
    );
    registry.register(
        class_name,
        "datagramSocketCreate",
        "()V",
        datagram_socket_create,
    );
    registry.register(class_name, "disconnect0", "(I)V", disconnect_0);
    registry.register(class_name, "getTTL", "()B", get_ttl);
    registry.register(class_name, "getTimeToLive", "()I", get_time_to_live);
    registry.register(class_name, "init", "()V", init);
    registry.register(
        class_name,
        "join",
        "(Ljava/net/InetAddress;Ljava/net/NetworkInterface;)V",
        join,
    );
    registry.register(
        class_name,
        "leave",
        "(Ljava/net/InetAddress;Ljava/net/NetworkInterface;)V",
        leave,
    );
    registry.register(class_name, "peek", "(Ljava/net/InetAddress;)I", peek);
    registry.register(
        class_name,
        "peekData",
        "(Ljava/net/DatagramPacket;)I",
        peek_data,
    );
    registry.register(
        class_name,
        "receive0",
        "(Ljava/net/DatagramPacket;)V",
        receive_0,
    );
    registry.register(class_name, "send0", "(Ljava/net/DatagramPacket;)V", send_0);
    registry.register(class_name, "setTTL", "(B)V", set_ttl);
    registry.register(class_name, "setTimeToLive", "(I)V", set_time_to_live);
    registry.register(
        class_name,
        "socketGetOption",
        "(I)Ljava/lang/Object;",
        socket_get_option,
    );
    registry.register(
        class_name,
        "socketSetOption0",
        "(ILjava/lang/Object;)V",
        socket_set_option_0,
    );
}

#[async_recursion(?Send)]
async fn bind_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.bind0(ILjava/net/InetAddress;)V")
}

#[async_recursion(?Send)]
async fn connect_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.connect0(Ljava/net/InetAddress;I)V")
}

#[async_recursion(?Send)]
async fn data_available(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.dataAvailable()I")
}

#[async_recursion(?Send)]
async fn datagram_socket_close(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.datagramSocketClose()V")
}

#[async_recursion(?Send)]
async fn datagram_socket_create(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.datagramSocketCreate()V")
}

#[async_recursion(?Send)]
async fn disconnect_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.disconnect0(I)V")
}

#[async_recursion(?Send)]
async fn get_ttl(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.getTTL()B")
}

#[async_recursion(?Send)]
async fn get_time_to_live(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.getTimeToLive()I")
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn join(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!(
        "java.net.PlainDatagramSocketImpl.join(Ljava/net/InetAddress;Ljava/net/NetworkInterface;)V"
    )
}

#[async_recursion(?Send)]
async fn leave(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.leave(Ljava/net/InetAddress;Ljava/net/NetworkInterface;)V")
}

#[async_recursion(?Send)]
async fn peek(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.peek(Ljava/net/InetAddress;)I")
}

#[async_recursion(?Send)]
async fn peek_data(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.peekData(Ljava/net/DatagramPacket;)I")
}

#[async_recursion(?Send)]
async fn receive_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.receive0(Ljava/net/DatagramPacket;)V")
}

#[async_recursion(?Send)]
async fn send(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.send(Ljava/net/DatagramPacket;)V")
}

#[async_recursion(?Send)]
async fn send_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.send0(Ljava/net/DatagramPacket;)V")
}

#[async_recursion(?Send)]
async fn set_ttl(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.setTTL(B)V")
}

#[async_recursion(?Send)]
async fn set_time_to_live(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.setTimeToLive(I)V")
}

#[async_recursion(?Send)]
async fn socket_get_option(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.socketGetOption(I)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn socket_set_option_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.PlainDatagramSocketImpl.socketSetOption0(ILjava/lang/Object;)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::new(&Version::Java9 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "java/net/PlainDatagramSocketImpl";
        assert!(registry
            .method(class_name, "bind0", "(ILjava/net/InetAddress;)V")
            .is_some());
        assert!(registry
            .method(class_name, "connect0", "(Ljava/net/InetAddress;I)V")
            .is_some());
        assert!(registry
            .method(class_name, "dataAvailable", "()I")
            .is_some());
        assert!(registry
            .method(class_name, "datagramSocketClose", "()V")
            .is_some());
        assert!(registry
            .method(class_name, "datagramSocketCreate", "()V")
            .is_some());
        assert!(registry.method(class_name, "disconnect0", "(I)V").is_some());
        assert!(registry.method(class_name, "getTTL", "()B").is_some());
        assert!(registry
            .method(class_name, "getTimeToLive", "()I")
            .is_some());
        assert!(registry.method(class_name, "init", "()V").is_some());
        assert!(registry
            .method(
                class_name,
                "join",
                "(Ljava/net/InetAddress;Ljava/net/NetworkInterface;)V"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "leave",
                "(Ljava/net/InetAddress;Ljava/net/NetworkInterface;)V"
            )
            .is_some());
        assert!(registry
            .method(class_name, "peek", "(Ljava/net/InetAddress;)I")
            .is_some());
        assert!(registry
            .method(class_name, "peekData", "(Ljava/net/DatagramPacket;)I")
            .is_some());
        assert!(registry
            .method(class_name, "receive0", "(Ljava/net/DatagramPacket;)V")
            .is_some());
        assert!(registry
            .method(class_name, "send0", "(Ljava/net/DatagramPacket;)V")
            .is_some());
        assert!(registry.method(class_name, "setTTL", "(B)V").is_some());
        assert!(registry
            .method(class_name, "setTimeToLive", "(I)V")
            .is_some());
        assert!(registry
            .method(class_name, "socketGetOption", "(I)Ljava/lang/Object;")
            .is_some());
        assert!(registry
            .method(class_name, "socketSetOption0", "(ILjava/lang/Object;)V")
            .is_some());
    }

    fn test_register_java_8() {
        let mut registry = MethodRegistry::new(&Version::Java8 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "java/net/PlainDatagramSocketImpl";
        assert!(registry
            .method(class_name, "send", "(Ljava/net/DatagramPacket;)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainDatagramSocketImpl.bind0(ILjava/net/InetAddress;)V"
    )]
    async fn test_bind_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = bind_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainDatagramSocketImpl.connect0(Ljava/net/InetAddress;I)V"
    )]
    async fn test_connect_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = connect_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainDatagramSocketImpl.dataAvailable()I"
    )]
    async fn test_data_available() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = data_available(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainDatagramSocketImpl.datagramSocketClose()V"
    )]
    async fn test_datagram_socket_close() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = datagram_socket_close(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainDatagramSocketImpl.datagramSocketCreate()V"
    )]
    async fn test_datagram_socket_create() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = datagram_socket_create(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainDatagramSocketImpl.disconnect0(I)V"
    )]
    async fn test_disconnect_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = disconnect_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.net.PlainDatagramSocketImpl.getTTL()B")]
    async fn test_get_ttl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_ttl(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainDatagramSocketImpl.getTimeToLive()I"
    )]
    async fn test_get_time_to_live() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_time_to_live(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init(thread, Arguments::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainDatagramSocketImpl.join(Ljava/net/InetAddress;Ljava/net/NetworkInterface;)V"
    )]
    async fn test_join() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = join(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainDatagramSocketImpl.leave(Ljava/net/InetAddress;Ljava/net/NetworkInterface;)V"
    )]
    async fn test_leave() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = leave(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainDatagramSocketImpl.peek(Ljava/net/InetAddress;)I"
    )]
    async fn test_peek() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = peek(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainDatagramSocketImpl.peekData(Ljava/net/DatagramPacket;)I"
    )]
    async fn test_peek_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = peek_data(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainDatagramSocketImpl.receive0(Ljava/net/DatagramPacket;)V"
    )]
    async fn test_receive_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = receive_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainDatagramSocketImpl.send(Ljava/net/DatagramPacket;)V"
    )]
    async fn test_send() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = send(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainDatagramSocketImpl.send0(Ljava/net/DatagramPacket;)V"
    )]
    async fn test_send_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = send_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.net.PlainDatagramSocketImpl.setTTL(B)V")]
    async fn test_set_ttl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_ttl(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainDatagramSocketImpl.setTimeToLive(I)V"
    )]
    async fn test_set_time_to_live() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_time_to_live(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainDatagramSocketImpl.socketGetOption(I)Ljava/lang/Object;"
    )]
    async fn test_socket_get_option() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = socket_get_option(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainDatagramSocketImpl.socketSetOption0(ILjava/lang/Object;)V"
    )]
    async fn test_socket_set_option_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = socket_set_option_0(thread, Arguments::default()).await;
    }
}
