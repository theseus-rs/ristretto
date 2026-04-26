use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::Equal;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("java/net/TwoStacksPlainSocketImpl.initProto()V", Equal(JAVA_8))]
#[async_method]
pub async fn init_proto<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/TwoStacksPlainSocketImpl.initProto()V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "java/net/TwoStacksPlainSocketImpl.socketAccept(Ljava/net/SocketImpl;)V",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn socket_accept<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/TwoStacksPlainSocketImpl.socketAccept(Ljava/net/SocketImpl;)V".to_string(),
    )
    .into())
}
#[intrinsic_method("java/net/TwoStacksPlainSocketImpl.socketAvailable()I", Equal(JAVA_8))]
#[async_method]
pub async fn socket_available<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/TwoStacksPlainSocketImpl.socketAvailable()I".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "java/net/TwoStacksPlainSocketImpl.socketBind(Ljava/net/InetAddress;IZ)V",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn socket_bind<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg2 = parameters.pop_bool()?;
    let _arg1 = parameters.pop_int()?;
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/TwoStacksPlainSocketImpl.socketBind(Ljava/net/InetAddress;IZ)V".to_string(),
    )
    .into())
}
#[intrinsic_method("java/net/TwoStacksPlainSocketImpl.socketClose0(Z)V", Equal(JAVA_8))]
#[async_method]
pub async fn socket_close0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/TwoStacksPlainSocketImpl.socketClose0(Z)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "java/net/TwoStacksPlainSocketImpl.socketConnect(Ljava/net/InetAddress;II)V",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn socket_connect<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg2 = parameters.pop_int()?;
    let _arg1 = parameters.pop_int()?;
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/TwoStacksPlainSocketImpl.socketConnect(Ljava/net/InetAddress;II)V".to_string(),
    )
    .into())
}
#[intrinsic_method("java/net/TwoStacksPlainSocketImpl.socketCreate(Z)V", Equal(JAVA_8))]
#[async_method]
pub async fn socket_create<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/TwoStacksPlainSocketImpl.socketCreate(Z)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "java/net/TwoStacksPlainSocketImpl.socketGetOption(ILjava/lang/Object;)I",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn socket_get_option<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg1 = parameters.pop_reference()?;
    let _arg0 = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/TwoStacksPlainSocketImpl.socketGetOption(ILjava/lang/Object;)I".to_string(),
    )
    .into())
}
#[intrinsic_method("java/net/TwoStacksPlainSocketImpl.socketListen(I)V", Equal(JAVA_8))]
#[async_method]
pub async fn socket_listen<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/TwoStacksPlainSocketImpl.socketListen(I)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "java/net/TwoStacksPlainSocketImpl.socketNativeSetOption(IZLjava/lang/Object;)V",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn socket_native_set_option<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg2 = parameters.pop_reference()?;
    let _arg1 = parameters.pop_bool()?;
    let _arg0 = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/TwoStacksPlainSocketImpl.socketNativeSetOption(IZLjava/lang/Object;)V"
            .to_string(),
    )
    .into())
}
#[intrinsic_method(
    "java/net/TwoStacksPlainSocketImpl.socketSendUrgentData(I)V",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn socket_send_urgent_data<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/TwoStacksPlainSocketImpl.socketSendUrgentData(I)V".to_string(),
    )
    .into())
}
#[intrinsic_method("java/net/TwoStacksPlainSocketImpl.socketShutdown(I)V", Equal(JAVA_8))]
#[async_method]
pub async fn socket_shutdown<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/TwoStacksPlainSocketImpl.socketShutdown(I)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init_proto() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_proto(thread, Parameters::default()).await;
        assert_eq!(
            "java/net/TwoStacksPlainSocketImpl.initProto()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_socket_accept() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = socket_accept(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "java/net/TwoStacksPlainSocketImpl.socketAccept(Ljava/net/SocketImpl;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_socket_available() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = socket_available(thread, Parameters::default()).await;
        assert_eq!(
            "java/net/TwoStacksPlainSocketImpl.socketAvailable()I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_socket_bind() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = socket_bind(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Int(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "java/net/TwoStacksPlainSocketImpl.socketBind(Ljava/net/InetAddress;IZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_socket_close0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = socket_close0(thread, Parameters::new(vec![Value::from(false)])).await;
        assert_eq!(
            "java/net/TwoStacksPlainSocketImpl.socketClose0(Z)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_socket_connect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = socket_connect(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "java/net/TwoStacksPlainSocketImpl.socketConnect(Ljava/net/InetAddress;II)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_socket_create() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = socket_create(thread, Parameters::new(vec![Value::from(false)])).await;
        assert_eq!(
            "java/net/TwoStacksPlainSocketImpl.socketCreate(Z)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_socket_get_option() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = socket_get_option(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "java/net/TwoStacksPlainSocketImpl.socketGetOption(ILjava/lang/Object;)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_socket_listen() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = socket_listen(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "java/net/TwoStacksPlainSocketImpl.socketListen(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_socket_native_set_option() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = socket_native_set_option(
            thread,
            Parameters::new(vec![Value::Int(0), Value::from(false), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "java/net/TwoStacksPlainSocketImpl.socketNativeSetOption(IZLjava/lang/Object;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_socket_send_urgent_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = socket_send_urgent_data(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "java/net/TwoStacksPlainSocketImpl.socketSendUrgentData(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_socket_shutdown() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = socket_shutdown(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "java/net/TwoStacksPlainSocketImpl.socketShutdown(I)V",
            result.unwrap_err().to_string()
        );
    }
}
