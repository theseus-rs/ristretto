use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/nio/fs/UnixFileSystem";

/// Register all intrinsic methods for `sun.nio.fs.UnixFileSystem`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "bufferedCopy0", "(IIJIJ)V", buffered_copy_0);
}

#[async_recursion(?Send)]
async fn buffered_copy_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixFileSystem.bufferedCopy0(IIJIJ)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.fs.UnixFileSystem.bufferedCopy0(IIJIJ)V"
    )]
    async fn test_buffered_copy_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = buffered_copy_0(thread, Parameters::default()).await;
    }
}
