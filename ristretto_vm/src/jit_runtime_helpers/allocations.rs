//! Allocation awaits run with no generated frame, raw managed borrow or context lock active.
use super::{
    Arc, BaseType, Class, InternalError, NegativeArraySizeException, Object, Ordering, Reference,
    Result, RuntimeContext, Value, allocate_array_in_async, allocate_primitive_array_in_async,
    build_multianewarray, resolve_class_ref_async, resolve_class_ref_no_init_async,
    set_top_frame_pc, store_pending_error_async,
};
use ristretto_classfile::attributes::ArrayType;
use ristretto_jit::{AllocationKind, AllocationRequest};

impl RuntimeContext {
    pub(crate) async fn invoke_allocation(
        &self,
        request: AllocationRequest,
        arguments: &[i64],
    ) -> Result<i64> {
        let bci = i32::try_from(request.bci)?;
        self.current_bci.store(bci, Ordering::Relaxed);
        set_top_frame_pc(self, bci);
        let result = allocate(self, request.kind, arguments).await;
        match result {
            Ok(bits) => Ok(bits),
            Err(error) => {
                store_pending_error_async(self, error).await;
                Ok(0)
            }
        }
    }
}

async fn allocate(ctx: &RuntimeContext, kind: AllocationKind, arguments: &[i64]) -> Result<i64> {
    match kind {
        AllocationKind::Object(index) => {
            if !arguments.is_empty() {
                return Err(InternalError("new has no dimension arguments".to_owned()));
            }
            object(ctx, index).await
        }
        AllocationKind::PrimitiveArray(kind) => {
            let base = match kind {
                ArrayType::Boolean => BaseType::Boolean,
                ArrayType::Byte => BaseType::Byte,
                ArrayType::Char => BaseType::Char,
                ArrayType::Short => BaseType::Short,
                ArrayType::Int => BaseType::Int,
                ArrayType::Long => BaseType::Long,
                ArrayType::Float => BaseType::Float,
                ArrayType::Double => BaseType::Double,
            };
            primitive_array(ctx, &base, single_size(arguments)?).await
        }
        AllocationKind::ReferenceArray(index) => {
            reference_array(ctx, index, single_size(arguments)?).await
        }
        AllocationKind::MultiArray(index) => {
            let sizes = arguments
                .iter()
                .map(|value| i32::try_from(*value).map_err(Into::into))
                .collect::<Result<Vec<_>>>()?;
            multi_array(ctx, index, &sizes).await
        }
    }
}

fn single_size(arguments: &[i64]) -> Result<i32> {
    let [size] = arguments else {
        return Err(InternalError("array requires one dimension".to_owned()));
    };
    Ok(i32::try_from(*size)?)
}

async fn publish(ctx: &RuntimeContext, reference: Reference) -> Result<i64> {
    let value = Value::try_new_object_async(ctx.garbage_collector(), reference).await?;
    retain(ctx, value)
}

fn retain(ctx: &RuntimeContext, value: Value) -> Result<i64> {
    match value {
        Value::Object(Some(owner)) => {
            ctx.retain_helper_result(&owner);
            Ok(owner.as_handle_i64())
        }
        Value::Object(None) => Ok(0),
        other => Err(InternalError(format!(
            "allocation returned a non-reference: {other:?}"
        ))),
    }
}

pub(super) async fn primitive_array(
    ctx: &RuntimeContext,
    kind: &BaseType,
    count: i32,
) -> Result<i64> {
    if count < 0 {
        return Err(NegativeArraySizeException(count.to_string()).into());
    }
    let reference =
        allocate_primitive_array_in_async(ctx.garbage_collector(), kind, usize::try_from(count)?)
            .await?;
    publish(ctx, reference).await
}

pub(super) async fn object(ctx: &RuntimeContext, index: u16) -> Result<i64> {
    let class = resolve_class_ref_async(ctx, index).await?;
    let object = Object::new_in_async(ctx.garbage_collector(), class).await?;
    publish(ctx, Reference::Object(object)).await
}

pub(super) async fn reference_array(ctx: &RuntimeContext, index: u16, count: i32) -> Result<i64> {
    let component = resolve_class_ref_no_init_async(ctx, index).await?;
    let array_class = ctx.thread.array_class(&component, 1).await?;
    if count < 0 {
        return Err(NegativeArraySizeException(count.to_string()).into());
    }
    let elements = allocate_array_in_async(
        ctx.garbage_collector(),
        usize::try_from(count)?,
        Value::Object(None),
    )
    .await?;
    publish(ctx, Reference::try_from((array_class, elements))?).await
}

pub(super) async fn multi_array(
    ctx: &RuntimeContext,
    index: u16,
    dimensions: &[i32],
) -> Result<i64> {
    if dimensions.is_empty() || dimensions.len() > 255 {
        return Err(InternalError(
            "multianewarray: invalid dimension count".to_owned(),
        ));
    }
    // Resolution precedes negative-size errors, and never initializes the component.
    let class: Arc<Class> = resolve_class_ref_no_init_async(ctx, index).await?;
    if !class.is_array() || class.array_dimensions() < dimensions.len() {
        return Err(InternalError(format!(
            "multianewarray: class {} has {} array dimensions but {} were requested",
            class.name(),
            class.array_dimensions(),
            dimensions.len()
        )));
    }
    let mut sizes = Vec::with_capacity(dimensions.len());
    for value in dimensions {
        if *value < 0 {
            return Err(NegativeArraySizeException(value.to_string()).into());
        }
        sizes.push(usize::try_from(*value)?);
    }
    let value = build_multianewarray(&ctx.thread, class, &sizes, 0).await?;
    retain(ctx, value)
}
