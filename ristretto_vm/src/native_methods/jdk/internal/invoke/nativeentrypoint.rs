use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `jdk.internal.invoke.NativeEntryPoint`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/internal/invoke/NativeEntryPoint";
    registry.register(class_name, "registerNatives", "()V", register_natives);
    registry.register(
        class_name,
        "vmStorageToVMReg",
        "(II)J",
        vm_storage_to_vm_reg,
    );
}

#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn vm_storage_to_vm_reg(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.invoke.NativeEntryPoint.vmStorageToVMReg(II)J")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "jdk/internal/invoke/NativeEntryPoint";
        assert!(registry
            .method(class_name, "registerNatives", "()V")
            .is_some());
        assert!(registry
            .method(class_name, "vmStorageToVMReg", "(II)J")
            .is_some());
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = register_natives(thread, Arguments::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(expected = "jdk.internal.invoke.NativeEntryPoint.vmStorageToVMReg(II)J")]
    async fn test_vm_storage_to_vm_reg() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = vm_storage_to_vm_reg(thread, Arguments::default()).await;
    }
}
