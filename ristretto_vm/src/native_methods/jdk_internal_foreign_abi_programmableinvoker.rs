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

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn generate_adapter(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn invoke_native(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}
