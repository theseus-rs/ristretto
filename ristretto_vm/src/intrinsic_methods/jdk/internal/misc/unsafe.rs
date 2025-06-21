use crate::Error::{InternalError, InvalidOperand};
use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use byteorder::{BigEndian, ReadBytesExt};
use ristretto_classfile::VersionSpecification::{Between, Equal, GreaterThan, GreaterThanOrEqual};
use ristretto_classfile::{BaseType, JAVA_11, JAVA_17};
use ristretto_classloader::{Reference, Value};
use ristretto_macros::intrinsic_method;
use std::io::Cursor;
use std::sync::Arc;

#[intrinsic_method("jdk/internal/misc/Unsafe.addressSize0()I", Equal(JAVA_11))]
#[async_recursion(?Send)]
pub(crate) async fn address_size_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(8))) // 64-bit pointers
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
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(1)))
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

    // TODO: the compare and set operation should be atomic
    let result = if let Some(Reference::Object(object)) = parameters.pop_reference()? {
        let class = object.class();
        let offset = usize::try_from(*offset)?;
        let field_name = class.field_name(offset)?;
        let field = object.field(&field_name)?;
        let value = field.value()?.to_int()?;
        if value == expected {
            field.set_value(Value::Int(x))?;
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

    // TODO: the compare and set operation should be atomic
    let result = if let Some(Reference::Object(object)) = parameters.pop_reference()? {
        let class = object.class();
        let offset = usize::try_from(*offset)?;
        let field_name = class.field_name(offset)?;
        let field = object.field(&field_name)?;
        let value = field.value()?.to_long()?;
        if value == expected {
            field.set_value(Value::Long(x))?;
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
    let reference: Reference = parameters.pop()?.try_into()?;

    // TODO: the compare and set operation should be atomic
    let result = match reference {
        Reference::Array(object_array) => {
            let Some(reference) = object_array.elements.get(offset)? else {
                return Err(InternalError(
                    "compareAndSetReference: Invalid reference index".to_string(),
                ));
            };
            let Value::Object(expected_reference) = expected else {
                return Err(InvalidOperand {
                    expected: "object".to_string(),
                    actual: expected.to_string(),
                });
            };

            if reference == expected_reference {
                let Value::Object(x_reference) = x else {
                    return Err(InvalidOperand {
                        expected: "object".to_string(),
                        actual: x.to_string(),
                    });
                };
                object_array.elements.set(offset, x_reference)?;
                1
            } else {
                0
            }
        }
        Reference::Object(object) => {
            let field_name = object.class().field_name(offset)?;
            let field = object.field(&field_name)?;
            let value = field.value()?;
            if value == expected {
                field.set_value(x)?;
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
    let value = match &reference {
        Reference::ByteArray(_) => {
            let bytes: Vec<u8> = reference.try_into()?;
            let mut bytes = Cursor::new(bytes);
            let position = u64::try_from(offset)?;
            bytes.set_position(position);
            let Some(base_type) = base_type else {
                return Err(InternalError(
                    "getReferenceType: Invalid base type".to_string(),
                ));
            };
            match base_type {
                BaseType::Boolean | BaseType::Byte => {
                    let value = bytes.read_u8()?;
                    Value::Int(i32::from(value))
                }
                BaseType::Char => {
                    let value = bytes.read_u16::<BigEndian>()?;
                    Value::Int(i32::from(value))
                }
                BaseType::Int => {
                    let value = bytes.read_i32::<BigEndian>()?;
                    Value::Int(value)
                }
                BaseType::Short => {
                    let value = bytes.read_i16::<BigEndian>()?;
                    Value::Int(i32::from(value))
                }
                BaseType::Long => {
                    let value = bytes.read_i64::<BigEndian>()?;
                    Value::Long(value)
                }
                BaseType::Float => {
                    let value = bytes.read_f32::<BigEndian>()?;
                    Value::Float(value)
                }
                BaseType::Double => {
                    let value = bytes.read_f64::<BigEndian>()?;
                    Value::Double(value)
                }
            }
        }
        Reference::CharArray(array) => {
            let Some(char) = array.get(offset)? else {
                return Err(InternalError(
                    "getReferenceType: Invalid char reference index".to_string(),
                ));
            };
            Value::Int(i32::from(char))
        }
        Reference::ShortArray(array) => {
            let Some(short) = array.get(offset)? else {
                return Err(InternalError(
                    "getReferenceType: Invalid short reference index".to_string(),
                ));
            };
            Value::Int(i32::from(short))
        }
        Reference::IntArray(array) => {
            let Some(int) = array.get(offset)? else {
                return Err(InternalError(
                    "getReferenceType: Invalid int reference index".to_string(),
                ));
            };
            Value::Int(int)
        }
        Reference::LongArray(array) => {
            let Some(long) = array.get(offset)? else {
                return Err(InternalError(
                    "getReferenceType: Invalid long reference index".to_string(),
                ));
            };
            Value::Long(long)
        }
        Reference::FloatArray(array) => {
            let Some(float) = array.get(offset)? else {
                return Err(InternalError(
                    "getReferenceType: Invalid float reference index".to_string(),
                ));
            };
            Value::Float(float)
        }
        Reference::DoubleArray(array) => {
            let Some(double) = array.get(offset)? else {
                return Err(InternalError(
                    "getReferenceType: Invalid double reference index".to_string(),
                ));
            };
            Value::Double(double)
        }
        Reference::Array(object_array) => {
            let Some(reference) = object_array.elements.get(offset)? else {
                return Err(InternalError(
                    "getReferenceType: Invalid array reference index".to_string(),
                ));
            };
            Value::Object(reference)
        }
        Reference::Object(object) => {
            let field_name = object.class().field_name(offset)?;
            object.value(&field_name)?
        }
    };
    Ok(Some(value))
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
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ = parameters.pop_object()?;
    Ok(Some(Value::Long(0)))
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
    let value = parameters.pop()?;
    let field_name: String = match value {
        Value::Object(_) => value.try_into()?,
        value => {
            return Err(InvalidOperand {
                expected: "object".to_string(),
                actual: value.to_string(),
            });
        }
    };
    let Some(Reference::Object(class_object)) = parameters.pop_reference()? else {
        return Err(InternalError(
            "objectFieldOffset1: Invalid class reference".to_string(),
        ));
    };
    let class_name: String = class_object.value("name")?.try_into()?;
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
pub(crate) async fn park(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.park(ZJ)V")
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
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let x = parameters.pop_bool()?;
    let offset = usize::try_from(parameters.pop_long()?)?;
    let Value::Object(ref mut object) = parameters.pop()? else {
        return Err(InternalError("putBoolean: Invalid reference".to_string()));
    };
    let bytes = Reference::from(vec![x; offset]);
    *object = Some(bytes);
    Ok(None)
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
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let x = i8::try_from(parameters.pop_int()?)?;
    let offset = usize::try_from(parameters.pop_long()?)?;
    let Value::Object(ref mut object) = parameters.pop()? else {
        return Err(InternalError("putByte: Invalid reference".to_string()));
    };
    let bytes = Reference::from(vec![x; offset]);
    *object = Some(bytes);
    Ok(None)
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
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    #[expect(clippy::cast_sign_loss)]
    let x = parameters.pop_int()? as u32;
    let Some(x) = char::from_u32(x) else {
        return Err(InternalError("putChar: Invalid character".to_string()));
    };
    let offset = usize::try_from(parameters.pop_long()?)?;
    let Value::Object(ref mut object) = parameters.pop()? else {
        return Err(InternalError("putChar: Invalid reference".to_string()));
    };
    let bytes = Reference::from(vec![x; offset]);
    *object = Some(bytes);
    Ok(None)
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
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let x = parameters.pop_double()?;
    let offset = usize::try_from(parameters.pop_long()?)?;
    let Value::Object(ref mut object) = parameters.pop()? else {
        return Err(InternalError("putDouble: Invalid reference".to_string()));
    };
    let bytes = Reference::from(vec![x; offset]);
    *object = Some(bytes);
    Ok(None)
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
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let x = parameters.pop_float()?;
    let offset = usize::try_from(parameters.pop_long()?)?;
    let Value::Object(ref mut object) = parameters.pop()? else {
        return Err(InternalError("putFloat: Invalid reference".to_string()));
    };
    let bytes = Reference::from(vec![x; offset]);
    *object = Some(bytes);
    Ok(None)
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
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let x = parameters.pop_int()?;
    let offset = usize::try_from(parameters.pop_long()?)?;
    let Value::Object(ref mut object) = parameters.pop()? else {
        return Err(InternalError("putInt: Invalid reference".to_string()));
    };
    let bytes = Reference::from(vec![x; offset]);
    *object = Some(bytes);
    Ok(None)
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
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let x = parameters.pop_long()?;
    let offset = usize::try_from(parameters.pop_long()?)?;
    let Value::Object(ref mut object) = parameters.pop()? else {
        return Err(InternalError("putlong: Invalid reference".to_string()));
    };
    let bytes = Reference::from(vec![x; offset]);
    *object = Some(bytes);
    Ok(None)
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
    put_reference_volatile(thread, parameters).await
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
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let x = parameters.pop()?;
    let offset = parameters.pop_long()?;
    let offset = usize::try_from(offset)?;
    let Some(object) = parameters.pop_reference()? else {
        return Err(InternalError(
            "putReferenceVolatile: Invalid reference".to_string(),
        ));
    };
    match object {
        Reference::Array(object_array) => {
            let x = x.to_reference()?;
            object_array.elements.set(offset, x)?;
        }
        Reference::Object(object) => {
            let field_name = object.class().field_name(offset)?;
            object.set_value(&field_name, x)?;
        }
        _ => {
            return Err(InternalError(
                "putReferenceVolatile: Invalid reference".to_string(),
            ));
        }
    }
    Ok(None)
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
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let x = i16::try_from(parameters.pop_int()?)?;
    let offset = usize::try_from(parameters.pop_long()?)?;
    let Value::Object(ref mut object) = parameters.pop()? else {
        return Err(InternalError("putShort: Invalid reference".to_string()));
    };
    let bytes = Reference::from(vec![x; offset]);
    *object = Some(bytes);
    Ok(None)
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
        // Set the endianness to big endian
        let class = thread.class("jdk.internal.misc.UnsafeConstants").await?;
        let big_endian = class.static_field("BIG_ENDIAN")?;
        big_endian.set_value(Value::from(true))?;
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
    let _ = parameters.pop_object()?;
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
    let _ = parameters.pop_object()?;
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
pub(crate) async fn unpark(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.unpark(Ljava/lang/Object;)V")
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

    /// Creates a java.lang.reflect.Field for testing purposes.
    async fn create_field(thread: &Arc<Thread>) -> Result<Value> {
        let vm = thread.vm()?;
        let descriptor =
            "Ljava/lang/Class;Ljava/lang/String;Ljava/lang/Class;IZILjava/lang/String;[B";
        let parameters = vec![
            Value::Object(None),               // Declaring Class
            "fieldName".to_object(&vm).await?, // Field name
            Value::Object(None),               // Type
            Value::Int(0),                     // Modifiers
            Value::from(false),                // Trusted Final
            Value::Int(0),                     // Slot
            "signature".to_object(&vm).await?, // Signature
            Value::Object(None),               // Annotations
        ];
        thread
            .object("java/lang/reflect/Field", descriptor, &parameters)
            .await
    }

    #[tokio::test]
    async fn test_address_size_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = address_size_0(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(8)));
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
        let result = array_index_scale_0(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(1)));
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
        let offset: i64 = value.try_into()?;
        assert_eq!(offset, 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_page_size() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let value = page_size(thread, Parameters::default())
            .await?
            .expect("page_size");
        let page_size: i32 = value.try_into()?;
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
    #[should_panic(expected = "not yet implemented: jdk.internal.misc.Unsafe.park(ZJ)V")]
    async fn test_park() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = park(thread, Parameters::default()).await;
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
        let offset: i64 = value.try_into()?;
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
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.Unsafe.unpark(Ljava/lang/Object;)V"
    )]
    async fn test_unpark() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = unpark(thread, Parameters::default()).await;
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
