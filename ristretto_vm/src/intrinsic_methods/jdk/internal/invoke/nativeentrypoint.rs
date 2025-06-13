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
    "jdk/internal/invoke/NativeEntryPoint.registerNatives()V",
    Equal(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn register_natives(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "jdk/internal/invoke/NativeEntryPoint.vmStorageToVMReg(II)J",
    Equal(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn vm_storage_to_vm_reg(
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
