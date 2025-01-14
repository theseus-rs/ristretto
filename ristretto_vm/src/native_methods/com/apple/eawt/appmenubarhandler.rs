use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_17: Version = Version::Java17 { minor: 0 };

/// Register all native methods for `com.apple.eawt._AppMenuBarHandler`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "com/apple/eawt/_AppMenuBarHandler";
    let java_version = registry.java_version();

    if java_version >= &JAVA_17 {
        registry.register(
            class_name,
            "nativeActivateDefaultMenuBar",
            "(J)V",
            native_activate_default_menu_bar,
        );
    }

    registry.register(
        class_name,
        "nativeSetDefaultMenuBar",
        "(J)V",
        native_set_default_menu_bar,
    );
    registry.register(
        class_name,
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

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "com/apple/eawt/_AppMenuBarHandler";
        assert!(registry
            .method(class_name, "nativeSetDefaultMenuBar", "(J)V")
            .is_some());
        assert!(registry
            .method(class_name, "nativeSetMenuState", "(IZZ)V")
            .is_some());
    }

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
