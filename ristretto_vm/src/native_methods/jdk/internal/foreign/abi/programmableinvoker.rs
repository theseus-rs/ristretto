use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `jdk.internal.foreign.abi.ProgrammableInvoker`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/internal/foreign/abi/ProgrammableInvoker";
    registry.register(
        class_name,
        "generateAdapter",
        "(Ljdk/internal/foreign/abi/ABIDescriptor;Ljdk/internal/foreign/abi/BufferLayout;)J",
        generate_adapter,
    );
    registry.register(class_name, "invokeNative", "(JJ)V", invoke_native);
    registry.register(class_name, "registerNatives", "()V", register_natives);
}

#[async_recursion(?Send)]
async fn generate_adapter(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.ProgrammableInvoker.generateAdapter(Ljdk/internal/foreign/abi/ABIDescriptor;Ljdk/internal/foreign/abi/BufferLayout;)J")
}

#[async_recursion(?Send)]
async fn invoke_native(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.ProgrammableInvoker.invokeNative(JJ)V")
}

#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "jdk/internal/foreign/abi/ProgrammableInvoker";
        assert!(registry
            .method(class_name, "generateAdapter", "(Ljdk/internal/foreign/abi/ABIDescriptor;Ljdk/internal/foreign/abi/BufferLayout;)J")
            .is_some());
        assert!(registry
            .method(class_name, "invokeNative", "(JJ)V")
            .is_some());
        assert!(registry
            .method(class_name, "registerNatives", "()V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "jdk.internal.foreign.abi.ProgrammableInvoker.generateAdapter(Ljdk/internal/foreign/abi/ABIDescriptor;Ljdk/internal/foreign/abi/BufferLayout;)J"
    )]
    async fn test_generate_adapter() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = generate_adapter(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "jdk.internal.foreign.abi.ProgrammableInvoker.invokeNative(JJ)V")]
    async fn test_invoke_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_native(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = register_natives(thread, Arguments::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
