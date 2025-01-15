use crate::arguments::Arguments;
use crate::native_methods::registry::{MethodRegistry, JAVA_17};
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "com/apple/eawt/_AppMenuBarHandler";

/// Register all native methods for `com.apple.eawt._AppMenuBarHandler`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() >= JAVA_17 {
        registry.register(
            CLASS_NAME,
            "nativeActivateDefaultMenuBar",
            "(J)V",
            native_activate_default_menu_bar,
        );
    }

    registry.register(
        CLASS_NAME,
        "nativeSetDefaultMenuBar",
        "(J)V",
        native_set_default_menu_bar,
    );
    registry.register(
        CLASS_NAME,
        "nativeSetMenuState",
        "(IZZ)V",
        native_set_menu_state,
    );
}

#[async_recursion(?Send)]
async fn native_activate_default_menu_bar(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.apple.eawt._AppMenuBarHandler.nativeActivateDefaultMenuBar(J)V")
}

#[async_recursion(?Send)]
async fn native_set_default_menu_bar(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.apple.eawt._AppMenuBarHandler.nativeSetDefaultMenuBar(J)V")
}

#[async_recursion(?Send)]
async fn native_set_menu_state(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.apple.eawt._AppMenuBarHandler.nativeSetMenuState(IZZ)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.eawt._AppMenuBarHandler.nativeSetDefaultMenuBar(J)V"
    )]
    async fn test_native_set_default_menu_bar() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_default_menu_bar(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.eawt._AppMenuBarHandler.nativeSetMenuState(IZZ)V"
    )]
    async fn test_native_set_menu_state() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_menu_state(thread, Arguments::default()).await;
    }
}
