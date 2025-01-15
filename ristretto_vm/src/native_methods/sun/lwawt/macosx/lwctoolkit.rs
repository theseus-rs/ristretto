use crate::arguments::Arguments;
use crate::native_methods::registry::{MethodRegistry, JAVA_11, JAVA_17};
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/lwawt/macosx/LWCToolkit";

/// Register all native methods for `sun.lwawt.macosx.LWCToolkit`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() >= JAVA_11 {
        registry.register(
            CLASS_NAME,
            "initAppkit",
            "(Ljava/lang/ThreadGroup;Z)V",
            init_appkit,
        );
        registry.register(
            CLASS_NAME,
            "performOnMainThreadAfterDelay",
            "(Ljava/lang/Runnable;J)V",
            perform_on_main_thread_after_delay,
        );
    }

    if registry.java_major_version() == JAVA_11 {
        registry.register(CLASS_NAME, "isInAquaSession", "()Z", is_in_aqua_session);
    }
    if registry.java_major_version() >= JAVA_17 {
        registry.register(CLASS_NAME, "getMultiClickTime", "()I", get_multi_click_time);
    }

    registry.register(
        CLASS_NAME,
        "activateApplicationIgnoringOtherApps",
        "()V",
        activate_application_ignoring_other_apps,
    );
    registry.register(CLASS_NAME, "beep", "()V", beep);
    registry.register(
        CLASS_NAME,
        "createAWTRunLoopMediator",
        "()J",
        create_awt_run_loop_mediator,
    );
    registry.register(
        CLASS_NAME,
        "doAWTRunLoopImpl",
        "(JZZ)V",
        do_awt_run_loop_impl,
    );
    registry.register(
        CLASS_NAME,
        "flushNativeSelectors",
        "()V",
        flush_native_selectors,
    );
    registry.register(CLASS_NAME, "initIDs", "()V", init_ids);
    registry.register(
        CLASS_NAME,
        "isApplicationActive",
        "()Z",
        is_application_active,
    );
    registry.register(CLASS_NAME, "isCapsLockOn", "()Z", is_caps_lock_on);
    registry.register(CLASS_NAME, "isEmbedded", "()Z", is_embedded);
    registry.register(
        CLASS_NAME,
        "loadNativeColors",
        "([I[I)V",
        load_native_colors,
    );
    registry.register(CLASS_NAME, "nativeSyncQueue", "(J)Z", native_sync_queue);
    registry.register(CLASS_NAME, "stopAWTRunLoop", "(J)V", stop_awt_run_loop);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.LWCToolkit.activateApplicationIgnoringOtherApps()V"
    )]
    async fn test_activate_application_ignoring_other_apps() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = activate_application_ignoring_other_apps(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.LWCToolkit.beep()V")]
    async fn test_beep() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = beep(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.LWCToolkit.createAWTRunLoopMediator()J"
    )]
    async fn test_create_awt_run_loop_mediator() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_awt_run_loop_mediator(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.LWCToolkit.doAWTRunLoopImpl(JZZ)V"
    )]
    async fn test_do_awt_run_loop_impl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = do_awt_run_loop_impl(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.LWCToolkit.flushNativeSelectors()V"
    )]
    async fn test_flush_native_selectors() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = flush_native_selectors(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Arguments::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.LWCToolkit.isApplicationActive()Z"
    )]
    async fn test_is_application_active() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_application_active(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.LWCToolkit.isCapsLockOn()Z")]
    async fn test_is_caps_lock_on() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_caps_lock_on(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.LWCToolkit.isEmbedded()Z")]
    async fn test_is_embedded() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_embedded(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.LWCToolkit.isInAquaSession()Z"
    )]
    async fn test_is_in_aqua_session() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_in_aqua_session(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.LWCToolkit.loadNativeColors([I[I)V"
    )]
    async fn test_load_native_colors() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = load_native_colors(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.LWCToolkit.nativeSyncQueue(J)Z"
    )]
    async fn test_native_sync_queue() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_sync_queue(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.LWCToolkit.stopAWTRunLoop(J)V"
    )]
    async fn test_stop_awt_run_loop() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = stop_awt_run_loop(thread, Arguments::default()).await;
    }
}
