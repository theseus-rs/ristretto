use crate::java::lang::class::get_class;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InvalidOperand;
use ristretto_types::JavaError::{
    ArrayIndexOutOfBoundsException, IllegalArgumentException, NegativeArraySizeException,
    NullPointerException,
};
use ristretto_types::Thread;
use ristretto_types::VM;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

/// Convert a Java int index to a Rust usize, returning `ArrayIndexOutOfBoundsException` for
/// negative values instead of propagating a conversion error.
fn to_array_index(index: i32, array_len: usize) -> Result<usize> {
    usize::try_from(index).map_err(|_| {
        ristretto_types::Error::from(ArrayIndexOutOfBoundsException {
            index,
            length: array_len,
        })
    })
}

fn get_class_name(value: &Value) -> Result<String> {
    let Value::Object(Some(reference)) = value else {
        return Err(NullPointerException(None).into());
    };
    let guard = reference.read();
    let component_type = guard.as_object_ref()?;
    let class_name = component_type.value("name")?.as_string()?;
    Ok(class_name)
}

/// Represents an unboxed Java primitive value extracted from a wrapper object.
enum PrimitiveValue {
    Boolean(bool),
    Byte(i8),
    Char(char),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
}

/// Unbox a Java wrapper object (Boolean, Byte, Character, Short, Integer, Long, Float, Double)
/// into its primitive value. Also handles raw VM values (Int, Long, Float, Double) for
/// internal callers.
fn unbox_value(value: &Value) -> Result<PrimitiveValue> {
    match value {
        Value::Int(v) => Ok(PrimitiveValue::Int(*v)),
        Value::Long(v) => Ok(PrimitiveValue::Long(*v)),
        Value::Float(v) => Ok(PrimitiveValue::Float(*v)),
        Value::Double(v) => Ok(PrimitiveValue::Double(*v)),
        Value::Object(Some(reference)) => {
            let guard = reference.read();
            let object = guard.as_object_ref()?;
            let class_name = object.class().name();
            match class_name {
                "java/lang/Boolean" => Ok(PrimitiveValue::Boolean(object.as_bool()?)),
                "java/lang/Byte" => Ok(PrimitiveValue::Byte(object.as_i8()?)),
                "java/lang/Character" => Ok(PrimitiveValue::Char(object.as_char()?)),
                "java/lang/Short" => Ok(PrimitiveValue::Short(object.as_i16()?)),
                "java/lang/Integer" => Ok(PrimitiveValue::Int(object.as_i32()?)),
                "java/lang/Long" => Ok(PrimitiveValue::Long(object.as_i64()?)),
                "java/lang/Float" => Ok(PrimitiveValue::Float(object.as_f32()?)),
                "java/lang/Double" => Ok(PrimitiveValue::Double(object.as_f64()?)),
                _ => Err(illegal_argument("primitive wrapper")),
            }
        }
        _ => Err(illegal_argument("primitive wrapper")),
    }
}

/// Set an unboxed primitive value into a primitive array, applying JLS §5.1.2 widening
/// conversions as needed.
fn set_primitive_to_array(
    array_ref: &mut Reference,
    index: i32,
    pv: &PrimitiveValue,
) -> Result<()> {
    match array_ref {
        Reference::BooleanArray(array) => match pv {
            PrimitiveValue::Boolean(v) => set_array_element(array, index, i8::from(*v)),
            _ => Err(illegal_argument("boolean")),
        },
        Reference::ByteArray(array) => match pv {
            PrimitiveValue::Byte(v) => set_array_element(array, index, *v),
            _ => Err(illegal_argument("byte")),
        },
        Reference::CharArray(array) => match pv {
            PrimitiveValue::Char(v) => set_array_element(array, index, *v as u16),
            _ => Err(illegal_argument("char")),
        },
        Reference::ShortArray(array) => match pv {
            PrimitiveValue::Byte(v) => set_array_element(array, index, i16::from(*v)),
            PrimitiveValue::Short(v) => set_array_element(array, index, *v),
            _ => Err(illegal_argument("short")),
        },
        Reference::IntArray(array) => match pv {
            PrimitiveValue::Byte(v) => set_array_element(array, index, i32::from(*v)),
            PrimitiveValue::Char(v) => set_array_element(array, index, i32::from(*v as u16)),
            PrimitiveValue::Short(v) => set_array_element(array, index, i32::from(*v)),
            PrimitiveValue::Int(v) => set_array_element(array, index, *v),
            _ => Err(illegal_argument("int")),
        },
        Reference::LongArray(array) => match pv {
            PrimitiveValue::Byte(v) => set_array_element(array, index, i64::from(*v)),
            PrimitiveValue::Char(v) => set_array_element(array, index, i64::from(*v as u16)),
            PrimitiveValue::Short(v) => set_array_element(array, index, i64::from(*v)),
            PrimitiveValue::Int(v) => set_array_element(array, index, i64::from(*v)),
            PrimitiveValue::Long(v) => set_array_element(array, index, *v),
            _ => Err(illegal_argument("long")),
        },
        #[expect(clippy::cast_precision_loss)]
        Reference::FloatArray(array) => match pv {
            PrimitiveValue::Byte(v) => set_array_element(array, index, f32::from(*v)),
            PrimitiveValue::Char(v) => set_array_element(array, index, f32::from(*v as u16)),
            PrimitiveValue::Short(v) => set_array_element(array, index, f32::from(*v)),
            PrimitiveValue::Int(v) => set_array_element(array, index, *v as f32),
            PrimitiveValue::Long(v) => set_array_element(array, index, *v as f32),
            PrimitiveValue::Float(v) => set_array_element(array, index, *v),
            _ => Err(illegal_argument("float")),
        },
        #[expect(clippy::cast_precision_loss)]
        Reference::DoubleArray(array) => match pv {
            PrimitiveValue::Byte(v) => set_array_element(array, index, f64::from(*v)),
            PrimitiveValue::Char(v) => set_array_element(array, index, f64::from(*v as u16)),
            PrimitiveValue::Short(v) => set_array_element(array, index, f64::from(*v)),
            PrimitiveValue::Int(v) => set_array_element(array, index, f64::from(*v)),
            PrimitiveValue::Long(v) => set_array_element(array, index, *v as f64),
            PrimitiveValue::Float(v) => set_array_element(array, index, f64::from(*v)),
            PrimitiveValue::Double(v) => set_array_element(array, index, *v),
            PrimitiveValue::Boolean(_) => Err(illegal_argument("double")),
        },
        _ => Err(not_an_array()),
    }
}

/// Return an `IllegalArgumentException` indicating the argument is not an array of the
/// expected type.
fn illegal_argument(expected: &str) -> ristretto_types::Error {
    IllegalArgumentException(format!("argument type mismatch: expected {expected} array")).into()
}

/// Return an `IllegalArgumentException` indicating the argument is not an array.
fn not_an_array() -> ristretto_types::Error {
    IllegalArgumentException("Argument is not an array".to_string()).into()
}

/// Set an element in a typed array slice, returning `ArrayIndexOutOfBoundsException` on failure.
fn set_array_element<T>(array: &mut [T], index: i32, value: T) -> Result<()> {
    let len = array.len();
    let idx = to_array_index(index, len)?;
    let element = array.get_mut(idx).ok_or(ristretto_types::Error::from(
        ArrayIndexOutOfBoundsException { index, length: len },
    ))?;
    *element = value;
    Ok(())
}

/// Get an element from a typed array slice, returning `ArrayIndexOutOfBoundsException` on failure.
fn get_array_element<T: Copy>(array: &[T], index: i32) -> Result<T> {
    let idx = to_array_index(index, array.len())?;
    array.get(idx).copied().ok_or_else(|| {
        ArrayIndexOutOfBoundsException {
            index,
            length: array.len(),
        }
        .into()
    })
}

/// Extract a value from an array reference at the given index. Returns the raw `Value` and
/// an optional `(class, method)` pair describing the boxing wrapper to invoke. When boxing
/// is `None`, the value is already an object reference and can be returned directly.
fn extract_array_element(
    reference: &Reference,
    index: i32,
) -> Result<(Value, Option<(&'static str, &'static str)>)> {
    match reference {
        Reference::BooleanArray(array) => {
            let v = get_array_element(array, index)?;
            Ok((
                Value::from(v != 0),
                Some(("java.lang.Boolean", "valueOf(Z)Ljava/lang/Boolean;")),
            ))
        }
        Reference::ByteArray(array) => {
            let v = get_array_element(array, index)?;
            Ok((
                Value::Int(i32::from(v)),
                Some(("java.lang.Byte", "valueOf(B)Ljava/lang/Byte;")),
            ))
        }
        Reference::CharArray(array) => {
            let v = get_array_element(array, index)?;
            Ok((
                Value::Int(i32::from(v)),
                Some(("java.lang.Character", "valueOf(C)Ljava/lang/Character;")),
            ))
        }
        Reference::FloatArray(array) => {
            let v = get_array_element(array, index)?;
            Ok((
                Value::Float(v),
                Some(("java.lang.Float", "valueOf(F)Ljava/lang/Float;")),
            ))
        }
        Reference::DoubleArray(array) => {
            let v = get_array_element(array, index)?;
            Ok((
                Value::Double(v),
                Some(("java.lang.Double", "valueOf(D)Ljava/lang/Double;")),
            ))
        }
        Reference::ShortArray(array) => {
            let v = get_array_element(array, index)?;
            Ok((
                Value::Int(i32::from(v)),
                Some(("java.lang.Short", "valueOf(S)Ljava/lang/Short;")),
            ))
        }
        Reference::IntArray(array) => {
            let v = get_array_element(array, index)?;
            Ok((
                Value::Int(v),
                Some(("java.lang.Integer", "valueOf(I)Ljava/lang/Integer;")),
            ))
        }
        Reference::LongArray(array) => {
            let v = get_array_element(array, index)?;
            Ok((
                Value::Long(v),
                Some(("java.lang.Long", "valueOf(J)Ljava/lang/Long;")),
            ))
        }
        Reference::Array(object_array) => {
            let array = &object_array.elements;
            let idx = to_array_index(index, array.len())?;
            let v = array.get(idx).ok_or(ristretto_types::Error::from(
                ArrayIndexOutOfBoundsException {
                    index,
                    length: array.len(),
                },
            ))?;
            Ok((v.clone(), None))
        }
        Reference::Object(_) => Err(not_an_array()),
    }
}

#[intrinsic_method(
    "java/lang/reflect/Array.get(Ljava/lang/Object;I)Ljava/lang/Object;",
    Any
)]
#[async_method]
pub async fn get<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let index = parameters.pop_int()?;
    let Some(reference) = parameters.pop_reference()? else {
        return Err(NullPointerException(Some("array cannot be null".to_string())).into());
    };

    // Per JVM spec, Array.get returns Object; primitive values must be boxed.
    // Extract the raw value under the lock; the guard is dropped at the end of
    // the block before any async boxing call.
    let (raw, boxing) = {
        let guard = reference.read();
        extract_array_element(&guard, index)?
    };

    // Box primitive values (lock is released); object values pass through directly
    let value = if let Some((class, method)) = boxing {
        thread.try_invoke(class, method, &[raw]).await?
    } else {
        raw
    };
    Ok(Some(value))
}

#[intrinsic_method("java/lang/reflect/Array.getBoolean(Ljava/lang/Object;I)Z", Any)]
#[async_method]
pub async fn get_boolean<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let index = parameters.pop_int()?;
    let Some(reference) = parameters.pop_reference()? else {
        return Err(NullPointerException(Some("array cannot be null".to_string())).into());
    };
    let guard = reference.read();
    let value = match &*guard {
        Reference::BooleanArray(array) => {
            let v = get_array_element(array, index)?;
            Value::from(v)
        }
        Reference::ByteArray(_)
        | Reference::CharArray(_)
        | Reference::ShortArray(_)
        | Reference::IntArray(_)
        | Reference::LongArray(_)
        | Reference::FloatArray(_)
        | Reference::DoubleArray(_)
        | Reference::Array(_) => return Err(illegal_argument("boolean")),
        Reference::Object(_) => return Err(not_an_array()),
    };
    Ok(Some(value))
}

#[intrinsic_method("java/lang/reflect/Array.getByte(Ljava/lang/Object;I)B", Any)]
#[async_method]
pub async fn get_byte<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let index = parameters.pop_int()?;
    let Some(reference) = parameters.pop_reference()? else {
        return Err(NullPointerException(Some("array cannot be null".to_string())).into());
    };
    let guard = reference.read();
    let value = match &*guard {
        Reference::ByteArray(array) => {
            let v = get_array_element(array, index)?;
            Value::Int(i32::from(v))
        }
        Reference::BooleanArray(_)
        | Reference::CharArray(_)
        | Reference::ShortArray(_)
        | Reference::IntArray(_)
        | Reference::LongArray(_)
        | Reference::FloatArray(_)
        | Reference::DoubleArray(_)
        | Reference::Array(_) => return Err(illegal_argument("byte")),
        Reference::Object(_) => return Err(not_an_array()),
    };
    Ok(Some(value))
}

#[intrinsic_method("java/lang/reflect/Array.getChar(Ljava/lang/Object;I)C", Any)]
#[async_method]
pub async fn get_char<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let index = parameters.pop_int()?;
    let Some(reference) = parameters.pop_reference()? else {
        return Err(NullPointerException(Some("array cannot be null".to_string())).into());
    };
    let guard = reference.read();
    let value = match &*guard {
        Reference::CharArray(array) => {
            let v = get_array_element(array, index)?;
            Value::Int(i32::from(v))
        }
        Reference::BooleanArray(_)
        | Reference::ByteArray(_)
        | Reference::ShortArray(_)
        | Reference::IntArray(_)
        | Reference::LongArray(_)
        | Reference::FloatArray(_)
        | Reference::DoubleArray(_)
        | Reference::Array(_) => return Err(illegal_argument("char")),
        Reference::Object(_) => return Err(not_an_array()),
    };
    Ok(Some(value))
}

#[intrinsic_method("java/lang/reflect/Array.getDouble(Ljava/lang/Object;I)D", Any)]
#[async_method]
pub async fn get_double<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let index = parameters.pop_int()?;
    let Some(reference) = parameters.pop_reference()? else {
        return Err(NullPointerException(Some("array cannot be null".to_string())).into());
    };
    let guard = reference.read();
    let value = match &*guard {
        Reference::DoubleArray(array) => Value::Double(get_array_element(array, index)?),
        Reference::FloatArray(array) => Value::Double(f64::from(get_array_element(array, index)?)),
        Reference::LongArray(array) => {
            #[expect(clippy::cast_precision_loss)]
            let v = get_array_element(array, index)? as f64;
            Value::Double(v)
        }
        Reference::IntArray(array) => Value::Double(f64::from(get_array_element(array, index)?)),
        Reference::ShortArray(array) => Value::Double(f64::from(get_array_element(array, index)?)),
        Reference::CharArray(array) => Value::Double(f64::from(get_array_element(array, index)?)),
        Reference::ByteArray(array) => Value::Double(f64::from(get_array_element(array, index)?)),
        Reference::BooleanArray(_) | Reference::Array(_) => return Err(illegal_argument("double")),
        Reference::Object(_) => return Err(not_an_array()),
    };
    Ok(Some(value))
}

#[intrinsic_method("java/lang/reflect/Array.getFloat(Ljava/lang/Object;I)F", Any)]
#[async_method]
pub async fn get_float<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let index = parameters.pop_int()?;
    let Some(reference) = parameters.pop_reference()? else {
        return Err(NullPointerException(Some("array cannot be null".to_string())).into());
    };
    let guard = reference.read();
    let value = match &*guard {
        Reference::FloatArray(array) => Value::Float(get_array_element(array, index)?),
        Reference::LongArray(array) => {
            #[expect(clippy::cast_precision_loss)]
            let v = get_array_element(array, index)? as f32;
            Value::Float(v)
        }
        Reference::IntArray(array) => {
            #[expect(clippy::cast_precision_loss)]
            let v = get_array_element(array, index)? as f32;
            Value::Float(v)
        }
        Reference::ShortArray(array) => Value::Float(f32::from(get_array_element(array, index)?)),
        Reference::CharArray(array) => Value::Float(f32::from(get_array_element(array, index)?)),
        Reference::ByteArray(array) => Value::Float(f32::from(get_array_element(array, index)?)),
        Reference::BooleanArray(_) | Reference::DoubleArray(_) | Reference::Array(_) => {
            return Err(illegal_argument("float"));
        }
        Reference::Object(_) => return Err(not_an_array()),
    };
    Ok(Some(value))
}

#[intrinsic_method("java/lang/reflect/Array.getInt(Ljava/lang/Object;I)I", Any)]
#[async_method]
pub async fn get_int<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let index = parameters.pop_int()?;
    let Some(reference) = parameters.pop_reference()? else {
        return Err(NullPointerException(Some("array cannot be null".to_string())).into());
    };
    let guard = reference.read();
    let value = match &*guard {
        Reference::IntArray(array) => Value::Int(get_array_element(array, index)?),
        Reference::ShortArray(array) => Value::Int(i32::from(get_array_element(array, index)?)),
        Reference::CharArray(array) => Value::Int(i32::from(get_array_element(array, index)?)),
        Reference::ByteArray(array) => Value::Int(i32::from(get_array_element(array, index)?)),
        Reference::BooleanArray(_)
        | Reference::LongArray(_)
        | Reference::FloatArray(_)
        | Reference::DoubleArray(_)
        | Reference::Array(_) => return Err(illegal_argument("int")),
        Reference::Object(_) => return Err(not_an_array()),
    };
    Ok(Some(value))
}

#[intrinsic_method("java/lang/reflect/Array.getLength(Ljava/lang/Object;)I", Any)]
#[async_method]
pub async fn get_length<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let Some(array) = parameters.pop_reference()? else {
        return Err(NullPointerException(Some("array cannot be null".to_string())).into());
    };
    let guard = array.read();
    let length = match &*guard {
        Reference::BooleanArray(array) | Reference::ByteArray(array) => array.len(),
        Reference::CharArray(array) => array.len(),
        Reference::FloatArray(array) => array.len(),
        Reference::DoubleArray(array) => array.len(),
        Reference::ShortArray(array) => array.len(),
        Reference::IntArray(array) => array.len(),
        Reference::LongArray(array) => array.len(),
        Reference::Array(object_array) => object_array.elements.len(),
        Reference::Object(_) => {
            return Err(not_an_array());
        }
    };
    let length = i32::try_from(length)?;
    Ok(Some(Value::Int(length)))
}

#[intrinsic_method("java/lang/reflect/Array.getLong(Ljava/lang/Object;I)J", Any)]
#[async_method]
pub async fn get_long<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let index = parameters.pop_int()?;
    let Some(reference) = parameters.pop_reference()? else {
        return Err(NullPointerException(Some("array cannot be null".to_string())).into());
    };
    let guard = reference.read();
    let value = match &*guard {
        Reference::LongArray(array) => Value::Long(get_array_element(array, index)?),
        Reference::IntArray(array) => Value::Long(i64::from(get_array_element(array, index)?)),
        Reference::ShortArray(array) => Value::Long(i64::from(get_array_element(array, index)?)),
        Reference::CharArray(array) => Value::Long(i64::from(get_array_element(array, index)?)),
        Reference::ByteArray(array) => Value::Long(i64::from(get_array_element(array, index)?)),
        Reference::BooleanArray(_)
        | Reference::FloatArray(_)
        | Reference::DoubleArray(_)
        | Reference::Array(_) => return Err(illegal_argument("long")),
        Reference::Object(_) => return Err(not_an_array()),
    };
    Ok(Some(value))
}

#[intrinsic_method("java/lang/reflect/Array.getShort(Ljava/lang/Object;I)S", Any)]
#[async_method]
pub async fn get_short<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let index = parameters.pop_int()?;
    let Some(reference) = parameters.pop_reference()? else {
        return Err(NullPointerException(Some("array cannot be null".to_string())).into());
    };
    let guard = reference.read();
    let value = match &*guard {
        Reference::ShortArray(array) => Value::Int(i32::from(get_array_element(array, index)?)),
        Reference::ByteArray(array) => Value::Int(i32::from(get_array_element(array, index)?)),
        Reference::BooleanArray(_)
        | Reference::CharArray(_)
        | Reference::IntArray(_)
        | Reference::LongArray(_)
        | Reference::FloatArray(_)
        | Reference::DoubleArray(_)
        | Reference::Array(_) => return Err(illegal_argument("short")),
        Reference::Object(_) => return Err(not_an_array()),
    };
    Ok(Some(value))
}

#[intrinsic_method(
    "java/lang/reflect/Array.multiNewArray(Ljava/lang/Class;[I)Ljava/lang/Object;",
    Any
)]
#[async_method]
pub async fn multi_new_array<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let dimensions = parameters.pop()?;
    let dimensions = {
        let dimensions = dimensions.as_int_vec_ref()?;
        dimensions.to_vec()
    };
    let class_object = parameters.pop()?;
    let class = get_class(&thread, &class_object).await?;

    if dimensions.is_empty() {
        return Err(InvalidOperand {
            expected: "non-empty dimensions array".to_string(),
            actual: "empty dimensions array".to_string(),
        });
    }

    let class_name = class.name();
    let array = create_multi_dimensional_array(&thread, class_name, &dimensions).await?;
    let array = Value::new_object(thread.vm()?.garbage_collector(), array);
    Ok(Some(array))
}

#[async_method]
async fn create_multi_dimensional_array<T: Thread + 'static>(
    thread: &T,
    class_name: &str,
    dimensions: &[i32],
) -> Result<Reference> {
    if dimensions.is_empty() {
        return Err(InvalidOperand {
            expected: "non-empty dimensions".to_string(),
            actual: "empty dimensions".to_string(),
        });
    }

    let length = if dimensions[0] < 0 {
        return Err(NegativeArraySizeException(dimensions[0].to_string()).into());
    } else {
        #[expect(clippy::cast_sign_loss)]
        let len = dimensions[0] as usize;
        len
    };

    if dimensions.len() == 1 {
        // Base case: create a single-dimensional array
        let array = match class_name {
            "boolean" => Reference::from(vec![false; length]),
            "byte" => Reference::from(vec![0i8; length]),
            "char" => Reference::from(vec![0 as char; length]),
            "float" => Reference::from(vec![0.0f32; length]),
            "double" => Reference::from(vec![0.0f64; length]),
            "int" => Reference::from(vec![0i32; length]),
            "long" => Reference::from(vec![0i64; length]),
            "short" => Reference::from(vec![0i16; length]),
            _ => {
                let array_class_name = format!("[L{class_name};");
                let class = thread.class(&array_class_name).await?;
                let vm = thread.vm()?;
                let collector = &vm.garbage_collector();
                Reference::new_array(collector, class, vec![None; length])
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
                "boolean" => format!("{}{}", "[".repeat(dimensions.len()), "Z"),
                "byte" => format!("{}{}", "[".repeat(dimensions.len()), "B"),
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
        let vm = thread.vm()?;
        let collector = &vm.garbage_collector();
        Ok(Reference::new_array(collector, class, elements))
    }
}

#[intrinsic_method(
    "java/lang/reflect/Array.newArray(Ljava/lang/Class;I)Ljava/lang/Object;",
    Any
)]
#[async_method]
pub async fn new_array<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let length_i32 = parameters.pop_int()?;
    if length_i32 < 0 {
        return Err(NegativeArraySizeException(length_i32.to_string()).into());
    }
    #[expect(clippy::cast_sign_loss)]
    let length = length_i32 as usize;
    let class_name = get_class_name(&parameters.pop()?)?;

    let vm = thread.vm()?;
    let collector = &vm.garbage_collector();
    let array = match class_name.as_str() {
        "boolean" => Value::new_object(collector, Reference::from(vec![false; length])),
        "byte" => Value::new_object(collector, Reference::from(vec![0i8; length])),
        "char" => Value::new_object(collector, Reference::from(vec![0 as char; length])),
        "float" => Value::new_object(collector, Reference::from(vec![0.0f32; length])),
        "double" => Value::new_object(collector, Reference::from(vec![0.0f64; length])),
        "int" => Value::new_object(collector, Reference::from(vec![0i32; length])),
        "long" => Value::new_object(collector, Reference::from(vec![0i64; length])),
        "short" => Value::new_object(collector, Reference::from(vec![0i16; length])),
        _ => {
            let class_name = format!("[L{class_name};");
            let class = thread.class(&class_name).await?;
            let reference = Reference::try_from((class, vec![Value::Object(None); length]))?;
            Value::new_object(collector, reference)
        }
    };

    Ok(Some(array))
}

#[intrinsic_method(
    "java/lang/reflect/Array.set(Ljava/lang/Object;ILjava/lang/Object;)V",
    Any
)]
#[async_method]
pub async fn set<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value = parameters.pop()?;
    let index = parameters.pop_int()?;
    let Some(reference) = parameters.pop_reference()? else {
        return Err(NullPointerException(Some("array cannot be null".to_string())).into());
    };
    let mut guard = reference.write();
    match &mut *guard {
        Reference::Array(object_array) => {
            let Value::Object(value) = value else {
                return Err(InvalidOperand {
                    expected: "reference".to_string(),
                    actual: format!("{value:?}"),
                });
            };
            let array = &mut object_array.elements;
            set_array_element(array, index, Value::Object(value))?;
        }
        Reference::Object(_) => return Err(not_an_array()),
        array_ref => {
            let pv = unbox_value(&value)?;
            set_primitive_to_array(array_ref, index, &pv)?;
        }
    }
    Ok(None)
}

#[intrinsic_method("java/lang/reflect/Array.setBoolean(Ljava/lang/Object;IZ)V", Any)]
#[async_method]
pub async fn set_boolean<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value = i8::try_from(parameters.pop_int()?)?;
    let index = parameters.pop_int()?;
    let Some(reference) = parameters.pop_reference()? else {
        return Err(NullPointerException(Some("array cannot be null".to_string())).into());
    };
    let mut guard = reference.write();
    match &mut *guard {
        Reference::BooleanArray(array) => {
            set_array_element(array, index, value)?;
        }
        Reference::ByteArray(_)
        | Reference::CharArray(_)
        | Reference::ShortArray(_)
        | Reference::IntArray(_)
        | Reference::LongArray(_)
        | Reference::FloatArray(_)
        | Reference::DoubleArray(_)
        | Reference::Array(_) => return Err(illegal_argument("boolean")),
        Reference::Object(_) => return Err(not_an_array()),
    }
    Ok(None)
}

#[intrinsic_method("java/lang/reflect/Array.setByte(Ljava/lang/Object;IB)V", Any)]
#[async_method]
pub async fn set_byte<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value = i8::try_from(parameters.pop_int()?)?;
    let index = parameters.pop_int()?;
    let Some(reference) = parameters.pop_reference()? else {
        return Err(NullPointerException(Some("array cannot be null".to_string())).into());
    };
    let mut guard = reference.write();
    match &mut *guard {
        Reference::ByteArray(array) => set_array_element(array, index, value)?,
        Reference::ShortArray(array) => set_array_element(array, index, i16::from(value))?,
        Reference::IntArray(array) => set_array_element(array, index, i32::from(value))?,
        Reference::LongArray(array) => set_array_element(array, index, i64::from(value))?,
        Reference::FloatArray(array) => set_array_element(array, index, f32::from(value))?,
        Reference::DoubleArray(array) => set_array_element(array, index, f64::from(value))?,
        Reference::BooleanArray(_) | Reference::CharArray(_) | Reference::Array(_) => {
            return Err(illegal_argument("byte"));
        }
        Reference::Object(_) => return Err(not_an_array()),
    }
    Ok(None)
}

#[intrinsic_method("java/lang/reflect/Array.setChar(Ljava/lang/Object;IC)V", Any)]
#[async_method]
pub async fn set_char<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value = u16::try_from(parameters.pop_int()?)?;
    let index = parameters.pop_int()?;
    let Some(reference) = parameters.pop_reference()? else {
        return Err(NullPointerException(Some("array cannot be null".to_string())).into());
    };
    let mut guard = reference.write();
    match &mut *guard {
        Reference::CharArray(array) => set_array_element(array, index, value)?,
        Reference::IntArray(array) => set_array_element(array, index, i32::from(value))?,
        Reference::LongArray(array) => set_array_element(array, index, i64::from(value))?,
        Reference::FloatArray(array) => set_array_element(array, index, f32::from(value))?,
        Reference::DoubleArray(array) => set_array_element(array, index, f64::from(value))?,
        Reference::BooleanArray(_)
        | Reference::ByteArray(_)
        | Reference::ShortArray(_)
        | Reference::Array(_) => return Err(illegal_argument("char")),
        Reference::Object(_) => return Err(not_an_array()),
    }
    Ok(None)
}

#[intrinsic_method("java/lang/reflect/Array.setDouble(Ljava/lang/Object;ID)V", Any)]
#[async_method]
pub async fn set_double<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value = parameters.pop_double()?;
    let index = parameters.pop_int()?;
    let Some(reference) = parameters.pop_reference()? else {
        return Err(NullPointerException(Some("array cannot be null".to_string())).into());
    };
    let mut guard = reference.write();
    match &mut *guard {
        Reference::DoubleArray(array) => set_array_element(array, index, value)?,
        Reference::BooleanArray(_)
        | Reference::ByteArray(_)
        | Reference::CharArray(_)
        | Reference::ShortArray(_)
        | Reference::IntArray(_)
        | Reference::LongArray(_)
        | Reference::FloatArray(_)
        | Reference::Array(_) => return Err(illegal_argument("double")),
        Reference::Object(_) => return Err(not_an_array()),
    }
    Ok(None)
}

#[intrinsic_method("java/lang/reflect/Array.setFloat(Ljava/lang/Object;IF)V", Any)]
#[async_method]
pub async fn set_float<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value = parameters.pop_float()?;
    let index = parameters.pop_int()?;
    let Some(reference) = parameters.pop_reference()? else {
        return Err(NullPointerException(Some("array cannot be null".to_string())).into());
    };
    let mut guard = reference.write();
    match &mut *guard {
        Reference::FloatArray(array) => set_array_element(array, index, value)?,
        Reference::DoubleArray(array) => set_array_element(array, index, f64::from(value))?,
        Reference::BooleanArray(_)
        | Reference::ByteArray(_)
        | Reference::CharArray(_)
        | Reference::ShortArray(_)
        | Reference::IntArray(_)
        | Reference::LongArray(_)
        | Reference::Array(_) => return Err(illegal_argument("float")),
        Reference::Object(_) => return Err(not_an_array()),
    }
    Ok(None)
}

#[intrinsic_method("java/lang/reflect/Array.setInt(Ljava/lang/Object;II)V", Any)]
#[async_method]
pub async fn set_int<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value = parameters.pop_int()?;
    let index = parameters.pop_int()?;
    let Some(reference) = parameters.pop_reference()? else {
        return Err(NullPointerException(Some("array cannot be null".to_string())).into());
    };
    let mut guard = reference.write();
    match &mut *guard {
        Reference::IntArray(array) => set_array_element(array, index, value)?,
        Reference::LongArray(array) => set_array_element(array, index, i64::from(value))?,
        Reference::FloatArray(array) => {
            #[expect(clippy::cast_precision_loss)]
            let v = value as f32;
            set_array_element(array, index, v)?;
        }
        Reference::DoubleArray(array) => set_array_element(array, index, f64::from(value))?,
        Reference::BooleanArray(_)
        | Reference::ByteArray(_)
        | Reference::CharArray(_)
        | Reference::ShortArray(_)
        | Reference::Array(_) => return Err(illegal_argument("int")),
        Reference::Object(_) => return Err(not_an_array()),
    }
    Ok(None)
}

#[intrinsic_method("java/lang/reflect/Array.setLong(Ljava/lang/Object;IJ)V", Any)]
#[async_method]
pub async fn set_long<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value = parameters.pop_long()?;
    let index = parameters.pop_int()?;
    let Some(reference) = parameters.pop_reference()? else {
        return Err(NullPointerException(Some("array cannot be null".to_string())).into());
    };
    let mut guard = reference.write();
    match &mut *guard {
        Reference::LongArray(array) => set_array_element(array, index, value)?,
        Reference::FloatArray(array) => {
            #[expect(clippy::cast_precision_loss)]
            let v = value as f32;
            set_array_element(array, index, v)?;
        }
        Reference::DoubleArray(array) => {
            #[expect(clippy::cast_precision_loss)]
            let v = value as f64;
            set_array_element(array, index, v)?;
        }
        Reference::BooleanArray(_)
        | Reference::ByteArray(_)
        | Reference::CharArray(_)
        | Reference::ShortArray(_)
        | Reference::IntArray(_)
        | Reference::Array(_) => return Err(illegal_argument("long")),
        Reference::Object(_) => return Err(not_an_array()),
    }
    Ok(None)
}

#[intrinsic_method("java/lang/reflect/Array.setShort(Ljava/lang/Object;IS)V", Any)]
#[async_method]
pub async fn set_short<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value = i16::try_from(parameters.pop_int()?)?;
    let index = parameters.pop_int()?;
    let Some(reference) = parameters.pop_reference()? else {
        return Err(NullPointerException(Some("array cannot be null".to_string())).into());
    };
    let mut guard = reference.write();
    match &mut *guard {
        Reference::ShortArray(array) => set_array_element(array, index, value)?,
        Reference::IntArray(array) => set_array_element(array, index, i32::from(value))?,
        Reference::LongArray(array) => set_array_element(array, index, i64::from(value))?,
        Reference::FloatArray(array) => set_array_element(array, index, f32::from(value))?,
        Reference::DoubleArray(array) => set_array_element(array, index, f64::from(value))?,
        Reference::BooleanArray(_)
        | Reference::ByteArray(_)
        | Reference::CharArray(_)
        | Reference::Array(_) => return Err(illegal_argument("short")),
        Reference::Object(_) => return Err(not_an_array()),
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_types::JavaObject;

    #[tokio::test]
    async fn test_get_and_set() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0i32]),
        );
        let index = Value::Int(0);
        let expected = 42i32;
        let parameters = Parameters::new(vec![array.clone(), index.clone(), Value::from(expected)]);
        let _ = set(thread.clone(), parameters).await?;
        let parameters = Parameters::new(vec![array, index]);
        let result = get(thread, parameters).await?.expect("value");
        let value = result.as_i32()?;
        assert_eq!(value, expected);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_and_set_boolean() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![false]),
        );
        let index = Value::Int(0);
        let expected = true;
        let parameters = Parameters::new(vec![array.clone(), index.clone(), Value::from(expected)]);
        let _ = set_boolean(thread.clone(), parameters).await?;
        let parameters = Parameters::new(vec![array, index]);
        let result = get_boolean(thread, parameters).await?.expect("value");
        let value = result.as_bool()?;
        assert_eq!(value, expected);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_and_set_byte() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let array = Value::new_object(thread.vm()?.garbage_collector(), Reference::from(vec![0i8]));
        let index = Value::Int(0);
        let expected = 42i8;
        let parameters = Parameters::new(vec![array.clone(), index.clone(), Value::from(expected)]);
        let _ = set_byte(thread.clone(), parameters).await?;
        let parameters = Parameters::new(vec![array, index]);
        let result = get_byte(thread, parameters).await?.expect("value");
        let value = result.as_i8()?;
        assert_eq!(value, expected);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_and_set_char() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0 as char]),
        );
        let index = Value::Int(0);
        let expected = 42u16;
        let parameters = Parameters::new(vec![array.clone(), index.clone(), Value::from(expected)]);
        let _ = set_char(thread.clone(), parameters).await?;
        let parameters = Parameters::new(vec![array, index]);
        let result = get_char(thread, parameters).await?.expect("value");
        let value = result.as_u16()?;
        assert_eq!(value, expected);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_and_set_double() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0.0f64]),
        );
        let index = Value::Int(0);
        let expected = 42.0f64;
        let parameters = Parameters::new(vec![array.clone(), index.clone(), Value::from(expected)]);
        let _ = set_double(thread.clone(), parameters).await?;
        let parameters = Parameters::new(vec![array, index]);
        let result = get_double(thread, parameters).await?.expect("value");
        let value = result.as_f64()?;
        let value = value - expected;
        assert!(value.abs() < f64::EPSILON);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_and_set_float() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0f32]),
        );
        let index = Value::Int(0);
        let expected = 42f32;
        let parameters = Parameters::new(vec![array.clone(), index.clone(), Value::from(expected)]);
        let _ = set_float(thread.clone(), parameters).await?;
        let parameters = Parameters::new(vec![array, index]);
        let result = get_float(thread, parameters).await?.expect("value");
        let value = result.as_f32()?;
        let value = value - expected;
        assert!(value.abs() < f32::EPSILON);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_and_set_int() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0i32]),
        );
        let index = Value::Int(0);
        let expected = 42i32;
        let parameters = Parameters::new(vec![array.clone(), index.clone(), Value::from(expected)]);
        let _ = set_int(thread.clone(), parameters).await?;
        let parameters = Parameters::new(vec![array, index]);
        let result = get_int(thread, parameters).await?.expect("value");
        let value = result.as_i32()?;
        assert_eq!(value, expected);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_length() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![1, 2, 3]),
        );
        let parameters = Parameters::new(vec![array]);
        let result = get_length(thread, parameters).await?.expect("Array length");
        let length = result.as_i32()?;
        assert_eq!(length, 3);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_and_set_long() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0i64]),
        );
        let index = Value::Int(0);
        let expected = 42i64;
        let parameters = Parameters::new(vec![array.clone(), index.clone(), Value::from(expected)]);
        let _ = set_long(thread.clone(), parameters).await?;
        let parameters = Parameters::new(vec![array, index]);
        let result = get_long(thread, parameters).await?.expect("value");
        let value = result.as_i64()?;
        assert_eq!(value, expected);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_and_set_short() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0i16]),
        );
        let index = Value::Int(0);
        let expected = 42i16;
        let parameters = Parameters::new(vec![array.clone(), index.clone(), Value::from(expected)]);
        let _ = set_short(thread.clone(), parameters).await?;
        let parameters = Parameters::new(vec![array, index]);
        let result = get_short(thread, parameters).await?.expect("value");
        let value = result.as_i16()?;
        assert_eq!(value, expected);
        Ok(())
    }

    #[tokio::test]
    async fn test_multi_new_array_null_class() -> Result<()> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let dimensions = Value::new_object(vm.garbage_collector(), Reference::from(vec![1]));
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
        let dimensions = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(Vec::<i32>::new()),
        );
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
        let dimensions = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![5i32]),
        );
        let parameters = Parameters::new(vec![class_object, dimensions]);

        let array = multi_new_array(thread.clone(), parameters)
            .await?
            .expect("array");
        let array = array.as_int_vec_ref()?;
        assert_eq!(*array, [0; 5]);
        Ok(())
    }

    #[tokio::test]
    async fn test_multi_new_array_two_dimensions() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let class = thread.class("int").await?;
        let class_object = class.to_object(&thread).await?;
        let dimensions = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![3i32, 4i32]),
        );
        let parameters = Parameters::new(vec![class_object, dimensions]);

        let array = multi_new_array(thread.clone(), parameters)
            .await?
            .expect("array");
        let reference = array.as_reference()?;
        let (_outer_class, outer_array) = reference.as_class_vec_ref()?;
        assert_eq!(outer_array.len(), 3);
        for i in 0..3 {
            let value = outer_array.get(i).expect("inner array");
            let inner_array = value.as_int_vec_ref()?;
            assert_eq!(inner_array.len(), 4);
            for j in 0..4 {
                assert_eq!(inner_array.get(j), Some(&0i32));
            }
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_new_array_boolean() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class = thread.class("boolean").await?;
        let class_object = class.to_object(&thread).await?;
        let params = Parameters::new(vec![class_object, Value::Int(3)]);
        let result = new_array(thread.clone(), params).await?.expect("array");
        let len_params = Parameters::new(vec![result]);
        let len = get_length(thread, len_params).await?.expect("length");
        assert_eq!(len.as_i32()?, 3);
        Ok(())
    }

    #[tokio::test]
    async fn test_new_array_byte() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class = thread.class("byte").await?;
        let class_object = class.to_object(&thread).await?;
        let params = Parameters::new(vec![class_object, Value::Int(2)]);
        let result = new_array(thread.clone(), params).await?.expect("array");
        let len_params = Parameters::new(vec![result]);
        let len = get_length(thread, len_params).await?.expect("length");
        assert_eq!(len.as_i32()?, 2);
        Ok(())
    }

    #[tokio::test]
    async fn test_new_array_char() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class = thread.class("char").await?;
        let class_object = class.to_object(&thread).await?;
        let params = Parameters::new(vec![class_object, Value::Int(4)]);
        let result = new_array(thread.clone(), params).await?.expect("array");
        let len_params = Parameters::new(vec![result]);
        let len = get_length(thread, len_params).await?.expect("length");
        assert_eq!(len.as_i32()?, 4);
        Ok(())
    }

    #[tokio::test]
    async fn test_new_array_short() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class = thread.class("short").await?;
        let class_object = class.to_object(&thread).await?;
        let params = Parameters::new(vec![class_object, Value::Int(5)]);
        let result = new_array(thread.clone(), params).await?.expect("array");
        let len_params = Parameters::new(vec![result]);
        let len = get_length(thread, len_params).await?.expect("length");
        assert_eq!(len.as_i32()?, 5);
        Ok(())
    }

    #[tokio::test]
    async fn test_new_array_int() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class = thread.class("int").await?;
        let class_object = class.to_object(&thread).await?;
        let params = Parameters::new(vec![class_object, Value::Int(6)]);
        let result = new_array(thread.clone(), params).await?.expect("array");
        let len_params = Parameters::new(vec![result]);
        let len = get_length(thread, len_params).await?.expect("length");
        assert_eq!(len.as_i32()?, 6);
        Ok(())
    }

    #[tokio::test]
    async fn test_new_array_long() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class = thread.class("long").await?;
        let class_object = class.to_object(&thread).await?;
        let params = Parameters::new(vec![class_object, Value::Int(7)]);
        let result = new_array(thread.clone(), params).await?.expect("array");
        let len_params = Parameters::new(vec![result]);
        let len = get_length(thread, len_params).await?.expect("length");
        assert_eq!(len.as_i32()?, 7);
        Ok(())
    }

    #[tokio::test]
    async fn test_new_array_float() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class = thread.class("float").await?;
        let class_object = class.to_object(&thread).await?;
        let params = Parameters::new(vec![class_object, Value::Int(8)]);
        let result = new_array(thread.clone(), params).await?.expect("array");
        let len_params = Parameters::new(vec![result]);
        let len = get_length(thread, len_params).await?.expect("length");
        assert_eq!(len.as_i32()?, 8);
        Ok(())
    }

    #[tokio::test]
    async fn test_new_array_double() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class = thread.class("double").await?;
        let class_object = class.to_object(&thread).await?;
        let params = Parameters::new(vec![class_object, Value::Int(9)]);
        let result = new_array(thread.clone(), params).await?.expect("array");
        let len_params = Parameters::new(vec![result]);
        let len = get_length(thread, len_params).await?.expect("length");
        assert_eq!(len.as_i32()?, 9);
        Ok(())
    }

    #[tokio::test]
    async fn test_new_array_zero_length() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class = thread.class("int").await?;
        let class_object = class.to_object(&thread).await?;
        let params = Parameters::new(vec![class_object, Value::Int(0)]);
        let result = new_array(thread.clone(), params).await?.expect("array");
        let len_params = Parameters::new(vec![result]);
        let len = get_length(thread, len_params).await?.expect("length");
        assert_eq!(len.as_i32()?, 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_new_array_negative_size() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class = thread.class("int").await?;
        let class_object = class.to_object(&thread).await?;
        let params = Parameters::new(vec![class_object, Value::Int(-1)]);
        let result = new_array(thread, params).await;
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_new_array_null_class() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let params = Parameters::new(vec![Value::Object(None), Value::Int(5)]);
        let result = new_array(thread, params).await;
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_null_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let params = Parameters::new(vec![Value::Object(None), Value::Int(0)]);
        assert!(get(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_boolean_null_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let params = Parameters::new(vec![Value::Object(None), Value::Int(0)]);
        assert!(get_boolean(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_byte_null_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let params = Parameters::new(vec![Value::Object(None), Value::Int(0)]);
        assert!(get_byte(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_char_null_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let params = Parameters::new(vec![Value::Object(None), Value::Int(0)]);
        assert!(get_char(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_double_null_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let params = Parameters::new(vec![Value::Object(None), Value::Int(0)]);
        assert!(get_double(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_float_null_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let params = Parameters::new(vec![Value::Object(None), Value::Int(0)]);
        assert!(get_float(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_int_null_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let params = Parameters::new(vec![Value::Object(None), Value::Int(0)]);
        assert!(get_int(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_long_null_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let params = Parameters::new(vec![Value::Object(None), Value::Int(0)]);
        assert!(get_long(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_short_null_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let params = Parameters::new(vec![Value::Object(None), Value::Int(0)]);
        assert!(get_short(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_length_null_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let params = Parameters::new(vec![Value::Object(None)]);
        assert!(get_length(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_set_null_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let params = Parameters::new(vec![Value::Object(None), Value::Int(0), Value::Int(1)]);
        assert!(set(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_set_boolean_null_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let params = Parameters::new(vec![Value::Object(None), Value::Int(0), Value::Int(1)]);
        assert!(set_boolean(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_set_byte_null_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let params = Parameters::new(vec![Value::Object(None), Value::Int(0), Value::Int(1)]);
        assert!(set_byte(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_set_char_null_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let params = Parameters::new(vec![Value::Object(None), Value::Int(0), Value::Int(1)]);
        assert!(set_char(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_set_double_null_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let params = Parameters::new(vec![Value::Object(None), Value::Int(0), Value::Double(1.0)]);
        assert!(set_double(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_set_float_null_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let params = Parameters::new(vec![Value::Object(None), Value::Int(0), Value::Float(1.0)]);
        assert!(set_float(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_set_int_null_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let params = Parameters::new(vec![Value::Object(None), Value::Int(0), Value::Int(1)]);
        assert!(set_int(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_set_long_null_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let params = Parameters::new(vec![Value::Object(None), Value::Int(0), Value::Long(1)]);
        assert!(set_long(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_set_short_null_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let params = Parameters::new(vec![Value::Object(None), Value::Int(0), Value::Int(1)]);
        assert!(set_short(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_boolean_index_out_of_bounds() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![false]),
        );
        let params = Parameters::new(vec![array, Value::Int(5)]);
        assert!(get_boolean(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_byte_index_out_of_bounds() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(thread.vm()?.garbage_collector(), Reference::from(vec![0i8]));
        let params = Parameters::new(vec![array, Value::Int(5)]);
        assert!(get_byte(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_char_index_out_of_bounds() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0 as char]),
        );
        let params = Parameters::new(vec![array, Value::Int(5)]);
        assert!(get_char(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_double_index_out_of_bounds() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0.0f64]),
        );
        let params = Parameters::new(vec![array, Value::Int(5)]);
        assert!(get_double(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_float_index_out_of_bounds() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0f32]),
        );
        let params = Parameters::new(vec![array, Value::Int(5)]);
        assert!(get_float(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_int_index_out_of_bounds() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0i32]),
        );
        let params = Parameters::new(vec![array, Value::Int(5)]);
        assert!(get_int(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_long_index_out_of_bounds() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0i64]),
        );
        let params = Parameters::new(vec![array, Value::Int(5)]);
        assert!(get_long(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_short_index_out_of_bounds() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0i16]),
        );
        let params = Parameters::new(vec![array, Value::Int(5)]);
        assert!(get_short(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_int_negative_index() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0i32]),
        );
        let params = Parameters::new(vec![array, Value::Int(-1)]);
        assert!(get_int(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_set_int_negative_index() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0i32]),
        );
        let params = Parameters::new(vec![array, Value::Int(-1), Value::Int(1)]);
        assert!(set_int(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_set_boolean_index_out_of_bounds() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![false]),
        );
        let params = Parameters::new(vec![array, Value::Int(5), Value::Int(1)]);
        assert!(set_boolean(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_set_byte_index_out_of_bounds() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(thread.vm()?.garbage_collector(), Reference::from(vec![0i8]));
        let params = Parameters::new(vec![array, Value::Int(5), Value::Int(1)]);
        assert!(set_byte(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_set_char_index_out_of_bounds() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0 as char]),
        );
        let params = Parameters::new(vec![array, Value::Int(5), Value::Int(1)]);
        assert!(set_char(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_set_double_index_out_of_bounds() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0.0f64]),
        );
        let params = Parameters::new(vec![array, Value::Int(5), Value::Double(1.0)]);
        assert!(set_double(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_set_float_index_out_of_bounds() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0f32]),
        );
        let params = Parameters::new(vec![array, Value::Int(5), Value::Float(1.0)]);
        assert!(set_float(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_set_int_index_out_of_bounds() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0i32]),
        );
        let params = Parameters::new(vec![array, Value::Int(5), Value::Int(1)]);
        assert!(set_int(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_set_long_index_out_of_bounds() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0i64]),
        );
        let params = Parameters::new(vec![array, Value::Int(5), Value::Long(1)]);
        assert!(set_long(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_set_short_index_out_of_bounds() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0i16]),
        );
        let params = Parameters::new(vec![array, Value::Int(5), Value::Int(1)]);
        assert!(set_short(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_boolean_type_mismatch() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0i32]),
        );
        let params = Parameters::new(vec![array, Value::Int(0)]);
        assert!(get_boolean(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_byte_type_mismatch() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![false]),
        );
        let params = Parameters::new(vec![array, Value::Int(0)]);
        assert!(get_byte(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_char_type_mismatch() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![false]),
        );
        let params = Parameters::new(vec![array, Value::Int(0)]);
        assert!(get_char(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_short_type_mismatch() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![false]),
        );
        let params = Parameters::new(vec![array, Value::Int(0)]);
        assert!(get_short(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_int_type_mismatch() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![false]),
        );
        let params = Parameters::new(vec![array, Value::Int(0)]);
        assert!(get_int(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_long_type_mismatch() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![false]),
        );
        let params = Parameters::new(vec![array, Value::Int(0)]);
        assert!(get_long(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_float_type_mismatch() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![false]),
        );
        let params = Parameters::new(vec![array, Value::Int(0)]);
        assert!(get_float(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_double_type_mismatch() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![false]),
        );
        let params = Parameters::new(vec![array, Value::Int(0)]);
        assert!(get_double(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_set_boolean_type_mismatch() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0i32]),
        );
        let params = Parameters::new(vec![array, Value::Int(0), Value::Int(1)]);
        assert!(set_boolean(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_set_byte_type_mismatch() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![false]),
        );
        let params = Parameters::new(vec![array, Value::Int(0), Value::Int(1)]);
        assert!(set_byte(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_set_char_type_mismatch() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![false]),
        );
        let params = Parameters::new(vec![array, Value::Int(0), Value::Int(1)]);
        assert!(set_char(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_set_short_type_mismatch() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![false]),
        );
        let params = Parameters::new(vec![array, Value::Int(0), Value::Int(1)]);
        assert!(set_short(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_set_int_type_mismatch() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![false]),
        );
        let params = Parameters::new(vec![array, Value::Int(0), Value::Int(1)]);
        assert!(set_int(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_set_long_type_mismatch() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![false]),
        );
        let params = Parameters::new(vec![array, Value::Int(0), Value::Long(1)]);
        assert!(set_long(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_set_float_type_mismatch() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![false]),
        );
        let params = Parameters::new(vec![array, Value::Int(0), Value::Float(1.0)]);
        assert!(set_float(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_set_double_type_mismatch() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![false]),
        );
        let params = Parameters::new(vec![array, Value::Int(0), Value::Double(1.0)]);
        assert!(set_double(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_short_from_byte_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![42i8]),
        );
        let params = Parameters::new(vec![array, Value::Int(0)]);
        let result = get_short(thread, params).await?.expect("value");
        assert_eq!(result.as_i16()?, 42);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_int_from_byte_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![42i8]),
        );
        let params = Parameters::new(vec![array, Value::Int(0)]);
        let result = get_int(thread, params).await?.expect("value");
        assert_eq!(result.as_i32()?, 42);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_int_from_short_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![42i16]),
        );
        let params = Parameters::new(vec![array, Value::Int(0)]);
        let result = get_int(thread, params).await?.expect("value");
        assert_eq!(result.as_i32()?, 42);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_int_from_char_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![42 as char]),
        );
        let params = Parameters::new(vec![array, Value::Int(0)]);
        let result = get_int(thread, params).await?.expect("value");
        assert_eq!(result.as_i32()?, 42);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_long_from_byte_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![42i8]),
        );
        let params = Parameters::new(vec![array, Value::Int(0)]);
        let result = get_long(thread, params).await?.expect("value");
        assert_eq!(result.as_i64()?, 42);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_long_from_short_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![42i16]),
        );
        let params = Parameters::new(vec![array, Value::Int(0)]);
        let result = get_long(thread, params).await?.expect("value");
        assert_eq!(result.as_i64()?, 42);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_long_from_char_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![42 as char]),
        );
        let params = Parameters::new(vec![array, Value::Int(0)]);
        let result = get_long(thread, params).await?.expect("value");
        assert_eq!(result.as_i64()?, 42);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_long_from_int_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![42i32]),
        );
        let params = Parameters::new(vec![array, Value::Int(0)]);
        let result = get_long(thread, params).await?.expect("value");
        assert_eq!(result.as_i64()?, 42);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_float_from_byte_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![42i8]),
        );
        let params = Parameters::new(vec![array, Value::Int(0)]);
        let result = get_float(thread, params).await?.expect("value");
        let diff = result.as_f32()? - 42.0;
        assert!(diff.abs() < f32::EPSILON);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_float_from_short_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![42i16]),
        );
        let params = Parameters::new(vec![array, Value::Int(0)]);
        let result = get_float(thread, params).await?.expect("value");
        let diff = result.as_f32()? - 42.0;
        assert!(diff.abs() < f32::EPSILON);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_float_from_char_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![42 as char]),
        );
        let params = Parameters::new(vec![array, Value::Int(0)]);
        let result = get_float(thread, params).await?.expect("value");
        let diff = result.as_f32()? - 42.0;
        assert!(diff.abs() < f32::EPSILON);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_float_from_int_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![42i32]),
        );
        let params = Parameters::new(vec![array, Value::Int(0)]);
        let result = get_float(thread, params).await?.expect("value");
        let diff = result.as_f32()? - 42.0;
        assert!(diff.abs() < f32::EPSILON);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_float_from_long_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![42i64]),
        );
        let params = Parameters::new(vec![array, Value::Int(0)]);
        let result = get_float(thread, params).await?.expect("value");
        let diff = result.as_f32()? - 42.0;
        assert!(diff.abs() < f32::EPSILON);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_double_from_byte_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![42i8]),
        );
        let params = Parameters::new(vec![array, Value::Int(0)]);
        let result = get_double(thread, params).await?.expect("value");
        let diff = result.as_f64()? - 42.0;
        assert!(diff.abs() < f64::EPSILON);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_double_from_short_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![42i16]),
        );
        let params = Parameters::new(vec![array, Value::Int(0)]);
        let result = get_double(thread, params).await?.expect("value");
        let diff = result.as_f64()? - 42.0;
        assert!(diff.abs() < f64::EPSILON);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_double_from_char_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![42 as char]),
        );
        let params = Parameters::new(vec![array, Value::Int(0)]);
        let result = get_double(thread, params).await?.expect("value");
        let diff = result.as_f64()? - 42.0;
        assert!(diff.abs() < f64::EPSILON);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_double_from_int_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![42i32]),
        );
        let params = Parameters::new(vec![array, Value::Int(0)]);
        let result = get_double(thread, params).await?.expect("value");
        let diff = result.as_f64()? - 42.0;
        assert!(diff.abs() < f64::EPSILON);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_double_from_long_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![42i64]),
        );
        let params = Parameters::new(vec![array, Value::Int(0)]);
        let result = get_double(thread, params).await?.expect("value");
        let diff = result.as_f64()? - 42.0;
        assert!(diff.abs() < f64::EPSILON);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_double_from_float_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![42.0f32]),
        );
        let params = Parameters::new(vec![array, Value::Int(0)]);
        let result = get_double(thread, params).await?.expect("value");
        let diff = result.as_f64()? - 42.0;
        assert!(diff.abs() < f64::EPSILON);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_byte_into_short_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0i16]),
        );
        let params = Parameters::new(vec![array.clone(), Value::Int(0), Value::Int(42)]);
        set_byte(thread.clone(), params).await?;
        let params = Parameters::new(vec![array, Value::Int(0)]);
        let result = get_short(thread, params).await?.expect("value");
        assert_eq!(result.as_i16()?, 42);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_byte_into_int_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0i32]),
        );
        let params = Parameters::new(vec![array.clone(), Value::Int(0), Value::Int(42)]);
        set_byte(thread.clone(), params).await?;
        let params = Parameters::new(vec![array, Value::Int(0)]);
        let result = get_int(thread, params).await?.expect("value");
        assert_eq!(result.as_i32()?, 42);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_byte_into_long_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0i64]),
        );
        let params = Parameters::new(vec![array.clone(), Value::Int(0), Value::Int(42)]);
        set_byte(thread.clone(), params).await?;
        let params = Parameters::new(vec![array, Value::Int(0)]);
        let result = get_long(thread, params).await?.expect("value");
        assert_eq!(result.as_i64()?, 42);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_byte_into_float_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0f32]),
        );
        let params = Parameters::new(vec![array.clone(), Value::Int(0), Value::Int(42)]);
        set_byte(thread.clone(), params).await?;
        let params = Parameters::new(vec![array, Value::Int(0)]);
        let result = get_float(thread, params).await?.expect("value");
        let diff = result.as_f32()? - 42.0;
        assert!(diff.abs() < f32::EPSILON);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_byte_into_double_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0.0f64]),
        );
        let params = Parameters::new(vec![array.clone(), Value::Int(0), Value::Int(42)]);
        set_byte(thread.clone(), params).await?;
        let params = Parameters::new(vec![array, Value::Int(0)]);
        let result = get_double(thread, params).await?.expect("value");
        let diff = result.as_f64()? - 42.0;
        assert!(diff.abs() < f64::EPSILON);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_char_into_int_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0i32]),
        );
        let params = Parameters::new(vec![array.clone(), Value::Int(0), Value::Int(65)]);
        set_char(thread.clone(), params).await?;
        let params = Parameters::new(vec![array, Value::Int(0)]);
        let result = get_int(thread, params).await?.expect("value");
        assert_eq!(result.as_i32()?, 65);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_char_into_long_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0i64]),
        );
        let params = Parameters::new(vec![array.clone(), Value::Int(0), Value::Int(65)]);
        set_char(thread.clone(), params).await?;
        let params = Parameters::new(vec![array, Value::Int(0)]);
        let result = get_long(thread, params).await?.expect("value");
        assert_eq!(result.as_i64()?, 65);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_char_into_float_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0f32]),
        );
        let params = Parameters::new(vec![array.clone(), Value::Int(0), Value::Int(65)]);
        set_char(thread.clone(), params).await?;
        let params = Parameters::new(vec![array, Value::Int(0)]);
        let result = get_float(thread, params).await?.expect("value");
        let diff = result.as_f32()? - 65.0;
        assert!(diff.abs() < f32::EPSILON);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_char_into_double_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0.0f64]),
        );
        let params = Parameters::new(vec![array.clone(), Value::Int(0), Value::Int(65)]);
        set_char(thread.clone(), params).await?;
        let params = Parameters::new(vec![array, Value::Int(0)]);
        let result = get_double(thread, params).await?.expect("value");
        let diff = result.as_f64()? - 65.0;
        assert!(diff.abs() < f64::EPSILON);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_short_into_int_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0i32]),
        );
        let params = Parameters::new(vec![array.clone(), Value::Int(0), Value::Int(42)]);
        set_short(thread.clone(), params).await?;
        let params = Parameters::new(vec![array, Value::Int(0)]);
        let result = get_int(thread, params).await?.expect("value");
        assert_eq!(result.as_i32()?, 42);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_short_into_long_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0i64]),
        );
        let params = Parameters::new(vec![array.clone(), Value::Int(0), Value::Int(42)]);
        set_short(thread.clone(), params).await?;
        let params = Parameters::new(vec![array, Value::Int(0)]);
        let result = get_long(thread, params).await?.expect("value");
        assert_eq!(result.as_i64()?, 42);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_short_into_float_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0f32]),
        );
        let params = Parameters::new(vec![array.clone(), Value::Int(0), Value::Int(42)]);
        set_short(thread.clone(), params).await?;
        let params = Parameters::new(vec![array, Value::Int(0)]);
        let result = get_float(thread, params).await?.expect("value");
        let diff = result.as_f32()? - 42.0;
        assert!(diff.abs() < f32::EPSILON);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_short_into_double_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0.0f64]),
        );
        let params = Parameters::new(vec![array.clone(), Value::Int(0), Value::Int(42)]);
        set_short(thread.clone(), params).await?;
        let params = Parameters::new(vec![array, Value::Int(0)]);
        let result = get_double(thread, params).await?.expect("value");
        let diff = result.as_f64()? - 42.0;
        assert!(diff.abs() < f64::EPSILON);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_int_into_long_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0i64]),
        );
        let params = Parameters::new(vec![array.clone(), Value::Int(0), Value::Int(42)]);
        set_int(thread.clone(), params).await?;
        let params = Parameters::new(vec![array, Value::Int(0)]);
        let result = get_long(thread, params).await?.expect("value");
        assert_eq!(result.as_i64()?, 42);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_int_into_float_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0f32]),
        );
        let params = Parameters::new(vec![array.clone(), Value::Int(0), Value::Int(42)]);
        set_int(thread.clone(), params).await?;
        let params = Parameters::new(vec![array, Value::Int(0)]);
        let result = get_float(thread, params).await?.expect("value");
        let diff = result.as_f32()? - 42.0;
        assert!(diff.abs() < f32::EPSILON);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_int_into_double_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0.0f64]),
        );
        let params = Parameters::new(vec![array.clone(), Value::Int(0), Value::Int(42)]);
        set_int(thread.clone(), params).await?;
        let params = Parameters::new(vec![array, Value::Int(0)]);
        let result = get_double(thread, params).await?.expect("value");
        let diff = result.as_f64()? - 42.0;
        assert!(diff.abs() < f64::EPSILON);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_long_into_float_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0f32]),
        );
        let params = Parameters::new(vec![array.clone(), Value::Int(0), Value::Long(42)]);
        set_long(thread.clone(), params).await?;
        let params = Parameters::new(vec![array, Value::Int(0)]);
        let result = get_float(thread, params).await?.expect("value");
        let diff = result.as_f32()? - 42.0;
        assert!(diff.abs() < f32::EPSILON);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_long_into_double_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0.0f64]),
        );
        let params = Parameters::new(vec![array.clone(), Value::Int(0), Value::Long(42)]);
        set_long(thread.clone(), params).await?;
        let params = Parameters::new(vec![array, Value::Int(0)]);
        let result = get_double(thread, params).await?.expect("value");
        let diff = result.as_f64()? - 42.0;
        assert!(diff.abs() < f64::EPSILON);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_float_into_double_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0.0f64]),
        );
        let params = Parameters::new(vec![array.clone(), Value::Int(0), Value::Float(42.0)]);
        set_float(thread.clone(), params).await?;
        let params = Parameters::new(vec![array, Value::Int(0)]);
        let result = get_double(thread, params).await?.expect("value");
        let diff = result.as_f64()? - 42.0;
        assert!(diff.abs() < f64::EPSILON);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_length_boolean_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![false, true]),
        );
        let params = Parameters::new(vec![array]);
        let result = get_length(thread, params).await?.expect("length");
        assert_eq!(result.as_i32()?, 2);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_length_byte_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![1i8, 2i8, 3i8]),
        );
        let params = Parameters::new(vec![array]);
        let result = get_length(thread, params).await?.expect("length");
        assert_eq!(result.as_i32()?, 3);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_length_char_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec!['a', 'b']),
        );
        let params = Parameters::new(vec![array]);
        let result = get_length(thread, params).await?.expect("length");
        assert_eq!(result.as_i32()?, 2);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_length_short_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![1i16]),
        );
        let params = Parameters::new(vec![array]);
        let result = get_length(thread, params).await?.expect("length");
        assert_eq!(result.as_i32()?, 1);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_length_long_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![1i64, 2i64, 3i64, 4i64]),
        );
        let params = Parameters::new(vec![array]);
        let result = get_length(thread, params).await?.expect("length");
        assert_eq!(result.as_i32()?, 4);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_length_float_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![1.0f32, 2.0f32]),
        );
        let params = Parameters::new(vec![array]);
        let result = get_length(thread, params).await?.expect("length");
        assert_eq!(result.as_i32()?, 2);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_length_double_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![1.0f64]),
        );
        let params = Parameters::new(vec![array]);
        let result = get_length(thread, params).await?.expect("length");
        assert_eq!(result.as_i32()?, 1);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_length_empty_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(Vec::<i32>::new()),
        );
        let params = Parameters::new(vec![array]);
        let result = get_length(thread, params).await?.expect("length");
        assert_eq!(result.as_i32()?, 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_multi_new_array_negative_dimension() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class = thread.class("int").await?;
        let class_object = class.to_object(&thread).await?;
        let dimensions = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![-1i32]),
        );
        let params = Parameters::new(vec![class_object, dimensions]);
        let result = multi_new_array(thread, params).await;
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_multi_new_array_boolean() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class = thread.class("boolean").await?;
        let class_object = class.to_object(&thread).await?;
        let dimensions = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![3i32]),
        );
        let params = Parameters::new(vec![class_object, dimensions]);
        let result = multi_new_array(thread.clone(), params)
            .await?
            .expect("array");
        let len_params = Parameters::new(vec![result]);
        let len = get_length(thread, len_params).await?.expect("length");
        assert_eq!(len.as_i32()?, 3);
        Ok(())
    }

    #[tokio::test]
    async fn test_multi_new_array_byte() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class = thread.class("byte").await?;
        let class_object = class.to_object(&thread).await?;
        let dimensions = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![3i32]),
        );
        let params = Parameters::new(vec![class_object, dimensions]);
        let result = multi_new_array(thread.clone(), params)
            .await?
            .expect("array");
        let len_params = Parameters::new(vec![result]);
        let len = get_length(thread, len_params).await?.expect("length");
        assert_eq!(len.as_i32()?, 3);
        Ok(())
    }

    #[tokio::test]
    async fn test_multi_new_array_char() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class = thread.class("char").await?;
        let class_object = class.to_object(&thread).await?;
        let dimensions = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![3i32]),
        );
        let params = Parameters::new(vec![class_object, dimensions]);
        let result = multi_new_array(thread.clone(), params)
            .await?
            .expect("array");
        let len_params = Parameters::new(vec![result]);
        let len = get_length(thread, len_params).await?.expect("length");
        assert_eq!(len.as_i32()?, 3);
        Ok(())
    }

    #[tokio::test]
    async fn test_multi_new_array_short() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class = thread.class("short").await?;
        let class_object = class.to_object(&thread).await?;
        let dimensions = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![3i32]),
        );
        let params = Parameters::new(vec![class_object, dimensions]);
        let result = multi_new_array(thread.clone(), params)
            .await?
            .expect("array");
        let len_params = Parameters::new(vec![result]);
        let len = get_length(thread, len_params).await?.expect("length");
        assert_eq!(len.as_i32()?, 3);
        Ok(())
    }

    #[tokio::test]
    async fn test_multi_new_array_long() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class = thread.class("long").await?;
        let class_object = class.to_object(&thread).await?;
        let dimensions = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![3i32]),
        );
        let params = Parameters::new(vec![class_object, dimensions]);
        let result = multi_new_array(thread.clone(), params)
            .await?
            .expect("array");
        let len_params = Parameters::new(vec![result]);
        let len = get_length(thread, len_params).await?.expect("length");
        assert_eq!(len.as_i32()?, 3);
        Ok(())
    }

    #[tokio::test]
    async fn test_multi_new_array_float() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class = thread.class("float").await?;
        let class_object = class.to_object(&thread).await?;
        let dimensions = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![3i32]),
        );
        let params = Parameters::new(vec![class_object, dimensions]);
        let result = multi_new_array(thread.clone(), params)
            .await?
            .expect("array");
        let len_params = Parameters::new(vec![result]);
        let len = get_length(thread, len_params).await?.expect("length");
        assert_eq!(len.as_i32()?, 3);
        Ok(())
    }

    #[tokio::test]
    async fn test_multi_new_array_double() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class = thread.class("double").await?;
        let class_object = class.to_object(&thread).await?;
        let dimensions = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![3i32]),
        );
        let params = Parameters::new(vec![class_object, dimensions]);
        let result = multi_new_array(thread.clone(), params)
            .await?
            .expect("array");
        let len_params = Parameters::new(vec![result]);
        let len = get_length(thread, len_params).await?.expect("length");
        assert_eq!(len.as_i32()?, 3);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_boolean_array_via_generic_set() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![false]),
        );
        // Value::Int(1) is unboxed as PrimitiveValue::Int, not Boolean,
        // so setting into a BooleanArray via generic set should fail
        let params = Parameters::new(vec![array, Value::Int(0), Value::Int(1)]);
        assert!(set(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_set_long_array_via_generic_set() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0i64]),
        );
        let params = Parameters::new(vec![array.clone(), Value::Int(0), Value::Long(99)]);
        set(thread.clone(), params).await?;
        let params = Parameters::new(vec![array, Value::Int(0)]);
        let result = get_long(thread, params).await?.expect("value");
        assert_eq!(result.as_i64()?, 99);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_float_array_via_generic_set() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0f32]),
        );
        let params = Parameters::new(vec![
            array.clone(),
            Value::Int(0),
            Value::Float(std::f32::consts::PI),
        ]);
        set(thread.clone(), params).await?;
        let params = Parameters::new(vec![array, Value::Int(0)]);
        let result = get_float(thread, params).await?.expect("value");
        let diff = result.as_f32()? - std::f32::consts::PI;
        assert!(diff.abs() < f32::EPSILON);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_double_array_via_generic_set() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0.0f64]),
        );
        let params = Parameters::new(vec![
            array.clone(),
            Value::Int(0),
            Value::Double(std::f64::consts::E),
        ]);
        set(thread.clone(), params).await?;
        let params = Parameters::new(vec![array, Value::Int(0)]);
        let result = get_double(thread, params).await?.expect("value");
        let diff = result.as_f64()? - std::f64::consts::E;
        assert!(diff.abs() < f64::EPSILON);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_index_out_of_bounds() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0i32]),
        );
        let params = Parameters::new(vec![array, Value::Int(5), Value::Int(1)]);
        assert!(set(thread, params).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_index_out_of_bounds() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let array = Value::new_object(
            thread.vm()?.garbage_collector(),
            Reference::from(vec![0i32]),
        );
        let params = Parameters::new(vec![array, Value::Int(5)]);
        assert!(get(thread, params).await.is_err());
        Ok(())
    }

    // Total new tests: 124
}
