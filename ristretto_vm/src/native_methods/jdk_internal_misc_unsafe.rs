use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_11: Version = Version::Java11 { minor: 0 };
const JAVA_17: Version = Version::Java17 { minor: 0 };

/// Register all native methods for `jdk.internal.misc.Unsafe`.
#[expect(clippy::too_many_lines)]
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/internal/misc/Unsafe";
    let java_version = registry.java_version().clone();

    if java_version <= JAVA_11 {
        registry.register(class_name, "addressSize0", "()I", address_size_0);
        registry.register(
            class_name,
            "compareAndExchangeObject",
            "(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            compare_and_exchange_object,
        );
        registry.register(
            class_name,
            "compareAndSetObject",
            "(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z",
            compare_and_set_object,
        );
        registry.register(
            class_name,
            "defineAnonymousClass0",
            "(Ljava/lang/Class;[B[Ljava/lang/Object;)Ljava/lang/Class;",
            define_anonymous_class_0,
        );
        registry.register(
            class_name,
            "getObject",
            "(Ljava/lang/Object;J)Ljava/lang/Object;",
            get_object,
        );
        registry.register(
            class_name,
            "getObjectVolatile",
            "(Ljava/lang/Object;J)Ljava/lang/Object;",
            get_object_volatile,
        );
        registry.register(class_name, "isBigEndian0", "()Z", is_big_endian_0);
        registry.register(class_name, "pageSize", "()I", page_size);
        registry.register(
            class_name,
            "putObject",
            "(Ljava/lang/Object;JLjava/lang/Object;)V",
            put_object,
        );
        registry.register(
            class_name,
            "putObjectVolatile",
            "(Ljava/lang/Object;JLjava/lang/Object;)V",
            put_object_volatile,
        );
        registry.register(class_name, "unalignedAccess0", "()Z", unaligned_access_0);
    } else {
        registry.register(
            class_name,
            "compareAndExchangeReference",
            "(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            compare_and_exchange_reference,
        );
        registry.register(
            class_name,
            "compareAndSetReference",
            "(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z",
            compare_and_set_reference,
        );
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
        registry.register(class_name, "writeback0", "(J)V", writeback_0);
        registry.register(
            class_name,
            "writebackPostSync0",
            "()V",
            writeback_post_sync_0,
        );
        registry.register(class_name, "writebackPreSync0", "()V", writeback_pre_sync_0);
    }

    if java_version <= JAVA_17 {
        registry.register(class_name, "loadFence", "()V", load_fence);
        registry.register(class_name, "storeFence", "()V", store_fence);
    }

    registry.register(
        class_name,
        "allocateInstance",
        "(Ljava/lang/Class;)Ljava/lang/Object;",
        allocate_instance,
    );
    registry.register(class_name, "allocateMemory0", "(J)J", allocate_memory_0);
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
        "compareAndExchangeInt",
        "(Ljava/lang/Object;JII)I",
        compare_and_exchange_int,
    );
    registry.register(
        class_name,
        "compareAndExchangeLong",
        "(Ljava/lang/Object;JJJ)J",
        compare_and_exchange_long,
    );
    registry.register(
        class_name,
        "compareAndExchangeReference",
        "(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
        compare_and_exchange_reference,
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
        "copyMemory0",
        "(Ljava/lang/Object;JLjava/lang/Object;JJ)V",
        copy_memory_0,
    );
    registry.register(
        class_name,
        "copySwapMemory0",
        "(Ljava/lang/Object;JLjava/lang/Object;JJJ)V",
        copy_swap_memory_0,
    );
    registry.register(class_name, "defineClass0", "(Ljava/lang/String;[BIILjava/lang/ClassLoader;Ljava/security/ProtectionDomain;)Ljava/lang/Class;", define_class_0);
    registry.register(
        class_name,
        "ensureClassInitialized0",
        "(Ljava/lang/Class;)V",
        ensure_class_initialized_0,
    );
    registry.register(class_name, "freeMemory0", "(J)V", free_memory_0);
    registry.register(class_name, "fullFence", "()V", full_fence);
    registry.register(
        class_name,
        "getBoolean",
        "(Ljava/lang/Object;J)Z",
        get_boolean,
    );
    registry.register(
        class_name,
        "getBooleanVolatile",
        "(Ljava/lang/Object;J)Z",
        get_boolean_volatile,
    );
    registry.register(class_name, "getByte", "(Ljava/lang/Object;J)B", get_byte);
    registry.register(
        class_name,
        "getByteVolatile",
        "(Ljava/lang/Object;J)B",
        get_byte_volatile,
    );
    registry.register(class_name, "getChar", "(Ljava/lang/Object;J)C", get_char);
    registry.register(
        class_name,
        "getCharVolatile",
        "(Ljava/lang/Object;J)C",
        get_char_volatile,
    );
    registry.register(
        class_name,
        "getDouble",
        "(Ljava/lang/Object;J)D",
        get_double,
    );
    registry.register(
        class_name,
        "getDoubleVolatile",
        "(Ljava/lang/Object;J)D",
        get_double_volatile,
    );
    registry.register(class_name, "getFloat", "(Ljava/lang/Object;J)F", get_float);
    registry.register(
        class_name,
        "getFloatVolatile",
        "(Ljava/lang/Object;J)F",
        get_float_volatile,
    );
    registry.register(class_name, "getInt", "(Ljava/lang/Object;J)I", get_int);
    registry.register(
        class_name,
        "getIntVolatile",
        "(Ljava/lang/Object;J)I",
        get_int_volatile,
    );
    registry.register(class_name, "getLoadAverage0", "([DI)I", get_load_average_0);
    registry.register(class_name, "getLong", "(Ljava/lang/Object;J)J", get_long);
    registry.register(
        class_name,
        "getLongVolatile",
        "(Ljava/lang/Object;J)J",
        get_long_volatile,
    );
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
    registry.register(class_name, "getShort", "(Ljava/lang/Object;J)S", get_short);
    registry.register(
        class_name,
        "getShortVolatile",
        "(Ljava/lang/Object;J)S",
        get_short_volatile,
    );
    registry.register(
        class_name,
        "getUncompressedObject",
        "(J)Ljava/lang/Object;",
        get_uncompressed_object,
    );
    registry.register(
        class_name,
        "objectFieldOffset0",
        "(Ljava/lang/reflect/Field;)J",
        object_field_offset_0,
    );
    registry.register(
        class_name,
        "objectFieldOffset1",
        "(Ljava/lang/Class;Ljava/lang/String;)J",
        object_field_offset_1,
    );
    registry.register(class_name, "park", "(ZJ)V", park);
    registry.register(
        class_name,
        "putBoolean",
        "(Ljava/lang/Object;JZ)V",
        put_boolean,
    );
    registry.register(
        class_name,
        "putBooleanVolatile",
        "(Ljava/lang/Object;JZ)V",
        put_boolean_volatile,
    );
    registry.register(class_name, "putByte", "(Ljava/lang/Object;JB)V", put_byte);
    registry.register(
        class_name,
        "putByteVolatile",
        "(Ljava/lang/Object;JB)V",
        put_byte_volatile,
    );
    registry.register(class_name, "putChar", "(Ljava/lang/Object;JC)V", put_char);
    registry.register(
        class_name,
        "putCharVolatile",
        "(Ljava/lang/Object;JC)V",
        put_char_volatile,
    );
    registry.register(
        class_name,
        "putDouble",
        "(Ljava/lang/Object;JD)V",
        put_double,
    );
    registry.register(
        class_name,
        "putDoubleVolatile",
        "(Ljava/lang/Object;JD)V",
        put_double_volatile,
    );
    registry.register(class_name, "putFloat", "(Ljava/lang/Object;JF)V", put_float);
    registry.register(
        class_name,
        "putFloatVolatile",
        "(Ljava/lang/Object;JF)V",
        put_float_volatile,
    );
    registry.register(class_name, "putInt", "(Ljava/lang/Object;JI)V", put_int);
    registry.register(
        class_name,
        "putIntVolatile",
        "(Ljava/lang/Object;JI)V",
        put_int_volatile,
    );
    registry.register(class_name, "putLong", "(Ljava/lang/Object;JJ)V", put_long);
    registry.register(
        class_name,
        "putLongVolatile",
        "(Ljava/lang/Object;JJ)V",
        put_long_volatile,
    );
    registry.register(
        class_name,
        "putReference",
        "(Ljava/lang/Object;JLjava/lang/Object;)V",
        put_reference,
    );
    registry.register(
        class_name,
        "putReferenceVolatile",
        "(Ljava/lang/Object;JLjava/lang/Object;)V",
        put_reference_volatile,
    );
    registry.register(class_name, "putShort", "(Ljava/lang/Object;JS)V", put_short);
    registry.register(
        class_name,
        "putShortVolatile",
        "(Ljava/lang/Object;JS)V",
        put_short_volatile,
    );
    registry.register(
        class_name,
        "reallocateMemory0",
        "(JJ)J",
        reallocate_memory_0,
    );
    registry.register(class_name, "registerNatives", "()V", register_natives);
    registry.register(
        class_name,
        "setMemory0",
        "(Ljava/lang/Object;JJB)V",
        set_memory_0,
    );
    registry.register(
        class_name,
        "shouldBeInitialized0",
        "(Ljava/lang/Class;)Z",
        should_be_initialized_0,
    );
    registry.register(
        class_name,
        "staticFieldBase0",
        "(Ljava/lang/reflect/Field;)Ljava/lang/Object;",
        static_field_base_0,
    );
    registry.register(
        class_name,
        "staticFieldOffset0",
        "(Ljava/lang/reflect/Field;)J",
        static_field_offset_0,
    );
    registry.register(
        class_name,
        "throwException",
        "(Ljava/lang/Throwable;)V",
        throw_exception,
    );
    registry.register(class_name, "unpark", "(Ljava/lang/Object;)V", unpark);
    registry.register(class_name, "writeback0", "(J)V", writeback_0);
    registry.register(
        class_name,
        "writebackPostSync0",
        "()V",
        writeback_post_sync_0,
    );
    registry.register(class_name, "writebackPreSync0", "()V", writeback_pre_sync_0);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn address_size_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn allocate_instance(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn allocate_memory_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn array_base_offset_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn array_index_scale_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn compare_and_exchange_int(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn compare_and_exchange_long(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn compare_and_exchange_object(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn compare_and_exchange_reference(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn compare_and_set_int(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn compare_and_set_long(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn compare_and_set_object(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn compare_and_set_reference(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn copy_memory_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn copy_swap_memory_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn define_anonymous_class_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn define_class_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn ensure_class_initialized_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn free_memory_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn full_fence(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_boolean(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_boolean_volatile(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_byte(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_byte_volatile(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_char(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_char_volatile(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_double(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_double_volatile(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_float(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_float_volatile(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_int(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_int_volatile(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_load_average_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_long(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_long_volatile(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_object(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_reference(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_object_volatile(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_reference_volatile(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_short(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_short_volatile(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_uncompressed_object(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn is_big_endian_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn load_fence(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn object_field_offset_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn object_field_offset_1(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn page_size(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn park(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn put_boolean(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn put_boolean_volatile(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn put_byte(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn put_byte_volatile(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn put_char(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn put_char_volatile(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn put_double(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn put_double_volatile(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn put_float(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn put_float_volatile(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn put_int(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn put_int_volatile(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn put_long(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn put_long_volatile(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn put_object(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn put_object_volatile(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn put_reference(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn put_reference_volatile(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn put_short(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn put_short_volatile(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn reallocate_memory_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_memory_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn should_be_initialized_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn static_field_base_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn static_field_offset_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn store_fence(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn throw_exception(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn unaligned_access_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn unpark(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn writeback_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn writeback_post_sync_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn writeback_pre_sync_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}
