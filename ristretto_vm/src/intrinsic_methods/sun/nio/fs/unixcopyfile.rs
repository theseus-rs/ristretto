use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/nio/fs/UnixCopyFile";

/// Register all intrinsic methods for `sun.nio.fs.UnixCopyFile`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "transfer", "(IIJ)V", transfer);
}

#[async_recursion(?Send)]
async fn transfer(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixCopyFile.transfer(IIJ)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixCopyFile.transfer(IIJ)V")]
    async fn test_transfer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = transfer(thread, Parameters::default()).await;
    }
}
