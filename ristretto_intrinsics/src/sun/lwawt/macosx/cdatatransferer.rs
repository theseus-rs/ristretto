use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/lwawt/macosx/CDataTransferer.formatForIndex(J)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn format_for_index<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _index = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CDataTransferer.formatForIndex(J)Ljava/lang/String;".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CDataTransferer.nativeDragQueryFile([B)[Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn native_drag_query_file<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _bytes = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CDataTransferer.nativeDragQueryFile([B)[Ljava/lang/String;".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CDataTransferer.registerFormatWithPasteboard(Ljava/lang/String;)J",
    Any
)]
#[async_method]
pub async fn register_format_with_pasteboard<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _newformat = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CDataTransferer.registerFormatWithPasteboard(Ljava/lang/String;)J"
            .to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_format_for_index() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = format_for_index(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.CDataTransferer.formatForIndex(J)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_drag_query_file() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            native_drag_query_file(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun.lwawt.macosx.CDataTransferer.nativeDragQueryFile([B)[Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_register_format_with_pasteboard() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            register_format_with_pasteboard(thread, Parameters::new(vec![Value::Object(None)]))
                .await;
        assert_eq!(
            "sun.lwawt.macosx.CDataTransferer.registerFormatWithPasteboard(Ljava/lang/String;)J",
            result.unwrap_err().to_string()
        );
    }
}
