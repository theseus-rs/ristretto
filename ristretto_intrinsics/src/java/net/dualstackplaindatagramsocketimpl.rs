use ristretto_classfile::VersionSpecification::{Between, Equal, LessThanOrEqual};
use ristretto_classfile::{JAVA_8, JAVA_11, JAVA_17};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "java/net/DualStackPlainDatagramSocketImpl.dataAvailable()I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn data_available<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/DualStackPlainDatagramSocketImpl.dataAvailable()I".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "java/net/DualStackPlainDatagramSocketImpl.initIDs()V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/DualStackPlainDatagramSocketImpl.initIDs()V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "java/net/DualStackPlainDatagramSocketImpl.socketBind(ILjava/net/InetAddress;IZ)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn socket_bind<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _excl_bind = parameters.pop_bool()?;
    let _port = parameters.pop_int()?;
    let _ia_obj = parameters.pop_reference()?;
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/DualStackPlainDatagramSocketImpl.socketBind(ILjava/net/InetAddress;IZ)V"
            .to_string(),
    )
    .into())
}
#[intrinsic_method(
    "java/net/DualStackPlainDatagramSocketImpl.socketClose(I)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn socket_close<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/DualStackPlainDatagramSocketImpl.socketClose(I)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "java/net/DualStackPlainDatagramSocketImpl.socketConnect(ILjava/net/InetAddress;I)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn socket_connect<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _port = parameters.pop_int()?;
    let _ia_obj = parameters.pop_reference()?;
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/DualStackPlainDatagramSocketImpl.socketConnect(ILjava/net/InetAddress;I)V"
            .to_string(),
    )
    .into())
}
#[intrinsic_method(
    "java/net/DualStackPlainDatagramSocketImpl.socketCreate()I",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn socket_create<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/DualStackPlainDatagramSocketImpl.socketCreate()I".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "java/net/DualStackPlainDatagramSocketImpl.socketCreate(Z)I",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn socket_create_windows_v8<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/DualStackPlainDatagramSocketImpl.socketCreate(Z)I".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "java/net/DualStackPlainDatagramSocketImpl.socketDisconnect(I)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn socket_disconnect<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/DualStackPlainDatagramSocketImpl.socketDisconnect(I)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "java/net/DualStackPlainDatagramSocketImpl.socketGetIntOption(II)I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn socket_get_int_option<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _cmd = parameters.pop_int()?;
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/DualStackPlainDatagramSocketImpl.socketGetIntOption(II)I".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "java/net/DualStackPlainDatagramSocketImpl.socketLocalAddress(I)Ljava/lang/Object;",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn socket_local_address<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/DualStackPlainDatagramSocketImpl.socketLocalAddress(I)Ljava/lang/Object;"
            .to_string(),
    )
    .into())
}
#[intrinsic_method(
    "java/net/DualStackPlainDatagramSocketImpl.socketLocalPort(I)I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn socket_local_port<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/DualStackPlainDatagramSocketImpl.socketLocalPort(I)I".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "java/net/DualStackPlainDatagramSocketImpl.socketReceiveOrPeekData(ILjava/net/DatagramPacket;IZZ)I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn socket_receive_or_peek_data<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _peek = parameters.pop_bool()?;
    let _connected = parameters.pop_bool()?;
    let _timeout = parameters.pop_int()?;
    let _dp_obj = parameters.pop_reference()?;
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("java/net/DualStackPlainDatagramSocketImpl.socketReceiveOrPeekData(ILjava/net/DatagramPacket;IZZ)I".to_string()).into())
}
#[intrinsic_method(
    "java/net/DualStackPlainDatagramSocketImpl.socketSend(I[BIILjava/net/InetAddress;IZ)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn socket_send<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _connected = parameters.pop_bool()?;
    let _port = parameters.pop_int()?;
    let _ia_obj = parameters.pop_reference()?;
    let _length = parameters.pop_int()?;
    let _offset = parameters.pop_int()?;
    let _data = parameters.pop_reference()?;
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/DualStackPlainDatagramSocketImpl.socketSend(I[BIILjava/net/InetAddress;IZ)V"
            .to_string(),
    )
    .into())
}
#[intrinsic_method(
    "java/net/DualStackPlainDatagramSocketImpl.socketSetIntOption(III)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn socket_set_int_option<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _value = parameters.pop_int()?;
    let _cmd = parameters.pop_int()?;
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/DualStackPlainDatagramSocketImpl.socketSetIntOption(III)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_data_available() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = data_available(thread, Parameters::default()).await;
        assert_eq!(
            "java/net/DualStackPlainDatagramSocketImpl.dataAvailable()I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init_ids() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await;
        assert_eq!(
            "java/net/DualStackPlainDatagramSocketImpl.initIDs()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_socket_bind() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = socket_bind(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "java/net/DualStackPlainDatagramSocketImpl.socketBind(ILjava/net/InetAddress;IZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_socket_close() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = socket_close(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "java/net/DualStackPlainDatagramSocketImpl.socketClose(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_socket_connect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = socket_connect(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "java/net/DualStackPlainDatagramSocketImpl.socketConnect(ILjava/net/InetAddress;I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_socket_create() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = socket_create(thread, Parameters::default()).await;
        assert_eq!(
            "java/net/DualStackPlainDatagramSocketImpl.socketCreate()I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_socket_create_windows_v8() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            socket_create_windows_v8(thread, Parameters::new(vec![Value::from(false)])).await;
        assert_eq!(
            "java/net/DualStackPlainDatagramSocketImpl.socketCreate(Z)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_socket_disconnect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = socket_disconnect(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "java/net/DualStackPlainDatagramSocketImpl.socketDisconnect(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_socket_get_int_option() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            socket_get_int_option(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)]))
                .await;
        assert_eq!(
            "java/net/DualStackPlainDatagramSocketImpl.socketGetIntOption(II)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_socket_local_address() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = socket_local_address(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "java/net/DualStackPlainDatagramSocketImpl.socketLocalAddress(I)Ljava/lang/Object;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_socket_local_port() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = socket_local_port(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "java/net/DualStackPlainDatagramSocketImpl.socketLocalPort(I)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_socket_receive_or_peek_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = socket_receive_or_peek_data(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::from(false),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "java/net/DualStackPlainDatagramSocketImpl.socketReceiveOrPeekData(ILjava/net/DatagramPacket;IZZ)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_socket_send() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = socket_send(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "java/net/DualStackPlainDatagramSocketImpl.socketSend(I[BIILjava/net/InetAddress;IZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_socket_set_int_option() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = socket_set_int_option(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "java/net/DualStackPlainDatagramSocketImpl.socketSetIntOption(III)V",
            result.unwrap_err().to_string()
        );
    }
}
