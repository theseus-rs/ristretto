use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "com/apple/laf/ScreenMenu";

/// Register all intrinsic methods for `com.apple.laf.ScreenMenu`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "addMenuListeners",
        "(Lcom/apple/laf/ScreenMenu;J)J",
        add_menu_listeners,
    );
    registry.register(
        CLASS_NAME,
        "removeMenuListeners",
        "(J)V",
        remove_menu_listeners,
    );
}

#[async_recursion(?Send)]
async fn add_menu_listeners(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.laf.ScreenMenu.addMenuListeners(Lcom/apple/laf/ScreenMenu;J)J")
}

#[async_recursion(?Send)]
async fn remove_menu_listeners(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.laf.ScreenMenu.removeMenuListeners(J)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.laf.ScreenMenu.addMenuListeners(Lcom/apple/laf/ScreenMenu;J)J"
    )]
    async fn test_add_menu_listeners() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = add_menu_listeners(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.laf.ScreenMenu.removeMenuListeners(J)V"
    )]
    async fn test_remove_menu_listeners() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = remove_menu_listeners(thread, Parameters::default()).await;
    }
}
