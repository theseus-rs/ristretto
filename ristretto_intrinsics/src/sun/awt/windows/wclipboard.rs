use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual, LessThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_17};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/awt/windows/WClipboard.closeClipboard()V",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn close_clipboard<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WClipboard.closeClipboard()V".to_string())
            .into(),
    )
}
#[intrinsic_method(
    "sun/awt/windows/WClipboard.closeClipboard0()V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn close_clipboard0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WClipboard.closeClipboard0()V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WClipboard.getClipboardData(J)[B", Any)]
#[async_method]
pub async fn get_clipboard_data<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _format = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WClipboard.getClipboardData(J)[B".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WClipboard.getClipboardFormats()[J", Any)]
#[async_method]
pub async fn get_clipboard_formats<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WClipboard.getClipboardFormats()[J".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WClipboard.init()V", Any)]
#[async_method]
pub async fn init<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun/awt/windows/WClipboard.init()V".to_string()).into())
}
#[intrinsic_method(
    "sun/awt/windows/WClipboard.openClipboard(Lsun/awt/datatransfer/SunClipboard;)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn open_clipboard<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _new_owner = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WClipboard.openClipboard(Lsun/awt/datatransfer/SunClipboard;)V"
            .to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WClipboard.openClipboard0(Lsun/awt/datatransfer/SunClipboard;)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn open_clipboard0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _new_owner = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WClipboard.openClipboard0(Lsun/awt/datatransfer/SunClipboard;)V"
            .to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WClipboard.publishClipboardData(J[B)V", Any)]
#[async_method]
pub async fn publish_clipboard_data<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _bytes = parameters.pop_reference()?;
    let _format = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WClipboard.publishClipboardData(J[B)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WClipboard.registerClipboard()V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn register_clipboard<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WClipboard.registerClipboard()V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WClipboard.registerClipboardViewer()V", Any)]
#[async_method]
pub async fn register_clipboard_viewer<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WClipboard.registerClipboardViewer()V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_close_clipboard() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = close_clipboard(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WClipboard.closeClipboard()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_close_clipboard0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = close_clipboard0(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WClipboard.closeClipboard0()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_clipboard_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_clipboard_data(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/windows/WClipboard.getClipboardData(J)[B",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_clipboard_formats() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_clipboard_formats(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WClipboard.getClipboardFormats()[J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WClipboard.init()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_open_clipboard() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = open_clipboard(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WClipboard.openClipboard(Lsun/awt/datatransfer/SunClipboard;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_open_clipboard0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = open_clipboard0(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WClipboard.openClipboard0(Lsun/awt/datatransfer/SunClipboard;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_publish_clipboard_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = publish_clipboard_data(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WClipboard.publishClipboardData(J[B)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_register_clipboard() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = register_clipboard(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WClipboard.registerClipboard()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_register_clipboard_viewer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = register_clipboard_viewer(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WClipboard.registerClipboardViewer()V",
            result.unwrap_err().to_string()
        );
    }
}
