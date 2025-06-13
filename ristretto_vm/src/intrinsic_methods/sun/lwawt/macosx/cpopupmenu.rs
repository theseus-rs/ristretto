use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("sun/lwawt/macosx/CPopupMenu.nativeCreatePopupMenu()J", Any)]
#[async_recursion(?Send)]
pub(crate) async fn native_create_popup_menu(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPopupMenu.nativeCreatePopupMenu()J")
}

#[intrinsic_method("sun/lwawt/macosx/CPopupMenu.nativeShowPopupMenu(JII)J", Any)]
#[async_recursion(?Send)]
pub(crate) async fn native_show_popup_menu(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPopupMenu.nativeShowPopupMenu(JII)J")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CPopupMenu.nativeCreatePopupMenu()J"
    )]
    async fn test_native_create_popup_menu() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_create_popup_menu(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CPopupMenu.nativeShowPopupMenu(JII)J"
    )]
    async fn test_native_show_popup_menu() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_show_popup_menu(thread, Parameters::default()).await;
    }
}
