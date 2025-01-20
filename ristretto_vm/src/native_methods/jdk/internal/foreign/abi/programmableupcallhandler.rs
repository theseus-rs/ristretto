use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "jdk/internal/foreign/abi/ProgrammableUpcallHandler";

/// Register all native methods for `jdk.internal.foreign.abi.ProgrammableUpcallHandler`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "allocateOptimizedUpcallStub", "(Ljava/lang/invoke/MethodHandle;Ljdk/internal/foreign/abi/ABIDescriptor;Ljdk/internal/foreign/abi/ProgrammableUpcallHandler$CallRegs;)J", allocate_optimized_upcall_stub);
    registry.register(CLASS_NAME, "allocateUpcallStub", "(Ljava/lang/invoke/MethodHandle;Ljdk/internal/foreign/abi/ABIDescriptor;Ljdk/internal/foreign/abi/BufferLayout;)J", allocate_upcall_stub);
    registry.register(CLASS_NAME, "registerNatives", "()V", register_natives);
    registry.register(
        CLASS_NAME,
        "supportsOptimizedUpcalls",
        "()Z",
        supports_optimized_upcalls,
    );
}

#[async_recursion(?Send)]
async fn allocate_optimized_upcall_stub(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.ProgrammableUpcallHandler.allocateOptimizedUpcallStub(Ljava/lang/invoke/MethodHandle;Ljdk/internal/foreign/abi/ABIDescriptor;Ljdk/internal/foreign/abi/ProgrammableUpcallHandler$CallRegs;)J")
}

#[async_recursion(?Send)]
async fn allocate_upcall_stub(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.ProgrammableUpcallHandler.allocateUpcallStub(Ljava/lang/invoke/MethodHandle;Ljdk/internal/foreign/abi/ABIDescriptor;Ljdk/internal/foreign/abi/BufferLayout;)J")
}

#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn supports_optimized_upcalls(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.ProgrammableUpcallHandler.supportsOptimizedUpcalls()Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.ProgrammableUpcallHandler.allocateOptimizedUpcallStub(Ljava/lang/invoke/MethodHandle;Ljdk/internal/foreign/abi/ABIDescriptor;Ljdk/internal/foreign/abi/ProgrammableUpcallHandler$CallRegs;)J"
    )]
    async fn test_allocate_optimized_upcall_stub() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = allocate_optimized_upcall_stub(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.ProgrammableUpcallHandler.allocateUpcallStub(Ljava/lang/invoke/MethodHandle;Ljdk/internal/foreign/abi/ABIDescriptor;Ljdk/internal/foreign/abi/BufferLayout;)J"
    )]
    async fn test_allocate_upcall_stub() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = allocate_upcall_stub(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.ProgrammableUpcallHandler.supportsOptimizedUpcalls()Z"
    )]
    async fn test_supports_optimized_upcalls() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = supports_optimized_upcalls(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = register_natives(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
