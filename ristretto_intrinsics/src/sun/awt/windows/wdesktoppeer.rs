use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/awt/windows/WDesktopPeer.ShellExecute(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn shell_execute<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _verb_j = parameters.pop_reference()?;
    let _file_or_uri_j = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/awt/windows/WDesktopPeer.ShellExecute(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;".to_string()).into())
}
#[intrinsic_method(
    "sun/awt/windows/WDesktopPeer.getDefaultBrowser()Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_default_browser<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WDesktopPeer.getDefaultBrowser()Ljava/lang/String;".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WDesktopPeer.init()V", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn init<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun/awt/windows/WDesktopPeer.init()V".to_string()).into())
}
#[intrinsic_method(
    "sun/awt/windows/WDesktopPeer.moveToTrash(Ljava/lang/String;)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn move_to_trash<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _jpath = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WDesktopPeer.moveToTrash(Ljava/lang/String;)Z".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WDesktopPeer.setSuddenTerminationEnabled(Z)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_sudden_termination_enabled<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _enabled = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WDesktopPeer.setSuddenTerminationEnabled(Z)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_shell_execute() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = shell_execute(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WDesktopPeer.ShellExecute(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_default_browser() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_default_browser(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WDesktopPeer.getDefaultBrowser()Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WDesktopPeer.init()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_move_to_trash() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = move_to_trash(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WDesktopPeer.moveToTrash(Ljava/lang/String;)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_sudden_termination_enabled() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            set_sudden_termination_enabled(thread, Parameters::new(vec![Value::from(false)])).await;
        assert_eq!(
            "sun/awt/windows/WDesktopPeer.setSuddenTerminationEnabled(Z)V",
            result.unwrap_err().to_string()
        );
    }
}
