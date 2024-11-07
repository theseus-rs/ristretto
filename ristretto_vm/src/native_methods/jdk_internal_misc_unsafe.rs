use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Error::{InternalError, InvalidOperand};
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::{Reference, Value};
use std::sync::Arc;

/// Register all native methods for jdk.internal.misc.Unsafe.
#[expect(clippy::too_many_lines)]
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/internal/misc/Unsafe";
    registry.register(class_name, "<init>", "()V", init);
    registry.register(class_name, "addressSize0", "()I", address_size_0);
    registry.register(
        class_name,
        "arrayBaseOffset0",
        "(Ljava/lang/Class;)I",
        array_base_offset_0,
    );
    registry.register(
        class_name,
        "arrayIndexScale0",
        "(Ljava/lang/Class;)I",
        array_index_scale_0,
    );
    registry.register(
        class_name,
        "compareAndSetInt",
        "(Ljava/lang/Object;JII)Z",
        compare_and_set_int,
    );
    registry.register(
        class_name,
        "compareAndSetLong",
        "(Ljava/lang/Object;JJJ)Z",
        compare_and_set_long,
    );
    registry.register(
        class_name,
        "compareAndSetReference",
        "(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z",
        compare_and_set_reference,
    );
    registry.register(
        class_name,
        "ensureClassInitialized0",
        "(Ljava/lang/Class;)V",
        ensure_class_initialized_0,
    );
    registry.register(class_name, "fullFence", "()V", full_fence);
    registry.register(class_name, "isBigEndian0", "()Z", is_big_endian_0);
    registry.register(class_name, "unalignedAccess0", "()Z", unaligned_access_0);
    registry.register(
        class_name,
        "getReference",
        "(Ljava/lang/Object;J)Ljava/lang/Object;",
        get_reference,
    );
    registry.register(
        class_name,
        "getReferenceVolatile",
        "(Ljava/lang/Object;J)Ljava/lang/Object;",
        get_reference_volatile,
    );
    registry.register(
        class_name,
        "objectFieldOffset1",
        "(Ljava/lang/Class;Ljava/lang/String;)J",
        object_field_offset_1,
    );
    registry.register(
        class_name,
        "putReferenceVolatile",
        "(Ljava/lang/Object;JLjava/lang/Object;)V",
        put_reference_volatile,
    );
    registry.register(class_name, "registerNatives", "()V", register_natives);

    // Delegate get/put type volatile methods

    registry.register(
        class_name,
        "getBooleanVolatile",
        "(Ljava/lang/Object;J)Z",
        get_reference_volatile,
    );
    registry.register(
        class_name,
        "putBooleanVolatile",
        "(Ljava/lang/Object;JZ)V",
        put_reference_volatile,
    );

    registry.register(
        class_name,
        "getByteVolatile",
        "(Ljava/lang/Object;J)B",
        get_reference_volatile,
    );
    registry.register(class_name, "loadFence", "()V", load_fence);
    registry.register(
        class_name,
        "putByteVolatile",
        "(Ljava/lang/Object;JB)V",
        put_reference_volatile,
    );

    registry.register(
        class_name,
        "getCharVolatile",
        "(Ljava/lang/Object;J)C",
        get_reference_volatile,
    );
    registry.register(
        class_name,
        "putCharVolatile",
        "(Ljava/lang/Object;JC)V",
        put_reference_volatile,
    );

    registry.register(
        class_name,
        "getDoubleVolatile",
        "(Ljava/lang/Object;J)D",
        get_reference_volatile,
    );
    registry.register(
        class_name,
        "putDoubleVolatile",
        "(Ljava/lang/Object;JD)V",
        put_reference_volatile,
    );

    registry.register(
        class_name,
        "getFloatVolatile",
        "(Ljava/lang/Object;J)F",
        get_reference_volatile,
    );
    registry.register(
        class_name,
        "putFloatVolatile",
        "(Ljava/lang/Object;JF)V",
        put_reference_volatile,
    );

    registry.register(
        class_name,
        "getIntVolatile",
        "(Ljava/lang/Object;J)I",
        get_reference_volatile,
    );
    registry.register(
        class_name,
        "putIntVolatile",
        "(Ljava/lang/Object;JI)V",
        put_reference_volatile,
    );

    registry.register(
        class_name,
        "getLongVolatile",
        "(Ljava/lang/Object;J)J",
        get_reference_volatile,
    );
    registry.register(
        class_name,
        "putLongVolatile",
        "(Ljava/lang/Object;JJ)V",
        put_reference_volatile,
    );

    registry.register(
        class_name,
        "getShortVolatile",
        "(Ljava/lang/Object;J)S",
        get_reference_volatile,
    );
    registry.register(
        class_name,
        "putShortVolatile",
        "(Ljava/lang/Object;JS)V",
        put_reference_volatile,
    );

    registry.register(class_name, "storeFence", "()V", store_fence);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    // Unsafe <init> is a no-op and the class is deprecated; override the default behavior to avoid
    // the performance penalty of creating a new frame.
    Ok(None)
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn address_size_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(Some(Value::Int(8))) // 64-bit pointers
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn array_base_offset_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(Some(Value::Int(0)))
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn array_index_scale_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(Some(Value::Int(1)))
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn compare_and_set_int(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let x = arguments.pop_int()?;
    let expected = arguments.pop_int()?;
    let offset = arguments.pop_long()?;
    let Some(Reference::Object(object)) = arguments.pop_object()? else {
        return Err(InternalError(
            "compareAndSetInt: Invalid reference".to_string(),
        ));
    };
    let class = object.class();
    let offset = usize::try_from(offset)?;
    let field_name = class.field_name(offset)?;
    let field = object.field(&field_name)?;
    let value = field.value()?.to_int()?;
    // TODO: the compare and set operation should be atomic
    let result = if value == expected {
        field.set_value(Value::Int(x))?;
        1
    } else {
        0
    };
    Ok(Some(Value::Int(result)))
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn compare_and_set_long(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let x = arguments.pop_long()?;
    let expected = arguments.pop_long()?;
    let offset = arguments.pop_long()?;
    let Some(Reference::Object(object)) = arguments.pop_object()? else {
        return Err(InternalError(
            "compareAndSetLong: Invalid reference".to_string(),
        ));
    };
    let class = object.class();
    let offset = usize::try_from(offset)?;
    let field_name = class.field_name(offset)?;
    let field = object.field(&field_name)?;
    let value = field.value()?.to_long()?;
    // TODO: the compare and set operation should be atomic
    let result = if value == expected {
        field.set_value(Value::Long(x))?;
        1
    } else {
        0
    };
    Ok(Some(Value::Int(result)))
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn compare_and_set_reference(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let x = arguments.pop_object()?;
    let expected = arguments.pop_object()?;
    let offset = arguments.pop_long()?;
    let offset = usize::try_from(offset)?;
    let Some(object) = arguments.pop_object()? else {
        return Err(InternalError(
            "compareAndSetReference: Invalid reference".to_string(),
        ));
    };
    let result = match object {
        Reference::Array(_class, array) => {
            let Some(reference) = array.get(offset)? else {
                return Err(InternalError(
                    "getReference: Invalid reference index".to_string(),
                ));
            };
            // TODO: the compare and set operation should be atomic
            if reference == expected {
                array.set(offset, x)?;
                1
            } else {
                0
            }
        }
        _ => {
            return Err(InternalError("getReference: Invalid reference".to_string()));
        }
    };
    Ok(Some(Value::Int(result)))
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn ensure_class_initialized_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(None)
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn full_fence(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn is_big_endian_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    if cfg!(target_endian = "big") {
        Ok(Some(Value::Int(1)))
    } else {
        Ok(Some(Value::Int(0)))
    }
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn unaligned_access_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(Some(Value::Int(0)))
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_reference(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let offset = arguments.pop_long()?;
    let offset = usize::try_from(offset)?;
    let Some(reference) = arguments.pop_object()? else {
        return Err(InternalError("getReference: Invalid reference".to_string()));
    };
    match reference {
        Reference::Array(_class, array) => {
            let Some(reference) = array.get(offset)? else {
                return Err(InternalError(
                    "getReference: Invalid reference index".to_string(),
                ));
            };
            Ok(Some(Value::Object(reference)))
        }
        Reference::Object(object) => {
            let field_name = object.class().field_name(offset)?;
            let value = object.value(&field_name)?;
            Ok(Some(value))
        }
        _ => Err(InternalError("getReference: Invalid reference".to_string())),
    }
}

#[inline]
#[async_recursion(?Send)]
async fn get_reference_volatile(
    thread: Arc<Thread>,
    arguments: Arguments,
) -> Result<Option<Value>> {
    get_reference(thread, arguments).await
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn load_fence(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn object_field_offset_1(
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
    let Some(Reference::Object(class_object)) = arguments.pop_object()? else {
        return Err(InternalError(
            "objectFieldOffset1: Invalid class reference".to_string(),
        ));
    };
    let class_name: String = class_object.value("name")?.try_into()?;
    let vm = thread.vm()?;
    let class = vm.load_class(&thread, &class_name).await?;
    let offset = class.field_offset(&field_name)?;
    let offset = i64::try_from(offset)?;
    Ok(Some(Value::Long(offset)))
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn put_reference_volatile(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let x = arguments.pop_object()?;
    let offset = arguments.pop_long()?;
    let offset = usize::try_from(offset)?;
    let Some(object) = arguments.pop_object()? else {
        return Err(InternalError(
            "compareAndSetReference: Invalid reference".to_string(),
        ));
    };
    match object {
        Reference::Array(_class, array) => {
            array.set(offset, x)?;
        }
        _ => {
            return Err(InternalError("getReference: Invalid reference".to_string()));
        }
    }
    Ok(None)
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn store_fence(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}
