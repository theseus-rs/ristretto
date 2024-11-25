use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `jdk.internal.util.SystemProps$Raw`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/internal/util/SystemProps$Raw";
    registry.register(
        class_name,
        "platformProperties",
        "()[Ljava/lang/String;",
        platform_properties,
    );
    registry.register(
        class_name,
        "vmProperties",
        "()[Ljava/lang/String;",
        vm_properties,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn platform_properties(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn vm_properties(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
