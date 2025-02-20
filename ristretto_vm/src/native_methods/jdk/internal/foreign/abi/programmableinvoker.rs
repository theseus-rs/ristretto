use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "jdk/internal/foreign/abi/ProgrammableInvoker";

/// Register all native methods for `jdk.internal.foreign.abi.ProgrammableInvoker`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "generateAdapter",
        "(Ljdk/internal/foreign/abi/ABIDescriptor;Ljdk/internal/foreign/abi/BufferLayout;)J",
        generate_adapter,
    );
    registry.register(CLASS_NAME, "invokeNative", "(JJ)V", invoke_native);
    registry.register(CLASS_NAME, "registerNatives", "()V", register_natives);
}

#[async_recursion(?Send)]
async fn generate_adapter(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "jdk.internal.foreign.abi.ProgrammableInvoker.generateAdapter(Ljdk/internal/foreign/abi/ABIDescriptor;Ljdk/internal/foreign/abi/BufferLayout;)J"
    )
}

#[async_recursion(?Send)]
async fn invoke_native(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.ProgrammableInvoker.invokeNative(JJ)V")
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
