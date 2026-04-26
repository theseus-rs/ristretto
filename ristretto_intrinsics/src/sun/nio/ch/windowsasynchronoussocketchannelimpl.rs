use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/nio/ch/WindowsAsynchronousSocketChannelImpl.closesocket0(J)V",
    Any
)]
#[async_method]
pub async fn closesocket0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _socket = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/ch/WindowsAsynchronousSocketChannelImpl.closesocket0(J)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/nio/ch/WindowsAsynchronousSocketChannelImpl.connect0(JZLjava/net/InetAddress;IJ)I",
    Any
)]
#[async_method]
pub async fn connect0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ov = parameters.pop_long()?;
    let _port = parameters.pop_int()?;
    let _iao = parameters.pop_reference()?;
    let _prefer_i_pv6 = parameters.pop_bool()?;
    let _socket = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/ch/WindowsAsynchronousSocketChannelImpl.connect0(JZLjava/net/InetAddress;IJ)I"
            .to_string(),
    )
    .into())
}
#[intrinsic_method("sun/nio/ch/WindowsAsynchronousSocketChannelImpl.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/ch/WindowsAsynchronousSocketChannelImpl.initIDs()V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/nio/ch/WindowsAsynchronousSocketChannelImpl.read0(JIJJ)I", Any)]
#[async_method]
pub async fn read0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ov = parameters.pop_long()?;
    let _address = parameters.pop_long()?;
    let _count = parameters.pop_int()?;
    let _socket = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/ch/WindowsAsynchronousSocketChannelImpl.read0(JIJJ)I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/nio/ch/WindowsAsynchronousSocketChannelImpl.shutdown0(JI)V", Any)]
#[async_method]
pub async fn shutdown0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _how = parameters.pop_int()?;
    let _socket = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/ch/WindowsAsynchronousSocketChannelImpl.shutdown0(JI)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/nio/ch/WindowsAsynchronousSocketChannelImpl.updateConnectContext(J)V",
    Any
)]
#[async_method]
pub async fn update_connect_context<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _socket = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/ch/WindowsAsynchronousSocketChannelImpl.updateConnectContext(J)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/nio/ch/WindowsAsynchronousSocketChannelImpl.write0(JIJJ)I", Any)]
#[async_method]
pub async fn write0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ov = parameters.pop_long()?;
    let _address = parameters.pop_long()?;
    let _count = parameters.pop_int()?;
    let _socket = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/ch/WindowsAsynchronousSocketChannelImpl.write0(JIJJ)I".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_closesocket0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = closesocket0(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/ch/WindowsAsynchronousSocketChannelImpl.closesocket0(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_connect0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = connect0(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::from(false),
                Value::Object(None),
                Value::Int(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/nio/ch/WindowsAsynchronousSocketChannelImpl.connect0(JZLjava/net/InetAddress;IJ)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init_ids() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await;
        assert_eq!(
            "sun/nio/ch/WindowsAsynchronousSocketChannelImpl.initIDs()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_read0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = read0(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Int(0),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/nio/ch/WindowsAsynchronousSocketChannelImpl.read0(JIJJ)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_shutdown0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = shutdown0(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)])).await;
        assert_eq!(
            "sun/nio/ch/WindowsAsynchronousSocketChannelImpl.shutdown0(JI)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_update_connect_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = update_connect_context(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/ch/WindowsAsynchronousSocketChannelImpl.updateConnectContext(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_write0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = write0(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Int(0),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/nio/ch/WindowsAsynchronousSocketChannelImpl.write0(JIJJ)I",
            result.unwrap_err().to_string()
        );
    }
}
