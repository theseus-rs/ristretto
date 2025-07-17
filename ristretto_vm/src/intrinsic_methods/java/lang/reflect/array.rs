use crate::Error::{InvalidOperand, InvalidStackValue};
use crate::JavaError::{IndexOutOfBoundsException, NullPointerException};
use crate::Result;
use crate::intrinsic_methods::java::lang::class::get_class;
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
pub(crate) async fn get(_thread: Arc<Thread>, mut parameters: Parameters) -> Result<Option<Value>> {
    let index = parameters.pop_int()?;
    let reference = parameters.pop_reference()?;
    let value = match reference {
        None => return Err(NullPointerException("array cannot be null".to_string()).into()),
        Some(Reference::ByteArray(ref array)) => {
            let Some(value) = array.get(usize::try_from(index)?)? else {
                let size = i32::try_from(array.len()?)?;
                return Err(IndexOutOfBoundsException { index, size }.into());
            };
            Value::from(value)
        }
        Some(Reference::CharArray(ref array)) => {
            let Some(value) = array.get(usize::try_from(index)?)? else {
                let size = i32::try_from(array.len()?)?;
                return Err(IndexOutOfBoundsException { index, size }.into());
            };
            Value::from(value)
        }
        Some(Reference::FloatArray(ref array)) => {
            let Some(value) = array.get(usize::try_from(index)?)? else {
                let size = i32::try_from(array.len()?)?;
                return Err(IndexOutOfBoundsException { index, size }.into());
            };
            Value::from(value)
        }
        Some(Reference::DoubleArray(ref array)) => {
            let Some(value) = array.get(usize::try_from(index)?)? else {
                let size = i32::try_from(array.len()?)?;
                return Err(IndexOutOfBoundsException { index, size }.into());
            };
            Value::from(value)
        }
        Some(Reference::ShortArray(ref array)) => {
            let Some(value) = array.get(usize::try_from(index)?)? else {
                let size = i32::try_from(array.len()?)?;
                return Err(IndexOutOfBoundsException { index, size }.into());
            };
            Value::from(value)
        }
        Some(Reference::IntArray(ref array)) => {
            let Some(value) = array.get(usize::try_from(index)?)? else {
                let size = i32::try_from(array.len()?)?;
                return Err(IndexOutOfBoundsException { index, size }.into());
            };
            Value::from(value)
        }
        Some(Reference::LongArray(ref array)) => {
            let Some(value) = array.get(usize::try_from(index)?)? else {
                let size = i32::try_from(array.len()?)?;
                return Err(IndexOutOfBoundsException { index, size }.into());
            };
            Value::from(value)
        }
        Some(Reference::Array(ref object_array)) => {
            let Some(value) = object_array.elements.get(usize::try_from(index)?)? else {
                let size = i32::try_from(object_array.elements.len()?)?;
                return Err(IndexOutOfBoundsException { index, size }.into());
            };
            Value::from(value)
        }
        Some(object) => {
            return Err(InvalidStackValue {
                expected: "array".to_string(),
                actual: object.to_string(),
            });
        }
    };
    Ok(Some(value))
}

#[intrinsic_method("java/lang/reflect/Array.getBoolean(Ljava/lang/Object;I)Z", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_boolean(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    get_byte(thread, parameters).await
}

#[intrinsic_method("java/lang/reflect/Array.getByte(Ljava/lang/Object;I)B", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_byte(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let index = parameters.pop_int()?;
    let reference = parameters.pop_reference()?;
    let value = match reference {
        None => return Err(NullPointerException("array cannot be null".to_string()).into()),
        Some(Reference::ByteArray(ref array)) => {
            let Some(value) = array.get(usize::try_from(index)?)? else {
                let size = i32::try_from(array.len()?)?;
                return Err(IndexOutOfBoundsException { index, size }.into());
            };
            Value::from(value)
        }
        _ => {
            return Err(InvalidStackValue {
                expected: "array".to_string(),
                actual: format!("{reference:?}"),
            });
        }
    };
    Ok(Some(value))
}

#[intrinsic_method("java/lang/reflect/Array.getChar(Ljava/lang/Object;I)C", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_char(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let index = parameters.pop_int()?;
    let reference = parameters.pop_reference()?;
    let value = match reference {
        None => return Err(NullPointerException("array cannot be null".to_string()).into()),
        Some(Reference::CharArray(ref array)) => {
            let Some(value) = array.get(usize::try_from(index)?)? else {
                let size = i32::try_from(array.len()?)?;
                return Err(IndexOutOfBoundsException { index, size }.into());
            };
            Value::from(value)
        }
        _ => {
            return Err(InvalidStackValue {
                expected: "array".to_string(),
                actual: format!("{reference:?}"),
            });
        }
    };
    Ok(Some(value))
}

#[intrinsic_method("java/lang/reflect/Array.getDouble(Ljava/lang/Object;I)D", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_double(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let index = parameters.pop_int()?;
    let reference = parameters.pop_reference()?;
    let value = match reference {
        None => return Err(NullPointerException("array cannot be null".to_string()).into()),
        Some(Reference::DoubleArray(ref array)) => {
            let Some(value) = array.get(usize::try_from(index)?)? else {
                let size = i32::try_from(array.len()?)?;
                return Err(IndexOutOfBoundsException { index, size }.into());
            };
            Value::from(value)
        }
        _ => {
            return Err(InvalidStackValue {
                expected: "array".to_string(),
                actual: format!("{reference:?}"),
            });
        }
    };
    Ok(Some(value))
}

#[intrinsic_method("java/lang/reflect/Array.getFloat(Ljava/lang/Object;I)F", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_float(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let index = parameters.pop_int()?;
    let reference = parameters.pop_reference()?;
    let value = match reference {
        None => return Err(NullPointerException("array cannot be null".to_string()).into()),
        Some(Reference::FloatArray(ref array)) => {
            let Some(value) = array.get(usize::try_from(index)?)? else {
                let size = i32::try_from(array.len()?)?;
                return Err(IndexOutOfBoundsException { index, size }.into());
            };
            Value::from(value)
        }
        _ => {
            return Err(InvalidStackValue {
                expected: "array".to_string(),
                actual: format!("{reference:?}"),
            });
        }
    };
    Ok(Some(value))
}

#[intrinsic_method("java/lang/reflect/Array.getInt(Ljava/lang/Object;I)I", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_int(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let index = parameters.pop_int()?;
    let reference = parameters.pop_reference()?;
    let value = match reference {
        None => return Err(NullPointerException("array cannot be null".to_string()).into()),
        Some(Reference::IntArray(ref array)) => {
            let Some(value) = array.get(usize::try_from(index)?)? else {
                let size = i32::try_from(array.len()?)?;
                return Err(IndexOutOfBoundsException { index, size }.into());
            };
            Value::from(value)
        }
        _ => {
            return Err(InvalidStackValue {
                expected: "array".to_string(),
                actual: format!("{reference:?}"),
            });
        }
    };
    Ok(Some(value))
}

#[intrinsic_method("java/lang/reflect/Array.getLength(Ljava/lang/Object;)I", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_length(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let array = parameters.pop_reference()?;
    let length = match array {
        None => return Err(NullPointerException("array cannot be null".to_string()).into()),
        Some(Reference::ByteArray(ref array)) => array.len()?,
        Some(Reference::CharArray(ref array)) => array.len()?,
        Some(Reference::FloatArray(ref array)) => array.len()?,
        Some(Reference::DoubleArray(ref array)) => array.len()?,
        Some(Reference::ShortArray(ref array)) => array.len()?,
        Some(Reference::IntArray(ref array)) => array.len()?,
        Some(Reference::LongArray(ref array)) => array.len()?,
        Some(Reference::Array(ref object_array)) => object_array.elements.len()?,
        Some(object) => {
            return Err(InvalidStackValue {
                expected: "array".to_string(),
                actual: object.to_string(),
            });
        }
    };
    let length = i32::try_from(length)?;
    Ok(Some(Value::Int(length)))
}

#[intrinsic_method("java/lang/reflect/Array.getLong(Ljava/lang/Object;I)J", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_long(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let index = parameters.pop_int()?;
    let reference = parameters.pop_reference()?;
    let value = match reference {
        None => return Err(NullPointerException("array cannot be null".to_string()).into()),
        Some(Reference::LongArray(ref array)) => {
            let Some(value) = array.get(usize::try_from(index)?)? else {
                let size = i32::try_from(array.len()?)?;
                return Err(IndexOutOfBoundsException { index, size }.into());
            };
            Value::from(value)
        }
        _ => {
            return Err(InvalidStackValue {
                expected: "array".to_string(),
                actual: format!("{reference:?}"),
            });
        }
    };
    Ok(Some(value))
}

#[intrinsic_method("java/lang/reflect/Array.getShort(Ljava/lang/Object;I)S", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_short(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let index = parameters.pop_int()?;
    let reference = parameters.pop_reference()?;
    let value = match reference {
        None => return Err(NullPointerException("array cannot be null".to_string()).into()),
        Some(Reference::ShortArray(ref array)) => {
            let Some(value) = array.get(usize::try_from(index)?)? else {
                let size = i32::try_from(array.len()?)?;
                return Err(IndexOutOfBoundsException { index, size }.into());
            };
            Value::from(value)
        }
        _ => {
            return Err(InvalidStackValue {
                expected: "array".to_string(),
                actual: format!("{reference:?}"),
            });
        }
    };
    Ok(Some(value))
}

#[intrinsic_method(
    "java/lang/reflect/Array.multiNewArray(Ljava/lang/Class;[I)Ljava/lang/Object;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn multi_new_array(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let dimensions: Vec<i32> = parameters.pop()?.try_into()?;
    let class_object = parameters.pop_object()?;
    let class = get_class(&thread, &class_object).await?;

    if dimensions.is_empty() {
        return Err(InvalidOperand {
            expected: "non-empty dimensions array".to_string(),
            actual: "empty dimensions array".to_string(),
        });
    }

    let class_name = class.name();
    let array = create_multi_dimensional_array(&thread, class_name, &dimensions).await?;
    Ok(Some(Value::from(array)))
}

#[async_recursion(?Send)]
async fn create_multi_dimensional_array(
    thread: &Thread,
    class_name: &str,
    dimensions: &[i32],
) -> Result<Reference> {
    if dimensions.is_empty() {
        return Err(InvalidOperand {
            expected: "non-empty dimensions".to_string(),
            actual: "empty dimensions".to_string(),
        });
    }

    let length = usize::try_from(dimensions[0])?;

    if dimensions.len() == 1 {
        // Base case: create a single-dimensional array
        let array = match class_name {
            "boolean" | "byte" => Reference::from(vec![0i8; length]),
            "char" => Reference::from(vec![0 as char; length]),
            "float" => Reference::from(vec![0.0f32; length]),
            "double" => Reference::from(vec![0.0f64; length]),
            "int" => Reference::from(vec![0i32; length]),
            "long" => Reference::from(vec![0i64; length]),
            "short" => Reference::from(vec![0i16; length]),
            _ => {
                let array_class_name = format!("[L{class_name};");
                let class = thread.class(&array_class_name).await?;
                Reference::from((class, vec![None; length]))
            }
        };
        Ok(array)
    } else {
        // Recursive case: create array of arrays
        let mut elements = Vec::with_capacity(length);
        for _ in 0..length {
            let sub_array =
                create_multi_dimensional_array(thread, class_name, &dimensions[1..]).await?;
            elements.push(Some(sub_array));
        }

        // Create the appropriate array class name for the multi-dimensional array
        let array_class_name = if class_name.starts_with('[') {
            format!("[{class_name}")
        } else {
            match class_name {
                "boolean" | "byte" => format!("{}{}", "[".repeat(dimensions.len()), "B"),
                "char" => format!("{}{}", "[".repeat(dimensions.len()), "C"),
                "float" => format!("{}{}", "[".repeat(dimensions.len()), "F"),
                "double" => format!("{}{}", "[".repeat(dimensions.len()), "D"),
                "int" => format!("{}{}", "[".repeat(dimensions.len()), "I"),
                "long" => format!("{}{}", "[".repeat(dimensions.len()), "J"),
                "short" => format!("{}{}", "[".repeat(dimensions.len()), "S"),
                _ => format!("{}[L{class_name};", "[".repeat(dimensions.len() - 1)),
            }
        };

        let class = thread.class(&array_class_name).await?;
        Ok(Reference::from((class, elements)))
    }
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
pub(crate) async fn set(_thread: Arc<Thread>, mut parameters: Parameters) -> Result<Option<Value>> {
    let value = parameters.pop()?;
    let index = usize::try_from(parameters.pop_int()?)?;
    let reference = parameters.pop_reference()?;
    match reference {
        None => return Err(NullPointerException("array cannot be null".to_string()).into()),
        Some(Reference::ByteArray(ref array)) => {
            let value = i8::try_from(value.to_int()?)?;
            array.set(index, value)?;
        }
        Some(Reference::CharArray(ref array)) => {
            let value = u16::try_from(value.to_int()?)?;
            array.set(index, value)?;
        }
        Some(Reference::FloatArray(ref array)) => {
            let value = value.to_float()?;
            array.set(index, value)?;
        }
        Some(Reference::DoubleArray(ref array)) => {
            let value = value.to_double()?;
            array.set(index, value)?;
        }
        Some(Reference::ShortArray(ref array)) => {
            let value = i16::try_from(value.to_int()?)?;
            array.set(index, value)?;
        }
        Some(Reference::IntArray(ref array)) => {
            let value = value.to_int()?;
            array.set(index, value)?;
        }
        Some(Reference::LongArray(ref array)) => {
            let value = value.to_long()?;
            array.set(index, value)?;
        }
        Some(Reference::Array(ref object_array)) => {
            let Value::Object(value) = value else {
                return Err(InvalidOperand {
                    expected: "reference".to_string(),
                    actual: format!("{value:?}"),
                });
            };
            object_array.elements.set(index, value)?;
        }
        Some(object) => {
            return Err(InvalidStackValue {
                expected: "array".to_string(),
                actual: object.to_string(),
            });
        }
    };
    Ok(None)
}

#[intrinsic_method("java/lang/reflect/Array.setBoolean(Ljava/lang/Object;IZ)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn set_boolean(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    set_byte(thread, parameters).await
}

#[intrinsic_method("java/lang/reflect/Array.setByte(Ljava/lang/Object;IB)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn set_byte(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value = i8::try_from(parameters.pop_int()?)?;
    let index = usize::try_from(parameters.pop_int()?)?;
    let reference = parameters.pop_reference()?;
    match reference {
        None => return Err(NullPointerException("array cannot be null".to_string()).into()),
        Some(Reference::ByteArray(ref array)) => {
            array.set(index, value)?;
        }
        _ => {
            return Err(InvalidStackValue {
                expected: "array".to_string(),
                actual: format!("{reference:?}"),
            });
        }
    };
    Ok(None)
}

#[intrinsic_method("java/lang/reflect/Array.setChar(Ljava/lang/Object;IC)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn set_char(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value = u16::try_from(parameters.pop_int()?)?;
    let index = usize::try_from(parameters.pop_int()?)?;
    let reference = parameters.pop_reference()?;
    match reference {
        None => return Err(NullPointerException("array cannot be null".to_string()).into()),
        Some(Reference::CharArray(ref array)) => {
            array.set(index, value)?;
        }
        _ => {
            return Err(InvalidStackValue {
                expected: "array".to_string(),
                actual: format!("{reference:?}"),
            });
        }
    };
    Ok(None)
}

#[intrinsic_method("java/lang/reflect/Array.setDouble(Ljava/lang/Object;ID)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn set_double(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value = parameters.pop_double()?;
    let index = usize::try_from(parameters.pop_int()?)?;
    let reference = parameters.pop_reference()?;
    match reference {
        None => return Err(NullPointerException("array cannot be null".to_string()).into()),
        Some(Reference::DoubleArray(ref array)) => {
            array.set(index, value)?;
        }
        _ => {
            return Err(InvalidStackValue {
                expected: "array".to_string(),
                actual: format!("{reference:?}"),
            });
        }
    };
    Ok(None)
}

#[intrinsic_method("java/lang/reflect/Array.setFloat(Ljava/lang/Object;IF)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn set_float(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value = parameters.pop_float()?;
    let index = usize::try_from(parameters.pop_int()?)?;
    let reference = parameters.pop_reference()?;
    match reference {
        None => return Err(NullPointerException("array cannot be null".to_string()).into()),
        Some(Reference::FloatArray(ref array)) => {
            array.set(index, value)?;
        }
        _ => {
            return Err(InvalidStackValue {
                expected: "array".to_string(),
                actual: format!("{reference:?}"),
            });
        }
    };
    Ok(None)
}

#[intrinsic_method("java/lang/reflect/Array.setInt(Ljava/lang/Object;II)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn set_int(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value = parameters.pop_int()?;
    let index = usize::try_from(parameters.pop_int()?)?;
    let reference = parameters.pop_reference()?;
    match reference {
        None => return Err(NullPointerException("array cannot be null".to_string()).into()),
        Some(Reference::IntArray(ref array)) => {
            array.set(index, value)?;
        }
        _ => {
            return Err(InvalidStackValue {
                expected: "array".to_string(),
                actual: format!("{reference:?}"),
            });
        }
    };
    Ok(None)
}

#[intrinsic_method("java/lang/reflect/Array.setLong(Ljava/lang/Object;IJ)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn set_long(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value = parameters.pop_long()?;
    let index = usize::try_from(parameters.pop_int()?)?;
    let reference = parameters.pop_reference()?;
    match reference {
        None => return Err(NullPointerException("array cannot be null".to_string()).into()),
        Some(Reference::LongArray(ref array)) => {
            array.set(index, value)?;
        }
        _ => {
            return Err(InvalidStackValue {
                expected: "array".to_string(),
                actual: format!("{reference:?}"),
            });
        }
    };
    Ok(None)
}

#[intrinsic_method("java/lang/reflect/Array.setShort(Ljava/lang/Object;IS)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn set_short(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value = i16::try_from(parameters.pop_int()?)?;
    let index = usize::try_from(parameters.pop_int()?)?;
    let reference = parameters.pop_reference()?;
    match reference {
        None => return Err(NullPointerException("array cannot be null".to_string()).into()),
        Some(Reference::ShortArray(ref array)) => {
            array.set(index, value)?;
        }
        _ => {
            return Err(InvalidStackValue {
                expected: "array".to_string(),
                actual: format!("{reference:?}"),
            });
        }
    };
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::JavaObject;

    #[tokio::test]
    async fn test_get_and_set() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let array = Value::from(vec![0i32]);
        let index = Value::Int(0);
        let expected = 42i32;
        let parameters = Parameters::new(vec![array.clone(), index.clone(), Value::from(expected)]);
        let _ = set(thread.clone(), parameters).await?;
        let parameters = Parameters::new(vec![array, index]);
        let result = get(thread, parameters).await?.expect("value");
        let value: i32 = result.try_into()?;
        assert_eq!(value, expected);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_and_set_boolean() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let array = Value::from(vec![false]);
        let index = Value::Int(0);
        let expected = true;
        let parameters = Parameters::new(vec![array.clone(), index.clone(), Value::from(expected)]);
        let _ = set_boolean(thread.clone(), parameters).await?;
        let parameters = Parameters::new(vec![array, index]);
        let result = get_boolean(thread, parameters).await?.expect("value");
        let value: bool = result.try_into()?;
        assert_eq!(value, expected);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_and_set_byte() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let array = Value::from(vec![0i8]);
        let index = Value::Int(0);
        let expected = 42i8;
        let parameters = Parameters::new(vec![array.clone(), index.clone(), Value::from(expected)]);
        let _ = set_byte(thread.clone(), parameters).await?;
        let parameters = Parameters::new(vec![array, index]);
        let result = get_byte(thread, parameters).await?.expect("value");
        let value: i8 = result.try_into()?;
        assert_eq!(value, expected);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_and_set_char() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let array = Value::from(vec![0 as char]);
        let index = Value::Int(0);
        let expected = 42u16;
        let parameters = Parameters::new(vec![array.clone(), index.clone(), Value::from(expected)]);
        let _ = set_char(thread.clone(), parameters).await?;
        let parameters = Parameters::new(vec![array, index]);
        let result = get_char(thread, parameters).await?.expect("value");
        let value: u16 = result.try_into()?;
        assert_eq!(value, expected);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_and_set_double() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let array = Value::from(vec![0.0f64]);
        let index = Value::Int(0);
        let expected = 42.0f64;
        let parameters = Parameters::new(vec![array.clone(), index.clone(), Value::from(expected)]);
        let _ = set_double(thread.clone(), parameters).await?;
        let parameters = Parameters::new(vec![array, index]);
        let result = get_double(thread, parameters).await?.expect("value");
        let value: f64 = result.try_into()?;
        assert_eq!(value, expected);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_and_set_float() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let array = Value::from(vec![0f32]);
        let index = Value::Int(0);
        let expected = 42f32;
        let parameters = Parameters::new(vec![array.clone(), index.clone(), Value::from(expected)]);
        let _ = set_float(thread.clone(), parameters).await?;
        let parameters = Parameters::new(vec![array, index]);
        let result = get_float(thread, parameters).await?.expect("value");
        let value: f32 = result.try_into()?;
        assert_eq!(value, expected);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_and_set_int() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let array = Value::from(vec![0i32]);
        let index = Value::Int(0);
        let expected = 42i32;
        let parameters = Parameters::new(vec![array.clone(), index.clone(), Value::from(expected)]);
        let _ = set_int(thread.clone(), parameters).await?;
        let parameters = Parameters::new(vec![array, index]);
        let result = get_int(thread, parameters).await?.expect("value");
        let value: i32 = result.try_into()?;
        assert_eq!(value, expected);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_length() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let array = Value::from(vec![1, 2, 3]);
        let parameters = Parameters::new(vec![array]);
        let result = get_length(thread, parameters).await?.expect("Array length");
        let length: i32 = result.try_into()?;
        assert_eq!(length, 3);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_and_set_long() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let array = Value::from(vec![0i64]);
        let index = Value::Int(0);
        let expected = 42i64;
        let parameters = Parameters::new(vec![array.clone(), index.clone(), Value::from(expected)]);
        let _ = set_long(thread.clone(), parameters).await?;
        let parameters = Parameters::new(vec![array, index]);
        let result = get_long(thread, parameters).await?.expect("value");
        let value: i64 = result.try_into()?;
        assert_eq!(value, expected);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_and_set_short() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let array = Value::from(vec![0i16]);
        let index = Value::Int(0);
        let expected = 42i16;
        let parameters = Parameters::new(vec![array.clone(), index.clone(), Value::from(expected)]);
        let _ = set_short(thread.clone(), parameters).await?;
        let parameters = Parameters::new(vec![array, index]);
        let result = get_short(thread, parameters).await?.expect("value");
        let value: i16 = result.try_into()?;
        assert_eq!(value, expected);
        Ok(())
    }

    #[tokio::test]
    async fn test_multi_new_array_null_class() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let dimensions = Value::from(vec![1]);
        let parameters = Parameters::new(vec![Value::Object(None), dimensions]);
        let result = multi_new_array(thread.clone(), parameters).await;
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_multi_new_array_empty_dimensions() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let class = thread.class("int").await?;
        let class_object = class.to_object(&thread).await?;
        let dimensions = Value::from(Vec::<i32>::new());
        let parameters = Parameters::new(vec![class_object, dimensions]);
        let result = multi_new_array(thread.clone(), parameters).await;
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_multi_new_array_one_dimension() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let class = thread.class("int").await?;
        let class_object = class.to_object(&thread).await?;
        let dimensions = Value::from(vec![5i32]);
        let parameters = Parameters::new(vec![class_object, dimensions]);

        let array = multi_new_array(thread.clone(), parameters)
            .await?
            .expect("array");
        if let Value::Object(Some(Reference::IntArray(array))) = array {
            assert_eq!(array.len()?, 5);
            for i in 0..5 {
                assert_eq!(array.get(i)?, Some(0i32));
            }
        } else {
            panic!("Expected IntArray but got {array:?}");
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_multi_new_array_two_dimensions() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let class = thread.class("int").await?;
        let class_object = class.to_object(&thread).await?;
        let dimensions = Value::from(vec![3i32, 4i32]);
        let parameters = Parameters::new(vec![class_object, dimensions]);

        let array = multi_new_array(thread.clone(), parameters)
            .await?
            .expect("array");
        if let Value::Object(Some(Reference::Array(outer_array))) = array {
            assert_eq!(outer_array.elements.len()?, 3);
            for i in 0..3 {
                let inner_array = outer_array.elements.get(i)?.expect("inner array");
                if let Some(Reference::IntArray(inner_array)) = inner_array {
                    assert_eq!(inner_array.len()?, 4);
                    for j in 0..4 {
                        assert_eq!(inner_array.get(j)?, Some(0i32));
                    }
                } else {
                    panic!("Expected IntArray but got {inner_array:?}");
                }
            }
        } else {
            panic!("Expected Array but got {array:?}");
        }
        Ok(())
    }
}
