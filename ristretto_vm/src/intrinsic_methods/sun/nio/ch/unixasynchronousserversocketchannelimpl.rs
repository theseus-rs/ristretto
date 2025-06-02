use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/nio/ch/UnixAsynchronousServerSocketChannelImpl";

/// Register all intrinsic methods for `sun.nio.ch.UnixAsynchronousServerSocketChannelImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "accept0",
        "(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;[Ljava/net/InetSocketAddress;)I",
        accept_0,
    );
    registry.register(CLASS_NAME, "initIDs", "()V", init_ids);
}

#[async_recursion(?Send)]
async fn accept_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "sun.nio.ch.UnixAsynchronousServerSocketChannelImpl.accept0(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;[Ljava/net/InetSocketAddress;)I"
    )
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.UnixAsynchronousServerSocketChannelImpl.accept0(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;[Ljava/net/InetSocketAddress;)I"
    )]
    async fn test_accept_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = accept_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
