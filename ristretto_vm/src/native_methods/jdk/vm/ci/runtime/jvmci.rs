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

#[async_recursion(?Send)]
async fn initialize_runtime(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.vm.ci.runtime.JVMCI.initializeRuntime()Ljdk/vm/ci/runtime/JVMCIRuntime;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "jdk/vm/ci/runtime/JVMCI";
        assert!(registry
            .method(
                class_name,
                "initializeRuntime",
                "()Ljdk/vm/ci/runtime/JVMCIRuntime;"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.vm.ci.runtime.JVMCI.initializeRuntime()Ljdk/vm/ci/runtime/JVMCIRuntime;"
    )]
    async fn test_initialize_runtime() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = initialize_runtime(thread, Arguments::default()).await;
    }
}
