use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.lwawt.macosx.LWCToolkit`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/LWCToolkit";
    registry.register(
        class_name,
        "activateApplicationIgnoringOtherApps",
        "()V",
        activate_application_ignoring_other_apps,
    );
    registry.register(class_name, "beep", "()V", beep);
    registry.register(
        class_name,
        "createAWTRunLoopMediator",
        "()J",
        create_awt_run_loop_mediator,
    );
    registry.register(
        class_name,
        "doAWTRunLoopImpl",
        "(JZZ)V",
        do_awt_run_loop_impl,
    );
    registry.register(
        class_name,
        "flushNativeSelectors",
        "()V",
        flush_native_selectors,
    );
    registry.register(class_name, "initIDs", "()V", init_ids);
    registry.register(
        class_name,
        "isApplicationActive",
        "()Z",
        is_application_active,
    );
    registry.register(class_name, "isCapsLockOn", "()Z", is_caps_lock_on);
    registry.register(class_name, "isEmbedded", "()Z", is_embedded);
    registry.register(
        class_name,
        "loadNativeColors",
        "([I[I)V",
        load_native_colors,
    );
    registry.register(class_name, "nativeSyncQueue", "(J)Z", native_sync_queue);
    registry.register(class_name, "stopAWTRunLoop", "(J)V", stop_awt_run_loop);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn activate_application_ignoring_other_apps(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn beep(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn create_awt_run_loop_mediator(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn do_awt_run_loop_impl(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn flush_native_selectors(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn is_application_active(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn is_caps_lock_on(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn is_embedded(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn load_native_colors(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_sync_queue(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn stop_awt_run_loop(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
