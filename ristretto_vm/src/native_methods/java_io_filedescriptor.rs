use crate::arguments::Arguments;
use crate::call_stack::CallStack;
use crate::native_methods::registry::MethodRegistry;
use crate::{Result, VM};
use ristretto_classloader::Value;

/// Register all native methods for java.io.FileDescriptor.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/io/FileDescriptor";
    registry.register(class_name, "getAppend", "(I)Z", get_append);
    registry.register(class_name, "getHandle", "(I)J", get_handle);
    registry.register(class_name, "initIDs", "()V", init_ids);
}

#[expect(clippy::match_same_arms)]
fn get_append(
    _vm: &VM,
    _call_stack: &mut CallStack,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let handle = arguments.pop_int()?;
    let append = match handle {
        0 => {
            // true if stdin is in append mode
            false
        }
        1 => {
            // true if stdout is in append mode
            false
        }
        2 => {
            // true if stderr is in append mode
            false
        }
        _ => false,
    };
    let append = i32::from(append);
    Ok(Some(Value::Int(append)))
}

fn get_handle(
    _vm: &VM,
    _call_stack: &mut CallStack,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let handle = arguments.pop_int()?;
    let handle = i64::from(handle);
    Ok(Some(Value::Long(handle)))
}

#[expect(clippy::needless_pass_by_value)]
#[expect(clippy::unnecessary_wraps)]
fn init_ids(_vm: &VM, _call_stack: &mut CallStack, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}
