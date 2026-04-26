use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/nio/ch/Iocp.close0(J)V", Any)]
#[async_method]
pub async fn close0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _handle = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun/nio/ch/Iocp.close0(J)V".to_string()).into())
}
#[intrinsic_method("sun/nio/ch/Iocp.createIoCompletionPort(JJII)J", Any)]
#[async_method]
pub async fn create_io_completion_port<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _concurrency = parameters.pop_int()?;
    let _completion_key = parameters.pop_int()?;
    let _existing_port = parameters.pop_long()?;
    let _handle = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/ch/Iocp.createIoCompletionPort(JJII)J".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/nio/ch/Iocp.getErrorMessage(I)Ljava/lang/String;", Any)]
#[async_method]
pub async fn get_error_message<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _error_code = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/ch/Iocp.getErrorMessage(I)Ljava/lang/String;".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/nio/ch/Iocp.getQueuedCompletionStatus(JLsun/nio/ch/Iocp$CompletionStatus;)V",
    Any
)]
#[async_method]
pub async fn get_queued_completion_status<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _obj = parameters.pop_reference()?;
    let _completion_port = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/ch/Iocp.getQueuedCompletionStatus(JLsun/nio/ch/Iocp$CompletionStatus;)V"
            .to_string(),
    )
    .into())
}
#[intrinsic_method("sun/nio/ch/Iocp.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun/nio/ch/Iocp.initIDs()V".to_string()).into())
}
#[intrinsic_method("sun/nio/ch/Iocp.postQueuedCompletionStatus(JI)V", Any)]
#[async_method]
pub async fn post_queued_completion_status<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _completion_key = parameters.pop_int()?;
    let _completion_port = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/ch/Iocp.postQueuedCompletionStatus(JI)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_close0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = close0(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/ch/Iocp.close0(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_create_io_completion_port() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_io_completion_port(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/nio/ch/Iocp.createIoCompletionPort(JJII)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_error_message() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_error_message(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/nio/ch/Iocp.getErrorMessage(I)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_queued_completion_status() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_queued_completion_status(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/nio/ch/Iocp.getQueuedCompletionStatus(JLsun/nio/ch/Iocp$CompletionStatus;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init_ids() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await;
        assert_eq!(
            "sun/nio/ch/Iocp.initIDs()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_post_queued_completion_status() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = post_queued_completion_status(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/ch/Iocp.postQueuedCompletionStatus(JI)V",
            result.unwrap_err().to_string()
        );
    }
}
