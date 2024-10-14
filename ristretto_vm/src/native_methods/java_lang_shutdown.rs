use crate::arguments::Arguments;
use crate::call_stack::CallStack;
use crate::native_methods::registry::MethodRegistry;
use crate::Result;
use ristretto_classloader::Value;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Register all native methods for java.lang.Shutdown.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/Shutdown";
    registry.register(class_name, "halt0", "(I)V", halt0);
}

#[expect(clippy::needless_pass_by_value)]
fn halt0(
    _call_stack: Arc<CallStack>,
    mut arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move {
        let code = arguments.pop_int()?;
        std::process::exit(code);
    })
}
