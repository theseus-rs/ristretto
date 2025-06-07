use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::{Any, Equal, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_17};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/lwawt/macosx/LWCToolkit.activateApplicationIgnoringOtherApps()V",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn activate_application_ignoring_other_apps(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.LWCToolkit.activateApplicationIgnoringOtherApps()V")
}

#[intrinsic_method("sun/lwawt/macosx/LWCToolkit.beep()V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn beep(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.LWCToolkit.beep()V")
}

#[intrinsic_method("sun/lwawt/macosx/LWCToolkit.createAWTRunLoopMediator()J", Any)]
#[async_recursion(?Send)]
pub(crate) async fn create_awt_run_loop_mediator(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.LWCToolkit.createAWTRunLoopMediator()J")
}

#[intrinsic_method("sun/lwawt/macosx/LWCToolkit.doAWTRunLoopImpl(JZZ)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn do_awt_run_loop_impl(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.LWCToolkit.doAWTRunLoopImpl(JZZ)V")
}

#[intrinsic_method("sun/lwawt/macosx/LWCToolkit.flushNativeSelectors()V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn flush_native_selectors(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.LWCToolkit.flushNativeSelectors()V")
}

#[intrinsic_method(
    "sun/lwawt/macosx/LWCToolkit.getMultiClickTime()I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_multi_click_time(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.LWCToolkit.getMultiClickTime()I")
}

#[intrinsic_method(
    "sun/lwawt/macosx/LWCToolkit.initAppkit(Ljava/lang/ThreadGroup;Z)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn init_appkit(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.LWCToolkit.initAppkit(Ljava/lang/ThreadGroup;Z)V")
}

#[intrinsic_method("sun/lwawt/macosx/LWCToolkit.initIDs()V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn init_ids(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("sun/lwawt/macosx/LWCToolkit.isApplicationActive()Z", Any)]
#[async_recursion(?Send)]
pub(crate) async fn is_application_active(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.LWCToolkit.isApplicationActive()Z")
}

#[intrinsic_method("sun/lwawt/macosx/LWCToolkit.isCapsLockOn()Z", Any)]
#[async_recursion(?Send)]
pub(crate) async fn is_caps_lock_on(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.LWCToolkit.isCapsLockOn()Z")
}

#[intrinsic_method("sun/lwawt/macosx/LWCToolkit.isEmbedded()Z", Any)]
#[async_recursion(?Send)]
pub(crate) async fn is_embedded(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.LWCToolkit.isEmbedded()Z")
}

#[intrinsic_method("sun/lwawt/macosx/LWCToolkit.isInAquaSession()Z", Equal(JAVA_11))]
#[async_recursion(?Send)]
pub(crate) async fn is_in_aqua_session(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.LWCToolkit.isInAquaSession()Z")
}

#[intrinsic_method("sun/lwawt/macosx/LWCToolkit.loadNativeColors([I[I)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn load_native_colors(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.LWCToolkit.loadNativeColors([I[I)V")
}

#[intrinsic_method("sun/lwawt/macosx/LWCToolkit.nativeSyncQueue(J)Z", Any)]
#[async_recursion(?Send)]
pub(crate) async fn native_sync_queue(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.LWCToolkit.nativeSyncQueue(J)Z")
}

#[intrinsic_method(
    "sun/lwawt/macosx/LWCToolkit.performOnMainThreadAfterDelay(Ljava/lang/Runnable;J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn perform_on_main_thread_after_delay(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.LWCToolkit.performOnMainThreadAfterDelay(Ljava/lang/Runnable;J)V")
}

#[intrinsic_method("sun/lwawt/macosx/LWCToolkit.stopAWTRunLoop(J)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn stop_awt_run_loop(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
        let _ = activate_application_ignoring_other_apps(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.LWCToolkit.beep()V")]
    async fn test_beep() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = beep(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.LWCToolkit.createAWTRunLoopMediator()J"
    )]
    async fn test_create_awt_run_loop_mediator() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_awt_run_loop_mediator(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.LWCToolkit.doAWTRunLoopImpl(JZZ)V"
    )]
    async fn test_do_awt_run_loop_impl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = do_awt_run_loop_impl(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.LWCToolkit.flushNativeSelectors()V"
    )]
    async fn test_flush_native_selectors() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = flush_native_selectors(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.LWCToolkit.getMultiClickTime()I"
    )]
    async fn test_get_multi_click_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_multi_click_time(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.LWCToolkit.initAppkit(Ljava/lang/ThreadGroup;Z)V"
    )]
    async fn test_init_appkit() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_appkit(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.LWCToolkit.isApplicationActive()Z"
    )]
    async fn test_is_application_active() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_application_active(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.LWCToolkit.isCapsLockOn()Z")]
    async fn test_is_caps_lock_on() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_caps_lock_on(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.LWCToolkit.isEmbedded()Z")]
    async fn test_is_embedded() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_embedded(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.LWCToolkit.isInAquaSession()Z"
    )]
    async fn test_is_in_aqua_session() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_in_aqua_session(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.LWCToolkit.loadNativeColors([I[I)V"
    )]
    async fn test_load_native_colors() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = load_native_colors(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.LWCToolkit.nativeSyncQueue(J)Z"
    )]
    async fn test_native_sync_queue() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_sync_queue(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.LWCToolkit.performOnMainThreadAfterDelay(Ljava/lang/Runnable;J)V"
    )]
    async fn test_perform_on_main_thread_after_delay() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = perform_on_main_thread_after_delay(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.LWCToolkit.stopAWTRunLoop(J)V"
    )]
    async fn test_stop_awt_run_loop() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = stop_awt_run_loop(thread, Parameters::default()).await;
    }
}
