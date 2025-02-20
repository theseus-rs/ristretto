use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/lwawt/macosx/CMenuBar";

/// Register all native methods for `sun.lwawt.macosx.CMenuBar`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "nativeCreateMenuBar",
        "()J",
        native_create_menu_bar,
    );
    registry.register(CLASS_NAME, "nativeDelMenu", "(JI)V", native_del_menu);
    registry.register(
        CLASS_NAME,
        "nativeSetHelpMenu",
        "(JJ)V",
        native_set_help_menu,
    );
}

#[async_recursion(?Send)]
async fn native_create_menu_bar(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CMenuBar.nativeCreateMenuBar()J")
}

#[async_recursion(?Send)]
async fn native_del_menu(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CMenuBar.nativeDelMenu(JI)V")
}

#[async_recursion(?Send)]
async fn native_set_help_menu(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CMenuBar.nativeSetHelpMenu(JJ)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CMenuBar.nativeCreateMenuBar()J"
    )]
    async fn test_native_create_menu_bar() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_create_menu_bar(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.CMenuBar.nativeDelMenu(JI)V")]
    async fn test_native_del_menu() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_del_menu(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CMenuBar.nativeSetHelpMenu(JJ)V"
    )]
    async fn test_native_set_help_menu() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_help_menu(thread, Parameters::default()).await;
    }
}
