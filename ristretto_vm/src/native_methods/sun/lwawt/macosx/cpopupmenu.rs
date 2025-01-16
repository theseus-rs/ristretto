use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/lwawt/macosx/CPopupMenu";

/// Register all native methods for `sun.lwawt.macosx.CPopupMenu`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "nativeCreatePopupMenu",
        "()J",
        native_create_popup_menu,
    );
    registry.register(
        CLASS_NAME,
        "nativeShowPopupMenu",
        "(JII)J",
        native_show_popup_menu,
    );
}

#[async_recursion(?Send)]
async fn native_create_popup_menu(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPopupMenu.nativeCreatePopupMenu()J")
}

#[async_recursion(?Send)]
async fn native_show_popup_menu(
    _thread: Arc<Thread>,
    _arguments: Arguments,
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
        let _ = native_create_popup_menu(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CPopupMenu.nativeShowPopupMenu(JII)J"
    )]
    async fn test_native_show_popup_menu() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_show_popup_menu(thread, Arguments::default()).await;
    }
}
