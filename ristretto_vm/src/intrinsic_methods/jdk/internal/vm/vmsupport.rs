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
    "jdk/internal/vm/VMSupport.getVMTemporaryDirectory()Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn get_vm_temporary_directory(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.vm.VMSupport.getVMTemporaryDirectory()Ljava/lang/String;")
}

#[intrinsic_method(
    "jdk/internal/vm/VMSupport.initAgentProperties(Ljava/util/Properties;)Ljava/util/Properties;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn init_agent_properties(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "jdk.internal.vm.VMSupport.initAgentProperties(Ljava/util/Properties;)Ljava/util/Properties;"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.vm.VMSupport.getVMTemporaryDirectory()Ljava/lang/String;"
    )]
    async fn test_get_vm_temporary_directory() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_vm_temporary_directory(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.vm.VMSupport.initAgentProperties(Ljava/util/Properties;)Ljava/util/Properties;"
    )]
    async fn test_init_agent_properties() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_agent_properties(thread, Parameters::default()).await;
    }
}
