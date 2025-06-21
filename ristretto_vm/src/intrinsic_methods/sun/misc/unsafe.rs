use crate::Result;
use crate::intrinsic_methods::jdk;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("sun/misc/Unsafe.addressSize()I", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn address_size(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::address_size_0(thread, parameters).await
}

#[intrinsic_method(
    "sun/misc/Unsafe.allocateInstance(Ljava/lang/Class;)Ljava/lang/Object;",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn allocate_instance(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::allocate_instance(thread, parameters).await
}

#[intrinsic_method("sun/misc/Unsafe.allocateMemory(J)J", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn allocate_memory(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::allocate_memory_0(thread, parameters).await
}

#[intrinsic_method(
    "sun/misc/Unsafe.arrayBaseOffset(Ljava/lang/Class;)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn array_base_offset(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::array_base_offset_0(thread, parameters).await
}

#[intrinsic_method(
    "sun/misc/Unsafe.arrayIndexScale(Ljava/lang/Class;)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn array_index_scale(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::array_index_scale_0(thread, parameters).await
}

#[intrinsic_method(
    "sun/misc/Unsafe.compareAndSwapInt(Ljava/lang/Object;JII)Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn compare_and_swap_int(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::compare_and_set_int(thread, parameters).await
}

#[intrinsic_method(
    "sun/misc/Unsafe.compareAndSwapLong(Ljava/lang/Object;JJJ)Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn compare_and_swap_long(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::compare_and_set_long(thread, parameters).await
}

#[intrinsic_method(
    "sun/misc/Unsafe.compareAndSwapObject(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn compare_and_swap_object(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::compare_and_set_reference(thread, parameters).await
}

#[intrinsic_method(
    "sun/misc/Unsafe.copyMemory(Ljava/lang/Object;JLjava/lang/Object;JJ)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn copy_memory(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::copy_memory_0(thread, parameters).await
}

#[intrinsic_method(
    "sun/misc/Unsafe.defineAnonymousClass(Ljava/lang/Class;[B[Ljava/lang/Object;)Ljava/lang/Class;",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn define_anonymous_class(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::define_anonymous_class_0(thread, parameters).await
}

#[intrinsic_method(
    "sun/misc/Unsafe.defineClass(Ljava/lang/String;[BIILjava/lang/ClassLoader;Ljava/security/ProtectionDomain;)Ljava/lang/Class;",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn define_class(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::define_class_0(thread, parameters).await
}

#[intrinsic_method(
    "sun/misc/Unsafe.ensureClassInitialized(Ljava/lang/Class;)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn ensure_class_initialized(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::ensure_class_initialized_0(thread, parameters).await
}

#[intrinsic_method("sun/misc/Unsafe.freeMemory(J)V", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn free_memory(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::free_memory_0(thread, parameters).await
}

#[intrinsic_method("sun/misc/Unsafe.fullFence()V", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn full_fence(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::full_fence(thread, parameters).await
}

#[intrinsic_method("sun/misc/Unsafe.getAddress(J)J", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn get_address(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.getAddress(J)J")
}

#[intrinsic_method(
    "sun/misc/Unsafe.getBoolean(Ljava/lang/Object;J)Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_boolean(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::get_boolean(thread, parameters).await
}

#[intrinsic_method(
    "sun/misc/Unsafe.getBooleanVolatile(Ljava/lang/Object;J)Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_boolean_volatile(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::get_boolean_volatile(thread, parameters).await
}

#[intrinsic_method("sun/misc/Unsafe.getByte(J)B", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn get_byte_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.getByte(J)B")
}

#[intrinsic_method(
    "sun/misc/Unsafe.getByte(Ljava/lang/Object;J)B",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_byte_2(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::get_byte(thread, parameters).await
}

#[intrinsic_method(
    "sun/misc/Unsafe.getByteVolatile(Ljava/lang/Object;J)B",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_byte_volatile(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::get_byte_volatile(thread, parameters).await
}

#[intrinsic_method("sun/misc/Unsafe.getChar(J)C", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn get_char_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.getChar(J)C")
}

#[intrinsic_method(
    "sun/misc/Unsafe.getChar(Ljava/lang/Object;J)C",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_char_2(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::get_char(thread, parameters).await
}

#[intrinsic_method(
    "sun/misc/Unsafe.getCharVolatile(Ljava/lang/Object;J)C",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_char_volatile(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::get_char_volatile(thread, parameters).await
}

#[intrinsic_method("sun/misc/Unsafe.getDouble(J)D", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn get_double_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.getDouble(J)D")
}

#[intrinsic_method(
    "sun/misc/Unsafe.getDouble(Ljava/lang/Object;J)D",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_double_2(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::get_double(thread, parameters).await
}

#[intrinsic_method(
    "sun/misc/Unsafe.getDoubleVolatile(Ljava/lang/Object;J)D",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_double_volatile(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::get_double_volatile(thread, parameters).await
}

#[intrinsic_method("sun/misc/Unsafe.getFloat(J)F", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn get_float_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.getFloat(J)F")
}

#[intrinsic_method(
    "sun/misc/Unsafe.getFloat(Ljava/lang/Object;J)F",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_float_2(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::get_float(thread, parameters).await
}

#[intrinsic_method(
    "sun/misc/Unsafe.getFloatVolatile(Ljava/lang/Object;J)F",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_float_volatile(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::get_float_volatile(thread, parameters).await
}

#[intrinsic_method("sun/misc/Unsafe.getInt(J)I", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn get_int_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.getInt(J)I")
}

#[intrinsic_method(
    "sun/misc/Unsafe.getInt(Ljava/lang/Object;J)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_int_2(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::get_int(thread, parameters).await
}

#[intrinsic_method(
    "sun/misc/Unsafe.getIntVolatile(Ljava/lang/Object;J)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_int_volatile(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::get_int_volatile(thread, parameters).await
}

#[intrinsic_method("sun/misc/Unsafe.getLoadAverage([DI)I", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn get_load_average(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::get_load_average_0(thread, parameters).await
}

#[intrinsic_method("sun/misc/Unsafe.getLong(J)J", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn get_long_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.getLong(J)J")
}

#[intrinsic_method(
    "sun/misc/Unsafe.getLong(Ljava/lang/Object;J)J",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_long_2(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::get_long(thread, parameters).await
}

#[intrinsic_method(
    "sun/misc/Unsafe.getLongVolatile(Ljava/lang/Object;J)J",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_long_volatile(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::get_long_volatile(thread, parameters).await
}

#[intrinsic_method(
    "sun/misc/Unsafe.getObject(Ljava/lang/Object;J)Ljava/lang/Object;",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_object(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::get_reference(thread, parameters).await
}

#[intrinsic_method(
    "sun/misc/Unsafe.getObjectVolatile(Ljava/lang/Object;J)Ljava/lang/Object;",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_object_volatile(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::get_reference_volatile(thread, parameters).await
}

#[intrinsic_method("sun/misc/Unsafe.getShort(J)S", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn get_short_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.getShort(J)S")
}

#[intrinsic_method(
    "sun/misc/Unsafe.getShort(Ljava/lang/Object;J)S",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_short_2(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::get_short(thread, parameters).await
}

#[intrinsic_method(
    "sun/misc/Unsafe.getShortVolatile(Ljava/lang/Object;J)S",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_short_volatile(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::get_short_volatile(thread, parameters).await
}

#[intrinsic_method("sun/misc/Unsafe.loadFence()V", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn load_fence(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::load_fence(thread, parameters).await
}

#[intrinsic_method(
    "sun/misc/Unsafe.monitorEnter(Ljava/lang/Object;)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn monitor_enter(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.monitorEnter(Ljava/lang/Object;)V")
}

#[intrinsic_method(
    "sun/misc/Unsafe.monitorExit(Ljava/lang/Object;)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn monitor_exit(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.monitorExit(Ljava/lang/Object;)V")
}

#[intrinsic_method(
    "sun/misc/Unsafe.objectFieldOffset(Ljava/lang/reflect/Field;)J",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn object_field_offset(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::object_field_offset_0(thread, parameters).await
}

#[intrinsic_method("sun/misc/Unsafe.pageSize()I", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn page_size(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::page_size(thread, parameters).await
}

#[intrinsic_method("sun/misc/Unsafe.park(ZJ)V", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn park(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::park(thread, parameters).await
}

#[intrinsic_method("sun/misc/Unsafe.putAddress(JJ)V", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn put_address(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.putAddress(JJ)V")
}

#[intrinsic_method(
    "sun/misc/Unsafe.putBoolean(Ljava/lang/Object;JZ)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn put_boolean(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::put_boolean(thread, parameters).await
}

#[intrinsic_method(
    "sun/misc/Unsafe.putBooleanVolatile(Ljava/lang/Object;JZ)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn put_boolean_volatile(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::put_boolean_volatile(thread, parameters).await
}

#[intrinsic_method("sun/misc/Unsafe.putByte(JB)V", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn put_byte_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.putByte(JB)V")
}

#[intrinsic_method(
    "sun/misc/Unsafe.putByte(Ljava/lang/Object;JB)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn put_byte_2(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::put_byte(thread, parameters).await
}

#[intrinsic_method(
    "sun/misc/Unsafe.putByteVolatile(Ljava/lang/Object;JB)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn put_byte_volatile(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::put_byte_volatile(thread, parameters).await
}

#[intrinsic_method("sun/misc/Unsafe.putChar(JC)V", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn put_char_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.putChar(JC)V")
}

#[intrinsic_method(
    "sun/misc/Unsafe.putChar(Ljava/lang/Object;JC)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn put_char_2(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::put_char(thread, parameters).await
}

#[intrinsic_method(
    "sun/misc/Unsafe.putCharVolatile(Ljava/lang/Object;JC)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn put_char_volatile(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::put_char_volatile(thread, parameters).await
}

#[intrinsic_method("sun/misc/Unsafe.putDouble(JD)V", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn put_double_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.putDouble(JD)V")
}

#[intrinsic_method(
    "sun/misc/Unsafe.putDouble(Ljava/lang/Object;JD)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn put_double_2(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::put_double(thread, parameters).await
}

#[intrinsic_method(
    "sun/misc/Unsafe.putDoubleVolatile(Ljava/lang/Object;JD)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn put_double_volatile(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::put_double_volatile(thread, parameters).await
}

#[intrinsic_method("sun/misc/Unsafe.putFloat(JF)V", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn put_float_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.putFloat(JF)V")
}

#[intrinsic_method(
    "sun/misc/Unsafe.putFloat(Ljava/lang/Object;JF)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn put_float_2(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::put_float(thread, parameters).await
}

#[intrinsic_method(
    "sun/misc/Unsafe.putFloatVolatile(Ljava/lang/Object;JF)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn put_float_volatile(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::put_float_volatile(thread, parameters).await
}

#[intrinsic_method("sun/misc/Unsafe.putInt(JI)V", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn put_int_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.putInt(JI)V")
}

#[intrinsic_method(
    "sun/misc/Unsafe.putInt(Ljava/lang/Object;JI)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn put_int_2(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::put_int(thread, parameters).await
}

#[intrinsic_method(
    "sun/misc/Unsafe.putIntVolatile(Ljava/lang/Object;JI)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn put_int_volatile(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::put_int_volatile(thread, parameters).await
}

#[intrinsic_method("sun/misc/Unsafe.putLong(JJ)V", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn put_long_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.putLong(JJ)V")
}

#[intrinsic_method(
    "sun/misc/Unsafe.putLong(Ljava/lang/Object;JJ)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn put_long_2(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::put_long(thread, parameters).await
}

#[intrinsic_method(
    "sun/misc/Unsafe.putLongVolatile(Ljava/lang/Object;JJ)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn put_long_volatile(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::put_long_volatile(thread, parameters).await
}

#[intrinsic_method(
    "sun/misc/Unsafe.putObject(Ljava/lang/Object;JLjava/lang/Object;)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn put_object(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::put_reference(thread, parameters).await
}

#[intrinsic_method(
    "sun/misc/Unsafe.putObjectVolatile(Ljava/lang/Object;JLjava/lang/Object;)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn put_object_volatile(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::put_reference_volatile(thread, parameters).await
}

#[intrinsic_method(
    "sun/misc/Unsafe.putOrderedInt(Ljava/lang/Object;JI)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn put_ordered_int(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.putOrderedInt(Ljava/lang/Object;JI)V")
}

#[intrinsic_method(
    "sun/misc/Unsafe.putOrderedLong(Ljava/lang/Object;JJ)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn put_ordered_long(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.putOrderedLong(Ljava/lang/Object;JJ)V")
}

#[intrinsic_method(
    "sun/misc/Unsafe.putOrderedObject(Ljava/lang/Object;JLjava/lang/Object;)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn put_ordered_object(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.putOrderedObject(Ljava/lang/Object;JLjava/lang/Object;)V")
}

#[intrinsic_method("sun/misc/Unsafe.putShort(JS)V", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn put_short_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.putShort(JS)V")
}

#[intrinsic_method(
    "sun/misc/Unsafe.putShort(Ljava/lang/Object;JS)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn put_short_2(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::put_short(thread, parameters).await
}

#[intrinsic_method(
    "sun/misc/Unsafe.putShortVolatile(Ljava/lang/Object;JS)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn put_short_volatile(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::put_short_volatile(thread, parameters).await
}

#[intrinsic_method("sun/misc/Unsafe.reallocateMemory(JJ)J", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn reallocate_memory(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::reallocate_memory_0(thread, parameters).await
}

#[intrinsic_method("sun/misc/Unsafe.registerNatives()V", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn register_natives(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "sun/misc/Unsafe.setMemory(Ljava/lang/Object;JJB)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn set_memory(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::set_memory_0(thread, parameters).await
}

#[intrinsic_method(
    "sun/misc/Unsafe.shouldBeInitialized(Ljava/lang/Class;)Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn should_be_initialized(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::should_be_initialized_0(thread, parameters).await
}

#[intrinsic_method(
    "sun/misc/Unsafe.staticFieldBase(Ljava/lang/reflect/Field;)Ljava/lang/Object;",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn static_field_base(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::static_field_base_0(thread, parameters).await
}

#[intrinsic_method(
    "sun/misc/Unsafe.staticFieldOffset(Ljava/lang/reflect/Field;)J",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn static_field_offset(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::static_field_offset_0(thread, parameters).await
}

#[intrinsic_method("sun/misc/Unsafe.storeFence()V", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn store_fence(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::store_fence(thread, parameters).await
}

#[intrinsic_method(
    "sun/misc/Unsafe.throwException(Ljava/lang/Throwable;)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn throw_exception(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::throw_exception(thread, parameters).await
}

#[intrinsic_method(
    "sun/misc/Unsafe.tryMonitorEnter(Ljava/lang/Object;)Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn try_monitor_enter(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.Unsafe.tryMonitorEnter(Ljava/lang/Object;)Z")
}

#[intrinsic_method("sun/misc/Unsafe.unpark(Ljava/lang/Object;)V", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn unpark(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::misc::r#unsafe::unpark(thread, parameters).await
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
    async fn test_address_size() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = address_size(thread, Parameters::default()).await?;
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
    async fn test_allocate_memory() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = allocate_memory(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_array_base_offset() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = array_base_offset(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_array_index_scale() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = array_index_scale(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_ensure_class_initialized() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = ensure_class_initialized(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_free_memory() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = free_memory(thread, Parameters::default()).await?;
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
    async fn test_load_fence() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = load_fence(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
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
    async fn test_object_field_offset() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let field = create_field(&thread).await?;
        let mut parameters = Parameters::default();
        parameters.push(field);
        let value = object_field_offset(thread, parameters)
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
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = register_natives(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_should_be_initialized() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = should_be_initialized(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::from(false)));
        Ok(())
    }

    #[tokio::test]
    async fn test_static_field_base() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let field = create_field(&thread).await?;
        let mut parameters = Parameters::default();
        parameters.push(field);
        let value = static_field_base(thread, parameters)
            .await?
            .expect("object");
        assert_eq!(value, Value::Object(None));
        Ok(())
    }

    #[tokio::test]
    async fn test_static_field_offset() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let field = create_field(&thread).await?;
        let mut parameters = Parameters::default();
        parameters.push(field);
        let value = static_field_offset(thread, parameters)
            .await?
            .expect("offset");
        let offset: i64 = value.try_into()?;
        assert_eq!(offset, 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_store_fence() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = crate::intrinsic_methods::jdk::internal::misc::r#unsafe::store_fence(
            thread,
            Parameters::default(),
        )
        .await?;
        assert_eq!(result, None);
        Ok(())
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
