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
    "java/net/TwoStacksPlainDatagramSocketImpl.bind0(ILjava/net/InetAddress;Z)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn bind0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _excl_bind = parameters.pop_bool()?;
    let _address_obj = parameters.pop_reference()?;
    let _port = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/TwoStacksPlainDatagramSocketImpl.bind0(ILjava/net/InetAddress;Z)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "java/net/TwoStacksPlainDatagramSocketImpl.connect0(Ljava/net/InetAddress;I)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn connect0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _port = parameters.pop_int()?;
    let _address = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/TwoStacksPlainDatagramSocketImpl.connect0(Ljava/net/InetAddress;I)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "java/net/TwoStacksPlainDatagramSocketImpl.dataAvailable()I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn data_available<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/TwoStacksPlainDatagramSocketImpl.dataAvailable()I".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "java/net/TwoStacksPlainDatagramSocketImpl.datagramSocketClose()V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn datagram_socket_close<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/TwoStacksPlainDatagramSocketImpl.datagramSocketClose()V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "java/net/TwoStacksPlainDatagramSocketImpl.datagramSocketCreate()V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn datagram_socket_create<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/TwoStacksPlainDatagramSocketImpl.datagramSocketCreate()V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "java/net/TwoStacksPlainDatagramSocketImpl.disconnect0(I)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn disconnect0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _family = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/TwoStacksPlainDatagramSocketImpl.disconnect0(I)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "java/net/TwoStacksPlainDatagramSocketImpl.getTTL()B",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn get_ttl<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/TwoStacksPlainDatagramSocketImpl.getTTL()B".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "java/net/TwoStacksPlainDatagramSocketImpl.getTimeToLive()I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn get_time_to_live<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/TwoStacksPlainDatagramSocketImpl.getTimeToLive()I".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "java/net/TwoStacksPlainDatagramSocketImpl.init()V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn init<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/TwoStacksPlainDatagramSocketImpl.init()V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "java/net/TwoStacksPlainDatagramSocketImpl.join(Ljava/net/InetAddress;Ljava/net/NetworkInterface;)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn join<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ni_obj = parameters.pop_reference()?;
    let _ia_obj = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("java/net/TwoStacksPlainDatagramSocketImpl.join(Ljava/net/InetAddress;Ljava/net/NetworkInterface;)V".to_string()).into())
}
#[intrinsic_method(
    "java/net/TwoStacksPlainDatagramSocketImpl.leave(Ljava/net/InetAddress;Ljava/net/NetworkInterface;)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn leave<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ni_obj = parameters.pop_reference()?;
    let _ia_obj = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("java/net/TwoStacksPlainDatagramSocketImpl.leave(Ljava/net/InetAddress;Ljava/net/NetworkInterface;)V".to_string()).into())
}
#[intrinsic_method(
    "java/net/TwoStacksPlainDatagramSocketImpl.peek(Ljava/net/InetAddress;)I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn peek<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _address_obj = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/TwoStacksPlainDatagramSocketImpl.peek(Ljava/net/InetAddress;)I".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "java/net/TwoStacksPlainDatagramSocketImpl.peekData(Ljava/net/DatagramPacket;)I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn peek_data<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _packet = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/TwoStacksPlainDatagramSocketImpl.peekData(Ljava/net/DatagramPacket;)I"
            .to_string(),
    )
    .into())
}
#[intrinsic_method(
    "java/net/TwoStacksPlainDatagramSocketImpl.receive0(Ljava/net/DatagramPacket;)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn receive0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _packet = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/TwoStacksPlainDatagramSocketImpl.receive0(Ljava/net/DatagramPacket;)V"
            .to_string(),
    )
    .into())
}
#[intrinsic_method(
    "java/net/TwoStacksPlainDatagramSocketImpl.send(Ljava/net/DatagramPacket;)V",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn send<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/TwoStacksPlainDatagramSocketImpl.send(Ljava/net/DatagramPacket;)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "java/net/TwoStacksPlainDatagramSocketImpl.send0(Ljava/net/DatagramPacket;)V",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn send0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _packet = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/TwoStacksPlainDatagramSocketImpl.send0(Ljava/net/DatagramPacket;)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "java/net/TwoStacksPlainDatagramSocketImpl.setTTL(B)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn set_ttl<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ttl = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/TwoStacksPlainDatagramSocketImpl.setTTL(B)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "java/net/TwoStacksPlainDatagramSocketImpl.setTimeToLive(I)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn set_time_to_live<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ttl = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/TwoStacksPlainDatagramSocketImpl.setTimeToLive(I)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "java/net/TwoStacksPlainDatagramSocketImpl.socketGetOption(I)Ljava/lang/Object;",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn socket_get_option<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _opt = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/TwoStacksPlainDatagramSocketImpl.socketGetOption(I)Ljava/lang/Object;"
            .to_string(),
    )
    .into())
}
#[intrinsic_method(
    "java/net/TwoStacksPlainDatagramSocketImpl.socketLocalAddress(I)Ljava/lang/Object;",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn socket_local_address<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _family = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/TwoStacksPlainDatagramSocketImpl.socketLocalAddress(I)Ljava/lang/Object;"
            .to_string(),
    )
    .into())
}
#[intrinsic_method(
    "java/net/TwoStacksPlainDatagramSocketImpl.socketNativeSetOption(ILjava/lang/Object;)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn socket_native_set_option<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _value = parameters.pop_reference()?;
    let _opt = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/net/TwoStacksPlainDatagramSocketImpl.socketNativeSetOption(ILjava/lang/Object;)V"
            .to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_bind0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = bind0(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "java/net/TwoStacksPlainDatagramSocketImpl.bind0(ILjava/net/InetAddress;Z)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_connect0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = connect0(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "java/net/TwoStacksPlainDatagramSocketImpl.connect0(Ljava/net/InetAddress;I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_data_available() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = data_available(thread, Parameters::default()).await;
        assert_eq!(
            "java/net/TwoStacksPlainDatagramSocketImpl.dataAvailable()I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_datagram_socket_close() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = datagram_socket_close(thread, Parameters::default()).await;
        assert_eq!(
            "java/net/TwoStacksPlainDatagramSocketImpl.datagramSocketClose()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_datagram_socket_create() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = datagram_socket_create(thread, Parameters::default()).await;
        assert_eq!(
            "java/net/TwoStacksPlainDatagramSocketImpl.datagramSocketCreate()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_disconnect0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = disconnect0(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "java/net/TwoStacksPlainDatagramSocketImpl.disconnect0(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_ttl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_ttl(thread, Parameters::default()).await;
        assert_eq!(
            "java/net/TwoStacksPlainDatagramSocketImpl.getTTL()B",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_time_to_live() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_time_to_live(thread, Parameters::default()).await;
        assert_eq!(
            "java/net/TwoStacksPlainDatagramSocketImpl.getTimeToLive()I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init(thread, Parameters::default()).await;
        assert_eq!(
            "java/net/TwoStacksPlainDatagramSocketImpl.init()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_join() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = join(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "java/net/TwoStacksPlainDatagramSocketImpl.join(Ljava/net/InetAddress;Ljava/net/NetworkInterface;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_leave() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = leave(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "java/net/TwoStacksPlainDatagramSocketImpl.leave(Ljava/net/InetAddress;Ljava/net/NetworkInterface;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_peek() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = peek(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "java/net/TwoStacksPlainDatagramSocketImpl.peek(Ljava/net/InetAddress;)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_peek_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = peek_data(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "java/net/TwoStacksPlainDatagramSocketImpl.peekData(Ljava/net/DatagramPacket;)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_receive0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = receive0(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "java/net/TwoStacksPlainDatagramSocketImpl.receive0(Ljava/net/DatagramPacket;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_send() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = send(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "java/net/TwoStacksPlainDatagramSocketImpl.send(Ljava/net/DatagramPacket;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_send0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = send0(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "java/net/TwoStacksPlainDatagramSocketImpl.send0(Ljava/net/DatagramPacket;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_ttl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_ttl(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "java/net/TwoStacksPlainDatagramSocketImpl.setTTL(B)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_time_to_live() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_time_to_live(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "java/net/TwoStacksPlainDatagramSocketImpl.setTimeToLive(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_socket_get_option() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = socket_get_option(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "java/net/TwoStacksPlainDatagramSocketImpl.socketGetOption(I)Ljava/lang/Object;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_socket_local_address() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = socket_local_address(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "java/net/TwoStacksPlainDatagramSocketImpl.socketLocalAddress(I)Ljava/lang/Object;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_socket_native_set_option() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = socket_native_set_option(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "java/net/TwoStacksPlainDatagramSocketImpl.socketNativeSetOption(ILjava/lang/Object;)V",
            result.unwrap_err().to_string()
        );
    }
}
