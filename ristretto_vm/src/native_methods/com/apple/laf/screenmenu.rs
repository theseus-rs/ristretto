use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `com.apple.laf.ScreenMenu`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "com/apple/laf/ScreenMenu";
    registry.register(
        class_name,
        "addMenuListeners",
        "(Lcom/apple/laf/ScreenMenu;J)J",
        add_menu_listeners,
    );
    registry.register(
        class_name,
        "removeMenuListeners",
        "(J)V",
        remove_menu_listeners,
    );
}

#[async_recursion(?Send)]
async fn add_menu_listeners(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.apple.laf.ScreenMenu.addMenuListeners(Lcom/apple/laf/ScreenMenu;J)J")
}

#[async_recursion(?Send)]
async fn remove_menu_listeners(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.apple.laf.ScreenMenu.removeMenuListeners(J)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "com/apple/laf/ScreenMenu";
        assert!(registry
            .method(
                class_name,
                "addMenuListeners",
                "(Lcom/apple/laf/ScreenMenu;J)J"
            )
            .is_some());
        assert!(registry
            .method(class_name, "removeMenuListeners", "(J)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.laf.ScreenMenu.addMenuListeners(Lcom/apple/laf/ScreenMenu;J)J"
    )]
    async fn test_add_menu_listeners() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = add_menu_listeners(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.laf.ScreenMenu.removeMenuListeners(J)V"
    )]
    async fn test_remove_menu_listeners() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = remove_menu_listeners(thread, Arguments::default()).await;
    }
}
