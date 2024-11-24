use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.net.PlainDatagramSocketImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/net/PlainDatagramSocketImpl";
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
    registry.register(class_name, "send", "(Ljava/net/DatagramPacket;)V", send);
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

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn bind_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn connect_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn data_available(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn datagram_socket_close(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn datagram_socket_create(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn disconnect_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_ttl(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_time_to_live(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn join(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn leave(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn peek(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn peek_data(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn receive_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn send(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_ttl(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_time_to_live(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn socket_get_option(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn socket_set_option_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
