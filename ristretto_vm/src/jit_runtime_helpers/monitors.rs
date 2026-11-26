//! Interpreter and generated code share monitor ownership and cancellation behavior.
use super::{
    InternalError, NullPointerException, Ordering, Result, RuntimeContext, ctx_from_ptr,
    gc_ref_from_ptr, guard_helper, run_async, set_top_frame_pc, store_pending_error,
    store_pending_error_async,
};
use ristretto_jit::{MonitorKind, MonitorRequest};

impl RuntimeContext {
    pub(crate) async fn invoke_monitor(
        &self,
        request: MonitorRequest,
        arguments: &[i64],
    ) -> Result<i64> {
        let bci = i32::try_from(request.bci)?;
        self.current_bci.store(bci, Ordering::Relaxed);
        set_top_frame_pc(self, bci);
        let [object] = arguments else {
            return Err(InternalError(
                "monitor requires one object argument".to_owned(),
            ));
        };
        if let Err(error) = monitor(self, *object, request.kind).await {
            store_pending_error_async(self, error).await;
        }
        Ok(0)
    }
}

async fn monitor(ctx: &RuntimeContext, object: i64, kind: MonitorKind) -> Result<()> {
    if object == 0 {
        return Err(NullPointerException(Some(format!(
            "{} called on null",
            if kind == MonitorKind::Enter {
                "monitorenter"
            } else {
                "monitorexit"
            },
        )))
        .into());
    }
    let reference = gc_ref_from_ptr(ctx, object)?;
    match kind {
        MonitorKind::Enter => ctx.thread.enter_monitor(&reference).await,
        MonitorKind::Exit => ctx.thread.exit_monitor(&reference),
    }
}

pub(super) extern "C" fn jit_monitorenter(context: *const u8, bci: i32, object: i64) {
    synchronous(context, bci, object, MonitorKind::Enter);
}

pub(super) extern "C" fn jit_monitorexit(context: *const u8, bci: i32, object: i64) {
    synchronous(context, bci, object, MonitorKind::Exit);
}

fn synchronous(context: *const u8, bci: i32, object: i64, kind: MonitorKind) {
    guard_helper("jit_monitor", || {
        let ctx = ctx_from_ptr(context);
        ctx.current_bci.store(bci, Ordering::Relaxed);
        set_top_frame_pc(ctx, bci);
        if let Err(error) = run_async(monitor(ctx, object, kind)) {
            store_pending_error(ctx, error);
        }
    });
}
