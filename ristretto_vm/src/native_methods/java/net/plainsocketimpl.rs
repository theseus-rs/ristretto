use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_11: Version = Version::Java11 { minor: 0 };

/// Register all native methods for `java.net.PlainSocketImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/net/PlainSocketImpl";
    let java_version = registry.java_version();

    if java_version <= &JAVA_11 {
        registry.register(class_name, "socketCreate", "(Z)V", socket_create);
    } else {
        registry.register(class_name, "socketCreate", "(ZZ)V", socket_create);
    }

    registry.register(class_name, "initProto", "()V", init_proto);
    registry.register(
        class_name,
        "socketAccept",
        "(Ljava/net/SocketImpl;)V",
        socket_accept,
    );
    registry.register(class_name, "socketAvailable", "()I", socket_available);
    registry.register(
        class_name,
        "socketBind",
        "(Ljava/net/InetAddress;I)V",
        socket_bind,
    );
    registry.register(class_name, "socketClose0", "(Z)V", socket_close_0);
    registry.register(
        class_name,
        "socketConnect",
        "(Ljava/net/InetAddress;II)V",
        socket_connect,
    );
    registry.register(class_name, "socketCreate", "(ZZ)V", socket_create);
    registry.register(
        class_name,
        "socketGetOption",
        "(ILjava/lang/Object;)I",
        socket_get_option,
    );
    registry.register(class_name, "socketListen", "(I)V", socket_listen);
    registry.register(
        class_name,
        "socketSendUrgentData",
        "(I)V",
        socket_send_urgent_data,
    );
    registry.register(
        class_name,
        "socketSetOption0",
        "(IZLjava/lang/Object;)V",
        socket_set_option_0,
    );
    registry.register(class_name, "socketShutdown", "(I)V", socket_shutdown);
}

#[async_recursion(?Send)]
async fn init_proto(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn socket_accept(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn socket_available(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn socket_bind(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn socket_close_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn socket_connect(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn socket_create(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn socket_get_option(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn socket_listen(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn socket_send_urgent_data(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn socket_set_option_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn socket_shutdown(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
