use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::Equal;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "jdk/internal/foreign/abi/ProgrammableInvoker.generateAdapter(Ljdk/internal/foreign/abi/ABIDescriptor;Ljdk/internal/foreign/abi/BufferLayout;)J",
    Equal(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn generate_adapter(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "jdk.internal.foreign.abi.ProgrammableInvoker.generateAdapter(Ljdk/internal/foreign/abi/ABIDescriptor;Ljdk/internal/foreign/abi/BufferLayout;)J"
    )
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/ProgrammableInvoker.invokeNative(JJ)V",
    Equal(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn invoke_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.ProgrammableInvoker.invokeNative(JJ)V")
}

#[intrinsic_method(
    "jdk/internal/foreign/abi/ProgrammableInvoker.registerNatives()V",
    Equal(JAVA_17)
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
        expected = "not yet implemented: jdk.internal.foreign.abi.ProgrammableInvoker.generateAdapter(Ljdk/internal/foreign/abi/ABIDescriptor;Ljdk/internal/foreign/abi/BufferLayout;)J"
    )]
    async fn test_generate_adapter() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = generate_adapter(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.foreign.abi.ProgrammableInvoker.invokeNative(JJ)V"
    )]
    async fn test_invoke_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = register_natives(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
