//! Type checks shared by synchronous embeddings and owning runtime requests.
use super::{
    ClassCastException, InternalError, Ordering, Result, RuntimeContext, gc_ref_from_ptr,
    resolve_class_ref_no_init_async, set_top_frame_pc, store_pending_error_async,
};
use ristretto_jit::{TypeCheckKind, TypeCheckRequest};

impl RuntimeContext {
    pub(crate) async fn invoke_type_check(
        &self,
        request: TypeCheckRequest,
        arguments: &[i64],
    ) -> Result<i64> {
        let bci = i32::try_from(request.bci)?;
        self.current_bci.store(bci, Ordering::Relaxed);
        set_top_frame_pc(self, bci);
        let result = match arguments {
            [object] => check(self, request.kind, request.index, *object).await,
            _ => Err(InternalError(
                "invalid type check argument count".to_owned(),
            )),
        };
        match result {
            Ok(bits) => Ok(bits),
            Err(error) => {
                store_pending_error_async(self, error).await;
                Ok(0)
            }
        }
    }
}

pub(super) async fn check(
    ctx: &RuntimeContext,
    kind: TypeCheckKind,
    index: u16,
    object: i64,
) -> Result<i64> {
    // Both instructions accept null without resolving the symbolic target.
    if object == 0 {
        return Ok(0);
    }
    let class = resolve_class_ref_no_init_async(ctx, index).await?;
    let owner = gc_ref_from_ptr(ctx, object)?;
    let matches = crate::instruction::is_instance_of(&ctx.thread, &owner, &class).await?;
    if kind == TypeCheckKind::InstanceOf {
        return Ok(i64::from(matches));
    }
    if !matches {
        let source_class_name = owner.read().class_name()?.replace('/', ".");
        let target_class_name = ctx
            .class
            .constant_pool()
            .try_get_class(index)?
            .to_rust_string()
            .replace('/', ".");
        return Err(ClassCastException {
            source_class_name,
            target_class_name,
        }
        .into());
    }
    Ok(object)
}
