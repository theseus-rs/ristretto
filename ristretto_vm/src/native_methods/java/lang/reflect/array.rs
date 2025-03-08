use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::{Object, Reference, Value};
use std::sync::Arc;

const CLASS_NAME: &str = "java/lang/reflect/Array";

/// Register all native methods for `java.lang.reflect.Array`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "get",
        "(Ljava/lang/Object;I)Ljava/lang/Object;",
        get,
    );
    registry.register(
        CLASS_NAME,
        "getBoolean",
        "(Ljava/lang/Object;I)Z",
        get_boolean,
    );
    registry.register(CLASS_NAME, "getByte", "(Ljava/lang/Object;I)B", get_byte);
    registry.register(CLASS_NAME, "getChar", "(Ljava/lang/Object;I)C", get_char);
    registry.register(
        CLASS_NAME,
        "getDouble",
        "(Ljava/lang/Object;I)D",
        get_double,
    );
    registry.register(CLASS_NAME, "getFloat", "(Ljava/lang/Object;I)F", get_float);
    registry.register(CLASS_NAME, "getInt", "(Ljava/lang/Object;I)I", get_int);
    registry.register(CLASS_NAME, "getLength", "(Ljava/lang/Object;)I", get_length);
    registry.register(CLASS_NAME, "getLong", "(Ljava/lang/Object;I)J", get_long);
    registry.register(CLASS_NAME, "getShort", "(Ljava/lang/Object;I)S", get_short);
    registry.register(
        CLASS_NAME,
        "multiNewArray",
        "(Ljava/lang/Class;[I)Ljava/lang/Object;",
        multi_new_array,
    );
    registry.register(
        CLASS_NAME,
        "newArray",
        "(Ljava/lang/Class;I)Ljava/lang/Object;",
        new_array,
    );
    registry.register(
        CLASS_NAME,
        "set",
        "(Ljava/lang/Object;ILjava/lang/Object;)V",
        set,
    );
    registry.register(
        CLASS_NAME,
        "setBoolean",
        "(Ljava/lang/Object;IZ)V",
        set_boolean,
    );
    registry.register(CLASS_NAME, "setByte", "(Ljava/lang/Object;IB)V", set_byte);
    registry.register(CLASS_NAME, "setChar", "(Ljava/lang/Object;IC)V", set_char);
    registry.register(
        CLASS_NAME,
        "setDouble",
        "(Ljava/lang/Object;ID)V",
        set_double,
    );
    registry.register(CLASS_NAME, "setFloat", "(Ljava/lang/Object;IF)V", set_float);
    registry.register(CLASS_NAME, "setInt", "(Ljava/lang/Object;II)V", set_int);
    registry.register(CLASS_NAME, "setLong", "(Ljava/lang/Object;IJ)V", set_long);
    registry.register(CLASS_NAME, "setShort", "(Ljava/lang/Object;IS)V", set_short);
}

fn get_class_name(value: Value) -> Result<String> {
    let component_type: Object = value.try_into()?;
    let class_name: String = component_type.value("name")?.try_into()?;
    Ok(class_name)
}

#[async_recursion(?Send)]
async fn get(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.get(Ljava/lang/Object;I)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn get_boolean(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.getBoolean(Ljava/lang/Object;I)Z")
}

#[async_recursion(?Send)]
async fn get_byte(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.getByte(Ljava/lang/Object;I)B")
}

#[async_recursion(?Send)]
async fn get_char(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.getChar(Ljava/lang/Object;I)C")
}

#[async_recursion(?Send)]
async fn get_double(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.getDouble(Ljava/lang/Object;I)D")
}

#[async_recursion(?Send)]
async fn get_float(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.getFloat(Ljava/lang/Object;I)F")
}

#[async_recursion(?Send)]
async fn get_int(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.getInt(Ljava/lang/Object;I)I")
}

#[async_recursion(?Send)]
async fn get_length(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.getLength(Ljava/lang/Object;)I")
}

#[async_recursion(?Send)]
async fn get_long(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.getLong(Ljava/lang/Object;I)J")
}

#[async_recursion(?Send)]
async fn get_short(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.getShort(Ljava/lang/Object;I)S")
}

#[async_recursion(?Send)]
async fn multi_new_array(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.multiNewArray(Ljava/lang/Class;[I)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn new_array(thread: Arc<Thread>, mut parameters: Parameters) -> Result<Option<Value>> {
    let length = usize::try_from(parameters.pop_int()?)?;
    let class_name = get_class_name(parameters.pop()?)?;

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
            Reference::from((class, vec![None; length]))
        }
    };

    let value = Value::from(array);
    Ok(Some(value))
}

#[async_recursion(?Send)]
async fn set(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.set(Ljava/lang/Object;ILjava/lang/Object;)V")
}

#[async_recursion(?Send)]
async fn set_boolean(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.setBoolean(Ljava/lang/Object;IZ)V")
}

#[async_recursion(?Send)]
async fn set_byte(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.setByte(Ljava/lang/Object;IB)V")
}

#[async_recursion(?Send)]
async fn set_char(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.setChar(Ljava/lang/Object;IC)V")
}

#[async_recursion(?Send)]
async fn set_double(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.setDouble(Ljava/lang/Object;ID)V")
}

#[async_recursion(?Send)]
async fn set_float(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.setFloat(Ljava/lang/Object;IF)V")
}

#[async_recursion(?Send)]
async fn set_int(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.setInt(Ljava/lang/Object;II)V")
}

#[async_recursion(?Send)]
async fn set_long(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.setLong(Ljava/lang/Object;IJ)V")
}

#[async_recursion(?Send)]
async fn set_short(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.setShort(Ljava/lang/Object;IS)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.reflect.Array.get(Ljava/lang/Object;I)Ljava/lang/Object;"
    )]
    async fn test_get() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.reflect.Array.getBoolean(Ljava/lang/Object;I)Z"
    )]
    async fn test_get_boolean() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_boolean(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.reflect.Array.getByte(Ljava/lang/Object;I)B"
    )]
    async fn test_get_byte() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_byte(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.reflect.Array.getChar(Ljava/lang/Object;I)C"
    )]
    async fn test_get_char() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_char(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.reflect.Array.getDouble(Ljava/lang/Object;I)D"
    )]
    async fn test_get_double() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_double(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.reflect.Array.getFloat(Ljava/lang/Object;I)F"
    )]
    async fn test_get_float() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_float(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.reflect.Array.getInt(Ljava/lang/Object;I)I"
    )]
    async fn test_get_int() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_int(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.reflect.Array.getLength(Ljava/lang/Object;)I"
    )]
    async fn test_get_length() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_length(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.reflect.Array.getLong(Ljava/lang/Object;I)J"
    )]
    async fn test_get_long() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_long(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.reflect.Array.getShort(Ljava/lang/Object;I)S"
    )]
    async fn test_get_short() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_short(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.reflect.Array.multiNewArray(Ljava/lang/Class;[I)Ljava/lang/Object;"
    )]
    async fn test_multi_new_array() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = multi_new_array(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.reflect.Array.set(Ljava/lang/Object;ILjava/lang/Object;)V"
    )]
    async fn test_set() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.reflect.Array.setBoolean(Ljava/lang/Object;IZ)V"
    )]
    async fn test_set_boolean() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_boolean(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.reflect.Array.setByte(Ljava/lang/Object;IB)V"
    )]
    async fn test_set_byte() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_byte(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.reflect.Array.setChar(Ljava/lang/Object;IC)V"
    )]
    async fn test_set_char() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_char(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.reflect.Array.setDouble(Ljava/lang/Object;ID)V"
    )]
    async fn test_set_double() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_double(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.reflect.Array.setFloat(Ljava/lang/Object;IF)V"
    )]
    async fn test_set_float() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_float(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.reflect.Array.setInt(Ljava/lang/Object;II)V"
    )]
    async fn test_set_int() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_int(thread, Parameters::default()).await;
    }
}
