use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for java.lang.ref.Reference.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/ref/Reference";
    registry.register(
        class_name,
        "refersTo0",
        "(Ljava/lang/Object;)Z",
        refers_to_0,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn refers_to_0(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let object_argument = arguments.pop_object()?;
    let object = arguments.pop_object()?;
    // TODO: this is performing a pointer equality check which is likely not the correct implementation;
    // re-evaluate this logic
    if object == object_argument {
        Ok(Some(Value::Int(1)))
    } else {
        Ok(Some(Value::Int(0)))
    }
}
