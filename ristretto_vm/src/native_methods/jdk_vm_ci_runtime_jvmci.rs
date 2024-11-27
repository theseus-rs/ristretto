use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `jdk.vm.ci.runtime.JVMCI`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/vm/ci/runtime/JVMCI";
    registry.register(
        class_name,
        "initializeRuntime",
        "()Ljdk/vm/ci/runtime/JVMCIRuntime;",
        initialize_runtime,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn initialize_runtime(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
