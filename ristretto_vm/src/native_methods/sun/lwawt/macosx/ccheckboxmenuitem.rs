use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/lwawt/macosx/CCheckboxMenuItem";

/// Register all native methods for `sun.lwawt.macosx.CCheckboxMenuItem`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "nativeSetIsCheckbox",
        "(J)V",
        native_set_is_checkbox,
    );
    registry.register(CLASS_NAME, "nativeSetState", "(JZ)V", native_set_state);
}

#[async_recursion(?Send)]
async fn native_set_is_checkbox(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CCheckboxMenuItem.nativeSetIsCheckbox(J)V")
}

#[async_recursion(?Send)]
async fn native_set_state(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CCheckboxMenuItem.nativeSetState(JZ)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CCheckboxMenuItem.nativeSetIsCheckbox(J)V"
    )]
    async fn test_native_set_is_checkbox() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_is_checkbox(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CCheckboxMenuItem.nativeSetState(JZ)V"
    )]
    async fn test_native_set_state() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_state(thread, Arguments::default()).await;
    }
}
