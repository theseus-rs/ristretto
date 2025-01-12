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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/misc/VMSupport";
        assert!(registry
            .method(
                class_name,
                "getVMTemporaryDirectory",
                "()Ljava/lang/String;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "initAgentProperties",
                "(Ljava/util/Properties;)Ljava/util/Properties;"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.misc.VMSupport.getVMTemporaryDirectory()Ljava/lang/String;")]
    async fn test_get_vm_temporary_directory() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_vm_temporary_directory(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.misc.VMSupport.initAgentProperties(Ljava/util/Properties;)Ljava/util/Properties;"
    )]
    async fn test_init_agent_properties() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_agent_properties(thread, Arguments::default()).await;
    }
}
