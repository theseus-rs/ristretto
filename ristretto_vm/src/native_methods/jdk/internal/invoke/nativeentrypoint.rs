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
    todo!()
}
