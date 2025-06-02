use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/net/SocketInputStream";

/// Register all intrinsic methods for `java.net.SocketInputStream`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "init", "()V", init);
    registry.register(
        CLASS_NAME,
        "socketRead0",
        "(Ljava/io/FileDescriptor;[BIII)I",
        socket_read_0,
    );
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn socket_read_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.net.SocketInputStream.socketRead0(Ljava/io/FileDescriptor;[BIII)I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.SocketInputStream.socketRead0(Ljava/io/FileDescriptor;[BIII)I"
    )]
    async fn test_socket_read_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = socket_read_0(thread, Parameters::default()).await;
    }
}
