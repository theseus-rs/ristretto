use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.lwawt.macosx.CMenuBar`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/CMenuBar";
    registry.register(
        class_name,
        "nativeCreateMenuBar",
        "()J",
        native_create_menu_bar,
    );
    registry.register(class_name, "nativeDelMenu", "(JI)V", native_del_menu);
    registry.register(
        class_name,
        "nativeSetHelpMenu",
        "(JJ)V",
        native_set_help_menu,
    );
}

#[async_recursion(?Send)]
async fn native_create_menu_bar(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CMenuBar.nativeCreateMenuBar()J")
}

#[async_recursion(?Send)]
async fn native_del_menu(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CMenuBar.nativeDelMenu(JI)V")
}

#[async_recursion(?Send)]
async fn native_set_help_menu(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CMenuBar.nativeSetHelpMenu(JJ)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/lwawt/macosx/CMenuBar";
        assert!(registry
            .method(class_name, "nativeCreateMenuBar", "()J")
            .is_some());
        assert!(registry
            .method(class_name, "nativeDelMenu", "(JI)V")
            .is_some());
        assert!(registry
            .method(class_name, "nativeSetHelpMenu", "(JJ)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CMenuBar.nativeCreateMenuBar()J")]
    async fn test_native_create_menu_bar() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_create_menu_bar(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CMenuBar.nativeDelMenu(JI)V")]
    async fn test_native_del_menu() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_del_menu(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CMenuBar.nativeSetHelpMenu(JJ)V")]
    async fn test_native_set_help_menu() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_help_menu(thread, Arguments::default()).await;
    }
}
