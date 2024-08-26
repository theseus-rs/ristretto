use crate::call_stack::CallStack;
use crate::native::registry::NativeRegistry;
use crate::Error::RuntimeError;
use crate::{Result, VM};
use ristretto_classloader::Value;

/// Register all native methods for the system class.
pub(crate) fn register(registry: &mut NativeRegistry) {
    let class_name = "java/lang/Object";
    registry.register(class_name, "getClass", "()Ljava/lang/Class;", get_class);
}

#[expect(clippy::needless_pass_by_value)]
fn get_class(vm: &VM, call_stack: &mut CallStack, arguments: Vec<Value>) -> Result<Option<Value>> {
    let Some(Value::Object(Some(reference))) = arguments.first() else {
        return Err(RuntimeError("no object reference defined".to_string()));
    };

    let class_name = reference.class_name();
    let class = vm.to_class_value(call_stack, class_name.as_str())?;
    Ok(Some(class))
}
