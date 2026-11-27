//! Checked field access shared by synchronous helpers and owning field requests.
use super::{
    Arc, CachedResolutionError, InternalError, NullPointerException, Ordering, Reference,
    ResolvedFieldRef, Result, RuntimeContext, RwLock, Value, calls, gc_ref_from_ptr,
    set_top_frame_pc, store_pending_error_async,
};
use ristretto_jit::{FieldAccess, FieldRequest};

impl RuntimeContext {
    pub(crate) async fn invoke_field(
        &self,
        request: FieldRequest,
        arguments: &[i64],
    ) -> Result<i64> {
        let bci = i32::try_from(request.bci)?;
        self.current_bci.store(bci, Ordering::Relaxed);
        set_top_frame_pc(self, bci);
        match access(self, request, arguments).await {
            Ok(bits) => Ok(bits),
            Err(error) => {
                store_pending_error_async(self, error).await;
                Ok(0)
            }
        }
    }
}

pub(super) async fn resolve(ctx: &RuntimeContext, index: u16) -> Result<Arc<ResolvedFieldRef>> {
    if let Some(errors) = ctx.field_errors.get()
        && let Some(error) = errors.read().get(&index)
    {
        return Err(error.to_error());
    }
    let entry = ctx.field_reference(index)?;
    let result =
        crate::field_ref_cache::resolve_field_reference(&ctx.thread, &ctx.class, entry, index)
            .await;
    if let Err(error) = &result {
        ctx.field_errors
            .get_or_init(RwLock::default)
            .write()
            .insert(index, CachedResolutionError::from_error(error));
    }
    result
}

pub(super) async fn prepare(
    ctx: &RuntimeContext,
    index: u16,
    is_static: bool,
    write: bool,
) -> Result<Arc<ResolvedFieldRef>> {
    let resolved = resolve(ctx, index).await?;
    resolved.check_kind(is_static)?;
    if write {
        let frame = ctx.thread.current_frame().ok();
        if frame
            .as_ref()
            .is_some_and(|frame| !Arc::ptr_eq(frame.class(), &ctx.class))
        {
            return Err(InternalError(
                "compiled field access has the wrong current frame".to_owned(),
            ));
        }
        resolved.check_write_from(
            &ctx.class,
            frame.as_ref().map(|frame| frame.method().name()),
        )?;
    }
    if is_static && !resolved.declaring_class.is_initialized()? {
        ctx.thread
            .initialize_class(&resolved.declaring_class)
            .await?;
    }
    Ok(resolved)
}

fn read(ctx: &RuntimeContext, resolved: &ResolvedFieldRef, receiver: Option<i64>) -> Result<Value> {
    let Some(receiver) = receiver else {
        return resolved.get_static();
    };
    if receiver == 0 {
        return Err(NullPointerException(None).into());
    }
    let owner = gc_ref_from_ptr(ctx, receiver)?;
    let guard = owner.read();
    match &*guard {
        Reference::Object(object) => resolved.get(object),
        _ => Err(InternalError(
            "getfield: expected object reference".to_owned(),
        )),
    }
}

fn write(
    ctx: &RuntimeContext,
    resolved: &ResolvedFieldRef,
    receiver: Option<i64>,
    value: Value,
) -> Result<()> {
    let Some(receiver) = receiver else {
        return resolved.put_static(value);
    };
    if receiver == 0 {
        return Err(NullPointerException(None).into());
    }
    let owner = gc_ref_from_ptr(ctx, receiver)?;
    let mut guard = owner.write();
    match &*guard {
        Reference::Object(_) => resolved.put(&mut guard, value),
        _ => Err(InternalError(
            "putfield: expected object reference".to_owned(),
        )),
    }
}

pub(super) async fn get(ctx: &RuntimeContext, index: u16, receiver: Option<i64>) -> Result<Value> {
    let resolved = prepare(ctx, index, receiver.is_none(), false).await?;
    read(ctx, &resolved, receiver)
}

pub(super) async fn put(
    ctx: &RuntimeContext,
    index: u16,
    receiver: Option<i64>,
    value: Value,
) -> Result<()> {
    let resolved = prepare(ctx, index, receiver.is_none(), true).await?;
    write(ctx, &resolved, receiver, value)
}

async fn access(ctx: &RuntimeContext, request: FieldRequest, arguments: &[i64]) -> Result<i64> {
    let is_static = matches!(
        request.kind,
        FieldAccess::GetStatic | FieldAccess::PutStatic
    );
    let store = matches!(request.kind, FieldAccess::PutStatic | FieldAccess::PutField);
    let count = usize::from(!is_static) + usize::from(store);
    if arguments.len() != count {
        return Err(InternalError(
            "invalid field request argument count".to_owned(),
        ));
    }
    let resolved = prepare(ctx, request.index, is_static, store).await?;
    let receiver = if is_static {
        None
    } else {
        arguments.first().copied()
    };
    if store {
        let bits = *arguments
            .last()
            .ok_or_else(|| InternalError("missing field value".to_owned()))?;
        let value = calls::decode_argument(ctx, resolved.field_type(), bits)?;
        write(ctx, &resolved, receiver, value)?;
        Ok(0)
    } else {
        let value = read(ctx, &resolved, receiver)?;
        calls::encode_result(ctx, Some(resolved.field_type()), Some(value))
    }
}
