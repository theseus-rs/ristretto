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
