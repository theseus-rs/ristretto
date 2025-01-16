use crate::arguments::Arguments;
use crate::native_methods::registry::{MethodRegistry, JAVA_11, JAVA_17};
use crate::thread::Thread;
use crate::Error::{InternalError, InvalidOperand};
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::BaseType;
use ristretto_classloader::{Reference, Value};
use std::sync::Arc;

const CLASS_NAME: &str = "jdk/internal/misc/Unsafe";

/// Register all native methods for `jdk.internal.misc.Unsafe`.
#[expect(clippy::too_many_lines)]
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_11 {
        registry.register(CLASS_NAME, "addressSize0", "()I", address_size_0);
        registry.register(
            CLASS_NAME,
            "compareAndExchangeObject",
            "(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            compare_and_exchange_object,
        );
        registry.register(
            CLASS_NAME,
            "compareAndSetObject",
            "(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z",
            compare_and_set_object,
        );
        registry.register(
            CLASS_NAME,
            "defineAnonymousClass0",
            "(Ljava/lang/Class;[B[Ljava/lang/Object;)Ljava/lang/Class;",
            define_anonymous_class_0,
        );
        registry.register(
            CLASS_NAME,
            "getObject",
            "(Ljava/lang/Object;J)Ljava/lang/Object;",
            get_object,
        );
        registry.register(
            CLASS_NAME,
            "getObjectVolatile",
            "(Ljava/lang/Object;J)Ljava/lang/Object;",
            get_object_volatile,
        );
        registry.register(CLASS_NAME, "isBigEndian0", "()Z", is_big_endian_0);
        registry.register(CLASS_NAME, "pageSize", "()I", page_size);
        registry.register(
            CLASS_NAME,
            "putObject",
            "(Ljava/lang/Object;JLjava/lang/Object;)V",
            put_object,
        );
        registry.register(
            CLASS_NAME,
            "putObjectVolatile",
            "(Ljava/lang/Object;JLjava/lang/Object;)V",
            put_object_volatile,
        );
        registry.register(CLASS_NAME, "unalignedAccess0", "()Z", unaligned_access_0);
    } else {
        registry.register(
            CLASS_NAME,
            "compareAndExchangeReference",
            "(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            compare_and_exchange_reference,
        );
        registry.register(
            CLASS_NAME,
            "compareAndSetReference",
            "(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z",
            compare_and_set_reference,
        );
        registry.register(
            CLASS_NAME,
            "getReference",
            "(Ljava/lang/Object;J)Ljava/lang/Object;",
            get_reference,
        );
        registry.register(
            CLASS_NAME,
            "getReferenceVolatile",
            "(Ljava/lang/Object;J)Ljava/lang/Object;",
            get_reference_volatile,
        );
        registry.register(
            CLASS_NAME,
            "putReference",
            "(Ljava/lang/Object;JLjava/lang/Object;)V",
            put_reference,
        );
        registry.register(
            CLASS_NAME,
            "putReferenceVolatile",
            "(Ljava/lang/Object;JLjava/lang/Object;)V",
            put_reference_volatile,
        );
        registry.register(CLASS_NAME, "writeback0", "(J)V", writeback_0);
        registry.register(
            CLASS_NAME,
            "writebackPostSync0",
            "()V",
            writeback_post_sync_0,
        );
        registry.register(CLASS_NAME, "writebackPreSync0", "()V", writeback_pre_sync_0);
    }

    if registry.java_major_version() <= JAVA_17 {
        registry.register(CLASS_NAME, "loadFence", "()V", load_fence);
        registry.register(CLASS_NAME, "storeFence", "()V", store_fence);
    }

    registry.register(
        CLASS_NAME,
        "allocateInstance",
        "(Ljava/lang/Class;)Ljava/lang/Object;",
        allocate_instance,
    );
    registry.register(CLASS_NAME, "allocateMemory0", "(J)J", allocate_memory_0);
    registry.register(
        CLASS_NAME,
        "arrayBaseOffset0",
        "(Ljava/lang/Class;)I",
        array_base_offset_0,
    );
    registry.register(
        CLASS_NAME,
        "arrayIndexScale0",
        "(Ljava/lang/Class;)I",
        array_index_scale_0,
    );
    registry.register(
        CLASS_NAME,
        "compareAndExchangeInt",
        "(Ljava/lang/Object;JII)I",
        compare_and_exchange_int,
    );
    registry.register(
        CLASS_NAME,
        "compareAndExchangeLong",
        "(Ljava/lang/Object;JJJ)J",
        compare_and_exchange_long,
    );
    registry.register(
        CLASS_NAME,
        "compareAndSetInt",
        "(Ljava/lang/Object;JII)Z",
        compare_and_set_int,
    );
    registry.register(
        CLASS_NAME,
        "compareAndSetLong",
        "(Ljava/lang/Object;JJJ)Z",
        compare_and_set_long,
    );
    registry.register(
        CLASS_NAME,
        "copyMemory0",
        "(Ljava/lang/Object;JLjava/lang/Object;JJ)V",
        copy_memory_0,
    );
    registry.register(
        CLASS_NAME,
        "copySwapMemory0",
        "(Ljava/lang/Object;JLjava/lang/Object;JJJ)V",
        copy_swap_memory_0,
    );
    registry.register(CLASS_NAME, "defineClass0", "(Ljava/lang/String;[BIILjava/lang/ClassLoader;Ljava/security/ProtectionDomain;)Ljava/lang/Class;", define_class_0);
    registry.register(
        CLASS_NAME,
        "ensureClassInitialized0",
        "(Ljava/lang/Class;)V",
        ensure_class_initialized_0,
    );
    registry.register(CLASS_NAME, "freeMemory0", "(J)V", free_memory_0);
    registry.register(CLASS_NAME, "fullFence", "()V", full_fence);
    registry.register(
        CLASS_NAME,
        "getBoolean",
        "(Ljava/lang/Object;J)Z",
        get_boolean,
    );
    registry.register(
        CLASS_NAME,
        "getBooleanVolatile",
        "(Ljava/lang/Object;J)Z",
        get_boolean_volatile,
    );
    registry.register(CLASS_NAME, "getByte", "(Ljava/lang/Object;J)B", get_byte);
    registry.register(
        CLASS_NAME,
        "getByteVolatile",
        "(Ljava/lang/Object;J)B",
        get_byte_volatile,
    );
    registry.register(CLASS_NAME, "getChar", "(Ljava/lang/Object;J)C", get_char);
    registry.register(
        CLASS_NAME,
        "getCharVolatile",
        "(Ljava/lang/Object;J)C",
        get_char_volatile,
    );
    registry.register(
        CLASS_NAME,
        "getDouble",
        "(Ljava/lang/Object;J)D",
        get_double,
    );
    registry.register(
        CLASS_NAME,
        "getDoubleVolatile",
        "(Ljava/lang/Object;J)D",
        get_double_volatile,
    );
    registry.register(CLASS_NAME, "getFloat", "(Ljava/lang/Object;J)F", get_float);
    registry.register(
        CLASS_NAME,
        "getFloatVolatile",
        "(Ljava/lang/Object;J)F",
        get_float_volatile,
    );
    registry.register(CLASS_NAME, "getInt", "(Ljava/lang/Object;J)I", get_int);
    registry.register(
        CLASS_NAME,
        "getIntVolatile",
        "(Ljava/lang/Object;J)I",
        get_int_volatile,
    );
    registry.register(CLASS_NAME, "getLoadAverage0", "([DI)I", get_load_average_0);
    registry.register(CLASS_NAME, "getLong", "(Ljava/lang/Object;J)J", get_long);
    registry.register(
        CLASS_NAME,
        "getLongVolatile",
        "(Ljava/lang/Object;J)J",
        get_long_volatile,
    );
    registry.register(CLASS_NAME, "getShort", "(Ljava/lang/Object;J)S", get_short);
    registry.register(
        CLASS_NAME,
        "getShortVolatile",
        "(Ljava/lang/Object;J)S",
        get_short_volatile,
    );
    registry.register(
        CLASS_NAME,
        "getUncompressedObject",
        "(J)Ljava/lang/Object;",
        get_uncompressed_object,
    );
    registry.register(
        CLASS_NAME,
        "objectFieldOffset0",
        "(Ljava/lang/reflect/Field;)J",
        object_field_offset_0,
    );
    registry.register(
        CLASS_NAME,
        "objectFieldOffset1",
        "(Ljava/lang/Class;Ljava/lang/String;)J",
        object_field_offset_1,
    );
    registry.register(CLASS_NAME, "park", "(ZJ)V", park);
    registry.register(
        CLASS_NAME,
        "putBoolean",
        "(Ljava/lang/Object;JZ)V",
        put_boolean,
    );
    registry.register(
        CLASS_NAME,
        "putBooleanVolatile",
        "(Ljava/lang/Object;JZ)V",
        put_boolean_volatile,
    );
    registry.register(CLASS_NAME, "putByte", "(Ljava/lang/Object;JB)V", put_byte);
    registry.register(
        CLASS_NAME,
        "putByteVolatile",
        "(Ljava/lang/Object;JB)V",
        put_byte_volatile,
    );
    registry.register(CLASS_NAME, "putChar", "(Ljava/lang/Object;JC)V", put_char);
    registry.register(
        CLASS_NAME,
        "putCharVolatile",
        "(Ljava/lang/Object;JC)V",
        put_char_volatile,
    );
    registry.register(
        CLASS_NAME,
        "putDouble",
        "(Ljava/lang/Object;JD)V",
        put_double,
    );
    registry.register(
        CLASS_NAME,
        "putDoubleVolatile",
        "(Ljava/lang/Object;JD)V",
        put_double_volatile,
    );
    registry.register(CLASS_NAME, "putFloat", "(Ljava/lang/Object;JF)V", put_float);
    registry.register(
        CLASS_NAME,
        "putFloatVolatile",
        "(Ljava/lang/Object;JF)V",
        put_float_volatile,
    );
    registry.register(CLASS_NAME, "putInt", "(Ljava/lang/Object;JI)V", put_int);
    registry.register(
        CLASS_NAME,
        "putIntVolatile",
        "(Ljava/lang/Object;JI)V",
        put_int_volatile,
    );
    registry.register(CLASS_NAME, "putLong", "(Ljava/lang/Object;JJ)V", put_long);
    registry.register(
        CLASS_NAME,
        "putLongVolatile",
        "(Ljava/lang/Object;JJ)V",
        put_long_volatile,
    );
    registry.register(CLASS_NAME, "putShort", "(Ljava/lang/Object;JS)V", put_short);
    registry.register(
        CLASS_NAME,
        "putShortVolatile",
        "(Ljava/lang/Object;JS)V",
        put_short_volatile,
    );
    registry.register(
        CLASS_NAME,
        "reallocateMemory0",
        "(JJ)J",
        reallocate_memory_0,
    );
    registry.register(CLASS_NAME, "registerNatives", "()V", register_natives);
    registry.register(
        CLASS_NAME,
        "setMemory0",
        "(Ljava/lang/Object;JJB)V",
        set_memory_0,
    );
    registry.register(
        CLASS_NAME,
        "shouldBeInitialized0",
        "(Ljava/lang/Class;)Z",
        should_be_initialized_0,
    );
    registry.register(
        CLASS_NAME,
        "staticFieldBase0",
        "(Ljava/lang/reflect/Field;)Ljava/lang/Object;",
        static_field_base_0,
    );
    registry.register(
        CLASS_NAME,
        "staticFieldOffset0",
        "(Ljava/lang/reflect/Field;)J",
        static_field_offset_0,
    );
    registry.register(
        CLASS_NAME,
        "throwException",
        "(Ljava/lang/Throwable;)V",
        throw_exception,
    );
    registry.register(CLASS_NAME, "unpark", "(Ljava/lang/Object;)V", unpark);
}

#[async_recursion(?Send)]
pub(crate) async fn address_size_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(8))) // 64-bit pointers
}

#[async_recursion(?Send)]
pub(crate) async fn allocate_instance(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.allocateInstance(Ljava/lang/Class;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
pub(crate) async fn allocate_memory_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.allocateMemory0(J)J")
}

#[async_recursion(?Send)]
pub(crate) async fn array_base_offset_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(0)))
}

#[async_recursion(?Send)]
pub(crate) async fn array_index_scale_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(1)))
}

#[async_recursion(?Send)]
pub(crate) async fn compare_and_exchange_int(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.compareAndExchangeInt(Ljava/lang/Object;JII)I")
}

#[async_recursion(?Send)]
pub(crate) async fn compare_and_exchange_long(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.compareAndExchangeLong(Ljava/lang/Object;JJJ)J")
}

#[async_recursion(?Send)]
pub(crate) async fn compare_and_exchange_object(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.compareAndExchangeObject(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
pub(crate) async fn compare_and_exchange_reference(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.compareAndExchangeReference(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
pub(crate) async fn compare_and_set_int(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let x = arguments.pop_int()?;
    let expected = arguments.pop_int()?;
    let mut offset = arguments.pop()?;
    let Value::Long(ref mut offset) = offset else {
        return Err(InvalidOperand {
            expected: "long".to_string(),
            actual: offset.to_string(),
        });
    };

    // TODO: the compare and set operation should be atomic
    let result = if let Some(Reference::Object(object)) = arguments.pop_reference()? {
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

#[async_recursion(?Send)]
pub(crate) async fn compare_and_set_long(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let x = arguments.pop_long()?;
    let expected = arguments.pop_long()?;
    let mut offset = arguments.pop()?;
    let Value::Long(ref mut offset) = offset else {
        return Err(InvalidOperand {
            expected: "long".to_string(),
            actual: offset.to_string(),
        });
    };

    // TODO: the compare and set operation should be atomic
    let result = if let Some(Reference::Object(object)) = arguments.pop_reference()? {
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

#[async_recursion(?Send)]
pub(crate) async fn compare_and_set_object(
    thread: Arc<Thread>,
    arguments: Arguments,
) -> Result<Option<Value>> {
    compare_and_set_reference(thread, arguments).await
}

#[async_recursion(?Send)]
pub(crate) async fn compare_and_set_reference(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let x = arguments.pop()?;
    let expected = arguments.pop()?;
    let offset = arguments.pop_long()?;
    let offset = usize::try_from(offset)?;
    let Some(object) = arguments.pop_reference()? else {
        return Err(InternalError(
            "compareAndSetReference: Invalid reference".to_string(),
        ));
    };

    // TODO: the compare and set operation should be atomic
    let result = match object {
        Reference::Array(_class, array) => {
            let Some(reference) = array.get(offset)? else {
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
                array.set(offset, x_reference)?;
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

#[async_recursion(?Send)]
pub(crate) async fn copy_memory_0(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let _bytes = usize::try_from(arguments.pop_long()?)?;
    let _destination_offset = usize::try_from(arguments.pop_long()?)?;
    let Value::Object(ref mut destination) = arguments.pop()? else {
        return Err(InternalError(
            "copyMemory0: Invalid destination".to_string(),
        ));
    };
    let _source_offset = usize::try_from(arguments.pop_long()?)?;
    let Value::Object(ref mut source) = arguments.pop()? else {
        return Err(InternalError("copyMemory0: Invalid source".to_string()));
    };
    destination.clone_from(source);
    Ok(None)
}

#[async_recursion(?Send)]
pub(crate) async fn copy_swap_memory_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.copySwapMemory0(Ljava/lang/Object;JLjava/lang/Object;JJJ)V")
}

#[async_recursion(?Send)]
pub(crate) async fn define_anonymous_class_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.defineAnonymousClass0(Ljava/lang/Class;[B[Ljava/lang/Object;)Ljava/lang/Class;")
}

#[async_recursion(?Send)]
pub(crate) async fn define_class_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.defineClass0(Ljava/lang/String;[BIILjava/lang/ClassLoader;Ljava/security/ProtectionDomain;)Ljava/lang/Class;")
}

#[async_recursion(?Send)]
pub(crate) async fn ensure_class_initialized_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
pub(crate) async fn free_memory_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
pub(crate) async fn full_fence(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(None)
}

#[expect(clippy::too_many_lines)]
fn get_reference_type(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
    base_type: Option<BaseType>,
) -> Result<Option<Value>> {
    let offset = arguments.pop_long()?;
    let Some(reference) = arguments.pop_reference()? else {
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
                    "getReferenceType: Invalid reference".to_string(),
                ));
            }
        };
        return Ok(Some(value));
    };

    let offset = usize::try_from(offset)?;
    let value = match reference {
        Reference::ByteArray(array) => {
            let Some(byte) = array.get(offset)? else {
                return Err(InternalError(
                    "getReferenceType: Invalid byte reference index".to_string(),
                ));
            };
            if matches!(base_type, Some(BaseType::Long)) {
                Value::Long(i64::from(byte))
            } else {
                Value::Int(i32::from(byte))
            }
        }
        Reference::CharArray(array) => {
            let Some(char) = array.get(offset)? else {
                return Err(InternalError(
                    "getReferenceType: Invalid char reference index".to_string(),
                ));
            };
            if matches!(base_type, Some(BaseType::Long)) {
                Value::Long(i64::from(char))
            } else {
                Value::Int(i32::from(char))
            }
        }
        Reference::ShortArray(array) => {
            let Some(short) = array.get(offset)? else {
                return Err(InternalError(
                    "getReferenceType: Invalid short reference index".to_string(),
                ));
            };
            if matches!(base_type, Some(BaseType::Long)) {
                Value::Long(i64::from(short))
            } else {
                Value::Int(i32::from(short))
            }
        }
        Reference::IntArray(array) => {
            let Some(int) = array.get(offset)? else {
                return Err(InternalError(
                    "getReferenceType: Invalid int reference index".to_string(),
                ));
            };
            if matches!(base_type, Some(BaseType::Long)) {
                Value::Long(i64::from(int))
            } else {
                Value::Int(int)
            }
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
        Reference::Array(_class, array) => {
            let Some(reference) = array.get(offset)? else {
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

#[async_recursion(?Send)]
pub(crate) async fn get_boolean(
    thread: Arc<Thread>,
    arguments: Arguments,
) -> Result<Option<Value>> {
    get_boolean_volatile(thread, arguments).await
}

#[async_recursion(?Send)]
pub(crate) async fn get_boolean_volatile(
    thread: Arc<Thread>,
    arguments: Arguments,
) -> Result<Option<Value>> {
    get_reference_type(thread, arguments, Some(BaseType::Boolean))
}

#[async_recursion(?Send)]
pub(crate) async fn get_byte(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    get_byte_volatile(thread, arguments).await
}

#[async_recursion(?Send)]
pub(crate) async fn get_byte_volatile(
    thread: Arc<Thread>,
    arguments: Arguments,
) -> Result<Option<Value>> {
    get_reference_type(thread, arguments, Some(BaseType::Byte))
}

#[async_recursion(?Send)]
pub(crate) async fn get_char(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    get_char_volatile(thread, arguments).await
}

#[async_recursion(?Send)]
pub(crate) async fn get_char_volatile(
    thread: Arc<Thread>,
    arguments: Arguments,
) -> Result<Option<Value>> {
    get_reference_type(thread, arguments, Some(BaseType::Char))
}

#[async_recursion(?Send)]
pub(crate) async fn get_double(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    get_double_volatile(thread, arguments).await
}

#[async_recursion(?Send)]
pub(crate) async fn get_double_volatile(
    thread: Arc<Thread>,
    arguments: Arguments,
) -> Result<Option<Value>> {
    get_reference_type(thread, arguments, Some(BaseType::Double))
}

#[async_recursion(?Send)]
pub(crate) async fn get_float(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    get_float_volatile(thread, arguments).await
}

#[async_recursion(?Send)]
pub(crate) async fn get_float_volatile(
    thread: Arc<Thread>,
    arguments: Arguments,
) -> Result<Option<Value>> {
    get_reference_type(thread, arguments, Some(BaseType::Float))
}

#[async_recursion(?Send)]
pub(crate) async fn get_int(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    get_int_volatile(thread, arguments).await
}

#[async_recursion(?Send)]
pub(crate) async fn get_int_volatile(
    thread: Arc<Thread>,
    arguments: Arguments,
) -> Result<Option<Value>> {
    get_reference_type(thread, arguments, Some(BaseType::Int))
}

#[async_recursion(?Send)]
pub(crate) async fn get_load_average_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.getLoadAverage0([DI)I")
}

#[async_recursion(?Send)]
pub(crate) async fn get_long(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    get_long_volatile(thread, arguments).await
}

#[async_recursion(?Send)]
pub(crate) async fn get_long_volatile(
    thread: Arc<Thread>,
    arguments: Arguments,
) -> Result<Option<Value>> {
    get_reference_type(thread, arguments, Some(BaseType::Long))
}

#[async_recursion(?Send)]
pub(crate) async fn get_object(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    get_object_volatile(thread, arguments).await
}

#[async_recursion(?Send)]
pub(crate) async fn get_object_volatile(
    thread: Arc<Thread>,
    arguments: Arguments,
) -> Result<Option<Value>> {
    get_reference_type(thread, arguments, None)
}

#[async_recursion(?Send)]
pub(crate) async fn get_reference(
    thread: Arc<Thread>,
    arguments: Arguments,
) -> Result<Option<Value>> {
    get_reference_volatile(thread, arguments).await
}

#[async_recursion(?Send)]
pub(crate) async fn get_reference_volatile(
    thread: Arc<Thread>,
    arguments: Arguments,
) -> Result<Option<Value>> {
    get_reference_type(thread, arguments, None)
}

#[async_recursion(?Send)]
pub(crate) async fn get_short(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    get_short_volatile(thread, arguments).await
}

#[async_recursion(?Send)]
pub(crate) async fn get_short_volatile(
    thread: Arc<Thread>,
    arguments: Arguments,
) -> Result<Option<Value>> {
    get_reference_type(thread, arguments, Some(BaseType::Short))
}

#[async_recursion(?Send)]
pub(crate) async fn get_uncompressed_object(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.getUncompressedObject(J)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
pub(crate) async fn is_big_endian_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    let big_endian = cfg!(target_endian = "big");
    Ok(Some(Value::from(big_endian)))
}

#[async_recursion(?Send)]
pub(crate) async fn load_fence(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
pub(crate) async fn object_field_offset_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(Some(Value::Long(0)))
}

#[async_recursion(?Send)]
pub(crate) async fn object_field_offset_1(
    thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let value = arguments.pop()?;
    let field_name: String = match value {
        Value::Object(_) => value.try_into()?,
        value => {
            return Err(InvalidOperand {
                expected: "object".to_string(),
                actual: value.to_string(),
            });
        }
    };
    let Some(Reference::Object(class_object)) = arguments.pop_reference()? else {
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

#[async_recursion(?Send)]
pub(crate) async fn page_size(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.pageSize()I")
}

#[async_recursion(?Send)]
pub(crate) async fn park(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.park(ZJ)V")
}

#[async_recursion(?Send)]
pub(crate) async fn put_boolean(
    thread: Arc<Thread>,
    arguments: Arguments,
) -> Result<Option<Value>> {
    put_boolean_volatile(thread, arguments).await
}

#[async_recursion(?Send)]
pub(crate) async fn put_boolean_volatile(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let x = arguments.pop_int()? != 0;
    let offset = usize::try_from(arguments.pop_long()?)?;
    let Value::Object(ref mut object) = arguments.pop()? else {
        return Err(InternalError("putBoolean: Invalid reference".to_string()));
    };
    let bytes = Reference::from(vec![x; offset]);
    *object = Some(bytes);
    Ok(None)
}

#[async_recursion(?Send)]
pub(crate) async fn put_byte(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    put_byte_volatile(thread, arguments).await
}

#[async_recursion(?Send)]
pub(crate) async fn put_byte_volatile(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let x = i8::try_from(arguments.pop_int()?)?;
    let offset = usize::try_from(arguments.pop_long()?)?;
    let Value::Object(ref mut object) = arguments.pop()? else {
        return Err(InternalError("putByte: Invalid reference".to_string()));
    };
    let bytes = Reference::from(vec![x; offset]);
    *object = Some(bytes);
    Ok(None)
}

#[async_recursion(?Send)]
pub(crate) async fn put_char(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    put_char_volatile(thread, arguments).await
}

#[async_recursion(?Send)]
pub(crate) async fn put_char_volatile(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    #[expect(clippy::cast_sign_loss)]
    let x = arguments.pop_int()? as u32;
    let Some(x) = char::from_u32(x) else {
        return Err(InternalError("putChar: Invalid character".to_string()));
    };
    let offset = usize::try_from(arguments.pop_long()?)?;
    let Value::Object(ref mut object) = arguments.pop()? else {
        return Err(InternalError("putChar: Invalid reference".to_string()));
    };
    let bytes = Reference::from(vec![x; offset]);
    *object = Some(bytes);
    Ok(None)
}

#[async_recursion(?Send)]
pub(crate) async fn put_double(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    put_double_volatile(thread, arguments).await
}

#[async_recursion(?Send)]
pub(crate) async fn put_double_volatile(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let x = arguments.pop_double()?;
    let offset = usize::try_from(arguments.pop_long()?)?;
    let Value::Object(ref mut object) = arguments.pop()? else {
        return Err(InternalError("putDouble: Invalid reference".to_string()));
    };
    let bytes = Reference::from(vec![x; offset]);
    *object = Some(bytes);
    Ok(None)
}

#[async_recursion(?Send)]
pub(crate) async fn put_float(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    put_float_volatile(thread, arguments).await
}

#[async_recursion(?Send)]
pub(crate) async fn put_float_volatile(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let x = arguments.pop_float()?;
    let offset = usize::try_from(arguments.pop_long()?)?;
    let Value::Object(ref mut object) = arguments.pop()? else {
        return Err(InternalError("putFloat: Invalid reference".to_string()));
    };
    let bytes = Reference::from(vec![x; offset]);
    *object = Some(bytes);
    Ok(None)
}

#[async_recursion(?Send)]
pub(crate) async fn put_int(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    put_int_volatile(thread, arguments).await
}

#[async_recursion(?Send)]
pub(crate) async fn put_int_volatile(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let x = arguments.pop_int()?;
    let offset = usize::try_from(arguments.pop_long()?)?;
    let Value::Object(ref mut object) = arguments.pop()? else {
        return Err(InternalError("putInt: Invalid reference".to_string()));
    };
    let bytes = Reference::from(vec![x; offset]);
    *object = Some(bytes);
    Ok(None)
}

#[async_recursion(?Send)]
pub(crate) async fn put_long(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    put_long_volatile(thread, arguments).await
}

#[async_recursion(?Send)]
pub(crate) async fn put_long_volatile(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let x = arguments.pop_long()?;
    let offset = usize::try_from(arguments.pop_long()?)?;
    let Value::Object(ref mut object) = arguments.pop()? else {
        return Err(InternalError("putlong: Invalid reference".to_string()));
    };
    let bytes = Reference::from(vec![x; offset]);
    *object = Some(bytes);
    Ok(None)
}

#[async_recursion(?Send)]
pub(crate) async fn put_object(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    put_object_volatile(thread, arguments).await
}

#[async_recursion(?Send)]
pub(crate) async fn put_object_volatile(
    thread: Arc<Thread>,
    arguments: Arguments,
) -> Result<Option<Value>> {
    put_reference_volatile(thread, arguments).await
}

#[async_recursion(?Send)]
pub(crate) async fn put_reference(
    thread: Arc<Thread>,
    arguments: Arguments,
) -> Result<Option<Value>> {
    put_reference_volatile(thread, arguments).await
}

#[async_recursion(?Send)]
pub(crate) async fn put_reference_volatile(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let x = arguments.pop()?;
    let offset = arguments.pop_long()?;
    let offset = usize::try_from(offset)?;
    let Some(object) = arguments.pop_reference()? else {
        return Err(InternalError(
            "putReferenceVolatile: Invalid reference".to_string(),
        ));
    };
    match object {
        Reference::Array(_class, array) => {
            let x = x.to_reference()?;
            array.set(offset, x)?;
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

#[async_recursion(?Send)]
pub(crate) async fn put_short(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    put_short_volatile(thread, arguments).await
}

#[async_recursion(?Send)]
pub(crate) async fn put_short_volatile(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let x = i16::try_from(arguments.pop_int()?)?;
    let offset = usize::try_from(arguments.pop_long()?)?;
    let Value::Object(ref mut object) = arguments.pop()? else {
        return Err(InternalError("putShort: Invalid reference".to_string()));
    };
    let bytes = Reference::from(vec![x; offset]);
    *object = Some(bytes);
    Ok(None)
}

#[async_recursion(?Send)]
pub(crate) async fn reallocate_memory_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.reallocateMemory0(JJ)J")
}

#[async_recursion(?Send)]
pub(crate) async fn register_natives(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
pub(crate) async fn set_memory_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.setMemory0(Ljava/lang/Object;JJB)V")
}

#[async_recursion(?Send)]
pub(crate) async fn should_be_initialized_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    // Classes are always initialized
    Ok(Some(Value::from(false)))
}

#[async_recursion(?Send)]
pub(crate) async fn static_field_base_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.staticFieldBase0(Ljava/lang/reflect/Field;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
pub(crate) async fn static_field_offset_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.staticFieldOffset0(Ljava/lang/reflect/Field;)J")
}

#[async_recursion(?Send)]
pub(crate) async fn store_fence(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
pub(crate) async fn throw_exception(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.throwException(Ljava/lang/Throwable;)V")
}

#[async_recursion(?Send)]
pub(crate) async fn unaligned_access_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
}

#[async_recursion(?Send)]
pub(crate) async fn unpark(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.unpark(Ljava/lang/Object;)V")
}

#[async_recursion(?Send)]
pub(crate) async fn writeback_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.writeback0(J)V")
}

#[async_recursion(?Send)]
pub(crate) async fn writeback_post_sync_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.writebackPostSync0()V")
}

#[async_recursion(?Send)]
pub(crate) async fn writeback_pre_sync_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.Unsafe.writebackPreSync0()V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_address_size_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = address_size_0(thread, Arguments::default()).await?;
        assert_eq!(result, Some(Value::Int(8)));
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.Unsafe.allocateInstance(Ljava/lang/Class;)Ljava/lang/Object;"
    )]
    async fn test_allocate_instance() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = allocate_instance(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.internal.misc.Unsafe.allocateMemory0(J)J")]
    async fn test_allocate_memory_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = allocate_memory_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_array_base_offset_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = array_base_offset_0(thread, Arguments::default()).await?;
        assert_eq!(result, Some(Value::Int(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_array_index_scale_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = array_index_scale_0(thread, Arguments::default()).await?;
        assert_eq!(result, Some(Value::Int(1)));
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.Unsafe.compareAndExchangeInt(Ljava/lang/Object;JII)I"
    )]
    async fn test_compare_and_exchange_int() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = compare_and_exchange_int(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.Unsafe.compareAndExchangeLong(Ljava/lang/Object;JJJ)J"
    )]
    async fn test_compare_and_exchange_long() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = compare_and_exchange_long(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.Unsafe.compareAndExchangeObject(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_compare_and_exchange_object() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = compare_and_exchange_object(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.Unsafe.compareAndExchangeReference(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_compare_and_exchange_reference() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = compare_and_exchange_reference(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.Unsafe.copySwapMemory0(Ljava/lang/Object;JLjava/lang/Object;JJJ)V"
    )]
    async fn test_copy_swap_memory_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = copy_swap_memory_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.Unsafe.defineAnonymousClass0(Ljava/lang/Class;[B[Ljava/lang/Object;)Ljava/lang/Class;"
    )]
    async fn test_define_anonymous_class_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = define_anonymous_class_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.Unsafe.defineClass0(Ljava/lang/String;[BIILjava/lang/ClassLoader;Ljava/security/ProtectionDomain;)Ljava/lang/Class;"
    )]
    async fn test_define_class_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = define_class_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_ensure_class_initialized_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = ensure_class_initialized_0(thread, Arguments::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_free_memory_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = free_memory_0(thread, Arguments::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_full_fence() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = full_fence(thread, Arguments::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.Unsafe.getLoadAverage0([DI)I"
    )]
    async fn test_get_load_average_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_load_average_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.Unsafe.getUncompressedObject(J)Ljava/lang/Object;"
    )]
    async fn test_get_uncompressed_object() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_uncompressed_object(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_is_big_endian_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = is_big_endian_0(thread, Arguments::default()).await?;
        let big_endian = cfg!(target_endian = "big");
        assert_eq!(result, Some(Value::from(big_endian)));
        Ok(())
    }

    #[tokio::test]
    async fn test_load_fence() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = load_fence(thread, Arguments::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_object_field_offset_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = object_field_offset_0(thread, Arguments::default()).await?;
        assert_eq!(result, Some(Value::Long(0)));
        Ok(())
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.internal.misc.Unsafe.pageSize()I")]
    async fn test_page_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = page_size(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.internal.misc.Unsafe.park(ZJ)V")]
    async fn test_park() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = park(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.Unsafe.reallocateMemory0(JJ)J"
    )]
    async fn test_reallocate_memory_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = reallocate_memory_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = register_natives(thread, Arguments::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.Unsafe.setMemory0(Ljava/lang/Object;JJB)V"
    )]
    async fn test_set_memory_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_memory_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_should_be_initialized_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = should_be_initialized_0(thread, Arguments::default()).await?;
        assert_eq!(result, Some(Value::from(false)));
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.Unsafe.staticFieldBase0(Ljava/lang/reflect/Field;)Ljava/lang/Object;"
    )]
    async fn test_static_field_base_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = static_field_base_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.Unsafe.staticFieldOffset0(Ljava/lang/reflect/Field;)J"
    )]
    async fn test_static_field_offset_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = static_field_offset_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_store_fence() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = store_fence(thread, Arguments::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.Unsafe.throwException(Ljava/lang/Throwable;)V"
    )]
    async fn test_throw_exception() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = throw_exception(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_unaligned_access_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = unaligned_access_0(thread, Arguments::default()).await?;
        assert_eq!(result, Some(Value::from(false)));
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.Unsafe.unpark(Ljava/lang/Object;)V"
    )]
    async fn test_unpark() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = unpark(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.internal.misc.Unsafe.writeback0(J)V")]
    async fn test_writeback_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = writeback_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.Unsafe.writebackPostSync0()V"
    )]
    async fn test_writeback_post_sync_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = writeback_post_sync_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.internal.misc.Unsafe.writebackPreSync0()V")]
    async fn test_writeback_pre_sync_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = writeback_pre_sync_0(thread, Arguments::default()).await;
    }
}
