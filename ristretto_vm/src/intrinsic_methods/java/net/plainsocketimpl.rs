use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::VersionSpecification::{Equal, LessThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_17};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("java/net/PlainSocketImpl.initProto()V", LessThanOrEqual(JAVA_17))]
#[async_method]
pub(crate) async fn init_proto(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.PlainSocketImpl.initProto()V")
}

#[intrinsic_method(
    "java/net/PlainSocketImpl.socketAccept(Ljava/net/SocketImpl;)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub(crate) async fn socket_accept(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.PlainSocketImpl.socketAccept(Ljava/net/SocketImpl;)V")
}

#[intrinsic_method(
    "java/net/PlainSocketImpl.socketAvailable()I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub(crate) async fn socket_available(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.PlainSocketImpl.socketAvailable()I")
}

#[intrinsic_method(
    "java/net/PlainSocketImpl.socketBind(Ljava/net/InetAddress;I)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub(crate) async fn socket_bind(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.PlainSocketImpl.socketBind(Ljava/net/InetAddress;I)V")
}

#[intrinsic_method("java/net/PlainSocketImpl.socketClose0(Z)V", LessThanOrEqual(JAVA_17))]
#[async_method]
pub(crate) async fn socket_close_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.PlainSocketImpl.socketClose0(Z)V")
}

#[intrinsic_method(
    "java/net/PlainSocketImpl.socketConnect(Ljava/net/InetAddress;II)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub(crate) async fn socket_connect(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.PlainSocketImpl.socketConnect(Ljava/net/InetAddress;II)V")
}

#[intrinsic_method("java/net/PlainSocketImpl.socketCreate(Z)V", LessThanOrEqual(JAVA_11))]
#[async_method]
pub(crate) async fn socket_create_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.PlainSocketImpl.socketCreate(Z)V")
}

#[intrinsic_method("java/net/PlainSocketImpl.socketCreate(ZZ)V", Equal(JAVA_17))]
#[async_method]
pub(crate) async fn socket_create_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.PlainSocketImpl.socketCreate(ZZ)V")
}

#[intrinsic_method(
    "java/net/PlainSocketImpl.socketGetOption(ILjava/lang/Object;)I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub(crate) async fn socket_get_option(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.PlainSocketImpl.socketGetOption(ILjava/lang/Object;)I")
}

#[intrinsic_method("java/net/PlainSocketImpl.socketListen(I)V", LessThanOrEqual(JAVA_17))]
#[async_method]
pub(crate) async fn socket_listen(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.PlainSocketImpl.socketListen(I)V")
}

#[intrinsic_method(
    "java/net/PlainSocketImpl.socketSendUrgentData(I)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub(crate) async fn socket_send_urgent_data(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.PlainSocketImpl.socketSendUrgentData(I)V")
}

#[intrinsic_method(
    "java/net/PlainSocketImpl.socketSetOption0(IZLjava/lang/Object;)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub(crate) async fn socket_set_option_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.PlainSocketImpl.socketSetOption0(IZLjava/lang/Object;)V")
}

#[intrinsic_method(
    "java/net/PlainSocketImpl.socketShutdown(I)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub(crate) async fn socket_shutdown(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.PlainSocketImpl.socketShutdown(I)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.net.PlainSocketImpl.initProto()V")]
    async fn test_init_proto() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_proto(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainSocketImpl.socketAccept(Ljava/net/SocketImpl;)V"
    )]
    async fn test_socket_accept() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = socket_accept(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.net.PlainSocketImpl.socketAvailable()I")]
    async fn test_socket_available() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = socket_available(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainSocketImpl.socketBind(Ljava/net/InetAddress;I)V"
    )]
    async fn test_socket_bind() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = socket_bind(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.net.PlainSocketImpl.socketClose0(Z)V")]
    async fn test_socket_close_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = socket_close_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainSocketImpl.socketConnect(Ljava/net/InetAddress;II)V"
    )]
    async fn test_socket_connect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = socket_connect(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.net.PlainSocketImpl.socketCreate(Z)V")]
    async fn test_socket_create_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = socket_create_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.net.PlainSocketImpl.socketCreate(ZZ)V")]
    async fn test_socket_create_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = socket_create_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainSocketImpl.socketGetOption(ILjava/lang/Object;)I"
    )]
    async fn test_socket_get_option() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = socket_get_option(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.net.PlainSocketImpl.socketListen(I)V")]
    async fn test_socket_listen() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = socket_listen(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainSocketImpl.socketSendUrgentData(I)V"
    )]
    async fn test_socket_send_urgent_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = socket_send_urgent_data(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.PlainSocketImpl.socketSetOption0(IZLjava/lang/Object;)V"
    )]
    async fn test_socket_set_option_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = socket_set_option_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.net.PlainSocketImpl.socketShutdown(I)V")]
    async fn test_socket_shutdown() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = socket_shutdown(thread, Parameters::default()).await;
    }
}
