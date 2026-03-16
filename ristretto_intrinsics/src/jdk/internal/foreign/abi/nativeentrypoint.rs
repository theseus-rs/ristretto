use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "jdk/internal/foreign/abi/NativeEntryPoint.freeDowncallStub0(J)Z",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn free_downcall_stub_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.foreign.abi.NativeEntryPoint.freeDowncallStub0(J)Z".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/NativeEntryPoint.makeDowncallStub(Ljava/lang/invoke/MethodType;Ljdk/internal/foreign/abi/ABIDescriptor;[Ljdk/internal/foreign/abi/VMStorage;[Ljdk/internal/foreign/abi/VMStorage;ZIZ)J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn make_downcall_stub<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("jdk.internal.foreign.abi.NativeEntryPoint.makeDowncallStub(Ljava/lang/invoke/MethodType;Ljdk/internal/foreign/abi/ABIDescriptor;[Ljdk/internal/foreign/abi/VMStorage;[Ljdk/internal/foreign/abi/VMStorage;Z)J".to_string()).into())
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/NativeEntryPoint.registerNatives()V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn register_natives<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_free_downcall_stub_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = free_downcall_stub_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_make_downcall_stub() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = make_downcall_stub(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = register_natives(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
