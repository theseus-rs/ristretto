use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "jdk/internal/foreign/abi/UpcallStubs.freeUpcallStub0(J)Z",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub(crate) async fn free_upcall_stub_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.UpcallStubs.freeUpcallStub0(J)Z")
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/UpcallStubs.registerNatives()V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub(crate) async fn register_natives(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
