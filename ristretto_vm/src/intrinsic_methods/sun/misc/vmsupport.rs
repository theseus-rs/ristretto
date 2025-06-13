use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/misc/VMSupport.getVMTemporaryDirectory()Ljava/lang/String;",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_vm_temporary_directory(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.VMSupport.getVMTemporaryDirectory()Ljava/lang/String;")
}

#[intrinsic_method(
    "sun/misc/VMSupport.initAgentProperties(Ljava/util/Properties;)Ljava/util/Properties;",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn init_agent_properties(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.VMSupport.initAgentProperties(Ljava/util/Properties;)Ljava/util/Properties;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.misc.VMSupport.getVMTemporaryDirectory()Ljava/lang/String;"
    )]
    async fn test_get_vm_temporary_directory() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_vm_temporary_directory(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.misc.VMSupport.initAgentProperties(Ljava/util/Properties;)Ljava/util/Properties;"
    )]
    async fn test_init_agent_properties() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_agent_properties(thread, Parameters::default()).await;
    }
}
