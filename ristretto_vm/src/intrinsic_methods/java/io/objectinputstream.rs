use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/io/ObjectInputStream";

/// Register all intrinsic methods for `java.io.ObjectInputStream`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= 11 {
        registry.register(CLASS_NAME, "bytesToDoubles", "([BI[DII)V", bytes_to_doubles);
        registry.register(CLASS_NAME, "bytesToFloats", "([BI[FII)V", bytes_to_floats);
    }
}

#[async_recursion(?Send)]
async fn bytes_to_doubles(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.ObjectInputStream.bytesToDoubles([BI[DII)V")
}

#[async_recursion(?Send)]
async fn bytes_to_floats(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.ObjectInputStream.bytesToFloats([BI[FII)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.ObjectInputStream.bytesToDoubles([BI[DII)V"
    )]
    async fn test_bytes_to_doubles() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = bytes_to_doubles(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.ObjectInputStream.bytesToFloats([BI[FII)V"
    )]
    async fn test_bytes_to_floats() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = bytes_to_floats(thread, Parameters::default()).await;
    }
}
