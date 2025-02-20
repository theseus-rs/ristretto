use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "jdk/internal/invoke/NativeEntryPoint";

/// Register all native methods for `jdk.internal.invoke.NativeEntryPoint`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "registerNatives", "()V", register_natives);
    registry.register(
        CLASS_NAME,
        "vmStorageToVMReg",
        "(II)J",
        vm_storage_to_vm_reg,
    );
}

#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn vm_storage_to_vm_reg(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.invoke.NativeEntryPoint.vmStorageToVMReg(II)J")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = register_natives(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.invoke.NativeEntryPoint.vmStorageToVMReg(II)J"
    )]
    async fn test_vm_storage_to_vm_reg() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = vm_storage_to_vm_reg(thread, Parameters::default()).await;
    }
}
