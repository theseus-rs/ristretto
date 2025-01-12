use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.nio.Bits`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/nio/Bits";
    registry.register(
        class_name,
        "copySwapMemory0",
        "(Ljava/lang/Object;JLjava/lang/Object;JJJ)V",
        copy_swap_memory_0,
    );
}

#[async_recursion(?Send)]
async fn copy_swap_memory_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.nio.Bits.copySwapMemory0(Ljava/lang/Object;JLjava/lang/Object;JJJ)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "java/nio/Bits";
        assert!(registry
            .method(
                class_name,
                "copySwapMemory0",
                "(Ljava/lang/Object;JLjava/lang/Object;JJJ)V"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.nio.Bits.copySwapMemory0(Ljava/lang/Object;JLjava/lang/Object;JJJ)V"
    )]
    async fn test_copy_swap_memory_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = copy_swap_memory_0(thread, Arguments::default()).await;
    }
}
