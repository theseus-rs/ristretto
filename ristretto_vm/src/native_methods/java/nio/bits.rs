use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/nio/Bits";

/// Register all native methods for `java.nio.Bits`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "copySwapMemory0",
        "(Ljava/lang/Object;JLjava/lang/Object;JJJ)V",
        copy_swap_memory_0,
    );
}

#[async_recursion(?Send)]
async fn copy_swap_memory_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.nio.Bits.copySwapMemory0(Ljava/lang/Object;JLjava/lang/Object;JJJ)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.nio.Bits.copySwapMemory0(Ljava/lang/Object;JLjava/lang/Object;JJJ)V"
    )]
    async fn test_copy_swap_memory_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = copy_swap_memory_0(thread, Parameters::default()).await;
    }
}
