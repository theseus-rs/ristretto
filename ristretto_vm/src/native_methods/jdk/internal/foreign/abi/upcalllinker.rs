use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `jdk.internal.foreign.abi.UpcallLinker`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/internal/foreign/abi/UpcallLinker";
    registry.register(class_name, "makeUpcallStub", "(Ljava/lang/invoke/MethodHandle;Ljdk/internal/foreign/abi/ABIDescriptor;Ljdk/internal/foreign/abi/UpcallLinker$CallRegs;ZJ)J", make_upcall_stub);
    registry.register(class_name, "registerNatives", "()V", register_natives);
}

#[async_recursion(?Send)]
async fn make_upcall_stub(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.UpcallLinker.makeUpcallStub(Ljava/lang/invoke/MethodHandle;Ljdk/internal/foreign/abi/ABIDescriptor;Ljdk/internal/foreign/abi/UpcallLinker$CallRegs;ZJ)J")
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
        let class_name = "jdk/internal/foreign/abi/UpcallLinker";
        assert!(registry.method(class_name, "makeUpcallStub", "(Ljava/lang/invoke/MethodHandle;Ljdk/internal/foreign/abi/ABIDescriptor;Ljdk/internal/foreign/abi/UpcallLinker$CallRegs;ZJ)J").is_some());
        assert!(registry
            .method(class_name, "registerNatives", "()V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "jdk.internal.foreign.abi.UpcallLinker.makeUpcallStub(Ljava/lang/invoke/MethodHandle;Ljdk/internal/foreign/abi/ABIDescriptor;Ljdk/internal/foreign/abi/UpcallLinker$CallRegs;ZJ)J"
    )]
    async fn test_make_upcall_stub() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = make_upcall_stub(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = register_natives(thread, Arguments::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
