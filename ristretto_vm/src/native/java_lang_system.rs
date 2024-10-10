use crate::call_stack::CallStack;
use crate::native::registry::NativeRegistry;
use crate::Error::RuntimeError;
use crate::{Result, VM};
use ristretto_classloader::Value;
use std::time::{SystemTime, UNIX_EPOCH};

/// Register all native methods for the system class.
pub(crate) fn register(registry: &mut NativeRegistry) {
    let class_name = "java/lang/System";
    registry.register(class_name, "currentTimeMillis", "()J", current_time_millis);
    registry.register(class_name, "gc", "()V", gc);
    registry.register(class_name, "nanoTime", "()J", nano_time);
    registry.register(class_name, "registerNatives", "()V", register_natives);
}

#[expect(clippy::needless_pass_by_value)]
fn current_time_millis(
    _vm: &VM,
    _call_stack: &mut CallStack,
    _arguments: Vec<Value>,
) -> Result<Option<Value>> {
    let now = SystemTime::now();
    let duration = now
        .duration_since(UNIX_EPOCH)
        .map_err(|error| RuntimeError(error.to_string()))?;
    let time = i64::try_from(duration.as_millis())?;
    Ok(Some(Value::Long(time)))
}

#[expect(clippy::needless_pass_by_value)]
fn exit(_vm: &VM, _call_stack: &mut CallStack, arguments: Vec<Value>) -> Result<Option<Value>> {
    let Some(Value::Int(code)) = arguments.first() else {
        return Err(RuntimeError("exit status must be an integer".to_string()));
    };
    std::process::exit(*code);
}

#[expect(clippy::needless_pass_by_value)]
#[expect(clippy::unnecessary_wraps)]
fn gc(_vm: &VM, _call_stack: &mut CallStack, _arguments: Vec<Value>) -> Result<Option<Value>> {
    Ok(None)
}

#[expect(clippy::needless_pass_by_value)]
fn nano_time(
    _vm: &VM,
    _call_stack: &mut CallStack,
    _arguments: Vec<Value>,
) -> Result<Option<Value>> {
    let now = SystemTime::now();
    let duration = now
        .duration_since(UNIX_EPOCH)
        .map_err(|error| RuntimeError(error.to_string()))?;
    let time = i64::try_from(duration.as_nanos())?;
    Ok(Some(Value::Long(time)))
}

#[expect(clippy::needless_pass_by_value)]
#[expect(clippy::unnecessary_wraps)]
fn register_natives(
    _vm: &VM,
    _call_stack: &mut CallStack,
    _arguments: Vec<Value>,
) -> Result<Option<Value>> {
    Ok(None)
}
