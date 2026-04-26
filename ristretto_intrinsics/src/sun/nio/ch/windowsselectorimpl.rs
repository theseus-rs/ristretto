use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::{Any, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/nio/ch/WindowsSelectorImpl.discardUrgentData(I)Z",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn discard_urgent_data<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _s = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/ch/WindowsSelectorImpl.discardUrgentData(I)Z".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/nio/ch/WindowsSelectorImpl.resetWakeupSocket0(I)V", Any)]
#[async_method]
pub async fn reset_wakeup_socket0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _scin_fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/ch/WindowsSelectorImpl.resetWakeupSocket0(I)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/nio/ch/WindowsSelectorImpl.setWakeupSocket0(I)V", Any)]
#[async_method]
pub async fn set_wakeup_socket0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _scout_fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/ch/WindowsSelectorImpl.setWakeupSocket0(I)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_discard_urgent_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = discard_urgent_data(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/nio/ch/WindowsSelectorImpl.discardUrgentData(I)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_reset_wakeup_socket0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = reset_wakeup_socket0(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/nio/ch/WindowsSelectorImpl.resetWakeupSocket0(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_wakeup_socket0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_wakeup_socket0(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/nio/ch/WindowsSelectorImpl.setWakeupSocket0(I)V",
            result.unwrap_err().to_string()
        );
    }
}
