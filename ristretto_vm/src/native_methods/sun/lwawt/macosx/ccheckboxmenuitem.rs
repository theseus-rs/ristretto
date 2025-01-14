use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.lwawt.macosx.CCheckboxMenuItem`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/CCheckboxMenuItem";
    registry.register(
        class_name,
        "nativeSetIsCheckbox",
        "(J)V",
        native_set_is_checkbox,
    );
    registry.register(class_name, "nativeSetState", "(JZ)V", native_set_state);
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

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/lwawt/macosx/CCheckboxMenuItem";
        assert!(registry
            .method(class_name, "nativeSetIsCheckbox", "(J)V")
            .is_some());
        assert!(registry
            .method(class_name, "nativeSetState", "(JZ)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CCheckboxMenuItem.nativeSetIsCheckbox(J)V")]
    async fn test_native_set_is_checkbox() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_is_checkbox(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CCheckboxMenuItem.nativeSetState(JZ)V")]
    async fn test_native_set_state() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_state(thread, Arguments::default()).await;
    }
}
