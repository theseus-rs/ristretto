use ristretto_classfile::VersionSpecification::{Any, Equal, LessThanOrEqual};
use ristretto_classfile::{JAVA_21, JAVA_25};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/windows/WMenuItemPeer._dispose()V", Any)]
#[async_method]
pub async fn dispose<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WMenuItemPeer._dispose()V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WMenuItemPeer._setFont(Ljava/awt/Font;)V", Any)]
#[async_method]
pub async fn set_font<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WMenuItemPeer._setFont(Ljava/awt/Font;)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WMenuItemPeer._setLabel()V", Equal(JAVA_25))]
#[async_method]
pub async fn set_label<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WMenuItemPeer._setLabel()V".to_string())
            .into(),
    )
}
#[intrinsic_method(
    "sun/awt/windows/WMenuItemPeer._setLabel(Ljava/lang/String;)V",
    LessThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn set_label_windows_le_v21<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WMenuItemPeer._setLabel(Ljava/lang/String;)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WMenuItemPeer.create(Lsun/awt/windows/WMenuPeer;)V",
    Any
)]
#[async_method]
pub async fn create<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _menu = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WMenuItemPeer.create(Lsun/awt/windows/WMenuPeer;)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WMenuItemPeer.enable(Z)V", Any)]
#[async_method]
pub async fn enable<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _on = parameters.pop_bool()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WMenuItemPeer.enable(Z)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WMenuItemPeer.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WMenuItemPeer.initIDs()V".to_string())
            .into(),
    )
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_dispose() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = dispose(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WMenuItemPeer._dispose()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_font() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_font(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WMenuItemPeer._setFont(Ljava/awt/Font;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_label() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_label(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WMenuItemPeer._setLabel()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_label_windows_le_v21() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            set_label_windows_le_v21(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WMenuItemPeer._setLabel(Ljava/lang/String;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_create() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WMenuItemPeer.create(Lsun/awt/windows/WMenuPeer;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_enable() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = enable(thread, Parameters::new(vec![Value::from(false)])).await;
        assert_eq!(
            "sun/awt/windows/WMenuItemPeer.enable(Z)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init_ids() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WMenuItemPeer.initIDs()V",
            result.unwrap_err().to_string()
        );
    }
}
