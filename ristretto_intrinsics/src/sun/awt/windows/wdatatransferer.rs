use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/awt/windows/WDataTransferer.dragQueryFile([B)[Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn drag_query_file<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _bytes = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WDataTransferer.dragQueryFile([B)[Ljava/lang/String;".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WDataTransferer.getClipboardFormatName(J)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_clipboard_format_name<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _format = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WDataTransferer.getClipboardFormatName(J)Ljava/lang/String;".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WDataTransferer.imageDataToPlatformImageBytes([BIIJ)[B",
    Any
)]
#[async_method]
pub async fn image_data_to_platform_image_bytes<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _format = parameters.pop_long()?;
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _image_data = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WDataTransferer.imageDataToPlatformImageBytes([BIIJ)[B".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WDataTransferer.platformImageBytesToImageData([BJ)[I",
    Any
)]
#[async_method]
pub async fn platform_image_bytes_to_image_data<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _format = parameters.pop_long()?;
    let _bytes = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WDataTransferer.platformImageBytesToImageData([BJ)[I".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WDataTransferer.registerClipboardFormat(Ljava/lang/String;)J",
    Any
)]
#[async_method]
pub async fn register_clipboard_format<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _str = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WDataTransferer.registerClipboardFormat(Ljava/lang/String;)J".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_drag_query_file() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = drag_query_file(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WDataTransferer.dragQueryFile([B)[Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_clipboard_format_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_clipboard_format_name(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/windows/WDataTransferer.getClipboardFormatName(J)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_image_data_to_platform_image_bytes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = image_data_to_platform_image_bytes(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WDataTransferer.imageDataToPlatformImageBytes([BIIJ)[B",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_platform_image_bytes_to_image_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = platform_image_bytes_to_image_data(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WDataTransferer.platformImageBytesToImageData([BJ)[I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_register_clipboard_format() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            register_clipboard_format(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WDataTransferer.registerClipboardFormat(Ljava/lang/String;)J",
            result.unwrap_err().to_string()
        );
    }
}
