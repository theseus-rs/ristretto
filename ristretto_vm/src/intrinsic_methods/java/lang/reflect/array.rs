use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::{Object, Reference, Value};
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

fn get_class_name(value: Value) -> Result<String> {
    let component_type: Object = value.try_into()?;
    let class_name: String = component_type.value("name")?.try_into()?;
    Ok(class_name)
}

#[intrinsic_method(
    "java/lang/reflect/Array.get(Ljava/lang/Object;I)Ljava/lang/Object;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn get(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.get(Ljava/lang/Object;I)Ljava/lang/Object;")
}

#[intrinsic_method("java/lang/reflect/Array.getBoolean(Ljava/lang/Object;I)Z", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_boolean(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.getBoolean(Ljava/lang/Object;I)Z")
}

#[intrinsic_method("java/lang/reflect/Array.getByte(Ljava/lang/Object;I)B", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_byte(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.getByte(Ljava/lang/Object;I)B")
}

#[intrinsic_method("java/lang/reflect/Array.getChar(Ljava/lang/Object;I)C", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_char(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.getChar(Ljava/lang/Object;I)C")
}

#[intrinsic_method("java/lang/reflect/Array.getDouble(Ljava/lang/Object;I)D", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_double(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.getDouble(Ljava/lang/Object;I)D")
}

#[intrinsic_method("java/lang/reflect/Array.getFloat(Ljava/lang/Object;I)F", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_float(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.getFloat(Ljava/lang/Object;I)F")
}

#[intrinsic_method("java/lang/reflect/Array.getInt(Ljava/lang/Object;I)I", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_int(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.getInt(Ljava/lang/Object;I)I")
}

#[intrinsic_method("java/lang/reflect/Array.getLength(Ljava/lang/Object;)I", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_length(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.getLength(Ljava/lang/Object;)I")
}

#[intrinsic_method("java/lang/reflect/Array.getLong(Ljava/lang/Object;I)J", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_long(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.getLong(Ljava/lang/Object;I)J")
}

#[intrinsic_method("java/lang/reflect/Array.getShort(Ljava/lang/Object;I)S", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_short(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.getShort(Ljava/lang/Object;I)S")
}

#[intrinsic_method(
    "java/lang/reflect/Array.multiNewArray(Ljava/lang/Class;[I)Ljava/lang/Object;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn multi_new_array(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.multiNewArray(Ljava/lang/Class;[I)Ljava/lang/Object;")
}

#[intrinsic_method(
    "java/lang/reflect/Array.newArray(Ljava/lang/Class;I)Ljava/lang/Object;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn new_array(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
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

#[intrinsic_method(
    "java/lang/reflect/Array.set(Ljava/lang/Object;ILjava/lang/Object;)V",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn set(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.set(Ljava/lang/Object;ILjava/lang/Object;)V")
}

#[intrinsic_method("java/lang/reflect/Array.setBoolean(Ljava/lang/Object;IZ)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn set_boolean(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.setBoolean(Ljava/lang/Object;IZ)V")
}

#[intrinsic_method("java/lang/reflect/Array.setByte(Ljava/lang/Object;IB)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn set_byte(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.setByte(Ljava/lang/Object;IB)V")
}

#[intrinsic_method("java/lang/reflect/Array.setChar(Ljava/lang/Object;IC)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn set_char(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.setChar(Ljava/lang/Object;IC)V")
}

#[intrinsic_method("java/lang/reflect/Array.setDouble(Ljava/lang/Object;ID)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn set_double(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.setDouble(Ljava/lang/Object;ID)V")
}

#[intrinsic_method("java/lang/reflect/Array.setFloat(Ljava/lang/Object;IF)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn set_float(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.setFloat(Ljava/lang/Object;IF)V")
}

#[intrinsic_method("java/lang/reflect/Array.setInt(Ljava/lang/Object;II)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn set_int(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.setInt(Ljava/lang/Object;II)V")
}

#[intrinsic_method("java/lang/reflect/Array.setLong(Ljava/lang/Object;IJ)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn set_long(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Array.setLong(Ljava/lang/Object;IJ)V")
}

#[intrinsic_method("java/lang/reflect/Array.setShort(Ljava/lang/Object;IS)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn set_short(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
