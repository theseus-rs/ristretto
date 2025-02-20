use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/lwawt/macosx/CInputMethodDescriptor";

/// Register all native methods for `sun.lwawt.macosx.CInputMethodDescriptor`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "nativeGetAvailableLocales",
        "()Ljava/util/List;",
        native_get_available_locales,
    );
    registry.register(CLASS_NAME, "nativeInit", "()V", native_init);
}

#[async_recursion(?Send)]
async fn native_get_available_locales(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CInputMethodDescriptor.nativeGetAvailableLocales()Ljava/util/List;")
}

#[async_recursion(?Send)]
async fn native_init(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CInputMethodDescriptor.nativeInit()V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CInputMethodDescriptor.nativeGetAvailableLocales()Ljava/util/List;"
    )]
    async fn test_native_get_available_locales() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_available_locales(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CInputMethodDescriptor.nativeInit()V"
    )]
    async fn test_native_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_init(thread, Parameters::default()).await;
    }
}
