use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::{Any, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/windows/WTextComponentPeer.enableEditing(Z)V", Any)]
#[async_method]
pub async fn enable_editing<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _on = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WTextComponentPeer.enableEditing(Z)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WTextComponentPeer.getSelectionEnd()I", Any)]
#[async_method]
pub async fn get_selection_end<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WTextComponentPeer.getSelectionEnd()I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WTextComponentPeer.getSelectionStart()I", Any)]
#[async_method]
pub async fn get_selection_start<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WTextComponentPeer.getSelectionStart()I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WTextComponentPeer.getText()Ljava/lang/String;", Any)]
#[async_method]
pub async fn get_text<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WTextComponentPeer.getText()Ljava/lang/String;".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WTextComponentPeer.initIDs()V",
    LessThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WTextComponentPeer.initIDs()V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WTextComponentPeer.select(II)V", Any)]
#[async_method]
pub async fn select<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _end = parameters.pop_int()?;
    let _start = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WTextComponentPeer.select(II)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WTextComponentPeer.setText(Ljava/lang/String;)V", Any)]
#[async_method]
pub async fn set_text<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _text = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WTextComponentPeer.setText(Ljava/lang/String;)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_enable_editing() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = enable_editing(thread, Parameters::new(vec![Value::from(false)])).await;
        assert_eq!(
            "sun/awt/windows/WTextComponentPeer.enableEditing(Z)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_selection_end() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_selection_end(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WTextComponentPeer.getSelectionEnd()I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_selection_start() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_selection_start(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WTextComponentPeer.getSelectionStart()I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_text() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_text(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WTextComponentPeer.getText()Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init_ids() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WTextComponentPeer.initIDs()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_select() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = select(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/windows/WTextComponentPeer.select(II)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_text() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_text(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WTextComponentPeer.setText(Ljava/lang/String;)V",
            result.unwrap_err().to_string()
        );
    }
}
