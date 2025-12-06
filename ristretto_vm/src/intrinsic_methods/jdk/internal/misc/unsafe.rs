use crate::Error::{InternalError, InvalidOperand};
use crate::JavaError::ArrayIndexOutOfBoundsException;
use crate::Result;
use crate::intrinsic_methods::java::lang::class::get_class;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::{Between, Equal, GreaterThan, GreaterThanOrEqual};
use ristretto_classfile::{BaseType, FieldAccessFlags, JAVA_11, JAVA_17};
use ristretto_classloader::{Reference, Value};
use ristretto_gc::Gc;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

pub(crate) const BOOLEAN_SIZE: usize = 1;
pub(crate) const BYTE_SIZE: usize = 1;
pub(crate) const CHAR_SIZE: usize = 2;
pub(crate) const SHORT_SIZE: usize = 2;
pub(crate) const INT_SIZE: usize = 4;
pub(crate) const LONG_SIZE: usize = 8;
pub(crate) const FLOAT_SIZE: usize = 4;
pub(crate) const DOUBLE_SIZE: usize = 8;

/// The size of a pointer in bytes
#[cfg(target_pointer_width = "64")]
pub(crate) const REFERENCE_SIZE: usize = 8;
#[cfg(target_pointer_width = "32")]
pub(crate) const REFERENCE_SIZE: usize = 4;

#[intrinsic_method("jdk/internal/misc/Unsafe.addressSize0()I", Equal(JAVA_11))]
#[async_recursion(?Send)]
pub(crate) async fn address_size_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let pointer_size = i32::try_from(REFERENCE_SIZE)?;
    Ok(Some(Value::Int(pointer_size)))
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.allocateInstance(Ljava/lang/Class;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn allocate_instance(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.allocateInstance(Ljava/lang/Class;)Ljava/lang/Object;")
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.allocateMemory0(J)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn allocate_memory_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.allocateMemory0(J)J")
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.arrayBaseOffset0(Ljava/lang/Class;)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn array_base_offset_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(0)))
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.arrayIndexScale0(Ljava/lang/Class;)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn array_index_scale_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let object = parameters.pop()?;
    let class = get_class(&thread, &object).await?;
    let class_name = class.name();
    let scale = match class_name {
        "[Z" => BOOLEAN_SIZE, // boolean
        "[B" => BYTE_SIZE,    // byte
        "[C" => CHAR_SIZE,    // char (Java is 2 bytes)
        "[S" => SHORT_SIZE,   // short
        "[I" => INT_SIZE,     // int
        "[F" => FLOAT_SIZE,   // float
        "[J" => LONG_SIZE,    // long
        "[D" => DOUBLE_SIZE,  // double
        _ if class_name.starts_with("[L") => {
            // object reference; use the address size
            REFERENCE_SIZE
        }
        _ => {
            return Err(InternalError(format!(
                "Unknown array class type '{class_name}'"
            )));
        }
    };
    Ok(Some(Value::Int(i32::try_from(scale)?)))
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.compareAndExchangeInt(Ljava/lang/Object;JII)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn compare_and_exchange_int(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.compareAndExchangeInt(Ljava/lang/Object;JII)I")
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.compareAndExchangeLong(Ljava/lang/Object;JJJ)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn compare_and_exchange_long(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.compareAndExchangeLong(Ljava/lang/Object;JJJ)J")
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.compareAndExchangeObject(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
    Equal(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn compare_and_exchange_object(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "jdk.internal.misc.Unsafe.compareAndExchangeObject(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.compareAndExchangeReference(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThan(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn compare_and_exchange_reference(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "jdk.internal.misc.Unsafe.compareAndExchangeReference(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.compareAndSetInt(Ljava/lang/Object;JII)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn compare_and_set_int(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let x = parameters.pop_int()?;
    let expected = parameters.pop_int()?;
    let mut offset = parameters.pop()?;
    let Value::Long(ref mut offset) = offset else {
        return Err(InvalidOperand {
            expected: "long".to_string(),
            actual: offset.to_string(),
        });
    };

    let value = parameters.pop()?;
    let result = if value.is_object() {
        let mut object = value.as_object_mut()?;
        let class = object.class();
        let offset = usize::try_from(*offset)?;
        let field_name = class.field_name(offset)?;
        let value = object.value(&field_name)?.as_i32()?;
        if value == expected {
            object.set_value(&field_name, Value::Int(x))?;
            1
        } else {
            0
        }
    } else if i32::try_from(*offset)? == expected {
        *offset = i64::from(x);
        1
    } else {
        0
    };
    Ok(Some(Value::Int(result)))
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.compareAndSetLong(Ljava/lang/Object;JJJ)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn compare_and_set_long(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let x = parameters.pop_long()?;
    let expected = parameters.pop_long()?;
    let mut offset = parameters.pop()?;
    let Value::Long(ref mut offset) = offset else {
        return Err(InvalidOperand {
            expected: "long".to_string(),
            actual: offset.to_string(),
        });
    };

    let value = parameters.pop()?;
    let result = if value.is_object() {
        let mut object = value.as_object_mut()?;
        let class = object.class();
        let offset = usize::try_from(*offset)?;
        let field_name = class.field_name(offset)?;
        let value = object.value(&field_name)?.as_i64()?;
        if value == expected {
            object.set_value(&field_name, Value::Long(x))?;
            1
        } else {
            0
        }
    } else if *offset == expected {
        *offset = x;
        1
    } else {
        0
    };
    Ok(Some(Value::Int(result)))
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.compareAndSetObject(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z",
    Equal(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn compare_and_set_object(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    compare_and_set_reference(thread, parameters).await
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.compareAndSetReference(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z",
    GreaterThan(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn compare_and_set_reference(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let x = parameters.pop()?;
    let expected = parameters.pop()?;
    let offset = parameters.pop_long()?;
    let offset = usize::try_from(offset)?;
    let reference = parameters.pop()?;
    let mut reference = reference.as_reference_mut()?;

    let result = match &mut *reference {
        Reference::Array(object_array) => {
            let offset = offset / REFERENCE_SIZE;
            let elements = &mut object_array.elements;
            let Some(value) = elements.get(offset) else {
                return Err(InternalError(
                    "compareAndSetReference: Invalid reference index".to_string(),
                ));
            };
            let Value::Object(ref expected_reference) = expected else {
                return Err(InvalidOperand {
                    expected: "object".to_string(),
                    actual: expected.to_string(),
                });
            };

            let Value::Object(reference) = value else {
                return Err(InvalidOperand {
                    expected: "object".to_string(),
                    actual: value.to_string(),
                });
            };

            let equal = match (reference, expected_reference) {
                (Some(r1), Some(r2)) => Gc::ptr_eq(r1, r2),
                (None, None) => true,
                _ => false,
            };

            if equal {
                let Value::Object(x_reference) = x else {
                    return Err(InvalidOperand {
                        expected: "object".to_string(),
                        actual: x.to_string(),
                    });
                };
                elements[offset] = Value::Object(x_reference);
                1
            } else {
                0
            }
        }
        Reference::Object(object) => {
            let field_name = object.class().field_name(offset)?;
            let value = object.value(&field_name)?;
            if value == expected {
                object.set_value(&field_name, x)?;
                1
            } else {
                0
            }
        }
        _ => {
            return Err(InternalError(
                "compareAndSetReference: Invalid reference".to_string(),
            ));
        }
    };
    Ok(Some(Value::Int(result)))
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.copyMemory0(Ljava/lang/Object;JLjava/lang/Object;JJ)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn copy_memory_0(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _bytes = usize::try_from(parameters.pop_long()?)?;
    let _destination_offset = usize::try_from(parameters.pop_long()?)?;
    let Value::Object(ref mut destination) = parameters.pop()? else {
        return Err(InternalError(
            "copyMemory0: Invalid destination".to_string(),
        ));
    };
    let _source_offset = usize::try_from(parameters.pop_long()?)?;
    let Value::Object(ref mut source) = parameters.pop()? else {
        return Err(InternalError("copyMemory0: Invalid source".to_string()));
    };
    destination.clone_from(source);
    Ok(None)
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.copySwapMemory0(Ljava/lang/Object;JLjava/lang/Object;JJJ)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn copy_swap_memory_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.copySwapMemory0(Ljava/lang/Object;JLjava/lang/Object;JJJ)V")
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.defineAnonymousClass0(Ljava/lang/Class;[B[Ljava/lang/Object;)Ljava/lang/Class;",
    Equal(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn define_anonymous_class_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "jdk.internal.misc.Unsafe.defineAnonymousClass0(Ljava/lang/Class;[B[Ljava/lang/Object;)Ljava/lang/Class;"
    )
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.defineClass0(Ljava/lang/String;[BIILjava/lang/ClassLoader;Ljava/security/ProtectionDomain;)Ljava/lang/Class;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn define_class_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "jdk.internal.misc.Unsafe.defineClass0(Ljava/lang/String;[BIILjava/lang/ClassLoader;Ljava/security/ProtectionDomain;)Ljava/lang/Class;"
    )
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.ensureClassInitialized0(Ljava/lang/Class;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn ensure_class_initialized_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.freeMemory0(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn free_memory_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("jdk/internal/misc/Unsafe.fullFence()V", GreaterThanOrEqual(JAVA_11))]
#[async_recursion(?Send)]
pub(crate) async fn full_fence(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[expect(clippy::too_many_lines)]
fn get_reference_type(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
    base_type: Option<BaseType>,
) -> Result<Option<Value>> {
    let offset = parameters.pop_long()?;
    let Some(reference) = parameters.pop_reference()? else {
        let Some(base_type) = base_type else {
            return Err(InternalError(
                "getReferenceType: Invalid reference".to_string(),
            ));
        };
        let value = match base_type {
            BaseType::Boolean
            | BaseType::Byte
            | BaseType::Char
            | BaseType::Int
            | BaseType::Short => Value::Int(i32::try_from(offset)?),
            BaseType::Long => Value::Long(offset),
            BaseType::Double | BaseType::Float => {
                return Err(InternalError(
                    "getReferenceType: Invalid offset".to_string(),
                ));
            }
        };
        return Ok(Some(value));
    };

    let offset = usize::try_from(offset)?;
    let guard = reference.read();
    match &*guard {
        Reference::Array(object_array) => {
            let array = &object_array.elements;
            let offset = offset / REFERENCE_SIZE;
            let Some(value) = array.get(offset) else {
                return Err(ArrayIndexOutOfBoundsException {
                    index: i32::try_from(offset)?,
                    length: array.len(),
                }
                .into());
            };
            Ok(Some(value.clone()))
        }
        Reference::Object(object) => {
            let class = object.class();
            let field_name = class.field_name(offset)?;
            let field = class.declared_field(&field_name)?;
            if field.access_flags().contains(FieldAccessFlags::STATIC) {
                Ok(Some(class.static_value(&field_name)?))
            } else {
                Ok(Some(object.value(&field_name)?))
            }
        }
        primitive_array => {
            let array: &[u8] = match primitive_array {
                Reference::ByteArray(array) => bytemuck::cast_slice(array.as_ref()),
                Reference::CharArray(array) => bytemuck::cast_slice(array.as_ref()),
                Reference::ShortArray(array) => bytemuck::cast_slice(array.as_ref()),
                Reference::IntArray(array) => bytemuck::cast_slice(array.as_ref()),
                Reference::LongArray(array) => bytemuck::cast_slice(array.as_ref()),
                Reference::FloatArray(array) => bytemuck::cast_slice(array.as_ref()),
                Reference::DoubleArray(array) => bytemuck::cast_slice(array.as_ref()),
                _ => unreachable!("Reference type should be handled"),
            };

            let Some(base_type) = base_type else {
                return Err(InternalError(
                    "getReferenceType: Invalid base type".to_string(),
                ));
            };

            // Check bounds and zero-fill if index is short
            let required_bytes = match base_type {
                BaseType::Boolean | BaseType::Byte => 1,
                BaseType::Char | BaseType::Short => 2,
                BaseType::Int | BaseType::Float => 4,
                BaseType::Long | BaseType::Double => 8,
            };

            // If offset is beyond the array, throw proper Java exception
            if offset >= array.len() {
                return Err(ArrayIndexOutOfBoundsException {
                    index: i32::try_from(offset)?,
                    length: array.len(),
                }
                .into());
            }

            // Create a zero-filled buffer and copy available bytes
            let mut buffer = [0u8; 8];
            let slice = &array[offset..];
            let available_bytes = slice.len().min(required_bytes);
            buffer[..available_bytes].copy_from_slice(&slice[..available_bytes]);

            let value = match base_type {
                BaseType::Boolean | BaseType::Byte => {
                    let value = buffer[0];
                    Value::Int(i32::from(value))
                }
                BaseType::Char => {
                    let value = u16::from_be_bytes([buffer[0], buffer[1]]);
                    Value::Int(i32::from(value))
                }
                BaseType::Int => {
                    let value = i32::from_be_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
                    Value::Int(value)
                }
                BaseType::Short => {
                    let value = i16::from_be_bytes([buffer[0], buffer[1]]);
                    Value::Int(i32::from(value))
                }
                BaseType::Long => {
                    let value = i64::from_be_bytes(buffer);
                    Value::Long(value)
                }
                BaseType::Float => {
                    let bits = u32::from_be_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
                    let value = f32::from_bits(bits);
                    Value::Float(value)
                }
                BaseType::Double => {
                    let bits = u64::from_be_bytes(buffer);
                    let value = f64::from_bits(bits);
                    Value::Double(value)
                }
            };
            Ok(Some(value))
        }
    }
}

#[expect(clippy::too_many_lines)]
fn put_reference_type(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
    base_type: Option<BaseType>,
) -> Result<Option<Value>> {
    let value = parameters.pop()?;
    // validate the value type
    match (&base_type, &value) {
        (
            Some(
                BaseType::Boolean
                | BaseType::Byte
                | BaseType::Char
                | BaseType::Int
                | BaseType::Short,
            ),
            Value::Int(_),
        )
        | (Some(BaseType::Long), Value::Long(_))
        | (Some(BaseType::Float), Value::Float(_))
        | (Some(BaseType::Double), Value::Double(_))
        | (None, Value::Object(_)) => {}
        _ => {
            return Err(InternalError(
                "putReferenceType: Invalid value type".to_string(),
            ));
        }
    }
    let offset = parameters.pop_long()?;
    let Some(reference) = parameters.pop_reference()? else {
        return Err(InternalError(
            "putReferenceType: Invalid reference".to_string(),
        ));
    };

    let offset = usize::try_from(offset)?;

    let mut guard = reference.write();
    match &mut *guard {
        Reference::Array(object_array) => {
            let Value::Object(object_value) = value else {
                return Err(InternalError(
                    "putReferenceType: Invalid value type for object array".to_string(),
                ));
            };
            let offset = offset / REFERENCE_SIZE;
            let array = &mut object_array.elements;
            if let Some(element) = array.get_mut(offset) {
                *element = Value::Object(object_value);
            } else {
                return Err(ArrayIndexOutOfBoundsException {
                    index: i32::try_from(offset)?,
                    length: array.len(),
                }
                .into());
            }
        }
        Reference::Object(object) => {
            let class = object.class();
            let field_name = class.field_name(offset)?;
            let field = class.declared_field(&field_name)?;
            if field.access_flags().contains(FieldAccessFlags::STATIC) {
                class.set_static_value(&field_name, value)?;
            } else {
                object.set_value(&field_name, value)?;
            }
        }
        primitive_array => {
            let array: &mut [u8] = match primitive_array {
                Reference::ByteArray(array) => bytemuck::cast_slice_mut(array.as_mut()),
                Reference::CharArray(array) => bytemuck::cast_slice_mut(array.as_mut()),
                Reference::ShortArray(array) => bytemuck::cast_slice_mut(array.as_mut()),
                Reference::IntArray(array) => bytemuck::cast_slice_mut(array.as_mut()),
                Reference::LongArray(array) => bytemuck::cast_slice_mut(array.as_mut()),
                Reference::FloatArray(array) => bytemuck::cast_slice_mut(array.as_mut()),
                Reference::DoubleArray(array) => bytemuck::cast_slice_mut(array.as_mut()),
                _ => unreachable!("Reference type should be handled"),
            };

            let bytes = match (base_type, &value) {
                (Some(BaseType::Boolean) | Some(BaseType::Byte), Value::Int(v)) => {
                    let v = i8::try_from(*v)?;
                    v.to_be_bytes().to_vec()
                }
                (Some(BaseType::Char), Value::Int(v)) => {
                    let v = u16::try_from(*v)?;
                    v.to_be_bytes().to_vec()
                }
                (Some(BaseType::Short), Value::Int(v)) => {
                    let v = i16::try_from(*v)?;
                    v.to_be_bytes().to_vec()
                }
                (Some(BaseType::Int), Value::Int(v)) => v.to_be_bytes().to_vec(),
                (Some(BaseType::Float), Value::Float(v)) => v.to_be_bytes().to_vec(),
                (Some(BaseType::Long), Value::Long(v)) => v.to_be_bytes().to_vec(),
                (Some(BaseType::Double), Value::Double(v)) => v.to_be_bytes().to_vec(),
                _ => {
                    return Err(InternalError(
                        "putReferenceType: Invalid value type".to_string(),
                    ));
                }
            };

            let Some(end) = offset.checked_add(bytes.len()) else {
                return Err(ArrayIndexOutOfBoundsException {
                    index: i32::try_from(offset)?,
                    length: array.len(),
                }
                .into());
            };
            if end > array.len() {
                return Err(ArrayIndexOutOfBoundsException {
                    index: i32::try_from(offset)?,
                    length: array.len(),
                }
                .into());
            }
            array[offset..end].copy_from_slice(&bytes);
        }
    }

    Ok(None)
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.getBoolean(Ljava/lang/Object;J)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_boolean(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    get_boolean_volatile(thread, parameters).await
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.getBooleanVolatile(Ljava/lang/Object;J)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_boolean_volatile(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    get_reference_type(thread, parameters, Some(BaseType::Boolean))
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.getByte(Ljava/lang/Object;J)B",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_byte(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    get_byte_volatile(thread, parameters).await
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.getByteVolatile(Ljava/lang/Object;J)B",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_byte_volatile(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    get_reference_type(thread, parameters, Some(BaseType::Byte))
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.getChar(Ljava/lang/Object;J)C",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_char(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    get_char_volatile(thread, parameters).await
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.getCharVolatile(Ljava/lang/Object;J)C",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_char_volatile(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    get_reference_type(thread, parameters, Some(BaseType::Char))
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.getDouble(Ljava/lang/Object;J)D",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_double(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    get_double_volatile(thread, parameters).await
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.getDoubleVolatile(Ljava/lang/Object;J)D",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_double_volatile(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    get_reference_type(thread, parameters, Some(BaseType::Double))
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.getFloat(Ljava/lang/Object;J)F",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_float(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    get_float_volatile(thread, parameters).await
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.getFloatVolatile(Ljava/lang/Object;J)F",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_float_volatile(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    get_reference_type(thread, parameters, Some(BaseType::Float))
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.getInt(Ljava/lang/Object;J)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_int(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    get_int_volatile(thread, parameters).await
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.getIntVolatile(Ljava/lang/Object;J)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_int_volatile(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    get_reference_type(thread, parameters, Some(BaseType::Int))
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.getLoadAverage0([DI)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_load_average_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.getLoadAverage0([DI)I")
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.getLong(Ljava/lang/Object;J)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_long(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    get_long_volatile(thread, parameters).await
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.getLongVolatile(Ljava/lang/Object;J)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_long_volatile(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    get_reference_type(thread, parameters, Some(BaseType::Long))
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.getObject(Ljava/lang/Object;J)Ljava/lang/Object;",
    Equal(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_object(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    get_object_volatile(thread, parameters).await
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.getObjectVolatile(Ljava/lang/Object;J)Ljava/lang/Object;",
    Equal(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_object_volatile(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    get_reference_type(thread, parameters, None)
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.getReference(Ljava/lang/Object;J)Ljava/lang/Object;",
    GreaterThan(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_reference(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    get_reference_volatile(thread, parameters).await
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.getReferenceVolatile(Ljava/lang/Object;J)Ljava/lang/Object;",
    GreaterThan(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_reference_volatile(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    get_reference_type(thread, parameters, None)
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.getShort(Ljava/lang/Object;J)S",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_short(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    get_short_volatile(thread, parameters).await
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.getShortVolatile(Ljava/lang/Object;J)S",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_short_volatile(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    get_reference_type(thread, parameters, Some(BaseType::Short))
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.getUncompressedObject(J)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_uncompressed_object(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.getUncompressedObject(J)Ljava/lang/Object;")
}

#[intrinsic_method("jdk/internal/misc/Unsafe.isBigEndian0()Z", Equal(JAVA_11))]
#[async_recursion(?Send)]
pub(crate) async fn is_big_endian_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(true)))
}

#[intrinsic_method("jdk/internal/misc/Unsafe.loadFence()V", Between(JAVA_11, JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn load_fence(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.objectFieldOffset0(Ljava/lang/reflect/Field;)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn object_field_offset_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let field = parameters.pop()?;
    let (class, name) = {
        let field = field.as_object_ref()?;
        let class = field.value("clazz")?;
        let name = field.value("name")?;
        (class, name)
    };
    let parameters = Parameters::new(vec![class, name]);
    object_field_offset_1(thread, parameters).await
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.objectFieldOffset1(Ljava/lang/Class;Ljava/lang/String;)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn object_field_offset_1(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let field_name = parameters.pop()?.as_string()?;
    let class_object = parameters.pop()?;
    let class_name = {
        let class_object = class_object.as_object_ref()?;
        class_object.value("name")?.as_string()?
    };
    let class = thread.class(&class_name).await?;
    let offset = class.field_offset(&field_name)?;
    let offset = i64::try_from(offset)?;
    Ok(Some(Value::Long(offset)))
}

#[intrinsic_method("jdk/internal/misc/Unsafe.pageSize()I", Equal(JAVA_11))]
#[async_recursion(?Send)]
pub(crate) async fn page_size(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let page_size;

    #[cfg(target_os = "macos")]
    {
        page_size = 16_384;
    }

    #[cfg(not(target_os = "macos"))]
    {
        // The page size is typically 4096 bytes on most systems.
        page_size = 4_096;
    }

    Ok(Some(Value::Int(page_size)))
}

#[intrinsic_method("jdk/internal/misc/Unsafe.park(ZJ)V", GreaterThanOrEqual(JAVA_11))]
#[async_recursion(?Send)]
pub(crate) async fn park(thread: Arc<Thread>, mut parameters: Parameters) -> Result<Option<Value>> {
    let time = u64::try_from(parameters.pop_long()?)?;
    let is_absolute = parameters.pop_bool()?;
    thread.park(is_absolute, time).await?;
    Ok(None)
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.putBoolean(Ljava/lang/Object;JZ)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn put_boolean(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    put_boolean_volatile(thread, parameters).await
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.putBooleanVolatile(Ljava/lang/Object;JZ)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn put_boolean_volatile(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    put_reference_type(thread, parameters, Some(BaseType::Boolean))
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.putByte(Ljava/lang/Object;JB)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn put_byte(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    put_byte_volatile(thread, parameters).await
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.putByteVolatile(Ljava/lang/Object;JB)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn put_byte_volatile(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    put_reference_type(thread, parameters, Some(BaseType::Byte))
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.putChar(Ljava/lang/Object;JC)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn put_char(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    put_char_volatile(thread, parameters).await
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.putCharVolatile(Ljava/lang/Object;JC)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn put_char_volatile(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    put_reference_type(thread, parameters, Some(BaseType::Char))
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.putDouble(Ljava/lang/Object;JD)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn put_double(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    put_double_volatile(thread, parameters).await
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.putDoubleVolatile(Ljava/lang/Object;JD)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn put_double_volatile(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    put_reference_type(thread, parameters, Some(BaseType::Double))
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.putFloat(Ljava/lang/Object;JF)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn put_float(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    put_float_volatile(thread, parameters).await
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.putFloatVolatile(Ljava/lang/Object;JF)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn put_float_volatile(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    put_reference_type(thread, parameters, Some(BaseType::Float))
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.putInt(Ljava/lang/Object;JI)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn put_int(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    put_int_volatile(thread, parameters).await
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.putIntVolatile(Ljava/lang/Object;JI)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn put_int_volatile(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    put_reference_type(thread, parameters, Some(BaseType::Int))
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.putLong(Ljava/lang/Object;JJ)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn put_long(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    put_long_volatile(thread, parameters).await
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.putLongVolatile(Ljava/lang/Object;JJ)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn put_long_volatile(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    put_reference_type(thread, parameters, Some(BaseType::Long))
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.putObject(Ljava/lang/Object;JLjava/lang/Object;)V",
    Equal(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn put_object(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    put_object_volatile(thread, parameters).await
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.putObjectVolatile(Ljava/lang/Object;JLjava/lang/Object;)V",
    Equal(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn put_object_volatile(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    put_reference_type(thread, parameters, None)
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.putReference(Ljava/lang/Object;JLjava/lang/Object;)V",
    GreaterThan(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn put_reference(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    put_reference_volatile(thread, parameters).await
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.putReferenceVolatile(Ljava/lang/Object;JLjava/lang/Object;)V",
    GreaterThan(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn put_reference_volatile(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    put_reference_type(thread, parameters, None)
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.putShort(Ljava/lang/Object;JS)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn put_short(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    put_short_volatile(thread, parameters).await
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.putShortVolatile(Ljava/lang/Object;JS)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn put_short_volatile(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    put_reference_type(thread, parameters, Some(BaseType::Short))
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.reallocateMemory0(JJ)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn reallocate_memory_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.reallocateMemory0(JJ)J")
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.registerNatives()V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn register_natives(
    thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let vm = thread.vm()?;
    if vm.java_major_version() >= 17 {
        // Set the endian to big endian
        let class = thread.class("jdk.internal.misc.UnsafeConstants").await?;
        class.set_static_value("BIG_ENDIAN", Value::from(true))?;
    }
    Ok(None)
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.setMemory0(Ljava/lang/Object;JJB)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn set_memory_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.setMemory0(Ljava/lang/Object;JJB)V")
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.shouldBeInitialized0(Ljava/lang/Class;)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn should_be_initialized_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // Classes are always initialized
    Ok(Some(Value::from(false)))
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.staticFieldBase0(Ljava/lang/reflect/Field;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn static_field_base_0(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ = parameters.pop()?;
    Ok(Some(Value::Object(None)))
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.staticFieldOffset0(Ljava/lang/reflect/Field;)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn static_field_offset_0(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ = parameters.pop()?;
    Ok(Some(Value::Long(0)))
}

#[intrinsic_method("jdk/internal/misc/Unsafe.storeFence()V", Between(JAVA_11, JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn store_fence(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.throwException(Ljava/lang/Throwable;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn throw_exception(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.throwException(Ljava/lang/Throwable;)V")
}

#[intrinsic_method("jdk/internal/misc/Unsafe.unalignedAccess0()Z", Equal(JAVA_11))]
#[async_recursion(?Send)]
pub(crate) async fn unaligned_access_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.unpark(Ljava/lang/Object;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn unpark(thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    thread.unpark();
    Ok(None)
}

#[intrinsic_method("jdk/internal/misc/Unsafe.writeback0(J)V", GreaterThan(JAVA_11))]
#[async_recursion(?Send)]
pub(crate) async fn writeback_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.writeback0(J)V")
}

#[intrinsic_method("jdk/internal/misc/Unsafe.writebackPostSync0()V", GreaterThan(JAVA_11))]
#[async_recursion(?Send)]
pub(crate) async fn writeback_post_sync_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.writebackPostSync0()V")
}

#[intrinsic_method("jdk/internal/misc/Unsafe.writebackPreSync0()V", GreaterThan(JAVA_11))]
#[async_recursion(?Send)]
pub(crate) async fn writeback_pre_sync_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.writebackPreSync0()V")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::JavaObject;
    use std::time::Duration;

    /// Creates a java.lang.reflect.Field for testing purposes.
    async fn create_field(thread: &Thread) -> Result<Value> {
        let string_class = thread.class("java/lang/String").await?;
        let string_class_object = string_class.to_object(thread).await?;
        let descriptor =
            "Ljava/lang/Class;Ljava/lang/String;Ljava/lang/Class;IZILjava/lang/String;[B";
        let parameters = vec![
            string_class_object,              // Declaring Class
            "value".to_object(thread).await?, // Field name
            Value::Object(None),              // Type
            Value::Int(0),                    // Modifiers
            Value::from(false),               // Trusted Final
            Value::Int(0),                    // Slot
            "[B".to_object(thread).await?,    // Signature
            Value::Object(None),              // Annotations
        ];
        thread
            .object("java/lang/reflect/Field", descriptor, &parameters)
            .await
    }

    #[tokio::test]
    async fn test_address_size_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = address_size_0(thread, Parameters::default()).await?;
        let pointer_size = i32::try_from(REFERENCE_SIZE)?;
        assert_eq!(result, Some(Value::Int(pointer_size)));
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.Unsafe.allocateInstance(Ljava/lang/Class;)Ljava/lang/Object;"
    )]
    async fn test_allocate_instance() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = allocate_instance(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.internal.misc.Unsafe.allocateMemory0(J)J")]
    async fn test_allocate_memory_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = allocate_memory_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_array_base_offset_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = array_base_offset_0(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_array_index_scale_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let tests = vec![
            ("[Z", BOOLEAN_SIZE),
            ("[B", BYTE_SIZE),
            ("[C", CHAR_SIZE),
            ("[S", SHORT_SIZE),
            ("[I", INT_SIZE),
            ("[F", FLOAT_SIZE),
            ("[J", LONG_SIZE),
            ("[D", DOUBLE_SIZE),
            ("[Ljava/lang/Object;", REFERENCE_SIZE),
        ];

        for (class_name, expected_scale) in tests {
            let expected_scale = i32::try_from(expected_scale)?;
            let class = thread.class(class_name).await?;
            let class_object = class.to_object(&thread).await?;
            let parameters = Parameters::new(vec![class_object]);
            let result = array_index_scale_0(thread.clone(), parameters)
                .await?
                .expect("scale");
            let scale = result.as_i32()?;
            assert_eq!(expected_scale, scale);
        }

        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.Unsafe.compareAndExchangeInt(Ljava/lang/Object;JII)I"
    )]
    async fn test_compare_and_exchange_int() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = compare_and_exchange_int(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.Unsafe.compareAndExchangeLong(Ljava/lang/Object;JJJ)J"
    )]
    async fn test_compare_and_exchange_long() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = compare_and_exchange_long(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.Unsafe.compareAndExchangeObject(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_compare_and_exchange_object() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = compare_and_exchange_object(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.Unsafe.compareAndExchangeReference(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_compare_and_exchange_reference() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = compare_and_exchange_reference(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.Unsafe.copySwapMemory0(Ljava/lang/Object;JLjava/lang/Object;JJJ)V"
    )]
    async fn test_copy_swap_memory_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = copy_swap_memory_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.Unsafe.defineAnonymousClass0(Ljava/lang/Class;[B[Ljava/lang/Object;)Ljava/lang/Class;"
    )]
    async fn test_define_anonymous_class_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = define_anonymous_class_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.Unsafe.defineClass0(Ljava/lang/String;[BIILjava/lang/ClassLoader;Ljava/security/ProtectionDomain;)Ljava/lang/Class;"
    )]
    async fn test_define_class_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = define_class_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_ensure_class_initialized_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = ensure_class_initialized_0(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_free_memory_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = free_memory_0(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_full_fence() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = full_fence(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.Unsafe.getLoadAverage0([DI)I"
    )]
    async fn test_get_load_average_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_load_average_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.Unsafe.getUncompressedObject(J)Ljava/lang/Object;"
    )]
    async fn test_get_uncompressed_object() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_uncompressed_object(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_is_big_endian_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = is_big_endian_0(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::from(true)));
        Ok(())
    }

    #[tokio::test]
    async fn test_load_fence() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = load_fence(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_object_field_offset_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let field = create_field(&thread).await?;
        let mut parameters = Parameters::default();
        parameters.push(field);
        let value = object_field_offset_0(thread, parameters)
            .await?
            .expect("offset");
        let offset = value.as_i64()?;
        assert_eq!(offset, 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_page_size() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let value = page_size(thread, Parameters::default())
            .await?
            .expect("page_size");
        let page_size = value.as_i32()?;
        let expected_page_size;

        #[cfg(target_os = "macos")]
        {
            expected_page_size = 16_384;
        }

        #[cfg(not(target_os = "macos"))]
        {
            expected_page_size = 4_096;
        }

        assert_eq!(page_size, expected_page_size);
        Ok(())
    }

    #[tokio::test]
    async fn test_park() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let start_time = std::time::Instant::now();
        let mut parameters = Parameters::default();
        parameters.push_bool(false);
        // Park the thread for 100 milliseconds
        parameters.push_long(100_000_000);
        let result = park(thread, parameters).await?;
        assert_eq!(result, None);
        let elapsed_time = start_time.elapsed();
        assert!(elapsed_time >= Duration::from_nanos(100_000_000));
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.Unsafe.reallocateMemory0(JJ)J"
    )]
    async fn test_reallocate_memory_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = reallocate_memory_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = register_natives(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.Unsafe.setMemory0(Ljava/lang/Object;JJB)V"
    )]
    async fn test_set_memory_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_memory_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_should_be_initialized_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = should_be_initialized_0(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::from(false)));
        Ok(())
    }

    #[tokio::test]
    async fn test_static_field_base_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let field = create_field(&thread).await?;
        let mut parameters = Parameters::default();
        parameters.push(field);
        let value = static_field_base_0(thread, parameters)
            .await?
            .expect("object");
        assert_eq!(value, Value::Object(None));
        Ok(())
    }

    #[tokio::test]
    async fn test_static_field_offset_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let field = create_field(&thread).await?;
        let mut parameters = Parameters::default();
        parameters.push(field);
        let value = static_field_offset_0(thread, parameters)
            .await?
            .expect("offset");
        let offset = value.as_i64()?;
        assert_eq!(offset, 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_store_fence() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = store_fence(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.Unsafe.throwException(Ljava/lang/Throwable;)V"
    )]
    async fn test_throw_exception() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = throw_exception(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_unaligned_access_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = unaligned_access_0(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::from(false)));
        Ok(())
    }

    #[tokio::test]
    async fn test_unpark() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = unpark(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.internal.misc.Unsafe.writeback0(J)V")]
    async fn test_writeback_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = writeback_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.Unsafe.writebackPostSync0()V"
    )]
    async fn test_writeback_post_sync_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = writeback_post_sync_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.internal.misc.Unsafe.writebackPreSync0()V")]
    async fn test_writeback_pre_sync_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = writeback_pre_sync_0(thread, Parameters::default()).await;
    }
}
