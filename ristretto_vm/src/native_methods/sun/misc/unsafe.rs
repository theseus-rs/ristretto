use crate::Result;
use crate::native_methods::jdk;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/misc/Unsafe";

/// Register all native methods for `sun.misc.Unsafe`.
#[expect(clippy::too_many_lines)]
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "addressSize", "()I", address_size);
    registry.register(
        CLASS_NAME,
        "allocateInstance",
        "(Ljava/lang/Class;)Ljava/lang/Object;",
        allocate_instance,
    );
    registry.register(CLASS_NAME, "allocateMemory", "(J)J", allocate_memory);
    registry.register(
        CLASS_NAME,
        "arrayBaseOffset",
        "(Ljava/lang/Class;)I",
        array_base_offset,
    );
    registry.register(
        CLASS_NAME,
        "arrayIndexScale",
        "(Ljava/lang/Class;)I",
        array_index_scale,
    );
    registry.register(
        CLASS_NAME,
        "compareAndSwapInt",
        "(Ljava/lang/Object;JII)Z",
        compare_and_swap_int,
    );
    registry.register(
        CLASS_NAME,
        "compareAndSwapLong",
        "(Ljava/lang/Object;JJJ)Z",
        compare_and_swap_long,
    );
    registry.register(
        CLASS_NAME,
        "compareAndSwapObject",
        "(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z",
        compare_and_swap_object,
    );
    registry.register(
        CLASS_NAME,
        "copyMemory",
        "(Ljava/lang/Object;JLjava/lang/Object;JJ)V",
        copy_memory,
    );
    registry.register(
        CLASS_NAME,
        "defineAnonymousClass",
        "(Ljava/lang/Class;[B[Ljava/lang/Object;)Ljava/lang/Class;",
        define_anonymous_class,
    );
    registry.register(CLASS_NAME, "defineClass", "(Ljava/lang/String;[BIILjava/lang/ClassLoader;Ljava/security/ProtectionDomain;)Ljava/lang/Class;", define_class);
    registry.register(
        CLASS_NAME,
        "ensureClassInitialized",
        "(Ljava/lang/Class;)V",
        ensure_class_initialized,
    );
    registry.register(CLASS_NAME, "freeMemory", "(J)V", free_memory);
    registry.register(CLASS_NAME, "fullFence", "()V", full_fence);
    registry.register(CLASS_NAME, "getAddress", "(J)J", get_address);
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
    registry.register(CLASS_NAME, "getByte", "(J)B", get_byte_1);
    registry.register(CLASS_NAME, "getByte", "(Ljava/lang/Object;J)B", get_byte_2);
    registry.register(
        CLASS_NAME,
        "getByteVolatile",
        "(Ljava/lang/Object;J)B",
        get_byte_volatile,
    );
    registry.register(CLASS_NAME, "getChar", "(J)C", get_char_1);
    registry.register(CLASS_NAME, "getChar", "(Ljava/lang/Object;J)C", get_char_2);
    registry.register(
        CLASS_NAME,
        "getCharVolatile",
        "(Ljava/lang/Object;J)C",
        get_char_volatile,
    );
    registry.register(CLASS_NAME, "getDouble", "(J)D", get_double_1);
    registry.register(
        CLASS_NAME,
        "getDouble",
        "(Ljava/lang/Object;J)D",
        get_double_2,
    );
    registry.register(
        CLASS_NAME,
        "getDoubleVolatile",
        "(Ljava/lang/Object;J)D",
        get_double_volatile,
    );
    registry.register(CLASS_NAME, "getFloat", "(J)F", get_float_1);
    registry.register(
        CLASS_NAME,
        "getFloat",
        "(Ljava/lang/Object;J)F",
        get_float_2,
    );
    registry.register(
        CLASS_NAME,
        "getFloatVolatile",
        "(Ljava/lang/Object;J)F",
        get_float_volatile,
    );
    registry.register(CLASS_NAME, "getInt", "(J)I", get_int_1);
    registry.register(CLASS_NAME, "getInt", "(Ljava/lang/Object;J)I", get_int_2);
    registry.register(
        CLASS_NAME,
        "getIntVolatile",
        "(Ljava/lang/Object;J)I",
        get_int_volatile,
    );
    registry.register(CLASS_NAME, "getLoadAverage", "([DI)I", get_load_average);
    registry.register(CLASS_NAME, "getLong", "(J)J", get_long_1);
    registry.register(CLASS_NAME, "getLong", "(Ljava/lang/Object;J)J", get_long_2);
    registry.register(
        CLASS_NAME,
        "getLongVolatile",
        "(Ljava/lang/Object;J)J",
        get_long_volatile,
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
    registry.register(CLASS_NAME, "getShort", "(J)S", get_short_1);
    registry.register(
        CLASS_NAME,
        "getShort",
        "(Ljava/lang/Object;J)S",
        get_short_2,
    );
    registry.register(
        CLASS_NAME,
        "getShortVolatile",
        "(Ljava/lang/Object;J)S",
        get_short_volatile,
    );
    registry.register(CLASS_NAME, "loadFence", "()V", load_fence);
    registry.register(
        CLASS_NAME,
        "monitorEnter",
        "(Ljava/lang/Object;)V",
        monitor_enter,
    );
    registry.register(
        CLASS_NAME,
        "monitorExit",
        "(Ljava/lang/Object;)V",
        monitor_exit,
    );
    registry.register(
        CLASS_NAME,
        "objectFieldOffset",
        "(Ljava/lang/reflect/Field;)J",
        object_field_offset,
    );
    registry.register(CLASS_NAME, "pageSize", "()I", page_size);
    registry.register(CLASS_NAME, "park", "(ZJ)V", park);
    registry.register(CLASS_NAME, "putAddress", "(JJ)V", put_address);
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
    registry.register(CLASS_NAME, "putByte", "(JB)V", put_byte_1);
    registry.register(CLASS_NAME, "putByte", "(Ljava/lang/Object;JB)V", put_byte_2);
    registry.register(
        CLASS_NAME,
        "putByteVolatile",
        "(Ljava/lang/Object;JB)V",
        put_byte_volatile,
    );
    registry.register(CLASS_NAME, "putChar", "(JC)V", put_char_1);
    registry.register(CLASS_NAME, "putChar", "(Ljava/lang/Object;JC)V", put_char_2);
    registry.register(
        CLASS_NAME,
        "putCharVolatile",
        "(Ljava/lang/Object;JC)V",
        put_char_volatile,
    );
    registry.register(CLASS_NAME, "putDouble", "(JD)V", put_double_1);
    registry.register(
        CLASS_NAME,
        "putDouble",
        "(Ljava/lang/Object;JD)V",
        put_double_2,
    );
    registry.register(
        CLASS_NAME,
        "putDoubleVolatile",
        "(Ljava/lang/Object;JD)V",
        put_double_volatile,
    );
    registry.register(CLASS_NAME, "putFloat", "(JF)V", put_float_1);
    registry.register(
        CLASS_NAME,
        "putFloat",
        "(Ljava/lang/Object;JF)V",
        put_float_2,
    );
    registry.register(
        CLASS_NAME,
        "putFloatVolatile",
        "(Ljava/lang/Object;JF)V",
        put_float_volatile,
    );
    registry.register(CLASS_NAME, "putInt", "(JI)V", put_int_1);
    registry.register(CLASS_NAME, "putInt", "(Ljava/lang/Object;JI)V", put_int_2);
    registry.register(
        CLASS_NAME,
        "putIntVolatile",
        "(Ljava/lang/Object;JI)V",
        put_int_volatile,
    );
    registry.register(CLASS_NAME, "putLong", "(JJ)V", put_long_1);
    registry.register(CLASS_NAME, "putLong", "(Ljava/lang/Object;JJ)V", put_long_2);
    registry.register(
        CLASS_NAME,
        "putLongVolatile",
        "(Ljava/lang/Object;JJ)V",
        put_long_volatile,
    );
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
    registry.register(
        CLASS_NAME,
        "putOrderedInt",
        "(Ljava/lang/Object;JI)V",
        put_ordered_int,
    );
    registry.register(
        CLASS_NAME,
        "putOrderedLong",
        "(Ljava/lang/Object;JJ)V",
        put_ordered_long,
    );
    registry.register(
        CLASS_NAME,
        "putOrderedObject",
        "(Ljava/lang/Object;JLjava/lang/Object;)V",
        put_ordered_object,
    );
    registry.register(CLASS_NAME, "putShort", "(JS)V", put_short_1);
    registry.register(
        CLASS_NAME,
        "putShort",
        "(Ljava/lang/Object;JS)V",
        put_short_2,
    );
    registry.register(
        CLASS_NAME,
        "putShortVolatile",
        "(Ljava/lang/Object;JS)V",
        put_short_volatile,
    );
    registry.register(CLASS_NAME, "reallocateMemory", "(JJ)J", reallocate_memory);
    registry.register(CLASS_NAME, "registerNatives", "()V", register_natives);
    registry.register(
        CLASS_NAME,
        "setMemory",
        "(Ljava/lang/Object;JJB)V",
        set_memory,
    );
    registry.register(
        CLASS_NAME,
        "shouldBeInitialized",
        "(Ljava/lang/Class;)Z",
        should_be_initialized,
    );
    registry.register(
        CLASS_NAME,
        "staticFieldBase",
        "(Ljava/lang/reflect/Field;)Ljava/lang/Object;",
        static_field_base,
    );
    registry.register(
        CLASS_NAME,
        "staticFieldOffset",
        "(Ljava/lang/reflect/Field;)J",
        static_field_offset,
    );
    registry.register(CLASS_NAME, "storeFence", "()V", store_fence);
    registry.register(
        CLASS_NAME,
        "throwException",
        "(Ljava/lang/Throwable;)V",
        throw_exception,
    );
    registry.register(
        CLASS_NAME,
        "tryMonitorEnter",
        "(Ljava/lang/Object;)Z",
        try_monitor_enter,
    );
    registry.register(CLASS_NAME, "unpark", "(Ljava/lang/Object;)V", unpark);
}

#[async_recursion(?Send)]
async fn address_size(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::address_size_0(thread, parameters).await
}

#[async_recursion(?Send)]
async fn allocate_instance(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::allocate_instance(thread, parameters).await
}

#[async_recursion(?Send)]
async fn allocate_memory(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::allocate_memory_0(thread, parameters).await
}

#[async_recursion(?Send)]
async fn array_base_offset(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::array_base_offset_0(thread, parameters).await
}

#[async_recursion(?Send)]
async fn array_index_scale(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::array_index_scale_0(thread, parameters).await
}

#[async_recursion(?Send)]
async fn compare_and_swap_int(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::compare_and_set_int(thread, parameters).await
}

#[async_recursion(?Send)]
async fn compare_and_swap_long(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.compareAndSwapLong(Ljava/lang/Object;JJJ)Z")
}

#[async_recursion(?Send)]
async fn compare_and_swap_object(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::compare_and_set_reference(thread, parameters).await
}

#[async_recursion(?Send)]
async fn copy_memory(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::copy_memory_0(thread, parameters).await
}

#[async_recursion(?Send)]
async fn define_anonymous_class(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::define_anonymous_class_0(thread, parameters).await
}

#[async_recursion(?Send)]
async fn define_class(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::define_class_0(thread, parameters).await
}

#[async_recursion(?Send)]
async fn ensure_class_initialized(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::ensure_class_initialized_0(thread, parameters).await
}

#[async_recursion(?Send)]
async fn free_memory(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::free_memory_0(thread, parameters).await
}

#[async_recursion(?Send)]
async fn full_fence(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::full_fence(thread, parameters).await
}

#[async_recursion(?Send)]
async fn get_address(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.getAddress(J)J")
}

#[async_recursion(?Send)]
async fn get_boolean(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::get_boolean(thread, parameters).await
}

#[async_recursion(?Send)]
async fn get_boolean_volatile(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::get_boolean_volatile(thread, parameters).await
}

#[async_recursion(?Send)]
async fn get_byte_1(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.getByte(J)B")
}

#[async_recursion(?Send)]
async fn get_byte_2(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::get_byte(thread, parameters).await
}

#[async_recursion(?Send)]
async fn get_byte_volatile(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::get_byte_volatile(thread, parameters).await
}

#[async_recursion(?Send)]
async fn get_char_1(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.getChar(J)C")
}

#[async_recursion(?Send)]
async fn get_char_2(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::get_char(thread, parameters).await
}

#[async_recursion(?Send)]
async fn get_char_volatile(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::get_char_volatile(thread, parameters).await
}

#[async_recursion(?Send)]
async fn get_double_1(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.getDouble(J)D")
}

#[async_recursion(?Send)]
async fn get_double_2(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::get_double(thread, parameters).await
}

#[async_recursion(?Send)]
async fn get_double_volatile(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::get_double_volatile(thread, parameters).await
}

#[async_recursion(?Send)]
async fn get_float_1(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.getFloat(J)F")
}

#[async_recursion(?Send)]
async fn get_float_2(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::get_float(thread, parameters).await
}

#[async_recursion(?Send)]
async fn get_float_volatile(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::get_float_volatile(thread, parameters).await
}

#[async_recursion(?Send)]
async fn get_int_1(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.getInt(J)I")
}

#[async_recursion(?Send)]
async fn get_int_2(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::get_int(thread, parameters).await
}

#[async_recursion(?Send)]
async fn get_int_volatile(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::get_int_volatile(thread, parameters).await
}

#[async_recursion(?Send)]
async fn get_load_average(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::get_load_average_0(thread, parameters).await
}

#[async_recursion(?Send)]
async fn get_long_1(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.getLong(J)J")
}

#[async_recursion(?Send)]
async fn get_long_2(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::get_long(thread, parameters).await
}

#[async_recursion(?Send)]
async fn get_long_volatile(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::get_long_volatile(thread, parameters).await
}

#[async_recursion(?Send)]
async fn get_object(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::get_reference(thread, parameters).await
}

#[async_recursion(?Send)]
async fn get_object_volatile(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::get_reference_volatile(thread, parameters).await
}

#[async_recursion(?Send)]
async fn get_short_1(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.getShort(J)S")
}

#[async_recursion(?Send)]
async fn get_short_2(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::get_short(thread, parameters).await
}

#[async_recursion(?Send)]
async fn get_short_volatile(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::get_short_volatile(thread, parameters).await
}

#[async_recursion(?Send)]
async fn load_fence(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::load_fence(thread, parameters).await
}

#[async_recursion(?Send)]
async fn monitor_enter(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.monitorEnter(Ljava/lang/Object;)V")
}

#[async_recursion(?Send)]
async fn monitor_exit(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.monitorExit(Ljava/lang/Object;)V")
}

#[async_recursion(?Send)]
async fn object_field_offset(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::object_field_offset_0(thread, parameters).await
}

#[async_recursion(?Send)]
async fn page_size(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::page_size(thread, parameters).await
}

#[async_recursion(?Send)]
async fn park(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::park(thread, parameters).await
}

#[async_recursion(?Send)]
async fn put_address(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.putAddress(JJ)V")
}

#[async_recursion(?Send)]
async fn put_boolean(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::put_boolean(thread, parameters).await
}

#[async_recursion(?Send)]
async fn put_boolean_volatile(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::put_boolean_volatile(thread, parameters).await
}

#[async_recursion(?Send)]
async fn put_byte_1(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.putByte(JB)V")
}

#[async_recursion(?Send)]
async fn put_byte_2(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::put_byte(thread, parameters).await
}

#[async_recursion(?Send)]
async fn put_byte_volatile(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::put_byte_volatile(thread, parameters).await
}

#[async_recursion(?Send)]
async fn put_char_1(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.putChar(JC)V")
}

#[async_recursion(?Send)]
async fn put_char_2(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::put_char(thread, parameters).await
}

#[async_recursion(?Send)]
async fn put_char_volatile(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::put_char_volatile(thread, parameters).await
}

#[async_recursion(?Send)]
async fn put_double_1(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.putDouble(JD)V")
}

#[async_recursion(?Send)]
async fn put_double_2(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::put_double(thread, parameters).await
}

#[async_recursion(?Send)]
async fn put_double_volatile(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::put_double_volatile(thread, parameters).await
}

#[async_recursion(?Send)]
async fn put_float_1(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.putFloat(JF)V")
}

#[async_recursion(?Send)]
async fn put_float_2(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::put_float(thread, parameters).await
}

#[async_recursion(?Send)]
async fn put_float_volatile(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::put_float_volatile(thread, parameters).await
}

#[async_recursion(?Send)]
async fn put_int_1(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.putInt(JI)V")
}

#[async_recursion(?Send)]
async fn put_int_2(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::put_int(thread, parameters).await
}

#[async_recursion(?Send)]
async fn put_int_volatile(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::put_int_volatile(thread, parameters).await
}

#[async_recursion(?Send)]
async fn put_long_1(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.putLong(JJ)V")
}

#[async_recursion(?Send)]
async fn put_long_2(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::put_long(thread, parameters).await
}

#[async_recursion(?Send)]
async fn put_long_volatile(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::put_long_volatile(thread, parameters).await
}

#[async_recursion(?Send)]
async fn put_object(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::put_reference(thread, parameters).await
}

#[async_recursion(?Send)]
async fn put_object_volatile(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::put_reference_volatile(thread, parameters).await
}

#[async_recursion(?Send)]
async fn put_ordered_int(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.putOrderedInt(Ljava/lang/Object;JI)V")
}

#[async_recursion(?Send)]
async fn put_ordered_long(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.putOrderedLong(Ljava/lang/Object;JJ)V")
}

#[async_recursion(?Send)]
async fn put_ordered_object(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.putOrderedObject(Ljava/lang/Object;JLjava/lang/Object;)V")
}

#[async_recursion(?Send)]
async fn put_short_1(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.putShort(JS)V")
}

#[async_recursion(?Send)]
async fn put_short_2(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.putShort(Ljava/lang/Object;JS)V")
}

#[async_recursion(?Send)]
async fn put_short_volatile(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.putShortVolatile(Ljava/lang/Object;JS)V")
}

#[async_recursion(?Send)]
async fn reallocate_memory(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::reallocate_memory_0(thread, parameters).await
}

#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn set_memory(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::set_memory_0(thread, parameters).await
}

#[async_recursion(?Send)]
async fn should_be_initialized(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::should_be_initialized_0(thread, parameters).await
}

#[async_recursion(?Send)]
async fn static_field_base(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::static_field_base_0(thread, parameters).await
}

#[async_recursion(?Send)]
async fn static_field_offset(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::static_field_offset_0(thread, parameters).await
}

#[async_recursion(?Send)]
async fn store_fence(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::store_fence(thread, parameters).await
}

#[async_recursion(?Send)]
async fn throw_exception(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::throw_exception(thread, parameters).await
}

#[async_recursion(?Send)]
async fn try_monitor_enter(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.tryMonitorEnter(Ljava/lang/Object;)Z")
}

#[async_recursion(?Send)]
async fn unpark(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::unpark(thread, parameters).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.misc.Unsafe.compareAndSwapLong(Ljava/lang/Object;JJJ)Z"
    )]
    async fn test_compare_and_swap_long() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = compare_and_swap_long(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.misc.Unsafe.getAddress(J)J")]
    async fn test_get_address() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_address(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.misc.Unsafe.getByte(J)B")]
    async fn test_get_byte_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_byte_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.misc.Unsafe.getChar(J)C")]
    async fn test_get_char_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_char_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.misc.Unsafe.getDouble(J)D")]
    async fn test_get_double_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_double_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.misc.Unsafe.getFloat(J)F")]
    async fn test_get_float_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_float_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.misc.Unsafe.getInt(J)I")]
    async fn test_get_int_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_int_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.misc.Unsafe.getLong(J)J")]
    async fn test_get_long_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_long_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.misc.Unsafe.getShort(J)S")]
    async fn test_get_short_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_short_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.misc.Unsafe.monitorEnter(Ljava/lang/Object;)V"
    )]
    async fn test_monitor_enter() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = monitor_enter(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.misc.Unsafe.monitorExit(Ljava/lang/Object;)V"
    )]
    async fn test_monitor_exit() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = monitor_exit(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.misc.Unsafe.putAddress(JJ)V")]
    async fn test_put_address() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_address(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.misc.Unsafe.putByte(JB)V")]
    async fn test_put_byte_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_byte_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.misc.Unsafe.putChar(JC)V")]
    async fn test_put_char_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_char_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.misc.Unsafe.putDouble(JD)V")]
    async fn test_put_double_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_double_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.misc.Unsafe.putFloat(JF)V")]
    async fn test_put_float_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_float_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.misc.Unsafe.putInt(JI)V")]
    async fn test_put_int_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_int_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.misc.Unsafe.putLong(JJ)V")]
    async fn test_put_long_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_long_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.misc.Unsafe.putOrderedInt(Ljava/lang/Object;JI)V"
    )]
    async fn test_put_ordered_int() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_ordered_int(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.misc.Unsafe.putOrderedLong(Ljava/lang/Object;JJ)V"
    )]
    async fn test_put_ordered_long() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_ordered_long(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.misc.Unsafe.putOrderedObject(Ljava/lang/Object;JLjava/lang/Object;)V"
    )]
    async fn test_put_ordered_object() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_ordered_object(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.misc.Unsafe.putShort(JS)V")]
    async fn test_put_short_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_short_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.misc.Unsafe.putShort(Ljava/lang/Object;JS)V"
    )]
    async fn test_put_short_2() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_short_2(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.misc.Unsafe.putShortVolatile(Ljava/lang/Object;JS)V"
    )]
    async fn test_put_short_volatile() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_short_volatile(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.misc.Unsafe.tryMonitorEnter(Ljava/lang/Object;)Z"
    )]
    async fn test_try_monitor_enter() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = try_monitor_enter(thread, Parameters::default()).await;
    }
}
