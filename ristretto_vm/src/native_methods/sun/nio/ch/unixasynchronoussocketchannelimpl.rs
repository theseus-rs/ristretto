use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/nio/ch/UnixAsynchronousSocketChannelImpl";

/// Register all native methods for `sun.nio.ch.UnixAsynchronousSocketChannelImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "checkConnect", "(I)V", check_connect);
}

#[async_recursion(?Send)]
async fn check_connect(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixAsynchronousSocketChannelImpl.checkConnect(I)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.UnixAsynchronousSocketChannelImpl.checkConnect(I)V"
    )]
    async fn test_check_connect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = check_connect(thread, Parameters::default()).await;
    }
}
