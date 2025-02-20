use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "jdk/internal/foreign/abi/UpcallStubs";

/// Register all native methods for `jdk.internal.foreign.abi.UpcallStubs`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "freeUpcallStub0", "(J)Z", free_upcall_stub_0);
    registry.register(CLASS_NAME, "registerNatives", "()V", register_natives);
}

#[async_recursion(?Send)]
async fn free_upcall_stub_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.UpcallStubs.freeUpcallStub0(J)Z")
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
        expected = "not yet implemented: jdk.internal.foreign.abi.UpcallStubs.freeUpcallStub0(J)Z"
    )]
    async fn test_free_upcall_stub_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = free_upcall_stub_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = register_natives(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
