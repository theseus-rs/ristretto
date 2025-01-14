use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.awt.X11InputMethod`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/awt/X11InputMethod";
    registry.register(class_name, "disposeXIC", "()V", dispose_xic);
    registry.register(class_name, "initIDs", "()V", init_ids);
    registry.register(
        class_name,
        "isCompositionEnabledNative",
        "()Z",
        is_composition_enabled_native,
    );
    registry.register(class_name, "resetXIC", "()Ljava/lang/String;", reset_xic);
    registry.register(
        class_name,
        "setCompositionEnabledNative",
        "(Z)Z",
        set_composition_enabled_native,
    );
    registry.register(
        class_name,
        "turnoffStatusWindow",
        "()V",
        turnoff_status_window,
    );
}

#[async_recursion(?Send)]
async fn dispose_xic(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.X11InputMethod.disposeXIC()V")
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn is_composition_enabled_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11InputMethod.isCompositionEnabledNative()Z")
}

#[async_recursion(?Send)]
async fn reset_xic(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.X11InputMethod.resetXIC()Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn set_composition_enabled_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11InputMethod.setCompositionEnabledNative(Z)Z")
}

#[async_recursion(?Send)]
async fn turnoff_status_window(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11InputMethod.turnoffStatusWindow()V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/awt/X11InputMethod";
        assert!(registry.method(class_name, "disposeXIC", "()V").is_some());
        assert!(registry.method(class_name, "initIDs", "()V").is_some());
        assert!(registry
            .method(class_name, "isCompositionEnabledNative", "()Z")
            .is_some());
        assert!(registry
            .method(class_name, "resetXIC", "()Ljava/lang/String;")
            .is_some());
        assert!(registry
            .method(class_name, "setCompositionEnabledNative", "(Z)Z")
            .is_some());
        assert!(registry
            .method(class_name, "turnoffStatusWindow", "()V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.awt.X11InputMethod.disposeXIC()V")]
    async fn test_dispose_xic() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = dispose_xic(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.awt.X11InputMethod.isCompositionEnabledNative()Z")]
    async fn test_is_composition_enabled_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_composition_enabled_native(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.awt.X11InputMethod.resetXIC()Ljava/lang/String;")]
    async fn test_reset_xic() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = reset_xic(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.awt.X11InputMethod.setCompositionEnabledNative(Z)Z")]
    async fn test_set_composition_enabled_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_composition_enabled_native(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.awt.X11InputMethod.turnoffStatusWindow()V")]
    async fn test_turnoff_status_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = turnoff_status_window(thread, Arguments::default()).await;
    }
}
