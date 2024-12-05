use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `jdk.internal.foreign.abi.ProgrammableUpcallHandler`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/internal/foreign/abi/ProgrammableUpcallHandler";
    registry.register(class_name, "allocateOptimizedUpcallStub", "(Ljava/lang/invoke/MethodHandle;Ljdk/internal/foreign/abi/ABIDescriptor;Ljdk/internal/foreign/abi/ProgrammableUpcallHandler$CallRegs;)J", allocate_optimized_upcall_stub);
    registry.register(class_name, "allocateUpcallStub", "(Ljava/lang/invoke/MethodHandle;Ljdk/internal/foreign/abi/ABIDescriptor;Ljdk/internal/foreign/abi/BufferLayout;)J", allocate_upcall_stub);
    registry.register(class_name, "registerNatives", "()V", register_natives);
    registry.register(
        class_name,
        "supportsOptimizedUpcalls",
        "()Z",
        supports_optimized_upcalls,
    );
}

#[async_recursion(?Send)]
async fn allocate_optimized_upcall_stub(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.ProgrammableUpcallHandler.allocateOptimizedUpcallStub(Ljava/lang/invoke/MethodHandle;Ljdk/internal/foreign/abi/ABIDescriptor;Ljdk/internal/foreign/abi/ProgrammableUpcallHandler$CallRegs;)J")
}

#[async_recursion(?Send)]
async fn allocate_upcall_stub(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.ProgrammableUpcallHandler.allocateUpcallStub(Ljava/lang/invoke/MethodHandle;Ljdk/internal/foreign/abi/ABIDescriptor;Ljdk/internal/foreign/abi/BufferLayout;)J")
}

#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn supports_optimized_upcalls(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.foreign.abi.ProgrammableUpcallHandler.supportsOptimizedUpcalls()Z")
}
