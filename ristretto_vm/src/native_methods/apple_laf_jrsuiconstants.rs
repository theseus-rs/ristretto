use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `apple.laf.JRSUIConstants`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "apple/laf/JRSUIConstants";
    registry.register(
        class_name,
        "getPtrForConstant",
        "(I)J",
        get_ptr_for_constant,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_ptr_for_constant(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}
