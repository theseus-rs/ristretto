use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/awt/X11InputMethod";

/// Register all native methods for `sun.awt.X11InputMethod`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "disposeXIC", "()V", dispose_xic);
    registry.register(CLASS_NAME, "initIDs", "()V", init_ids);
    registry.register(
        CLASS_NAME,
        "isCompositionEnabledNative",
        "()Z",
        is_composition_enabled_native,
    );
    registry.register(CLASS_NAME, "resetXIC", "()Ljava/lang/String;", reset_xic);
    registry.register(
        CLASS_NAME,
        "setCompositionEnabledNative",
        "(Z)Z",
        set_composition_enabled_native,
    );
    registry.register(
        CLASS_NAME,
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

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.awt.X11InputMethod.disposeXIC()V")]
    async fn test_dispose_xic() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = dispose_xic(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Arguments::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.X11InputMethod.isCompositionEnabledNative()Z"
    )]
    async fn test_is_composition_enabled_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_composition_enabled_native(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.X11InputMethod.resetXIC()Ljava/lang/String;"
    )]
    async fn test_reset_xic() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = reset_xic(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.X11InputMethod.setCompositionEnabledNative(Z)Z"
    )]
    async fn test_set_composition_enabled_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_composition_enabled_native(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.awt.X11InputMethod.turnoffStatusWindow()V")]
    async fn test_turnoff_status_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = turnoff_status_window(thread, Arguments::default()).await;
    }
}
