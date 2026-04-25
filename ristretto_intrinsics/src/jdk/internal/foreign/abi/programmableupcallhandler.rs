use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::Equal;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "jdk/internal/foreign/abi/ProgrammableUpcallHandler.allocateOptimizedUpcallStub(Ljava/lang/invoke/MethodHandle;Ljdk/internal/foreign/abi/ABIDescriptor;Ljdk/internal/foreign/abi/ProgrammableUpcallHandler$CallRegs;)J",
    Equal(JAVA_17)
)]
#[async_method]
pub async fn allocate_optimized_upcall_stub<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg2 = parameters.pop_reference()?;
    let _arg1 = parameters.pop_reference()?;
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("jdk.internal.foreign.abi.ProgrammableUpcallHandler.allocateOptimizedUpcallStub(Ljava/lang/invoke/MethodHandle;Ljdk/internal/foreign/abi/ABIDescriptor;Ljdk/internal/foreign/abi/ProgrammableUpcallHandler$CallRegs;)J".to_string()).into())
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/ProgrammableUpcallHandler.allocateUpcallStub(Ljava/lang/invoke/MethodHandle;Ljdk/internal/foreign/abi/ABIDescriptor;Ljdk/internal/foreign/abi/BufferLayout;)J",
    Equal(JAVA_17)
)]
#[async_method]
pub async fn allocate_upcall_stub<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _layout = parameters.pop_reference()?;
    let _abi = parameters.pop_reference()?;
    let _mh = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("jdk.internal.foreign.abi.ProgrammableUpcallHandler.allocateUpcallStub(Ljava/lang/invoke/MethodHandle;Ljdk/internal/foreign/abi/ABIDescriptor;Ljdk/internal/foreign/abi/BufferLayout;)J".to_string()).into())
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/ProgrammableUpcallHandler.registerNatives()V",
    Equal(JAVA_17)
)]
#[async_method]
pub async fn register_natives<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/ProgrammableUpcallHandler.supportsOptimizedUpcalls()Z",
    Equal(JAVA_17)
)]
#[async_method]
pub async fn supports_optimized_upcalls<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.foreign.abi.ProgrammableUpcallHandler.supportsOptimizedUpcalls()Z"
            .to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_allocate_optimized_upcall_stub() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = allocate_optimized_upcall_stub(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "jdk.internal.foreign.abi.ProgrammableUpcallHandler.allocateOptimizedUpcallStub(Ljava/lang/invoke/MethodHandle;Ljdk/internal/foreign/abi/ABIDescriptor;Ljdk/internal/foreign/abi/ProgrammableUpcallHandler$CallRegs;)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_allocate_upcall_stub() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = allocate_upcall_stub(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "jdk.internal.foreign.abi.ProgrammableUpcallHandler.allocateUpcallStub(Ljava/lang/invoke/MethodHandle;Ljdk/internal/foreign/abi/ABIDescriptor;Ljdk/internal/foreign/abi/BufferLayout;)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_supports_optimized_upcalls() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = supports_optimized_upcalls(thread, Parameters::default()).await;
        assert_eq!(
            "jdk.internal.foreign.abi.ProgrammableUpcallHandler.supportsOptimizedUpcalls()Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::java17_thread().await?;
        let result = register_natives(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
