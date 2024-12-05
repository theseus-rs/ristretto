use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.misc.VMSupport`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/misc/VMSupport";
    registry.register(
        class_name,
        "getVMTemporaryDirectory",
        "()Ljava/lang/String;",
        get_vm_temporary_directory,
    );
    registry.register(
        class_name,
        "initAgentProperties",
        "(Ljava/util/Properties;)Ljava/util/Properties;",
        init_agent_properties,
    );
}

#[async_recursion(?Send)]
async fn get_vm_temporary_directory(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.misc.VMSupport.getVMTemporaryDirectory()Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn init_agent_properties(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.misc.VMSupport.initAgentProperties(Ljava/util/Properties;)Ljava/util/Properties;")
}
