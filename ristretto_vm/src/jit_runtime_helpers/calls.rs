//! Shared invocation semantics for synchronous helpers and suspended native callers.
//! Outgoing references are stable identities protected by the caller's published
//! snapshot; decoded Values own their arguments across resolution and suspension.
use super::{
    Arc, BaseType, InternalError, Ordering, Result, RuntimeContext, Value, ctx_from_ptr,
    gc_ref_from_ptr, guard_helper, run_async, set_top_frame_pc, store_pending_error,
    store_pending_error_async,
};
use crate::method_ref_cache::InvokeKind;
use ristretto_classfile::{FieldType, JavaStr};

use ristretto_jit::{CallKind, CallRequest};

impl RuntimeContext {
    /// Resolve and await a call after its generated caller has returned. The
    /// caller snapshot and activation cells remain owned across this future.
    pub(crate) async fn invoke_call(&self, request: CallRequest, arguments: &[i64]) -> Result<i64> {
        let bci = i32::try_from(request.bci)?;
        self.current_bci.store(bci, Ordering::Relaxed);
        set_top_frame_pc(self, bci);
        match invoke(self, request.index, arguments, request.kind).await {
            Ok(bits) => Ok(bits),
            Err(error) => {
                store_pending_error_async(self, error).await;
                Ok(0)
            }
        }
    }
}

/// The generated activation owns a readable, aligned buffer until this helper
/// returns. No managed payload pointer or context mutex spans an async call.
pub(super) extern "C" fn jit_invokestatic(
    context: *const u8,
    bci: i32,
    method_index: i32,
    arguments: *const i64,
    count: i32,
) -> i64 {
    invoke_helper(
        context,
        bci,
        method_index,
        arguments,
        count,
        CallKind::Static,
    )
}

/// Same argument-cell contract as `jit_invokestatic`, with the owning receiver
/// represented by the first cell. Constructors return no value.
pub(super) extern "C" fn jit_invokespecial(
    context: *const u8,
    bci: i32,
    method_index: i32,
    arguments: *const i64,
    count: i32,
) -> i64 {
    invoke_helper(
        context,
        bci,
        method_index,
        arguments,
        count,
        CallKind::Special,
    )
}

/// Receiver-first ABI with runtime override selection shared with the interpreter.
pub(super) extern "C" fn jit_invokevirtual(
    context: *const u8,
    bci: i32,
    method_index: i32,
    arguments: *const i64,
    count: i32,
) -> i64 {
    invoke_helper(
        context,
        bci,
        method_index,
        arguments,
        count,
        CallKind::Virtual,
    )
}

/// Receiver-first ABI with interface membership and target selection shared with the interpreter.
pub(super) extern "C" fn jit_invokeinterface(
    context: *const u8,
    bci: i32,
    method_index: i32,
    arguments: *const i64,
    count: i32,
) -> i64 {
    invoke_helper(
        context,
        bci,
        method_index,
        arguments,
        count,
        CallKind::Interface,
    )
}

fn invoke_helper(
    context: *const u8,
    bci: i32,
    method_index: i32,
    arguments: *const i64,
    count: i32,
    kind: CallKind,
) -> i64 {
    guard_helper("jit_invoke", || {
        let ctx = ctx_from_ptr(context);
        ctx.current_bci.store(bci, Ordering::Relaxed);
        set_top_frame_pc(ctx, bci);
        let result = (|| -> Result<i64> {
            let count = usize::try_from(count)?;
            if count > ristretto_jit::CALL_ARGUMENT_CAPACITY
                || (count != 0 && (arguments.is_null() || !arguments.is_aligned()))
            {
                return Err(InternalError(
                    "invalid compiled-call argument buffer".to_owned(),
                ));
            }
            let arguments = if count == 0 {
                &[]
            } else {
                // SAFETY: the generated caller provides count initialized cells,
                // retains their allocation and never writes them during this call.
                unsafe { std::slice::from_raw_parts(arguments, count) }
            };
            run_async(invoke(ctx, u16::try_from(method_index)?, arguments, kind))
        })();
        match result {
            Ok(bits) => bits,
            Err(error) => {
                store_pending_error(ctx, error);
                0
            }
        }
    })
}

async fn invoke(
    ctx: &RuntimeContext,
    index: u16,
    arguments: &[i64],
    kind: CallKind,
) -> Result<i64> {
    let frame = ctx.thread.current_frame()?;
    if !Arc::ptr_eq(frame.class(), &ctx.class) {
        return Err(InternalError(
            "compiled call has the wrong current frame".to_owned(),
        ));
    }
    let (invoke_kind, receiver_count) = match kind {
        CallKind::Static => (InvokeKind::Static, 0),
        CallKind::Special => (InvokeKind::Special, 1),
        CallKind::Virtual => (InvokeKind::Virtual, 1),
        CallKind::Interface => (InvokeKind::Interface, 1),
    };
    let resolved = crate::instruction::resolve_method_ref(&frame, index, invoke_kind).await?;
    let descriptor = JavaStr::cow_from_str(&resolved.method_descriptor);
    let (types, result_type) = FieldType::parse_method_descriptor(&descriptor)?;
    if types.len() + receiver_count != arguments.len() {
        return Err(InternalError(
            "compiled-call argument count disagrees with descriptor".to_owned(),
        ));
    }
    let mut parameters = Vec::with_capacity(arguments.len());
    let arguments = if receiver_count == 0 {
        arguments
    } else {
        let (receiver, rest) = arguments
            .split_first()
            .ok_or_else(|| InternalError("missing compiled receiver".to_owned()))?;
        parameters.push(decode_reference(ctx, *receiver)?);
        rest
    };
    for (field_type, &bits) in types.iter().zip(arguments) {
        parameters.push(decode_argument(ctx, field_type, bits)?);
    }
    let (class, method) = match kind {
        CallKind::Static => {
            // Own converted arguments before initialization can suspend. Only
            // static calls trigger declaring-class initialization at this point.
            if !resolved.declaring_class.is_initialized()? {
                ctx.thread
                    .initialize_class(&resolved.declaring_class)
                    .await?;
            }
            (resolved.declaring_class.clone(), resolved.method.clone())
        }
        CallKind::Special => {
            let receiver = parameters
                .first()
                .ok_or_else(|| InternalError("missing compiled receiver".to_owned()))?;
            // Resolution/kind errors precede null checking; selection errors
            // follow it. Use exactly the interpreter's cached special selection.
            crate::instruction::receiver_class(receiver)?;
            let (target, _) =
                resolved.select_special(frame.class(), ctx.thread.vm()?.method_registry())?;
            (target.declaring_class, target.method)
        }
        CallKind::Virtual => {
            let receiver = parameters
                .first()
                .ok_or_else(|| InternalError("missing compiled receiver".to_owned()))?;
            let (target, _) =
                crate::instruction::select_virtual_target(&frame, &resolved, receiver).await?;
            (target.declaring_class, target.method)
        }
        CallKind::Interface => {
            let receiver = parameters
                .first()
                .ok_or_else(|| InternalError("missing compiled receiver".to_owned()))?;
            let (target, _) =
                crate::instruction::select_interface_target(&frame, &resolved, receiver).await?;
            (target.declaring_class, target.method)
        }
    };
    let result = ctx.thread.execute(&class, &method, &parameters).await?;
    encode_result(ctx, result_type.as_ref(), result)
}

pub(super) fn decode_argument(ctx: &RuntimeContext, kind: &FieldType, bits: i64) -> Result<Value> {
    Ok(match kind {
        FieldType::Base(BaseType::Long) => Value::Long(bits),
        FieldType::Base(BaseType::Double) => Value::Double(f64::from_bits(bits as u64)),
        FieldType::Base(BaseType::Float) => Value::Float(f32::from_bits(bits as u32)),
        FieldType::Base(_) => Value::Int(bits as i32),
        FieldType::Object(_) | FieldType::Array(_) => decode_reference(ctx, bits)?,
    })
}

fn decode_reference(ctx: &RuntimeContext, bits: i64) -> Result<Value> {
    if bits == 0 {
        return Ok(Value::Object(None));
    }
    let published = ctx.reference_roots.lock().get(&bits).cloned();
    let owner = match published {
        Some(owner) => owner,
        None => gc_ref_from_ptr(ctx, bits)?,
    };
    Ok(Value::Object(Some(owner)))
}

pub(super) fn encode_result(
    ctx: &RuntimeContext,
    kind: Option<&FieldType>,
    result: Option<Value>,
) -> Result<i64> {
    Ok(match (kind, result) {
        (None, None)
        | (Some(FieldType::Object(_) | FieldType::Array(_)), Some(Value::Object(None))) => 0,
        (Some(FieldType::Base(BaseType::Long)), Some(Value::Long(value))) => value,
        (Some(FieldType::Base(BaseType::Double)), Some(Value::Double(value))) => {
            value.to_bits() as i64
        }
        (Some(FieldType::Base(BaseType::Float)), Some(Value::Float(value))) => {
            i64::from(value.to_bits())
        }
        (
            Some(FieldType::Base(
                BaseType::Boolean
                | BaseType::Byte
                | BaseType::Char
                | BaseType::Short
                | BaseType::Int,
            )),
            Some(Value::Int(value)),
        ) => i64::from(value),
        (Some(FieldType::Object(_) | FieldType::Array(_)), Some(Value::Object(Some(owner)))) => {
            let handle = owner.as_handle_i64();
            ctx.retain_reference_owner(owner.rooted_clone());
            handle
        }
        (kind, result) => {
            return Err(InternalError(format!(
                "compiled-call return mismatch: {kind:?}, {result:?}"
            )));
        }
    })
}
