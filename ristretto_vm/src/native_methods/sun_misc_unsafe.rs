use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.misc.Unsafe`.
#[expect(clippy::too_many_lines)]
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/misc/Unsafe";
    registry.register(class_name, "addressSize", "()I", address_size);
    registry.register(
        class_name,
        "allocateInstance",
        "(Ljava/lang/Class;)Ljava/lang/Object;",
        allocate_instance,
    );
    registry.register(class_name, "allocateMemory", "(J)J", allocate_memory);
    registry.register(
        class_name,
        "arrayBaseOffset",
        "(Ljava/lang/Class;)I",
        array_base_offset,
    );
    registry.register(
        class_name,
        "arrayIndexScale",
        "(Ljava/lang/Class;)I",
        array_index_scale,
    );
    registry.register(
        class_name,
        "compareAndSwapInt",
        "(Ljava/lang/Object;JII)Z",
        compare_and_swap_int,
    );
    registry.register(
        class_name,
        "compareAndSwapLong",
        "(Ljava/lang/Object;JJJ)Z",
        compare_and_swap_long,
    );
    registry.register(
        class_name,
        "compareAndSwapObject",
        "(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z",
        compare_and_swap_object,
    );
    registry.register(
        class_name,
        "copyMemory",
        "(Ljava/lang/Object;JLjava/lang/Object;JJ)V",
        copy_memory,
    );
    registry.register(
        class_name,
        "defineAnonymousClass",
        "(Ljava/lang/Class;[B[Ljava/lang/Object;)Ljava/lang/Class;",
        define_anonymous_class,
    );
    registry.register(class_name, "defineClass", "(Ljava/lang/String;[BIILjava/lang/ClassLoader;Ljava/security/ProtectionDomain;)Ljava/lang/Class;", define_class);
    registry.register(
        class_name,
        "ensureClassInitialized",
        "(Ljava/lang/Class;)V",
        ensure_class_initialized,
    );
    registry.register(class_name, "freeMemory", "(J)V", free_memory);
    registry.register(class_name, "fullFence", "()V", full_fence);
    registry.register(class_name, "getAddress", "(J)J", get_address);
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
    registry.register(class_name, "getByte", "(J)B", get_byte_1);
    registry.register(class_name, "getByte", "(Ljava/lang/Object;J)B", get_byte_2);
    registry.register(
        class_name,
        "getByteVolatile",
        "(Ljava/lang/Object;J)B",
        get_byte_volatile,
    );
    registry.register(class_name, "getChar", "(J)C", get_char_1);
    registry.register(class_name, "getChar", "(Ljava/lang/Object;J)C", get_char_2);
    registry.register(
        class_name,
        "getCharVolatile",
        "(Ljava/lang/Object;J)C",
        get_char_volatile,
    );
    registry.register(class_name, "getDouble", "(J)D", get_double_1);
    registry.register(
        class_name,
        "getDouble",
        "(Ljava/lang/Object;J)D",
        get_double_2,
    );
    registry.register(
        class_name,
        "getDoubleVolatile",
        "(Ljava/lang/Object;J)D",
        get_double_volatile,
    );
    registry.register(class_name, "getFloat", "(J)F", get_float_1);
    registry.register(
        class_name,
        "getFloat",
        "(Ljava/lang/Object;J)F",
        get_float_2,
    );
    registry.register(
        class_name,
        "getFloatVolatile",
        "(Ljava/lang/Object;J)F",
        get_float_volatile,
    );
    registry.register(class_name, "getInt", "(J)I", get_int_1);
    registry.register(class_name, "getInt", "(Ljava/lang/Object;J)I", get_int_2);
    registry.register(
        class_name,
        "getIntVolatile",
        "(Ljava/lang/Object;J)I",
        get_int_volatile,
    );
    registry.register(class_name, "getLoadAverage", "([DI)I", get_load_average);
    registry.register(class_name, "getLong", "(J)J", get_long_1);
    registry.register(class_name, "getLong", "(Ljava/lang/Object;J)J", get_long_2);
    registry.register(
        class_name,
        "getLongVolatile",
        "(Ljava/lang/Object;J)J",
        get_long_volatile,
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
    registry.register(class_name, "getShort", "(J)S", get_short_1);
    registry.register(
        class_name,
        "getShort",
        "(Ljava/lang/Object;J)S",
        get_short_2,
    );
    registry.register(
        class_name,
        "getShortVolatile",
        "(Ljava/lang/Object;J)S",
        get_short_volatile,
    );
    registry.register(class_name, "loadFence", "()V", load_fence);
    registry.register(
        class_name,
        "monitorEnter",
        "(Ljava/lang/Object;)V",
        monitor_enter,
    );
    registry.register(
        class_name,
        "monitorExit",
        "(Ljava/lang/Object;)V",
        monitor_exit,
    );
    registry.register(
        class_name,
        "objectFieldOffset",
        "(Ljava/lang/reflect/Field;)J",
        object_field_offset,
    );
    registry.register(class_name, "pageSize", "()I", page_size);
    registry.register(class_name, "park", "(ZJ)V", park);
    registry.register(class_name, "putAddress", "(JJ)V", put_address);
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
    registry.register(class_name, "putByte", "(JB)V", put_byte_1);
    registry.register(class_name, "putByte", "(Ljava/lang/Object;JB)V", put_byte_2);
    registry.register(
        class_name,
        "putByteVolatile",
        "(Ljava/lang/Object;JB)V",
        put_byte_volatile,
    );
    registry.register(class_name, "putChar", "(JC)V", put_char_1);
    registry.register(class_name, "putChar", "(Ljava/lang/Object;JC)V", put_char_2);
    registry.register(
        class_name,
        "putCharVolatile",
        "(Ljava/lang/Object;JC)V",
        put_char_volatile,
    );
    registry.register(class_name, "putDouble", "(JD)V", put_double_1);
    registry.register(
        class_name,
        "putDouble",
        "(Ljava/lang/Object;JD)V",
        put_double_2,
    );
    registry.register(
        class_name,
        "putDoubleVolatile",
        "(Ljava/lang/Object;JD)V",
        put_double_volatile,
    );
    registry.register(class_name, "putFloat", "(JF)V", put_float_1);
    registry.register(
        class_name,
        "putFloat",
        "(Ljava/lang/Object;JF)V",
        put_float_2,
    );
    registry.register(
        class_name,
        "putFloatVolatile",
        "(Ljava/lang/Object;JF)V",
        put_float_volatile,
    );
    registry.register(class_name, "putInt", "(JI)V", put_int_1);
    registry.register(class_name, "putInt", "(Ljava/lang/Object;JI)V", put_int_2);
    registry.register(
        class_name,
        "putIntVolatile",
        "(Ljava/lang/Object;JI)V",
        put_int_volatile,
    );
    registry.register(class_name, "putLong", "(JJ)V", put_long_1);
    registry.register(class_name, "putLong", "(Ljava/lang/Object;JJ)V", put_long_2);
    registry.register(
        class_name,
        "putLongVolatile",
        "(Ljava/lang/Object;JJ)V",
        put_long_volatile,
    );
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
    registry.register(
        class_name,
        "putOrderedInt",
        "(Ljava/lang/Object;JI)V",
        put_ordered_int,
    );
    registry.register(
        class_name,
        "putOrderedLong",
        "(Ljava/lang/Object;JJ)V",
        put_ordered_long,
    );
    registry.register(
        class_name,
        "putOrderedObject",
        "(Ljava/lang/Object;JLjava/lang/Object;)V",
        put_ordered_object,
    );
    registry.register(class_name, "putShort", "(JS)V", put_short_1);
    registry.register(
        class_name,
        "putShort",
        "(Ljava/lang/Object;JS)V",
        put_short_2,
    );
    registry.register(
        class_name,
        "putShortVolatile",
        "(Ljava/lang/Object;JS)V",
        put_short_volatile,
    );
    registry.register(class_name, "reallocateMemory", "(JJ)J", reallocate_memory);
    registry.register(class_name, "registerNatives", "()V", register_natives);
    registry.register(
        class_name,
        "setMemory",
        "(Ljava/lang/Object;JJB)V",
        set_memory,
    );
    registry.register(
        class_name,
        "shouldBeInitialized",
        "(Ljava/lang/Class;)Z",
        should_be_initialized,
    );
    registry.register(
        class_name,
        "staticFieldBase",
        "(Ljava/lang/reflect/Field;)Ljava/lang/Object;",
        static_field_base,
    );
    registry.register(
        class_name,
        "staticFieldOffset",
        "(Ljava/lang/reflect/Field;)J",
        static_field_offset,
    );
    registry.register(class_name, "storeFence", "()V", store_fence);
    registry.register(
        class_name,
        "throwException",
        "(Ljava/lang/Throwable;)V",
        throw_exception,
    );
    registry.register(
        class_name,
        "tryMonitorEnter",
        "(Ljava/lang/Object;)Z",
        try_monitor_enter,
    );
    registry.register(class_name, "unpark", "(Ljava/lang/Object;)V", unpark);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn address_size(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn allocate_instance(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn allocate_memory(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn array_base_offset(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn array_index_scale(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn compare_and_swap_int(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn compare_and_swap_long(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn compare_and_swap_object(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn copy_memory(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn define_anonymous_class(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn define_class(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn ensure_class_initialized(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn free_memory(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn full_fence(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_address(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
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
async fn get_byte_1(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_byte_2(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_byte_volatile(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_char_1(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_char_2(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_char_volatile(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_double_1(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_double_2(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_double_volatile(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_float_1(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_float_2(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_float_volatile(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_int_1(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_int_2(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_int_volatile(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_load_average(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_long_1(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_long_2(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
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
async fn get_object_volatile(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_short_1(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_short_2(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_short_volatile(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn load_fence(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn monitor_enter(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn monitor_exit(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn object_field_offset(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
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
async fn put_address(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
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
async fn put_byte_1(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn put_byte_2(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn put_byte_volatile(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn put_char_1(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn put_char_2(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn put_char_volatile(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn put_double_1(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn put_double_2(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn put_double_volatile(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn put_float_1(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn put_float_2(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn put_float_volatile(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn put_int_1(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn put_int_2(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn put_int_volatile(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn put_long_1(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn put_long_2(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
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
async fn put_ordered_int(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn put_ordered_long(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn put_ordered_object(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn put_short_1(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn put_short_2(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn put_short_volatile(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn reallocate_memory(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_memory(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn should_be_initialized(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn static_field_base(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn static_field_offset(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
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
async fn try_monitor_enter(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn unpark(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
