use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "jdk/vm/ci/runtime/JVMCI.initializeRuntime()Ljdk/vm/ci/runtime/JVMCIRuntime;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn initialize_runtime<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.vm.ci.runtime.JVMCI.initializeRuntime()Ljdk/vm/ci/runtime/JVMCIRuntime;".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_initialize_runtime() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = initialize_runtime(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
