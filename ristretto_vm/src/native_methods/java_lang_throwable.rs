use crate::arguments::Arguments;
use crate::call_stack::CallStack;
use crate::native_methods::registry::MethodRegistry;
use crate::{Result, VM};
use ristretto_classloader::Value;

/// Register all native methods for java.lang.Throwable.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/Throwable";
    registry.register(
        class_name,
        "fillInStackTrace",
        "(I)Ljava/lang/Throwable;",
        fill_in_stack_trace,
    );
}

fn fill_in_stack_trace(
    _vm: &VM,
    _call_stack: &mut CallStack,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let _dummy = arguments.pop_int()?;
    let throwable = arguments.pop_object()?;
    // TODO: Implement fillInStackTrace
    Ok(Some(Value::Object(throwable)))
}
