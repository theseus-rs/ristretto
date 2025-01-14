use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.net.SocketInputStream`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/net/SocketInputStream";
    registry.register(class_name, "init", "()V", init);
    registry.register(
        class_name,
        "socketRead0",
        "(Ljava/io/FileDescriptor;[BIII)I",
        socket_read_0,
    );
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn socket_read_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.SocketInputStream.socketRead0(Ljava/io/FileDescriptor;[BIII)I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "java/net/SocketInputStream";
        assert!(registry.method(class_name, "init", "()V").is_some());
        assert!(registry
            .method(
                class_name,
                "socketRead0",
                "(Ljava/io/FileDescriptor;[BIII)I"
            )
            .is_some());
    }

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init(thread, Arguments::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.SocketInputStream.socketRead0(Ljava/io/FileDescriptor;[BIII)I"
    )]
    async fn test_socket_read_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = socket_read_0(thread, Arguments::default()).await;
    }
}
