use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_11: Version = Version::Java11 { minor: 0 };
const JAVA_17: Version = Version::Java17 { minor: 0 };

/// Register all native methods for `sun.lwawt.macosx.LWCToolkit`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/LWCToolkit";
    let java_version = registry.java_version().clone();

    if java_version >= JAVA_11 {
        registry.register(
            class_name,
            "initAppkit",
            "(Ljava/lang/ThreadGroup;Z)V",
            init_appkit,
        );
        registry.register(
            class_name,
            "performOnMainThreadAfterDelay",
            "(Ljava/lang/Runnable;J)V",
            perform_on_main_thread_after_delay,
        );
    }

    if java_version == JAVA_11 {
        registry.register(class_name, "isInAquaSession", "()Z", is_in_aqua_session);
    }
    if java_version == JAVA_17 {
        registry.register(class_name, "getMultiClickTime", "()I", get_multi_click_time);
    }

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

#[async_recursion(?Send)]
async fn activate_application_ignoring_other_apps(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.LWCToolkit.activateApplicationIgnoringOtherApps()V")
}

#[async_recursion(?Send)]
async fn beep(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.LWCToolkit.beep()V")
}

#[async_recursion(?Send)]
async fn create_awt_run_loop_mediator(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.LWCToolkit.createAWTRunLoopMediator()J")
}

#[async_recursion(?Send)]
async fn do_awt_run_loop_impl(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.LWCToolkit.doAWTRunLoopImpl(JZZ)V")
}

#[async_recursion(?Send)]
async fn flush_native_selectors(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.LWCToolkit.flushNativeSelectors()V")
}

#[async_recursion(?Send)]
async fn get_multi_click_time(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.LWCToolkit.getMultiClickTime()I")
}

#[async_recursion(?Send)]
async fn init_appkit(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.LWCToolkit.initAppkit(Ljava/lang/ThreadGroup;Z)V")
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn is_application_active(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.LWCToolkit.isApplicationActive()Z")
}

#[async_recursion(?Send)]
async fn is_caps_lock_on(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.LWCToolkit.isCapsLockOn()Z")
}

#[async_recursion(?Send)]
async fn is_embedded(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.LWCToolkit.isEmbedded()Z")
}

#[async_recursion(?Send)]
async fn is_in_aqua_session(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.LWCToolkit.isInAquaSession()Z")
}

#[async_recursion(?Send)]
async fn load_native_colors(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.LWCToolkit.loadNativeColors([I[I)V")
}

#[async_recursion(?Send)]
async fn native_sync_queue(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.LWCToolkit.nativeSyncQueue(J)Z")
}

#[async_recursion(?Send)]
async fn perform_on_main_thread_after_delay(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.LWCToolkit.performOnMainThreadAfterDelay(Ljava/lang/Runnable;J)V")
}

#[async_recursion(?Send)]
async fn stop_awt_run_loop(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.LWCToolkit.stopAWTRunLoop(J)V")
}
