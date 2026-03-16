use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;
use tracing::debug;

/// Access mode categories for `VarHandle` operations.
#[derive(Debug, Clone, Copy)]
#[repr(i32)]
enum AccessMode {
    Get = 0,
    Set = 1,
    GetVolatile = 2,
    SetVolatile = 3,
    GetAcquire = 4,
    SetRelease = 5,
    GetOpaque = 6,
    SetOpaque = 7,
    CompareAndSet = 8,
    CompareAndExchange = 9,
    CompareAndExchangeAcquire = 10,
    CompareAndExchangeRelease = 11,
    WeakCompareAndSetPlain = 12,
    WeakCompareAndSet = 13,
    WeakCompareAndSetAcquire = 14,
    WeakCompareAndSetRelease = 15,
    GetAndSet = 16,
    GetAndSetAcquire = 17,
    GetAndSetRelease = 18,
    GetAndAdd = 19,
    GetAndAddAcquire = 20,
    GetAndAddRelease = 21,
    GetAndBitwiseOr = 22,
    GetAndBitwiseOrRelease = 23,
    GetAndBitwiseOrAcquire = 24,
    GetAndBitwiseAnd = 25,
    GetAndBitwiseAndRelease = 26,
    GetAndBitwiseAndAcquire = 27,
    GetAndBitwiseXor = 28,
    GetAndBitwiseXorRelease = 29,
    GetAndBitwiseXorAcquire = 30,
}

/// The storage type of a `VarHandle`, determined structurally from its fields.
#[derive(Debug)]
enum VarHandleStorage {
    FieldInstance { field_offset: usize },
    FieldStatic { field_offset: usize },
    Array,
}

/// Classify a `VarHandle` by inspecting its fields structurally to determine how it performs
/// access.
fn classify_var_handle(var_handle: &Value) -> Option<VarHandleStorage> {
    let vh_ref = var_handle.as_object_ref().ok()?;

    // Array VarHandles have an `ashift` field
    if vh_ref.value("ashift").is_ok() {
        return Some(VarHandleStorage::Array);
    }

    // Field-based VarHandles have a `fieldOffset` field
    let field_offset = vh_ref.value("fieldOffset").ok()?.as_i64().ok()?;

    // Static field VarHandles have a `base` field (the Class mirror)
    if vh_ref.value("base").is_ok() {
        // Strip STATIC_FIELD_OFFSET_MASK (bit 62) set by Unsafe.staticFieldOffset0
        let offset = usize::try_from(field_offset & !(1i64 << 62)).ok()?;
        Some(VarHandleStorage::FieldStatic {
            field_offset: offset,
        })
    } else {
        let offset = usize::try_from(field_offset).ok()?;
        Some(VarHandleStorage::FieldInstance {
            field_offset: offset,
        })
    }
}

/// Convert a `Value` to an i64 for numeric operations.
fn value_to_i64(value: &Value) -> Result<i64> {
    match value {
        Value::Int(i) => Ok(i64::from(*i)),
        Value::Long(l) => Ok(*l),
        _ => Err(InternalError(format!("Cannot convert {value:?} to i64"))),
    }
}

/// Apply an i64 arithmetic result, producing a `Value` matching the type of `old`.
fn apply_numeric_result(old: &Value, result: i64) -> Result<Value> {
    match old {
        Value::Int(_) =>
        {
            #[expect(clippy::cast_possible_truncation)]
            Ok(Value::Int(result as i32))
        }
        Value::Long(_) => Ok(Value::Long(result)),
        _ => Err(InternalError(format!(
            "Cannot apply numeric result to {old:?}"
        ))),
    }
}

/// Read a field value from a target object `Value` at the given offset.
fn read_field_by_offset(target: &Value, offset: usize) -> Result<Value> {
    let obj = target.as_object_ref()?;
    let class = obj.class();
    let field_name = class.field_name(offset)?;
    Ok(obj.value(&field_name)?)
}

/// Write a field value to a target object `Value` at the given offset.
fn write_field_by_offset(target: &Value, offset: usize, value: Value) -> Result<()> {
    let mut object = target.as_object_mut()?;
    let class = object.class().clone();
    let field_name = class.field_name(offset)?;
    Ok(object.set_value(&field_name, value)?)
}

/// Directly implements `VarHandle` access modes by inspecting the `VarHandle`'s fields
/// structurally and performing field access without going through the JDK's
/// `MethodHandle`/`LambdaForm` chain.
#[async_method]
async fn invoke_var_handle_access_mode<T: Thread + 'static>(
    thread: &Arc<T>,
    parameters: Parameters,
    access_mode: AccessMode,
) -> Result<Option<Value>> {
    let all_params = parameters.into_vec();
    if all_params.is_empty() {
        return Err(InternalError(
            "VarHandle access mode requires at least the VarHandle".to_string(),
        ));
    }

    let var_handle = &all_params[0];
    let remaining_args = &all_params[1..];
    let storage = classify_var_handle(var_handle);

    debug!(
        "VarHandle access: storage={:?}, mode={:?}, args={}",
        storage,
        access_mode,
        remaining_args.len()
    );

    match storage {
        Some(VarHandleStorage::FieldInstance { field_offset }) => {
            dispatch_field_instance(access_mode, field_offset, remaining_args)
        }
        Some(VarHandleStorage::FieldStatic { field_offset }) => {
            let base_object = {
                let vh_ref = var_handle.as_object_ref()?;
                vh_ref.value("base")?
            };
            dispatch_field_static(
                thread,
                access_mode,
                field_offset,
                &base_object,
                remaining_args,
            )
            .await
        }
        Some(VarHandleStorage::Array) => dispatch_array(access_mode, remaining_args),
        None => {
            debug!("VarHandle: unrecognized structure, falling back to toMethodHandle",);
            invoke_via_method_handle(thread, all_params, access_mode).await
        }
    }
}

/// Dispatch a `VarHandle` access mode on an instance field.
#[expect(clippy::too_many_lines)]
fn dispatch_field_instance(
    access_mode: AccessMode,
    offset: usize,
    args: &[Value],
) -> Result<Option<Value>> {
    match access_mode {
        AccessMode::Get
        | AccessMode::GetVolatile
        | AccessMode::GetAcquire
        | AccessMode::GetOpaque => {
            let target = args
                .first()
                .ok_or_else(|| InternalError("VarHandle.get: missing target object".into()))?;
            let value = read_field_by_offset(target, offset)?;
            Ok(Some(value))
        }
        AccessMode::Set
        | AccessMode::SetVolatile
        | AccessMode::SetRelease
        | AccessMode::SetOpaque => {
            let target = args
                .first()
                .ok_or_else(|| InternalError("VarHandle.set: missing target object".into()))?;
            let new_value = args
                .get(1)
                .ok_or_else(|| InternalError("VarHandle.set: missing value".into()))?;
            write_field_by_offset(target, offset, new_value.clone())?;
            Ok(None)
        }
        AccessMode::CompareAndSet
        | AccessMode::WeakCompareAndSet
        | AccessMode::WeakCompareAndSetPlain
        | AccessMode::WeakCompareAndSetAcquire
        | AccessMode::WeakCompareAndSetRelease => {
            let target = args
                .first()
                .ok_or_else(|| InternalError("VarHandle.CAS: missing target".into()))?;
            let expected = args
                .get(1)
                .ok_or_else(|| InternalError("VarHandle.CAS: missing expected".into()))?;
            let new_value = args
                .get(2)
                .ok_or_else(|| InternalError("VarHandle.CAS: missing new value".into()))?;
            // Hold a single write lock for the entire read-check-write to ensure atomicity
            let mut object = target.as_object_mut()?;
            let class = object.class().clone();
            let field_name = class.field_name(offset)?;
            let current = object.value(&field_name)?;
            if values_equal(&current, expected) {
                object.set_value(&field_name, new_value.clone())?;
                Ok(Some(Value::Int(1)))
            } else {
                Ok(Some(Value::Int(0)))
            }
        }
        AccessMode::CompareAndExchange
        | AccessMode::CompareAndExchangeAcquire
        | AccessMode::CompareAndExchangeRelease => {
            let target = args
                .first()
                .ok_or_else(|| InternalError("VarHandle.CAE: missing target".into()))?;
            let expected = args
                .get(1)
                .ok_or_else(|| InternalError("VarHandle.CAE: missing expected".into()))?;
            let new_val = args
                .get(2)
                .ok_or_else(|| InternalError("VarHandle.CAE: missing new value".into()))?;
            let mut object = target.as_object_mut()?;
            let class = object.class().clone();
            let field_name = class.field_name(offset)?;
            let current = object.value(&field_name)?;
            if values_equal(&current, expected) {
                object.set_value(&field_name, new_val.clone())?;
            }
            Ok(Some(current))
        }
        AccessMode::GetAndSet | AccessMode::GetAndSetAcquire | AccessMode::GetAndSetRelease => {
            let target = args
                .first()
                .ok_or_else(|| InternalError("VarHandle.getAndSet: missing target".into()))?;
            let new_value = args
                .get(1)
                .ok_or_else(|| InternalError("VarHandle.getAndSet: missing new value".into()))?;
            let mut object = target.as_object_mut()?;
            let class = object.class().clone();
            let field_name = class.field_name(offset)?;
            let old = object.value(&field_name)?;
            object.set_value(&field_name, new_value.clone())?;
            Ok(Some(old))
        }
        AccessMode::GetAndAdd | AccessMode::GetAndAddAcquire | AccessMode::GetAndAddRelease => {
            let target = args
                .first()
                .ok_or_else(|| InternalError("VarHandle.getAndAdd: missing target".into()))?;
            let delta = args
                .get(1)
                .ok_or_else(|| InternalError("VarHandle.getAndAdd: missing delta".into()))?;
            let mut object = target.as_object_mut()?;
            let class = object.class().clone();
            let field_name = class.field_name(offset)?;
            let old = object.value(&field_name)?;
            let old_i64 = value_to_i64(&old)?;
            let delta_i64 = value_to_i64(delta)?;
            let new_value = apply_numeric_result(&old, old_i64.wrapping_add(delta_i64))?;
            object.set_value(&field_name, new_value)?;
            Ok(Some(old))
        }
        AccessMode::GetAndBitwiseOr
        | AccessMode::GetAndBitwiseOrAcquire
        | AccessMode::GetAndBitwiseOrRelease => {
            let target = args
                .first()
                .ok_or_else(|| InternalError("VarHandle.getAndBitwiseOr: missing target".into()))?;
            let mask = args
                .get(1)
                .ok_or_else(|| InternalError("VarHandle.getAndBitwiseOr: missing mask".into()))?;
            let mut object = target.as_object_mut()?;
            let class = object.class().clone();
            let field_name = class.field_name(offset)?;
            let old = object.value(&field_name)?;
            let old_i64 = value_to_i64(&old)?;
            let mask_i64 = value_to_i64(mask)?;
            let new_value = apply_numeric_result(&old, old_i64 | mask_i64)?;
            object.set_value(&field_name, new_value)?;
            Ok(Some(old))
        }
        AccessMode::GetAndBitwiseAnd
        | AccessMode::GetAndBitwiseAndAcquire
        | AccessMode::GetAndBitwiseAndRelease => {
            let target = args.first().ok_or_else(|| {
                InternalError("VarHandle.getAndBitwiseAnd: missing target".into())
            })?;
            let mask = args
                .get(1)
                .ok_or_else(|| InternalError("VarHandle.getAndBitwiseAnd: missing mask".into()))?;
            let mut object = target.as_object_mut()?;
            let class = object.class().clone();
            let field_name = class.field_name(offset)?;
            let old = object.value(&field_name)?;
            let old_i64 = value_to_i64(&old)?;
            let mask_i64 = value_to_i64(mask)?;
            let new_value = apply_numeric_result(&old, old_i64 & mask_i64)?;
            object.set_value(&field_name, new_value)?;
            Ok(Some(old))
        }
        AccessMode::GetAndBitwiseXor
        | AccessMode::GetAndBitwiseXorAcquire
        | AccessMode::GetAndBitwiseXorRelease => {
            let target = args.first().ok_or_else(|| {
                InternalError("VarHandle.getAndBitwiseXor: missing target".into())
            })?;
            let mask = args
                .get(1)
                .ok_or_else(|| InternalError("VarHandle.getAndBitwiseXor: missing mask".into()))?;
            let mut object = target.as_object_mut()?;
            let class = object.class().clone();
            let field_name = class.field_name(offset)?;
            let old = object.value(&field_name)?;
            let old_i64 = value_to_i64(&old)?;
            let mask_i64 = value_to_i64(mask)?;
            let new_value = apply_numeric_result(&old, old_i64 ^ mask_i64)?;
            object.set_value(&field_name, new_value)?;
            Ok(Some(old))
        }
    }
}

/// Dispatch a `VarHandle` access mode on a static field.
#[async_method]
#[expect(clippy::too_many_lines)]
async fn dispatch_field_static<T: Thread + 'static>(
    thread: &Arc<T>,
    access_mode: AccessMode,
    offset: usize,
    base_object: &Value,
    args: &[Value],
) -> Result<Option<Value>> {
    // base_obj is the Class mirror; get the class name from it
    let class_name = {
        let base_ref = base_object.as_object_ref()?;
        base_ref.value("name")?.as_string()?
    };
    let class = thread.class(&class_name).await?;
    let field_name = class.field_name(offset)?;

    match access_mode {
        AccessMode::Get
        | AccessMode::GetVolatile
        | AccessMode::GetAcquire
        | AccessMode::GetOpaque => {
            let value = class.static_value(&field_name)?;
            Ok(Some(value))
        }
        AccessMode::Set
        | AccessMode::SetVolatile
        | AccessMode::SetRelease
        | AccessMode::SetOpaque => {
            let new_value = args
                .first()
                .ok_or_else(|| InternalError("VarHandle static set: missing value".into()))?;
            class.set_static_value(&field_name, new_value.clone())?;
            Ok(None)
        }
        AccessMode::CompareAndSet
        | AccessMode::WeakCompareAndSet
        | AccessMode::WeakCompareAndSetPlain
        | AccessMode::WeakCompareAndSetAcquire
        | AccessMode::WeakCompareAndSetRelease => {
            let expected = args
                .first()
                .ok_or_else(|| InternalError("VarHandle static CAS: missing expected".into()))?;
            let new_value = args
                .get(1)
                .ok_or_else(|| InternalError("VarHandle static CAS: missing new value".into()))?;
            let current = class.static_value(&field_name)?;
            if values_equal(&current, expected) {
                class.set_static_value(&field_name, new_value.clone())?;
                Ok(Some(Value::Int(1)))
            } else {
                Ok(Some(Value::Int(0)))
            }
        }
        AccessMode::CompareAndExchange
        | AccessMode::CompareAndExchangeAcquire
        | AccessMode::CompareAndExchangeRelease => {
            let expected = args
                .first()
                .ok_or_else(|| InternalError("VarHandle static CAE: missing expected".into()))?;
            let new_value = args
                .get(1)
                .ok_or_else(|| InternalError("VarHandle static CAE: missing new value".into()))?;
            let current = class.static_value(&field_name)?;
            if values_equal(&current, expected) {
                class.set_static_value(&field_name, new_value.clone())?;
            }
            Ok(Some(current))
        }
        AccessMode::GetAndSet | AccessMode::GetAndSetAcquire | AccessMode::GetAndSetRelease => {
            let new_value = args.first().ok_or_else(|| {
                InternalError("VarHandle static getAndSet: missing new value".into())
            })?;
            let old = class.static_value(&field_name)?;
            class.set_static_value(&field_name, new_value.clone())?;
            Ok(Some(old))
        }
        AccessMode::GetAndAdd | AccessMode::GetAndAddAcquire | AccessMode::GetAndAddRelease => {
            let delta = args
                .first()
                .ok_or_else(|| InternalError("VarHandle static getAndAdd: missing delta".into()))?;
            let old = class.static_value(&field_name)?;
            let old_i64 = value_to_i64(&old)?;
            let delta_i64 = value_to_i64(delta)?;
            let new_value = apply_numeric_result(&old, old_i64.wrapping_add(delta_i64))?;
            class.set_static_value(&field_name, new_value)?;
            Ok(Some(old))
        }
        AccessMode::GetAndBitwiseOr
        | AccessMode::GetAndBitwiseOrAcquire
        | AccessMode::GetAndBitwiseOrRelease => {
            let mask = args.first().ok_or_else(|| {
                InternalError("VarHandle static getAndBitwiseOr: missing mask".into())
            })?;
            let old = class.static_value(&field_name)?;
            let old_i64 = value_to_i64(&old)?;
            let mask_i64 = value_to_i64(mask)?;
            let new_value = apply_numeric_result(&old, old_i64 | mask_i64)?;
            class.set_static_value(&field_name, new_value)?;
            Ok(Some(old))
        }
        AccessMode::GetAndBitwiseAnd
        | AccessMode::GetAndBitwiseAndAcquire
        | AccessMode::GetAndBitwiseAndRelease => {
            let mask = args.first().ok_or_else(|| {
                InternalError("VarHandle static getAndBitwiseAnd: missing mask".into())
            })?;
            let old = class.static_value(&field_name)?;
            let old_i64 = value_to_i64(&old)?;
            let mask_i64 = value_to_i64(mask)?;
            let new_value = apply_numeric_result(&old, old_i64 & mask_i64)?;
            class.set_static_value(&field_name, new_value)?;
            Ok(Some(old))
        }
        AccessMode::GetAndBitwiseXor
        | AccessMode::GetAndBitwiseXorAcquire
        | AccessMode::GetAndBitwiseXorRelease => {
            let mask = args.first().ok_or_else(|| {
                InternalError("VarHandle static getAndBitwiseXor: missing mask".into())
            })?;
            let old = class.static_value(&field_name)?;
            let old_i64 = value_to_i64(&old)?;
            let mask_i64 = value_to_i64(mask)?;
            let new_value = apply_numeric_result(&old, old_i64 ^ mask_i64)?;
            class.set_static_value(&field_name, new_value)?;
            Ok(Some(old))
        }
    }
}

/// Dispatch a `VarHandle` access mode on an array element.
#[expect(clippy::too_many_lines)]
fn dispatch_array(access_mode: AccessMode, args: &[Value]) -> Result<Option<Value>> {
    let target = args
        .first()
        .ok_or_else(|| InternalError("VarHandle array: missing array".into()))?;
    let index = args
        .get(1)
        .ok_or_else(|| InternalError("VarHandle array: missing index".into()))?;
    let idx = usize::try_from(index.as_i32()?).map_err(|e| InternalError(e.to_string()))?;

    match access_mode {
        AccessMode::Get
        | AccessMode::GetVolatile
        | AccessMode::GetAcquire
        | AccessMode::GetOpaque => read_array_element(target, idx),
        AccessMode::Set
        | AccessMode::SetVolatile
        | AccessMode::SetRelease
        | AccessMode::SetOpaque => {
            let new_value = args
                .get(2)
                .ok_or_else(|| InternalError("VarHandle array set: missing value".into()))?;
            write_array_element(target, idx, new_value)?;
            Ok(None)
        }
        AccessMode::CompareAndSet
        | AccessMode::WeakCompareAndSet
        | AccessMode::WeakCompareAndSetPlain
        | AccessMode::WeakCompareAndSetAcquire
        | AccessMode::WeakCompareAndSetRelease => {
            let expected = args
                .get(2)
                .ok_or_else(|| InternalError("VarHandle array CAS: missing expected".into()))?;
            let new_value = args
                .get(3)
                .ok_or_else(|| InternalError("VarHandle array CAS: missing new value".into()))?;
            // Hold a single write lock for the entire read-check-write to ensure atomicity
            let (current, success) = cas_array_element(target, idx, expected, new_value)?;
            let _ = current;
            Ok(Some(Value::from(success)))
        }
        AccessMode::CompareAndExchange
        | AccessMode::CompareAndExchangeAcquire
        | AccessMode::CompareAndExchangeRelease => {
            let expected = args
                .get(2)
                .ok_or_else(|| InternalError("VarHandle array CAE: missing expected".into()))?;
            let new_value = args
                .get(3)
                .ok_or_else(|| InternalError("VarHandle array CAE: missing new value".into()))?;
            let (current, _success) = cas_array_element(target, idx, expected, new_value)?;
            Ok(Some(current))
        }
        AccessMode::GetAndSet | AccessMode::GetAndSetAcquire | AccessMode::GetAndSetRelease => {
            let new_value = args.get(2).ok_or_else(|| {
                InternalError("VarHandle array getAndSet: missing new value".into())
            })?;
            let old = atomic_read_write_array(target, idx, |_old| Ok(new_value.clone()))?;
            Ok(Some(old))
        }
        AccessMode::GetAndAdd | AccessMode::GetAndAddAcquire | AccessMode::GetAndAddRelease => {
            let delta = args
                .get(2)
                .ok_or_else(|| InternalError("VarHandle array getAndAdd: missing delta".into()))?;
            let delta_i64 = value_to_i64(delta)?;
            let old = atomic_read_write_array(target, idx, |old| {
                let old_i64 = value_to_i64(old)?;
                apply_numeric_result(old, old_i64.wrapping_add(delta_i64))
            })?;
            Ok(Some(old))
        }
        AccessMode::GetAndBitwiseOr
        | AccessMode::GetAndBitwiseOrAcquire
        | AccessMode::GetAndBitwiseOrRelease => {
            let mask = args.get(2).ok_or_else(|| {
                InternalError("VarHandle array getAndBitwiseOr: missing mask".into())
            })?;
            let mask_i64 = value_to_i64(mask)?;
            let old = atomic_read_write_array(target, idx, |old| {
                apply_numeric_result(old, value_to_i64(old)? | mask_i64)
            })?;
            Ok(Some(old))
        }
        AccessMode::GetAndBitwiseAnd
        | AccessMode::GetAndBitwiseAndAcquire
        | AccessMode::GetAndBitwiseAndRelease => {
            let mask = args.get(2).ok_or_else(|| {
                InternalError("VarHandle array getAndBitwiseAnd: missing mask".into())
            })?;
            let mask_i64 = value_to_i64(mask)?;
            let old = atomic_read_write_array(target, idx, |old| {
                apply_numeric_result(old, value_to_i64(old)? & mask_i64)
            })?;
            Ok(Some(old))
        }
        AccessMode::GetAndBitwiseXor
        | AccessMode::GetAndBitwiseXorAcquire
        | AccessMode::GetAndBitwiseXorRelease => {
            let mask = args.get(2).ok_or_else(|| {
                InternalError("VarHandle array getAndBitwiseXor: missing mask".into())
            })?;
            let mask_i64 = value_to_i64(mask)?;
            let old = atomic_read_write_array(target, idx, |old| {
                apply_numeric_result(old, value_to_i64(old)? ^ mask_i64)
            })?;
            Ok(Some(old))
        }
    }
}

/// Read an element from an array at the given index.
fn read_array_element(target: &Value, index: usize) -> Result<Option<Value>> {
    let arr_ref = target.as_reference()?;
    match &*arr_ref {
        Reference::IntArray(array) => Ok(array.get(index).map(|v| Value::Int(*v))),
        Reference::LongArray(array) => Ok(array.get(index).map(|v| Value::Long(*v))),
        Reference::FloatArray(array) => Ok(array.get(index).map(|v| Value::Float(*v))),
        Reference::DoubleArray(array) => Ok(array.get(index).map(|v| Value::Double(*v))),
        Reference::ByteArray(array) => Ok(array.get(index).map(|v| Value::Int(i32::from(*v)))),
        Reference::CharArray(array) => Ok(array.get(index).map(|v| Value::Int(i32::from(*v)))),
        Reference::ShortArray(array) => Ok(array.get(index).map(|v| Value::Int(i32::from(*v)))),
        Reference::Array(object_array) => Ok(object_array.elements.get(index).cloned()),
        _ => Err(InternalError(
            "VarHandle array: unsupported array type".into(),
        )),
    }
}

/// Write a value to an array at the given index.
fn write_array_element(target: &Value, index: usize, new_value: &Value) -> Result<()> {
    let mut arr_ref = target.as_reference_mut()?;
    match &mut *arr_ref {
        Reference::IntArray(array) => {
            let slot = array
                .get_mut(index)
                .ok_or_else(|| InternalError(format!("array index {index} out of bounds")))?;
            *slot = new_value.as_i32()?;
        }
        Reference::LongArray(array) => {
            let slot = array
                .get_mut(index)
                .ok_or_else(|| InternalError(format!("array index {index} out of bounds")))?;
            *slot = new_value.as_i64()?;
        }
        Reference::FloatArray(array) => {
            let slot = array
                .get_mut(index)
                .ok_or_else(|| InternalError(format!("array index {index} out of bounds")))?;
            *slot = new_value.as_f32()?;
        }
        Reference::DoubleArray(array) => {
            let slot = array
                .get_mut(index)
                .ok_or_else(|| InternalError(format!("array index {index} out of bounds")))?;
            *slot = new_value.as_f64()?;
        }
        Reference::ByteArray(array) => {
            let slot = array
                .get_mut(index)
                .ok_or_else(|| InternalError(format!("array index {index} out of bounds")))?;
            #[expect(clippy::cast_possible_truncation)]
            {
                *slot = new_value.as_i32()? as i8;
            }
        }
        Reference::CharArray(array) => {
            let slot = array
                .get_mut(index)
                .ok_or_else(|| InternalError(format!("array index {index} out of bounds")))?;
            #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            {
                *slot = new_value.as_i32()? as u16;
            }
        }
        Reference::ShortArray(array) => {
            let slot = array
                .get_mut(index)
                .ok_or_else(|| InternalError(format!("array index {index} out of bounds")))?;
            #[expect(clippy::cast_possible_truncation)]
            {
                *slot = new_value.as_i32()? as i16;
            }
        }
        Reference::Array(object_array) => {
            let slot = object_array
                .elements
                .get_mut(index)
                .ok_or_else(|| InternalError(format!("array index {index} out of bounds")))?;
            *slot = new_value.clone();
        }
        _ => {
            return Err(InternalError(
                "VarHandle array: unsupported array type for write".into(),
            ));
        }
    }
    Ok(())
}

/// Read the current value from a mutable reference at the given array index.
fn read_from_ref(reference: &Reference, index: usize) -> Result<Option<Value>> {
    match reference {
        Reference::IntArray(array) => Ok(array.get(index).map(|v| Value::Int(*v))),
        Reference::LongArray(array) => Ok(array.get(index).map(|v| Value::Long(*v))),
        Reference::FloatArray(array) => Ok(array.get(index).map(|v| Value::Float(*v))),
        Reference::DoubleArray(array) => Ok(array.get(index).map(|v| Value::Double(*v))),
        Reference::ByteArray(array) => Ok(array.get(index).map(|v| Value::Int(i32::from(*v)))),
        Reference::CharArray(array) => Ok(array.get(index).map(|v| Value::Int(i32::from(*v)))),
        Reference::ShortArray(array) => Ok(array.get(index).map(|v| Value::Int(i32::from(*v)))),
        Reference::Array(object_array) => Ok(object_array.elements.get(index).cloned()),
        _ => Err(InternalError(
            "VarHandle array: unsupported array type".into(),
        )),
    }
}

/// Write a value to a mutable reference at the given array index.
#[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn write_to_ref(reference: &mut Reference, index: usize, new_value: &Value) -> Result<()> {
    match reference {
        Reference::IntArray(array) => {
            let slot = array
                .get_mut(index)
                .ok_or_else(|| InternalError(format!("array index {index} out of bounds")))?;
            *slot = new_value.as_i32()?;
        }
        Reference::LongArray(array) => {
            let slot = array
                .get_mut(index)
                .ok_or_else(|| InternalError(format!("array index {index} out of bounds")))?;
            *slot = new_value.as_i64()?;
        }
        Reference::FloatArray(array) => {
            let slot = array
                .get_mut(index)
                .ok_or_else(|| InternalError(format!("array index {index} out of bounds")))?;
            *slot = new_value.as_f32()?;
        }
        Reference::DoubleArray(array) => {
            let slot = array
                .get_mut(index)
                .ok_or_else(|| InternalError(format!("array index {index} out of bounds")))?;
            *slot = new_value.as_f64()?;
        }
        Reference::ByteArray(array) => {
            let slot = array
                .get_mut(index)
                .ok_or_else(|| InternalError(format!("array index {index} out of bounds")))?;
            *slot = new_value.as_i32()? as i8;
        }
        Reference::CharArray(array) => {
            let slot = array
                .get_mut(index)
                .ok_or_else(|| InternalError(format!("array index {index} out of bounds")))?;
            *slot = new_value.as_i32()? as u16;
        }
        Reference::ShortArray(array) => {
            let slot = array
                .get_mut(index)
                .ok_or_else(|| InternalError(format!("array index {index} out of bounds")))?;
            *slot = new_value.as_i32()? as i16;
        }
        Reference::Array(object_array) => {
            let slot = object_array
                .elements
                .get_mut(index)
                .ok_or_else(|| InternalError(format!("array index {index} out of bounds")))?;
            *slot = new_value.clone();
        }
        _ => {
            return Err(InternalError(
                "VarHandle array: unsupported array type for write".into(),
            ));
        }
    }
    Ok(())
}

/// Atomic compare and swap on an array element under a single write lock.
fn cas_array_element(
    target: &Value,
    index: usize,
    expected: &Value,
    new_value: &Value,
) -> Result<(Value, bool)> {
    let mut array_ref = target.as_reference_mut()?;
    let current = read_from_ref(&array_ref, index)?
        .ok_or_else(|| InternalError("VarHandle array CAS: index out of bounds".into()))?;
    let equal = values_equal(&current, expected);
    if equal {
        write_to_ref(&mut array_ref, index, new_value)?;
    }
    Ok((current, equal))
}

/// Atomic read-modify-write on an array element under a single write lock.
fn atomic_read_write_array<F>(target: &Value, index: usize, compute: F) -> Result<Value>
where
    F: FnOnce(&Value) -> Result<Value>,
{
    let mut array_ref = target.as_reference_mut()?;
    let old = read_from_ref(&array_ref, index)?
        .ok_or_else(|| InternalError("VarHandle array: index out of bounds".into()))?;
    let new_val = compute(&old)?;
    write_to_ref(&mut array_ref, index, &new_val)?;
    Ok(old)
}

/// Compare two values for equality using their actual `Value` types.
fn values_equal(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Int(a_val), Value::Int(b_val)) => a_val == b_val,
        (Value::Long(a_val), Value::Long(b_val)) => a_val == b_val,
        (Value::Float(a_val), Value::Float(b_val)) => a_val.to_bits() == b_val.to_bits(),
        (Value::Double(a_val), Value::Double(b_val)) => a_val.to_bits() == b_val.to_bits(),
        (Value::Object(None), Value::Object(None)) => true,
        (Value::Object(Some(a_ref)), Value::Object(Some(b_ref))) => std::ptr::eq(
            std::ptr::from_ref(a_ref.as_ref()).cast::<u8>(),
            std::ptr::from_ref(b_ref.as_ref()).cast::<u8>(),
        ),
        _ => false,
    }
}

/// Fallback: invoke via `toMethodHandle()` for unrecognized `VarHandle` subclasses.
#[async_method]
async fn invoke_via_method_handle<T: Thread + 'static>(
    thread: &Arc<T>,
    all_params: Vec<Value>,
    access_mode: AccessMode,
) -> Result<Option<Value>> {
    let var_handle = all_params[0].clone();
    let remaining_args: Vec<Value> = all_params[1..].to_vec();

    let access_mode_ordinal = access_mode as i32;

    let access_mode_class = thread
        .class("java/lang/invoke/VarHandle$AccessMode")
        .await?;
    let values_method =
        access_mode_class.try_get_method("values", "()[Ljava/lang/invoke/VarHandle$AccessMode;")?;
    let values_result = thread
        .execute(&access_mode_class, &values_method, &[] as &[Value])
        .await?;
    let values_array = values_result
        .ok_or_else(|| InternalError("AccessMode.values() returned null".to_string()))?;

    let access_mode_value = {
        let values_ref = values_array.as_reference()?;
        let (_, elements) = values_ref.as_class_vec_ref()?;
        let ordinal =
            usize::try_from(access_mode_ordinal).map_err(|e| InternalError(e.to_string()))?;
        if ordinal >= elements.len() {
            return Err(InternalError(format!(
                "AccessMode ordinal {ordinal} out of bounds"
            )));
        }
        elements[ordinal].clone()
    };

    let var_handle_class = thread.class("java/lang/invoke/VarHandle").await?;
    let to_method_handle = var_handle_class.try_get_method(
        "toMethodHandle",
        "(Ljava/lang/invoke/VarHandle$AccessMode;)Ljava/lang/invoke/MethodHandle;",
    )?;

    let mh_result = thread
        .execute(
            &var_handle_class,
            &to_method_handle,
            &[var_handle, access_mode_value],
        )
        .await?;

    let method_handle = mh_result
        .ok_or_else(|| InternalError("VarHandle.toMethodHandle returned null".to_string()))?;

    let mut invoke_args = vec![method_handle];
    invoke_args.extend(remaining_args);

    let invoke_params = Parameters::new(invoke_args);
    super::methodhandle::invoke(thread.clone(), invoke_params).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.compareAndExchange([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn compare_and_exchange<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::CompareAndExchange).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.compareAndExchangeAcquire([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn compare_and_exchange_acquire<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::CompareAndExchangeAcquire).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.compareAndExchangeRelease([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn compare_and_exchange_release<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::CompareAndExchangeRelease).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.compareAndSet([Ljava/lang/Object;)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn compare_and_set<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::CompareAndSet).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.get([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::Get).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAcquire([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_acquire<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::GetAcquire).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndAdd([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_and_add<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::GetAndAdd).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndAddAcquire([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_and_add_acquire<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::GetAndAddAcquire).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndAddRelease([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_and_add_release<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::GetAndAddRelease).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndBitwiseAnd([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_and_bitwise_and<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::GetAndBitwiseAnd).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndBitwiseAndAcquire([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_and_bitwise_and_acquire<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::GetAndBitwiseAndAcquire).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndBitwiseAndRelease([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_and_bitwise_and_release<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::GetAndBitwiseAndRelease).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndBitwiseOr([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_and_bitwise_or<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::GetAndBitwiseOr).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndBitwiseOrAcquire([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_and_bitwise_or_acquire<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::GetAndBitwiseOrAcquire).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndBitwiseOrRelease([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_and_bitwise_or_release<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::GetAndBitwiseOrRelease).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndBitwiseXor([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_and_bitwise_xor<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::GetAndBitwiseXor).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndBitwiseXorAcquire([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_and_bitwise_xor_acquire<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::GetAndBitwiseXorAcquire).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndBitwiseXorRelease([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_and_bitwise_xor_release<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::GetAndBitwiseXorRelease).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndSet([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_and_set<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::GetAndSet).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndSetAcquire([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_and_set_acquire<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::GetAndSetAcquire).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getAndSetRelease([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_and_set_release<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::GetAndSetRelease).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getOpaque([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_opaque<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::GetOpaque).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.getVolatile([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_volatile<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::GetVolatile).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.set([Ljava/lang/Object;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::Set).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.setOpaque([Ljava/lang/Object;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_opaque<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::SetOpaque).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.setRelease([Ljava/lang/Object;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_release<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::SetRelease).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.setVolatile([Ljava/lang/Object;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_volatile<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::SetVolatile).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.weakCompareAndSet([Ljava/lang/Object;)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn weak_compare_and_set<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::WeakCompareAndSet).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.weakCompareAndSetAcquire([Ljava/lang/Object;)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn weak_compare_and_set_acquire<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::WeakCompareAndSetAcquire).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.weakCompareAndSetPlain([Ljava/lang/Object;)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn weak_compare_and_set_plain<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::WeakCompareAndSetPlain).await
}

#[intrinsic_method(
    "java/lang/invoke/VarHandle.weakCompareAndSetRelease([Ljava/lang/Object;)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn weak_compare_and_set_release<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    invoke_var_handle_access_mode(&thread, parameters, AccessMode::WeakCompareAndSetRelease).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_compare_and_exchange_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = compare_and_exchange(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_compare_and_exchange_acquire_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = compare_and_exchange_acquire(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_compare_and_exchange_release_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = compare_and_exchange_release(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_compare_and_set_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = compare_and_set(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get(thread, Parameters::default()).await;
        // With no arguments, get should return an error
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_acquire_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_acquire(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_and_add_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_and_add(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_and_add_acquire_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_and_add_acquire(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_and_add_release_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_and_add_release(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_and_bitwise_and_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_and_bitwise_and(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_and_bitwise_and_acquire_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_and_bitwise_and_acquire(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_and_bitwise_and_release_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_and_bitwise_and_release(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_and_bitwise_or_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_and_bitwise_or(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_and_bitwise_or_acquire_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_and_bitwise_or_acquire(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_and_bitwise_or_release_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_and_bitwise_or_release(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_and_bitwise_xor_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_and_bitwise_xor(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_and_bitwise_xor_acquire_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_and_bitwise_xor_acquire(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_and_bitwise_xor_release_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_and_bitwise_xor_release(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_and_set_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_and_set(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_and_set_acquire_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_and_set_acquire(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_and_set_release_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_and_set_release(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_opaque_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_opaque(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_volatile_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_volatile(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_opaque_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_opaque(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_release_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_release(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_volatile_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_volatile(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_weak_compare_and_set_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = weak_compare_and_set(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_weak_compare_and_set_acquire_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = weak_compare_and_set_acquire(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_weak_compare_and_set_plain_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = weak_compare_and_set_plain(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_weak_compare_and_set_release_requires_varhandle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = weak_compare_and_set_release(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
