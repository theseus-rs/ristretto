use crate::arguments::Arguments;
use crate::call_stack::CallStack;
use crate::native_methods::registry::MethodRegistry;
use crate::Error::RuntimeError;
use crate::Result;
use ristretto_classloader::Value;
use std::hash::{DefaultHasher, Hash, Hasher};

/// Register all native methods for java.lang.Object.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/Object";
    registry.register(class_name, "<init>", "()V", init);
    registry.register(class_name, "clone", "()Ljava/lang/Object;", clone);
    registry.register(class_name, "getClass", "()Ljava/lang/Class;", get_class);
    registry.register(class_name, "hashCode", "()I", hash_code);
    registry.register(class_name, "notifyAll", "()V", notify_all);
    registry.register(class_name, "registerNatives", "()V", register_natives);
}

#[expect(clippy::needless_pass_by_value)]
#[expect(clippy::unnecessary_wraps)]
fn init(_call_stack: &CallStack, _arguments: Arguments) -> Result<Option<Value>> {
    // This is a no-op method to optimize Object initialization since it is called frequently.
    // This prevents the need to create a new frame and allocate memory unnecessarily for the call
    // to the constructor for every object.
    Ok(None)
}

fn clone(_call_stack: &CallStack, mut arguments: Arguments) -> Result<Option<Value>> {
    let object = arguments.pop_object()?;
    let cloned_object = object.clone();
    Ok(Some(Value::Object(cloned_object)))
}

fn get_class(call_stack: &CallStack, mut arguments: Arguments) -> Result<Option<Value>> {
    let Some(object) = arguments.pop_object()? else {
        return Err(RuntimeError("no object reference defined".to_string()));
    };

    let class_name = object.class_name();
    let vm = call_stack.vm()?;
    let class = vm.to_class_value(call_stack, class_name.as_str())?;
    Ok(Some(class))
}

fn hash_code(_call_stack: &CallStack, mut arguments: Arguments) -> Result<Option<Value>> {
    let Some(object) = arguments.pop_object()? else {
        return Err(RuntimeError("no object reference defined".to_string()));
    };
    let value = format!("{object}");
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    let hash_code = hasher.finish();
    #[expect(clippy::cast_possible_truncation)]
    let hash_code = hash_code as i32;
    Ok(Some(Value::Int(hash_code)))
}

#[expect(clippy::needless_pass_by_value)]
#[expect(clippy::unnecessary_wraps)]
fn notify_all(_call_stack: &CallStack, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[expect(clippy::needless_pass_by_value)]
#[expect(clippy::unnecessary_wraps)]
fn register_natives(_call_stack: &CallStack, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}
