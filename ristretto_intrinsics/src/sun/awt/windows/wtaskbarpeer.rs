use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/awt/windows/WTaskbarPeer.flashWindow(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn flash_window<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _window = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WTaskbarPeer.flashWindow(J)V".to_string())
            .into(),
    )
}
#[intrinsic_method(
    "sun/awt/windows/WTaskbarPeer.nativeInit()Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn native_init<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WTaskbarPeer.nativeInit()Z".to_string())
            .into(),
    )
}
#[intrinsic_method(
    "sun/awt/windows/WTaskbarPeer.setOverlayIcon(J[III)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_overlay_icon<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _buf = parameters.pop_reference()?;
    let _window = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WTaskbarPeer.setOverlayIcon(J[III)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WTaskbarPeer.setProgressState(JLjava/awt/Taskbar$State;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_progress_state<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _state = parameters.pop_reference()?;
    let _window = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WTaskbarPeer.setProgressState(JLjava/awt/Taskbar$State;)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WTaskbarPeer.setProgressValue(JI)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_progress_value<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _value = parameters.pop_int()?;
    let _window = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WTaskbarPeer.setProgressValue(JI)V".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_flash_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = flash_window(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/windows/WTaskbarPeer.flashWindow(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_native_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_init(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WTaskbarPeer.nativeInit()Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_overlay_icon() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_overlay_icon(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WTaskbarPeer.setOverlayIcon(J[III)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_progress_state() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_progress_state(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WTaskbarPeer.setProgressState(JLjava/awt/Taskbar$State;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_progress_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            set_progress_value(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/windows/WTaskbarPeer.setProgressValue(JI)V",
            result.unwrap_err().to_string()
        );
    }
}
