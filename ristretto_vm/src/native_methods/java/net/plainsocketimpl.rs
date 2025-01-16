use crate::arguments::Arguments;
use crate::native_methods::registry::{MethodRegistry, JAVA_11};
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/net/PlainSocketImpl";

/// Register all native methods for `java.net.PlainSocketImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_11 {
        registry.register(CLASS_NAME, "socketCreate", "(Z)V", socket_create);
    } else {
        registry.register(CLASS_NAME, "socketCreate", "(ZZ)V", socket_create);
    }

    registry.register(CLASS_NAME, "initProto", "()V", init_proto);
    registry.register(
        CLASS_NAME,
        "socketAccept",
        "(Ljava/net/SocketImpl;)V",
        socket_accept,
    );
    registry.register(CLASS_NAME, "socketAvailable", "()I", socket_available);
    registry.register(
        CLASS_NAME,
        "socketBind",
        "(Ljava/net/InetAddress;I)V",
        socket_bind,
    );
    registry.register(CLASS_NAME, "socketClose0", "(Z)V", socket_close_0);
    registry.register(
        CLASS_NAME,
        "socketConnect",
        "(Ljava/net/InetAddress;II)V",
        socket_connect,
    );
    registry.register(
        CLASS_NAME,
        "socketGetOption",
        "(ILjava/lang/Object;)I",
        socket_get_option,
    );
    registry.register(CLASS_NAME, "socketListen", "(I)V", socket_listen);
    registry.register(
        CLASS_NAME,
        "socketSendUrgentData",
        "(I)V",
        socket_send_urgent_data,
    );
    registry.register(
        CLASS_NAME,
        "socketSetOption0",
        "(IZLjava/lang/Object;)V",
        socket_set_option_0,
    );
    registry.register(CLASS_NAME, "socketShutdown", "(I)V", socket_shutdown);
}

#[async_recursion(?Send)]
async fn init_proto(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.PlainSocketImpl.initProto()V")
}

#[async_recursion(?Send)]
async fn socket_accept(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.PlainSocketImpl.socketAccept(Ljava/net/SocketImpl;)V")
}

#[async_recursion(?Send)]
async fn socket_available(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.PlainSocketImpl.socketAvailable()I")
}

#[async_recursion(?Send)]
async fn socket_bind(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.PlainSocketImpl.socketBind(Ljava/net/InetAddress;I)V")
}

#[async_recursion(?Send)]
async fn socket_close_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.PlainSocketImpl.socketClose0(Z)V")
}

#[async_recursion(?Send)]
async fn socket_connect(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.PlainSocketImpl.socketConnect(Ljava/net/InetAddress;II)V")
}

#[async_recursion(?Send)]
async fn socket_create(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.PlainSocketImpl.socketCreate(Z)V")
}

#[async_recursion(?Send)]
async fn socket_get_option(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.PlainSocketImpl.socketGetOption(ILjava/lang/Object;)I")
}

#[async_recursion(?Send)]
async fn socket_listen(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.PlainSocketImpl.socketListen(I)V")
}

#[async_recursion(?Send)]
async fn socket_send_urgent_data(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.net.PlainSocketImpl.socketSendUrgentData(I)V")
}

#[async_recursion(?Send)]
async fn socket_set_option_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.PlainSocketImpl.socketSetOption0(IZLjava/lang/Object;)V")
}

#[async_recursion(?Send)]
async fn socket_shutdown(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.PlainSocketImpl.socketShutdown(I)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.net.PlainSocketImpl.initProto()V")]
    async fn test_init_proto() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_proto(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainSocketImpl.socketAccept(Ljava/net/SocketImpl;)V"
    )]
    async fn test_socket_accept() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = socket_accept(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.net.PlainSocketImpl.socketAvailable()I")]
    async fn test_socket_available() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = socket_available(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainSocketImpl.socketBind(Ljava/net/InetAddress;I)V"
    )]
    async fn test_socket_bind() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = socket_bind(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.net.PlainSocketImpl.socketClose0(Z)V")]
    async fn test_socket_close_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = socket_close_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainSocketImpl.socketConnect(Ljava/net/InetAddress;II)V"
    )]
    async fn test_socket_connect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = socket_connect(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.net.PlainSocketImpl.socketCreate(Z)V")]
    async fn test_socket_create() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = socket_create(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainSocketImpl.socketGetOption(ILjava/lang/Object;)I"
    )]
    async fn test_socket_get_option() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = socket_get_option(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.net.PlainSocketImpl.socketListen(I)V")]
    async fn test_socket_listen() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = socket_listen(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainSocketImpl.socketSendUrgentData(I)V"
    )]
    async fn test_socket_send_urgent_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = socket_send_urgent_data(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainSocketImpl.socketSetOption0(IZLjava/lang/Object;)V"
    )]
    async fn test_socket_set_option_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = socket_set_option_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.net.PlainSocketImpl.socketShutdown(I)V")]
    async fn test_socket_shutdown() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = socket_shutdown(thread, Arguments::default()).await;
    }
}
