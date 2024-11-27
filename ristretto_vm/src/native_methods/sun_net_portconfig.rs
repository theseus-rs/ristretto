use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.net.PortConfig`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/net/PortConfig";
    registry.register(class_name, "getLower0", "()I", get_lower_0);
    registry.register(class_name, "getUpper0", "()I", get_upper_0);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_lower_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_upper_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
