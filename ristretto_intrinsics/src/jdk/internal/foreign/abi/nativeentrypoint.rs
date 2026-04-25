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
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _downcall_stub = parameters.pop_long()?;
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
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _needs_transition = parameters.pop_bool()?;
    let _captured_state_mask = parameters.pop_int()?;
    let _needs_return_buffer = parameters.pop_bool()?;
    let _enc_ret_moves = parameters.pop_reference()?;
    let _enc_arg_moves = parameters.pop_reference()?;
    let _abi = parameters.pop_reference()?;
    let _method_type = parameters.pop_reference()?;
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
        let result = free_downcall_stub_0(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "jdk.internal.foreign.abi.NativeEntryPoint.freeDowncallStub0(J)Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_make_downcall_stub() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = make_downcall_stub(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::from(false),
                Value::Int(0),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "jdk.internal.foreign.abi.NativeEntryPoint.makeDowncallStub(Ljava/lang/invoke/MethodType;Ljdk/internal/foreign/abi/ABIDescriptor;[Ljdk/internal/foreign/abi/VMStorage;[Ljdk/internal/foreign/abi/VMStorage;Z)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = register_natives(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
