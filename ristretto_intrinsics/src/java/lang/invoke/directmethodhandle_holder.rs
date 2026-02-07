//! Intrinsic methods for `java.lang.invoke.DirectMethodHandle$Holder`.
//!
//! These methods are used for direct method invocation and field access in the JVM.

use super::methodhandle::dispatch_holder_method;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::{Any, Equal, GreaterThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

/// Helper function for holder method implementations.
/// Dispatches the method call through the `LambdaForm` interpreter.
async fn holder_method_stub<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    method_name: &str,
    parameters: Parameters,
) -> Result<Option<Value>> {
    let arguments: Vec<Value> = parameters.into_vec();
    let result = dispatch_holder_method(thread, method_name, arguments).await?;
    Ok(Some(result))
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.getBoolean(Ljava/lang/Object;)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_boolean_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.getBoolean(Ljava/lang/Object;)I")
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.getBoolean(Ljava/lang/Object;Ljava/lang/Object;)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_boolean_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getBoolean(Ljava/lang/Object;Ljava/lang/Object;)I"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.getBooleanVolatile(Ljava/lang/Object;)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_boolean_volatile_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.getBooleanVolatile(Ljava/lang/Object;)I")
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.getBooleanVolatile(Ljava/lang/Object;Ljava/lang/Object;)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_boolean_volatile_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getBooleanVolatile(Ljava/lang/Object;Ljava/lang/Object;)I"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.getByte(Ljava/lang/Object;)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_byte_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.getByte(Ljava/lang/Object;)I")
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.getByte(Ljava/lang/Object;Ljava/lang/Object;)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_byte_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getByte(Ljava/lang/Object;Ljava/lang/Object;)I"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.getByteVolatile(Ljava/lang/Object;)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_byte_volatile_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.getByteVolatile(Ljava/lang/Object;)I")
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.getByteVolatile(Ljava/lang/Object;Ljava/lang/Object;)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_byte_volatile_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getByteVolatile(Ljava/lang/Object;Ljava/lang/Object;)I"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.getChar(Ljava/lang/Object;)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_char_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.getChar(Ljava/lang/Object;)I")
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.getChar(Ljava/lang/Object;Ljava/lang/Object;)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_char_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getChar(Ljava/lang/Object;Ljava/lang/Object;)I"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.getCharVolatile(Ljava/lang/Object;)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_char_volatile_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.getCharVolatile(Ljava/lang/Object;)I")
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.getCharVolatile(Ljava/lang/Object;Ljava/lang/Object;)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_char_volatile_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getCharVolatile(Ljava/lang/Object;Ljava/lang/Object;)I"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.getDouble(Ljava/lang/Object;)D",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_double_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.getDouble(Ljava/lang/Object;)D")
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.getDouble(Ljava/lang/Object;Ljava/lang/Object;)D",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_double_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getDouble(Ljava/lang/Object;Ljava/lang/Object;)D"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.getDoubleVolatile(Ljava/lang/Object;)D",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_double_volatile_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.getDoubleVolatile(Ljava/lang/Object;)D")
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.getDoubleVolatile(Ljava/lang/Object;Ljava/lang/Object;)D",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_double_volatile_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getDoubleVolatile(Ljava/lang/Object;Ljava/lang/Object;)D"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.getFloat(Ljava/lang/Object;)F",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_float_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.getFloat(Ljava/lang/Object;)F")
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.getFloat(Ljava/lang/Object;Ljava/lang/Object;)F",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_float_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getFloat(Ljava/lang/Object;Ljava/lang/Object;)F"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.getFloatVolatile(Ljava/lang/Object;)F",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_float_volatile_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.getFloatVolatile(Ljava/lang/Object;)F")
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.getFloatVolatile(Ljava/lang/Object;Ljava/lang/Object;)F",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_float_volatile_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getFloatVolatile(Ljava/lang/Object;Ljava/lang/Object;)F"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.getInt(Ljava/lang/Object;)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_int_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.getInt(Ljava/lang/Object;)I")
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.getInt(Ljava/lang/Object;Ljava/lang/Object;)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_int_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getInt(Ljava/lang/Object;Ljava/lang/Object;)I"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.getIntVolatile(Ljava/lang/Object;)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_int_volatile_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.getIntVolatile(Ljava/lang/Object;)I")
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.getIntVolatile(Ljava/lang/Object;Ljava/lang/Object;)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_int_volatile_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getIntVolatile(Ljava/lang/Object;Ljava/lang/Object;)I"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.getLong(Ljava/lang/Object;)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_long_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.getLong(Ljava/lang/Object;)J")
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.getLong(Ljava/lang/Object;Ljava/lang/Object;)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_long_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getLong(Ljava/lang/Object;Ljava/lang/Object;)J"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.getLongVolatile(Ljava/lang/Object;)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_long_volatile_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.getLongVolatile(Ljava/lang/Object;)J")
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.getLongVolatile(Ljava/lang/Object;Ljava/lang/Object;)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_long_volatile_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getLongVolatile(Ljava/lang/Object;Ljava/lang/Object;)J"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.getObject(Ljava/lang/Object;)Ljava/lang/Object;",
    Equal(JAVA_11)
)]
#[async_method]
pub async fn get_object_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getObject(Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.getObject(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
    Equal(JAVA_11)
)]
#[async_method]
pub async fn get_object_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getObject(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.getObjectVolatile(Ljava/lang/Object;)Ljava/lang/Object;",
    Equal(JAVA_11)
)]
#[async_method]
pub async fn get_object_volatile_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getObjectVolatile(Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.getObjectVolatile(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
    Equal(JAVA_11)
)]
#[async_method]
pub async fn get_object_volatile_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getObjectVolatile(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.getReference(Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_reference_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getReference(Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.getReference([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_reference_1<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    holder_method_stub(thread, "getReference", parameters).await
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.getReference(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_reference_2<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getReference(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.getReferenceVolatile(Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_reference_volatile_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getReferenceVolatile(Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.getReferenceVolatile(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_reference_volatile_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getReferenceVolatile(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.getShort(Ljava/lang/Object;)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_short_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.getShort(Ljava/lang/Object;)I")
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.getShort(Ljava/lang/Object;Ljava/lang/Object;)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_short_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getShort(Ljava/lang/Object;Ljava/lang/Object;)I"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.getShortVolatile(Ljava/lang/Object;)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_short_volatile_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.getShortVolatile(Ljava/lang/Object;)I")
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.getShortVolatile(Ljava/lang/Object;Ljava/lang/Object;)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_short_volatile_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.getShortVolatile(Ljava/lang/Object;Ljava/lang/Object;)I"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.invokeInterface([Ljava/lang/Object;)Ljava/lang/Object;",
    Any
)]
#[async_method]
pub async fn invoke_interface<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    holder_method_stub(thread, "invokeInterface", parameters).await
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.invokeSpecial([Ljava/lang/Object;)Ljava/lang/Object;",
    Any
)]
#[async_method]
pub async fn invoke_special<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    holder_method_stub(thread, "invokeSpecial", parameters).await
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.invokeStatic([Ljava/lang/Object;)Ljava/lang/Object;",
    Any
)]
#[async_method]
pub async fn invoke_static<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    holder_method_stub(thread, "invokeStatic", parameters).await
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.invokeVirtual([Ljava/lang/Object;)Ljava/lang/Object;",
    Any
)]
#[async_method]
pub async fn invoke_virtual<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    holder_method_stub(thread, "invokeVirtual", parameters).await
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.newInvokeSpecial([Ljava/lang/Object;)Ljava/lang/Object;",
    Any
)]
#[async_method]
pub async fn new_invoke_special<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    holder_method_stub(thread, "newInvokeSpecial", parameters).await
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.putBoolean(Ljava/lang/Object;I)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn put_boolean_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.putBoolean(Ljava/lang/Object;I)V")
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.putBoolean(Ljava/lang/Object;Ljava/lang/Object;I)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn put_boolean_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putBoolean(Ljava/lang/Object;Ljava/lang/Object;I)V"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.putBooleanVolatile(Ljava/lang/Object;I)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn put_boolean_volatile_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.putBooleanVolatile(Ljava/lang/Object;I)V")
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.putBooleanVolatile(Ljava/lang/Object;Ljava/lang/Object;I)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn put_boolean_volatile_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putBooleanVolatile(Ljava/lang/Object;Ljava/lang/Object;I)V"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.putByte(Ljava/lang/Object;I)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn put_byte_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.putByte(Ljava/lang/Object;I)V")
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.putByte(Ljava/lang/Object;Ljava/lang/Object;I)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn put_byte_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putByte(Ljava/lang/Object;Ljava/lang/Object;I)V"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.putByteVolatile(Ljava/lang/Object;I)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn put_byte_volatile_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.putByteVolatile(Ljava/lang/Object;I)V")
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.putByteVolatile(Ljava/lang/Object;Ljava/lang/Object;I)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn put_byte_volatile_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putByteVolatile(Ljava/lang/Object;Ljava/lang/Object;I)V"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.putChar(Ljava/lang/Object;I)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn put_char_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.putChar(Ljava/lang/Object;I)V")
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.putChar(Ljava/lang/Object;Ljava/lang/Object;I)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn put_char_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putChar(Ljava/lang/Object;Ljava/lang/Object;I)V"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.putCharVolatile(Ljava/lang/Object;I)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn put_char_volatile_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.putCharVolatile(Ljava/lang/Object;I)V")
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.putCharVolatile(Ljava/lang/Object;Ljava/lang/Object;I)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn put_char_volatile_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putCharVolatile(Ljava/lang/Object;Ljava/lang/Object;I)V"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.putDouble(Ljava/lang/Object;D)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn put_double_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.putDouble(Ljava/lang/Object;D)V")
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.putDouble(Ljava/lang/Object;Ljava/lang/Object;D)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn put_double_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putDouble(Ljava/lang/Object;Ljava/lang/Object;D)V"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.putDoubleVolatile(Ljava/lang/Object;D)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn put_double_volatile_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.putDoubleVolatile(Ljava/lang/Object;D)V")
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.putDoubleVolatile(Ljava/lang/Object;Ljava/lang/Object;D)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn put_double_volatile_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putDoubleVolatile(Ljava/lang/Object;Ljava/lang/Object;D)V"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.putFloat(Ljava/lang/Object;F)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn put_float_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.putFloat(Ljava/lang/Object;F)V")
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.putFloat(Ljava/lang/Object;Ljava/lang/Object;F)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn put_float_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putFloat(Ljava/lang/Object;Ljava/lang/Object;F)V"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.putFloatVolatile(Ljava/lang/Object;F)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn put_float_volatile_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.putFloatVolatile(Ljava/lang/Object;F)V")
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.putFloatVolatile(Ljava/lang/Object;Ljava/lang/Object;F)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn put_float_volatile_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putFloatVolatile(Ljava/lang/Object;Ljava/lang/Object;F)V"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.putInt(Ljava/lang/Object;I)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn put_int_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.putInt(Ljava/lang/Object;I)V")
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.putInt(Ljava/lang/Object;Ljava/lang/Object;I)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn put_int_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putInt(Ljava/lang/Object;Ljava/lang/Object;I)V"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.putIntVolatile(Ljava/lang/Object;I)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn put_int_volatile_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.putIntVolatile(Ljava/lang/Object;I)V")
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.putIntVolatile(Ljava/lang/Object;Ljava/lang/Object;I)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn put_int_volatile_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putIntVolatile(Ljava/lang/Object;Ljava/lang/Object;I)V"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.putLong(Ljava/lang/Object;J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn put_long_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.putLong(Ljava/lang/Object;J)V")
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.putLong(Ljava/lang/Object;Ljava/lang/Object;J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn put_long_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putLong(Ljava/lang/Object;Ljava/lang/Object;J)V"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.putLongVolatile(Ljava/lang/Object;J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn put_long_volatile_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.putLongVolatile(Ljava/lang/Object;J)V")
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.putLongVolatile(Ljava/lang/Object;Ljava/lang/Object;J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn put_long_volatile_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putLongVolatile(Ljava/lang/Object;Ljava/lang/Object;J)V"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.putObject(Ljava/lang/Object;Ljava/lang/Object;)V",
    Equal(JAVA_11)
)]
#[async_method]
pub async fn put_object_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putObject(Ljava/lang/Object;Ljava/lang/Object;)V"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.putObject(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V",
    Equal(JAVA_11)
)]
#[async_method]
pub async fn put_object_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putObject(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.putObjectVolatile(Ljava/lang/Object;Ljava/lang/Object;)V",
    Equal(JAVA_11)
)]
#[async_method]
pub async fn put_object_volatile_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putObjectVolatile(Ljava/lang/Object;Ljava/lang/Object;)V"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.putObjectVolatile(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V",
    Equal(JAVA_11)
)]
#[async_method]
pub async fn put_object_volatile_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putObjectVolatile(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.putReference(Ljava/lang/Object;Ljava/lang/Object;)V",
    Equal(JAVA_11)
)]
#[async_method]
pub async fn put_reference_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putReference(Ljava/lang/Object;Ljava/lang/Object;)V"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.putReference(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn put_reference_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putReference(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.putReferenceVolatile(Ljava/lang/Object;Ljava/lang/Object;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn put_reference_volatile_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putReferenceVolatile(Ljava/lang/Object;Ljava/lang/Object;)V"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.putReferenceVolatile(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn put_reference_volatile_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putReferenceVolatile(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.putShort(Ljava/lang/Object;I)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn put_short_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.putShort(Ljava/lang/Object;I)V")
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.putShort(Ljava/lang/Object;Ljava/lang/Object;I)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn put_short_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putShort(Ljava/lang/Object;Ljava/lang/Object;I)V"
    )
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.putShortVolatile(Ljava/lang/Object;I)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn put_short_volatile_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.DirectMethodHandle$Holder.putShortVolatile(Ljava/lang/Object;I)V")
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.putShortVolatile(Ljava/lang/Object;Ljava/lang/Object;I)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn put_short_volatile_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.DirectMethodHandle$Holder.putShortVolatile(Ljava/lang/Object;Ljava/lang/Object;I)V"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getBoolean(Ljava/lang/Object;)I"
    )]
    async fn test_get_boolean_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_boolean_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getBoolean(Ljava/lang/Object;Ljava/lang/Object;)I"
    )]
    async fn test_get_boolean_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_boolean_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getBooleanVolatile(Ljava/lang/Object;)I"
    )]
    async fn test_get_boolean_volatile_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_boolean_volatile_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getBooleanVolatile(Ljava/lang/Object;Ljava/lang/Object;)I"
    )]
    async fn test_get_boolean_volatile_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_boolean_volatile_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getByte(Ljava/lang/Object;)I"
    )]
    async fn test_get_byte_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_byte_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getByte(Ljava/lang/Object;Ljava/lang/Object;)I"
    )]
    async fn test_get_byte_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_byte_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getByteVolatile(Ljava/lang/Object;)I"
    )]
    async fn test_get_byte_volatile_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_byte_volatile_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getByteVolatile(Ljava/lang/Object;Ljava/lang/Object;)I"
    )]
    async fn test_get_byte_volatile_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_byte_volatile_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getChar(Ljava/lang/Object;)I"
    )]
    async fn test_get_char_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_char_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getChar(Ljava/lang/Object;Ljava/lang/Object;)I"
    )]
    async fn test_get_char_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_char_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getCharVolatile(Ljava/lang/Object;)I"
    )]
    async fn test_get_char_volatile_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_char_volatile_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getCharVolatile(Ljava/lang/Object;Ljava/lang/Object;)I"
    )]
    async fn test_get_char_volatile_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_char_volatile_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getDouble(Ljava/lang/Object;)D"
    )]
    async fn test_get_double_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_double_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getDouble(Ljava/lang/Object;Ljava/lang/Object;)D"
    )]
    async fn test_get_double_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_double_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getDoubleVolatile(Ljava/lang/Object;)D"
    )]
    async fn test_get_double_volatile_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_double_volatile_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getDoubleVolatile(Ljava/lang/Object;Ljava/lang/Object;)D"
    )]
    async fn test_get_double_volatile_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_double_volatile_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getFloat(Ljava/lang/Object;)F"
    )]
    async fn test_get_float_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_float_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getFloat(Ljava/lang/Object;Ljava/lang/Object;)F"
    )]
    async fn test_get_float_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_float_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getFloatVolatile(Ljava/lang/Object;)F"
    )]
    async fn test_get_float_volatile_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_float_volatile_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getFloatVolatile(Ljava/lang/Object;Ljava/lang/Object;)F"
    )]
    async fn test_get_float_volatile_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_float_volatile_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getInt(Ljava/lang/Object;)I"
    )]
    async fn test_get_int_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_int_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getInt(Ljava/lang/Object;Ljava/lang/Object;)I"
    )]
    async fn test_get_int_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_int_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getIntVolatile(Ljava/lang/Object;)I"
    )]
    async fn test_get_int_volatile_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_int_volatile_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getIntVolatile(Ljava/lang/Object;Ljava/lang/Object;)I"
    )]
    async fn test_get_int_volatile_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_int_volatile_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getLong(Ljava/lang/Object;)J"
    )]
    async fn test_get_long_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_long_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getLong(Ljava/lang/Object;Ljava/lang/Object;)J"
    )]
    async fn test_get_long_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_long_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getLongVolatile(Ljava/lang/Object;)J"
    )]
    async fn test_get_long_volatile_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_long_volatile_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getLongVolatile(Ljava/lang/Object;Ljava/lang/Object;)J"
    )]
    async fn test_get_long_volatile_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_long_volatile_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getObject(Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_object_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_object_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getObject(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_object_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_object_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getObjectVolatile(Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_object_volatile_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_object_volatile_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getObjectVolatile(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_object_volatile_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_object_volatile_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getReference(Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_reference_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_reference_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_get_reference_1_requires_method_handle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_reference_1(thread, Parameters::default()).await;
        // With no arguments, dispatch_holder_method should return an error
        assert!(matches!(
            result,
            Err(ristretto_types::Error::InternalError(_))
        ));
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getReference(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_reference_2() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_reference_2(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getReferenceVolatile(Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_reference_volatile_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_reference_volatile_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getReferenceVolatile(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_get_reference_volatile_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_reference_volatile_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getShort(Ljava/lang/Object;)I"
    )]
    async fn test_get_short_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_short_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getShort(Ljava/lang/Object;Ljava/lang/Object;)I"
    )]
    async fn test_get_short_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_short_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getShortVolatile(Ljava/lang/Object;)I"
    )]
    async fn test_get_short_volatile_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_short_volatile_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.getShortVolatile(Ljava/lang/Object;Ljava/lang/Object;)I"
    )]
    async fn test_get_short_volatile_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_short_volatile_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_invoke_interface_requires_method_handle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = invoke_interface(thread, Parameters::default()).await;
        // With no arguments, dispatch_holder_method should return an error
        assert!(matches!(
            result,
            Err(ristretto_types::Error::InternalError(_))
        ));
    }

    #[tokio::test]
    async fn test_invoke_special_requires_method_handle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = invoke_special(thread, Parameters::default()).await;
        // With no arguments, dispatch_holder_method should return an error
        assert!(matches!(
            result,
            Err(ristretto_types::Error::InternalError(_))
        ));
    }

    #[tokio::test]
    async fn test_invoke_static_requires_method_handle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = invoke_static(thread, Parameters::default()).await;
        // With no arguments, dispatch_holder_method should return an error
        assert!(matches!(
            result,
            Err(ristretto_types::Error::InternalError(_))
        ));
    }

    #[tokio::test]
    async fn test_invoke_virtual_requires_method_handle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = invoke_virtual(thread, Parameters::default()).await;
        // With no arguments, dispatch_holder_method should return an error
        assert!(matches!(
            result,
            Err(ristretto_types::Error::InternalError(_))
        ));
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putBoolean(Ljava/lang/Object;I)V"
    )]
    async fn test_put_boolean_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_boolean_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putBoolean(Ljava/lang/Object;Ljava/lang/Object;I)V"
    )]
    async fn test_put_boolean_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_boolean_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putBooleanVolatile(Ljava/lang/Object;I)V"
    )]
    async fn test_put_boolean_volatile_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_boolean_volatile_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putBooleanVolatile(Ljava/lang/Object;Ljava/lang/Object;I)V"
    )]
    async fn test_put_boolean_volatile_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_boolean_volatile_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putByte(Ljava/lang/Object;I)V"
    )]
    async fn test_put_byte_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_byte_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putByte(Ljava/lang/Object;Ljava/lang/Object;I)V"
    )]
    async fn test_put_byte_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_byte_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putByteVolatile(Ljava/lang/Object;I)V"
    )]
    async fn test_put_byte_volatile_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_byte_volatile_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putByteVolatile(Ljava/lang/Object;Ljava/lang/Object;I)V"
    )]
    async fn test_put_byte_volatile_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_byte_volatile_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putChar(Ljava/lang/Object;I)V"
    )]
    async fn test_put_char_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_char_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putChar(Ljava/lang/Object;Ljava/lang/Object;I)V"
    )]
    async fn test_put_char_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_char_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putCharVolatile(Ljava/lang/Object;I)V"
    )]
    async fn test_put_char_volatile_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_char_volatile_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putCharVolatile(Ljava/lang/Object;Ljava/lang/Object;I)V"
    )]
    async fn test_put_char_volatile_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_char_volatile_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putDouble(Ljava/lang/Object;D)V"
    )]
    async fn test_put_double_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_double_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putDouble(Ljava/lang/Object;Ljava/lang/Object;D)V"
    )]
    async fn test_put_double_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_double_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putDoubleVolatile(Ljava/lang/Object;D)V"
    )]
    async fn test_put_double_volatile_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_double_volatile_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putDoubleVolatile(Ljava/lang/Object;Ljava/lang/Object;D)V"
    )]
    async fn test_put_double_volatile_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_double_volatile_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putFloat(Ljava/lang/Object;F)V"
    )]
    async fn test_put_float_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_float_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putFloat(Ljava/lang/Object;Ljava/lang/Object;F)V"
    )]
    async fn test_put_float_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_float_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putFloatVolatile(Ljava/lang/Object;F)V"
    )]
    async fn test_put_float_volatile_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_float_volatile_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putFloatVolatile(Ljava/lang/Object;Ljava/lang/Object;F)V"
    )]
    async fn test_put_float_volatile_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_float_volatile_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putInt(Ljava/lang/Object;I)V"
    )]
    async fn test_put_int_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_int_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putInt(Ljava/lang/Object;Ljava/lang/Object;I)V"
    )]
    async fn test_put_int_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_int_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putIntVolatile(Ljava/lang/Object;I)V"
    )]
    async fn test_put_int_volatile_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_int_volatile_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putIntVolatile(Ljava/lang/Object;Ljava/lang/Object;I)V"
    )]
    async fn test_put_int_volatile_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_int_volatile_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putLong(Ljava/lang/Object;J)V"
    )]
    async fn test_put_long_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_long_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putLong(Ljava/lang/Object;Ljava/lang/Object;J)V"
    )]
    async fn test_put_long_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_long_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putLongVolatile(Ljava/lang/Object;J)V"
    )]
    async fn test_put_long_volatile_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_long_volatile_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putLongVolatile(Ljava/lang/Object;Ljava/lang/Object;J)V"
    )]
    async fn test_put_long_volatile_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_long_volatile_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putObject(Ljava/lang/Object;Ljava/lang/Object;)V"
    )]
    async fn test_put_object_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_object_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putObject(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V"
    )]
    async fn test_put_object_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_object_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putObjectVolatile(Ljava/lang/Object;Ljava/lang/Object;)V"
    )]
    async fn test_put_object_volatile_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_object_volatile_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putObjectVolatile(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V"
    )]
    async fn test_put_object_volatile_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_object_volatile_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putReference(Ljava/lang/Object;Ljava/lang/Object;)V"
    )]
    async fn test_put_reference_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_reference_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putReference(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V"
    )]
    async fn test_put_reference_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_reference_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putReferenceVolatile(Ljava/lang/Object;Ljava/lang/Object;)V"
    )]
    async fn test_put_reference_volatile_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_reference_volatile_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putReferenceVolatile(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V"
    )]
    async fn test_put_reference_volatile_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_reference_volatile_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putShort(Ljava/lang/Object;I)V"
    )]
    async fn test_put_short_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_short_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putShort(Ljava/lang/Object;Ljava/lang/Object;I)V"
    )]
    async fn test_put_short_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_short_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putShortVolatile(Ljava/lang/Object;I)V"
    )]
    async fn test_put_short_volatile_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_short_volatile_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.DirectMethodHandle$Holder.putShortVolatile(Ljava/lang/Object;Ljava/lang/Object;I)V"
    )]
    async fn test_put_short_volatile_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_short_volatile_1(thread, Parameters::default()).await;
    }
}
