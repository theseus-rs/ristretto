use crate::bounds;
use crate::java::lang::class::get_class_name;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::{InternalError, PoisonedLock};
use ristretto_types::JavaError::ClassCastException;
use ristretto_types::JavaError::{
    ArrayIndexOutOfBoundsException, ArrayStoreException, NullPointerException,
    WrongMethodTypeException,
};
use ristretto_types::Thread;
use ristretto_types::method_resolution::receiver_class;
use ristretto_types::{Assignable, Parameters, Result, VM};
use std::sync::Arc;

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

/// Returns the direct `VarHandle` wrapped by a `LazyInitializingVarHandle`, or `None` if
/// `var_handle` is not such a wrapper.
///
/// Since Java 22, a `VarHandle` for a static field created before its declaring class is
/// initialized (e.g. in that class's own static initializer) is a `LazyInitializingVarHandle`,
/// which initializes the class on first use and delegates to its `target`. Native static
/// dispatch initializes the class via `get_class`. JDK access methods receive the original
/// wrapper so their call to `target()` initializes the class before any value cast.
fn lazy_initializing_target(var_handle: &Value) -> Result<Option<Value>> {
    let Ok(object) = var_handle.as_object_ref() else {
        return Ok(None);
    };
    if object.class().name() != "java/lang/invoke/LazyInitializingVarHandle" {
        return Ok(None);
    }
    Ok(Some(object.value("target")?))
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

/// A mode yields a return value and, when the operation succeeds, a new storage value.
impl AccessMode {
    fn read_only(self) -> bool {
        matches!(
            self,
            Self::Get | Self::GetVolatile | Self::GetAcquire | Self::GetOpaque
        )
    }

    fn update(self, current: Value, operands: &[Value]) -> Result<(Option<Value>, Option<Value>)> {
        let rhs = operands
            .last()
            .ok_or_else(|| InternalError("Missing VarHandle operand".into()))?;
        let replacement = match self {
            Self::Set | Self::SetVolatile | Self::SetOpaque | Self::SetRelease => {
                return Ok((None, Some(rhs.clone())));
            }
            Self::CompareAndSet
            | Self::WeakCompareAndSet
            | Self::WeakCompareAndSetAcquire
            | Self::WeakCompareAndSetRelease
            | Self::WeakCompareAndSetPlain
            | Self::CompareAndExchange
            | Self::CompareAndExchangeAcquire
            | Self::CompareAndExchangeRelease => {
                let expected = bounds::index(operands, 0, "VarHandle expected value")?;
                let matches = values_equal(&current, expected);
                let result = if matches!(
                    self,
                    Self::CompareAndExchange
                        | Self::CompareAndExchangeAcquire
                        | Self::CompareAndExchangeRelease
                ) {
                    current
                } else {
                    Value::from(matches)
                };
                return Ok((Some(result), matches.then(|| rhs.clone())));
            }
            Self::GetAndSet | Self::GetAndSetAcquire | Self::GetAndSetRelease => rhs.clone(),
            Self::GetAndAdd | Self::GetAndAddAcquire | Self::GetAndAddRelease => {
                match (&current, rhs) {
                    (Value::Float(a), Value::Float(b)) => Value::Float(a + b),
                    (Value::Double(a), Value::Double(b)) => Value::Double(a + b),
                    _ => apply_numeric_result(
                        &current,
                        value_to_i64(&current)?.wrapping_add(value_to_i64(rhs)?),
                    )?,
                }
            }
            Self::GetAndBitwiseOr | Self::GetAndBitwiseOrAcquire | Self::GetAndBitwiseOrRelease => {
                apply_numeric_result(&current, value_to_i64(&current)? | value_to_i64(rhs)?)?
            }
            Self::GetAndBitwiseAnd
            | Self::GetAndBitwiseAndAcquire
            | Self::GetAndBitwiseAndRelease => {
                apply_numeric_result(&current, value_to_i64(&current)? & value_to_i64(rhs)?)?
            }
            Self::GetAndBitwiseXor
            | Self::GetAndBitwiseXorAcquire
            | Self::GetAndBitwiseXorRelease => {
                apply_numeric_result(&current, value_to_i64(&current)? ^ value_to_i64(rhs)?)?
            }
            _ => return Ok((Some(current), None)),
        };
        Ok((Some(current), Some(replacement)))
    }

    /// Resolve mode support before adapting arguments. A direct update also uses
    /// the returned member to preserve the access method's coordinate checks.
    async fn resolve_member<T: Thread + 'static>(
        self,
        thread: &T,
        handle: &Value,
        storage: Option<&VarHandleStorage>,
        adapting: bool,
    ) -> Result<Option<Value>> {
        if storage.is_none() && adapting {
            thread
                .try_invoke(
                    "java/lang/invoke/VarHandle",
                    "getMethodHandle(I)Ljava/lang/invoke/MethodHandle;",
                    &[handle.clone(), Value::Int(self as i32)],
                )
                .await?;
        } else if storage.is_some() && !self.read_only() {
            let form = handle.as_object_ref()?.value("vform")?;
            return thread
                .try_invoke(
                    "java/lang/invoke/VarForm",
                    "getMemberName(I)Ljava/lang/invoke/MemberName;",
                    &[form, Value::Int(self as i32)],
                )
                .await
                .map(Some);
        }
        Ok(None)
    }

    async fn java_value<T: Thread + 'static>(self, thread: &T) -> Result<Value> {
        let class = thread
            .class("java/lang/invoke/VarHandle$AccessMode")
            .await?;
        let values = class.static_value("$VALUES")?;
        let array = values.as_reference()?;
        let (_, elements) = array.as_class_vec_ref()?;
        Ok(bounds::index(elements, self as usize, "VarHandle access mode")?.clone())
    }
}

fn wrapper(primitive: &str) -> Option<(&'static str, char)> {
    Some(match primitive {
        "boolean" => ("Boolean", 'Z'),
        "byte" => ("Byte", 'B'),
        "char" => ("Character", 'C'),
        "short" => ("Short", 'S'),
        "int" => ("Integer", 'I'),
        "long" => ("Long", 'J'),
        "float" => ("Float", 'F'),
        "double" => ("Double", 'D'),
        _ => return None,
    })
}

#[expect(
    clippy::cast_precision_loss,
    reason = "JVM widening rounds to the target precision"
)]
fn widen_primitive(value: Value, target: &str) -> Result<Value> {
    match (value, target) {
        (Value::Int(n), "byte" | "short" | "char" | "int" | "boolean") => Ok(Value::Int(n)),
        (Value::Int(n), "long") => Ok(Value::Long(i64::from(n))),
        (Value::Int(n), "float") => Ok(Value::Float(n as f32)),
        (Value::Int(n), "double") => Ok(Value::Double(f64::from(n))),
        (Value::Long(n), "float") => Ok(Value::Float(n as f32)),
        (Value::Long(n), "double") => Ok(Value::Double(n as f64)),
        (Value::Float(n), "double") => Ok(Value::Double(f64::from(n))),
        (value, _) => Err(InternalError(format!(
            "Invalid widening from {value} to {target}"
        ))),
    }
}

async fn adapt_value<T: Thread + 'static>(
    thread: &T,
    source: &Value,
    target: &Value,
    value: Option<Value>,
) -> Result<Option<Value>> {
    if values_equal(source, target) {
        return Ok(value);
    }
    let source_name = get_class_name(&*source.as_object_ref()?)?;
    let target_name = get_class_name(&*target.as_object_ref()?)?;
    if target_name == "void" {
        return Ok(None);
    }
    let Some(mut value) = value else {
        return Ok(Some(match target_name.as_str() {
            "float" => Value::Float(0.0),
            "double" => Value::Double(0.0),
            "long" => Value::Long(0),
            name if wrapper(name).is_some() => Value::Int(0),
            _ => Value::Object(None),
        }));
    };
    if let Some((name, descriptor)) = wrapper(&source_name) {
        if wrapper(&target_name).is_some() {
            // MethodType.isConvertibleTo already checked the Java widening rules.
            return Ok(Some(widen_primitive(value, &target_name)?));
        }
        value = thread
            .try_invoke(
                &format!("java/lang/{name}"),
                &format!("valueOf({descriptor})Ljava/lang/{name};"),
                &[value],
            )
            .await?;
    }
    if let Some((name, descriptor)) = wrapper(&target_name) {
        let value = thread
            .try_invoke(
                "sun/invoke/util/ValueConversions",
                &format!("unbox{name}(Ljava/lang/Object;Z){descriptor}"),
                &[value, Value::from(false)],
            )
            .await?;
        return Ok(Some(value));
    }
    if !value.is_null() {
        let source = receiver_class(thread, &value).await?;
        let target = crate::java::lang::class::get_class_no_init(thread, target).await?;
        if !target.is_assignable_from(thread, &source).await? {
            return Err(ClassCastException {
                source_class_name: source.name().replace('/', "."),
                target_class_name: target.name().replace('/', "."),
            }
            .into());
        }
    }
    Ok(Some(value))
}

fn signature(method_type: &Value) -> Result<(Vec<Value>, Value)> {
    let object = method_type.as_object_ref()?;
    let params = object.value("ptypes")?;
    let array = params.as_reference()?;
    Ok((array.as_class_vec_ref()?.1.to_vec(), object.value("rtype")?))
}

// Use the JDK atomic accessor for subword operands. Verified bytecode can pass any JVM int,
// and the accessor narrows the expected value, replacement and witness at different stages.
fn has_subword_operand(method_type: &Value) -> Result<bool> {
    let parameters = method_type.as_object_ref()?.value("ptypes")?;
    let parameters = parameters.as_reference()?;
    let (_, types) = parameters.as_class_vec_ref()?;
    let value_type = types
        .last()
        .ok_or_else(|| InternalError("Missing atomic operand type".into()))?;
    Ok(matches!(
        get_class_name(&*value_type.as_object_ref()?)?.as_str(),
        "boolean" | "byte" | "short" | "char"
    ))
}

/// Direct access methods erase reference types but retain each primitive type.
fn same_erasure(first: &Value, second: &Value) -> Result<bool> {
    if values_equal(first, second) {
        return Ok(true);
    }
    let is_reference = |mirror: &Value| -> Result<bool> {
        let name = get_class_name(&*mirror.as_object_ref()?)?;
        Ok(name != "void" && wrapper(&name).is_none())
    };
    Ok(is_reference(first)? && is_reference(second)?)
}

fn same_parameter_erasure(sources: &[Value], targets: &[Value]) -> Result<bool> {
    for (source, target) in sources.iter().zip(targets) {
        if !same_erasure(source, target)? {
            return Ok(false);
        }
    }
    Ok(sources.len() == targets.len())
}

/// Convert operands before accessing storage. When several conversions are needed,
/// an asType adapter on an argument collector runs the filters in JDK order.
async fn adapt_operands<T: Thread + 'static>(
    thread: &Arc<T>,
    site: &Value,
    sources: &[Value],
    target_type: &Value,
    targets: &[Value],
    operands: &mut [Value],
) -> Result<()> {
    let mut conversions = operands
        .iter_mut()
        .zip(sources)
        .zip(targets)
        .filter(|((_, source), target)| !values_equal(source, target));
    let Some(((operand, source), target)) = conversions.next() else {
        return Ok(());
    };
    if conversions.next().is_none() {
        *operand = adapt_value(&**thread, source, target, Some(operand.clone()))
            .await?
            .ok_or_else(|| InternalError("Void argument".into()))?;
        return Ok(());
    }
    let collector = thread
        .try_invoke(
            "java/lang/invoke/MethodHandleImpl",
            "varargsArray(I)Ljava/lang/invoke/MethodHandle;",
            &[Value::Int(i32::try_from(operands.len())?)],
        )
        .await?;
    let array_class = collector
        .as_object_ref()?
        .value("type")?
        .as_object_ref()?
        .value("rtype")?;
    let collector_type = async |ty: &Value| {
        thread
            .try_invoke(
                "java/lang/invoke/MethodType",
                "changeReturnType(Ljava/lang/Class;)Ljava/lang/invoke/MethodType;",
                &[ty.clone(), array_class.clone()],
            )
            .await
    };
    let collector = thread
        .try_invoke(
            "java/lang/invoke/MethodHandle",
            "asType(Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MethodHandle;",
            &[collector, collector_type(target_type).await?],
        )
        .await?;
    let mut invocation = Parameters::default();
    invocation.push(collector);
    for arg in operands.iter() {
        invocation.push(arg.clone());
    }
    invocation.call_site_type = Some(collector_type(site).await?);
    let collected = super::methodhandle::invoke(thread.clone(), invocation)
        .await?
        .ok_or_else(|| InternalError("Argument collector returned void".into()))?;
    let (_, values) = collected.as_class_vec_ref()?;
    for ((arg, ty), value) in operands.iter_mut().zip(targets).zip(values.iter()) {
        *arg = if wrapper(&get_class_name(&*ty.as_object_ref()?)?).is_some() {
            value.as_object_ref()?.value("value")?
        } else {
            value.clone()
        };
    }
    Ok(())
}

#[async_method]
async fn invoke_var_handle_access_mode<T: Thread + 'static>(
    thread: &Arc<T>,
    mut parameters: Parameters,
    mode: AccessMode,
) -> Result<Option<Value>> {
    let call_site = parameters.call_site_type.take();
    let mut args = parameters.into_vec();
    let original = bounds::index(&args, 0, "VarHandle receiver")?.clone();
    let handle = lazy_initializing_target(&original)?.unwrap_or_else(|| original.clone());
    let storage = classify_var_handle(&handle);
    let mut return_types = None;

    let mode_value = mode.java_value(&**thread).await?;
    let target_type = thread
        .try_invoke(
            "java/lang/invoke/VarHandle",
            "accessModeType(Ljava/lang/invoke/VarHandle$AccessMode;)Ljava/lang/invoke/MethodType;",
            &[original.clone(), mode_value.clone()],
        )
        .await?;
    let mismatch = call_site.filter(|site| !values_equal(site, &target_type));
    if mismatch.is_some()
        && original
            .as_object_ref()?
            .value("exact")
            .is_ok_and(|v| v.as_bool().unwrap_or(false))
    {
        return Err(
            WrongMethodTypeException("Expected exact VarHandle access mode type".into()).into(),
        );
    }
    let direct_member = mode
        .resolve_member(&**thread, &handle, storage.as_ref(), mismatch.is_some())
        .await?;
    if let Some(site) = mismatch {
        let convertible = thread
            .try_invoke(
                "java/lang/invoke/MethodType",
                "isConvertibleTo(Ljava/lang/invoke/MethodType;)Z",
                &[target_type.clone(), site.clone()],
            )
            .await?;
        if !convertible.as_bool()? {
            return Err(
                WrongMethodTypeException("Cannot adapt VarHandle access mode type".into()).into(),
            );
        }
        let (sources, site_return) = signature(&site)?;
        let (targets, actual_return) = signature(&target_type)?;
        // The direct JDK entry checks the receiver, index, and values in the order
        // required by the access mode. An asType adapter casts arguments earlier.
        if let Some(member) = &direct_member
            // Plain array assignment casts the value before checking the destination.
            && !matches!((&storage, mode), (Some(VarHandleStorage::Array), AccessMode::Set))
            && let Some(last) = targets.last()
            && wrapper(&get_class_name(&*last.as_object_ref()?)?).is_none()
        {
            let discard = get_class_name(&*site_return.as_object_ref()?)? == "void";
            if (discard || same_erasure(&site_return, &actual_return)?)
                && same_parameter_erasure(&sources, &targets)?
            {
                // Static accessors call target() on the original lazy wrapper before casting.
                let result =
                    super::methodhandle::call_method_handle_target(thread.clone(), member, args)
                        .await?;
                return if discard {
                    Ok(None)
                } else {
                    adapt_value(&**thread, &actual_return, &site_return, Some(result)).await
                };
            }
        }
        let operands = args
            .get_mut(1..)
            .ok_or_else(|| InternalError("Missing VarHandle receiver".into()))?;
        adapt_operands(thread, &site, &sources, &target_type, &targets, operands).await?;
        return_types = Some((actual_return, site_return));
    }
    let result = if let Some(member) = direct_member
        && mode as i32 >= AccessMode::CompareAndSet as i32
        && has_subword_operand(&target_type)?
    {
        Some(super::methodhandle::call_method_handle_target(thread.clone(), &member, args).await?)
    } else {
        let operands = bounds::range_from(&args, 1.., "VarHandle operands")?;
        match storage {
            Some(VarHandleStorage::FieldInstance { field_offset }) => {
                dispatch_field_instance(mode, field_offset, operands)
            }
            Some(VarHandleStorage::FieldStatic { field_offset }) => {
                dispatch_field_static(
                    &**thread,
                    mode,
                    &handle,
                    field_offset,
                    operands,
                    !values_equal(&original, &handle),
                )
                .await
            }
            Some(VarHandleStorage::Array) => dispatch_array(&**thread, mode, operands).await,
            None => invoke_via_method_handle(thread, args, mode, mode_value).await,
        }?
    };
    if let Some((source, target)) = return_types {
        adapt_value(&**thread, &source, &target, result).await
    } else {
        Ok(result)
    }
}

async fn dispatch_field_static<T: Thread + 'static>(
    thread: &T,
    mode: AccessMode,
    handle: &Value,
    field_offset: usize,
    operands: &[Value],
    initialize: bool,
) -> Result<Option<Value>> {
    let base = handle.as_object_ref()?.value("base")?;
    // Only the lazy wrapper supplies an initialization barrier. Older factories can
    // initialize a lookup class without initializing the interface declaring the field.
    let class = if initialize {
        crate::java::lang::class::get_class(thread, &base).await?
    } else {
        crate::java::lang::class::get_class_no_init(thread, &base).await?
    };
    let (field, storage) = class.static_field_value(field_offset)?;
    Ok(if mode.read_only() {
        Some(
            storage
                .read()
                .map_err(|e| PoisonedLock(e.to_string()))?
                .clone(),
        )
    } else {
        let mut slot = storage.write().map_err(|e| PoisonedLock(e.to_string()))?;
        let (result, new) = mode.update(slot.clone(), operands)?;
        if let Some(value) = new {
            *slot = field.checked_value(value)?;
        }
        result
    })
}

fn dispatch_field_instance(
    mode: AccessMode,
    offset: usize,
    args: &[Value],
) -> Result<Option<Value>> {
    let (target, operands) = args
        .split_first()
        .ok_or_else(|| InternalError("Missing VarHandle receiver".into()))?;
    if target.is_null() {
        return Err(NullPointerException(None).into());
    }
    if mode.read_only() {
        return Ok(Some(target.as_object_ref()?.value(offset)?));
    }
    let mut object = target.as_object_mut()?;
    let (result, new) = mode.update(object.value(offset)?, operands)?;
    if let Some(value) = new {
        object.set_value(offset, value)?;
    }
    Ok(result)
}

async fn dispatch_array<T: Thread + 'static>(
    thread: &T,
    mode: AccessMode,
    args: &[Value],
) -> Result<Option<Value>> {
    let [target, index, operands @ ..] = args else {
        return Err(InternalError("Missing VarHandle array coordinates".into()));
    };
    if target.is_null() {
        return Err(NullPointerException(None).into());
    }
    let index = index.as_i32()?;
    let read = |array: &Reference| -> Result<Value> {
        macro_rules! element {
            ($elements:expr) => {{
                let elements = &$elements;
                usize::try_from(index)
                    .ok()
                    .and_then(|i| elements.get(i))
                    .cloned()
                    .map(Value::from)
                    .ok_or_else(|| {
                        ArrayIndexOutOfBoundsException {
                            index,
                            length: elements.len(),
                        }
                        .into()
                    })
            }};
        }
        match array {
            Reference::BooleanArray(a) | Reference::ByteArray(a) => element!(a),
            Reference::CharArray(a) => element!(a),
            Reference::ShortArray(a) => element!(a),
            Reference::IntArray(a) => element!(a),
            Reference::LongArray(a) => element!(a),
            Reference::FloatArray(a) => element!(a),
            Reference::DoubleArray(a) => element!(a),
            Reference::Array(a) => element!(a.elements),
            Reference::Object(_) => Err(InternalError("Expected array".into())),
        }
    };
    if mode.read_only() {
        return Ok(Some(read(&*target.as_reference()?)?));
    }
    let array_class = match &*target.as_reference()? {
        Reference::Array(a) => {
            if usize::try_from(index).map_or(true, |i| i >= a.elements.len()) {
                return Err(ArrayIndexOutOfBoundsException {
                    index,
                    length: a.elements.len(),
                }
                .into());
            }
            Some(a.class.clone())
        }
        _ => None,
    };
    if let Some(class) = array_class
        && let Some(new) = operands.last().filter(|v| !v.is_null())
    {
        let loader = thread.vm()?.class_loader().read().await.clone();
        let component = loader
            .array_component_class(&class)
            .await?
            .ok_or_else(|| InternalError("Missing array component class".into()))?;
        let component = thread.resolve_class(&component, false).await?;
        let source = receiver_class(thread, new).await?;
        if !component.is_assignable_from(thread, &source).await? {
            return Err(ArrayStoreException(source.name().to_owned()).into());
        }
    }
    let mut array = target.as_reference_mut()?;
    let (result, new) = mode.update(read(&array)?, operands)?;
    if let Some(value) = new {
        write_to_ref(&mut array, usize::try_from(index)?, &value)?;
    }
    Ok(result)
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
        Reference::BooleanArray(array) => {
            *bounds::index_mut(array, index, "boolean array store")? =
                (new_value.as_i32()? & 1) as i8;
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
        Reference::Object(_) => {
            return Err(InternalError(
                "VarHandle array: unsupported array type for write".into(),
            ));
        }
    }
    Ok(())
}

/// Compare two values for equality using their actual `Value` types.
fn values_equal(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Int(a_val), Value::Int(b_val)) => a_val == b_val,
        (Value::Long(a_val), Value::Long(b_val)) => a_val == b_val,
        (Value::Float(a_val), Value::Float(b_val)) => a_val.to_bits() == b_val.to_bits(),
        (Value::Double(a_val), Value::Double(b_val)) => a_val.to_bits() == b_val.to_bits(),
        (Value::Object(None), Value::Object(None)) => true,
        (Value::Object(Some(a_ref)), Value::Object(Some(b_ref))) => a_ref.ptr_eq(b_ref),
        _ => false,
    }
}

/// Read the unbound access handle cached by the JDK. Java 11/17 keep the table
/// in `TypesAndInvokers`; newer JDKs store it on the `VarHandle` itself.
fn cached_access_handle(var_handle: &Value, access_mode: AccessMode) -> Result<Option<Value>> {
    let table = {
        let handle = var_handle.as_object_ref()?;
        let form = handle.value("vform")?;
        if form.as_object_ref()?.value("memberName_table")?.is_null() {
            // Indirect handles can bind a different receiver through asDirect().
            // Leave their adapters to the JDK's toMethodHandle implementation.
            return Ok(None);
        }
        if let Ok(table) = handle.value("methodHandleTable") {
            table
        } else {
            let invokers = handle.value("typesAndInvokers")?;
            if invokers.is_null() {
                return Ok(None);
            }
            invokers.as_object_ref()?.value("methodHandle_table")?
        }
    };
    if table.is_null() {
        return Ok(None);
    }
    let table = table.as_reference()?;
    let (_, handles) = table.as_class_vec_ref()?;
    Ok(handles
        .get(access_mode as usize)
        .filter(|handle| !handle.is_null())
        .cloned())
}

/// Invoke an access handle for storage types implemented by the JDK.
///
/// On the first access, retain the ordinary `toMethodHandle` path so the JDK
/// resolves the mode and reports unsupported operations. Subsequent accesses
/// reuse its unbound handle and pass the receiver explicitly. This avoids
/// allocating a bound adapter and interpreting its `LambdaForm` on every access,
/// while retaining the JDK's casts, bounds, alignment, and memory ordering.
#[async_method]
async fn invoke_via_method_handle<T: Thread + 'static>(
    thread: &Arc<T>,
    all_params: Vec<Value>,
    access_mode: AccessMode,
    access_mode_value: Value,
) -> Result<Option<Value>> {
    let (var_handle, remaining_args) = all_params.split_first().ok_or_else(|| {
        InternalError("VarHandle fallback requires at least the VarHandle".to_string())
    })?;
    if let Some(method_handle) = cached_access_handle(var_handle, access_mode)? {
        let mut invoke_args = Vec::with_capacity(all_params.len() + 1);
        invoke_args.push(method_handle);
        invoke_args.extend_from_slice(&all_params);
        return super::methodhandle::invoke(thread.clone(), Parameters::new(invoke_args)).await;
    }
    let var_handle = var_handle.clone();
    let remaining_args = remaining_args.to_vec();

    let mh_result = thread
        .invoke(
            "java/lang/invoke/VarHandle",
            "toMethodHandle(Ljava/lang/invoke/VarHandle$AccessMode;)Ljava/lang/invoke/MethodHandle;",
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
    async fn widening_float_return_does_not_allocate_java_objects() -> Result<()> {
        use ristretto_types::JavaObject;

        let (vm, thread) = crate::test::thread().await?;
        let source = thread.class("float").await?.to_object(&thread).await?;
        let target = thread.class("double").await?.to_object(&thread).await?;
        let collector = vm.garbage_collector();
        let before = collector.statistics()?.bytes_live;
        for n in [-0.0_f32, 1.25, f32::MAX, f32::MIN_POSITIVE] {
            let value = adapt_value(&*thread, &source, &target, Some(Value::Float(n)))
                .await?
                .expect("double");
            assert_eq!(value.as_f64()?.to_bits(), f64::from(n).to_bits());
        }
        assert_eq!(collector.statistics()?.bytes_live, before);
        Ok(())
    }

    #[tokio::test]
    async fn test_dispatch_field_instance_updates_by_offset() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let target = thread
            .object("java/lang/Integer", "I", &[Value::Int(10)])
            .await?;
        let class = thread.class("java/lang/Integer").await?;
        let offset = class.object_field_offset("value")?;

        let result = dispatch_field_instance(
            AccessMode::GetAndSet,
            offset,
            &[target.clone(), Value::Int(12)],
        )?;
        assert_eq!(Some(Value::Int(10)), result);
        let result = dispatch_field_instance(
            AccessMode::GetAndAdd,
            offset,
            &[target.clone(), Value::Int(3)],
        )?;
        assert_eq!(Some(Value::Int(12)), result);
        let result = dispatch_field_instance(
            AccessMode::GetAndBitwiseAnd,
            offset,
            &[target.clone(), Value::Int(7)],
        )?;
        assert_eq!(Some(Value::Int(15)), result);
        let result = dispatch_field_instance(
            AccessMode::GetAndBitwiseXor,
            offset,
            &[target.clone(), Value::Int(2)],
        )?;
        assert_eq!(Some(Value::Int(7)), result);
        assert_eq!(Value::Int(5), target.as_object_ref()?.value(offset)?);
        Ok(())
    }

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
