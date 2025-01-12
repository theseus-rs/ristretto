use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.io.ObjectOutputStream`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/io/ObjectOutputStream";
    registry.register(class_name, "doublesToBytes", "([DI[BII)V", doubles_to_bytes);
    registry.register(class_name, "floatsToBytes", "([FI[BII)V", floats_to_bytes);
}

#[async_recursion(?Send)]
async fn doubles_to_bytes(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.ObjectOutputStream.doublesToBytes([DI[BII)V")
}

#[async_recursion(?Send)]
async fn floats_to_bytes(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.ObjectOutputStream.floatsToBytes([FI[BII)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "java/io/ObjectOutputStream";
        assert!(registry
            .method(class_name, "doublesToBytes", "([DI[BII)V")
            .is_some());
        assert!(registry
            .method(class_name, "floatsToBytes", "([FI[BII)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.ObjectOutputStream.doublesToBytes([DI[BII)V"
    )]
    async fn test_doubles_to_bytes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = doubles_to_bytes(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.ObjectOutputStream.floatsToBytes([FI[BII)V"
    )]
    async fn test_floats_to_bytes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = floats_to_bytes(thread, Arguments::default()).await;
    }
}
