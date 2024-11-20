use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::{ConcurrentVec, Object, Reference, Value};
use std::sync::Arc;

/// Register all native methods for java.lang.reflect.Array.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/reflect/Array";
    registry.register(
        class_name,
        "newArray",
        "(Ljava/lang/Class;I)Ljava/lang/Object;",
        new_array,
    );
}

fn get_class_name(value: Value) -> Result<String> {
    let component_type: Object = value.try_into()?;
    let class_name: String = component_type.value("name")?.try_into()?;
    Ok(class_name)
}

#[async_recursion(?Send)]
async fn new_array(thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let length = usize::try_from(arguments.pop_int()?)?;
    let class_name = get_class_name(arguments.pop()?)?;

    let array = match class_name.as_str() {
        "boolean" | "byte" => Reference::from(vec![0i8; length]),
        "char" => Reference::from(vec![0 as char; length]),
        "float" => Reference::from(vec![0.0f32; length]),
        "double" => Reference::from(vec![0.0f64; length]),
        "int" => Reference::from(vec![0i32; length]),
        "long" => Reference::from(vec![0i64; length]),
        "short" => Reference::from(vec![0i16; length]),
        _ => {
            let class_name = format!("[L{class_name};");
            let class = thread.class(&class_name).await?;
            Reference::Array(class, ConcurrentVec::with_capacity(length))
        }
    };

    let value = Value::from(array);
    Ok(Some(value))
}
