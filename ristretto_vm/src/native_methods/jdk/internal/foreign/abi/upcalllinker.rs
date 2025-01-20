use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "jdk/internal/foreign/abi/UpcallLinker";

/// Register all native methods for `jdk.internal.foreign.abi.UpcallLinker`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "makeUpcallStub", "(Ljava/lang/invoke/MethodHandle;Ljdk/internal/foreign/abi/ABIDescriptor;Ljdk/internal/foreign/abi/UpcallLinker$CallRegs;ZJ)J", make_upcall_stub);
    registry.register(CLASS_NAME, "registerNatives", "()V", register_natives);
}

#[async_recursion(?Send)]
async fn make_upcall_stub(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.UpcallLinker.makeUpcallStub(Ljava/lang/invoke/MethodHandle;Ljdk/internal/foreign/abi/ABIDescriptor;Ljdk/internal/foreign/abi/UpcallLinker$CallRegs;ZJ)J")
}

#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.UpcallLinker.makeUpcallStub(Ljava/lang/invoke/MethodHandle;Ljdk/internal/foreign/abi/ABIDescriptor;Ljdk/internal/foreign/abi/UpcallLinker$CallRegs;ZJ)J"
    )]
    async fn test_make_upcall_stub() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = make_upcall_stub(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = register_natives(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
