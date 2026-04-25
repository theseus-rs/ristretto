use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/windows/WFileDialogPeer._dispose()V", Any)]
#[async_method]
pub async fn dispose<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WFileDialogPeer._dispose()V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WFileDialogPeer._hide()V", Any)]
#[async_method]
pub async fn hide<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WFileDialogPeer._hide()V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WFileDialogPeer._show()V", Any)]
#[async_method]
pub async fn show<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WFileDialogPeer._show()V".to_string())
            .into(),
    )
}
#[intrinsic_method(
    "sun/awt/windows/WFileDialogPeer.getLocationOnScreen()Ljava/awt/Point;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_location_on_screen<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WFileDialogPeer.getLocationOnScreen()Ljava/awt/Point;".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WFileDialogPeer.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WFileDialogPeer.initIDs()V".to_string())
            .into(),
    )
}
#[intrinsic_method(
    "sun/awt/windows/WFileDialogPeer.setFilterString(Ljava/lang/String;)V",
    Any
)]
#[async_method]
pub async fn set_filter_string<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _filter_description = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WFileDialogPeer.setFilterString(Ljava/lang/String;)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WFileDialogPeer.toBack()V", Any)]
#[async_method]
pub async fn to_back<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WFileDialogPeer.toBack()V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WFileDialogPeer.toFront()V", Any)]
#[async_method]
pub async fn to_front<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WFileDialogPeer.toFront()V".to_string())
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
            "sun/awt/windows/WFileDialogPeer._dispose()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_hide() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = hide(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WFileDialogPeer._hide()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_show() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = show(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WFileDialogPeer._show()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_location_on_screen() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_location_on_screen(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WFileDialogPeer.getLocationOnScreen()Ljava/awt/Point;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init_ids() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WFileDialogPeer.initIDs()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_filter_string() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_filter_string(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WFileDialogPeer.setFilterString(Ljava/lang/String;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_to_back() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = to_back(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WFileDialogPeer.toBack()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_to_front() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = to_front(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WFileDialogPeer.toFront()V",
            result.unwrap_err().to_string()
        );
    }
}
