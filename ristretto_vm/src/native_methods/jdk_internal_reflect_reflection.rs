use crate::arguments::Arguments;
use crate::call_stack::CallStack;
use crate::native_methods::registry::MethodRegistry;
use crate::Result;
use ristretto_classloader::Value;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Register all native methods for jdk.internal.reflect.Reflection.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/internal/reflect/Reflection";
    registry.register(
        class_name,
        "getCallerClass",
        "()Ljava/lang/Class;",
        get_caller_class,
    );
}

#[expect(clippy::needless_pass_by_value)]
fn get_caller_class(
    call_stack: Arc<CallStack>,
    _arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move {
        let frames = call_stack.frames()?;
        for frame in frames.iter().rev() {
            let class = frame.class();
            let class_name = class.name();

            if class_name == "java/lang/MethodHandles" {
                continue;
            }

            let vm = call_stack.vm()?;
            let class = vm.to_class_value(&call_stack, class_name).await?;
            return Ok(Some(class));
        }
        Ok(None)
    })
}
