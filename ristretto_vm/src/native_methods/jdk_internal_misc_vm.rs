use crate::arguments::Arguments;
use crate::call_stack::CallStack;
use crate::native_methods::registry::MethodRegistry;
use crate::Result;
use ristretto_classloader::Value;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Register all native methods for jdk.internal.misc.VM.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/internal/misc/VM";
    registry.register(class_name, "initialize", "()V", initialize);
}

// #[expect(clippy::needless_pass_by_value)]
// #[expect(clippy::unnecessary_wraps)]
// async fn initialize(_call_stack: Arc<CallStack>, _arguments: Arguments) -> Result<Option<Value>> {
//     Ok(None)
// }

#[expect(clippy::needless_pass_by_value)]
fn initialize(
    _call_stack: Arc<CallStack>,
    _arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move { Ok(None) })
}
