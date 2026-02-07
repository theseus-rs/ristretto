use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/lwawt/macosx/CDataTransferer.formatForIndex(J)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn format_for_index<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CDataTransferer.formatForIndex(J)Ljava/lang/String;")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CDataTransferer.nativeDragQueryFile([B)[Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn native_drag_query_file<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CDataTransferer.nativeDragQueryFile([B)[Ljava/lang/String;")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CDataTransferer.registerFormatWithPasteboard(Ljava/lang/String;)J",
    Any
)]
#[async_method]
pub async fn register_format_with_pasteboard<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CDataTransferer.registerFormatWithPasteboard(Ljava/lang/String;)J")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CDataTransferer.formatForIndex(J)Ljava/lang/String;"
    )]
    async fn test_format_for_index() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = format_for_index(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CDataTransferer.nativeDragQueryFile([B)[Ljava/lang/String;"
    )]
    async fn test_native_drag_query_file() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_drag_query_file(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CDataTransferer.registerFormatWithPasteboard(Ljava/lang/String;)J"
    )]
    async fn test_register_format_with_pasteboard() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = register_format_with_pasteboard(thread, Parameters::default()).await;
    }
}
