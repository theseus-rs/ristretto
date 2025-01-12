use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.io.ObjectInputStream`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/io/ObjectInputStream";
    registry.register(class_name, "bytesToDoubles", "([BI[DII)V", bytes_to_doubles);
    registry.register(class_name, "bytesToFloats", "([BI[FII)V", bytes_to_floats);
}

#[async_recursion(?Send)]
async fn bytes_to_doubles(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.ObjectInputStream.bytesToDoubles([BI[DII)V")
}

#[async_recursion(?Send)]
async fn bytes_to_floats(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.ObjectInputStream.bytesToFloats([BI[FII)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "java/io/ObjectInputStream";
        assert!(registry
            .method(class_name, "bytesToDoubles", "([BI[DII)V")
            .is_some());
        assert!(registry
            .method(class_name, "bytesToFloats", "([BI[FII)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.ObjectInputStream.bytesToDoubles([BI[DII)V"
    )]
    async fn test_bytes_to_doubles() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = bytes_to_doubles(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.ObjectInputStream.bytesToFloats([BI[FII)V"
    )]
    async fn test_bytes_to_floats() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = bytes_to_floats(thread, Arguments::default()).await;
    }
}
