use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "jdk/internal/foreign/abi/NativeEntryPoint.freeDowncallStub0(J)Z",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn free_downcall_stub_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.NativeEntryPoint.freeDowncallStub0(J)Z")
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/NativeEntryPoint.makeDowncallStub(Ljava/lang/invoke/MethodType;Ljdk/internal/foreign/abi/ABIDescriptor;[Ljdk/internal/foreign/abi/VMStorage;[Ljdk/internal/foreign/abi/VMStorage;ZIZ)J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn make_downcall_stub(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "jdk.internal.foreign.abi.NativeEntryPoint.makeDowncallStub(Ljava/lang/invoke/MethodType;Ljdk/internal/foreign/abi/ABIDescriptor;[Ljdk/internal/foreign/abi/VMStorage;[Ljdk/internal/foreign/abi/VMStorage;Z)J"
    )
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/NativeEntryPoint.registerNatives()V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
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
        expected = "not yet implemented: jdk.internal.foreign.abi.NativeEntryPoint.freeDowncallStub0(J)Z"
    )]
    async fn test_free_downcall_stub_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = free_downcall_stub_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.NativeEntryPoint.makeDowncallStub(Ljava/lang/invoke/MethodType;Ljdk/internal/foreign/abi/ABIDescriptor;[Ljdk/internal/foreign/abi/VMStorage;[Ljdk/internal/foreign/abi/VMStorage;Z)J"
    )]
    async fn test_make_downcall_stub() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = make_downcall_stub(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = register_natives(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
