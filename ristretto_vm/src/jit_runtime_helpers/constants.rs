//! Constant resolution with owning roots across every allocation/loader await.
use super::{
    InternalError, Ordering, Result, RuntimeContext, Value, set_top_frame_pc,
    store_pending_error_async,
};
use ristretto_jit::ConstantRequest;

impl RuntimeContext {
    pub(crate) async fn invoke_constant(
        &self,
        request: ConstantRequest,
        arguments: &[i64],
    ) -> Result<i64> {
        let bci = i32::try_from(request.bci)?;
        self.current_bci.store(bci, Ordering::Relaxed);
        set_top_frame_pc(self, bci);
        let result = if arguments.is_empty() {
            load(self, request.index).await
        } else {
            Err(InternalError(
                "invalid constant request argument count".to_owned(),
            ))
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

pub(super) async fn load(ctx: &RuntimeContext, index: u16) -> Result<i64> {
    match crate::instruction::resolve_reference_constant(&ctx.thread, &ctx.class, index).await? {
        Value::Object(Some(owner)) => {
            ctx.retain_helper_result(&owner);
            Ok(owner.as_handle_i64())
        }
        _ => Err(InternalError(
            "reference constant resolved to a non-reference".to_owned(),
        )),
    }
}
