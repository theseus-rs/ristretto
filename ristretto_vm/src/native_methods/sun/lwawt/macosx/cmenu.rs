use crate::native_methods::registry::{MethodRegistry, JAVA_11};
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/lwawt/macosx/CMenu";

/// Register all native methods for `sun.lwawt.macosx.CMenu`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_11 {
        registry.register(
            CLASS_NAME,
            "nativeAddSeparator",
            "(J)V",
            native_add_separator,
        );
    }

    registry.register(CLASS_NAME, "nativeCreateMenu", "(JZI)J", native_create_menu);
    registry.register(
        CLASS_NAME,
        "nativeCreateSubMenu",
        "(J)J",
        native_create_sub_menu,
    );
    registry.register(CLASS_NAME, "nativeDeleteItem", "(JI)V", native_delete_item);
    registry.register(CLASS_NAME, "nativeGetNSMenu", "(J)J", native_get_ns_menu);
    registry.register(
        CLASS_NAME,
        "nativeSetMenuTitle",
        "(JLjava/lang/String;)V",
        native_set_menu_title,
    );
}

#[async_recursion(?Send)]
async fn native_add_separator(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CMenu.nativeAddSeparator(J)V")
}

#[async_recursion(?Send)]
async fn native_create_menu(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CMenu.nativeCreateMenu(JZI)J")
}

#[async_recursion(?Send)]
async fn native_create_sub_menu(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CMenu.nativeCreateSubMenu(J)J")
}

#[async_recursion(?Send)]
async fn native_delete_item(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CMenu.nativeDeleteItem(JI)V")
}

#[async_recursion(?Send)]
async fn native_get_ns_menu(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CMenu.nativeGetNSMenu(J)J")
}

#[async_recursion(?Send)]
async fn native_set_menu_title(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CMenu.nativeSetMenuTitle(JLjava/lang/String;)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.CMenu.nativeAddSeparator(J)V")]
    async fn test_native_add_separator() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_add_separator(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.CMenu.nativeCreateMenu(JZI)J")]
    async fn test_native_create_menu() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_create_menu(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CMenu.nativeCreateSubMenu(J)J"
    )]
    async fn test_native_create_sub_menu() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_create_sub_menu(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.CMenu.nativeDeleteItem(JI)V")]
    async fn test_native_delete_item() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_delete_item(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.CMenu.nativeGetNSMenu(J)J")]
    async fn test_native_get_ns_menu() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_ns_menu(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CMenu.nativeSetMenuTitle(JLjava/lang/String;)V"
    )]
    async fn test_native_set_menu_title() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_menu_title(thread, Parameters::default()).await;
    }
}
