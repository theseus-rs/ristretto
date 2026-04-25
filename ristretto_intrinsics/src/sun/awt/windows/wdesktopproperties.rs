use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/windows/WDesktopProperties.getWindowsParameters()V", Any)]
#[async_method]
pub async fn get_windows_parameters<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WDesktopProperties.getWindowsParameters()V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WDesktopProperties.init()V", Any)]
#[async_method]
pub async fn init<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WDesktopProperties.init()V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WDesktopProperties.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WDesktopProperties.initIDs()V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WDesktopProperties.playWindowsSound(Ljava/lang/String;)V",
    Any
)]
#[async_method]
pub async fn play_windows_sound<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _event = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WDesktopProperties.playWindowsSound(Ljava/lang/String;)V".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_windows_parameters() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_windows_parameters(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WDesktopProperties.getWindowsParameters()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WDesktopProperties.init()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init_ids() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WDesktopProperties.initIDs()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_play_windows_sound() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = play_windows_sound(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WDesktopProperties.playWindowsSound(Ljava/lang/String;)V",
            result.unwrap_err().to_string()
        );
    }
}
