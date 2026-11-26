//! Reference stores resolve types with no generated frame or managed guard held.
use super::{
    InternalError, NullPointerException, Ordering, Result, RuntimeContext, gc_ref_from_ptr,
    set_top_frame_pc, store_pending_error_async,
};
use ristretto_jit::ArrayStoreRequest;

impl RuntimeContext {
    pub(crate) async fn invoke_array_store(
        &self,
        request: ArrayStoreRequest,
        arguments: &[i64],
    ) -> Result<i64> {
        let bci = i32::try_from(request.bci)?;
        self.current_bci.store(bci, Ordering::Relaxed);
        set_top_frame_pc(self, bci);
        let result = match arguments {
            [array, index, value] => match i32::try_from(*index) {
                Ok(index) => store(self, *array, index, *value).await,
                Err(error) => Err(error.into()),
            },
            _ => Err(InternalError(
                "invalid reference-store argument count".to_owned(),
            )),
        };
        if let Err(error) = result {
            store_pending_error_async(self, error).await;
        }
        Ok(0)
    }
}

pub(super) async fn store(ctx: &RuntimeContext, array: i64, index: i32, value: i64) -> Result<()> {
    if array == 0 {
        return Err(NullPointerException(None).into());
    }
    let array = gc_ref_from_ptr(ctx, array)?;
    let value = if value == 0 {
        None
    } else {
        Some(gc_ref_from_ptr(ctx, value)?)
    };
    crate::instruction::store_reference_array(&ctx.thread, &array, index, value).await
}
