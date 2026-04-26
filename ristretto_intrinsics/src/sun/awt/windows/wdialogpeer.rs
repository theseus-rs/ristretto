use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/awt/windows/WDialogPeer.createAwtDialog(Lsun/awt/windows/WComponentPeer;)V",
    Any
)]
#[async_method]
pub async fn create_awt_dialog<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _parent = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WDialogPeer.createAwtDialog(Lsun/awt/windows/WComponentPeer;)V"
            .to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WDialogPeer.endModal()V", Any)]
#[async_method]
pub async fn end_modal<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WDialogPeer.endModal()V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WDialogPeer.pSetIMMOption(Ljava/lang/String;)V", Any)]
#[async_method]
pub async fn p_set_immoption<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _option = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WDialogPeer.pSetIMMOption(Ljava/lang/String;)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WDialogPeer.showModal()V", Any)]
#[async_method]
pub async fn show_modal<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WDialogPeer.showModal()V".to_string())
            .into(),
    )
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_create_awt_dialog() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_awt_dialog(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WDialogPeer.createAwtDialog(Lsun/awt/windows/WComponentPeer;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_end_modal() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = end_modal(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WDialogPeer.endModal()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_p_set_immoption() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = p_set_immoption(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WDialogPeer.pSetIMMOption(Ljava/lang/String;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_show_modal() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = show_modal(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WDialogPeer.showModal()V",
            result.unwrap_err().to_string()
        );
    }
}
