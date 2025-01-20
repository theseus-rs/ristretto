use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/nio/fs/BsdFileSystem";

/// Register all native methods for `sun.nio.fs.BsdFileSystem`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "directCopy0", "(IIJ)I", direct_copy_0);
}

#[async_recursion(?Send)]
async fn direct_copy_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.BsdFileSystem.directCopy0(IJJ)I");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.BsdFileSystem.directCopy0(IJJ)I")]
    async fn test_direct_copy_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = direct_copy_0(thread, Parameters::default()).await;
    }
}
