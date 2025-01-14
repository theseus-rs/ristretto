use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.net.SocketOutputStream`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/net/SocketOutputStream";
    registry.register(class_name, "init", "()V", init);
    registry.register(
        class_name,
        "socketWrite0",
        "(Ljava/io/FileDescriptor;[BII)V",
        socket_write_0,
    );
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn socket_write_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.SocketOutputStream.socketWrite0(Ljava/io/FileDescriptor;[BII)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "java/net/SocketOutputStream";
        assert!(registry.method(class_name, "init", "()V").is_some());
        assert!(registry
            .method(
                class_name,
                "socketWrite0",
                "(Ljava/io/FileDescriptor;[BII)V"
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
        expected = "not yet implemented: java.net.SocketOutputStream.socketWrite0(Ljava/io/FileDescriptor;[BII)V"
    )]
    async fn test_socket_write_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = socket_write_0(thread, Arguments::default()).await;
    }
}
