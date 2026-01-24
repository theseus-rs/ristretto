use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "jdk/vm/ci/runtime/JVMCI.initializeRuntime()Ljdk/vm/ci/runtime/JVMCIRuntime;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn initialize_runtime(
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
