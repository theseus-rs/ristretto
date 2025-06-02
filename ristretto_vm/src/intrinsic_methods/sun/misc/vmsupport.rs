use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/misc/VMSupport";

/// Register all intrinsic methods for `sun.misc.VMSupport`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "getVMTemporaryDirectory",
        "()Ljava/lang/String;",
        get_vm_temporary_directory,
    );
    registry.register(
        CLASS_NAME,
        "initAgentProperties",
        "(Ljava/util/Properties;)Ljava/util/Properties;",
        init_agent_properties,
    );
}

#[async_recursion(?Send)]
async fn get_vm_temporary_directory(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.VMSupport.getVMTemporaryDirectory()Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn init_agent_properties(
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
