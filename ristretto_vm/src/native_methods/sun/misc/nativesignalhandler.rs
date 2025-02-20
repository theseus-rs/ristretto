use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/misc/NativeSignalHandler";

/// Register all native methods for `sun.misc.NativeSignalHandler`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "handle0", "(IJ)V", handle_0);
}

#[async_recursion(?Send)]
async fn handle_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.misc.NativeSignalHandler.handle0(IJ)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.misc.NativeSignalHandler.handle0(IJ)V")]
    async fn test_handle_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = handle_0(thread, Parameters::default()).await;
    }
}
