use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/nio/ch/WindowsAsynchronousServerSocketChannelImpl.accept0(JJJJ)I",
    Any
)]
#[async_method]
pub async fn accept0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _buf = parameters.pop_long()?;
    let _ov = parameters.pop_long()?;
    let _accept_socket = parameters.pop_long()?;
    let _listen_socket = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/ch/WindowsAsynchronousServerSocketChannelImpl.accept0(JJJJ)I".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/nio/ch/WindowsAsynchronousServerSocketChannelImpl.closesocket0(J)V",
    Any
)]
#[async_method]
pub async fn closesocket0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _socket = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/ch/WindowsAsynchronousServerSocketChannelImpl.closesocket0(J)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/nio/ch/WindowsAsynchronousServerSocketChannelImpl.initIDs()V",
    Any
)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/ch/WindowsAsynchronousServerSocketChannelImpl.initIDs()V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/nio/ch/WindowsAsynchronousServerSocketChannelImpl.updateAcceptContext(JJ)V",
    Any
)]
#[async_method]
pub async fn update_accept_context<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _accept_socket = parameters.pop_long()?;
    let _listen_socket = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/ch/WindowsAsynchronousServerSocketChannelImpl.updateAcceptContext(JJ)V"
            .to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_accept0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = accept0(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/nio/ch/WindowsAsynchronousServerSocketChannelImpl.accept0(JJJJ)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_closesocket0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = closesocket0(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/ch/WindowsAsynchronousServerSocketChannelImpl.closesocket0(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init_ids() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await;
        assert_eq!(
            "sun/nio/ch/WindowsAsynchronousServerSocketChannelImpl.initIDs()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_update_accept_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = update_accept_context(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/ch/WindowsAsynchronousServerSocketChannelImpl.updateAcceptContext(JJ)V",
            result.unwrap_err().to_string()
        );
    }
}
