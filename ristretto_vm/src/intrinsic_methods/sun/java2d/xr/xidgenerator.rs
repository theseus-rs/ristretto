use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/java2d/xr/XIDGenerator";

/// Register all intrinsic methods for `sun.java2d.xr.XIDGenerator`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "bufferXIDs", "([II)V", buffer_x_ids);
}

#[async_recursion(?Send)]
async fn buffer_x_ids(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XIDGenerator.bufferXIDs([II)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.java2d.xr.XIDGenerator.bufferXIDs([II)V")]
    async fn test_buffer_x_ids() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = buffer_x_ids(thread, Parameters::default()).await;
    }
}
