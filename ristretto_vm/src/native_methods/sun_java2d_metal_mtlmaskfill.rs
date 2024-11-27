use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.java2d.metal.MTLMaskFill`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/metal/MTLMaskFill";
    registry.register(class_name, "maskFill", "(IIIIIII[B)V", mask_fill);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn mask_fill(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
