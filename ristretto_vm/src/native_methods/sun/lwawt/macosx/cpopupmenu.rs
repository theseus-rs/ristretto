use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.lwawt.macosx.CPopupMenu`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/CPopupMenu";
    registry.register(
        class_name,
        "nativeCreatePopupMenu",
        "()J",
        native_create_popup_menu,
    );
    registry.register(
        class_name,
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

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/lwawt/macosx/CPopupMenu";
        assert!(registry
            .method(class_name, "nativeCreatePopupMenu", "()J")
            .is_some());
        assert!(registry
            .method(class_name, "nativeShowPopupMenu", "(JII)J")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CPopupMenu.nativeCreatePopupMenu()J")]
    async fn test_native_create_popup_menu() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_create_popup_menu(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CPopupMenu.nativeShowPopupMenu(JII)J")]
    async fn test_native_show_popup_menu() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_show_popup_menu(thread, Arguments::default()).await;
    }
}
