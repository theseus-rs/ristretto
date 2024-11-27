use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.awt.PlatformGraphicsInfo`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/awt/PlatformGraphicsInfo";
    registry.register(class_name, "isInAquaSession", "()Z", is_in_aqua_session);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn is_in_aqua_session(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
