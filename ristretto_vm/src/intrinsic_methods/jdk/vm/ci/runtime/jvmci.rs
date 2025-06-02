use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "jdk/vm/ci/runtime/JVMCI";

/// Register all intrinsic methods for `jdk.vm.ci.runtime.JVMCI`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "initializeRuntime",
        "()Ljdk/vm/ci/runtime/JVMCIRuntime;",
        initialize_runtime,
    );
}

#[async_recursion(?Send)]
async fn initialize_runtime(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.vm.ci.runtime.JVMCI.initializeRuntime()Ljdk/vm/ci/runtime/JVMCIRuntime;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.vm.ci.runtime.JVMCI.initializeRuntime()Ljdk/vm/ci/runtime/JVMCIRuntime;"
    )]
    async fn test_initialize_runtime() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = initialize_runtime(thread, Parameters::default()).await;
    }
}
