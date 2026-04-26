use ristretto_classfile::VersionSpecification::{Any, Equal, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_17};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/lwawt/macosx/LWCToolkit.activateApplicationIgnoringOtherApps()V",
    Any
)]
#[async_method]
pub async fn activate_application_ignoring_other_apps<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.LWCToolkit.activateApplicationIgnoringOtherApps()V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/LWCToolkit.beep()V", Any)]
#[async_method]
pub async fn beep<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.LWCToolkit.beep()V".to_string()).into())
}

#[intrinsic_method("sun/lwawt/macosx/LWCToolkit.createAWTRunLoopMediator()J", Any)]
#[async_method]
pub async fn create_awt_run_loop_mediator<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.LWCToolkit.createAWTRunLoopMediator()J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/LWCToolkit.doAWTRunLoopImpl(JZZ)V", Any)]
#[async_method]
pub async fn do_awt_run_loop_impl<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _in_awt = parameters.pop_bool()?;
    let _process_events = parameters.pop_bool()?;
    let _mediator = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.LWCToolkit.doAWTRunLoopImpl(JZZ)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/LWCToolkit.flushNativeSelectors()V", Any)]
#[async_method]
pub async fn flush_native_selectors<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.LWCToolkit.flushNativeSelectors()V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/LWCToolkit.getMultiClickTime()I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn get_multi_click_time<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.LWCToolkit.getMultiClickTime()I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/LWCToolkit.initAppkit(Ljava/lang/ThreadGroup;Z)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn init_appkit<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _headless = parameters.pop_bool()?;
    let _appkit_thread_group = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.LWCToolkit.initAppkit(Ljava/lang/ThreadGroup;Z)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/LWCToolkit.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("sun/lwawt/macosx/LWCToolkit.isApplicationActive()Z", Any)]
#[async_method]
pub async fn is_application_active<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.LWCToolkit.isApplicationActive()Z".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/LWCToolkit.isCapsLockOn()Z", Any)]
#[async_method]
pub async fn is_caps_lock_on<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.LWCToolkit.isCapsLockOn()Z".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/lwawt/macosx/LWCToolkit.isEmbedded()Z", Any)]
#[async_method]
pub async fn is_embedded<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.LWCToolkit.isEmbedded()Z".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/lwawt/macosx/LWCToolkit.isInAquaSession()Z", Equal(JAVA_11))]
#[async_method]
pub async fn is_in_aqua_session<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.LWCToolkit.isInAquaSession()Z".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/LWCToolkit.loadNativeColors([I[I)V", Any)]
#[async_method]
pub async fn load_native_colors<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _apple_colors = parameters.pop_reference()?;
    let _system_colors = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.LWCToolkit.loadNativeColors([I[I)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/LWCToolkit.nativeSyncQueue(J)Z", Any)]
#[async_method]
pub async fn native_sync_queue<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _timeout = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.LWCToolkit.nativeSyncQueue(J)Z".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/LWCToolkit.performOnMainThreadAfterDelay(Ljava/lang/Runnable;J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn perform_on_main_thread_after_delay<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _delay = parameters.pop_long()?;
    let _runnable = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.LWCToolkit.performOnMainThreadAfterDelay(Ljava/lang/Runnable;J)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/LWCToolkit.stopAWTRunLoop(J)V", Any)]
#[async_method]
pub async fn stop_awt_run_loop<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _mediator = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.LWCToolkit.stopAWTRunLoop(J)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_activate_application_ignoring_other_apps() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = activate_application_ignoring_other_apps(thread, Parameters::default()).await;
        assert_eq!(
            "sun.lwawt.macosx.LWCToolkit.activateApplicationIgnoringOtherApps()V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_beep() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = beep(thread, Parameters::default()).await;
        assert_eq!(
            "sun.lwawt.macosx.LWCToolkit.beep()V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_create_awt_run_loop_mediator() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_awt_run_loop_mediator(thread, Parameters::default()).await;
        assert_eq!(
            "sun.lwawt.macosx.LWCToolkit.createAWTRunLoopMediator()J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_do_awt_run_loop_impl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_awt_run_loop_impl(
            thread,
            Parameters::new(vec![Value::Long(0), Value::from(false), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.LWCToolkit.doAWTRunLoopImpl(JZZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_flush_native_selectors() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = flush_native_selectors(thread, Parameters::default()).await;
        assert_eq!(
            "sun.lwawt.macosx.LWCToolkit.flushNativeSelectors()V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_multi_click_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_multi_click_time(thread, Parameters::default()).await;
        assert_eq!(
            "sun.lwawt.macosx.LWCToolkit.getMultiClickTime()I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_appkit() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_appkit(
            thread,
            Parameters::new(vec![Value::Object(None), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.LWCToolkit.initAppkit(Ljava/lang/ThreadGroup;Z)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_is_application_active() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_application_active(thread, Parameters::default()).await;
        assert_eq!(
            "sun.lwawt.macosx.LWCToolkit.isApplicationActive()Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_is_caps_lock_on() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_caps_lock_on(thread, Parameters::default()).await;
        assert_eq!(
            "sun.lwawt.macosx.LWCToolkit.isCapsLockOn()Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_is_embedded() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_embedded(thread, Parameters::default()).await;
        assert_eq!(
            "sun.lwawt.macosx.LWCToolkit.isEmbedded()Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_is_in_aqua_session() {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let result = is_in_aqua_session(thread, Parameters::default()).await;
        assert_eq!(
            "sun.lwawt.macosx.LWCToolkit.isInAquaSession()Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_load_native_colors() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = load_native_colors(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.LWCToolkit.loadNativeColors([I[I)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_sync_queue() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_sync_queue(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.LWCToolkit.nativeSyncQueue(J)Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_perform_on_main_thread_after_delay() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = perform_on_main_thread_after_delay(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.LWCToolkit.performOnMainThreadAfterDelay(Ljava/lang/Runnable;J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_stop_awt_run_loop() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = stop_awt_run_loop(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.LWCToolkit.stopAWTRunLoop(J)V",
            result.unwrap_err().to_string()
        );
    }
}
