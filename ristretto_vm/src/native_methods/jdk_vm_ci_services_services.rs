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

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn read_system_properties_info(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}