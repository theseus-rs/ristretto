use ristretto_classfile::JAVA_25;
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/lwawt/macosx/CClipboard.checkPasteboardWithoutNotification()Z",
    Any
)]
#[async_method]
pub async fn check_pasteboard_without_notification<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CClipboard.checkPasteboardWithoutNotification()Z".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CClipboard.declareTypes([JLsun/awt/datatransfer/SunClipboard;)V",
    Any
)]
#[async_method]
pub async fn declare_types<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _in_java_clip = parameters.pop_reference()?;
    let _in_types = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CClipboard.declareTypes([JLsun/awt/datatransfer/SunClipboard;)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CClipboard.getClipboardData(J)[B", Any)]
#[async_method]
pub async fn get_clipboard_data<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _format = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CClipboard.getClipboardData(J)[B".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CClipboard.getClipboardFormats()[J", Any)]
#[async_method]
pub async fn get_clipboard_formats<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CClipboard.getClipboardFormats()[J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CClipboard.setData([BJ)V", Any)]
#[async_method]
pub async fn set_data<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _format = parameters.pop_long()?;
    let _data = parameters.pop_reference()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.CClipboard.setData([BJ)V".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "sun/lwawt/macosx/CClipboard.writeFileObjects([B)V",
    GreaterThanOrEqual(JAVA_25)
)]
#[async_method]
pub async fn write_file_objects<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _data = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CClipboard.writeFileObjects([B)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_check_pasteboard_without_notification() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = check_pasteboard_without_notification(thread, Parameters::default()).await;
        assert_eq!(
            "sun.lwawt.macosx.CClipboard.checkPasteboardWithoutNotification()Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_declare_types() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = declare_types(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CClipboard.declareTypes([JLsun/awt/datatransfer/SunClipboard;)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_clipboard_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_clipboard_data(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.CClipboard.getClipboardData(J)[B",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_clipboard_formats() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_clipboard_formats(thread, Parameters::default()).await;
        assert_eq!(
            "sun.lwawt.macosx.CClipboard.getClipboardFormats()[J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_set_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_data(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CClipboard.setData([BJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_write_file_objects() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = write_file_objects(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun.lwawt.macosx.CClipboard.writeFileObjects([B)V",
            result.unwrap_err().to_string()
        );
    }
}
