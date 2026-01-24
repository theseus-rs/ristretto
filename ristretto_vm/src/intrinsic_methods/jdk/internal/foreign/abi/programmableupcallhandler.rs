use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::Equal;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "jdk/internal/foreign/abi/ProgrammableUpcallHandler.allocateOptimizedUpcallStub(Ljava/lang/invoke/MethodHandle;Ljdk/internal/foreign/abi/ABIDescriptor;Ljdk/internal/foreign/abi/ProgrammableUpcallHandler$CallRegs;)J",
    Equal(JAVA_17)
)]
#[async_method]
pub(crate) async fn allocate_optimized_upcall_stub(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "jdk.internal.foreign.abi.ProgrammableUpcallHandler.allocateOptimizedUpcallStub(Ljava/lang/invoke/MethodHandle;Ljdk/internal/foreign/abi/ABIDescriptor;Ljdk/internal/foreign/abi/ProgrammableUpcallHandler$CallRegs;)J"
    )
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/ProgrammableUpcallHandler.allocateUpcallStub(Ljava/lang/invoke/MethodHandle;Ljdk/internal/foreign/abi/ABIDescriptor;Ljdk/internal/foreign/abi/BufferLayout;)J",
    Equal(JAVA_17)
)]
#[async_method]
pub(crate) async fn allocate_upcall_stub(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "jdk.internal.foreign.abi.ProgrammableUpcallHandler.allocateUpcallStub(Ljava/lang/invoke/MethodHandle;Ljdk/internal/foreign/abi/ABIDescriptor;Ljdk/internal/foreign/abi/BufferLayout;)J"
    )
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/ProgrammableUpcallHandler.registerNatives()V",
    Equal(JAVA_17)
)]
#[async_method]
pub(crate) async fn register_natives(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/ProgrammableUpcallHandler.supportsOptimizedUpcalls()Z",
    Equal(JAVA_17)
)]
#[async_method]
pub(crate) async fn supports_optimized_upcalls(
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
