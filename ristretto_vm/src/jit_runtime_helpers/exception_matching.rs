//! Typed handler lookup without a generated frame or payload guard across waits.
use super::{
    InternalError, Ordering, Reference, Result, RuntimeContext, gc_ref_from_ptr,
    resolve_class_ref_no_init_async, set_top_frame_pc, store_pending_error_async,
};
use crate::assignable::Assignable;
use ristretto_jit::ExceptionMatchRequest;

impl RuntimeContext {
    pub(crate) async fn invoke_exception_match(
        &self,
        request: ExceptionMatchRequest,
        arguments: &[i64],
    ) -> Result<i64> {
        let bci = i32::try_from(request.bci)?;
        self.current_bci.store(bci, Ordering::Relaxed);
        set_top_frame_pc(self, bci);
        let result = if arguments.is_empty() {
            matches(self, request.index).await
        } else {
            Err(InternalError(
                "invalid exception match argument count".to_owned(),
            ))
        };
        match result {
            Ok(matches) => Ok(i64::from(matches)),
            Err(error) => {
                store_pending_error_async(self, error).await;
                Ok(-1)
            }
        }
    }
}

pub(super) async fn matches(ctx: &RuntimeContext, index: u16) -> Result<bool> {
    let pending = ctx.pending_exception();
    if pending == 0 {
        return Ok(false);
    }
    // Own the exact throwable before the loader or payload access can suspend.
    let owner = gc_ref_from_ptr(ctx, pending)?;
    let target = resolve_class_ref_no_init_async(ctx, index).await?;
    let source = loop {
        if let Some(value) = owner.try_read() {
            let Reference::Object(object) = &*value else {
                return Err(InternalError(
                    "pending throwable is not an object".to_owned(),
                ));
            };
            break object.class().clone();
        }
        tokio::task::yield_now().await;
    };
    target.is_assignable_from(&ctx.thread, &source).await
}
