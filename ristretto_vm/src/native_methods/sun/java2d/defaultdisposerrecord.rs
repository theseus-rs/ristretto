use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/java2d/DefaultDisposerRecord";

/// Register all native methods for `sun.java2d.DefaultDisposerRecord`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "invokeNativeDispose",
        "(JJ)V",
        invoke_native_dispose,
    );
}

#[async_recursion(?Send)]
async fn invoke_native_dispose(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.java2d.DefaultDisposerRecord.invokeNativeDispose(JJ)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.DefaultDisposerRecord.invokeNativeDispose(JJ)V"
    )]
    async fn test_invoke_native_dispose() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_native_dispose(thread, Arguments::default()).await;
    }
}
