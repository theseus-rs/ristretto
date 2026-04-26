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
    "jdk/internal/vm/VMSupport.getVMTemporaryDirectory()Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_vm_temporary_directory<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.vm.VMSupport.getVMTemporaryDirectory()Ljava/lang/String;".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/internal/vm/VMSupport.initAgentProperties(Ljava/util/Properties;)Ljava/util/Properties;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn init_agent_properties<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _props = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("jdk.internal.vm.VMSupport.initAgentProperties(Ljava/util/Properties;)Ljava/util/Properties;".to_string()).into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_vm_temporary_directory() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_vm_temporary_directory(thread, Parameters::default()).await;
        assert_eq!(
            "jdk.internal.vm.VMSupport.getVMTemporaryDirectory()Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_agent_properties() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            init_agent_properties(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "jdk.internal.vm.VMSupport.initAgentProperties(Ljava/util/Properties;)Ljava/util/Properties;",
            result.unwrap_err().to_string()
        );
    }
}
