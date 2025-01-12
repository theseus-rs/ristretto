use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::{ConcurrentVec, Object, Reference, Value};
use std::sync::Arc;

/// Register all native methods for `java.lang.reflect.Array`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/reflect/Array";
    registry.register(
        class_name,
        "get",
        "(Ljava/lang/Object;I)Ljava/lang/Object;",
        get,
    );
    registry.register(
        class_name,
        "getBoolean",
        "(Ljava/lang/Object;I)Z",
        get_boolean,
    );
    registry.register(class_name, "getByte", "(Ljava/lang/Object;I)B", get_byte);
    registry.register(class_name, "getChar", "(Ljava/lang/Object;I)C", get_char);
    registry.register(
        class_name,
        "getDouble",
        "(Ljava/lang/Object;I)D",
        get_double,
    );
    registry.register(class_name, "getFloat", "(Ljava/lang/Object;I)F", get_float);
    registry.register(class_name, "getInt", "(Ljava/lang/Object;I)I", get_int);
    registry.register(class_name, "getLength", "(Ljava/lang/Object;)I", get_length);
    registry.register(class_name, "getLong", "(Ljava/lang/Object;I)J", get_long);
    registry.register(class_name, "getShort", "(Ljava/lang/Object;I)S", get_short);
    registry.register(
        class_name,
        "multiNewArray",
        "(Ljava/lang/Class;[I)Ljava/lang/Object;",
        multi_new_array,
    );
    registry.register(
        class_name,
        "newArray",
        "(Ljava/lang/Class;I)Ljava/lang/Object;",
        new_array,
    );
    registry.register(
        class_name,
        "set",
        "(Ljava/lang/Object;ILjava/lang/Object;)V",
        set,
    );
    registry.register(
        class_name,
        "setBoolean",
        "(Ljava/lang/Object;IZ)V",
        set_boolean,
    );
    registry.register(class_name, "setByte", "(Ljava/lang/Object;IB)V", set_byte);
    registry.register(class_name, "setChar", "(Ljava/lang/Object;IC)V", set_char);
    registry.register(
        class_name,
        "setDouble",
        "(Ljava/lang/Object;ID)V",
        set_double,
    );
    registry.register(class_name, "setFloat", "(Ljava/lang/Object;IF)V", set_float);
    registry.register(class_name, "setInt", "(Ljava/lang/Object;II)V", set_int);
    registry.register(class_name, "setLong", "(Ljava/lang/Object;IJ)V", set_long);
    registry.register(class_name, "setShort", "(Ljava/lang/Object;IS)V", set_short);
}

fn get_class_name(value: Value) -> Result<String> {
    let component_type: Object = value.try_into()?;
    let class_name: String = component_type.value("name")?.try_into()?;
    Ok(class_name)
}

#[async_recursion(?Send)]
async fn get(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.get(Ljava/lang/Object;I)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn get_boolean(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.getBoolean(Ljava/lang/Object;I)Z")
}

#[async_recursion(?Send)]
async fn get_byte(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.getByte(Ljava/lang/Object;I)B")
}

#[async_recursion(?Send)]
async fn get_char(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.getChar(Ljava/lang/Object;I)C")
}

#[async_recursion(?Send)]
async fn get_double(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.getDouble(Ljava/lang/Object;I)D")
}

#[async_recursion(?Send)]
async fn get_float(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.getFloat(Ljava/lang/Object;I)F")
}

#[async_recursion(?Send)]
async fn get_int(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.getInt(Ljava/lang/Object;I)I")
}

#[async_recursion(?Send)]
async fn get_length(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.getLength(Ljava/lang/Object;)I")
}

#[async_recursion(?Send)]
async fn get_long(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.getLong(Ljava/lang/Object;I)J")
}

#[async_recursion(?Send)]
async fn get_short(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.getShort(Ljava/lang/Object;I)S")
}

#[async_recursion(?Send)]
async fn multi_new_array(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.multiNewArray(Ljava/lang/Class;[I)Ljava/lang/Object;")
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
            Reference::Array(class, ConcurrentVec::from(vec![None; length]))
        }
    };

    let value = Value::from(array);
    Ok(Some(value))
}

#[async_recursion(?Send)]
async fn set(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.set(Ljava/lang/Object;ILjava/lang/Object;)V")
}

#[async_recursion(?Send)]
async fn set_boolean(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.setBoolean(Ljava/lang/Object;IZ)V")
}

#[async_recursion(?Send)]
async fn set_byte(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.setByte(Ljava/lang/Object;IB)V")
}

#[async_recursion(?Send)]
async fn set_char(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.setChar(Ljava/lang/Object;IC)V")
}

#[async_recursion(?Send)]
async fn set_double(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.setDouble(Ljava/lang/Object;ID)V")
}

#[async_recursion(?Send)]
async fn set_float(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.setFloat(Ljava/lang/Object;IF)V")
}

#[async_recursion(?Send)]
async fn set_int(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.setInt(Ljava/lang/Object;II)V")
}

#[async_recursion(?Send)]
async fn set_long(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.setLong(Ljava/lang/Object;IJ)V")
}

#[async_recursion(?Send)]
async fn set_short(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.setShort(Ljava/lang/Object;IS)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "java/lang/reflect/Array";
        assert!(registry
            .method(class_name, "get", "(Ljava/lang/Object;I)Ljava/lang/Object;")
            .is_some());
        assert!(registry
            .method(class_name, "getBoolean", "(Ljava/lang/Object;I)Z")
            .is_some());
        assert!(registry
            .method(class_name, "getByte", "(Ljava/lang/Object;I)B")
            .is_some());
        assert!(registry
            .method(class_name, "getChar", "(Ljava/lang/Object;I)C")
            .is_some());
        assert!(registry
            .method(class_name, "getDouble", "(Ljava/lang/Object;I)D")
            .is_some());
        assert!(registry
            .method(class_name, "getFloat", "(Ljava/lang/Object;I)F")
            .is_some());
        assert!(registry
            .method(class_name, "getInt", "(Ljava/lang/Object;I)I")
            .is_some());
        assert!(registry
            .method(class_name, "getLength", "(Ljava/lang/Object;)I")
            .is_some());
        assert!(registry
            .method(class_name, "getLong", "(Ljava/lang/Object;I)J")
            .is_some());
        assert!(registry
            .method(class_name, "getShort", "(Ljava/lang/Object;I)S")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "multiNewArray",
                "(Ljava/lang/Class;[I)Ljava/lang/Object;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "newArray",
                "(Ljava/lang/Class;I)Ljava/lang/Object;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "set",
                "(Ljava/lang/Object;ILjava/lang/Object;)V"
            )
            .is_some());
        assert!(registry
            .method(class_name, "setBoolean", "(Ljava/lang/Object;IZ)V")
            .is_some());
        assert!(registry
            .method(class_name, "setByte", "(Ljava/lang/Object;IB)V")
            .is_some());
        assert!(registry
            .method(class_name, "setChar", "(Ljava/lang/Object;IC)V")
            .is_some());
        assert!(registry
            .method(class_name, "setDouble", "(Ljava/lang/Object;ID)V")
            .is_some());
        assert!(registry
            .method(class_name, "setFloat", "(Ljava/lang/Object;IF)V")
            .is_some());
        assert!(registry
            .method(class_name, "setInt", "(Ljava/lang/Object;II)V")
            .is_some());
        assert!(registry
            .method(class_name, "setLong", "(Ljava/lang/Object;IJ)V")
            .is_some());
        assert!(registry
            .method(class_name, "setShort", "(Ljava/lang/Object;IS)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.reflect.Array.get(Ljava/lang/Object;I)Ljava/lang/Object;"
    )]
    async fn test_get() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.reflect.Array.getBoolean(Ljava/lang/Object;I)Z"
    )]
    async fn test_get_boolean() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_boolean(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.reflect.Array.getByte(Ljava/lang/Object;I)B"
    )]
    async fn test_get_byte() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_byte(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.reflect.Array.getChar(Ljava/lang/Object;I)C"
    )]
    async fn test_get_char() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_char(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.reflect.Array.getDouble(Ljava/lang/Object;I)D"
    )]
    async fn test_get_double() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_double(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.reflect.Array.getFloat(Ljava/lang/Object;I)F"
    )]
    async fn test_get_float() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_float(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.reflect.Array.getInt(Ljava/lang/Object;I)I"
    )]
    async fn test_get_int() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_int(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.reflect.Array.getLength(Ljava/lang/Object;)I"
    )]
    async fn test_get_length() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_length(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.reflect.Array.getLong(Ljava/lang/Object;I)J"
    )]
    async fn test_get_long() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_long(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.reflect.Array.getShort(Ljava/lang/Object;I)S"
    )]
    async fn test_get_short() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_short(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.reflect.Array.multiNewArray(Ljava/lang/Class;[I)Ljava/lang/Object;"
    )]
    async fn test_multi_new_array() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = multi_new_array(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.reflect.Array.set(Ljava/lang/Object;ILjava/lang/Object;)V"
    )]
    async fn test_set() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.reflect.Array.setBoolean(Ljava/lang/Object;IZ)V"
    )]
    async fn test_set_boolean() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_boolean(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.reflect.Array.setByte(Ljava/lang/Object;IB)V"
    )]
    async fn test_set_byte() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_byte(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.reflect.Array.setChar(Ljava/lang/Object;IC)V"
    )]
    async fn test_set_char() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_char(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.reflect.Array.setDouble(Ljava/lang/Object;ID)V"
    )]
    async fn test_set_double() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_double(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.reflect.Array.setFloat(Ljava/lang/Object;IF)V"
    )]
    async fn test_set_float() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_float(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.reflect.Array.setInt(Ljava/lang/Object;II)V"
    )]
    async fn test_set_int() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_int(thread, Arguments::default()).await;
    }
}
