use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `jdk.vm.ci.services.Services`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/vm/ci/services/Services";
    registry.register(
        class_name,
        "readSystemPropertiesInfo",
        "([I)J",
        read_system_properties_info,
    );
}

#[async_recursion(?Send)]
async fn read_system_properties_info(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.vm.ci.services.Services.readSystemPropertiesInfo([I)J")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "jdk/vm/ci/services/Services";
        assert!(registry
            .method(class_name, "readSystemPropertiesInfo", "([I)J")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.vm.ci.services.Services.readSystemPropertiesInfo([I)J"
    )]
    async fn test_read_system_properties_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = read_system_properties_info(thread, Arguments::default()).await;
    }
}
