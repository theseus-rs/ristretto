use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/awt/windows/WCheckboxPeer.create(Lsun/awt/windows/WComponentPeer;)V",
    Any
)]
#[async_method]
pub async fn create<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _parent = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WCheckboxPeer.create(Lsun/awt/windows/WComponentPeer;)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WCheckboxPeer.getCheckMarkSize()I", Any)]
#[async_method]
pub async fn get_check_mark_size<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WCheckboxPeer.getCheckMarkSize()I".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WCheckboxPeer.setCheckboxGroup(Ljava/awt/CheckboxGroup;)V",
    Any
)]
#[async_method]
pub async fn set_checkbox_group<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _group = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WCheckboxPeer.setCheckboxGroup(Ljava/awt/CheckboxGroup;)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WCheckboxPeer.setLabel(Ljava/lang/String;)V", Any)]
#[async_method]
pub async fn set_label<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _label = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WCheckboxPeer.setLabel(Ljava/lang/String;)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WCheckboxPeer.setState(Z)V", Any)]
#[async_method]
pub async fn set_state<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _state = parameters.pop_bool()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WCheckboxPeer.setState(Z)V".to_string())
            .into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_create() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WCheckboxPeer.create(Lsun/awt/windows/WComponentPeer;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_check_mark_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_check_mark_size(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WCheckboxPeer.getCheckMarkSize()I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_checkbox_group() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_checkbox_group(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WCheckboxPeer.setCheckboxGroup(Ljava/awt/CheckboxGroup;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_label() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_label(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WCheckboxPeer.setLabel(Ljava/lang/String;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_state() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_state(thread, Parameters::new(vec![Value::from(false)])).await;
        assert_eq!(
            "sun/awt/windows/WCheckboxPeer.setState(Z)V",
            result.unwrap_err().to_string()
        );
    }
}
