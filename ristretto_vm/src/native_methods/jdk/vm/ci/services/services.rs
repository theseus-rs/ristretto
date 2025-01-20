use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "jdk/vm/ci/services/Services";

/// Register all native methods for `jdk.vm.ci.services.Services`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "readSystemPropertiesInfo",
        "([I)J",
        read_system_properties_info,
    );
}

#[async_recursion(?Send)]
async fn read_system_properties_info(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.vm.ci.services.Services.readSystemPropertiesInfo([I)J")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.vm.ci.services.Services.readSystemPropertiesInfo([I)J"
    )]
    async fn test_read_system_properties_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = read_system_properties_info(thread, Parameters::default()).await;
    }
}
