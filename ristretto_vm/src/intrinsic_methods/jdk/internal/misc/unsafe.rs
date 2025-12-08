use crate::Error::{InternalError, InvalidOperand};
use crate::JavaError::{ArrayIndexOutOfBoundsException, ClassFormatError};
use crate::Result;
use crate::intrinsic_methods::java::lang::class::get_class;
use crate::java_object::JavaObject;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::ClassFile;
use ristretto_classfile::VersionSpecification::{Between, Equal, GreaterThan, GreaterThanOrEqual};
use ristretto_classfile::{BaseType, JAVA_11, JAVA_17};
use ristretto_classloader::{Class, Object, Reference, Value};
use ristretto_gc::Gc;
use ristretto_macros::intrinsic_method;
use std::io::Cursor;
use std::sync::Arc;
use zerocopy::transmute_ref;

pub(crate) const BOOLEAN_SIZE: usize = 1;
pub(crate) const BYTE_SIZE: usize = 1;
pub(crate) const CHAR_SIZE: usize = 2;
pub(crate) const SHORT_SIZE: usize = 2;
pub(crate) const INT_SIZE: usize = 4;
pub(crate) const LONG_SIZE: usize = 8;
pub(crate) const FLOAT_SIZE: usize = 4;
pub(crate) const DOUBLE_SIZE: usize = 8;
pub(crate) const STATIC_FIELD_OFFSET_MASK: i64 = 1 << 62;

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
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let class_object = parameters.pop()?;
    let class = get_class(&thread, &class_object).await?;
    let object = Object::new(class)?;
    Ok(Some(Value::from(object)))
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.allocateMemory0(J)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn allocate_memory_0(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let bytes = parameters.pop_long()?;
    if bytes < 0 {
        return Err(crate::JavaError::IllegalArgumentException(format!(
            "Invalid memory allocation size: {bytes}"
        ))
        .into());
    }
    Ok(Some(Value::Long(0)))
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
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let new_value = parameters.pop_int()?;
    let expected_value = parameters.pop_int()?;
    let mut offset = parameters.pop()?;
    let Value::Long(ref mut offset) = offset else {
        return Err(InvalidOperand {
            expected: "long".to_string(),
            actual: offset.to_string(),
        });
    };

    let value = parameters.pop()?;
    let current_value = if value.is_object() {
        let mut object = value.as_object_mut()?;
        let offset_long = *offset;
        if offset_long & STATIC_FIELD_OFFSET_MASK != 0 {
            let offset = usize::try_from(offset_long & !STATIC_FIELD_OFFSET_MASK)?;
            let class_name = object.value("name")?.as_string()?;
            let class = thread.class(&class_name).await?;
            let field_name = class.field_name(offset)?;
            let value_object = class.static_value(&field_name)?;
            let current_value = if let Value::Object(None) = value_object {
                0
            } else {
                value_object.as_i32()?
            };
            if current_value == expected_value {
                class.set_static_value(&field_name, Value::Int(new_value))?;
            }
            current_value
        } else {
            let class = object.class();
            let offset = usize::try_from(offset_long)?;
            let field_name = class.field_name(offset)?;
            let value_object = object.value(&field_name)?;
            let current_value = if let Value::Object(None) = value_object {
                0
            } else {
                value_object.as_i32()?
            };
            if current_value == expected_value {
                object.set_value(&field_name, Value::Int(new_value))?;
            }
            current_value
        }
    } else {
        let current_value = i32::try_from(*offset)?;
        if current_value == expected_value {
            *offset = i64::from(new_value);
        }
        current_value
    };
    Ok(Some(Value::Int(current_value)))
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.compareAndExchangeLong(Ljava/lang/Object;JJJ)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn compare_and_exchange_long(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let new_value = parameters.pop_long()?;
    let expected_value = parameters.pop_long()?;
    let mut offset = parameters.pop()?;
    let Value::Long(ref mut offset) = offset else {
        return Err(InvalidOperand {
            expected: "long".to_string(),
            actual: offset.to_string(),
        });
    };

    let value = parameters.pop()?;
    let current_value = if value.is_object() {
        let mut object = value.as_object_mut()?;
        let offset_long = *offset;
        if offset_long & STATIC_FIELD_OFFSET_MASK != 0 {
            let offset = usize::try_from(offset_long & !STATIC_FIELD_OFFSET_MASK)?;
            let class_name = object.value("name")?.as_string()?;
            let class = thread.class(&class_name).await?;
            let field_name = class.field_name(offset)?;
            let value_object = class.static_value(&field_name)?;
            let current_value = if let Value::Object(None) = value_object {
                0
            } else {
                value_object.as_i64()?
            };
            if current_value == expected_value {
                class.set_static_value(&field_name, Value::Long(new_value))?;
            }
            current_value
        } else {
            let class = object.class();
            let offset = usize::try_from(offset_long)?;
            let field_name = class.field_name(offset)?;
            let value_object = object.value(&field_name)?;
            let current_value = if let Value::Object(None) = value_object {
                0
            } else {
                value_object.as_i64()?
            };
            if current_value == expected_value {
                object.set_value(&field_name, Value::Long(new_value))?;
            }
            current_value
        }
    } else {
        let current_value = *offset;
        if current_value == expected_value {
            *offset = new_value;
        }
        current_value
    };
    Ok(Some(Value::Long(current_value)))
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.compareAndExchangeObject(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
    Equal(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn compare_and_exchange_object(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    compare_and_exchange_reference(thread, parameters).await
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.compareAndExchangeReference(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThan(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn compare_and_exchange_reference(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let new_value = parameters.pop()?;
    let expected_value = parameters.pop()?;
    let offset = parameters.pop_long()?;
    let offset = usize::try_from(offset & !STATIC_FIELD_OFFSET_MASK)?;
    let obj = parameters.pop()?;
    let mut obj = obj.as_reference_mut()?;

    let result = match &mut *obj {
        Reference::Object(object) => {
            let field_name = object.class().field_name(offset)?;
            let current_value = object.value(&field_name)?;

            let equal = match (&current_value, &expected_value) {
                (Value::Object(Some(r1)), Value::Object(Some(r2))) => Gc::ptr_eq(r1, r2),
                (Value::Object(None), Value::Object(None)) => true,
                _ => false,
            };

            if equal {
                object.set_value(&field_name, new_value)?;
            }
            current_value
        }
        Reference::Array(object_array) => {
            let offset = offset / REFERENCE_SIZE;
            let elements = &mut object_array.elements;
            let Some(current_value) = elements.get(offset) else {
                return Err(InternalError("Invalid array index".to_string()));
            };
            let current_value = current_value.clone();

            let equal = match (&current_value, &expected_value) {
                (Value::Object(Some(r1)), Value::Object(Some(r2))) => Gc::ptr_eq(r1, r2),
                (Value::Object(None), Value::Object(None)) => true,
                _ => false,
            };

            if equal {
                elements[offset] = new_value;
            }
            current_value
        }
        _ => return Err(InternalError("Invalid reference type".to_string())),
    };
    Ok(Some(result))
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.compareAndSetInt(Ljava/lang/Object;JII)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn compare_and_set_int(
    thread: Arc<Thread>,
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
        let offset_long = *offset;
        if offset_long & STATIC_FIELD_OFFSET_MASK != 0 {
            let offset = usize::try_from(offset_long & !STATIC_FIELD_OFFSET_MASK)?;
            let class_name = object.value("name")?.as_string()?;
            let class = thread.class(&class_name).await?;
            let field_name = class.field_name(offset)?;
            let value_object = class.static_value(&field_name)?;
            let value = if let Value::Object(None) = value_object {
                0
            } else {
                value_object.as_i32()?
            };
            if value == expected {
                class.set_static_value(&field_name, Value::Int(x))?;
                1
            } else {
                0
            }
        } else {
            let class = object.class();
            let offset = usize::try_from(offset_long)?;
            let field_name = class.field_name(offset)?;
            let value_object = object.value(&field_name)?;
            let value = if let Value::Object(None) = value_object {
                0
            } else {
                value_object.as_i32()?
            };
            if value == expected {
                object.set_value(&field_name, Value::Int(x))?;
                1
            } else {
                0
            }
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
    thread: Arc<Thread>,
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
        let offset_long = *offset;
        if offset_long & STATIC_FIELD_OFFSET_MASK != 0 {
            let offset = usize::try_from(offset_long & !STATIC_FIELD_OFFSET_MASK)?;
            let class_name = object.value("name")?.as_string()?;
            let class = thread.class(&class_name).await?;
            let field_name = class.field_name(offset)?;
            let value_object = class.static_value(&field_name)?;
            let value = if let Value::Object(None) = value_object {
                0
            } else {
                value_object.as_i64()?
            };
            if value == expected {
                class.set_static_value(&field_name, Value::Long(x))?;
                1
            } else {
                0
            }
        } else {
            let class = object.class();
            let offset = usize::try_from(offset_long)?;
            let field_name = class.field_name(offset)?;
            let value_object = object.value(&field_name)?;
            let value = if let Value::Object(None) = value_object {
                0
            } else {
                value_object.as_i64()?
            };
            if value == expected {
                object.set_value(&field_name, Value::Long(x))?;
                1
            } else {
                0
            }
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
    let offset = usize::try_from(offset & !STATIC_FIELD_OFFSET_MASK)?;
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
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let element_size = parameters.pop_long()?;
    let bytes = parameters.pop_long()?;
    let dest_offset = parameters.pop_long()?;
    let dest = parameters.pop()?;
    let src_offset = parameters.pop_long()?;
    let src = parameters.pop()?;

    let element_size = usize::try_from(element_size)?;
    let bytes = usize::try_from(bytes)?;
    let dest_offset = usize::try_from(dest_offset)?;
    let src_offset = usize::try_from(src_offset)?;

    // Validate element size (must be 2, 4, or 8)
    if element_size != 2 && element_size != 4 && element_size != 8 {
        return Err(crate::JavaError::IllegalArgumentException(format!(
            "Invalid element size: {element_size}"
        ))
        .into());
    }

    // Get source bytes
    let src_bytes: Vec<u8> = {
        let src_ref = src.as_reference()?;
        let Some(bytes) = src_ref.as_bytes() else {
            return Err(InternalError(
                "copySwapMemory0: Invalid source type".to_string(),
            ));
        };
        bytes.to_vec()
    };

    // Validate source bounds
    if src_offset + bytes > src_bytes.len() {
        return Err(ArrayIndexOutOfBoundsException {
            index: i32::try_from(src_offset + bytes)?,
            length: src_bytes.len(),
        }
        .into());
    }

    // Swap bytes according to element size
    let src_slice = &src_bytes[src_offset..src_offset + bytes];
    let mut swapped = Vec::with_capacity(bytes);
    for chunk in src_slice.chunks(element_size) {
        swapped.extend(chunk.iter().rev());
    }

    // Write to destination
    let mut dest_ref = dest.as_reference_mut()?;
    let Some(dest_slice) = dest_ref.as_bytes_mut() else {
        return Err(InternalError(
            "copySwapMemory0: Invalid destination type".to_string(),
        ));
    };

    if dest_offset + bytes > dest_slice.len() {
        return Err(ArrayIndexOutOfBoundsException {
            index: i32::try_from(dest_offset + bytes)?,
            length: dest_slice.len(),
        }
        .into());
    }
    dest_slice[dest_offset..dest_offset + bytes].copy_from_slice(&swapped);

    Ok(None)
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.defineAnonymousClass0(Ljava/lang/Class;[B[Ljava/lang/Object;)Ljava/lang/Class;",
    Equal(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn define_anonymous_class_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _cp_patches = parameters.pop()?;
    let bytes = parameters.pop()?;
    let host_class_object = parameters.pop()?;
    let host_class = get_class(&thread, &host_class_object).await?;
    tracing::info!(
        "defineAnonymousClass0 called for host: {}",
        host_class.name()
    );

    let bytes = {
        let bytes = bytes.as_byte_vec_ref()?;
        let bytes: &[u8] = transmute_ref!(&*bytes);
        bytes.to_vec()
    };

    let mut cursor = Cursor::new(bytes);
    let class_file = match ClassFile::from_bytes(&mut cursor) {
        Ok(class_file) => class_file,
        Err(error) => {
            tracing::error!("ClassFormatError in defineAnonymousClass0: {}", error);
            return Err(ClassFormatError(error.to_string()).into());
        }
    };
    tracing::info!(
        "Defined anonymous class name: {:?}",
        class_file.class_name()
    );

    // TODO: Apply constant pool patches
    // TODO: Set host class (nest host)

    let class_loader = host_class.class_loader()?;
    let class_loader = class_loader.map(|cl| Arc::downgrade(&cl));
    let class = match Class::from(class_loader, class_file) {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Class::from failed in defineAnonymousClass0: {:?}", e);
            return Err(InternalError(format!("Class::from failed: {}", e)));
        }
    };
    let class_object = class.to_object(&thread).await?;
    Ok(Some(class_object))
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.defineClass0(Ljava/lang/String;[BIILjava/lang/ClassLoader;Ljava/security/ProtectionDomain;)Ljava/lang/Class;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn define_class_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _protection_domain = parameters.pop()?;
    let class_loader = parameters.pop()?;
    let length = parameters.pop_int()?;
    let offset = parameters.pop_int()?;
    let bytes = parameters.pop()?;
    let name = parameters.pop()?;

    tracing::info!("defineClass0 called");

    let bytes = {
        let bytes = bytes.as_byte_vec_ref()?;
        let bytes: &[u8] = transmute_ref!(&*bytes);
        bytes.to_vec()
    };
    let offset = usize::try_from(offset)?;
    let length = usize::try_from(length)?;

    let mut cursor = Cursor::new(bytes[offset..offset + length].to_vec());
    let class_file = match ClassFile::from_bytes(&mut cursor) {
        Ok(class_file) => class_file,
        Err(error) => {
            tracing::error!("ClassFormatError in defineClass0: {}", error);
            return Err(ClassFormatError(error.to_string()).into());
        }
    };

    if !name.is_null() {
        let expected_name = name.as_string()?;
        // TODO: Verify name matches class_file.class_name()
        tracing::info!("Defining class: {}", expected_name);
    } else {
        tracing::info!(
            "Defining class (no name provided): {:?}",
            class_file.class_name()
        );
    }

    let class = Class::from(None, class_file)?;
    let class_object = class.to_object(&thread).await?;

    if !class_loader.is_null() {
        let mut object = class_object.as_object_mut()?;
        object.set_value("classLoader", class_loader)?;
    }

    Ok(Some(class_object))
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.defineHiddenClass(Ljava/lang/String;[BIILjava/lang/ClassLoader;Ljava/security/ProtectionDomain;ZILjava/lang/Object;)Ljava/lang/Class;",
    GreaterThan(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn define_hidden_class(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _class_data = parameters.pop()?;
    let _flags = parameters.pop_int()?;
    let _initialize = parameters.pop_bool()?;
    let _protection_domain = parameters.pop()?;
    let class_loader = parameters.pop()?;
    let length = parameters.pop_int()?;
    let offset = parameters.pop_int()?;
    let bytes = parameters.pop()?;
    let _name = parameters.pop()?;

    let bytes = {
        let bytes = bytes.as_byte_vec_ref()?;
        let bytes: &[u8] = transmute_ref!(&*bytes);
        bytes.to_vec()
    };
    let offset = usize::try_from(offset)?;
    let length = usize::try_from(length)?;

    let mut cursor = Cursor::new(bytes[offset..offset + length].to_vec());
    let class_file =
        ClassFile::from_bytes(&mut cursor).map_err(|e| ClassFormatError(e.to_string()))?;

    let class = Class::from(None, class_file)?;
    let class_object = class.to_object(&thread).await?;

    if !class_loader.is_null() {
        let mut obj = class_object.as_object_mut()?;
        obj.set_value("classLoader", class_loader)?;
    }

    Ok(Some(class_object))
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

async fn get_static_field_target(
    thread: &Arc<Thread>,
    reference: &Reference,
    offset: usize,
) -> Result<(Arc<Class>, String)> {
    match reference {
        Reference::Object(object) => {
            let name_value = object.value("name")?;
            let name = name_value.as_string()?;
            let class = thread.class(&name).await?;
            let field_name = class.field_name(offset)?;
            Ok((class, field_name))
        }
        _ => Err(InternalError(
            "Static field access expects Class object".to_string(),
        )),
    }
}

#[expect(clippy::too_many_lines)]
async fn get_reference_type(
    thread: Arc<Thread>,
    mut parameters: Parameters,
    base_type: Option<BaseType>,
) -> Result<Option<Value>> {
    let offset_long = parameters.pop_long()?;
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
            | BaseType::Short => Value::Int(i32::try_from(offset_long)?),
            BaseType::Long => Value::Long(offset_long),
            BaseType::Double | BaseType::Float => {
                return Err(InternalError(
                    "getReferenceType: Invalid offset".to_string(),
                ));
            }
        };
        return Ok(Some(value));
    };

    let is_static = offset_long & STATIC_FIELD_OFFSET_MASK != 0;
    let offset = usize::try_from(offset_long & !STATIC_FIELD_OFFSET_MASK)?;

    if is_static {
        let class_name = {
            let guard = reference.read();
            let object = match &*guard {
                Reference::Object(o) => o,
                _ => {
                    return Err(InternalError(
                        "Static field access on non-object".to_string(),
                    ));
                }
            };
            object.value("name")?.as_string()?
        };
        let class = thread.class(&class_name).await?;
        let field_name = class.field_name(offset)?;
        let value = class.static_value(&field_name)?;
        match value {
            Value::Object(None) => {
                if let Some(base_type) = base_type {
                    match base_type {
                        BaseType::Boolean
                        | BaseType::Byte
                        | BaseType::Char
                        | BaseType::Short
                        | BaseType::Int => Ok(Some(Value::Int(0))),
                        BaseType::Long => Ok(Some(Value::Long(0))),
                        BaseType::Float => Ok(Some(Value::Float(0.0))),
                        BaseType::Double => Ok(Some(Value::Double(0.0))),
                    }
                } else {
                    Ok(Some(value))
                }
            }
            _ => Ok(Some(value)),
        }
    } else {
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
                let value = object.value(&field_name)?;
                match value {
                    Value::Object(None) => {
                        if let Some(base_type) = base_type {
                            match base_type {
                                BaseType::Boolean
                                | BaseType::Byte
                                | BaseType::Char
                                | BaseType::Short
                                | BaseType::Int => Ok(Some(Value::Int(0))),
                                BaseType::Long => Ok(Some(Value::Long(0))),
                                BaseType::Float => Ok(Some(Value::Float(0.0))),
                                BaseType::Double => Ok(Some(Value::Double(0.0))),
                            }
                        } else {
                            Ok(Some(value))
                        }
                    }
                    _ => Ok(Some(value)),
                }
            }
            primitive_array => {
                let Some(array) = primitive_array.as_bytes() else {
                    return Err(InternalError(
                        "getReferenceType: Invalid primitive array type".to_string(),
                    ));
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
                        let value =
                            i32::from_be_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
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
}

#[expect(clippy::too_many_lines)]
async fn put_reference_type(
    thread: Arc<Thread>,
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
    let offset_long = parameters.pop_long()?;
    let Some(reference) = parameters.pop_reference()? else {
        return Err(InternalError(
            "putReferenceType: Invalid reference".to_string(),
        ));
    };

    let is_static = offset_long & STATIC_FIELD_OFFSET_MASK != 0;
    let offset = usize::try_from(offset_long & !STATIC_FIELD_OFFSET_MASK)?;

    if is_static {
        let class_name = {
            let guard = reference.read();
            let object = match &*guard {
                Reference::Object(o) => o,
                _ => {
                    return Err(InternalError(
                        "Static field access on non-object".to_string(),
                    ));
                }
            };
            object.value("name")?.as_string()?
        };
        let class = thread.class(&class_name).await?;
        let field_name = class.field_name(offset)?;
        class.set_static_value(&field_name, value)?;
    } else {
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
                object.set_value(&field_name, value)?;
            }
            primitive_array => {
                let Some(array) = primitive_array.as_bytes_mut() else {
                    return Err(InternalError(
                        "putReferenceType: Invalid primitive array type".to_string(),
                    ));
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
    get_reference_type(thread, parameters, Some(BaseType::Boolean)).await
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
    get_reference_type(thread, parameters, Some(BaseType::Byte)).await
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
    get_reference_type(thread, parameters, Some(BaseType::Char)).await
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
    get_reference_type(thread, parameters, Some(BaseType::Double)).await
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
    get_reference_type(thread, parameters, Some(BaseType::Float)).await
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
    get_reference_type(thread, parameters, Some(BaseType::Int)).await
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.getLoadAverage0([DI)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_load_average_0(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let nelems = parameters.pop_int()?;
    let Some(reference) = parameters.pop_reference()? else {
        return Ok(Some(Value::Int(-1)));
    };

    let load_avg = sysinfo::System::load_average();
    let averages = [load_avg.one, load_avg.five, load_avg.fifteen];

    let mut guard = reference.write();
    let Reference::DoubleArray(array) = &mut *guard else {
        return Ok(Some(Value::Int(-1)));
    };

    let count = std::cmp::min(nelems as usize, std::cmp::min(3, array.len()));
    for i in 0..count {
        array[i] = averages[i];
    }

    Ok(Some(Value::Int(i32::try_from(count)?)))
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
    get_reference_type(thread, parameters, Some(BaseType::Long)).await
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
    get_reference_type(thread, parameters, None).await
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
    get_reference_type(thread, parameters, None).await
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
    get_reference_type(thread, parameters, Some(BaseType::Short)).await
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.getUncompressedObject(J)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_uncompressed_object(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _address = parameters.pop_long()?;
    Ok(Some(Value::Object(None)))
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
    put_reference_type(thread, parameters, Some(BaseType::Boolean)).await
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
    put_reference_type(thread, parameters, Some(BaseType::Byte)).await
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
    put_reference_type(thread, parameters, Some(BaseType::Char)).await
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
    put_reference_type(thread, parameters, Some(BaseType::Double)).await
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
    put_reference_type(thread, parameters, Some(BaseType::Float)).await
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
    put_reference_type(thread, parameters, Some(BaseType::Int)).await
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
    put_reference_type(thread, parameters, Some(BaseType::Long)).await
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
    put_reference_type(thread, parameters, None).await
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
    put_reference_type(thread, parameters, None).await
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
    put_reference_type(thread, parameters, Some(BaseType::Short)).await
}

#[intrinsic_method(
    "jdk/internal/misc/Unsafe.reallocateMemory0(JJ)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn reallocate_memory_0(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let bytes = parameters.pop_long()?;
    let _address = parameters.pop_long()?;

    if bytes < 0 {
        return Err(crate::JavaError::IllegalArgumentException(format!(
            "Invalid memory reallocation size: {bytes}"
        ))
        .into());
    }

    Ok(Some(Value::Long(0)))
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
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value = parameters.pop_int()? as u8;
    let bytes = parameters.pop_long()?;
    let offset = parameters.pop_long()?;
    let object = parameters.pop()?;

    let bytes = usize::try_from(bytes)?;
    let offset = usize::try_from(offset)?;

    if object.is_null() {
        return Ok(None);
    }

    let mut reference = object.as_reference_mut()?;
    let Some(slice) = reference.as_bytes_mut() else {
        return Err(InternalError("setMemory0: Invalid object type".to_string()));
    };

    if offset + bytes > slice.len() {
        return Err(ArrayIndexOutOfBoundsException {
            index: i32::try_from(offset + bytes)?,
            length: slice.len(),
        }
        .into());
    }

    slice[offset..offset + bytes].fill(value);

    Ok(None)
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
    let field = parameters.pop()?;
    let class = {
        let field = field.as_object_ref()?;
        field.value("clazz")?
    };
    Ok(Some(class))
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
    let field = parameters.pop()?;
    let (class, name) = {
        let field = field.as_object_ref()?;
        let class = field.value("clazz")?;
        let name = field.value("name")?;
        (class, name)
    };
    let parameters = Parameters::new(vec![class, name]);
    let result = object_field_offset_1(_thread, parameters).await?;
    if let Some(Value::Long(offset)) = result {
        Ok(Some(Value::Long(offset | STATIC_FIELD_OFFSET_MASK)))
    } else {
        Ok(result)
    }
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
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let throwable = parameters.pop()?;
    Err(crate::Error::Throwable(throwable))
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
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _address = parameters.pop_long()?;
    Ok(None)
}

#[intrinsic_method("jdk/internal/misc/Unsafe.writebackPostSync0()V", GreaterThan(JAVA_11))]
#[async_recursion(?Send)]
pub(crate) async fn writeback_post_sync_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("jdk/internal/misc/Unsafe.writebackPreSync0()V", GreaterThan(JAVA_11))]
#[async_recursion(?Send)]
pub(crate) async fn writeback_pre_sync_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
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
    async fn test_allocate_instance() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let class = thread.class("java.lang.Object").await?;
        let class_object = class.to_object(&thread).await?;
        let mut parameters = Parameters::default();
        parameters.push(class_object);
        let result = allocate_instance(thread, parameters).await?;
        assert!(matches!(result, Some(Value::Object(_))));
        Ok(())
    }

    #[tokio::test]
    async fn test_allocate_memory_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let mut parameters = Parameters::default();
        parameters.push_long(1024); // Request 1024 bytes
        let result = allocate_memory_0(thread, parameters).await?;
        // The implementation returns 0 as a pseudo-address
        assert_eq!(result, Some(Value::Long(0)));
        Ok(())
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
    async fn test_compare_and_exchange_int() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        // Test with a raw offset (no object)
        let mut parameters = Parameters::default();
        parameters.push(Value::Object(None)); // null object
        parameters.push_long(42); // offset (used as current value when object is null)
        parameters.push_int(42); // expected value
        parameters.push_int(100); // new value
        let result = compare_and_exchange_int(thread, parameters).await?;
        // When object is null, it uses offset as the current value
        assert_eq!(result, Some(Value::Int(42)));
        Ok(())
    }

    #[tokio::test]
    async fn test_compare_and_exchange_long() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        // Test with a raw offset (no object)
        let mut parameters = Parameters::default();
        parameters.push(Value::Object(None)); // null object
        parameters.push_long(42); // offset (used as current value when object is null)
        parameters.push_long(42); // expected value
        parameters.push_long(100); // new value
        let result = compare_and_exchange_long(thread, parameters).await?;
        // When object is null, it uses offset as the current value
        assert_eq!(result, Some(Value::Long(42)));
        Ok(())
    }

    #[tokio::test]
    async fn test_compare_and_exchange_object() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        // Test with an object that has fields
        let class = thread.class("java/lang/Object").await?;
        let object = Object::new(class)?;
        let reference = Reference::Object(object);
        let object_value = Value::from(reference);

        let mut parameters = Parameters::default();
        parameters.push(object_value);
        parameters.push_long(0); // offset
        parameters.push(Value::Object(None)); // expected value
        parameters.push(Value::Object(None)); // new value
        // TODO: verify results
        let _ = compare_and_exchange_object(thread, parameters).await;
        Ok(())
    }

    #[tokio::test]
    async fn test_compare_and_exchange_reference() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        // Test with an object that has fields
        let class = thread.class("java/lang/Object").await?;
        let object = Object::new(class)?;
        let reference = Reference::Object(object);
        let object_value = Value::from(reference);

        let mut parameters = Parameters::default();
        parameters.push(object_value);
        parameters.push_long(0); // offset
        parameters.push(Value::Object(None)); // expected value
        parameters.push(Value::Object(None)); // new value
        // TODO: verify results
        let _ = compare_and_exchange_reference(thread, parameters).await;
        Ok(())
    }

    #[tokio::test]
    async fn test_copy_swap_memory_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        // Create source and destination short arrays
        let source: Vec<i16> = vec![0x0102, 0x0304, 0x0506, 0x0708];
        let dest: Vec<i16> = vec![0, 0, 0, 0];

        let source_ref = Reference::from(source);
        let dest_ref = Reference::from(dest);

        let source_value = Value::from(source_ref);
        let dest_value = Value::from(dest_ref);

        let mut parameters = Parameters::default();
        parameters.push(source_value);
        parameters.push_long(0); // source offset
        parameters.push(dest_value.clone());
        parameters.push_long(0); // dest offset
        parameters.push_long(8); // 8 bytes (4 shorts)
        parameters.push_long(2); // element size = 2 (short)

        let result = copy_swap_memory_0(thread, parameters).await?;
        assert_eq!(result, None);

        // After byte swap, 0x0102 becomes 0x0201, etc.
        let dest_guard = dest_value.as_reference()?;
        if let Reference::ShortArray(arr) = &*dest_guard {
            assert_eq!(arr[0], 0x0201);
            assert_eq!(arr[1], 0x0403);
            assert_eq!(arr[2], 0x0605);
            assert_eq!(arr[3], 0x0807);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_define_anonymous_class_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        // Create class file bytes from Minimum.class
        let class_bytes = include_bytes!("../../../../../../classes/Minimum.class");
        let bytes: Vec<i8> = class_bytes.iter().map(|&b| b as i8).collect();
        let bytes_value = Value::from(bytes);

        // Get host class (java.lang.Object)
        let host_class = thread.class("java/lang/Object").await?;
        let host_class_object = host_class.to_object(&thread).await?;

        let mut parameters = Parameters::default();
        parameters.push(host_class_object);
        parameters.push(bytes_value);
        parameters.push(Value::Object(None)); // cp_patches (null)

        let result = define_anonymous_class_0(thread, parameters).await?;
        assert!(result.is_some());
        // The result should be a Class object
        let class_value = result.expect("class");
        assert!(matches!(class_value, Value::Object(Some(_))));
        Ok(())
    }

    #[tokio::test]
    async fn test_define_class_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        // Create class file bytes from Minimum.class
        let class_bytes = include_bytes!("../../../../../../classes/Minimum.class");
        let bytes: Vec<i8> = class_bytes.iter().map(|&b| b as i8).collect();
        let bytes_len = bytes.len();
        let bytes_value = Value::from(bytes);

        // Create the class name
        let class_name = "Minimum".to_object(&thread).await?;

        let mut parameters = Parameters::default();
        parameters.push(class_name);
        parameters.push(bytes_value);
        parameters.push_int(0); // offset
        parameters.push_int(i32::try_from(bytes_len)?); // length
        parameters.push(Value::Object(None)); // classLoader (null = bootstrap)
        parameters.push(Value::Object(None)); // protectionDomain (null)

        let result = define_class_0(thread, parameters).await?;
        assert!(result.is_some());
        // The result should be a Class object
        let class_value = result.expect("class");
        assert!(matches!(class_value, Value::Object(Some(_))));
        Ok(())
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
    async fn test_get_load_average_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let loadavg = Reference::from(vec![0.0f64, 0.0, 0.0]);
        let loadavg_value = Value::from(loadavg);
        let mut parameters = Parameters::default();
        parameters.push(loadavg_value.clone());
        parameters.push_int(3); // nelems
        let result = get_load_average_0(thread, parameters).await?;
        // Returns 3 indicating all 3 load averages were filled
        assert_eq!(result, Some(Value::Int(3)));
        // Verify load averages are non-negative (valid values)
        let guard = loadavg_value.as_reference()?;
        let Reference::DoubleArray(array) = &*guard else {
            panic!("Expected DoubleArray");
        };
        assert!(
            array[0] >= 0.0,
            "1-minute load average should be non-negative"
        );
        assert!(
            array[1] >= 0.0,
            "5-minute load average should be non-negative"
        );
        assert!(
            array[2] >= 0.0,
            "15-minute load average should be non-negative"
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_get_uncompressed_object() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let mut parameters = Parameters::default();
        parameters.push_long(0); // address
        let result = get_uncompressed_object(thread, parameters).await?;
        // Returns null since this VM doesn't use compressed oops
        assert_eq!(result, Some(Value::Object(None)));
        Ok(())
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
    async fn test_reallocate_memory_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let mut parameters = Parameters::default();
        parameters.push_long(0); // address
        parameters.push_long(2048); // new size
        let result = reallocate_memory_0(thread, parameters).await?;
        // Returns 0 as a pseudo-address since we don't manage native memory
        assert_eq!(result, Some(Value::Long(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = register_natives(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_memory_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let arr = Reference::from(vec![0i8, 0, 0, 0, 0, 0, 0, 0]);
        let arr_value = Value::from(arr);
        let mut parameters = Parameters::default();
        parameters.push(arr_value.clone());
        parameters.push_long(0); // offset
        parameters.push_long(8); // bytes
        parameters.push_int(0xFF); // value
        let result = set_memory_0(thread, parameters).await?;
        assert_eq!(result, None);

        // Verify the array was filled
        let guard = arr_value.as_reference()?;
        if let Reference::ByteArray(arr) = &*guard {
            for i in 0..8 {
                assert_eq!(arr[i], -1); // 0xFF as i8 is -1
            }
        }
        Ok(())
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
        let value = static_field_base_0(thread.clone(), parameters)
            .await?
            .expect("object");
        // static_field_base_0 returns the declaring class (clazz field) of the Field
        let string_class = thread.class("java/lang/String").await?;
        let string_class_object = string_class.to_object(&thread).await?;
        assert_eq!(value, string_class_object);
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
        assert_eq!(offset, STATIC_FIELD_OFFSET_MASK);
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
    async fn test_throw_exception() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let mut parameters = Parameters::default();
        parameters.push(Value::Object(None)); // null throwable
        let result = throw_exception(thread, parameters).await;
        // Should return an Error::Throwable
        assert!(matches!(result, Err(crate::Error::Throwable(_))));
        Ok(())
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
    async fn test_writeback_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let mut parameters = Parameters::default();
        parameters.push_long(0); // address
        let result = writeback_0(thread, parameters).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_writeback_post_sync_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = writeback_post_sync_0(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_writeback_pre_sync_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = writeback_pre_sync_0(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
