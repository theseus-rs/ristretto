use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_24;
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/lwawt/macosx/CClipboard.checkPasteboardWithoutNotification()Z",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn check_pasteboard_without_notification(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CClipboard.checkPasteboardWithoutNotification()Z")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CClipboard.declareTypes([JLsun/awt/datatransfer/SunClipboard;)V",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn declare_types(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CClipboard.declareTypes([JLsun/awt/datatransfer/SunClipboard;)V")
}

#[intrinsic_method("sun/lwawt/macosx/CClipboard.getClipboardData(J)[B", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_clipboard_data(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CClipboard.getClipboardData(J)[B")
}

#[intrinsic_method("sun/lwawt/macosx/CClipboard.getClipboardFormats()[J", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_clipboard_formats(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CClipboard.getClipboardFormats()[J")
}

#[intrinsic_method("sun/lwawt/macosx/CClipboard.setData([BJ)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn set_data(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CClipboard.setData([BJ)V")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CClipboard.writeFileObjects([B)V",
    GreaterThanOrEqual(JAVA_24)
)]
#[async_recursion(?Send)]
pub(crate) async fn write_file_objects(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CClipboard.writeFileObjects([B)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CClipboard.checkPasteboardWithoutNotification()Z"
    )]
    async fn test_check_pasteboard_without_notification() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = check_pasteboard_without_notification(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CClipboard.declareTypes([JLsun/awt/datatransfer/SunClipboard;)V"
    )]
    async fn test_declare_types() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = declare_types(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CClipboard.getClipboardData(J)[B"
    )]
    async fn test_get_clipboard_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_clipboard_data(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CClipboard.getClipboardFormats()[J"
    )]
    async fn test_get_clipboard_formats() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_clipboard_formats(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.CClipboard.setData([BJ)V")]
    async fn test_set_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_data(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CClipboard.writeFileObjects([B)V"
    )]
    async fn test_write_file_objects() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = write_file_objects(thread, Parameters::default()).await;
    }
}
